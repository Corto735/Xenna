// Impôt sur le revenu indien (TDS mensuel) — nouveau régime (défaut, sec. 115BAC)
// et ancien régime. Barèmes annuels FY 2025-26 (AY 2026-27), déduction standard,
// rebate 87A, cess santé & éducation 4 %. Célibataire, sans déduction 80C
// (ancien régime : seule la déduction standard est appliquée — lacune assumée).
//
// Source : Income-tax Act 1961 ; Finance Act 2025 (barèmes FY 2025-26).

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Résultat impôt : (impôt mensuel, impôt annuel avec cess, régime normalisé,
/// taux marginal appliqué, revenu imposable annuel).
pub struct ResultImpot {
    pub mensuel: Decimal,
    pub annuel:  Decimal,
    pub regime:  &'static str,
    pub marginal: Decimal,
    pub imposable: Decimal,
}

/// Tranches annuelles (limite supérieure, taux). Dernière borne = ∞ (i64::MAX cast).
fn slabs(nouveau: bool) -> Vec<(Decimal, Decimal)> {
    if nouveau {
        vec![
            (dec!(400000),  dec!(0.00)),
            (dec!(800000),  dec!(0.05)),
            (dec!(1200000), dec!(0.10)),
            (dec!(1600000), dec!(0.15)),
            (dec!(2000000), dec!(0.20)),
            (dec!(2400000), dec!(0.25)),
            (dec!(99999999999), dec!(0.30)),
        ]
    } else {
        vec![
            (dec!(250000),  dec!(0.00)),
            (dec!(500000),  dec!(0.05)),
            (dec!(1000000), dec!(0.20)),
            (dec!(99999999999), dec!(0.30)),
        ]
    }
}

pub fn calcul_impot(brut_mensuel: Decimal, regime: &str, _annee: i32) -> ResultImpot {
    let nouveau = regime != "ancien";
    let annuel_brut = brut_mensuel * dec!(12);

    // Déduction standard + seuil de rebate 87A (revenu imposable en deçà → impôt nul).
    let (std_ded, rebate_seuil) = if nouveau {
        (dec!(75000), dec!(1200000))
    } else {
        (dec!(50000), dec!(500000))
    };

    let imposable = (annuel_brut - std_ded).max(Decimal::ZERO);

    let mut impot = Decimal::ZERO;
    let mut marginal = Decimal::ZERO;
    if imposable > rebate_seuil {
        let mut bas = Decimal::ZERO;
        for (haut, taux) in slabs(nouveau) {
            let assiette = imposable.min(haut) - bas;
            if assiette > Decimal::ZERO {
                impot += assiette * taux;
                if taux > Decimal::ZERO { marginal = taux; }
            }
            bas = haut;
            if imposable <= haut { break; }
        }
    }

    // Cess santé & éducation 4 % sur l'impôt.
    let annuel = (impot * dec!(1.04)).round_dp(2);
    let mensuel = (annuel / dec!(12)).round_dp(2);

    ResultImpot {
        mensuel,
        annuel,
        regime: if nouveau { "nouveau" } else { "ancien" },
        marginal,
        imposable: imposable.round_dp(2),
    }
}
