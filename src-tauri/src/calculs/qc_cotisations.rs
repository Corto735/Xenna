use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;
use super::ca_cotisations::{mga_mensuel, mgap2_mensuel, maga_mensuel, EXEMPTION_BASE};

/// MAGA RQAP mensuel (Maximum des gains assurables RQAP — Québec).
pub fn maga_rqap_mensuel(ctx: &ContextPaie) -> Decimal {
    match ctx.date_paie.year() {
        i32::MIN..=2019 => dec!(6375.00),  // 76 500 / 12
        2020            => dec!(6541.67),  // 78 500
        2021            => dec!(6958.33),  // 83 500
        2022            => dec!(7333.33),  // 88 000
        2023            => dec!(7583.33),  // 91 000
        2024            => dec!(7833.33),  // 94 000
        2025            => dec!(8125.00),  // 97 500
        _               => dec!(8400.00),  // 2026+ estimation
    }
}

// ── RRQ — Régime de rentes du Québec ─────────────────────────────────────────

pub fn qc_rrq(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let mga      = mga_mensuel(ctx);
    let pensionn = (brut.min(mga) - EXEMPTION_BASE).max(Decimal::ZERO);
    let ts = ctx.taux_sal("QC_RRQ");
    let tp = ctx.taux_pat("QC_RRQ");
    LigneCotisation {
        code:        "QC_RRQ".into(),
        libelle:     "RRQ — Régime de rentes du Québec".into(),
        base:        pensionn,
        taux_sal:    ts,
        montant_sal: (pensionn * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (pensionn * tp).round_dp(2),
        categorie:   "Retraite Québec".into(),
        explication: format!(
            "Le Régime de rentes du Québec (RRQ / QPP) est l'équivalent québécois du RPC, \
            mais géré indépendamment par Retraite Québec depuis 1966 (RLRQ, ch. R-9). \
            Les travailleurs québécois cotisent au RRQ et non au RPC fédéral. \
            \n\n\
            [ Calcul {} ]\n\
            Gains pensionnables = min(brut, MGA/12) − exonération de base\n\
            = min({:.2}, {:.2}) − {:.2} = {:.2} CAD\n\
            Taux {} : {:.2} % salarié = {:.2} % employeur\n\
            \n\
            Depuis 2019, le RRQ est bonifié progressivement (identique au RPC) : \
            le taux a augmenté chaque année de 5,55 % (2019) à 6,40 % (2023+). \
            Le taux RRQ est légèrement supérieur au RPC pour les mêmes dates, \
            en raison de la démographie et de l'historique du fonds québécois.",
            ctx.date_paie.year(),
            brut, mga, EXEMPTION_BASE, pensionn,
            ctx.date_paie.year(), ts * dec!(100), tp * dec!(100),
        ),
        loi_ref: Some("RLRQ, ch. R-9, art. 50 et 52 — Retraite Québec".into()),
    }
}

// ── RRQ2 — Bonification supplémentaire Québec (dès 2024) ─────────────────────

pub fn qc_rrq2(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let ts = ctx.taux_sal("QC_RRQ2");
    if ts == Decimal::ZERO {
        return None;
    }
    let mga   = mga_mensuel(ctx);
    let mgap2 = mgap2_mensuel(ctx);
    let base2 = (brut.min(mgap2) - mga).max(Decimal::ZERO);
    if base2 == Decimal::ZERO {
        return None;
    }
    let tp = ctx.taux_pat("QC_RRQ2");
    Some(LigneCotisation {
        code:        "QC_RRQ2".into(),
        libelle:     "RRQ2 — Bonification supplémentaire (phase 2)".into(),
        base:        base2,
        taux_sal:    ts,
        montant_sal: (base2 * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base2 * tp).round_dp(2),
        categorie:   "Retraite Québec".into(),
        explication: format!(
            "La phase 2 de la bonification du RRQ s'applique sur les gains entre \
            le MGA ({:.2} CAD/mois) et le MGAP2 ({:.2} CAD/mois) à un taux de 4 %. \
            Gains supplémentaires {} : {:.2} CAD. Identique au RPC2 sauf que géré par Retraite Québec.",
            mga, mgap2, ctx.date_paie.year(), base2,
        ),
        loi_ref: Some("RLRQ, ch. R-9, art. 50.0.1 — L.Q. 2018, ch. 2".into()),
    })
}

// ── AE Québec — taux réduit ───────────────────────────────────────────────────

pub fn qc_ae(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let maga = maga_mensuel(ctx);
    let base = brut.min(maga);
    let ts   = ctx.taux_sal("QC_AE");
    let tp   = ctx.taux_pat("QC_AE");
    LigneCotisation {
        code:        "QC_AE".into(),
        libelle:     "AE — Assurance-emploi (taux réduit Québec)".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Chômage fédéral".into(),
        explication: format!(
            "Les travailleurs québécois paient un taux d'AE réduit en vertu de l'art. 69 \
            de la Loi sur l'assurance-emploi, car le RQAP prend en charge les prestations \
            parentales (maternité, paternité, parental, adoption). \
            \n\n\
            Taux {} : {:.2} % salarié + {:.2} % employeur (= salarié × 1,4)\n\
            vs. régime général : différentiel d'environ 0,35 pp (salarié)\n\
            \n\
            Cette réduction reflète le transfert de responsabilité fédéral → provincial \
            pour les prestations parentales, grâce à l'accord Canada-Québec de 2005 \
            ayant permis la création du RQAP.",
            ctx.date_paie.year(), ts * dec!(100), tp * dec!(100),
        ),
        loi_ref: Some("L.C. 1996, ch. 23, art. 69 — Accord Canada-Québec sur le RQAP (2005)".into()),
    }
}

// ── RQAP — Régime québécois d'assurance parentale ────────────────────────────

pub fn qc_rqap(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let plafond = maga_rqap_mensuel(ctx);
    let base    = brut.min(plafond);
    let ts      = ctx.taux_sal("QC_RQAP");
    let tp      = ctx.taux_pat("QC_RQAP");
    LigneCotisation {
        code:        "QC_RQAP".into(),
        libelle:     "RQAP — Assurance parentale (Québec)".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Parentalité Québec".into(),
        explication: format!(
            "Le Régime québécois d'assurance parentale (RQAP) a remplacé les prestations \
            parentales de l'AE fédérale pour les Québécois depuis le 1er janvier 2006 \
            (RLRQ, ch. A-29.011). Il offre des conditions plus généreuses que l'AE. \
            \n\n\
            Plafond RQAP {} : {:.2} CAD/mois ({:.0} CAD/an)\n\
            Taux : {:.3} % salarié + {:.3} % employeur\n\
            \n\
            Prestations couvertes (plan de base) :\n\
            • Maternité : 18 semaines à 70 % du revenu\n\
            • Paternité : 5 semaines à 70 %\n\
            • Parental : 40 semaines (ou 25 sem. en plan bonifié à 75 %)\n\
            • Adoption : 37 semaines à 70 %\n\
            \n\
            Le taux de cotisation est plus faible que l'AE car les prestations \
            parentales ont un coût actuariel moindre que les prestations régulières.",
            ctx.date_paie.year(), plafond, plafond * dec!(12),
            ts * dec!(100), tp * dec!(100),
        ),
        loi_ref: Some("RLRQ, ch. A-29.011 — Accord Canada-Québec (2005) — Règlement RQAP".into()),
    }
}

// ── FSS — Fonds des services de santé ────────────────────────────────────────

pub fn qc_fss(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let tp = ctx.taux_pat("QC_FSS");
    LigneCotisation {
        code:        "QC_FSS".into(),
        libelle:     "FSS — Fonds des services de santé (Québec)".into(),
        base:        brut,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (brut * tp).round_dp(2),
        categorie:   "Santé Québec".into(),
        explication: format!(
            "Le Fonds des services de santé (FSS) est une contribution patronale unique \
            au Québec, versée à Revenu Québec, qui finance le régime public d'assurance \
            maladie (RLRQ, ch. R-5). \
            \n\n\
            Taux affiché : {:.2} % (indicatif — masse salariale intermédiaire, secteur services)\n\
            Taux réel selon la masse salariale totale annuelle de l''entreprise :\n\
            • Masse <= 1 000 000 CAD : 1,65 %\n\
            • Masse 1 000 001–6 000 000 CAD : entre 1,65 % et 4,26 % (progressif)\n\
            • Masse > 6 000 000 CAD (services) : 4,26 %\n\
            • Masse > 6 000 000 CAD (secteur primaire/manufacturier) : 1,25 %\n\
            \n\
            Pas de plafond par salarié — assiette = totalité du salaire. \
            Déclaré et payé via le relevé 1 / TP-64.3.",
            tp * dec!(100),
        ),
        loi_ref: Some("RLRQ, ch. R-5, art. 34 — Revenu Québec TP-64.3".into()),
    }
}

// ── CNT — Contribution aux normes du travail ─────────────────────────────────

pub fn qc_cnt(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let plafond = maga_rqap_mensuel(ctx);
    let base    = brut.min(plafond);
    let tp      = ctx.taux_pat("QC_CNT");
    LigneCotisation {
        code:        "QC_CNT".into(),
        libelle:     "CNT — Contribution aux normes du travail (CNESST)".into(),
        base,
        taux_sal:    Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Autres".into(),
        explication: format!(
            "Contribution patronale de 0,06 % versée à la CNESST (Commission des normes, \
            de l''équité, de la santé et de la sécurité du travail), \
            anciennement CNT (Commission des normes du travail). \
            Finance les activités d''inspection des normes du travail, \
            l''aide aux travailleurs lésés et la promotion des droits. \
            Plafond identique au MAGA-RQAP ({:.0} CAD/an). \
            Très faible impact financier — souvent intégrée aux frais d''administration.",
            plafond * dec!(12),
        ),
        loi_ref: Some("RLRQ, ch. N-1.1, art. 39.0.2 — Loi sur les normes du travail".into()),
    }
}
