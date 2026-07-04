// ── Slovénie — prispevki (cotisations) + dohodnina (impôt progressif) ────────────
//
// 2025 :
//   • Prispevki salarié 22,1 % (retraite/invalidité 15,5 % + maladie 6,36 %
//     + chômage 0,14 % + parental 0,10 %) / employeur 16,1 %.
//   • Dohodnina : barème progressif 2025 (16 / 26 / 33 / 39 / 50 %), assiette
//     = revenu après cotisations salariales et abattement général (5 000 €/an).
//
// 2026 : prispevki inchangés ; barème dohodnina indexé (seuils 9 721,43 / 28 592,44 /
// 57 184,88 / 82 346,23 €, taux 16/26/33/39/50 % inchangés) ; abattement général de
// base 5 551,93 €/an.
// Simplification : abattement général fixé (5 000 €/an en 2025, 5 551,93 € en 2026) ;
// l'abattement majoré dégressif pour bas revenus n'est pas modélisé (net prudent).
// Source : ZPIZ/ZZZS (prispevki) ; FURS (dohodnina 2025 et 2026).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Dohodnina annuelle (barème progressif, cumul par tranche) selon l'année.
fn dohodnina(t: Decimal, annee: i32) -> Decimal {
    if annee >= 2026 {
        return if t <= dec!(9721.43) {
            t * dec!(0.16)
        } else if t <= dec!(28592.44) {
            dec!(1555.43) + (t - dec!(9721.43)) * dec!(0.26)
        } else if t <= dec!(57184.88) {
            dec!(6461.89) + (t - dec!(28592.44)) * dec!(0.33)
        } else if t <= dec!(82346.23) {
            dec!(15897.40) + (t - dec!(57184.88)) * dec!(0.39)
        } else {
            dec!(25710.33) + (t - dec!(82346.23)) * dec!(0.50)
        };
    }
    if t <= dec!(9210.26) {
        t * dec!(0.16)
    } else if t <= dec!(27089.34) {
        dec!(1473.64) + (t - dec!(9210.26)) * dec!(0.26)
    } else if t <= dec!(54178.69) {
        dec!(6122.20) + (t - dec!(27089.34)) * dec!(0.33)
    } else if t <= dec!(78016.32) {
        dec!(15061.69) + (t - dec!(54178.69)) * dec!(0.39)
    } else {
        dec!(24358.37) + (t - dec!(78016.32)) * dec!(0.50)
    }
}

pub fn generer_bulletin_si(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "SI",
            "Slovénie : données disponibles pour 2025 et 2026.", ctx);
    }
    let abattement = if annee >= 2026 { dec!(5551.93) } else { dec!(5000) };

    let ts = ctx.taux_sal("SI_PRISPEVKI");
    let tp = ctx.taux_pat("SI_PRISPEVKI");
    let prisp_sal = (brut * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "SI_PRISPEVKI".into(),
        libelle: ctx.libelle("SI_PRISPEVKI", "Prispevki — Cotisations sociales"),
        base: brut, taux_sal: ts, montant_sal: prisp_sal,
        taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("SI_PRISPEVKI",
            "Prispevki — salarié {ts} % (retraite/invalidité 15,5 %, maladie 6,36 %, \
            chômage 0,14 %, parental 0,10 %) / employeur {tp} %. Salarié : {ms} €.")
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{ms}", &format!("{:.2}", prisp_sal)),
        loi_ref: Some(ctx.loi_ref("ZPIZ-2 / ZZVZZ")),
    }];

    // Dohodnina : base annuelle = (brut − cotisations salariales) × 12 − abattement général.
    let base_an = (((brut - prisp_sal).max(Decimal::ZERO)) * dec!(12) - abattement).max(Decimal::ZERO);
    let impot_mens = (dohodnina(base_an, annee) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "SI_DOHODNINA".into(),
        libelle: ctx.libelle("SI_DOHODNINA", "Dohodnina — Impôt sur le revenu"),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("SI_DOHODNINA",
            "Impôt sur le revenu {annee} (annualisé).\n\n\
            Base = (brut − cotisations) × 12 − abattement {ab} € = {b} €\n\
            Barème 16 / 26 / 33 / 39 / 50 % (seuils {seuils})\n\
            → {im} €/mois.\n\n\
            Note : abattement majoré pour bas revenus non modélisé (net prudent).\n\
            Source : FURS.")
            .replace("{annee}", &annee.to_string())
            .replace("{ab}", &format!("{:.0}", abattement))
            .replace("{b}", &format!("{:.0}", base_an))
            .replace("{im}", &format!("{:.2}", impot_mens))
            .replace("{seuils}", if annee >= 2026 {
                "9 721 / 28 592 / 57 185 / 82 346 €"
            } else {
                "9 210 / 27 089 / 54 179 / 78 016 €"
            }),
        loi_ref: Some(ctx.loi_ref("Zakon o dohodnini (ZDoh-2)")),
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
