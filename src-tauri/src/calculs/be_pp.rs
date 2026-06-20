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
//   SPF Finances — Barèmes PP/BV 2015-2025 (circulaires annuelles)
//   CIR92 art. 130-145 (IPP, 4 tranches depuis réforme)
//   Décret flamand — Vlaamse korting (réduction flamande)
//   Décret wallon — additionnels régionaux

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Forfait frais professionnels (30 % du brut, plafonné) ────────────────────

fn forfait_fp(brut_annuel: Decimal, annee: i32) -> Decimal {
    let cap = match annee {
        i32::MIN..=2019 => dec!(4720),
        2020             => dec!(5100),
        2021             => dec!(5500),
        2022             => dec!(5750),
        2023 | 2024      => dec!(5940),
        _                => dec!(6010), // 2025+
    };
    (brut_annuel * dec!(0.30)).min(cap)
}

// ── Exonération de base (quotient exempté / belastingvrije som) ───────────────

fn exoneration(annee: i32) -> Decimal {
    match annee {
        i32::MIN..=2015 => dec!(7090),
        2016 | 2017      => dec!(7130),
        2018             => dec!(7270),
        2019             => dec!(8860),
        2020             => dec!(8990),
        2021             => dec!(9050),
        2022             => dec!(9270),
        2023             => dec!(10160),
        2024             => dec!(10570),
        _                => dec!(10910), // 2025+
    }
}

// ── Barème IPP fédéral (annuel, sur revenu imposable net) ────────────────────
//
// 4 tranches (25 / 40 / 45 / 50 %) + quotient exempté.
// Les seuils A/B/C sont indexés annuellement.

fn ipp_annuel(revenu_net: Decimal, annee: i32) -> Decimal {
    let (a, b, c) = match annee {
        i32::MIN..=2016 => (dec!(11070), dec!(12720), dec!(21190)),
        2017             => (dec!(11070), dec!(12720), dec!(21190)),
        2018             => (dec!(12990), dec!(21170), dec!(38830)),
        2019             => (dec!(13250), dec!(23390), dec!(40453)),
        2020             => (dec!(13540), dec!(23900), dec!(41360)),
        2021             => (dec!(13540), dec!(23900), dec!(41360)),
        2022             => (dec!(13870), dec!(24800), dec!(42370)),
        2023             => (dec!(15200), dec!(26830), dec!(46440)),
        2024             => (dec!(15820), dec!(27920), dec!(48320)),
        _                => (dec!(16340), dec!(28830), dec!(49900)), // 2025+
    };

    if revenu_net <= Decimal::ZERO {
        return Decimal::ZERO;
    }
    if revenu_net <= a {
        revenu_net * dec!(0.25)
    } else if revenu_net <= b {
        a * dec!(0.25) + (revenu_net - a) * dec!(0.40)
    } else if revenu_net <= c {
        a * dec!(0.25) + (b - a) * dec!(0.40) + (revenu_net - b) * dec!(0.45)
    } else {
        a * dec!(0.25) + (b - a) * dec!(0.40) + (c - b) * dec!(0.45) + (revenu_net - c) * dec!(0.50)
    }
}

// ── Coefficient régional ──────────────────────────────────────────────────────
//
// Appliqué sur le PP fédéral calculé (après exonération).
// Flandres : réduction flamande (Vlaamse korting) → coefficient < 1
//   ≤2022 : ~5,5 % (≈ 0,945) — 2023+ : ~5,47 % (≈ 0,9453)
//   Source : circulaires SPF Finances — tables PP/BV Vlaanderen
// Wallonie : additionnels régionaux ~9 % → coefficient = 1,09 (stable depuis 2015)
// Bruxelles : pas d'additionnel → coefficient = 1

fn facteur_regional(region: &str, annee: i32) -> Decimal {
    match region {
        "wallonie" => dec!(1.09),
        "flandre"  => match annee {
            i32::MIN..=2022 => dec!(0.945),  // Vlaamse korting ~5,5 %
            _               => dec!(0.9453), // 2023+ ~5,47 %
        },
        _ => dec!(1.00), // bruxelles ou inconnu
    }
}

// ── Précompte professionnel mensuel ──────────────────────────────────────────

pub fn precompte_professionnel(brut: Decimal, region: &str, ctx: &ContextPaie) -> LigneCotisation {
    let annee      = ctx.date_paie.year();
    let brut_a     = brut * dec!(12);
    let fp         = forfait_fp(brut_a, annee);
    let revenu_net = (brut_a - fp).max(Decimal::ZERO);
    let exo        = exoneration(annee);

    // IPP brute = IPP(revenu_net) - IPP(exonération)
    let ipp_brute = (ipp_annuel(revenu_net, annee) - ipp_annuel(exo, annee)).max(Decimal::ZERO);
    let facteur   = facteur_regional(region, annee);
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
            Revenu annuel estimé          : {brut_a} € (× 12)\n\
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
