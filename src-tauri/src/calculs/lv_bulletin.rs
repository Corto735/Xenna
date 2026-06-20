// ── Lettonie — VSAOI (sécurité sociale) + IIN (impôt sur le revenu) ──────────────
//
// 2025 :
//   • VSAOI (valsts sociālās apdrošināšanas obligātās iemaksas) :
//     salarié 10,50 % / employeur 23,59 % (régime général).
//   • IIN (iedzīvotāju ienākuma nodoklis) : barème simplifié 2025
//     25,5 % jusqu'à 105 300 €/an (8 775 €/mois), 33 % au-delà.
//   • Minimum non imposable fixe 2025 : 510 €/mois.
//
// Simplification : minimum non imposable fixé (le dispositif différencié a été remplacé
// par un montant fixe en 2025) : 510 €/mois en 2025, 550 €/mois en 2026.
// Base IIN = brut − VSAOI salarié − minimum non imposable. Barème 25,5 % / 33 % inchangé
// (surtaxe +3 % au-delà de 200 000 €/an non modélisée — n'affecte que les très hauts revenus).
// Source : Valsts ieņēmumu dienests (VID) ; réforme IIN 2025 ; paramètres 2026.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn generer_bulletin_lv(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "EUR", "Lettonie : données disponibles pour 2025 et 2026.");
    }
    // Minimum non imposable mensuel : 510 € (2025), 550 € (2026).
    let min_non_imp = if annee >= 2026 { dec!(550) } else { dec!(510) };

    // VSAOI salarié + patronal (taux lus en base).
    let ts = ctx.taux_sal("LV_VSAOI");
    let tp = ctx.taux_pat("LV_VSAOI");
    let vsaoi_sal = (brut * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "LV_VSAOI".into(),
        libelle: ctx.libelle("LV_VSAOI", "VSAOI — Cotisations sociales obligatoires"),
        base: brut, taux_sal: ts, montant_sal: vsaoi_sal,
        taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("LV_VSAOI",
            "VSAOI — salarié {ts} % / employeur {tp} % (retraite, maladie, chômage, \
            maternité, accidents). Salarié : {ms} €.")
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
            .replace("{ms}", &format!("{:.2}", vsaoi_sal)),
        loi_ref: Some(ctx.loi_ref("Likums «Par valsts sociālo apdrošināšanu»")),
    }];

    // IIN : base = brut − VSAOI salarié − minimum non imposable ; 25,5 % jusqu'à 8 775 €/mois, 33 % au-delà.
    let base = (brut - vsaoi_sal - min_non_imp).max(Decimal::ZERO);
    let part_haute = (brut - dec!(8775)).max(Decimal::ZERO); // tranche à 33 %
    let part_basse = (base - part_haute).max(Decimal::ZERO);
    let iin = (part_basse * dec!(0.255) + part_haute * dec!(0.33)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (iin / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "LV_IIN".into(),
        libelle: ctx.libelle("LV_IIN", "IIN — Impôt sur le revenu"),
        base: brut, taux_sal: taux_imp, montant_sal: iin,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("LV_IIN",
            "Impôt sur le revenu {annee}.\n\n\
            Base = brut − VSAOI {vs} € − minimum non imposable {mni} € = {b} €\n\
            Taux 25,5 % (jusqu'à 8 775 €/mois) puis 33 % au-delà → {iin} €/mois.\n\n\
            Source : Valsts ieņēmumu dienests.")
            .replace("{annee}", &annee.to_string())
            .replace("{vs}", &format!("{:.2}", vsaoi_sal))
            .replace("{mni}", &format!("{:.0}", min_non_imp))
            .replace("{b}", &format!("{:.2}", base))
            .replace("{iin}", &format!("{:.2}", iin)),
        loi_ref: Some(ctx.loi_ref("Likums «Par iedzīvotāju ienākuma nodokli»")),
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
