// ── IRS Portugal — Retenção na Fonte ─────────────────────────────────────────
//
// Méthode : annualisation (brut × 12), déduction spécifique emploi,
//           barème progressif CIRS art. 68, division par 12.
//
// Note : les tables officielles de retenção na fonte (AT) donnent un taux
// effectif par tranche de salaire mensuel et situation familiale.
// Le calcul par barème annualisé est une approximation utilisée pour la
// simulation (même approche que IT_IRPEF dans ce projet).
//
// Sources : CIRS art. 68 + Lei do OE annuelles (2015-2025).

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Déduction spécifique emploi (dedução específica, CIRS art. 25) ───────────
//
// Réduit la base imposable. Correspond aux cotisations SS ou à un montant
// forfaitaire, le plus élevé des deux étant retenu.
// Simplification : on utilise le minimum légal (forfait annuel).
fn deducao_especifica(annee: i32) -> Decimal {
    match annee {
        i32::MIN..=2022 => dec!(4104),
        2023            => dec!(4208),
        2024            => dec!(4462),
        _               => dec!(4718), // 2025+
    }
}

// ── Barème annuel IRS (CIRS art. 68) ─────────────────────────────────────────
//
// Calcule l'IRS annuelle brute sur le revenu imposable annuel.
// Les seuils de tranche sont en euros annuels.
pub fn irs_annuel(rendimento: Decimal, annee: i32) -> Decimal {
    match annee {
        i32::MIN..=2015 => {
            // OE 2015 (Lei 82-B/2014) — 5 tranches
            if rendimento <= dec!(7000) {
                rendimento * dec!(0.1450)
            } else if rendimento <= dec!(20000) {
                dec!(1015.00) + (rendimento - dec!(7000)) * dec!(0.2850)
            } else if rendimento <= dec!(40000) {
                dec!(4720.00) + (rendimento - dec!(20000)) * dec!(0.3700)
            } else if rendimento <= dec!(80000) {
                dec!(12120.00) + (rendimento - dec!(40000)) * dec!(0.4500)
            } else {
                dec!(30120.00) + (rendimento - dec!(80000)) * dec!(0.4800)
            }
        }
        2016 => {
            // OE 2016 (Lei 7-A/2016) — 5 tranches, seuils légèrement ajustés
            if rendimento <= dec!(7035) {
                rendimento * dec!(0.1450)
            } else if rendimento <= dec!(20000) {
                dec!(1020.08) + (rendimento - dec!(7035)) * dec!(0.2850)
            } else if rendimento <= dec!(40000) {
                dec!(4715.21) + (rendimento - dec!(20000)) * dec!(0.3700)
            } else if rendimento <= dec!(80000) {
                dec!(12115.21) + (rendimento - dec!(40000)) * dec!(0.4500)
            } else {
                dec!(30115.21) + (rendimento - dec!(80000)) * dec!(0.4800)
            }
        }
        2017 => {
            // OE 2017 (Lei 42/2016) — 5 tranches
            if rendimento <= dec!(7091) {
                rendimento * dec!(0.1450)
            } else if rendimento <= dec!(20261) {
                dec!(1028.20) + (rendimento - dec!(7091)) * dec!(0.2850)
            } else if rendimento <= dec!(40522) {
                dec!(4781.15) + (rendimento - dec!(20261)) * dec!(0.3700)
            } else if rendimento <= dec!(80640) {
                dec!(12277.72) + (rendimento - dec!(40522)) * dec!(0.4500)
            } else {
                dec!(30331.32) + (rendimento - dec!(80640)) * dec!(0.4800)
            }
        }
        2018 | 2019 => {
            // OE 2018 (Lei 114/2017) + OE 2019 (Lei 71/2018) — 7 tranches
            if rendimento <= dec!(7091) {
                rendimento * dec!(0.1450)
            } else if rendimento <= dec!(10700) {
                dec!(1028.20) + (rendimento - dec!(7091)) * dec!(0.2300)
            } else if rendimento <= dec!(20261) {
                dec!(1858.27) + (rendimento - dec!(10700)) * dec!(0.2850)
            } else if rendimento <= dec!(25000) {
                dec!(4583.16) + (rendimento - dec!(20261)) * dec!(0.3500)
            } else if rendimento <= dec!(36856) {
                dec!(6241.81) + (rendimento - dec!(25000)) * dec!(0.3700)
            } else if rendimento <= dec!(80640) {
                dec!(10628.53) + (rendimento - dec!(36856)) * dec!(0.4500)
            } else {
                dec!(30330.33) + (rendimento - dec!(80640)) * dec!(0.4800)
            }
        }
        2020 | 2021 => {
            // OE 2020 (Lei 2/2020) + OE 2021 (Lei 75-B/2020) — 7 tranches
            if rendimento <= dec!(7112) {
                rendimento * dec!(0.1450)
            } else if rendimento <= dec!(10732) {
                dec!(1031.24) + (rendimento - dec!(7112)) * dec!(0.2300)
            } else if rendimento <= dec!(20322) {
                dec!(1863.84) + (rendimento - dec!(10732)) * dec!(0.2850)
            } else if rendimento <= dec!(25075) {
                dec!(4597.99) + (rendimento - dec!(20322)) * dec!(0.3500)
            } else if rendimento <= dec!(36967) {
                dec!(6261.54) + (rendimento - dec!(25075)) * dec!(0.3700)
            } else if rendimento <= dec!(80882) {
                dec!(10661.58) + (rendimento - dec!(36967)) * dec!(0.4500)
            } else {
                dec!(30423.33) + (rendimento - dec!(80882)) * dec!(0.4800)
            }
        }
        2022 => {
            // OE 2022 (Lei 12/2022) — 7 tranches
            if rendimento <= dec!(7116) {
                rendimento * dec!(0.1450)
            } else if rendimento <= dec!(10736) {
                dec!(1031.82) + (rendimento - dec!(7116)) * dec!(0.2300)
            } else if rendimento <= dec!(20322) {
                dec!(1864.42) + (rendimento - dec!(10736)) * dec!(0.2850)
            } else if rendimento <= dec!(25075) {
                dec!(4595.92) + (rendimento - dec!(20322)) * dec!(0.3500)
            } else if rendimento <= dec!(36967) {
                dec!(6259.47) + (rendimento - dec!(25075)) * dec!(0.3700)
            } else if rendimento <= dec!(80882) {
                dec!(10659.51) + (rendimento - dec!(36967)) * dec!(0.4500)
            } else {
                dec!(30421.26) + (rendimento - dec!(80882)) * dec!(0.4800)
            }
        }
        2023 => {
            // OE 2023 (Lei 24-D/2022) — 9 tranches (réforme majeure)
            if rendimento <= dec!(7479) {
                rendimento * dec!(0.1325)
            } else if rendimento <= dec!(11284) {
                dec!(990.97) + (rendimento - dec!(7479)) * dec!(0.1800)
            } else if rendimento <= dec!(15992) {
                dec!(1675.87) + (rendimento - dec!(11284)) * dec!(0.2300)
            } else if rendimento <= dec!(20700) {
                dec!(2758.71) + (rendimento - dec!(15992)) * dec!(0.2600)
            } else if rendimento <= dec!(26355) {
                dec!(3982.79) + (rendimento - dec!(20700)) * dec!(0.3275)
            } else if rendimento <= dec!(38632) {
                dec!(5834.80) + (rendimento - dec!(26355)) * dec!(0.3700)
            } else if rendimento <= dec!(50483) {
                dec!(10377.29) + (rendimento - dec!(38632)) * dec!(0.4350)
            } else if rendimento <= dec!(78834) {
                dec!(15532.48) + (rendimento - dec!(50483)) * dec!(0.4500)
            } else {
                dec!(28290.43) + (rendimento - dec!(78834)) * dec!(0.4800)
            }
        }
        2024 => {
            // OE 2024 (Lei 24/2023) — 8 tranches
            if rendimento <= dec!(7703) {
                rendimento * dec!(0.1325)
            } else if rendimento <= dec!(11623) {
                dec!(1020.65) + (rendimento - dec!(7703)) * dec!(0.1800)
            } else if rendimento <= dec!(16472) {
                dec!(1726.25) + (rendimento - dec!(11623)) * dec!(0.2300)
            } else if rendimento <= dec!(22000) {
                dec!(2841.52) + (rendimento - dec!(16472)) * dec!(0.2600)
            } else if rendimento <= dec!(28000) {
                dec!(4278.80) + (rendimento - dec!(22000)) * dec!(0.3275)
            } else if rendimento <= dec!(40000) {
                dec!(6243.80) + (rendimento - dec!(28000)) * dec!(0.3700)
            } else if rendimento <= dec!(80000) {
                dec!(10683.80) + (rendimento - dec!(40000)) * dec!(0.4350)
            } else {
                dec!(28083.80) + (rendimento - dec!(80000)) * dec!(0.4800)
            }
        }
        _ => {
            // 2025+ (OE 2025, Lei 24-D/2024) — 9 tranches
            if rendimento <= dec!(8059) {
                rendimento * dec!(0.1300)
            } else if rendimento <= dec!(12160) {
                dec!(1047.67) + (rendimento - dec!(8059)) * dec!(0.1650)
            } else if rendimento <= dec!(17233) {
                dec!(1724.34) + (rendimento - dec!(12160)) * dec!(0.2200)
            } else if rendimento <= dec!(22306) {
                dec!(2840.40) + (rendimento - dec!(17233)) * dec!(0.2500)
            } else if rendimento <= dec!(28400) {
                dec!(4108.65) + (rendimento - dec!(22306)) * dec!(0.3200)
            } else if rendimento <= dec!(41629) {
                dec!(6058.73) + (rendimento - dec!(28400)) * dec!(0.3550)
            } else if rendimento <= dec!(44987) {
                dec!(10755.02) + (rendimento - dec!(41629)) * dec!(0.4350)
            } else if rendimento <= dec!(83696) {
                dec!(12215.75) + (rendimento - dec!(44987)) * dec!(0.4500)
            } else {
                dec!(29634.80) + (rendimento - dec!(83696)) * dec!(0.4800)
            }
        }
    }
}

// ── Retenção na fonte mensuelle ───────────────────────────────────────────────

pub fn irs_retencao(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee        = ctx.date_paie.year();
    let rendimento_a = brut * dec!(12);

    // Dedução específica : max(SS annuel, forfait légal)
    let ss_annuel     = (rendimento_a * dec!(0.11)).round_dp(2);
    let deducao_min   = deducao_especifica(annee);
    let deducao       = ss_annuel.max(deducao_min);

    let base_irs      = (rendimento_a - deducao).max(Decimal::ZERO);
    let irs_anual     = irs_annuel(base_irs, annee);
    let irs_mensal    = (irs_anual / dec!(12)).round_dp(2);

    let taux_eff = if brut > Decimal::ZERO {
        (irs_mensal / brut).round_dp(4)
    } else {
        Decimal::ZERO
    };

    let nb_tranches = match annee {
        i32::MIN..=2017 => 5,
        2018 | 2019     => 7,
        2020..=2022     => 7,
        2023            => 9,
        2024            => 8,
        _               => 9,
    };

    LigneCotisation {
        code:        "PT_IRS".into(),
        libelle:     ctx.libelle("PT_IRS", "IRS — Retenção na Fonte {annee}")
                        .replace("{annee}", &annee.to_string()),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: irs_mensal,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: ctx.expl("PT_IRS",
            "Retenue mensuelle à la source (retenção na fonte) de l'IRS \
            (Imposto sobre o Rendimento das Pessoas Singulares). \
            L'employeur (substituto tributário) retient chaque mois une avance \
            sur l'IRS annuel. Régularisation lors de la déclaration Modelo 3 (avril). \
            \n\n\
            [ Calcul {annee} — barème CIRS art. 68, {nb_tr} tranches ]\n\
            Rendimento mensal bruto    : {brut} €\n\
            Rendimento anual estimado  : {rend_a} € (× 12)\n\
            Dedução específica (art.25): − {ded} € (max(SS {ss} €, forfait {df} €))\n\
            Base imposable annuelle    : {base_irs} €\n\
            IRS annuelle               : {irs_a} €\n\
            Retenção mensuelle         : {irs_m} € (÷ 12)\n\
            Taux effectif              : {teff} %\n\
            \n\
            Note : le calcul par barème annualisé est une approximation. \
            Les tables officielles AT (tabelas de retenção na fonte) sont publiées \
            annuellement et tiennent compte de la situation familiale. \
            Base légale : CIRS art. 99 + Tables AT {annee}.")
            .replace("{annee}", &annee.to_string())
            .replace("{nb_tr}", &nb_tranches.to_string())
            .replace("{brut}", &format!("{:.2}", brut))
            .replace("{rend_a}", &format!("{:.2}", rendimento_a))
            .replace("{ded}", &format!("{:.2}", deducao))
            .replace("{ss}", &format!("{:.2}", ss_annuel))
            .replace("{df}", &format!("{:.2}", deducao_min))
            .replace("{base_irs}", &format!("{:.2}", base_irs))
            .replace("{irs_a}", &format!("{:.2}", irs_anual))
            .replace("{irs_m}", &format!("{:.2}", irs_mensal))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("CIRS art. 68 (barème) + art. 99 (retenção) — Lei OE {annee}")
                        .replace("{annee}", &annee.to_string())),
    }
}
