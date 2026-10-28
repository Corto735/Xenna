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
// Forfait PT_DEDUCAO_ESPECIFICA, barème PT_IRS et table PT_TABELA_I en base
// (migration 0152).

/// Le régime est-il couvert à la date de paie ? (pt_bulletin)
pub fn couvert(ctx: &ContextPaie) -> bool {
    ctx.bareme("PT_IRS").is_some() && ctx.plafond("PT_DEDUCAO_ESPECIFICA").is_some()
}

/// IRS annuelle sur le revenu imposable : barème progressif PT_IRS.
pub fn irs_annuel(rendimento: Decimal, ctx: &ContextPaie) -> Decimal {
    ctx.bareme_progressif("PT_IRS", rendimento).unwrap_or(Decimal::ZERO)
}

// ── Tables officielles de retenue (depuis 2026) ──────────────────────────────
//
// Table I du Continent, travail salarié, non marié sans personne à charge (ou
// marié deux titulaires) — Despacho n.º 233-A/2026. Retenue = R × taux −
// parcela a abater ; pour les deux premières zones, la parcela vaut
// taux × k × (C − R). Renvoie (taux, parcela) pour la rémunération mensuelle R,
// ou None si la table n'est pas en base à la date.
fn tabela_i(r: Decimal, ctx: &ContextPaie) -> Option<(Decimal, Decimal)> {
    let p = |c: &str| ctx.plafond(c);
    if r <= p("PT_TAB_EXEMPT")? {
        return Some((Decimal::ZERO, Decimal::ZERO));
    }
    for z in ["Z1", "Z2"] {
        if r <= p(&format!("PT_TAB_{z}_MAX"))? {
            let t = p(&format!("PT_TAB_{z}_TAUX"))?;
            return Some((t, t * p(&format!("PT_TAB_{z}_K"))? * (p(&format!("PT_TAB_{z}_C"))? - r)));
        }
    }
    let ligne = ctx.bareme("PT_TABELA_I")?.iter().find(|t| t.borne_sup.map_or(true, |b| r <= b))?;
    Some((ligne.taux, ligne.fixe))
}

// ── Retenção na fonte mensuelle ───────────────────────────────────────────────

pub fn irs_retencao(brut: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let annee        = ctx.date_paie.year();
    let rendimento_a = brut * dec!(12);

    // Dedução específica : max(SS annuel, forfait légal)
    let ss_annuel     = (rendimento_a * ctx.plafond("PT_SS_DEDUCTIBLE_TAUX").unwrap_or(Decimal::ZERO)).round_dp(2);
    let deducao_min   = ctx.plafond("PT_DEDUCAO_ESPECIFICA").unwrap_or(Decimal::ZERO);
    let deducao       = ss_annuel.max(deducao_min);

    let base_irs      = (rendimento_a - deducao).max(Decimal::ZERO);
    let irs_anual     = irs_annuel(base_irs, ctx);
    let irs_estime    = (irs_anual / dec!(12)).round_dp(2);
    // Depuis 2026, la retenue suit la table officielle ; le calcul annualisé
    // reste affiché à titre indicatif.
    let tabela = tabela_i(brut, ctx);
    let irs_mensal = match tabela {
        Some((taux, parcela)) => (brut * taux - parcela).max(Decimal::ZERO).round_dp(2),
        None => irs_estime,
    };

    let taux_eff = if brut > Decimal::ZERO {
        (irs_mensal / brut).round_dp(4)
    } else {
        Decimal::ZERO
    };

    let nb_tranches = ctx.bareme("PT_IRS").map_or(0, |t| t.len());

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
