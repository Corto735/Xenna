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
        libelle: "VSAOI — Cotisations sociales obligatoires".into(),
        base: brut, taux_sal: ts, montant_sal: vsaoi_sal,
        taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "VSAOI — salarié {ts:.2} % / employeur {tp:.2} % (retraite, maladie, chômage, \
            maternité, accidents). Salarié : {ms:.2} €.",
            ts = ts * dec!(100), tp = tp * dec!(100), ms = vsaoi_sal,
        ),
        loi_ref: Some("Likums «Par valsts sociālo apdrošināšanu»".into()),
    }];

    // IIN : base = brut − VSAOI salarié − minimum non imposable ; 25,5 % jusqu'à 8 775 €/mois, 33 % au-delà.
    let base = (brut - vsaoi_sal - min_non_imp).max(Decimal::ZERO);
    let part_haute = (brut - dec!(8775)).max(Decimal::ZERO); // tranche à 33 %
    let part_basse = (base - part_haute).max(Decimal::ZERO);
    let iin = (part_basse * dec!(0.255) + part_haute * dec!(0.33)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (iin / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "LV_IIN".into(),
        libelle: "IIN — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_imp, montant_sal: iin,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu {annee}.\n\n\
            Base = brut − VSAOI {vs:.2} € − minimum non imposable {mni:.0} € = {b:.2} €\n\
            Taux 25,5 % (jusqu'à 8 775 €/mois) puis 33 % au-delà → {iin:.2} €/mois.\n\n\
            Source : Valsts ieņēmumu dienests.",
            annee = annee, vs = vsaoi_sal, mni = min_non_imp, b = base, iin = iin,
        ),
        loi_ref: Some("Likums «Par iedzīvotāju ienākuma nodokli»".into()),
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
