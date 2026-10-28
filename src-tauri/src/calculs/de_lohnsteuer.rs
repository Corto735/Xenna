use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Paramètres datés (plafond_reference, migration 0151) ──────────────────────
//
// Grundfreibetrag (§32a), Entlastungsbetrag (§24b), Arbeitnehmer-Pauschbetrag
// (§9a), part retraite de la Vorsorgepauschale (§39b Abs. 4), Freigrenze de la
// Soli (§3 SolZG), fins des zones 1 et 2 du tarif et, de 2023 à 2026, ses
// coefficients exacts. La formule du §32a reste ici ; ses paramètres sont en base.

/// Coefficients du tarif exact du §32a al. 1 : coef. zone 1, coef. zone 2,
/// constante zone 2, abattements des tranches à 42 % et 45 %.
struct Exact { a1: Decimal, a2: Decimal, c2: Decimal, k42: Decimal, k45: Decimal }

struct Params {
    gbf: Decimal,
    entlastung: Decimal,
    an_pausch: Decimal,
    vorsorge_rv: Decimal,
    soli_frei: Decimal,
    z1: Decimal,
    z2: Decimal,
    exact: Option<Exact>,
    /// Plancher de la Vorsorgepauschale supprimé et part chômage ajoutée (2026).
    regime_2026: bool,
}

impl Params {
    fn charger(ctx: &ContextPaie) -> Option<Params> {
        let p = |code: &str| ctx.plafond(code);
        let exact = match (p("DE_ESTG_A1"), p("DE_ESTG_A2"), p("DE_ESTG_C2"), p("DE_ESTG_K42"), p("DE_ESTG_K45")) {
            (Some(a1), Some(a2), Some(c2), Some(k42), Some(k45)) => Some(Exact { a1, a2, c2, k42, k45 }),
            _ => None,
        };
        Some(Params {
            gbf: p("DE_GRUNDFREIBETRAG")?,
            entlastung: p("DE_ENTLASTUNG")?,
            an_pausch: p("DE_AN_PAUSCH")?,
            vorsorge_rv: p("DE_VORSORGE_RV")?,
            soli_frei: p("DE_SOLI_FREIGRENZE")?,
            z1: p("DE_ESTG_Z1")?,
            z2: p("DE_ESTG_Z2")?,
            exact,
            regime_2026: ctx.date_paie.year() >= 2026,
        })
    }
}

/// Le régime est-il couvert à la date de paie ? (de_bulletin)
pub fn couvert(ctx: &ContextPaie) -> bool {
    Params::charger(ctx).is_some()
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

/// Début de la tranche à 45 % (Reichensteuer), stable sur la période couverte.
const Z3: Decimal = dec!(277825);

/// Tarif exact du §32a al. 1 EStG sur le revenu imposable `x` (arrondi à l'euro
/// inférieur), quand ses coefficients sont en base (2023-2026).
/// Sources : gesetze-im-internet.de (VZ 2026), buzer.de (VZ 2024), EStH 2025.
fn tarif_32a_exact(x: Decimal, p: &Params) -> Option<Decimal> {
    let e = p.exact.as_ref()?;
    let x = x.floor();
    let st = if x <= p.gbf {
        Decimal::ZERO
    } else if x <= p.z1 {
        let y = (x - p.gbf) / dec!(10000);
        (e.a1 * y + dec!(1400)) * y
    } else if x <= p.z2 {
        let z = (x - p.z1) / dec!(10000);
        (e.a2 * z + dec!(2397)) * z + e.c2
    } else if x <= Z3 {
        dec!(0.42) * x - e.k42
    } else {
        dec!(0.45) * x - e.k45
    };
    Some(st.max(Decimal::ZERO).floor())
}

/// Impôt sur le revenu imposable `x` : tarif exact dès 2023, approximation
/// historique (zones à coefficients fixes) avant.
fn einkommensteuer_annuel(revenu: Decimal, p: &Params) -> Decimal {
    tarif_32a_exact(revenu, p).unwrap_or_else(|| einkommensteuer_ancien(revenu, p))
}

/// APPROXIMATION antérieure à 2023, faute de coefficients relevés : bornes des
/// zones datées (en base), coefficients et abattements de 2023 en dur.
fn einkommensteuer_ancien(revenu: Decimal, p: &Params) -> Decimal {
    if revenu <= Decimal::ZERO {
        return Decimal::ZERO;
    }
    let (z1_debut, z1_fin, z2_fin) = (p.gbf + dec!(1), p.z1, p.z2);

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
    } else if revenu <= Z3 {
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

/// Vorsorgepauschale annuelle : part retraite (100 % dès 2023, 88 % en 2022…), puis
/// maladie (au taux réduit du PAP, voir de_bulletin.rs) + dépendance. Jusqu'en 2025,
/// plancher de 12 % du salaire plafonné à 1 900 € (3 000 € en classe III) ; dès 2026,
/// plancher supprimé et part chômage ajoutée tant que maladie + dépendance +
/// chômage ne dépassent pas 1 900 €.
fn vorsorgepauschale(brut_an: Decimal, v: Vorsorge, steuerklasse: u8, p: &Params) -> Decimal {
    let rv = v.rv * dec!(12) * p.vorsorge_rv;
    let kv_pv = (v.kv + v.pv) * dec!(12);
    let sante = if p.regime_2026 {
        kv_pv + (v.av * dec!(12)).min((dec!(1900) - kv_pv).max(Decimal::ZERO))
    } else {
        let plafond = if steuerklasse == 3 { dec!(3000) } else { dec!(1900) };
        kv_pv.max((brut_an * dec!(0.12)).min(plafond))
    };
    (rv + sante).ceil()
}

fn lohnsteuer_annuel(brut_mensuel: Decimal, v: Vorsorge, steuerklasse: u8, p: &Params) -> Decimal {
    let brut_an = brut_mensuel * dec!(12);
    // Forfaits : frais professionnels (sauf classe VI) et dépenses spéciales (36 €,
    // 72 € en classe III), Vorsorgepauschale, Entlastungsbetrag en classe II.
    let forfaits = match steuerklasse {
        6 => Decimal::ZERO,
        3 => p.an_pausch + dec!(72),
        _ => p.an_pausch + dec!(36),
    };
    let mut zve = brut_an - forfaits - vorsorgepauschale(brut_an, v, steuerklasse, p);
    if steuerklasse == 2 { zve -= p.entlastung; }
    let zve = zve.max(Decimal::ZERO);

    match steuerklasse {
        // Classe III : procédure du splitting (2 × tarif sur la moitié).
        3 => einkommensteuer_annuel(zve / dec!(2), p) * dec!(2),
        // Classes V et VI : sans Grundfreibetrag, §39b al. 2 phrase 7 — impôt égal
        // au double de l'écart entre les tarifs sur 1,25 × zvE et 0,75 × zvE, au
        // minimum 14 % (plafonds de la formule officielle non modélisés).
        5 | 6 => {
            let ecart = einkommensteuer_annuel(zve * dec!(1.25), p)
                - einkommensteuer_annuel(zve * dec!(0.75), p);
            (ecart * dec!(2)).max(zve * dec!(0.14)).floor()
        }
        _ => einkommensteuer_annuel(zve, p),
    }
}

// ── Solidaritätszuschlag ──────────────────────────────────────────────────────

/// Solidaritätszuschlag : 5,5 % de l'impôt au-delà d'un seuil d'exonération
/// (Freigrenze, doublé en classe III), avec zone de transition à 11,9 % de
/// l'excédent (SolzG §3 et §4).
/// Seuil d'exonération de la Soli, doublé en classe III.
fn soli_freigrenze(steuerklasse: u8, p: &Params) -> Decimal {
    p.soli_frei * if steuerklasse == 3 { dec!(2) } else { dec!(1) }
}

/// Fin de la zone de transition : là où 11,9 % de l'excédent rejoint 5,5 % de
/// l'impôt entier, soit Freigrenze × 0,119 / 0,064.
fn soli_fin_transition(steuerklasse: u8, p: &Params) -> Decimal {
    (soli_freigrenze(steuerklasse, p) * dec!(0.119) / dec!(0.064)).floor()
}

fn solidaritaetszuschlag(lohnsteuer_annuel: Decimal, steuerklasse: u8, p: &Params) -> Decimal {
    let freigrenze = soli_freigrenze(steuerklasse, p);
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
    let p = Params::charger(ctx).expect("paramètres DE vérifiés par de_bulletin");
    let lst_annuel = lohnsteuer_annuel(brut, vorsorge, steuerklasse, &p);
    let lst_mensuel = (lst_annuel / dec!(12)).round_dp(2);

    let soli_annuel  = solidaritaetszuschlag(lst_annuel, steuerklasse, &p);
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
            .replace("{gbf}", &format!("{:.0}", p.gbf)),
        2 => ctx.expl("DE_GBF_SK2", "Grundfreibetrag + Entlastungsbetrag Alleinerziehende ({ent} €/an)")
            .replace("{ent}", &format!("{:.0}", p.entlastung)),
        3 => ctx.expl("DE_GBF_SK3", "Grundfreibetrag doublé ({gbf2} €/an) — conjoint en SK V")
            .replace("{gbf2}", &format!("{:.0}", p.gbf * dec!(2))),
        5 => ctx.expl("DE_GBF_SK5", "Pas de Grundfreibetrag — revenu entièrement imposable"),
        6 => ctx.expl("DE_GBF_SK6", "Pas de Grundfreibetrag + majoration second emploi"),
        _ => ctx.expl("DE_GBF_STD", "Grundfreibetrag ({gbf} €/an) appliqué")
            .replace("{gbf}", &format!("{:.0}", p.gbf)),
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
            .replace("{gbf}", &format!("{:.0}", p.gbf))
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
                .replace("{seuil}", &if annee >= 2021 {
                    format!("{:.0}", soli_freigrenze(steuerklasse, &p))
                } else { "0 (taux plein)".to_string() })
                .replace("{seuil_haut}", &format!("{:.0}", soli_fin_transition(steuerklasse, &p)))
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
    use rust_decimal_macros::dec;

    /// Le tarif §32a est nul jusqu'au Grundfreibetrag et continu aux bornes de
    /// zones (à l'euro près) : une coquille dans un coefficient en base casse
    /// l'un ou l'autre.
    #[tokio::test]
    async fn tarif_32a_nul_puis_continu() {
        let path = std::env::temp_dir().join(format!("xenna_de_tarif_{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let pool = crate::db::init_db(&path).await.unwrap();
        for annee in 2023..=2026 {
            let date = chrono::NaiveDate::from_ymd_opt(annee, 6, 15).unwrap();
            let ctx = ContextPaie::charger(&pool, date).await.unwrap();
            let p = Params::charger(&ctx).unwrap();
            assert_eq!(tarif_32a_exact(p.gbf, &p), Some(Decimal::ZERO), "{annee}");
            let st = |x: Decimal| tarif_32a_exact(x, &p).unwrap();
            for b in [p.z1, p.z2, Z3] {
                let saut = st(b + dec!(1)) - st(b);
                assert!(saut >= Decimal::ZERO && saut <= dec!(2), "{annee} : saut de {saut} € à {b} €");
            }
            if annee == 2026 {
                // VZ 2026, 40 000 € de revenu imposable : 7 209 € (formule de la zone 2).
                assert_eq!(tarif_32a_exact(dec!(40000), &p), Some(dec!(7209)));
            }
        }
        let _ = std::fs::remove_file(&path);
    }
}
