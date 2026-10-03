// ── Finlande — cotisations + impôt d'État + impôt communal ───────────────────
//
// Salarié secteur privé (17-68 ans). Côté salarié :
//   • TyEL 7,30 % + chômage 0,89 % + päivärahamaksu 0,88 % (si revenu ≥ 17 255 €)
//     → déductibles du revenu imposable ;
//   • sairaanhoitomaksu 1,10 % (non déductible) ;
//   • impôt d'État progressif (5 tranches) + impôt communal moyen 7,50 %.
//
// Simplification documentée : les crédits d'impôt sur les revenus du travail
// (työtulovähennys, perusvähennys, ansiotulovähennys) NE sont PAS modélisés → le net
// affiché est prudent (impôt légèrement surestimé). Taux de cotisation lus en base.
// Sources : Tuloverolaki ; TyEL ; Sairausvakuutuslaki ; Työttömyysetuuksien rahoituslaki (2026).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Impôt d'État annuel, barème 2026 (Tuloverolaki ; VM, tuloveroasteikko 2026) :
/// 12,64 % jusqu'à 22 000 €, puis 19 / 30,25 / 33,25 / 37,5 %.
fn impot_etat(t: Decimal) -> Decimal {
    if t <= dec!(22000) {
        t * dec!(0.1264)
    } else if t <= dec!(32600) {
        dec!(2780.80) + (t - dec!(22000)) * dec!(0.19)
    } else if t <= dec!(40100) {
        dec!(4794.80) + (t - dec!(32600)) * dec!(0.3025)
    } else if t <= dec!(52100) {
        dec!(7063.55) + (t - dec!(40100)) * dec!(0.3325)
    } else {
        dec!(11053.55) + (t - dec!(52100)) * dec!(0.375)
    }
}

/// Crédit d'impôt sur le revenu du travail (työtulovähennys) 2026 : 18 % du
/// salaire, plafonné à 3 430 €, réduit de 2 % du revenu net au-delà de 35 000 €
/// (Veronmaksajat, vähennykset 2026). Au-delà de 50 550 €, réduction non
/// relevée : on poursuit à 2 %.
fn tyotulovahennys(salaire: Decimal, revenu_net: Decimal) -> Decimal {
    let plein = (salaire * dec!(0.18)).min(dec!(3430));
    (plein - (revenu_net - dec!(35000)).max(Decimal::ZERO) * dec!(0.02)).max(Decimal::ZERO)
}

/// Abattement de base communal (perusvähennys) 2026 : 4 265 €, réduit de 18 %
/// du revenu communal imposable au-delà de 4 265 €.
fn perusvahennys(revenu: Decimal) -> Decimal {
    (dec!(4265) - (revenu - dec!(4265)).max(Decimal::ZERO) * dec!(0.18)).max(Decimal::ZERO).min(revenu)
}

fn ligne_cot(code: &str, libelle: &str, base: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    let lib = ctx.libelle(code, libelle);
    let explication = ctx.expl("FI_GENERIC",
        "{libelle}. Salarié {ts} % / employeur {tp} %. Salarié : {ms} €.")
        .replace("{libelle}", &lib)
        .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
        .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
        .replace("{ms}", &format!("{:.2}", (base * ts).round_dp(2)));
    LigneCotisation {
        code: code.into(), libelle: lib, base,
        taux_sal: ts, montant_sal: (base * ts).round_dp(2),
        taux_pat: tp, montant_pat: (base * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("Lainsäädäntö (Finlande)")),
    }
}

pub fn generer_bulletin_fi(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee != 2026 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "FI",
            "Finlande : données disponibles pour 2026.", ctx);
    }

    let g = brut * dec!(12);

    let mut cotisations = vec![
        ligne_cot("FI_TYEL",         "TyEL — Retraite",            brut, ctx),
        ligne_cot("FI_TYOTTOMYYS",   "Työttömyysvakuutus — Chômage", brut, ctx),
    ];

    // Päivärahamaksu : uniquement si revenu annuel ≥ 17 255 €
    let paiva_ts = ctx.taux_sal("FI_PAIVARAHA");
    let paiva_montant = if g >= dec!(17255) { (brut * paiva_ts).round_dp(2) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "FI_PAIVARAHA".into(), libelle: ctx.libelle("FI_PAIVARAHA", "Päivärahamaksu — Indemnités journalières"),
        base: brut, taux_sal: if g >= dec!(17255) { paiva_ts } else { Decimal::ZERO },
        montant_sal: paiva_montant, taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("FI_PAIVARAHA",
            "Päivärahamaksu — 0,88 % (uniquement si revenu annuel ≥ 17 255 €). Déductible.\n\
            Revenu annuel : {g} € → {m} €/mois.")
            .replace("{g}", &format!("{:.0}", g))
            .replace("{m}", &format!("{:.2}", paiva_montant)),
        loi_ref: Some(ctx.loi_ref("Sairausvakuutuslaki")),
    });

    cotisations.push(ligne_cot("FI_SAIRAANHOITO", "Sairaanhoitomaksu — Soins de santé", brut, ctx));
    cotisations.push(ligne_cot("FI_TYONANTAJA_SV", "Sairausvakuutus employeur", brut, ctx));

    // Revenu net : salaire − déduction forfaitaire de frais (750 €) − cotisations
    // salariales déductibles (TyEL, chômage, päiväraha).
    let ded_annuel = dec!(750).min(g)
        + (ctx.taux_sal("FI_TYEL") + ctx.taux_sal("FI_TYOTTOMYYS")) * g
        + paiva_montant * dec!(12);
    let taxable = (g - ded_annuel).max(Decimal::ZERO);

    let etat = impot_etat(taxable);
    // Impôt communal moyen 2026 : 7,57 %, après perusvähennys.
    let communal = (taxable - perusvahennys(taxable)).max(Decimal::ZERO) * dec!(0.0757);
    let credit = tyotulovahennys(g, taxable).min(etat + communal);
    let impot_mens = ((etat + communal - credit) / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "FI_TULOVERO".into(), libelle: ctx.libelle("FI_TULOVERO", "Tulovero — Impôt (État + communal)"),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("FI_TULOVERO",
            "Impôt sur le revenu 2026 (annualisé).\n\n\
            Revenu imposable : {g} € − frais (750 €) et cotisations déductibles = {ded} € → {tx} €\n\
            Barème d'État : 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n\
            (seuils 22 000 / 32 600 / 40 100 / 52 100 €) → {et} €\n\
            Impôt communal moyen 7,57 % (après perusvähennys) → {co} €\n\
            − työtulovähennys {cr} €\n\
            = {im} €/mois.\n\n\
            Base légale : Tuloverolaki.")
            .replace("{g}", &format!("{:.0}", g))
            .replace("{ded}", &format!("{:.0}", ded_annuel))
            .replace("{tx}", &format!("{:.0}", taxable))
            .replace("{et}", &format!("{:.0}", etat))
            .replace("{co}", &format!("{:.0}", communal))
            .replace("{cr}", &format!("{:.0}", credit))
            .replace("{im}", &format!("{:.2}", impot_mens)),
        loi_ref: Some(ctx.loi_ref("Tuloverolaki")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(), absence: None, heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), salarie,
    }
}
