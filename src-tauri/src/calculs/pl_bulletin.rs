// ── Pologne — ZUS + składka zdrowotna + PIT ──────────────────────────────────
//
// Salarié secteur privé (umowa o pracę). Côté salarié :
//   • ZUS social : emerytalne 9,76 % + rentowe 1,5 % (plafonnées au 30-krotność)
//     + chorobowe 2,45 % (non plafonnée) ;
//   • składka zdrowotna 9 % sur (brut − ZUS social) ;
//   • PIT 12 % / 32 % avec montant réducteur 3 600 PLN/an et KUP 3 000 PLN/an.
//
// Taux ZUS lus en base (cotisation_taux). Plafond, santé et PIT calculés ici.
// Sources : Ustawa o systemie ubezpieczeń społecznych ; Ustawa o PIT ; NFZ.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

struct PlParams {
    plafond_30x_annuel: Decimal, // emerytalne + rentowe
    kup_annuel:    Decimal,      // koszty uzyskania przychodu
    kwota_relief:  Decimal,      // montant réducteur d'impôt annuel (kwota wolna)
    pit_seuil:     Decimal,      // seuil 32 %
    pit_t1:        Decimal,
    pit_t2:        Decimal,
    sante_taux:    Decimal,
}

/// Paramètres en base (migration 0153) : plafond annuel « 30-krotność »
/// (PL_PLAFOND_30X, relevé de 2022 à 2026) et structure du PIT, stable depuis la
/// réforme Polski Ład 2.0 de 2022 (12 % / 32 %, seuil 120 000, montant réducteur
/// 3 600, KUP 3 000, santé 9 % non déductible). ZUS salarial constant depuis 2012.
/// None si un paramètre manque à la date : régime non couvert.
fn pl_params(ctx: &ContextPaie) -> Option<PlParams> {
    let p = |c: &str| ctx.plafond(c);
    Some(PlParams {
        plafond_30x_annuel: p("PL_PLAFOND_30X")?,
        kup_annuel:   p("PL_KUP")?,
        kwota_relief: p("PL_KWOTA_RELIEF")?,
        pit_seuil:    p("PL_PIT_SEUIL")?,
        pit_t1:       p("PL_PIT_T1")?,
        pit_t2:       p("PL_PIT_T2")?,
        sante_taux:   p("PL_SANTE_TAUX")?,
    })
}

fn ligne(code: &str, libelle: &str, categorie: &str, base: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    let lib = ctx.libelle(code, libelle);
    let explication = ctx.expl("PL_GENERIC",
        "{libelle} — ZUS.\nTaux : {tsp} % sal / {tpp} % pat. Assiette : {base} PLN.\n\
        Salarié : {ms} PLN | Employeur : {mp} PLN.\n\nBase légale : Ustawa o systemie ubezpieczeń społecznych.")
        .replace("{libelle}", &lib)
        .replace("{tsp}", &format!("{:.2}", ts * dec!(100)))
        .replace("{tpp}", &format!("{:.2}", tp * dec!(100)))
        .replace("{base}", &format!("{:.2}", base))
        .replace("{ms}", &format!("{:.2}", (base * ts).round_dp(2)))
        .replace("{mp}", &format!("{:.2}", (base * tp).round_dp(2)));
    LigneCotisation {
        code: code.into(), libelle: lib, base,
        taux_sal: ts, montant_sal: (base * ts).round_dp(2),
        taux_pat: tp, montant_pat: (base * tp).round_dp(2),
        categorie: categorie.into(),
        explication,
        loi_ref: Some(ctx.loi_ref("Ustawa o systemie ubezpieczeń społecznych")),
    }
}

pub fn generer_bulletin_pl(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;

    let Some(p) = pl_params(ctx) else {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "PLN", "PL",
            "Pologne : barèmes intégrés de 2022 à 2026.", ctx);
    };

    // Plafond mensuel emerytalne + rentowe (annualisation du 30-krotność).
    let cap_m = (p.plafond_30x_annuel / dec!(12)).round_dp(2);
    let base_cap = brut.min(cap_m);

    let mut cotisations = vec![
        ligne("PL_EMERYTALNE", "Emerytalne — Vieillesse",        "Sécurité sociale", base_cap, ctx),
        ligne("PL_RENTOWE",    "Rentowe — Invalidité/décès",     "Sécurité sociale", base_cap, ctx),
        ligne("PL_CHOROBOWE",  "Chorobowe — Maladie",            "Sécurité sociale", brut,     ctx),
        ligne("PL_WYPADKOWE",  "Wypadkowe — Accidents (employeur)", "Cotisations patronales", brut, ctx),
        ligne("PL_FP",         "Fundusz Pracy (employeur)",      "Cotisations patronales", brut, ctx),
        ligne("PL_FGSP",       "FGŚP (employeur)",               "Cotisations patronales", brut, ctx),
    ];

    // ZUS social salarial (base santé + PIT)
    let zus_social_sal: Decimal = cotisations.iter()
        .filter(|c| matches!(c.code.as_str(), "PL_EMERYTALNE" | "PL_RENTOWE" | "PL_CHOROBOWE"))
        .map(|c| c.montant_sal).sum();

    // Składka zdrowotna : 9 % sur (brut − ZUS social salarial)
    let base_sante = (brut - zus_social_sal).max(Decimal::ZERO);
    let sante = (base_sante * p.sante_taux).round_dp(2);
    cotisations.push(LigneCotisation {
        code: "PL_ZDROWOTNE".into(),
        libelle: ctx.libelle("PL_ZDROWOTNE", "Składka zdrowotna — Assurance maladie (9 %)"),
        base: base_sante, taux_sal: p.sante_taux, montant_sal: sante,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("PL_ZDROWOTNE",
            "Składka zdrowotna — 9 % de l'assiette (brut − ZUS social salarial).\n\
            Assiette : {b} PLN → {s} PLN/mois. Non déductible du PIT depuis 2022.\n\n\
            Base légale : Ustawa o świadczeniach opieki zdrowotnej.")
            .replace("{b}", &format!("{:.2}", base_sante))
            .replace("{s}", &format!("{:.2}", sante)),
        loi_ref: Some(ctx.loi_ref("Ustawa o świadczeniach opieki zdrowotnej")),
    });

    // PIT (annualisé) : base = brut annuel − ZUS social annuel − KUP
    let brut_ann = brut * dec!(12);
    let zus_social_ann = zus_social_sal * dec!(12);
    let taxable = (brut_ann - zus_social_ann - p.kup_annuel).max(Decimal::ZERO);
    let pit_brut = if taxable <= p.pit_seuil {
        taxable * p.pit_t1
    } else {
        p.pit_seuil * p.pit_t1 + (taxable - p.pit_seuil) * p.pit_t2
    };
    let pit_ann = (pit_brut - p.kwota_relief).max(Decimal::ZERO);
    // Avance mensuelle (art. 32 ustawy o PIT) : base et avance arrondies au
    // złoty entier (Ordynacja podatkowa art. 63 : moins de 50 gr tronqués, 50 gr
    // et plus au złoty supérieur). Au-delà du seuil de 120 000 PLN, moyenne
    // annuelle, elle aussi arrondie au złoty.
    let zloty = |x: Decimal| x.round_dp_with_strategy(0, rust_decimal::RoundingStrategy::MidpointAwayFromZero);
    let base_mens = zloty(taxable / dec!(12));
    let pit_mens = if taxable <= p.pit_seuil {
        zloty((base_mens * p.pit_t1 - p.kwota_relief / dec!(12)).max(Decimal::ZERO))
    } else {
        zloty(pit_ann / dec!(12))
    };
    let taux_pit = if brut > Decimal::ZERO { (pit_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "PL_PIT".into(),
        libelle: ctx.libelle("PL_PIT", "PIT — Impôt sur le revenu"),
        base: brut, taux_sal: taux_pit, montant_sal: pit_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("PL_PIT",
            "Impôt sur le revenu (PIT) 2025 — annualisé.\n\n\
            Revenu annuel : {ba} PLN − ZUS social {za} PLN − KUP {kup} PLN\n\
            = base imposable {tx} PLN\n\
            Barème : 12 % jusqu'à 120 000 PLN, 32 % au-delà ; − montant réducteur 3 600 PLN.\n\
            = {pa} PLN/an / 12 = {pm} PLN/mois.\n\n\
            Base légale : Ustawa o PIT.")
            .replace("{ba}", &format!("{:.0}", brut_ann))
            .replace("{za}", &format!("{:.0}", zus_social_ann))
            .replace("{kup}", &format!("{:.0}", p.kup_annuel))
            .replace("{tx}", &format!("{:.0}", taxable))
            .replace("{pa}", &format!("{:.0}", pit_ann))
            .replace("{pm}", &format!("{:.2}", pit_mens)),
        loi_ref: Some(ctx.loi_ref("Ustawa o PIT")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "PLN".into(),
        absence: None,
        heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), pas: None, droits: None, alertes: Vec::new(), evolutions: Vec::new(),
        salarie,
    }
}
