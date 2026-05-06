use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Plafonds annuels (en CAD/mois = annuel ÷ 12) ─────────────────────────────

/// MGA mensuel (Maximum des gains annuels ouvrant droit à pension).
pub fn mga_mensuel(ctx: &ContextPaie) -> Decimal {
    match ctx.date_paie.year() {
        i32::MIN..=2015 => dec!(4466.67),  // 53 600 / 12
        2016            => dec!(4575.00),  // 54 900 / 12
        2017            => dec!(4608.33),  // 55 300 / 12
        2018            => dec!(4658.33),  // 55 900 / 12
        2019            => dec!(4783.33),  // 57 400 / 12
        2020            => dec!(4891.67),  // 58 700
        2021            => dec!(5133.33),  // 61 600
        2022            => dec!(5408.33),  // 64 900
        2023            => dec!(5550.00),  // 66 600
        2024            => dec!(5708.33),  // 68 500
        2025            => dec!(5941.67),  // 71 300
        _               => dec!(6133.33),  // 2026+ estimation
    }
}

/// MGAP2 mensuel (plafond RPC2/RRQ2, dès 2024).
pub fn mgap2_mensuel(ctx: &ContextPaie) -> Decimal {
    match ctx.date_paie.year() {
        2024 => dec!(6100.00),  // 73 200 / 12
        2025 => dec!(6825.00),  // 81 900 / 12
        _    => dec!(7033.33),  // 2026+ estimation
    }
}

/// MAGA mensuel (Maximum des gains assurables AE).
pub fn maga_mensuel(ctx: &ContextPaie) -> Decimal {
    match ctx.date_paie.year() {
        i32::MIN..=2015 => dec!(4125.00),  // 49 500 / 12
        2016            => dec!(4233.33),  // 50 800 / 12
        2017            => dec!(4275.00),  // 51 300 / 12
        2018            => dec!(4308.33),  // 51 700 / 12
        2019            => dec!(4425.00),  // 53 100 / 12
        2020            => dec!(4516.67),  // 54 200
        2021            => dec!(4691.67),  // 56 300
        2022            => dec!(5025.00),  // 60 300
        2023            => dec!(5125.00),  // 61 500
        2024            => dec!(5266.67),  // 63 200
        2025            => dec!(5475.00),  // 65 700
        _               => dec!(5691.67),  // 2026+ estimation
    }
}

/// Exonération de base mensuelle RPC/RRQ (3 500 CAD/an ÷ 12).
pub const EXEMPTION_BASE: Decimal = dec!(291.67);

// ── RPC — Régime de pensions du Canada ───────────────────────────────────────

pub fn ca_rpc(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let mga      = mga_mensuel(ctx);
    let pensionn = (brut.min(mga) - EXEMPTION_BASE).max(Decimal::ZERO);
    let ts = ctx.taux_sal("CA_RPC");
    let tp = ctx.taux_pat("CA_RPC");
    LigneCotisation {
        code:        "CA_RPC".into(),
        libelle:     "RPC — Régime de pensions du Canada".into(),
        base:        pensionn,
        taux_sal:    ts,
        montant_sal: (pensionn * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (pensionn * tp).round_dp(2),
        categorie:   "Retraite fédérale".into(),
        explication: format!(
            "Le Régime de pensions du Canada (RPC / CPP) est le régime de retraite \
            obligatoire fédéral, en vigueur depuis 1966 (L.R.C. 1985, ch. C-8). \
            Il verse des rentes de retraite, d'invalidité et de survivant. \
            \n\n\
            [ Calcul {} ]\n\
            Gains pensionnables = min(brut, MGA/12) − exonération de base\n\
            = min({:.2}, {:.2}) − {:.2} = {:.2} CAD\n\
            Taux : {:.2} % salarié = {:.2} % employeur (taux appariés)\n\
            \n\
            La bonification progressive 2019-2023 a porté le taux de 4,95 % à 5,95 %. \
            Les gains au-delà du MGA ({:.2} CAD/mois) ne donnent pas droit à pension \
            dans le régime de base (voir RPC2 pour la phase 2). \
            L'exonération de base ({:.2} CAD/mois = 3 500 CAD/an) s'applique à tous.",
            ctx.date_paie.year(),
            brut, mga, EXEMPTION_BASE, pensionn,
            ts * dec!(100), tp * dec!(100),
            mga, EXEMPTION_BASE,
        ),
        loi_ref: Some("L.R.C. 1985, ch. C-8, art. 8 et 9 — L.C. 2018, ch. 12".into()),
    }
}

// ── RPC2 — Bonification supplémentaire (dès 2024) ────────────────────────────

pub fn ca_rpc2(brut: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    let ts = ctx.taux_sal("CA_RPC2");
    if ts == Decimal::ZERO {
        return None; // pas encore en vigueur
    }
    let mga   = mga_mensuel(ctx);
    let mgap2 = mgap2_mensuel(ctx);
    let base2 = (brut.min(mgap2) - mga).max(Decimal::ZERO);
    if base2 == Decimal::ZERO {
        return None;
    }
    let tp = ctx.taux_pat("CA_RPC2");
    Some(LigneCotisation {
        code:        "CA_RPC2".into(),
        libelle:     "RPC2 — Bonification supplémentaire (phase 2)".into(),
        base:        base2,
        taux_sal:    ts,
        montant_sal: (base2 * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base2 * tp).round_dp(2),
        categorie:   "Retraite fédérale".into(),
        explication: format!(
            "Le RPC2 (phase 2 de la bonification du RPC) s'applique sur la tranche de gains \
            comprise entre le MGA ({:.2} CAD/mois) et le MGAP2 ({:.2} CAD/mois). \
            \n\n\
            Gains supplémentaires {} : {:.2} CAD\n\
            Taux : 4,00 % salarié + 4,00 % employeur (sans exonération de base)\n\
            \n\
            Le RPC2 cible les travailleurs à revenus moyens-élevés qui cotisent déjà \
            au maximum du RPC de base. À terme, la pension RPC2 représentera \
            un remplacement de revenu supérieur au régime de base seul. \
            Introduit par la Loi no 1 d'exécution du budget de 2018 (L.C. 2018, ch. 12).",
            mga, mgap2, ctx.date_paie.year(), base2,
        ),
        loi_ref: Some("L.C. 2018, ch. 12, art. 5 — Règlement sur le MGAP2".into()),
    })
}

// ── AE — Assurance-emploi (Canada hors Québec) ───────────────────────────────

pub fn ca_ae(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let maga  = maga_mensuel(ctx);
    let base  = brut.min(maga);
    let ts    = ctx.taux_sal("CA_AE");
    let tp    = ctx.taux_pat("CA_AE");
    LigneCotisation {
        code:        "CA_AE".into(),
        libelle:     "AE — Assurance-emploi (régime général)".into(),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Chômage fédéral".into(),
        explication: format!(
            "L'Assurance-emploi (AE / EI) est le régime fédéral d'indemnisation du chômage, \
            régi par la Loi sur l'assurance-emploi (L.C. 1996, ch. 23). \
            Elle verse des prestations régulières (chômage), spéciales (maladie, maternité, \
            paternité, soignants) et de travail partagé. \
            \n\n\
            MAGA {} : {:.2} CAD/mois ({:.0} CAD/an)\n\
            Taux salarié : {:.2} % — Taux employeur : {:.2} % (= salarié × 1,4)\n\
            \n\
            Le multiplicateur 1,4 est fixé par l'art. 68 de la LAE. L'employeur cotise \
            davantage pour financer le risque global assumé par le régime. \
            Les travailleurs québécois paient un taux réduit car le RQAP couvre \
            leurs prestations parentales (voir taux QC_AE).",
            ctx.date_paie.year(), maga, maga * dec!(12),
            ts * dec!(100), tp * dec!(100),
        ),
        loi_ref: Some("L.C. 1996, ch. 23, art. 66-68 — Règlement sur l'assurance-emploi".into()),
    }
}
