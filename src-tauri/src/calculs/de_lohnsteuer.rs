use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Grundfreibetrag annuel (EStG §32a) ────────────────────────────────────────

fn grundfreibetrag(annee: i32) -> Decimal {
    match annee {
        i32::MIN..=2015 => dec!(8472),
        2016            => dec!(8652),
        2017            => dec!(8820),
        2018            => dec!(9000),
        2019            => dec!(9168),
        2020            => dec!(9408),
        2021            => dec!(9744),
        2022            => dec!(10347),
        2023            => dec!(10908),
        2024            => dec!(11784), // relevé rétroactivement (loi du 23/12/2024)
        2025            => dec!(12096),
        _               => dec!(12348), // 2026 — §32a EStG en vigueur
    }
}

// ── Entlastungsbetrag für Alleinerziehende (Steuerklasse II) ─────────────────

fn entlastungsbetrag(annee: i32) -> Decimal {
    // EStG §24b — revalorisation annuelle
    match annee {
        i32::MIN..=2019 => dec!(1908),
        2020 | 2021     => dec!(4008), // doublement temporaire COVID
        2022            => dec!(4008),
        2023            => dec!(4260),
        2024            => dec!(4260),
        _               => dec!(4260), // 2025-2026 : stable
    }
}

// ── Barème Einkommensteuer / Lohnsteuer (EStG §32a) ──────────────────────────
//
// Zones de progression (VZ 2026 — Grundfreibetrag = 12 348 €) :
//   Zone 0 : 0 € → 12 348 € → 0 %
//   Zone 1 : 12 349 € → 17 799 € → (914,51·y + 1 400)·y      (14 % → 24 %)
//   Zone 2 : 17 800 € → 69 878 € → (173,10·z + 2 397)·z + 1 034,87 (24 % → 42 %)
//   Zone 3 : 69 879 € → 277 825 € → 0,42·x − 11 135,63
//   Zone 4 : > 277 825 € → 0,45·x − 19 470,38
// Le tarif s'applique au revenu imposable (zvE), pas au brut.

/// Tarif exact du §32a al. 1 EStG sur le revenu imposable `x` (arrondi à l'euro
/// inférieur), 2023-2026 : (Grundfreibetrag, fin zone 1, fin zone 2,
/// coef. zone 1, coef. zone 2, constante zone 2, abattement 42 %, abattement 45 %).
/// Sources : gesetze-im-internet.de (VZ 2026), buzer.de (VZ 2024), EStH 2025.
fn tarif_32a_exact(x: Decimal, annee: i32) -> Option<Decimal> {
    let (gbf, z1, z2, a1, a2, c2, k42, k45) = match annee {
        2023 => (dec!(10908), dec!(15999), dec!(62809), dec!(979.18), dec!(192.59), dec!(966.53),  dec!(9972.98),  dec!(18307.73)),
        2024 => (dec!(11784), dec!(17005), dec!(66760), dec!(954.80), dec!(181.19), dec!(991.21),  dec!(10636.31), dec!(18971.06)),
        2025 => (dec!(12096), dec!(17443), dec!(68480), dec!(932.30), dec!(176.64), dec!(1015.13), dec!(10911.92), dec!(19246.67)),
        2026.. => (dec!(12348), dec!(17799), dec!(69878), dec!(914.51), dec!(173.10), dec!(1034.87), dec!(11135.63), dec!(19470.38)),
        _ => return None,
    };
    let x = x.floor();
    let st = if x <= gbf {
        Decimal::ZERO
    } else if x <= z1 {
        let y = (x - gbf) / dec!(10000);
        (a1 * y + dec!(1400)) * y
    } else if x <= z2 {
        let z = (x - z1) / dec!(10000);
        (a2 * z + dec!(2397)) * z + c2
    } else if x <= dec!(277825) {
        dec!(0.42) * x - k42
    } else {
        dec!(0.45) * x - k45
    };
    Some(st.max(Decimal::ZERO).floor())
}

/// Impôt sur le revenu imposable `x` : tarif exact dès 2023, approximation
/// historique (zones à coefficients fixes) avant.
fn einkommensteuer_annuel(revenu: Decimal, annee: i32) -> Decimal {
    tarif_32a_exact(revenu, annee).unwrap_or_else(|| einkommensteuer_ancien(revenu, annee))
}

/// Approximation antérieure à 2023 (coefficients 2023 appliqués à des seuils datés).
fn einkommensteuer_ancien(revenu: Decimal, annee: i32) -> Decimal {
    if revenu <= Decimal::ZERO {
        return Decimal::ZERO;
    }

    // Seuils des zones selon l'année (EStG §32a — Progressionszonen)
    let (z1_debut, z1_fin, z2_fin): (Decimal, Decimal, Decimal) = match annee {
        i32::MIN..=2015 => (dec!(8473),  dec!(13469),  dec!(52881)),
        2016            => (dec!(8653),  dec!(13669),  dec!(53665)),
        2017            => (dec!(8821),  dec!(13769),  dec!(54057)),
        2018            => (dec!(9001),  dec!(13996),  dec!(54949)),
        2019            => (dec!(9169),  dec!(14254),  dec!(55960)),
        2020            => (dec!(9409),  dec!(14532),  dec!(57051)),
        2021            => (dec!(9745),  dec!(14754),  dec!(57918)),
        2022            => (dec!(10348), dec!(14927),  dec!(58597)),
        2023            => (dec!(10909), dec!(15999),  dec!(62809)),
        2024            => (dec!(11605), dec!(17005),  dec!(66760)),
        2025            => (dec!(12097), dec!(17430),  dec!(68430)),
        _               => (dec!(12097), dec!(17430),  dec!(68430)),
    };
    let z3_fin = dec!(277825); // Reichensteuersatz — stable

    if revenu <= z1_debut - dec!(1) {
        return Decimal::ZERO;
    }

    if revenu <= z1_fin {
        // Zone 1 : progression linéaire de 14 % à ~24 %
        // Formule EStG : (228,74 * y + 1400) * y   avec y = (revenu - GBF) / 10 000
        let y = (revenu - (z1_debut - dec!(1))) / dec!(10000);
        ((dec!(228.74) * y + dec!(1400)) * y).round_dp(0)
    } else if revenu <= z2_fin {
        // Zone 2 : progression de ~24 % à 42 %
        // Formule EStG : (108,73 * z + 2397) * z + seuil_zone1
        let steuer_z1 = {
            let y = (z1_fin - (z1_debut - dec!(1))) / dec!(10000);
            ((dec!(228.74) * y + dec!(1400)) * y).round_dp(0)
        };
        let z = (revenu - z1_fin) / dec!(10000);
        (steuer_z1 + (dec!(108.73) * z + dec!(2397)) * z).round_dp(0)
    } else if revenu <= z3_fin {
        // Zone 3 : 42 % (Spitzensteuersatz) — moins abattement
        let abat = dec!(9972); // Abzugsbetrag 2023
        (revenu * dec!(0.42) - abat).max(Decimal::ZERO).round_dp(0)
    } else {
        // Zone 4 : 45 % (Reichensteuersatz) — moins abattement
        let abat = dec!(18307); // Abzugsbetrag zone 4 2023
        (revenu * dec!(0.45) - abat).max(Decimal::ZERO).round_dp(0)
    }
}

// ── Lohnsteuer mensuelle ──────────────────────────────────────────────────────
//
// Méthode : annualisation du salaire mensuel → application du barème EStG →
// division par 12. Standard pour les salaires fixes (Lohnsteuerklassen I-VI).

/// Cotisations sociales salariales mensuelles prises en compte dans la
/// Vorsorgepauschale (§39b al. 2 phrase 5 n° 3 EStG).
#[derive(Clone, Copy, Default)]
pub struct Vorsorge {
    pub rv: Decimal,
    pub kv: Decimal,
    pub pv: Decimal,
    pub av: Decimal,
}

/// Arbeitnehmer-Pauschbetrag (§9a EStG).
fn an_pauschbetrag(annee: i32) -> Decimal {
    match annee {
        i32::MIN..=2021 => dec!(1000),
        2022            => dec!(1200),
        _               => dec!(1230),
    }
}

/// Vorsorgepauschale annuelle : part retraite (100 % dès 2023, 88 % en 2022…), puis
/// maladie (au taux réduit du PAP, voir de_bulletin.rs) + dépendance. Jusqu'en 2025,
/// plancher de 12 % du salaire plafonné à 1 900 € (3 000 € en classe III) ; dès 2026,
/// plancher supprimé et part chômage ajoutée tant que maladie + dépendance +
/// chômage ne dépassent pas 1 900 €.
fn vorsorgepauschale(brut_an: Decimal, v: Vorsorge, steuerklasse: u8, annee: i32) -> Decimal {
    let part_rv = match annee {
        i32::MIN..=2017 => dec!(0.68),
        2018 => dec!(0.72), 2019 => dec!(0.76), 2020 => dec!(0.80),
        2021 => dec!(0.84), 2022 => dec!(0.88),
        _ => dec!(1),
    };
    let rv = v.rv * dec!(12) * part_rv;
    let kv_pv = (v.kv + v.pv) * dec!(12);
    let sante = if annee >= 2026 {
        kv_pv + (v.av * dec!(12)).min((dec!(1900) - kv_pv).max(Decimal::ZERO))
    } else {
        let plafond = if steuerklasse == 3 { dec!(3000) } else { dec!(1900) };
        kv_pv.max((brut_an * dec!(0.12)).min(plafond))
    };
    (rv + sante).ceil()
}

fn lohnsteuer_annuel(brut_mensuel: Decimal, v: Vorsorge, steuerklasse: u8, annee: i32) -> Decimal {
    let brut_an = brut_mensuel * dec!(12);
    // Forfaits : frais professionnels (sauf classe VI) et dépenses spéciales (36 €,
    // 72 € en classe III), Vorsorgepauschale, Entlastungsbetrag en classe II.
    let forfaits = match steuerklasse {
        6 => Decimal::ZERO,
        3 => an_pauschbetrag(annee) + dec!(72),
        _ => an_pauschbetrag(annee) + dec!(36),
    };
    let mut zve = brut_an - forfaits - vorsorgepauschale(brut_an, v, steuerklasse, annee);
    if steuerklasse == 2 { zve -= entlastungsbetrag(annee); }
    let zve = zve.max(Decimal::ZERO);

    match steuerklasse {
        // Classe III : procédure du splitting (2 × tarif sur la moitié).
        3 => einkommensteuer_annuel(zve / dec!(2), annee) * dec!(2),
        // Classes V et VI : sans Grundfreibetrag, §39b al. 2 phrase 7 — impôt égal
        // au double de l'écart entre les tarifs sur 1,25 × zvE et 0,75 × zvE, au
        // minimum 14 % (plafonds de la formule officielle non modélisés).
        5 | 6 => {
            let ecart = einkommensteuer_annuel(zve * dec!(1.25), annee)
                - einkommensteuer_annuel(zve * dec!(0.75), annee);
            (ecart * dec!(2)).max(zve * dec!(0.14)).floor()
        }
        _ => einkommensteuer_annuel(zve, annee),
    }
}

// ── Solidaritätszuschlag ──────────────────────────────────────────────────────

/// Solidaritätszuschlag : 5,5 % de l'impôt au-delà d'un seuil d'exonération
/// (Freigrenze, doublé en classe III), avec zone de transition à 11,9 % de
/// l'excédent (SolzG §3 et §4).
fn solidaritaetszuschlag(lohnsteuer_annuel: Decimal, steuerklasse: u8, annee: i32) -> Decimal {
    let freigrenze = match annee {
        i32::MIN..=2020 => dec!(972),
        2021 | 2022     => dec!(16956),
        2023            => dec!(17543),
        2024            => dec!(18130),
        2025            => dec!(19950),
        _               => dec!(20350),
    } * if steuerklasse == 3 { dec!(2) } else { dec!(1) };
    if lohnsteuer_annuel <= freigrenze {
        return Decimal::ZERO;
    }
    let plein = lohnsteuer_annuel * dec!(0.055);
    let transition = (lohnsteuer_annuel - freigrenze) * dec!(0.119);
    plein.min(transition).round_dp(2)
}

// ── Kirchensteuer ─────────────────────────────────────────────────────────────

fn taux_kirchensteuer(land: &str) -> Decimal {
    match land {
        "BY" | "BW" => dec!(0.08), // Bayern et Baden-Württemberg : 8 %
        _           => dec!(0.09), // Tous les autres Länder : 9 %
    }
}

// ── Point d'entrée public — renvoie les lignes Lohnsteuer/Soli/Kirchensteuer ─

pub fn lohnsteuer_mensuel(
    brut: Decimal,
    vorsorge: Vorsorge,
    steuerklasse: u8,
    kirchenmitglied: bool,
    land: &str,
    ctx: &ContextPaie,
) -> Vec<LigneCotisation> {
    let annee = ctx.date_paie.year();
    let lst_annuel = lohnsteuer_annuel(brut, vorsorge, steuerklasse, annee);
    let lst_mensuel = (lst_annuel / dec!(12)).round_dp(2);

    let soli_annuel  = solidaritaetszuschlag(lst_annuel, steuerklasse, annee);
    let soli_mensuel = (soli_annuel / dec!(12)).round_dp(2);

    let sk_libelle = ctx.libelle(
        match steuerklasse {
            1 => "DE_SK1", 2 => "DE_SK2", 3 => "DE_SK3",
            4 => "DE_SK4", 5 => "DE_SK5", 6 => "DE_SK6", _ => "DE_SK1",
        },
        match steuerklasse {
            1 => "I — célibataire",
            2 => "II — parent isolé",
            3 => "III — marié·e (revenu élevé)",
            4 => "IV — marié·e (revenus égaux)",
            5 => "V — marié·e (revenu faible)",
            6 => "VI — second emploi",
            _ => "I",
        },
    );

    let gbf_info = match steuerklasse {
        1 | 4 => ctx.expl("DE_GBF_STD", "Grundfreibetrag ({gbf} €/an) appliqué")
            .replace("{gbf}", &format!("{:.0}", grundfreibetrag(annee))),
        2 => ctx.expl("DE_GBF_SK2", "Grundfreibetrag + Entlastungsbetrag Alleinerziehende ({ent} €/an)")
            .replace("{ent}", &format!("{:.0}", entlastungsbetrag(annee))),
        3 => ctx.expl("DE_GBF_SK3", "Grundfreibetrag doublé ({gbf2} €/an) — conjoint en SK V")
            .replace("{gbf2}", &format!("{:.0}", grundfreibetrag(annee) * dec!(2))),
        5 => ctx.expl("DE_GBF_SK5", "Pas de Grundfreibetrag — revenu entièrement imposable"),
        6 => ctx.expl("DE_GBF_SK6", "Pas de Grundfreibetrag + majoration second emploi"),
        _ => ctx.expl("DE_GBF_STD", "Grundfreibetrag ({gbf} €/an) appliqué")
            .replace("{gbf}", &format!("{:.0}", grundfreibetrag(annee))),
    };

    let mut lignes = Vec::new();

    // ── Lohnsteuer ─────────────────────────────────────────
    lignes.push(LigneCotisation {
        code:        "DE_LOHNSTEUER".into(),
        libelle:     ctx.libelle("DE_LOHNSTEUER", "Lohnsteuer — Steuerklasse {sk}")
            .replace("{sk}", &sk_libelle),
        base:        brut, // base mensuelle (le calcul est annualisé en interne)
        taux_sal:    if brut > Decimal::ZERO {
            (lst_mensuel / brut).round_dp(4)
        } else {
            Decimal::ZERO
        },
        montant_sal: lst_mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: ctx.expl("DE_LOHNSTEUER",
            "La Lohnsteuer est l'impôt sur les salaires allemand, prélevé à la source par l'employeur \
            (EStG §38). Elle est calculée sur le revenu annualisé ({revenu_an} €/an) selon le barème \
            progressif EStG §32a, puis divisée par 12 pour le bulletin mensuel.\n\n\
            Steuerklasse {skn} ({skl}) : {gbf_info}.\n\n\
            Grundfreibetrag {annee} : {gbf} €/an. \
            Barème {annee} : 0 % jusqu'au Grundfreibetrag → progression 14 %-42 % → \
            taux marginal 42 % (Spitzensteuersatz) → 45 % au-delà de 277 825 €/an.\n\n\
            Lohnsteuer annuelle calculée : {lst_an} € → mensuelle : {lst_m} €. \
            Note : le taux effectif affiché est indicatif (LSt mensuelle / brut mensuel).")
            .replace("{revenu_an}", &format!("{:.0}", brut * dec!(12)))
            .replace("{skn}", &steuerklasse.to_string())
            .replace("{skl}", &sk_libelle)
            .replace("{gbf_info}", &gbf_info)
            .replace("{gbf}", &format!("{:.0}", grundfreibetrag(annee)))
            .replace("{lst_an}", &format!("{:.2}", lst_annuel))
            .replace("{lst_m}", &format!("{:.2}", lst_mensuel))
            .replace("{annee}", &annee.to_string()),
        loi_ref: Some(ctx.loi_ref("EStG §32a, §38, §39 — Jahressteuergesetz annuels")),
    });

    // ── Solidaritätszuschlag ────────────────────────────────
    if soli_mensuel > Decimal::ZERO {
        lignes.push(LigneCotisation {
            code:        "DE_SOLI".into(),
            libelle:     ctx.libelle("DE_SOLI", "Solidaritätszuschlag"),
            base:        lst_mensuel,
            taux_sal:    dec!(0.055),
            montant_sal: soli_mensuel,
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "Impôt sur le revenu".into(),
            explication: ctx.expl("DE_SOLI",
                "Le Solidaritätszuschlag (\"Soli\") est une surtaxe de 5,5 % sur la Lohnsteuer, \
                instituée en 1991 pour financer la réunification allemande (SolZG). \
                Depuis le 01/01/2021, il est supprimé pour ~90 % des contribuables : \
                exonération si Lohnsteuer annuelle ≤ {seuil} €. \
                Zone de transition jusqu'à {seuil_haut} € de Lohnsteuer annuelle : taux progressif 11,9 %. \
                Au-delà : taux plein 5,5 %. {annee_info}")
                .replace("{seuil}", if annee >= 2021 { "17 543" } else { "0 (taux plein)" })
                .replace("{seuil_haut}", "66 915")
                .replace("{annee_info}", &if annee <= 2020 {
                    ctx.expl("DE_SOLI_ANNEE_PRE", "En {an}, le taux plein s'appliquait à tous.")
                        .replace("{an}", &annee.to_string())
                } else {
                    ctx.expl("DE_SOLI_ANNEE_POST", "En {an}, Lohnsteuer annuelle = {lst} € → Soli applicable.")
                        .replace("{an}", &annee.to_string())
                        .replace("{lst}", &format!("{:.2}", lst_annuel))
                }),
            loi_ref: Some(ctx.loi_ref("SolZG — Jahressteuergesetz 2021")),
        });
    }

    // ── Kirchensteuer ───────────────────────────────────────
    if kirchenmitglied {
        let taux_k = taux_kirchensteuer(land);
        let kirche_mensuel = (lst_mensuel * taux_k).round_dp(2);
        let taux_pct = if taux_k == dec!(0.08) { 8 } else { 9 };
        lignes.push(LigneCotisation {
            code:        "DE_KIRCHENSTEUER".into(),
            libelle:     ctx.libelle("DE_KIRCHENSTEUER", "Kirchensteuer ({land} — {tp} %)")
                .replace("{land}", land)
                .replace("{tp}", &taux_pct.to_string()),
            base:        lst_mensuel,
            taux_sal:    taux_k,
            montant_sal: kirche_mensuel,
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "Impôt sur le revenu".into(),
            explication: ctx.expl("DE_KIRCHENSTEUER",
                "La taxe d'église (Kirchensteuer) est prélevée par l'employeur sur la Lohnsteuer \
                au profit des grandes confessions (catholique, protestante, judaïque). \
                Elle est obligatoire si le salarié est enregistré comme membre auprès \
                de l'administration fiscale (Finanzamt).\n\n\
                Taux en {land} : {taux_pct} % de la Lohnsteuer. \
                Bayern (BY) et Baden-Württemberg (BW) appliquent 8 %, \
                les 14 autres Länder appliquent 9 %.\n\n\
                Le salarié peut se désengager (Kirchenaustritt) auprès du registre civil \
                — la Kirchensteuer disparaît alors du bulletin.")
                .replace("{land}", land)
                .replace("{taux_pct}", &taux_pct.to_string()),
            loi_ref: Some(ctx.loi_ref(&format!("KiStG {land} — EStG §51a"))),
        });
    }

    lignes
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Le tarif §32a est nul jusqu'au Grundfreibetrag et continu aux bornes de
    /// zones (à l'euro près) : une coquille dans un coefficient casse l'un ou l'autre.
    #[test]
    fn tarif_32a_nul_puis_continu() {
        for annee in 2023..=2026 {
            let gbf = grundfreibetrag(annee);
            assert_eq!(tarif_32a_exact(gbf, annee), Some(Decimal::ZERO), "{annee}");
            let st = |x: Decimal| tarif_32a_exact(x, annee).unwrap();
            let bornes = match annee {
                2023 => [dec!(15999), dec!(62809), dec!(277825)],
                2024 => [dec!(17005), dec!(66760), dec!(277825)],
                2025 => [dec!(17443), dec!(68480), dec!(277825)],
                _    => [dec!(17799), dec!(69878), dec!(277825)],
            };
            for b in bornes {
                let saut = st(b + dec!(1)) - st(b);
                assert!(saut >= Decimal::ZERO && saut <= dec!(2), "{annee} : saut de {saut} € à {b} €");
            }
        }
        // VZ 2026, 40 000 € de revenu imposable : 7 209 € (formule de la zone 2).
        assert_eq!(tarif_32a_exact(dec!(40000), 2026), Some(dec!(7209)));
    }
}
