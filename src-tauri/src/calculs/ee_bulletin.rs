// ── Estonie — sotsiaalmaks (pat) + chômage + 2ᵉ pilier + impôt 22 % ──────────────
//
// Côté salarié 2025 :
//   • töötuskindlustusmakse (chômage) 1,6 % ;
//   • kogumispension (2ᵉ pilier obligatoire) 2 % (taux par défaut) ;
//   • tulumaks (impôt) 22 % depuis le 01/01/2025, sur le revenu après cotisations
//     salariales et abattement de base (maksuvaba tulu) dégressif.
// Côté employeur : sotsiaalmaks 33 % + töötuskindlustus 0,8 %.
//
// Abattement de base 2025 (dégressif sur le revenu annuel brut) :
//   ≤ 14 400 € → 7 848 € ; ≥ 25 200 € → 0 ; linéaire entre les deux.
// Source : Maksu- ja Tolliamet (tulumaks 22 % en 2025) ; Sotsiaalmaksuseadus.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

fn ligne_cot(code: &str, libelle: &str, base: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    LigneCotisation {
        code: code.into(), libelle: libelle.into(), base,
        taux_sal: ts, montant_sal: (base * ts).round_dp(2),
        taux_pat: tp, montant_pat: (base * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "{libelle}. Salarié {ts:.2} % / employeur {tp:.2} %.",
            ts = ts * dec!(100), tp = tp * dec!(100),
        ),
        loi_ref: Some("Sotsiaalmaksuseadus".into()),
    }
}

/// Abattement de base annuel 2025 (maksuvaba tulu), dégressif.
fn abattement_annuel(g: Decimal) -> Decimal {
    if g <= dec!(14400) {
        dec!(7848)
    } else if g >= dec!(25200) {
        Decimal::ZERO
    } else {
        // 7848 − 7848/10800 × (g − 14400)
        (dec!(7848) - dec!(7848) / dec!(10800) * (g - dec!(14400))).max(Decimal::ZERO)
    }
}

pub fn generer_bulletin_ee(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Estonie : données disponibles pour 2025.");
    }

    let g = brut * dec!(12);

    let mut cotisations = vec![
        ligne_cot("EE_TOOTUS",        "Töötuskindlustusmakse — Chômage",   brut, ctx),
        ligne_cot("EE_KOGUMISPENSION","Kogumispension — Retraite 2ᵉ pilier", brut, ctx),
        ligne_cot("EE_SOTSIAALMAKS",  "Sotsiaalmaks — Charge sociale (employeur)", brut, ctx),
    ];

    // Base imposable : brut − cotisations salariales − abattement de base.
    let cot_sal_taux = ctx.taux_sal("EE_TOOTUS") + ctx.taux_sal("EE_KOGUMISPENSION");
    let abatt_an = abattement_annuel(g);
    let base_imp_an = (g * (Decimal::ONE - cot_sal_taux) - abatt_an).max(Decimal::ZERO);
    let impot_mens = (base_imp_an * dec!(0.22) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "EE_TULUMAKS".into(),
        libelle: "Tulumaks — Impôt sur le revenu (22 %)".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu 2025 : 22 % (taux unique).\n\n\
            Revenu annuel {g:.0} € − cotisations salariales − abattement de base {ab:.0} €\n\
            = base imposable {b:.0} € → {im:.2} €/mois.\n\n\
            Abattement de base dégressif (7 848 € si ≤ 14 400 €/an, nul si ≥ 25 200 €/an).\n\
            Source : Maksu- ja Tolliamet.",
            g = g, ab = abatt_an, b = base_imp_an, im = impot_mens,
        ),
        loi_ref: Some("Tulumaksuseadus".into()),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(), absence: None, heures_sup: None, salarie,
    }
}
