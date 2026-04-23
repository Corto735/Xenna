// ContextPaie : snapshot immuable de tous les paramètres de paie valides
// à une date donnée. On charge tout depuis SQLite en une passe async, puis
// tous les calculs de cotisations sont purement synchrones (pas d'I/O).
//
// Logique de sélection temporelle (pattern commun à toutes les requêtes) :
//   date_debut <= date_paie   → la période a commencé
//   date_fin IS NULL          → toujours valide (valeur courante)
//   date_fin > date_paie      → la période n'est pas encore terminée
//   ORDER BY date_debut DESC LIMIT 1 → prend la version la plus récente si overlap

use std::collections::HashMap;
use anyhow::{Context, Result};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::SqlitePool;

/// Snapshot de tous les paramètres de paie valides à une date donnée.
/// Chargé une fois en base, puis le calcul est purement synchrone.
pub struct ContextPaie {
    pub date_paie: NaiveDate,
    pub pmss: Decimal,
    pub smic_mensuel: Decimal,

    // Paramètres Fillon — None si aucune ligne ne correspond à la date.
    //
    // Deux formules coexistent selon la période :
    //
    //   2015-2018 (linéaire) : tmin=None, puissance=None
    //     C = (coeff_max / 0,6) × (seuil × SMIC / brut − 1)  ∈ [0 ; coeff_max]
    //
    //   2019+    (puissance) : tmin=Some(0.02), puissance=Some(1.75)
    //     C = tmin + (tdelta × [(1/2) × (seuil × SMIC / brut − 1)]^P)  ∈ [0 ; coeff_max]
    //     Où tdelta = coeff_max − tmin = 0,3781
    //
    // Le code choisit la formule en testant fillon_puissance.is_some().
    pub fillon_coeff_max:  Option<Decimal>,   // Tmax = Tmin + Tdelta (ex: 0,3981)
    pub fillon_seuil_smic: Option<Decimal>,   // seuil en × SMIC (1,6 ancienne / 3,0 nouvelle)
    pub fillon_tmin:       Option<Decimal>,   // Tmin = 0,0200 (None = ancienne formule)
    pub fillon_puissance:  Option<Decimal>,   // P = 1,75 (None = ancienne formule)

    // Map code_cotisation → (taux_salarial, taux_patronal) en Decimal.
    // Les codes absents retournent Decimal::ZERO via taux_sal/taux_pat.
    taux: HashMap<String, (Decimal, Decimal)>,
}

impl ContextPaie {
    pub async fn charger(pool: &SqlitePool, date: NaiveDate) -> Result<Self> {
        // SQLite ne comprend pas NaiveDate directement → on formate en ISO 8601.
        let d = date.format("%Y-%m-%d").to_string();

        // fetch_one échoue si aucune ligne → message d'erreur explicite propagé
        // jusqu'au frontend. La base doit couvrir toutes les dates utilisées.
        let (pmss_str,): (String,) = sqlx::query_as(
            "SELECT valeur FROM plafond_reference
             WHERE code = 'PMSS' AND date_debut <= ? AND (date_fin IS NULL OR date_fin > ?)
             ORDER BY date_debut DESC LIMIT 1",
        )
        .bind(&d).bind(&d)
        .fetch_one(pool)
        .await
        .context("PMSS introuvable pour cette date")?;

        let (smic_str,): (String,) = sqlx::query_as(
            "SELECT valeur FROM plafond_reference
             WHERE code = 'SMIC_MENSUEL' AND date_debut <= ? AND (date_fin IS NULL OR date_fin > ?)
             ORDER BY date_debut DESC LIMIT 1",
        )
        .bind(&d).bind(&d)
        .fetch_one(pool)
        .await
        .context("SMIC_MENSUEL introuvable pour cette date")?;

        // Charge tous les taux en une seule requête plutôt que N requêtes par cotisation.
        // Les taux sont stockés en TEXT pour préserver la précision exacte
        // (SQLite NUMERIC peut arrondir silencieusement).
        let rows: Vec<(String, String, String)> = sqlx::query_as(
            "SELECT c.code, ct.taux_salarial, ct.taux_patronal
             FROM cotisation c
             JOIN cotisation_taux ct ON ct.cotisation_id = c.id
             WHERE ct.date_debut <= ? AND (ct.date_fin IS NULL OR ct.date_fin > ?)",
        )
        .bind(&d).bind(&d)
        .fetch_all(pool)
        .await
        .context("Erreur chargement taux")?;

        let mut taux = HashMap::with_capacity(rows.len());
        for (code, ts, tp) in rows {
            let sal: Decimal = ts.parse().with_context(|| format!("taux_sal invalide pour {code}"))?;
            let pat: Decimal = tp.parse().with_context(|| format!("taux_pat invalide pour {code}"))?;
            taux.insert(code, (sal, pat));
        }

        // fetch_optional : Fillon peut ne pas exister pour la date (avant 2015).
        // Toutes les colonnes Fillon sont NULLables → Option<Option<String>>.
        // tmin et puissance sont NULL pour la formule 2015-2018 (linéaire),
        // présents pour la formule 2019+ (puissance).
        let fillon: Option<(Option<String>, Option<String>, Option<String>, Option<String>)> =
            sqlx::query_as(
                "SELECT ap.coeff_max_fillon, ap.seuil_smic_coeff, ap.tmin, ap.puissance
                 FROM allegement_param ap
                 JOIN allegement_type at2 ON at2.id = ap.allegement_type_id
                 WHERE at2.code = 'FILLON'
                   AND ap.date_debut <= ? AND (ap.date_fin IS NULL OR ap.date_fin > ?)
                 ORDER BY ap.date_debut DESC LIMIT 1",
            )
            .bind(&d).bind(&d)
            .fetch_optional(pool)
            .await
            .context("Erreur chargement params Fillon")?;

        let (fillon_coeff_max, fillon_seuil_smic, fillon_tmin, fillon_puissance) = match fillon {
            Some((c, s, t, p)) => (
                c.and_then(|v| v.parse().ok()),
                s.and_then(|v| v.parse().ok()),
                t.and_then(|v| v.parse().ok()),
                p.and_then(|v| v.parse().ok()),
            ),
            None => (None, None, None, None),
        };

        Ok(ContextPaie {
            date_paie: date,
            pmss: pmss_str.parse().context("PMSS non parseable")?,
            smic_mensuel: smic_str.parse().context("SMIC non parseable")?,
            taux,
            fillon_coeff_max,
            fillon_seuil_smic,
            fillon_tmin,
            fillon_puissance,
        })
    }

    // Retourne Decimal::ZERO si le code est absent du contexte (cotisation non
    // applicable à cette date ou code inexistant en base). C'est intentionnel :
    // une cotisation à taux 0 ne contribue pas mais n'est pas une erreur.
    pub fn taux_sal(&self, code: &str) -> Decimal {
        self.taux.get(code).map(|(s, _)| *s).unwrap_or(Decimal::ZERO)
    }

    pub fn taux_pat(&self, code: &str) -> Decimal {
        self.taux.get(code).map(|(_, p)| *p).unwrap_or(Decimal::ZERO)
    }
}
