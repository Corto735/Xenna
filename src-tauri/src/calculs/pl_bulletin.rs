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

use chrono::Datelike;
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

fn pl_params(annee: i32) -> Option<PlParams> {
    // Depuis 2023 : structure PIT stable (12 % / 32 %, seuil 120 000, montant réducteur
    // 3 600, KUP 3 000) et składka zdrowotna 9 % non déductible. Seul le plafond annuel
    // ZUS (« 30-krotność ») change chaque année. ZUS salarial constant depuis 2012.
    // 2022 : réforme « Polski Ład 2.0 » → 12 %/32 %, seuil 120 000, kwota wolna 30 000 pour
    // toute l'année 2022 (santé non déductible depuis 2022). Même structure que 2023+.
    let plafond_30x = match annee {
        2022 => dec!(177660),
        2023 => dec!(208050),
        2024 => dec!(234720),
        2025 => dec!(260190),
        2026 => dec!(282600),
        _ => return None,
    };
    Some(PlParams {
        plafond_30x_annuel: plafond_30x,
        kup_annuel:   dec!(3000),
        kwota_relief: dec!(3600),
        pit_seuil:    dec!(120000),
        pit_t1:       dec!(0.12),
        pit_t2:       dec!(0.32),
        sante_taux:   dec!(0.09),
    })
}

fn ligne(code: &str, libelle: &str, categorie: &str, base: Decimal, ctx: &ContextPaie) -> LigneCotisation {
    let ts = ctx.taux_sal(code);
    let tp = ctx.taux_pat(code);
    LigneCotisation {
        code: code.into(), libelle: libelle.into(), base,
        taux_sal: ts, montant_sal: (base * ts).round_dp(2),
        taux_pat: tp, montant_pat: (base * tp).round_dp(2),
        categorie: categorie.into(),
        explication: format!(
            "{libelle} — ZUS.\nTaux : {tsp:.2} % sal / {tpp:.2} % pat. Assiette : {base:.2} PLN.\n\
            Salarié : {ms:.2} PLN | Employeur : {mp:.2} PLN.\n\nBase légale : Ustawa o systemie ubezpieczeń społecznych.",
            tsp = ts * dec!(100), tpp = tp * dec!(100), base = base,
            ms = (base * ts).round_dp(2), mp = (base * tp).round_dp(2),
        ),
        loi_ref: Some("Ustawa o systemie ubezpieczeń społecznych".into()),
    }
}

pub fn generer_bulletin_pl(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    let Some(p) = pl_params(annee) else {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "PLN", "Pologne : données disponibles pour 2025.");
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
        libelle: "Składka zdrowotna — Assurance maladie (9 %)".into(),
        base: base_sante, taux_sal: p.sante_taux, montant_sal: sante,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: format!(
            "Składka zdrowotna — 9 % de l'assiette (brut − ZUS social salarial).\n\
            Assiette : {b:.2} PLN → {s:.2} PLN/mois. Non déductible du PIT depuis 2022.\n\n\
            Base légale : Ustawa o świadczeniach opieki zdrowotnej.",
            b = base_sante, s = sante,
        ),
        loi_ref: Some("Ustawa o świadczeniach opieki zdrowotnej".into()),
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
    let pit_mens = (pit_ann / dec!(12)).round_dp(2);
    let taux_pit = if brut > Decimal::ZERO { (pit_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "PL_PIT".into(),
        libelle: "PIT — Impôt sur le revenu".into(),
        base: brut, taux_sal: taux_pit, montant_sal: pit_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: format!(
            "Impôt sur le revenu (PIT) 2025 — annualisé.\n\n\
            Revenu annuel : {ba:.0} PLN − ZUS social {za:.0} PLN − KUP {kup:.0} PLN\n\
            = base imposable {tx:.0} PLN\n\
            Barème : 12 % jusqu'à 120 000 PLN, 32 % au-delà ; − montant réducteur 3 600 PLN.\n\
            = {pa:.0} PLN/an / 12 = {pm:.2} PLN/mois.\n\n\
            Base légale : Ustawa o PIT.",
            ba = brut_ann, za = zus_social_ann, kup = p.kup_annuel,
            tx = taxable, pa = pit_ann, pm = pit_mens,
        ),
        loi_ref: Some("Ustawa o PIT".into()),
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
        salarie,
    }
}
