// ── Précompte Professionnel belge (PP/Bedrijfsvoorheffing) ────────────────────
//
// Méthode : annualisation (brut × 12), déduction forfait frais professionnels,
//           barème IPP fédéral 4 tranches, exonération de base, coefficient
//           régional (Wallonie / Flandres / Bruxelles), division par 12.
//
// Note : les tables officielles SPF Finances (barèmes PP/BV publiés chaque
//        janvier) tiennent compte de la situation familiale et d'autres
//        déductions. Le calcul par barème annualisé est une approximation
//        (même approche que IT_IRPEF et PT_IRS dans ce projet).
//
// Sources :
//   SPF Finances — Barèmes PP/BV 2015-2026 (circulaires annuelles) ; montants indexés
//   revenus 2025 et 2026 (SPF Finances, 12/02/2026)
//   CIR92 art. 130-145 (IPP, 4 tranches depuis réforme)
//   Décret flamand — Vlaamse korting (réduction flamande)
//   Décret wallon — additionnels régionaux

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Paramètres (migration 0152) ───────────────────────────────────────────────
//
// Forfait de frais professionnels BE_FP_TAUX du brut, plafonné à BE_FP_PLAFOND ;
// quotité exemptée BE_EXONERATION ; barème IPP BE_IPP (25 / 40 / 45 / 50 %,
// seuils indexés chaque année) ; coefficients régionaux appliqués au PP
// fédéral : Wallonie, additionnels (BE_FACTEUR_WALLONIE) ; Flandres, réduction
// flamande (BE_FACTEUR_FLANDRE) ; Bruxelles, aucun (1).

/// Le régime est-il couvert à la date de paie ? (be_bulletin)
pub fn couvert(ctx: &ContextPaie) -> bool {
    ctx.bareme("BE_IPP").is_some()
        && ["BE_FP_TAUX", "BE_FP_PLAFOND", "BE_EXONERATION", "BE_FACTEUR_WALLONIE", "BE_FACTEUR_FLANDRE"]
            .iter().all(|c| ctx.plafond(c).is_some())
}

fn forfait_fp(brut_annuel: Decimal, ctx: &ContextPaie) -> Decimal {
    let p = |c: &str| ctx.plafond(c).unwrap_or(Decimal::ZERO);
    (brut_annuel * p("BE_FP_TAUX")).min(p("BE_FP_PLAFOND"))
}

fn ipp_annuel(revenu_net: Decimal, ctx: &ContextPaie) -> Decimal {
    if revenu_net <= Decimal::ZERO {
        return Decimal::ZERO;
    }
    ctx.bareme_progressif("BE_IPP", revenu_net).unwrap_or(Decimal::ZERO)
}

fn facteur_regional(region: &str, ctx: &ContextPaie) -> Decimal {
    match region {
        "wallonie" => ctx.plafond("BE_FACTEUR_WALLONIE"),
        "flandre"  => ctx.plafond("BE_FACTEUR_FLANDRE"),
        _ => None, // bruxelles ou inconnu
    }.unwrap_or(Decimal::ONE)
}

// ── Précompte professionnel mensuel ──────────────────────────────────────────

pub fn precompte_professionnel(brut: Decimal, region: &str, ctx: &ContextPaie) -> LigneCotisation {
    let annee      = ctx.date_paie.year();
    // Rémunération imposable = brut − cotisation ONSS personnelle (formule-clé,
    // annexe III AR/CIR 92), annualisée.
    let onss_sal   = (brut * ctx.taux_sal("BE_ONSS_SAL")).round_dp(2);
    let brut_a     = (brut - onss_sal) * dec!(12);
    let fp         = forfait_fp(brut_a, ctx);
    let revenu_net = (brut_a - fp).max(Decimal::ZERO);
    let exo        = ctx.plafond("BE_EXONERATION").unwrap_or(Decimal::ZERO);

    // IPP brute = IPP(revenu_net) - IPP(exonération)
    let ipp_brute = (ipp_annuel(revenu_net, ctx) - ipp_annuel(exo, ctx)).max(Decimal::ZERO);
    let facteur   = facteur_regional(region, ctx);
    let pp_annuel = ipp_brute * facteur;
    let pp_mensuel = (pp_annuel / dec!(12)).round_dp(2);

    let taux_eff = if brut > Decimal::ZERO { (pp_mensuel / brut).round_dp(4) } else { Decimal::ZERO };

    let korting_pct = ((dec!(1) - facteur) * dec!(100)).round_dp(2);
    let region_label = match region {
        "wallonie" => ctx.libelle("BE_REG_WALLONIE", "Wallonie (+9 % additionnels régionaux)"),
        "flandre"  => ctx.libelle("BE_REG_FLANDRE", "Flandres (réduction flamande −{korting} %)")
            .replace("{korting}", &korting_pct.to_string()),
        _          => ctx.libelle("BE_REG_BXL", "Bruxelles-Capitale (pas d'additionnel)"),
    };

    LigneCotisation {
        code:        "BE_PP".into(),
        libelle:     ctx.libelle("BE_PP", "Précompte professionnel (PP/BV) {annee} — {rl}")
            .replace("{annee}", &annee.to_string())
            .replace("{rl}", &region_label),
        base:        brut,
        taux_sal:    taux_eff,
        montant_sal: pp_mensuel,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt sur le revenu".into(),
        explication: ctx.expl("BE_PP",
            "Retenue mensuelle à la source de l'IPP (Impôt des Personnes Physiques). \
            L'employeur est tenu d'effectuer cette retenue (art. 270 et s. CIR92). \
            Régularisation lors de la déclaration IPP annuelle.\n\n\
            [ Calcul {annee} — région : {rl} ]\n\
            Salaire brut mensuel          : {brut} €\n\
            Rémunération imposable       : {brut_a} € ((brut − ONSS) × 12)\n\
            Forfait frais pro (30 %,  cap): − {fp} €\n\
            Revenu net imposable estimé   : {rn} €\n\
            IPP brute annuelle            : {ipp} €\n\
            Exonération de base           : − IPP({exo} €) déduite\n\
            Coefficient régional          : × {fac}\n\
            PP annuel                     : {ppa} €\n\
            Précompte mensuel             : {ppm} € (÷ 12)\n\
            Taux effectif                 : {teff} %\n\
            \n\
            Note : approximation par barème annualisé. \
            Les tables officielles SPF Finances tiennent compte de la \
            situation familiale et d'autres déductions.\n\
            Base légale : CIR92 art. 130-145 + Circulaires SPF Finances {annee}.")
            .replace("{annee}", &annee.to_string())
            .replace("{rl}", &region_label)
            .replace("{brut_a}", &format!("{:.2}", brut_a))
            .replace("{brut}", &format!("{:.2}", brut))
            .replace("{fp}", &format!("{:.2}", fp))
            .replace("{rn}", &format!("{:.2}", revenu_net))
            .replace("{ipp}", &format!("{:.2}", ipp_brute))
            .replace("{exo}", &format!("{:.0}", exo))
            .replace("{fac}", &format!("{:.2}", facteur))
            .replace("{ppa}", &format!("{:.2}", pp_annuel))
            .replace("{ppm}", &format!("{:.2}", pp_mensuel))
            .replace("{teff}", &format!("{:.2}", taux_eff * dec!(100))),
        loi_ref: Some(ctx.loi_ref(&format!(
            "CIR92 art. 130-145 + Circulaire SPF Finances PP/BV {annee}"
        ))),
    }
}
