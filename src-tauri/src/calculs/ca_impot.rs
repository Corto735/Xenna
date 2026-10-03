use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Cotisations du mois et paramètres T4127 ─────────────────────────────────
//
// La retenue d'impôt suit l'ARC, T4127 « Formules pour le calcul des retenues
// sur la paie » (122ᵉ édition, 01/01/2026), en version annualisée :
//   A  = 12 × brut − F5, F5 = partie bonifiée du RPC/RRQ (taux supplémentaire
//        ÷ taux total des cotisations) + RPC2/RRQ2 ;
//   K2 = taux minimal × (part de base du RPC/RRQ + AE [+ RQAP au Québec]) ;
//   K4 = taux minimal × min(A, montant canadien pour emploi) — fédéral seul.
// Au Québec, l'impôt provincial (TP-1015.F) n'accorde aucun crédit pour ces
// cotisations : seuls la partie supplémentaire du RRQ et la déduction pour
// travailleurs réduisent le revenu.

/// Cotisations salariales MENSUELLES du bulletin, qui entrent dans la retenue.
#[derive(Clone, Copy, Default)]
pub struct RetenuesCa {
    /// RPC ou RRQ (taux total, base + supplémentaire).
    pub rpc: Decimal,
    /// Taux salarial RPC/RRQ appliqué (pour isoler la part supplémentaire).
    pub taux_rpc: Decimal,
    /// RPC2 ou RRQ2.
    pub rpc2: Decimal,
    /// Assurance-emploi.
    pub ae: Decimal,
    /// RQAP (Québec seulement).
    pub rqap: Decimal,
}

/// Taux de la cotisation supplémentaire (bonification) RPC/RRQ, par année.
fn taux_bonifie(annee: i32) -> Decimal {
    match annee {
        i32::MIN..=2018 => Decimal::ZERO,
        2019 => dec!(0.0015),
        2020 => dec!(0.003),
        2021 => dec!(0.005),
        2022 => dec!(0.0075),
        _    => dec!(0.01),
    }
}

impl RetenuesCa {
    /// F5 annuel : part supplémentaire du RPC/RRQ + RPC2/RRQ2, déduits du revenu.
    fn f5(&self, annee: i32) -> Decimal {
        let part_sup = if self.taux_rpc > Decimal::ZERO {
            self.rpc * taux_bonifie(annee) / self.taux_rpc
        } else {
            Decimal::ZERO
        };
        ((part_sup + self.rpc2) * dec!(12)).round_dp(2)
    }

    /// Cotisations ouvrant droit au crédit K2 (annuel) : part de base du
    /// RPC/RRQ, AE et, pour l'impôt fédéral d'un Québécois, RQAP.
    fn base_k2(&self, annee: i32, avec_rqap: bool) -> Decimal {
        let part_base = if self.taux_rpc > Decimal::ZERO {
            self.rpc * (self.taux_rpc - taux_bonifie(annee)) / self.taux_rpc
        } else {
            Decimal::ZERO
        };
        let rqap = if avec_rqap { self.rqap } else { Decimal::ZERO };
        ((part_base + self.ae + rqap) * dec!(12)).round_dp(2)
    }
}

/// Montant canadien pour emploi (crédit K4), par année.
fn montant_emploi(annee: i32) -> Decimal {
    match annee {
        i32::MIN..=2015 => dec!(1146),
        2016 => dec!(1161),
        2017 => dec!(1178),
        2018 => dec!(1195),
        2019 => dec!(1222),
        2020 => dec!(1245),
        2021 => dec!(1257),
        2022 => dec!(1287),
        2023 => dec!(1368),
        2024 => dec!(1433),
        2025 => dec!(1471),
        _    => dec!(1501),
    }
}

/// Note commune aux explications : déduction F5 et crédits K2/K4 du calcul.
fn note_t4127(ctx: &ContextPaie, f5: Decimal, k2: Decimal, k4: Decimal) -> String {
    ctx.expl("CA_T4127_NOTE",
        "\nFormule T4127 : revenu diminué de la cotisation RPC supplémentaire ({f5} CAD) ; \
        crédits pour cotisations de base RPC/AE ({k2} CAD) et montant pour emploi ({k4} CAD).")
        .replace("{f5}", &format!("{:.2}", f5))
        .replace("{k2}", &format!("{:.2}", k2))
        .replace("{k4}", &format!("{:.2}", k4))
}

// ── Impôt fédéral ─────────────────────────────────────────────────────────────

fn impot_fed_annuel(revenu: Decimal, annee: i32) -> Decimal {
    if annee <= 2019 {
        if revenu <= dec!(47630) { revenu * dec!(0.15) }
        else if revenu <= dec!(95259) { dec!(7144.50) + (revenu - dec!(47630)) * dec!(0.205) }
        else if revenu <= dec!(147667) { dec!(16904.45) + (revenu - dec!(95259)) * dec!(0.26) }
        else if revenu <= dec!(210371) { dec!(30531.53) + (revenu - dec!(147667)) * dec!(0.29) }
        else { dec!(48715.69) + (revenu - dec!(210371)) * dec!(0.33) }
    } else if annee == 2020 {
        if revenu <= dec!(48535) { revenu * dec!(0.15) }
        else if revenu <= dec!(97069) { dec!(7280.25) + (revenu - dec!(48535)) * dec!(0.205) }
        else if revenu <= dec!(150473) { dec!(17229.72) + (revenu - dec!(97069)) * dec!(0.26) }
        else if revenu <= dec!(214368) { dec!(31115.28) + (revenu - dec!(150473)) * dec!(0.29) }
        else { dec!(49644.83) + (revenu - dec!(214368)) * dec!(0.33) }
    } else if annee == 2021 {
        if revenu <= dec!(49020) { revenu * dec!(0.15) }
        else if revenu <= dec!(98040) { dec!(7353.00) + (revenu - dec!(49020)) * dec!(0.205) }
        else if revenu <= dec!(151978) { dec!(17401.10) + (revenu - dec!(98040)) * dec!(0.26) }
        else if revenu <= dec!(216511) { dec!(31425.98) + (revenu - dec!(151978)) * dec!(0.29) }
        else { dec!(50139.55) + (revenu - dec!(216511)) * dec!(0.33) }
    } else if annee == 2022 {
        if revenu <= dec!(50197) { revenu * dec!(0.15) }
        else if revenu <= dec!(100392) { dec!(7529.55) + (revenu - dec!(50197)) * dec!(0.205) }
        else if revenu <= dec!(155625) { dec!(17829.53) + (revenu - dec!(100392)) * dec!(0.26) }
        else if revenu <= dec!(221708) { dec!(32190.25) + (revenu - dec!(155625)) * dec!(0.29) }
        else { dec!(51352.32) + (revenu - dec!(221708)) * dec!(0.33) }
    } else if annee == 2023 {
        if revenu <= dec!(53359) { revenu * dec!(0.15) }
        else if revenu <= dec!(106717) { dec!(8003.85) + (revenu - dec!(53359)) * dec!(0.205) }
        else if revenu <= dec!(165430) { dec!(18942.24) + (revenu - dec!(106717)) * dec!(0.26) }
        else if revenu <= dec!(235675) { dec!(34207.26) + (revenu - dec!(165430)) * dec!(0.29) }
        else { dec!(54581.01) + (revenu - dec!(235675)) * dec!(0.33) }
    } else if annee == 2024 {
        if revenu <= dec!(55867) { revenu * dec!(0.15) }
        else if revenu <= dec!(111733) { dec!(8380.05) + (revenu - dec!(55867)) * dec!(0.205) }
        else if revenu <= dec!(154906) { dec!(19832.58) + (revenu - dec!(111733)) * dec!(0.26) }
        else if revenu <= dec!(220000) { dec!(31057.56) + (revenu - dec!(154906)) * dec!(0.29) }
        else { dec!(49934.82) + (revenu - dec!(220000)) * dec!(0.33) }
    } else {
        // 2025 : taux de la 1ʳᵉ tranche ramené de 15 à 14 % au 01/07/2025, soit 14,5 %
        // pour l'année d'imposition ; 2026 : 14 %. ARC, T4127 (121ᵉ et 122ᵉ éditions).
        let (s, t1): ([Decimal; 4], Decimal) = if annee == 2025 {
            ([dec!(57375), dec!(114750), dec!(177882), dec!(253414)], dec!(0.145))
        } else {
            ([dec!(58523), dec!(117045), dec!(181440), dec!(258482)], dec!(0.14))
        };
        impot_prov_brackets(revenu,
            &[s[0], s[1], s[2], s[3], dec!(999999999)],
            &[t1, dec!(0.205), dec!(0.26), dec!(0.29), dec!(0.33)])
    }
}

/// Taux de la 1ʳᵉ tranche fédérale, qui sert aussi au crédit du MPB.
fn taux_base_fed(annee: i32) -> Decimal {
    match annee {
        i32::MIN..=2024 => dec!(0.15),
        2025            => dec!(0.145),
        _               => dec!(0.14),
    }
}

fn bpa_credit_fed(annee: i32) -> Decimal {
    // Montant personnel de base (MPB) × 15 % (taux de base fédéral)
    let bpa = match annee {
        i32::MIN..=2019 => dec!(12069),
        2020            => dec!(13229),
        2021            => dec!(13808),
        2022            => dec!(14398),
        2023            => dec!(15000),
        2024            => dec!(15705),
        2025            => dec!(16129),
        _               => dec!(16452), // 2026 (T4127, 122ᵉ édition)
    };
    (bpa * taux_base_fed(annee)).round_dp(2)
}

pub fn ca_impot_federal(brut: Decimal, r: &RetenuesCa, ctx: &ContextPaie) -> LigneCotisation {
    impot_federal(brut, r, ctx, false)
}

/// Impôt fédéral d'un résident du Québec : l'impôt fédéral de base est réduit de
/// l'abattement du Québec remboursable de 16,5 % (Loi de l'impôt sur le revenu,
/// art. 120(2) ; Loi sur les arrangements fiscaux, art. 27).
pub fn ca_impot_federal_qc(brut: Decimal, r: &RetenuesCa, ctx: &ContextPaie) -> LigneCotisation {
    impot_federal(brut, r, ctx, true)
}

fn impot_federal(brut: Decimal, r: &RetenuesCa, ctx: &ContextPaie, quebec: bool) -> LigneCotisation {
    let annee        = ctx.date_paie.year();
    let f5           = r.f5(annee);
    let revenu_ann   = brut * dec!(12) - f5;
    let impot_brut   = impot_fed_annuel(revenu_ann, annee);
    let credit_bpa   = bpa_credit_fed(annee);
    let k2           = (taux_base_fed(annee) * r.base_k2(annee, quebec)).round_dp(2);
    let k4           = (taux_base_fed(annee) * revenu_ann.min(montant_emploi(annee))).round_dp(2);
    let impot_base   = (impot_brut - credit_bpa - k2 - k4).max(Decimal::ZERO);
    let abattement   = if quebec { (impot_base * dec!(0.165)).round_dp(2) } else { Decimal::ZERO };
    let impot_net    = impot_base - abattement;
    let impot_mens   = (impot_net / dec!(12)).round_dp(2);
    let taux_eff     = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code:        "CA_IMPOT_FED".into(),
        libelle:     ctx.libelle("CA_IMPOT_FED", "Impôt fédéral — retenue {annee}")
            .replace("{annee}", &annee.to_string()),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: impot_mens,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt fédéral".into(),
        explication: ctx.expl("CA_IMPOT_FED",
            "Retenue mensuelle d'impôt fédéral sur le revenu. L'employeur est \
            sostituto d'imposta (retenues à la source — formulaire TD1). \
            \n\n\
            [ Calcul {an} — barème fédéral ]\n\
            Revenu annuel estimé    : {rev} CAD\n\
            Impôt brut annuel       : {ib} CAD\n\
            Crédit personnel (MPB)  : − {cred} CAD\n\
            Abattement du Québec    : − {abat} CAD\n\
            Impôt net annuel        : {inet} CAD\n\
            Retenue mensuelle       : {mens} CAD (÷ 12)\n\
            Taux effectif           : {teff} %\n\
            \n\
            Barème {an} : {t1}/20,5/26/29/33 %. \
            Le Montant personnel de base ({mpb} CAD) génère un crédit de {t1} % = {cred} CAD/an. \
            Résident du Québec : impôt fédéral réduit de l'abattement de 16,5 %. \
            Régularisation en décembre ou déclaration T1 annuelle.{t4127}")
            .replace("{an}", &annee.to_string())
            .replace("{rev}", &format!("{:.2}", revenu_ann))
            .replace("{ib}", &format!("{:.2}", impot_brut))
            .replace("{cred}", &format!("{:.2}", credit_bpa))
            .replace("{abat}", &format!("{:.2}", abattement))
            .replace("{t1}", &format!("{}", (taux_base_fed(annee) * dec!(100)).normalize()).replace('.', ","))
            .replace("{inet}", &format!("{:.2}", impot_net))
            .replace("{mens}", &format!("{:.2}", impot_mens))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100)))
            .replace("{mpb}", match annee {
                i32::MIN..=2019 => "12 069", 2020 => "13 229", 2021 => "13 808",
                2022 => "14 398", 2023 => "15 000", 2024 => "15 705", 2025 => "16 129",
                _ => "16 452"
            })
            .replace("{t4127}", &note_t4127(ctx, f5, k2, k4)),
        loi_ref: Some(ctx.loi_ref("L.R.C. 1985, ch. 1 (5e suppl.), art. 117-117.1 — Formulaire TD1")),
    }
}

// ── Impôt provincial Ontario (référence hors Québec) ─────────────────────────

/// Barème de l'Ontario : (seuils d'entrée, MPB). ARC, T4127 ; TD1ON 2026 pour le MPB 2026.
fn bareme_on(annee: i32) -> ([Decimal; 4], Decimal) {
    match annee {
        i32::MIN..=2024 => ([dec!(51446), dec!(102894), dec!(150000), dec!(220000)], dec!(11865)),
        2025            => ([dec!(52886), dec!(105775), dec!(150000), dec!(220000)], dec!(12747)),
        _               => ([dec!(53891), dec!(107785), dec!(150000), dec!(220000)], dec!(12989)),
    }
}

fn impot_on_annuel(revenu: Decimal, annee: i32) -> Decimal {
    let (a, _) = bareme_on(annee);
    impot_prov_brackets(revenu,
        &[a[0], a[1], a[2], a[3], dec!(999999999)],
        &[dec!(0.0505), dec!(0.0915), dec!(0.1116), dec!(0.1216), dec!(0.1316)])
}

/// Surtaxe de l'Ontario sur l'impôt provincial de base : 20 % au-delà du 1ᵉʳ seuil,
/// + 36 % au-delà du 2ᵉ (T4127 : 5 710 / 7 307 $ en 2025 ; 5 818 / 7 446 $ en 2026).
/// Seuils antérieurs non relevés : pas de surtaxe avant 2025.
fn surtaxe_on(impot_base: Decimal, annee: i32) -> Decimal {
    let (s1, s2) = match annee {
        i32::MIN..=2024 => return Decimal::ZERO,
        2025            => (dec!(5710), dec!(7307)),
        _               => (dec!(5818), dec!(7446)),
    };
    (impot_base - s1).max(Decimal::ZERO) * dec!(0.20) + (impot_base - s2).max(Decimal::ZERO) * dec!(0.36)
}

/// Contribution-santé de l'Ontario (barème non indexé, Loi de 2007 sur les impôts, art. 33.1).
fn contribution_sante_on(revenu: Decimal) -> Decimal {
    let r = revenu;
    if r <= dec!(20000) { Decimal::ZERO }
    else if r <= dec!(36000) { ((r - dec!(20000)) * dec!(0.06)).min(dec!(300)) }
    else if r <= dec!(48000) { (dec!(300) + (r - dec!(36000)) * dec!(0.06)).min(dec!(450)) }
    else if r <= dec!(72000) { (dec!(450) + (r - dec!(48000)) * dec!(0.25)).min(dec!(600)) }
    else if r <= dec!(200000) { (dec!(600) + (r - dec!(72000)) * dec!(0.25)).min(dec!(750)) }
    else { (dec!(750) + (r - dec!(200000)) * dec!(0.25)).min(dec!(900)) }
}

/// Réduction d'impôt de l'Ontario (T4127, facteur S, sans personne à charge) :
/// min(T4 + V1, 2 × montant de base − (T4 + V1)), nulle si négative.
fn reduction_on(impot_et_surtaxe: Decimal, annee: i32) -> Decimal {
    let base = match annee {
        i32::MIN..=2024 => return Decimal::ZERO,
        2025            => dec!(294),
        _               => dec!(300),
    };
    impot_et_surtaxe.min(dec!(2) * base - impot_et_surtaxe).max(Decimal::ZERO)
}

pub fn ca_impot_ontario(brut: Decimal, r: &RetenuesCa, ctx: &ContextPaie) -> LigneCotisation {
    let annee      = ctx.date_paie.year();
    let f5         = r.f5(annee);
    let revenu_ann = brut * dec!(12) - f5;
    let (_, mpb)   = bareme_on(annee);
    let impot_brut = impot_on_annuel(revenu_ann, annee);
    let k2p        = (dec!(0.0505) * r.base_k2(annee, false)).round_dp(2);
    let credit_bpa = (mpb * dec!(0.0505)).round_dp(2) + k2p;
    let impot_base = (impot_brut - credit_bpa).max(Decimal::ZERO);
    let surtaxe    = surtaxe_on(impot_base, annee).round_dp(2);
    let reduction  = reduction_on(impot_base + surtaxe, annee).round_dp(2);
    let sante      = contribution_sante_on(revenu_ann).round_dp(2);
    let impot_net  = impot_base + surtaxe - reduction + sante;
    let impot_mens = (impot_net / dec!(12)).round_dp(2);
    let taux_eff   = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code:        "ON_IMPOT_PROV".into(),
        libelle:     ctx.libelle("ON_IMPOT_PROV", "Impôt provincial Ontario — retenue {annee}")
            .replace("{annee}", &annee.to_string()),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: impot_mens,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt provincial".into(),
        explication: ctx.expl("ON_IMPOT_PROV",
            "Retenue mensuelle d'impôt provincial de l'Ontario (province de référence hors Québec). \
            Barème {an} : 5,05/9,15/11,16/12,16/13,16 %. \
            MPB Ontario {an} : {mpb} CAD → crédit de {cred} CAD/an. \
            \n\n\
            Revenu annuel estimé  : {rev} CAD\n\
            Impôt brut annuel     : {ib} CAD\n\
            Crédit MPB            : − {cred} CAD\n\
            Surtaxe de l'Ontario  : + {surt} CAD\n\
            Contribution-santé    : + {sante} CAD\n\
            Impôt net annuel      : {inet} CAD\n\
            Retenue mensuelle     : {mens} CAD\n\
            Taux effectif         : {teff} %\n\
            \n\
            Note : non applicable au Québec (province ayant son propre impôt séparé). \
            Les autres provinces (CB, AB, QC excl.) ont leurs propres barèmes — \
            utiliser Ontario comme approximation générale.{t4127}")
            .replace("{t4127}", &note_t4127(ctx, f5, k2p, Decimal::ZERO))
            .replace("{an}", &annee.to_string())
            .replace("{mpb}", &mpb.to_string())
            .replace("{cred}", &format!("{:.2}", credit_bpa))
            .replace("{surt}", &format!("{:.2}", surtaxe))
            .replace("{sante}", &format!("{:.2}", sante))
            .replace("{rev}", &format!("{:.2}", revenu_ann))
            .replace("{ib}", &format!("{:.2}", impot_brut))
            .replace("{inet}", &format!("{:.2}", impot_net))
            .replace("{mens}", &format!("{:.2}", impot_mens))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("L.O. 2007, ch. 11, ann. A — Formulaire TD1ON")),
    }
}

// ── Impôt provincial Québec ───────────────────────────────────────────────────

/// Table d'imposition du Québec : seuils des tranches à 14/19/24/25,75 %.
/// 2025-2026 : ministère des Finances du Québec, « Paramètres du régime
/// d'imposition des particuliers » (novembre 2025), tableau 3.
fn seuils_qc(annee: i32) -> [Decimal; 3] {
    match annee {
        i32::MIN..=2024 => [dec!(51780), dec!(103545), dec!(126000)],
        2025            => [dec!(53255), dec!(106495), dec!(129590)],
        _               => [dec!(54345), dec!(108680), dec!(132245)],
    }
}

fn impot_qc_annuel(revenu: Decimal, annee: i32) -> Decimal {
    let s = seuils_qc(annee);
    impot_prov_brackets(revenu,
        &[s[0], s[1], s[2], dec!(999999999)],
        &[dec!(0.14), dec!(0.19), dec!(0.24), dec!(0.2575)])
}

fn bpa_qc(annee: i32) -> Decimal {
    match annee {
        i32::MIN..=2021 => dec!(15270),
        2022            => dec!(16143),
        2023            => dec!(16143),
        2024            => dec!(17183),
        2025            => dec!(18571),
        _               => dec!(18952),
    }
}

fn bpa_credit_qc(annee: i32) -> Decimal {
    (bpa_qc(annee) * dec!(0.14)).round_dp(2)
}

/// Déduction pour travailleurs (Québec) : 6 % du revenu de travail, plafonnée.
/// Plafonds relevés depuis 2024 seulement (CFFP, Université de Sherbrooke) :
/// avant, aucune déduction n'est appliquée.
fn deduction_travailleurs(revenu_travail: Decimal, annee: i32) -> Decimal {
    let plafond = match annee {
        i32::MIN..=2023 => return Decimal::ZERO,
        2024 => dec!(1380),
        2025 => dec!(1420),
        _    => dec!(1450),
    };
    (revenu_travail * dec!(0.06)).min(plafond).round_dp(2)
}

pub fn qc_impot_provincial(brut: Decimal, r: &RetenuesCa, ctx: &ContextPaie) -> LigneCotisation {
    let annee      = ctx.date_paie.year();
    let f5         = r.f5(annee);
    let travail    = deduction_travailleurs(brut * dec!(12), annee);
    let revenu_ann = brut * dec!(12) - f5 - travail;
    let impot_brut = impot_qc_annuel(revenu_ann, annee);
    let credit_bpa = bpa_credit_qc(annee);
    let impot_net  = (impot_brut - credit_bpa).max(Decimal::ZERO);
    let impot_mens = (impot_net / dec!(12)).round_dp(2);
    let taux_eff   = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };

    LigneCotisation {
        code:        "QC_IMPOT_PROV".into(),
        libelle:     ctx.libelle("QC_IMPOT_PROV", "Impôt provincial Québec — retenue {annee}")
            .replace("{annee}", &annee.to_string()),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: impot_mens,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt provincial".into(),
        explication: ctx.expl("QC_IMPOT_PROV",
            "Le Québec perçoit son propre impôt provincial directement (unique au Canada) \
            via Revenu Québec, contrairement aux autres provinces où l'ARC perçoit \
            les deux impôts conjointement. \
            \n\n\
            Barème {an} : 14/19/24/25,75 %. MPB Québec : {mpb} CAD → crédit : {cred} CAD/an.\n\
            \n\
            [ Calcul ]\n\
            Revenu annuel estimé    : {rev} CAD\n\
            Impôt brut annuel       : {ib} CAD\n\
            Crédit MPB              : − {cred} CAD\n\
            Impôt net annuel        : {inet} CAD\n\
            Retenue mensuelle       : {mens} CAD\n\
            Taux effectif           : {teff} %\n\
            \n\
            L'employeur produit le relevé 1 (RL-1) au lieu du T4. \
            Le salarié québécois produit deux déclarations : T1 (fédéral) + TP-1 (provincial).{tp1015}")
            .replace("{tp1015}", &ctx.expl("QC_TP1015_NOTE",
                "\nRevenu diminué de la cotisation RRQ supplémentaire ({f5} CAD) et de la déduction \
                pour travailleurs ({trav} CAD) ; aucun crédit pour les cotisations de base (TP-1015.F).")
                .replace("{f5}", &format!("{:.2}", f5))
                .replace("{trav}", &format!("{:.2}", travail)))
            .replace("{an}", &annee.to_string())
            .replace("{mpb}", &bpa_qc(annee).to_string())
            .replace("{cred}", &format!("{:.2}", credit_bpa))
            .replace("{rev}", &format!("{:.2}", revenu_ann))
            .replace("{ib}", &format!("{:.2}", impot_brut))
            .replace("{inet}", &format!("{:.2}", impot_net))
            .replace("{mens}", &format!("{:.2}", impot_mens))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref("RLRQ, ch. I-3, art. 750 — Formulaire TP-1015.3 — Relevé 1 (RL-1)")),
    }
}

// ── Impôt provincial — toutes provinces/territoires (hors Québec) ─────────────
//
// Taux 2024 (Lois provinciales sur l'impôt sur le revenu).
// Indexés annuellement — valeurs à vérifier chaque année.
// Source : ARC T1 Guide, feuillets T4 et formulaires TD1 provinciaux.

fn impot_prov_brackets(revenu: Decimal, seuils: &[Decimal], taux: &[Decimal]) -> Decimal {
    let mut impot = Decimal::ZERO;
    let mut prev  = Decimal::ZERO;
    for (i, &seuil) in seuils.iter().enumerate() {
        if revenu <= prev { break; }
        let taxable = revenu.min(seuil) - prev;
        impot += taxable * taux[i];
        prev = seuil;
        if revenu <= seuil { break; }
    }
    impot
}

/// Barème provincial (hors Québec et Ontario) : seuils d'entrée des tranches (A),
/// taux (V) et montant personnel de base, par année. Sources : ARC, T4127
/// « Payroll Deductions Formulas » — 120ᵉ/121ᵉ éditions (2025) et 122ᵉ (2026),
/// tableaux 8.1 et 8.2. Pour 2025, les valeurs sont celles de l'année d'imposition
/// (Alberta : taux de 8 % sur les 60 000 premiers $ ; Î.-P.-É. : MPB 14 650 $ ;
/// Saskatchewan : MPB 19 491 $), non les taux « au prorata » du second semestre.
/// Manitoba : MPB gelé à 15 780 $ (réduit au-delà de 200 000 $ de revenu, non
/// modélisé) ; Yukon : MPB égal au fédéral.
fn bareme_prov(province: &str, annee: i32) -> Option<(&'static [u32], &'static [&'static str], u32)> {
    Some(match (province, annee) {
        ("AB", 2026..) => (&[0, 61200, 154259, 185111, 246813, 370220], &["0.08", "0.10", "0.12", "0.13", "0.14", "0.15"], 22769),
        ("AB", 2025)   => (&[0, 60000, 151234, 181481, 241974, 362961], &["0.08", "0.10", "0.12", "0.13", "0.14", "0.15"], 22323),
        ("AB", _)      => (&[0, 148269, 177922, 237230, 355845], &["0.10", "0.12", "0.13", "0.14", "0.15"], 21003),
        ("BC", 2026..) => (&[0, 50363, 100728, 115648, 140430, 190405, 265545], &["0.0506", "0.077", "0.105", "0.1229", "0.147", "0.168", "0.205"], 13216),
        ("BC", 2025)   => (&[0, 49279, 98560, 113158, 137407, 186306, 259829], &["0.0506", "0.077", "0.105", "0.1229", "0.147", "0.168", "0.205"], 12932),
        ("BC", _)      => (&[0, 45654, 91310, 104835, 127299, 172602, 240716], &["0.0506", "0.077", "0.105", "0.1229", "0.147", "0.168", "0.205"], 11981),
        ("MB", 2026..) => (&[0, 47000, 100000], &["0.108", "0.1275", "0.174"], 15780),
        ("MB", 2025)   => (&[0, 46513, 98796], &["0.108", "0.1275", "0.174"], 15780),
        ("MB", _)      => (&[0, 36842, 79625], &["0.108", "0.1275", "0.174"], 15780),
        ("NB", 2026..) => (&[0, 52333, 104666, 193861], &["0.094", "0.14", "0.16", "0.195"], 13664),
        ("NB", 2025)   => (&[0, 51306, 102614, 190060], &["0.094", "0.14", "0.16", "0.195"], 13396),
        ("NB", _)      => (&[0, 49958, 99916, 185064], &["0.094", "0.1482", "0.1652", "0.1784"], 12458),
        ("NL", 2026..) => (&[0, 44678, 89354, 159528, 223340, 285319, 570638, 1141275], &["0.087", "0.145", "0.158", "0.178", "0.198", "0.208", "0.213", "0.218"], 11188),
        ("NL", 2025)   => (&[0, 44192, 88382, 157792, 220910, 282214, 564429, 1128858], &["0.087", "0.145", "0.158", "0.178", "0.198", "0.208", "0.213", "0.218"], 11067),
        ("NL", _)      => (&[0, 43198, 86395, 154244, 215943, 275870, 551739], &["0.087", "0.145", "0.158", "0.178", "0.198", "0.208", "0.213"], 10818),
        ("NS", 2026..) => (&[0, 30995, 61991, 97417, 157124], &["0.0879", "0.1495", "0.1667", "0.175", "0.21"], 11932),
        ("NS", 2025)   => (&[0, 30507, 61015, 95883, 154650], &["0.0879", "0.1495", "0.1667", "0.175", "0.21"], 11744),
        ("NS", _)      => (&[0, 29590, 59180, 93000, 150000], &["0.0879", "0.1495", "0.1667", "0.175", "0.21"], 8481),
        ("NT", 2026..) => (&[0, 53003, 106009, 172346], &["0.059", "0.086", "0.122", "0.1405"], 18198),
        ("NT", 2025)   => (&[0, 51964, 103930, 168967], &["0.059", "0.086", "0.122", "0.1405"], 17842),
        ("NT", _)      => (&[0, 50597, 101198, 164525], &["0.059", "0.086", "0.122", "0.1405"], 16593),
        ("NU", 2026..) => (&[0, 55801, 111602, 181439], &["0.04", "0.07", "0.09", "0.115"], 19659),
        ("NU", 2025)   => (&[0, 54707, 109413, 177881], &["0.04", "0.07", "0.09", "0.115"], 19274),
        ("NU", _)      => (&[0, 53268, 106537, 173205], &["0.04", "0.07", "0.09", "0.115"], 17925),
        ("PE", 2026..) => (&[0, 33928, 65820, 106890, 142520], &["0.095", "0.1347", "0.166", "0.1762", "0.19"], 15000),
        ("PE", 2025)   => (&[0, 33328, 64656, 105000, 140000], &["0.095", "0.1347", "0.166", "0.1762", "0.19"], 14650),
        ("PE", _)      => (&[0, 32656, 64313, 105000, 140000], &["0.0965", "0.1363", "0.1665", "0.18", "0.1875"], 12000),
        ("SK", 2026..) => (&[0, 54532, 155805], &["0.105", "0.125", "0.145"], 20381),
        ("SK", 2025)   => (&[0, 53463, 152750], &["0.105", "0.125", "0.145"], 19491),
        ("SK", _)      => (&[0, 49720, 142058], &["0.105", "0.125", "0.145"], 17661),
        ("YT", 2026..) => (&[0, 58523, 117045, 181440, 500000], &["0.064", "0.09", "0.109", "0.128", "0.15"], 16452),
        ("YT", 2025)   => (&[0, 57375, 114750, 177882, 500000], &["0.064", "0.09", "0.109", "0.128", "0.15"], 16129),
        ("YT", _)      => (&[0, 55867, 111733, 154906, 500000], &["0.064", "0.09", "0.109", "0.128", "0.15"], 15705),
        _ => return None,
    })
}

/// « 5,06/7,70/… %. MPB 12 932 CAD. » à partir d'un barème.
fn desc_bareme(taux: &[Decimal], bpa: Decimal) -> String {
    let t: Vec<String> = taux.iter()
        .map(|x| format!("{:.2}", x * dec!(100)).replace('.', ","))
        .collect();
    format!("{} %. MPB {} CAD.", t.join("/"), bpa.round_dp(0))
}

pub fn ca_impot_provincial(brut: Decimal, province: &str, r: &RetenuesCa, ctx: &ContextPaie) -> LigneCotisation {
    if province == "ON" {
        return ca_impot_ontario(brut, r, ctx);
    }

    let annee      = ctx.date_paie.year();
    let f5         = r.f5(annee);
    let revenu_ann = brut * dec!(12) - f5;

    let (nom, loi) = match province {
        "AB" => ("Alberta", "Alberta Personal Income Tax Act, SA 1999 c A-33.5"),
        "BC" => ("Colombie-Britannique", "Income Tax Act (B.C.), RSBC 1996 c 215"),
        "MB" => ("Manitoba", "Income Tax Act (Manitoba), CCSM c I10"),
        "NB" => ("Nouveau-Brunswick", "Loi de l'impôt sur le revenu (N.-B.), LRN-B 2000 c I-2.2"),
        "NL" => ("Terre-Neuve-et-Labrador", "Income Tax Act, 2000 (N.L.), SNL2000 c I-1.1"),
        "NS" => ("Nouvelle-Écosse", "Income Tax Act (Nova Scotia), RSNS 1989 c 217"),
        "NT" => ("Territoires du Nord-Ouest", "Income Tax Act (Northwest Territories), RSNWT 1988 c I-3"),
        "NU" => ("Nunavut", "Income Tax Act (Nunavut), RSNWT 1988 c I-3 (adapté)"),
        "PE" => ("Île-du-Prince-Édouard", "Income Tax Act (P.E.I.), RSPEI 1988 c I-1"),
        "SK" => ("Saskatchewan", "The Income Tax Act, 2000 (Saskatchewan), SS 2000 c I-2.01"),
        "YT" => ("Yukon", "Income Tax Act (Yukon), RSY 2002 c 118"),
        _ => return ca_impot_ontario(brut, r, ctx),  // fallback Ontario
    };
    let (entrees, taux_txt, bpa) = bareme_prov(province, annee).expect("province couverte");
    // Seuils d'entrée → bornes hautes (dernière tranche ouverte).
    let seuils: Vec<Decimal> = entrees.iter().skip(1).map(|a| Decimal::from(*a))
        .chain(std::iter::once(dec!(999999999))).collect();
    let taux: Vec<Decimal> = taux_txt.iter().map(|t| t.parse().expect("taux")).collect();
    let bpa = Decimal::from(bpa);
    let impot_brut = impot_prov_brackets(revenu_ann, &seuils, &taux);
    let k2p = (taux[0] * r.base_k2(annee, false)).round_dp(2);
    let bpa_credit = (bpa * taux[0]).round_dp(2) + k2p;
    let tranches_desc_s = desc_bareme(&taux, bpa);
    let tranches_desc = tranches_desc_s.as_str();

    let impot_net  = (impot_brut - bpa_credit).max(Decimal::ZERO);
    let impot_mens = (impot_net / dec!(12)).round_dp(2);
    let taux_eff   = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };

    let code = format!("{province}_IMPOT_PROV");
    LigneCotisation {
        libelle:     ctx.libelle(&code, "Impôt provincial {nom} — retenue {annee}")
            .replace("{nom}", nom)
            .replace("{annee}", &annee.to_string()),
        explication: ctx.expl(&code,
            "Retenue mensuelle d'impôt provincial — {nom}.\n\
            Barème {annee} : {tranches_desc}\n\
            \n\
            [ Calcul ]\n\
            Revenu annuel estimé  : {revenu} CAD\n\
            Impôt brut annuel     : {ib} CAD\n\
            Crédit MPB            : − {bpa} CAD\n\
            Impôt net annuel      : {inet} CAD\n\
            Retenue mensuelle     : {mens} CAD (÷ 12)\n\
            Taux effectif         : {teff} %\n\
            \n\
            Retenu conjointement avec l'impôt fédéral par l'employeur (sostituto d'imposta). \
            Régularisation via déclaration T1 annuelle (ARC) et, si applicable, \
            déclaration provinciale complémentaire.{t4127}")
            .replace("{t4127}", &note_t4127(ctx, f5, k2p, Decimal::ZERO))
            .replace("{nom}", nom)
            .replace("{annee}", &annee.to_string())
            .replace("{tranches_desc}", tranches_desc)
            .replace("{revenu}", &format!("{:.2}", revenu_ann))
            .replace("{ib}", &format!("{:.2}", impot_brut))
            .replace("{bpa}", &format!("{:.2}", bpa_credit))
            .replace("{inet}", &format!("{:.2}", impot_net))
            .replace("{mens}", &format!("{:.2}", impot_mens))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        code,
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: impot_mens,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt provincial".into(),
        loi_ref: Some(ctx.loi_ref(loi)),
    }
}
