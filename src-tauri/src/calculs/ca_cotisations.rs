use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Plafonds annuels, lus en base (CAD/an ÷ 12, au cent) ─────────────────────
// CA_MGA (RPC/RRQ), CA_MGAP2 (RPC2/RRQ2, dès 2024), CA_MAGA (AE) : migrations
// 0021 et 0154.

fn mensuel(ctx: &ContextPaie, code: &str) -> Decimal {
    ctx.plafond(code).map(|a| (a / dec!(12)).round_dp(2)).unwrap_or_default()
}

/// Plafonds de cotisation présents en base à la date de paie.
pub fn couvert(ctx: &ContextPaie) -> bool {
    ctx.plafond("CA_MGA").is_some() && ctx.plafond("CA_MAGA").is_some()
}

/// MGA mensuel (Maximum des gains annuels ouvrant droit à pension).
pub fn mga_mensuel(ctx: &ContextPaie) -> Decimal { mensuel(ctx, "CA_MGA") }

/// MGAP2 mensuel (plafond RPC2/RRQ2, dès 2024 ; nul avant).
pub fn mgap2_mensuel(ctx: &ContextPaie) -> Decimal { mensuel(ctx, "CA_MGAP2") }

/// MAGA mensuel (Maximum des gains assurables AE).
pub fn maga_mensuel(ctx: &ContextPaie) -> Decimal { mensuel(ctx, "CA_MAGA") }

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
        libelle:     ctx.libelle("CA_RPC", "RPC — Régime de pensions du Canada"),
        base:        pensionn,
        taux_sal:    ts,
        montant_sal: (pensionn * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (pensionn * tp).round_dp(2),
        categorie:   "Retraite fédérale".into(),
        explication: ctx.expl("CA_RPC",
            "Le Régime de pensions du Canada (RPC / CPP) est le régime de retraite \
            obligatoire fédéral, en vigueur depuis 1966 (L.R.C. 1985, ch. C-8). \
            Il verse des rentes de retraite, d'invalidité et de survivant. \
            \n\n\
            [ Calcul {an} ]\n\
            Gains pensionnables = min(brut, MGA/12) − exonération de base\n\
            = min({brut}, {mga}) − {exo} = {pens} CAD\n\
            Taux : {ts} % salarié = {tp} % employeur (taux appariés)\n\
            \n\
            La bonification progressive 2019-2023 a porté le taux de 4,95 % à 5,95 %. \
            Les gains au-delà du MGA ({mga} CAD/mois) ne donnent pas droit à pension \
            dans le régime de base (voir RPC2 pour la phase 2). \
            L'exonération de base ({exo} CAD/mois = 3 500 CAD/an) s'applique à tous.")
            .replace("{an}", &ctx.date_paie.year().to_string())
            .replace("{brut}", &format!("{:.2}", brut))
            .replace("{mga}", &format!("{:.2}", mga))
            .replace("{exo}", &format!("{:.2}", EXEMPTION_BASE))
            .replace("{pens}", &format!("{:.2}", pensionn))
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100))),
        loi_ref: Some(ctx.loi_ref("L.R.C. 1985, ch. C-8, art. 8 et 9 — L.C. 2018, ch. 12")),
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
        libelle:     ctx.libelle("CA_RPC2", "RPC2 — Bonification supplémentaire (phase 2)"),
        base:        base2,
        taux_sal:    ts,
        montant_sal: (base2 * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base2 * tp).round_dp(2),
        categorie:   "Retraite fédérale".into(),
        explication: ctx.expl("CA_RPC2",
            "Le RPC2 (phase 2 de la bonification du RPC) s'applique sur la tranche de gains \
            comprise entre le MGA ({mga} CAD/mois) et le MGAP2 ({mgap2} CAD/mois). \
            \n\n\
            Gains supplémentaires {an} : {base2} CAD\n\
            Taux : 4,00 % salarié + 4,00 % employeur (sans exonération de base)\n\
            \n\
            Le RPC2 cible les travailleurs à revenus moyens-élevés qui cotisent déjà \
            au maximum du RPC de base. À terme, la pension RPC2 représentera \
            un remplacement de revenu supérieur au régime de base seul. \
            Introduit par la Loi no 1 d'exécution du budget de 2018 (L.C. 2018, ch. 12).")
            .replace("{mgap2}", &format!("{:.2}", mgap2))
            .replace("{mga}", &format!("{:.2}", mga))
            .replace("{an}", &ctx.date_paie.year().to_string())
            .replace("{base2}", &format!("{:.2}", base2)),
        loi_ref: Some(ctx.loi_ref("L.C. 2018, ch. 12, art. 5 — Règlement sur le MGAP2")),
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
        libelle:     ctx.libelle("CA_AE", "AE — Assurance-emploi (régime général)"),
        base,
        taux_sal:    ts,
        montant_sal: (base * ts).round_dp(2),
        taux_pat:    tp,
        montant_pat: (base * tp).round_dp(2),
        categorie:   "Chômage fédéral".into(),
        explication: ctx.expl("CA_AE",
            "L'Assurance-emploi (AE / EI) est le régime fédéral d'indemnisation du chômage, \
            régi par la Loi sur l'assurance-emploi (L.C. 1996, ch. 23). \
            Elle verse des prestations régulières (chômage), spéciales (maladie, maternité, \
            paternité, soignants) et de travail partagé. \
            \n\n\
            MAGA {an} : {maga} CAD/mois ({magaa} CAD/an)\n\
            Taux salarié : {ts} % — Taux employeur : {tp} % (= salarié × 1,4)\n\
            \n\
            Le multiplicateur 1,4 est fixé par l'art. 68 de la LAE. L'employeur cotise \
            davantage pour financer le risque global assumé par le régime. \
            Les travailleurs québécois paient un taux réduit car le RQAP couvre \
            leurs prestations parentales (voir taux QC_AE).")
            .replace("{an}", &ctx.date_paie.year().to_string())
            .replace("{magaa}", &format!("{:.0}", maga * dec!(12)))
            .replace("{maga}", &format!("{:.2}", maga))
            .replace("{ts}", &format!("{:.2}", ts * dec!(100)))
            .replace("{tp}", &format!("{:.2}", tp * dec!(100))),
        loi_ref: Some(ctx.loi_ref("L.C. 1996, ch. 23, art. 66-68 — Règlement sur l'assurance-emploi")),
    }
}
