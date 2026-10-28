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

// Paramètres et barèmes en base (migration 0152) : CA_TAUX_BONIFIE,
// CA_MONTANT_EMPLOI ; barèmes progressifs CA_PROV_FED, CA_PROV_ON, CA_PROV_QC
// et CA_PROV_<XX>, avec leur montant personnel de base CA_PROV_<XX>_MPB.

/// Taux de la cotisation supplémentaire (bonification) RPC/RRQ.
fn taux_bonifie(ctx: &ContextPaie) -> Decimal {
    ctx.plafond("CA_TAUX_BONIFIE").unwrap_or(Decimal::ZERO)
}

/// Barème `CA_PROV_<code>` : bornes hautes (dernière ouverte) et taux.
fn bareme(code: &str, ctx: &ContextPaie) -> (Vec<Decimal>, Vec<Decimal>) {
    let t = ctx.bareme(&format!("CA_PROV_{code}")).unwrap_or(&[]);
    (t.iter().map(|x| x.borne_sup.unwrap_or(Decimal::MAX)).collect(), t.iter().map(|x| x.taux).collect())
}

/// Impôt progressif du barème `CA_PROV_<code>`.
fn impot_bareme(code: &str, revenu: Decimal, ctx: &ContextPaie) -> Decimal {
    ctx.bareme_progressif(&format!("CA_PROV_{code}"), revenu).unwrap_or(Decimal::ZERO)
}

/// Taux de la 1ʳᵉ tranche, qui sert aussi au crédit du montant personnel de base.
fn taux_base(code: &str, ctx: &ContextPaie) -> Decimal {
    ctx.bareme(&format!("CA_PROV_{code}")).and_then(|t| t.first()).map_or(Decimal::ZERO, |t| t.taux)
}

/// Montant personnel de base du barème `code`.
fn mpb(code: &str, ctx: &ContextPaie) -> Decimal {
    ctx.plafond(&format!("CA_PROV_{code}_MPB")).unwrap_or(Decimal::ZERO)
}

/// Le régime est-il couvert à la date de paie ? (ca_bulletin, qc_bulletin)
pub fn couvert(ctx: &ContextPaie) -> bool {
    ctx.bareme("CA_PROV_FED").is_some() && ctx.plafond("CA_PROV_FED_MPB").is_some()
}

impl RetenuesCa {
    /// F5 annuel : part supplémentaire du RPC/RRQ + RPC2/RRQ2, déduits du revenu.
    fn f5(&self, ctx: &ContextPaie) -> Decimal {
        let part_sup = if self.taux_rpc > Decimal::ZERO {
            self.rpc * taux_bonifie(ctx) / self.taux_rpc
        } else {
            Decimal::ZERO
        };
        ((part_sup + self.rpc2) * dec!(12)).round_dp(2)
    }

    /// Cotisations ouvrant droit au crédit K2 (annuel) : part de base du
    /// RPC/RRQ, AE et, pour l'impôt fédéral d'un Québécois, RQAP.
    fn base_k2(&self, ctx: &ContextPaie, avec_rqap: bool) -> Decimal {
        let part_base = if self.taux_rpc > Decimal::ZERO {
            self.rpc * (self.taux_rpc - taux_bonifie(ctx)) / self.taux_rpc
        } else {
            Decimal::ZERO
        };
        let rqap = if avec_rqap { self.rqap } else { Decimal::ZERO };
        ((part_base + self.ae + rqap) * dec!(12)).round_dp(2)
    }
}

/// Montant canadien pour emploi (crédit K4).
fn montant_emploi(ctx: &ContextPaie) -> Decimal {
    ctx.plafond("CA_MONTANT_EMPLOI").unwrap_or(Decimal::ZERO)
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

// 2025 : taux de la 1ʳᵉ tranche ramené de 15 à 14 % au 01/07/2025, soit 14,5 %
// pour l'année d'imposition ; 2026 : 14 %. ARC, T4127 (121ᵉ et 122ᵉ éditions).

fn bpa_credit_fed(ctx: &ContextPaie) -> Decimal {
    // Montant personnel de base (MPB) × taux de base fédéral
    (mpb("FED", ctx) * taux_base("FED", ctx)).round_dp(2)
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
    let f5           = r.f5(ctx);
    let revenu_ann   = brut * dec!(12) - f5;
    let impot_brut   = impot_bareme("FED", revenu_ann, ctx);
    let credit_bpa   = bpa_credit_fed(ctx);
    let k2           = (taux_base("FED", ctx) * r.base_k2(ctx, quebec)).round_dp(2);
    let k4           = (taux_base("FED", ctx) * revenu_ann.min(montant_emploi(ctx))).round_dp(2);
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
            .replace("{t1}", &format!("{}", (taux_base("FED", ctx) * dec!(100)).normalize()).replace('.', ","))
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

/// Surtaxe de l'Ontario sur l'impôt provincial de base : 20 % au-delà du 1ᵉʳ seuil,
/// + 36 % au-delà du 2ᵉ (CA_ON_SURTAXE_1 / _2 : 5 710 / 7 307 $ en 2025 ;
/// 5 818 / 7 446 $ en 2026). Seuils antérieurs non relevés : pas de surtaxe.
fn surtaxe_on(impot_base: Decimal, ctx: &ContextPaie) -> Decimal {
    let (Some(s1), Some(s2)) = (ctx.plafond("CA_ON_SURTAXE_1"), ctx.plafond("CA_ON_SURTAXE_2")) else {
        return Decimal::ZERO;
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
fn reduction_on(impot_et_surtaxe: Decimal, ctx: &ContextPaie) -> Decimal {
    let Some(base) = ctx.plafond("CA_ON_REDUCTION_BASE") else {
        return Decimal::ZERO;
    };
    impot_et_surtaxe.min(dec!(2) * base - impot_et_surtaxe).max(Decimal::ZERO)
}

pub fn ca_impot_ontario(brut: Decimal, r: &RetenuesCa, ctx: &ContextPaie) -> LigneCotisation {
    let annee      = ctx.date_paie.year();
    let f5         = r.f5(ctx);
    let revenu_ann = brut * dec!(12) - f5;
    let impot_brut = impot_bareme("ON", revenu_ann, ctx);
    let t1         = taux_base("ON", ctx);
    let k2p        = (t1 * r.base_k2(ctx, false)).round_dp(2);
    let credit_bpa = (mpb("ON", ctx) * t1).round_dp(2) + k2p;
    let impot_base = (impot_brut - credit_bpa).max(Decimal::ZERO);
    let surtaxe    = surtaxe_on(impot_base, ctx).round_dp(2);
    let reduction  = reduction_on(impot_base + surtaxe, ctx).round_dp(2);
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
            .replace("{mpb}", &mpb("ON", ctx).to_string())
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

// Table d'imposition du Québec (CA_PROV_QC) : tranches à 14/19/24/25,75 %.
// 2025-2026 : ministère des Finances du Québec, « Paramètres du régime
// d'imposition des particuliers » (novembre 2025), tableau 3.

/// Déduction pour travailleurs (Québec) : 6 % du revenu de travail, plafonnée.
/// Plafonds relevés depuis 2024 seulement (CFFP, Université de Sherbrooke) :
/// avant, aucune déduction n'est appliquée.
fn deduction_travailleurs(revenu_travail: Decimal, ctx: &ContextPaie) -> Decimal {
    let Some(plafond) = ctx.plafond("CA_QC_DED_TRAV_PLAFOND") else {
        return Decimal::ZERO;
    };
    (revenu_travail * dec!(0.06)).min(plafond).round_dp(2)
}

pub fn qc_impot_provincial(brut: Decimal, r: &RetenuesCa, ctx: &ContextPaie) -> LigneCotisation {
    let annee      = ctx.date_paie.year();
    let f5         = r.f5(ctx);
    let travail    = deduction_travailleurs(brut * dec!(12), ctx);
    let revenu_ann = brut * dec!(12) - f5 - travail;
    let impot_brut = impot_bareme("QC", revenu_ann, ctx);
    let credit_bpa = (mpb("QC", ctx) * taux_base("QC", ctx)).round_dp(2);
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
            .replace("{mpb}", &mpb("QC", ctx).to_string())
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

// Barèmes CA_PROV_<XX> (hors Québec et Ontario) : sources ARC, T4127 « Payroll
// Deductions Formulas » — 120ᵉ/121ᵉ éditions (2025) et 122ᵉ (2026), tableaux 8.1
// et 8.2. Pour 2025, les valeurs de l'année d'imposition (Alberta : 8 % sur les
// 60 000 premiers $ ; Î.-P.-É. : MPB 14 650 $ ; Saskatchewan : MPB 19 491 $),
// non les taux « au prorata » du second semestre. Manitoba : MPB gelé à
// 15 780 $ (réduit au-delà de 200 000 $ de revenu, non modélisé) ; Yukon : MPB
// égal au fédéral.

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
    let f5         = r.f5(ctx);
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
    let (_, taux) = bareme(province, ctx);
    let Some(&t1) = taux.first() else { return ca_impot_ontario(brut, r, ctx) };
    let bpa = mpb(province, ctx);
    let impot_brut = impot_bareme(province, revenu_ann, ctx);
    let k2p = (t1 * r.base_k2(ctx, false)).round_dp(2);
    let bpa_credit = (bpa * t1).round_dp(2) + k2p;
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
