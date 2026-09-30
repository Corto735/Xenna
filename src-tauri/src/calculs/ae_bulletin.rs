// Bulletin de paie des Émirats arabes unis : aucun impôt sur le revenu.
//
// Expatrié (défaut) : aucune cotisation sociale → net = brut. Une ligne
// d'information le rappelle. National émirati (`emirati_national`) : régime de
// retraite GPSSA — salarié 5 % + employeur 12,5 % (l'État abonde 2,5 %
// supplémentaires, hors bulletin). Assiette contributive plafonnée à 50 000 AED.
// C'est le régime de la loi fédérale 7/1999, qui reste celui des Émiratis employés
// avant le 31/10/2023 (paramètres inchangés en 2026). Le décret-loi 57/2023 (entrées
// depuis cette date : salarié 11 %, employeur 15 % dont 2,5 % pris en charge par
// l'État sous 20 000 AED, plafond 70 000 AED) n'est pas modélisé : le simulateur ne
// connaît pas la date d'embauche.
// Devise AED. Données : 2025-2026.
//
// Sources : Federal Law No. 7 of 1999 ; Federal Decree-Law No. 57 of 2023 ;
// General Pension and Social Security Authority (GPSSA).

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Plafond mensuel de l'assiette contributive GPSSA (AED).
const PLAFOND_GPSSA: Decimal = dec!(50000);

pub fn generer_bulletin_ae(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let national = salarie.emirati_national.unwrap_or(false);
    let mut cotisations = Vec::new();

    if national {
        // GPSSA : assiette = min(brut, plafond).
        let base = brut.min(PLAFOND_GPSSA);
        let ts = ctx.taux_sal("AE_GPSSA");
        let tp = ctx.taux_pat("AE_GPSSA");
        let expl = ctx.expl("AE_GPSSA",
            "GPSSA (General Pension and Social Security Authority) — régime de retraite \
            des nationaux émiratis. Salarié {ts} % + employeur {tp} % sur l'assiette \
            contributive (plafonnée à 50 000 AED) ; l'État abonde 2,5 % en plus. \
            Base {base} AED. Base légale : Federal Decree-Law 57/2023.")
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{base}", &format!("{:.2}", base));
        cotisations.push(LigneCotisation {
            code: "AE_GPSSA".into(),
            libelle: ctx.libelle("AE_GPSSA", "GPSSA — Retraite (national émirati)"),
            base,
            taux_sal: ts, montant_sal: (base * ts).round_dp(2),
            taux_pat: tp, montant_pat: (base * tp).round_dp(2),
            categorie: "Retraite".into(),
            explication: expl,
            loi_ref: Some(ctx.loi_ref("Federal Decree-Law 57/2023 — GPSSA")),
        });
    } else {
        // Expatrié : rien à prélever, on l'explicite plutôt que de rendre un bulletin vide.
        cotisations.push(LigneCotisation {
            code: "AE_EXPAT".into(),
            libelle: ctx.libelle("AE_EXPAT", "Aucune cotisation (expatrié)"),
            base: brut,
            taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
            taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
            categorie: "Information".into(),
            explication: ctx.expl("AE_EXPAT",
                "Les Émirats arabes unis ne prélèvent ni impôt sur le revenu ni cotisation \
                sociale sur les salariés expatriés : le net égale le brut. Le régime de \
                retraite GPSSA ne concerne que les nationaux émiratis (cocher l'option)."),
            loi_ref: None,
        });
    }

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "AED".into(), absence: None, heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), salarie,
    }
}
