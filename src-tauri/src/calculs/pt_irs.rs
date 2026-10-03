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
// Sources : CIRS art. 68 + Lei do OE annuelles (2015-2026) ; Lei 55-A/2025 pour 2025.

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
        // Depuis 2024 : 8,54 × IAS (CIRS art. 25) — IAS 509,26 / 522,50 / 537,13 €.
        2024            => dec!(4349.08),
        2025            => dec!(4462.15),
        _               => dec!(4587.09), // 2026 (Portaria 480-A/2025/1)
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
        2025 => {
            // 2025, barème rétroactif de la Lei 55-A/2025 (2ᵉ à 8ᵉ taux abaissés).
            tranches(rendimento,
                &[dec!(8059), dec!(12160), dec!(17233), dec!(22306), dec!(28400), dec!(41629), dec!(44987), dec!(83696)],
                &[dec!(0.125), dec!(0.16), dec!(0.215), dec!(0.244), dec!(0.314), dec!(0.349), dec!(0.431), dec!(0.446), dec!(0.48)])
        }
        _ => {
            // 2026+ (OE 2026, Lei 73-A/2025) — seuils relevés, 2ᵉ à 5ᵉ taux −0,3 point.
            tranches(rendimento,
                &[dec!(8342), dec!(12587), dec!(17838), dec!(23089), dec!(29397), dec!(43090), dec!(46566), dec!(86634)],
                &[dec!(0.125), dec!(0.157), dec!(0.212), dec!(0.241), dec!(0.311), dec!(0.349), dec!(0.431), dec!(0.446), dec!(0.48)])
        }
    }
}

/// Impôt progressif : `seuils` = bornes hautes des tranches, `taux` = une de plus.
fn tranches(revenu: Decimal, seuils: &[Decimal], taux: &[Decimal]) -> Decimal {
    let mut impot = Decimal::ZERO;
    let mut bas = Decimal::ZERO;
    for (i, t) in taux.iter().enumerate() {
        let haut = seuils.get(i).copied().unwrap_or(Decimal::MAX);
        if revenu > bas {
            impot += (revenu.min(haut) - bas) * t;
        }
        bas = haut;
    }
    impot
}

// ── Tables officielles de retenue (depuis 2026) ──────────────────────────────
//
// Table I du Continent, travail salarié, non marié sans personne à charge (ou
// marié deux titulaires) — Despacho n.º 233-A/2026. Retenue = R × taux −
// parcela a abater ; pour les deux premières tranches, la parcela vaut
// taux × k × (C − R). Renvoie (taux, parcela) pour la rémunération mensuelle R.
fn tabela_i_2026(r: Decimal) -> (Decimal, Decimal) {
    let lignes: [(Decimal, Decimal, Decimal); 11] = [
        (dec!(1154),  dec!(0.157),  dec!(94.71)),
        (dec!(1212),  dec!(0.212),  dec!(158.18)),
        (dec!(1819),  dec!(0.241),  dec!(193.33)),
        (dec!(2119),  dec!(0.311),  dec!(320.66)),
        (dec!(2499),  dec!(0.349),  dec!(401.19)),
        (dec!(3305),  dec!(0.3836), dec!(487.66)),
        (dec!(5547),  dec!(0.3969), dec!(531.62)),
        (dec!(20221), dec!(0.4495), dec!(823.40)),
        (Decimal::MAX, dec!(0.4717), dec!(1272.31)),
        (Decimal::ZERO, Decimal::ZERO, Decimal::ZERO),
        (Decimal::ZERO, Decimal::ZERO, Decimal::ZERO),
    ];
    if r <= dec!(920) {
        return (Decimal::ZERO, Decimal::ZERO);
    }
    if r <= dec!(1042) {
        return (dec!(0.125), dec!(0.125) * dec!(2.60) * (dec!(1273.85) - r));
    }
    if r <= dec!(1108) {
        return (dec!(0.157), dec!(0.157) * dec!(1.35) * (dec!(1554.83) - r));
    }
    let (_, taux, parcela) = lignes.iter().find(|(plafond, _, _)| r <= *plafond).copied()
        .unwrap_or((Decimal::MAX, dec!(0.4717), dec!(1272.31)));
    (taux, parcela)
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
    let irs_estime    = (irs_anual / dec!(12)).round_dp(2);
    // Depuis 2026, la retenue suit la table officielle ; le calcul annualisé
    // reste affiché à titre indicatif.
    let tabela = (annee >= 2026).then(|| tabela_i_2026(brut));
    let irs_mensal = match tabela {
        Some((taux, parcela)) => (brut * taux - parcela).max(Decimal::ZERO).round_dp(2),
        None => irs_estime,
    };

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
            Base légale : CIRS art. 99 + Tables AT {annee}.{tabela}")
            .replace("{tabela}", &match tabela {
                Some((taux, parcela)) => ctx.expl("PT_IRS_TABELA",
                    "\nRetenue appliquée : table officielle I du Continent (non marié sans \
                    personne à charge, Despacho n.º 233-A/2026) : {brut} × {taux} % − {parcela} = \
                    {ret} €. Le calcul annualisé ci-dessus est indicatif.")
                    .replace("{brut}", &format!("{:.2}", brut))
                    .replace("{taux}", &format!("{:.2}", taux * dec!(100)))
                    .replace("{parcela}", &format!("{:.2}", parcela))
                    .replace("{ret}", &format!("{:.2}", irs_mensal)),
                None => String::new(),
            })
            .replace("{annee}", &annee.to_string())
            .replace("{nb_tr}", &nb_tranches.to_string())
            .replace("{brut}", &format!("{:.2}", brut))
            .replace("{rend_a}", &format!("{:.2}", rendimento_a))
            .replace("{ded}", &format!("{:.2}", deducao))
            .replace("{ss}", &format!("{:.2}", ss_annuel))
            .replace("{df}", &format!("{:.2}", deducao_min))
            .replace("{base_irs}", &format!("{:.2}", base_irs))
            .replace("{irs_a}", &format!("{:.2}", irs_anual))
            .replace("{irs_m}", &format!("{:.2}", irs_estime))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("CIRS art. 68 (barème) + art. 99 (retenção) — Lei OE {annee}")
                        .replace("{annee}", &annee.to_string())),
    }
}
