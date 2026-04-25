use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::SqlitePool;

use crate::db::ContextPaie;
use crate::models::{LigneMensuelle, Salarie, SimulationAnnuelle, Statut};
use super::bulletin::generer_bulletin;
use super::cotisations::fillon_coeff;

const MOIS_FR: [&str; 12] = [
    "Janvier", "Février", "Mars", "Avril", "Mai", "Juin",
    "Juillet", "Août", "Septembre", "Octobre", "Novembre", "Décembre",
];

/// Simule 12 bulletins mensuels avec Fillon annualisé et régularisation.
///
/// La régularisation corrige chaque mois l'écart entre :
/// - le Fillon théorique calculé sur les cumuls (brut et SMIC depuis janvier)
/// - le Fillon déjà versé les mois précédents
///
/// Cela permet de refléter fidèlement les hausses de SMIC en cours d'année
/// (ex : 3 revalorisations en 2022 : janvier, mai, août).
pub async fn generer_annee(
    pool: &SqlitePool,
    brut: Decimal,
    statut: Statut,
    annee: i32,
) -> Result<SimulationAnnuelle> {
    let mut lignes        = Vec::with_capacity(12);
    let mut brut_cumule   = Decimal::ZERO;
    let mut smic_cumule   = Decimal::ZERO;
    let mut fillon_verse  = Decimal::ZERO;  // cumul Fillon régularisé déjà versé

    for mois in 1u32..=12 {
        let date = NaiveDate::from_ymd_opt(annee, mois, 1)
            .ok_or_else(|| anyhow!("Date invalide : {annee}-{mois:02}-01"))?;

        let ctx = ContextPaie::charger(pool, date).await?;

        // On génère un bulletin complet pour réutiliser toute la logique de
        // cotisations (SS, AGIRC-ARRCO, etc.) sans la dupliquer ici.
        // Le Fillon inclus dans ce bulletin est le calcul mensuel simple ;
        // on l'extrait séparément pour le remplacer par la version régularisée.
        let dummy = Salarie {
            nom: String::new(), prenom: String::new(),
            salaire_brut: brut, statut: statut.clone(),
            alsace_moselle: false,
        };
        let bulletin = generer_bulletin(dummy, &ctx);

        // total_pat_brut = charges patronales hors Fillon.
        // On exclu REDUCTION_FILLON du cumul car on recalcule le montant
        // régularisé ci-dessous (fillon_verse/fillon_reg).
        let total_sal: Decimal = bulletin.cotisations.iter()
            .map(|c| c.montant_sal)
            .sum();
        let total_pat_brut: Decimal = bulletin.cotisations.iter()
            .filter(|c| c.code != "REDUCTION_FILLON")
            .map(|c| c.montant_pat)
            .sum();

        // Fillon mensuel simple (pour la colonne Δ régularisation)
        let fillon_simple = bulletin.cotisations.iter()
            .find(|c| c.code == "REDUCTION_FILLON")
            .map(|c| c.montant_pat.abs())
            .unwrap_or(Decimal::ZERO);

        // ── Fillon régularisé ────────────────────────────────────────────
        brut_cumule  += brut;
        smic_cumule  += ctx.smic_mensuel;

        let fillon_reg = fillon_regularise_mois(
            brut_cumule, smic_cumule, fillon_verse, &ctx,
        );
        fillon_verse += fillon_reg;

        lignes.push(LigneMensuelle {
            mois,
            mois_libelle:     MOIS_FR[(mois - 1) as usize].to_string(),
            brut,
            smic:             ctx.smic_mensuel,
            pmss:             ctx.pmss,
            total_sal,
            total_pat_brut,
            fillon_simple,
            fillon_regularise: fillon_reg,
            net_a_payer:      bulletin.net_a_payer,
            cout_employeur:   (brut + total_pat_brut - fillon_reg).round_dp(2),
        });
    }

    Ok(SimulationAnnuelle {
        annee,
        total_brut:     lignes.iter().map(|l| l.brut).sum(),
        total_fillon:   lignes.iter().map(|l| l.fillon_regularise).sum(),
        total_net:      lignes.iter().map(|l| l.net_a_payer).sum(),
        total_cout:     lignes.iter().map(|l| l.cout_employeur).sum(),
        total_sal:      lignes.iter().map(|l| l.total_sal).sum(),
        total_pat_brut: lignes.iter().map(|l| l.total_pat_brut).sum(),
        lignes,
    })
}

/// Fillon régularisé pour le mois N (méthode officielle annualisée).
///
/// Principe de la régularisation :
///   1. On calcule le Fillon THÉORIQUE cumulé depuis janvier, avec le coefficient
///      calculé sur les cumuls de brut et de SMIC (= moyenne pondérée si SMIC change).
///   2. On soustrait le Fillon déjà versé les mois précédents.
///   3. Le résultat est forcément ≥ 0 (on ne peut pas "rembourser" une réduction).
///
/// L'utilisation de `fillon_coeff(smic_cumule, brut_cumule, ctx)` applique
/// automatiquement la bonne formule (puissance 2019+ ou linéaire 2015-2018)
/// selon les paramètres chargés depuis la DB pour la date du mois en cours.
fn fillon_regularise_mois(
    brut_cumule:  Decimal,
    smic_cumule:  Decimal,
    fillon_verse: Decimal,
    ctx:          &ContextPaie,
) -> Decimal {
    if brut_cumule <= Decimal::ZERO {
        return Decimal::ZERO;
    }

    // Le ratio smic_cumule/brut_cumule représente la moyenne pondérée SMIC/brut
    // sur les mois écoulés → reflète correctement les revalorisations du SMIC en cours d'année.
    let coeff = fillon_coeff(smic_cumule, brut_cumule, ctx);
    if coeff == Decimal::ZERO {
        return Decimal::ZERO;
    }

    let theorique_cumule = (brut_cumule * coeff).round_dp(2);
    (theorique_cumule - fillon_verse).max(Decimal::ZERO)
}
