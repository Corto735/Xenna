// Entreprise adaptée (EA) — aide au poste.
//
// L'aide au poste est un forfait annuel PAR ETP versé à l'EMPLOYEUR par l'ASP
// au titre de l'emploi d'un travailleur handicapé (RQTH). Ce n'est PAS une
// cotisation : elle ne modifie ni le brut, ni le net, ni le net imposable du
// salarié. Sur le bulletin elle apparaît comme une ligne à montant patronal
// négatif (un gain pour l'employeur) qui réduit le coût total employeur.
//
// Montant : forfait annuel selon la tranche d'âge (aide socle EA classique,
// métropole), versé par douzième et proratisé au temps de travail (ETP).
// En cas d'arrêt maladie/accident, la part absente est minorée à 30 % du SMIC
// horaire brut. Comme heures_absence = absent_fraction × 151,67 × etp_ratio et
// smic_horaire = smic_mensuel / 151,67, la part absente se simplifie en
// 0,30 × smic_mensuel × absent_fraction × etp_ratio.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{LigneCotisation, Salarie};

/// Code de plafond (forfait annuel) selon la tranche d'âge déclarée.
fn code_forfait(tranche: Option<&str>) -> &'static str {
    match tranche {
        Some("50_55") => "AIDE_POSTE_EA_50_55",
        Some("56p")   => "AIDE_POSTE_EA_56P",
        _             => "AIDE_POSTE_EA_M50", // défaut < 50 ans
    }
}

/// Ligne « aide au poste » pour une entreprise adaptée, ou None si aucun barème
/// n'est en base pour la date de paie (ex. avant le 01/11/2024).
///
/// `brut_plein` = brut mensuel de référence (sert d'assiette d'affichage).
/// `absent_fraction` = part du mois en arrêt maladie (0 si pas d'absence).
pub fn aide_poste_ea(
    salarie: &Salarie,
    brut_plein: Decimal,
    absent_fraction: Decimal,
    ctx: &ContextPaie,
) -> Option<LigneCotisation> {
    let code = code_forfait(salarie.tranche_age_ea.as_deref());
    let tranche_libelle = match salarie.tranche_age_ea.as_deref() {
        Some("50_55") => "50 à 55 ans",
        Some("56p")   => "56 ans et plus",
        _             => "moins de 50 ans",
    };
    let forfait_annuel = ctx.plafond(code)?;
    let forfait_mensuel = forfait_annuel / dec!(12);

    // Ratio temps partiel (ETP/100), borné comme dans le reste du calcul.
    let etp_ratio: Decimal = format!("{:.6}", (salarie.etp / 100.0).clamp(0.0, 2.0))
        .parse()
        .unwrap_or(dec!(1));

    let frac = absent_fraction.clamp(Decimal::ZERO, Decimal::ONE);
    let part_travaillee = forfait_mensuel * (Decimal::ONE - frac);
    let part_absente = dec!(0.30) * ctx.smic_mensuel * frac;
    let aide = (etp_ratio * (part_travaillee + part_absente)).round_dp(2);

    // Bloc de détail pour le modal « formule » du front : valeurs authoritatives
    // (forfait issu de la base, ETP, minoration arrêt) afin d'afficher le calcul
    // pas à pas sans réimplémenter le barème côté JS. Préfixé par \u{1} (SOH),
    // que le front isole et retire du texte d'explication affiché.
    let explication = ctx.expl("AIDE_POSTE_EA", "L'aide au poste est une aide financière de l'État, versée à \
        l'employeur par l'Agence de services et de paiement (ASP), au titre de l'emploi d'un \
        travailleur handicapé (RQTH) en entreprise adaptée. Forfait annuel par équivalent temps \
        plein, versé par douzième et proratisé au temps de travail. Montant selon la tranche \
        d'âge du salarié : 18 230 € (< 50 ans), 18 465 € (50-55 ans), 18 941 € (56 ans et +) \
        depuis le 01/11/2024. En cas d'arrêt maladie ou accident, la part absente est minorée à \
        30 % du SMIC horaire brut. Cette aide ne modifie ni le brut, ni le net du salarié : elle \
        réduit le coût réel supporté par l'employeur.");

    let detail = format!(
        "\u{1}{{\"tranche\":\"{}\",\"forfait_annuel\":\"{}\",\"forfait_mensuel\":\"{}\",\"etp\":{:.2},\"etp_ratio\":\"{}\",\"absent_fraction\":\"{}\",\"part_absente\":\"{}\",\"smic_mensuel\":\"{}\",\"aide\":\"{}\"}}",
        tranche_libelle,
        forfait_annuel.round_dp(2),
        (forfait_annuel / dec!(12)).round_dp(2),
        salarie.etp,
        etp_ratio,
        frac,
        part_absente.round_dp(2),
        ctx.smic_mensuel.round_dp(2),
        aide,
    );

    Some(LigneCotisation {
        code:        "AIDE_POSTE_EA".into(),
        libelle:     ctx.libelle("AIDE_POSTE_EA", "Aide au poste — entreprise adaptée (État/ASP)"),
        base:        brut_plein,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    Decimal::ZERO,
        montant_pat: -aide, // négatif = gain employeur, réduit le coût total
        categorie:   "Aide à l'emploi".into(),
        explication: format!("{explication}{detail}"),
        loi_ref: Some("CT art. L5213-19 et R5213-76 — Arrêté du 16/01/2025".into()),
    })
}
