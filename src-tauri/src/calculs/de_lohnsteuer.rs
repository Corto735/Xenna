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
        2024            => dec!(11604),
        2025            => dec!(12096),
        _               => dec!(12648), // 2026 — estimation loi de finances
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
// Zones de progression (valeurs 2026 — Grundfreibetrag = 12 648 €) :
//   Zone 0 : 0 € → 12 648 € → 0 %
//   Zone 1 : 12 649 € → ~17 220 € → formule progressive (14 % → 24 %)
//   Zone 2 : ~17 221 € → ~68 430 € → formule progressive (24 % → 42 %)
//   Zone 3 : ~68 431 € → 277 825 € → 42 % (Spitzensteuersatz)
//   Zone 4 : > 277 825 € → 45 % (Reichensteuersatz)
//
// Les seuils des zones 1 et 2 sont ajustés chaque année proportionnellement
// au Grundfreibetrag. On utilise des seuils fixes par tranche d'années.

fn einkommensteuer_annuel(revenu: Decimal, annee: i32) -> Decimal {
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
        _               => (dec!(12649), dec!(17222),  dec!(68430)), // 2026 estimation
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
        let abat = dec!(9972); // Abzugsbetrag 2026 approximatif
        (revenu * dec!(0.42) - abat).max(Decimal::ZERO).round_dp(0)
    } else {
        // Zone 4 : 45 % (Reichensteuersatz) — moins abattement
        let abat = dec!(18307); // Abzugsbetrag zone 4 approximatif
        (revenu * dec!(0.45) - abat).max(Decimal::ZERO).round_dp(0)
    }
}

// ── Lohnsteuer mensuelle ──────────────────────────────────────────────────────
//
// Méthode : annualisation du salaire mensuel → application du barème EStG →
// division par 12. Standard pour les salaires fixes (Lohnsteuerklassen I-VI).

fn lohnsteuer_annuel(brut_mensuel: Decimal, steuerklasse: u8, annee: i32) -> Decimal {
    let revenu_annuel = brut_mensuel * dec!(12);
    let gbf = grundfreibetrag(annee);

    // Revenu imposable selon Steuerklasse
    let imposable = match steuerklasse {
        // SK I et IV : abattement standard (Grundfreibetrag)
        1 | 4 => (revenu_annuel - gbf).max(Decimal::ZERO),
        // SK II : abattement + Entlastungsbetrag Alleinerziehende
        2 => (revenu_annuel - gbf - entlastungsbetrag(annee)).max(Decimal::ZERO),
        // SK III : doublement du Grundfreibetrag (époux à revenu élevé)
        3 => (revenu_annuel - gbf * dec!(2)).max(Decimal::ZERO),
        // SK V : pas de Grundfreibetrag (époux à faible revenu, conjoint en SK III)
        5 => revenu_annuel,
        // SK VI : aucun abattement (second emploi)
        6 => revenu_annuel,
        _ => (revenu_annuel - gbf).max(Decimal::ZERO),
    };

    // SK V et VI ont une retenue forfaitaire minimale supplémentaire — simplifiée ici
    let steuer_brute = einkommensteuer_annuel(imposable, annee);

    // SK VI : majoration de 10 % (second emploi — approximation)
    if steuerklasse == 6 {
        steuer_brute * dec!(1.1)
    } else {
        steuer_brute
    }
}

// ── Solidaritätszuschlag ──────────────────────────────────────────────────────

fn solidaritaetszuschlag(lohnsteuer_annuel: Decimal, annee: i32) -> Decimal {
    if annee <= 2020 {
        // Taux plein 5,5 %
        (lohnsteuer_annuel * dec!(0.055)).round_dp(2)
    } else {
        // Depuis 2021 : exonération quasi-totale pour revenus courants
        // Seuil annuel : 17 543 € de Lohnsteuer → mensuel : ~1 462 €
        let seuil_an = dec!(17543);
        let seuil_haut = dec!(66915); // zone de transition
        if lohnsteuer_annuel <= seuil_an {
            Decimal::ZERO
        } else if lohnsteuer_annuel <= seuil_haut {
            // Zone de transition : 11,9 % de (LSt - seuil)
            ((lohnsteuer_annuel - seuil_an) * dec!(0.119)).round_dp(2)
        } else {
            (lohnsteuer_annuel * dec!(0.055)).round_dp(2)
        }
    }
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
    steuerklasse: u8,
    kirchenmitglied: bool,
    land: &str,
    ctx: &ContextPaie,
) -> Vec<LigneCotisation> {
    let annee = ctx.date_paie.year();
    let lst_annuel = lohnsteuer_annuel(brut, steuerklasse, annee);
    let lst_mensuel = (lst_annuel / dec!(12)).round_dp(2);

    let soli_annuel  = solidaritaetszuschlag(lst_annuel, annee);
    let soli_mensuel = (soli_annuel / dec!(12)).round_dp(2);

    let sk_libelle = match steuerklasse {
        1 => "I — célibataire",
        2 => "II — parent isolé",
        3 => "III — marié·e (revenu élevé)",
        4 => "IV — marié·e (revenus égaux)",
        5 => "V — marié·e (revenu faible)",
        6 => "VI — second emploi",
        _ => "I",
    };

    let mut lignes = Vec::new();

    // ── Lohnsteuer ─────────────────────────────────────────
    lignes.push(LigneCotisation {
        code:        "DE_LOHNSTEUER".into(),
        libelle:     format!("Lohnsteuer — Steuerklasse {sk_libelle}"),
        base:        brut * dec!(12), // base annuelle affichée
        taux_sal:    if brut > Decimal::ZERO {
            (lst_mensuel / brut).round_dp(4)
        } else {
            Decimal::ZERO
        },
        montant_sal: lst_mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: format!(
            "La Lohnsteuer est l'impôt sur les salaires allemand, prélevé à la source par l'employeur \
            (EStG §38). Elle est calculée sur le revenu annualisé ({revenu_an:.0} €/an) selon le barème \
            progressif EStG §32a, puis divisée par 12 pour le bulletin mensuel.\n\n\
            Steuerklasse {sk} ({sk_lib}) : {gbf_info}.\n\n\
            Grundfreibetrag {annee} : {gbf:.0} €/an. \
            Barème {annee} : 0 % jusqu'au Grundfreibetrag → progression 14 %-42 % → \
            taux marginal 42 % (Spitzensteuersatz) → 45 % au-delà de 277 825 €/an.\n\n\
            Lohnsteuer annuelle calculée : {lst_an:.2} € → mensuelle : {lst_m:.2} €. \
            Note : le taux effectif affiché est indicatif (LSt mensuelle / brut mensuel).",
            revenu_an = brut * dec!(12),
            sk        = steuerklasse,
            sk_lib    = sk_libelle,
            gbf_info  = match steuerklasse {
                1 | 4 => format!("Grundfreibetrag ({:.0} €/an) appliqué", grundfreibetrag(annee)),
                2     => format!("Grundfreibetrag + Entlastungsbetrag Alleinerziehende ({:.0} €/an)", entlastungsbetrag(annee)),
                3     => format!("Grundfreibetrag doublé ({:.0} €/an) — conjoint en SK V", grundfreibetrag(annee) * dec!(2)),
                5     => "Pas de Grundfreibetrag — revenu entièrement imposable".to_string(),
                6     => "Pas de Grundfreibetrag + majoration second emploi".to_string(),
                _     => "Grundfreibetrag appliqué".to_string(),
            },
            annee     = annee,
            gbf       = grundfreibetrag(annee),
            lst_an    = lst_annuel,
            lst_m     = lst_mensuel,
        ),
        loi_ref: Some("EStG §32a, §38, §39 — Jahressteuergesetz annuels".into()),
    });

    // ── Solidaritätszuschlag ────────────────────────────────
    if soli_mensuel > Decimal::ZERO {
        lignes.push(LigneCotisation {
            code:        "DE_SOLI".into(),
            libelle:     "Solidaritätszuschlag".into(),
            base:        lst_mensuel,
            taux_sal:    dec!(0.055),
            montant_sal: soli_mensuel,
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "Impôt sur le revenu".into(),
            explication: format!(
                "Le Solidaritätszuschlag (\"Soli\") est une surtaxe de 5,5 % sur la Lohnsteuer, \
                instituée en 1991 pour financer la réunification allemande (SolZG). \
                Depuis le 01/01/2021, il est supprimé pour ~90 % des contribuables : \
                exonération si Lohnsteuer annuelle ≤ {seuil} €. \
                Zone de transition jusqu'à {seuil_haut} € de Lohnsteuer annuelle : taux progressif 11,9 %. \
                Au-delà : taux plein 5,5 %. {annee_info}",
                seuil      = if annee >= 2021 { "17 543" } else { "0 (taux plein)" },
                seuil_haut = "66 915",
                annee_info = if annee <= 2020 {
                    format!("En {}, le taux plein s'appliquait à tous.", annee)
                } else {
                    format!("En {}, Lohnsteuer annuelle = {:.2} € → Soli applicable.", annee, lst_annuel)
                },
            ),
            loi_ref: Some("SolZG — Jahressteuergesetz 2021".into()),
        });
    }

    // ── Kirchensteuer ───────────────────────────────────────
    if kirchenmitglied {
        let taux_k = taux_kirchensteuer(land);
        let kirche_mensuel = (lst_mensuel * taux_k).round_dp(2);
        let taux_pct = if taux_k == dec!(0.08) { 8 } else { 9 };
        lignes.push(LigneCotisation {
            code:        "DE_KIRCHENSTEUER".into(),
            libelle:     format!("Kirchensteuer ({land} — {taux_pct} %)"),
            base:        lst_mensuel,
            taux_sal:    taux_k,
            montant_sal: kirche_mensuel,
            taux_pat:    Decimal::ZERO,
            montant_pat: Decimal::ZERO,
            categorie:   "Impôt sur le revenu".into(),
            explication: format!(
                "La taxe d'église (Kirchensteuer) est prélevée par l'employeur sur la Lohnsteuer \
                au profit des grandes confessions (catholique, protestante, judaïque). \
                Elle est obligatoire si le salarié est enregistré comme membre auprès \
                de l'administration fiscale (Finanzamt).\n\n\
                Taux en {land} : {taux_pct} % de la Lohnsteuer. \
                Bayern (BY) et Baden-Württemberg (BW) appliquent 8 %, \
                les 14 autres Länder appliquent 9 %.\n\n\
                Le salarié peut se désengager (Kirchenaustritt) auprès du registre civil \
                — la Kirchensteuer disparaît alors du bulletin.",
                land     = land,
                taux_pct = taux_pct,
            ),
            loi_ref: Some(format!("KiStG {land} — EStG §51a").into()),
        });
    }

    lignes
}
