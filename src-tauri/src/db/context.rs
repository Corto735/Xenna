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
    pub fillon_coeff_max: Option<Decimal>,
    pub fillon_seuil_smic: Option<Decimal>,
    taux: HashMap<String, (Decimal, Decimal)>,
}

impl ContextPaie {
    pub async fn charger(pool: &SqlitePool, date: NaiveDate) -> Result<Self> {
        let d = date.format("%Y-%m-%d").to_string();

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

        let fillon: Option<(Option<String>, Option<String>)> = sqlx::query_as(
            "SELECT ap.coeff_max_fillon, ap.seuil_smic_coeff
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

        let (fillon_coeff_max, fillon_seuil_smic) = match fillon {
            Some((Some(c), Some(s))) => (c.parse().ok(), s.parse().ok()),
            _ => (None, None),
        };

        Ok(ContextPaie {
            date_paie: date,
            pmss: pmss_str.parse().context("PMSS non parseable")?,
            smic_mensuel: smic_str.parse().context("SMIC non parseable")?,
            taux,
            fillon_coeff_max,
            fillon_seuil_smic,
        })
    }

    pub fn taux_sal(&self, code: &str) -> Decimal {
        self.taux.get(code).map(|(s, _)| *s).unwrap_or(Decimal::ZERO)
    }

    pub fn taux_pat(&self, code: &str) -> Decimal {
        self.taux.get(code).map(|(_, p)| *p).unwrap_or(Decimal::ZERO)
    }
}
