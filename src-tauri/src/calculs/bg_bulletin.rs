// ── Bulgarie — cotisations sociales (plafonnées) + impôt 10 % ────────────────────
//
// 2025 :
//   • Cotisations sociales salarié ≈ 13,78 % (retraite DOO + maladie NZOK 3,2 %
//     + 2ᵉ pilier UPF 2,2 %) / employeur ≈ 18,92 %, sur l'assiette plafonnée à
//     3 750 BGN/mois (revenu maximal assurable 2025).
//   • Impôt sur le revenu : 10 % proportionnel, sur le revenu après cotisations.
//
// Devise : lev (BGN) en 2025 ; l'euro est adopté au 01/01/2026 (1 € = 1,95583 BGN).
// 2026 : taux inchangés (13,78 % / 18,92 %, la hausse de 2 points des retraites n'ayant
// pas été votée) ; plafond d'assurance 4 130 BGN converti = 2 111,64 €/mois ; impôt 10 %.
// Source : НАП (NRA) ; НОИ (NOI) ; barème 2025 et 2026 (adoption de l'euro).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn generer_bulletin_bg(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "BGN", "Bulgarie : données disponibles pour 2025 et 2026.");
    }

    // Euro adopté au 01/01/2026 ; plafond d'assurance converti.
    let euro = annee >= 2026;
    let devise = if euro { "EUR" } else { "BGN" };
    let sym = if euro { "€" } else { "BGN" };
    let plafond = if euro { dec!(2111.64) } else { dec!(3750) };
    // Cotisations sur assiette plafonnée.
    let assiette = brut.min(plafond);
    let ts = ctx.taux_sal("BG_OSIG");
    let tp = ctx.taux_pat("BG_OSIG");
    let osig_sal = (assiette * ts).round_dp(2);
    let mut cotisations = vec![LigneCotisation {
        code: "BG_OSIG".into(),
        libelle: "Осигуровки — Cotisations sociales".into(),
        base: assiette, taux_sal: ts, montant_sal: osig_sal,
        taux_pat: tp, montant_pat: (assiette * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "Cotisations sociales — salarié {ts:.2} % / employeur {tp:.2} % (retraite, \
            maladie NZOK, 2ᵉ pilier). Assiette plafonnée à {pl:.2} {sym}/mois. Salarié : {ms:.2} {sym}.",
            ts = ts * dec!(100), tp = tp * dec!(100), pl = plafond, sym = sym, ms = osig_sal,
        ),
        loi_ref: Some("Кодекс за социално осигуряване".into()),
    }];

    // Impôt : base = brut − cotisations salariales ; 10 %.
    let base = (brut - osig_sal).max(Decimal::ZERO);
    let impot = (base * dec!(0.10)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "BG_DANAK".into(),
        libelle: "Данък върху доходите — Impôt sur le revenu (10 %)".into(),
        base: brut, taux_sal: taux_imp, montant_sal: impot,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu {annee} : 10 % proportionnel.\n\n\
            Base = brut − cotisations salariales = {b:.2} {sym} → {im:.2} {sym}/mois.\n\n\
            Source : НАП (NRA).",
            annee = annee, b = base, sym = sym, im = impot,
        ),
        loi_ref: Some("Закон за данъците върху доходите на физическите лица".into()),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: devise.into(), absence: None, heures_sup: None, salarie,
    }
}
