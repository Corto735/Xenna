// Bulletin de paie brésilien (regime CLT) : INSS + IRRF salarié, FGTS + INSS
// patronal côté employeur. Devise BRL. Données : 2025 et 2026.
//
// Salarié : INSS progressif par tranches (7,5 / 9 / 12 / 14 %) plafonné au
// teto ; IRRF mensuel (barème progressif, base = brut − INSS ou desconto
// simplificado, le plus favorable). Employeur : FGTS 8 % + INSS patronal 20 %
// (RAT/terceiros ~ jusqu'à +8,8 % non détaillés — lacune assumée). Barèmes en
// Rust ; taux INSS/FGTS/patronal en base.
//
// Sources : Lei 8.212/1991 (INSS) + Portaria interministerial MPS/MF 2025 ;
// Lei 7.713/1988 + tabela IRRF (IN RFB 2025) ; Lei 8.036/1990 (FGTS).

use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

/// Tranches INSS : (limite supérieure, taux). La dernière borne est le teto.
/// 2025 : Portaria MPS/MF 6/2025 ; 2026 : Portaria Interministerial MPS/MF 13/2026.
fn tranches_inss(annee: i32) -> [(Decimal, Decimal); 4] {
    if annee >= 2026 {
        [
            (dec!(1621.00), dec!(0.075)),
            (dec!(2902.84), dec!(0.09)),
            (dec!(4354.27), dec!(0.12)),
            (dec!(8475.55), dec!(0.14)),
        ]
    } else {
        [
            (dec!(1518.00), dec!(0.075)),
            (dec!(2793.88), dec!(0.09)),
            (dec!(4190.83), dec!(0.12)),
            (dec!(8157.41), dec!(0.14)),
        ]
    }
}

/// INSS salarié progressif : chaque tranche à son taux, plafonné au teto.
fn calcul_inss(brut: Decimal, annee: i32) -> Decimal {
    let tr = tranches_inss(annee);
    let mut inss = Decimal::ZERO;
    let mut bas = Decimal::ZERO;
    for (haut, taux) in tr {
        let assiette = brut.min(haut) - bas;
        if assiette > Decimal::ZERO {
            inss += assiette * taux;
        }
        bas = haut;
        if brut <= haut { break; }
    }
    inss.round_dp(2)
}

/// Barème IRRF mensuel : (limite inférieure, taux, part à déduire) et desconto
/// simplificado. Janvier-avril 2025 : table de 2024 ; dès mai 2025 (Lei
/// 15.191/2025) : exonération jusqu'à 2 428,80 R$, reconduite en 2026.
fn bareme_irrf(d: NaiveDate) -> ([(Decimal, Decimal, Decimal); 5], Decimal) {
    if d < NaiveDate::from_ymd_opt(2025, 5, 1).unwrap() {
        ([
            (dec!(0.00),    dec!(0.000), dec!(0.00)),
            (dec!(2259.21), dec!(0.075), dec!(169.44)),
            (dec!(2826.66), dec!(0.150), dec!(381.44)),
            (dec!(3751.06), dec!(0.225), dec!(662.77)),
            (dec!(4664.69), dec!(0.275), dec!(896.00)),
        ], dec!(564.80))
    } else {
        ([
            (dec!(0.00),    dec!(0.000), dec!(0.00)),
            (dec!(2428.81), dec!(0.075), dec!(182.16)),
            (dec!(2826.66), dec!(0.150), dec!(394.16)),
            (dec!(3751.06), dec!(0.225), dec!(675.49)),
            (dec!(4664.69), dec!(0.275), dec!(908.73)),
        ], dec!(607.20))
    }
}

/// Réduction mensuelle de la Lei 15.270/2025 (dès 2026), sur les revenus
/// imposables bruts : impôt annulé jusqu'à 5 000 R$ ; entre 5 000,01 et 7 350 R$,
/// réduction de 978,62 − 0,133145 × revenu ; rien au-delà.
fn reducao_15270(rendimento: Decimal, irrf: Decimal, annee: i32) -> Decimal {
    if annee < 2026 {
        return Decimal::ZERO;
    }
    let r = if rendimento <= dec!(5000) {
        irrf
    } else if rendimento <= dec!(7350) {
        (dec!(978.62) - dec!(0.133145) * rendimento).max(Decimal::ZERO)
    } else {
        Decimal::ZERO
    };
    r.min(irrf).round_dp(2)
}

/// IRRF mensuel : base = brut − max(INSS, desconto simplificado) ; puis barème,
/// puis réduction de la Lei 15.270/2025. Retourne (irrf, base, taux marginal, réduction).
fn calcul_irrf(brut: Decimal, inss: Decimal, d: NaiveDate) -> (Decimal, Decimal, Decimal, Decimal) {
    let (bareme, desconto) = bareme_irrf(d);
    let deduction = inss.max(desconto);
    let base = (brut - deduction).max(Decimal::ZERO);
    let mut choisi = bareme[0];
    for &t in bareme.iter() {
        if base >= t.0 { choisi = t; } else { break; }
    }
    let (_li, taux, deduire) = choisi;
    let brut_irrf = (base * taux - deduire).max(Decimal::ZERO).round_dp(2);
    let reduction = reducao_15270(brut, brut_irrf, d.year());
    (brut_irrf - reduction, base.round_dp(2), taux, reduction)
}

pub fn generer_bulletin_br(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if annee < 2025 {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "BRL", "BR",
            "Brésil : données disponibles à partir de 2025.", ctx);
    }

    let mut cotisations = Vec::new();

    // ── INSS salarié (progressif, plafonné) ──────────────────
    let inss = calcul_inss(brut, annee);
    let teto = tranches_inss(annee)[3].0;
    let taux_eff_inss = if brut > Decimal::ZERO { (inss / brut.min(teto)).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "BR_INSS".into(),
        libelle: ctx.libelle("BR_INSS", "INSS — Prévoyance sociale"),
        base: brut.min(teto),
        taux_sal: taux_eff_inss, montant_sal: inss,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("BR_INSS",
            "INSS (Instituto Nacional do Seguro Social) — contribution salariale progressive \
            par tranches (7,5 / 9 / 12 / 14 %), plafonnée au teto de {teto} R$. Cotisation {inss} R$ \
            (taux effectif {teff} %). Base légale : Lei 8.212/1991.")
            .replace("{teto}", &format!("{:.2}", teto))
            .replace("{inss}", &format!("{:.2}", inss))
            .replace("{teff}", &format!("{:.2}", taux_eff_inss * dec!(100))),
        loi_ref: Some(ctx.loi_ref(if annee >= 2026 { "Lei 8.212/1991 art. 20 — Portaria MPS/MF 13/2026" } else { "Lei 8.212/1991 art. 20 — tabela INSS 2025" })),
    });

    // ── IRRF (impôt sur le revenu retenu à la source) ────────
    let (irrf, base_irrf, taux_irrf, reduction) = calcul_irrf(brut, inss, ctx.date_paie);
    let taux_eff_irrf = if brut > Decimal::ZERO { (irrf / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "BR_IRRF".into(),
        libelle: ctx.libelle("BR_IRRF", "IRRF — Impôt sur le revenu retenu"),
        base: base_irrf,
        taux_sal: taux_eff_irrf, montant_sal: irrf,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("BR_IRRF",
            "IRRF (Imposto de Renda Retido na Fonte) — base = brut − max(INSS, desconto \
            simplificado {desc} R$) = {base} R$ ; barème mensuel progressif, tranche marginale \
            {taux} %. Impôt {irrf} R$ après la réduction de la Lei 15.270/2025 ({red} R$). \
            Base légale : Lei 7.713/1988.")
            .replace("{desc}", &format!("{:.2}", bareme_irrf(ctx.date_paie).1))
            .replace("{red}", &format!("{:.2}", reduction))
            .replace("{base}", &format!("{:.2}", base_irrf))
            .replace("{taux}", &format!("{:.1}", taux_irrf * dec!(100)))
            .replace("{irrf}", &format!("{:.2}", irrf)),
        loi_ref: Some(ctx.loi_ref(if annee >= 2026 { "Lei 7.713/1988 — Lei 15.270/2025" } else { "Lei 7.713/1988 — Lei 15.191/2025" })),
    });

    // ── Employeur : INSS patronal 20 % + FGTS 8 % ────────────
    let tp_inss = ctx.taux_pat("BR_INSS_PAT");
    cotisations.push(LigneCotisation {
        code: "BR_INSS_PAT".into(),
        libelle: ctx.libelle("BR_INSS_PAT", "INSS patronal (employeur)"),
        base: brut,
        taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: tp_inss, montant_pat: (brut * tp_inss).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("BR_INSS_PAT",
            "INSS patronal — 20 % employeur sur la masse salariale (contribution previdenciária \
            patronal). RAT (risques) et terceiros (Sistema S) — jusqu'à +8,8 % — non détaillés. \
            Taux {tp} % × {base} R$. Base légale : Lei 8.212/1991 art. 22.")
            .replace("{tp}", &format!("{:.2}", tp_inss * dec!(100)))
            .replace("{base}", &format!("{:.2}", brut)),
        loi_ref: Some(ctx.loi_ref("Lei 8.212/1991 art. 22 — INSS patronal")),
    });
    let tp_fgts = ctx.taux_pat("BR_FGTS");
    cotisations.push(LigneCotisation {
        code: "BR_FGTS".into(),
        libelle: ctx.libelle("BR_FGTS", "FGTS — Fonds de garantie (employeur)"),
        base: brut,
        taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: tp_fgts, montant_pat: (brut * tp_fgts).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("BR_FGTS",
            "FGTS (Fundo de Garantia do Tempo de Serviço) — 8 % employeur déposés sur le compte \
            lié du salarié (ne réduit pas le net, mobilisable au licenciement/achat immobilier). \
            Taux {tp} % × {base} R$. Base légale : Lei 8.036/1990.")
            .replace("{tp}", &format!("{:.2}", tp_fgts * dec!(100)))
            .replace("{base}", &format!("{:.2}", brut)),
        loi_ref: Some(ctx.loi_ref("Lei 8.036/1990 — FGTS")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "BRL".into(), absence: None, heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), pas: None, droits: None, alertes: Vec::new(), evolutions: Vec::new(), salarie,
    }
}
