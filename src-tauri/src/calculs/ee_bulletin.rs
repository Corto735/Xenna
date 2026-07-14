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
//
// 2026 : suppression de la dégressivité (« tax hump ») — abattement forfaitaire
// universel de 700 €/mois = 8 400 €/an quel que soit le revenu. Taux d'impôt
// inchangé à 22 %. (Abattement majoré 776 €/mois pour les retraités, non modélisé.)
// Source : Maksu- ja Tolliamet (tulumaks 22 % ; maksuvaba tulu 8 400 € en 2026) ;
// Sotsiaalmaksuseadus.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

fn ligne_cot(code: &str, libelle: &str, base: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    let lib = ctx.libelle(code, libelle);
    let explication = ctx.expl("EE_GENERIC", "{libelle}. Salarié {ts} % / employeur {tp} %.")
        .replace("{libelle}", &lib)
        .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
        .replace("{tp}", &format!("{:.2}", tp * dec!(100)));
    LigneCotisation {
        code: code.into(), libelle: lib, base,
        taux_sal: ts, montant_sal: (base * ts).round_dp(2),
        taux_pat: tp, montant_pat: (base * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("Sotsiaalmaksuseadus")),
    }
}

/// Abattement de base annuel (maksuvaba tulu) selon l'année.
/// 2025 : dégressif. 2026 : forfaitaire 8 400 €/an (réforme « tax hump »).
fn abattement_annuel(g: Decimal, annee: i32) -> Decimal {
    if annee >= 2026 {
        return dec!(8400);
    }
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

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "EE",
            "Estonie : données disponibles pour 2025 et 2026.", ctx);
    }

    let g = brut * dec!(12);

    let mut cotisations = vec![
        ligne_cot("EE_TOOTUS",        "Töötuskindlustusmakse — Chômage",   brut, ctx),
        ligne_cot("EE_KOGUMISPENSION","Kogumispension — Retraite 2ᵉ pilier", brut, ctx),
        ligne_cot("EE_SOTSIAALMAKS",  "Sotsiaalmaks — Charge sociale (employeur)", brut, ctx),
    ];

    // Base imposable : brut − cotisations salariales − abattement de base.
    let cot_sal_taux = ctx.taux_sal("EE_TOOTUS") + ctx.taux_sal("EE_KOGUMISPENSION");
    let abatt_an = abattement_annuel(g, annee);
    let base_imp_an = (g * (Decimal::ONE - cot_sal_taux) - abatt_an).max(Decimal::ZERO);
    let impot_mens = (base_imp_an * dec!(0.22) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "EE_TULUMAKS".into(),
        libelle: ctx.libelle("EE_TULUMAKS", "Tulumaks — Impôt sur le revenu (22 %)"),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("EE_TULUMAKS",
            "Impôt sur le revenu {annee} : 22 % (taux unique).\n\n\
            Revenu annuel {g} € − cotisations salariales − abattement de base {ab} €\n\
            = base imposable {b} € → {im} €/mois.\n\n\
            {abatt_txt}\n\
            Source : Maksu- ja Tolliamet.")
            .replace("{annee}", &annee.to_string())
            .replace("{g}", &format!("{:.0}", g))
            .replace("{ab}", &format!("{:.0}", abatt_an))
            .replace("{b}", &format!("{:.0}", base_imp_an))
            .replace("{im}", &format!("{:.2}", impot_mens))
            .replace("{abatt_txt}", if annee >= 2026 {
                "Abattement de base forfaitaire 8 400 €/an (700 €/mois), uniforme depuis 2026 (fin de la dégressivité)."
            } else {
                "Abattement de base dégressif (7 848 € si ≤ 14 400 €/an, nul si ≥ 25 200 €/an)."
            }),
        loi_ref: Some(ctx.loi_ref("Tulumaksuseadus")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(), absence: None, heures_sup: None, conges: None, salarie,
    }
}
