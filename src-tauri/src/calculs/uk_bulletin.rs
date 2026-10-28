use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, Salarie};
use super::uk_cotisations::{uk_ni_sal, uk_ni_pat, uk_income_tax};

pub fn generer_bulletin_uk(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;

    // Seuils et barème datés en base (migration 0151) depuis l'exercice 2024/25.
    if !super::uk_cotisations::couvert(ctx) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "GBP", "UK",
            "Royaume-Uni : barèmes intégrés à partir de l'exercice 2024/25.", ctx);
    }

    let cotisations = vec![
        uk_ni_sal(brut, ctx),
        uk_ni_pat(brut, ctx),
        uk_income_tax(brut, ctx),
    ];

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "GBP".into(),
        absence: None,
        heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), pas: None, droits: None, alertes: Vec::new(), evolutions: Vec::new(),
        salarie,
    }
}
