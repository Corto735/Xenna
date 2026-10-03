// ── Suède — arbetsgivaravgifter (100 % patronales) + impôt communal + impôt d'État ──
//
// Particularité suédoise : pas de cotisations sociales salariales nettes. La seule
// retenue salariale légale est l'« allmän pensionsavgift » (7 %), mais elle est
// INTÉGRALEMENT compensée par une réduction d'impôt → effet net nul. Elle n'est donc
// pas affichée (sinon le net serait sous-estimé). L'employeur paie les
// arbetsgivaravgifter (31,42 % en 2025), lues en base.
//
// Impôt sur le revenu : impôt communal moyen (kommunalskatt, ≈ 32,41 % en 2025)
// + impôt d'État (statlig inkomstskatt) 20 % au-delà de 625 800 SEK/an (skiktgräns 2025).
//
// Grundavdrag, jobbskatteavdrag, réduction pour revenu d'activité et redevance
// public service selon Skatteverket (SKV 433). Kommunalskatt = moyenne nationale,
// hors begravningsavgift et Église.
// Sources : Skatteverket (SKV 433 éd. 35 et 36) ; arbetsgivaravgifter 2025.

use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn generer_bulletin_se(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    if !(2025..=2026).contains(&annee) {
        return super::pays_non_couvert::bulletin_non_couvert(
            salarie, brut, "SEK", "SE",
            "Suède : données disponibles pour 2025 et 2026.", ctx);
    }

    let g = brut * dec!(12); // revenu annuel
    // 2026 : kommunalskatt moyen 32,38 % ; skiktgräns 643 000 SEK (impôt d'État 20 %).
    let taux_communal = if annee >= 2026 { dec!(0.3238) } else { dec!(0.3241) };
    let skiktgrans = if annee >= 2026 { dec!(643000) } else { dec!(625800) };

    // Cotisation patronale unique : arbetsgivaravgifter 31,42 % (taux lu en base).
    let tp = ctx.taux_pat("SE_ARBETSGIVARAVGIFT");
    let mut cotisations = vec![LigneCotisation {
        code: "SE_ARBETSGIVARAVGIFT".into(),
        libelle: ctx.libelle("SE_ARBETSGIVARAVGIFT", "Arbetsgivaravgifter — cotisations patronales"),
        base: brut,
        taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: tp, montant_pat: (brut * tp).round_dp(2),
        categorie: "Sécurité sociale".into(),
        explication: ctx.expl("SE_ARBETSGIVARAVGIFT",
            "Arbetsgivaravgifter — {tp} % à la charge de l'employeur (retraite, maladie, \
            parentalité, accident, marché du travail, taxe générale sur salaires).\n\n\
            Côté salarié : l'allmän pensionsavgift (7 %) est intégralement compensée par \
            une réduction d'impôt (effet net nul) → non affichée.")
            .replace("{tp}", &format!("{:.2}", tp * dec!(100))),
        loi_ref: Some(ctx.loi_ref("Socialavgiftslagen (2000:980)")),
    }];

    // Impôt sur le revenu selon la « Teknisk beskrivning » de Skatteverket (SKV 433,
    // éditions 35 et 36), salarié de moins de 66 ans :
    //   grundavdrag (paliers en prisbasbelopp, arrondi à la centaine supérieure) ;
    //   impôt communal moyen sur le revenu imposable, impôt d'État 20 % au-delà de
    //   la skiktgräns ; jobbskatteavdrag (contre l'impôt communal seul) ;
    //   réduction pour revenu d'activité (0,75 % entre 40 000 et 240 000, 1 500
    //   max) ; redevance public service 1 % plafonnée.
    // La pension générale (7 %) est intégralement compensée : ni l'une ni l'autre.
    let (pbb, jsa_t2, jsa_max, ps_max) = if annee >= 2026 {
        (dec!(59200), dec!(0.251), dec!(3.027), dec!(1184))
    } else {
        (dec!(58800), dec!(0.199), dec!(2.776), dec!(1144))
    };
    let ga_brut = if g <= dec!(0.99) * pbb {
        dec!(0.423) * pbb
    } else if g <= dec!(2.72) * pbb {
        dec!(0.423) * pbb + (g - dec!(0.99) * pbb) * dec!(0.20)
    } else if g <= dec!(3.11) * pbb {
        dec!(0.77) * pbb
    } else if g <= dec!(7.88) * pbb {
        dec!(0.77) * pbb - (g - dec!(3.11) * pbb) * dec!(0.10)
    } else {
        dec!(0.293) * pbb
    };
    let ga = ((ga_brut / dec!(100)).ceil() * dec!(100)).min(g);
    let imposable = (g - ga).max(Decimal::ZERO);
    let communal = (imposable * taux_communal).floor();
    let etat = if imposable > skiktgrans { (imposable - skiktgrans) * dec!(0.20) } else { Decimal::ZERO };
    let ai = (g / dec!(100)).floor() * dec!(100);
    let jsa_base = if ai <= dec!(0.91) * pbb {
        ai
    } else if ai <= dec!(3.24) * pbb {
        dec!(0.91) * pbb + dec!(0.3874) * (ai - dec!(0.91) * pbb)
    } else if ai <= dec!(8.08) * pbb {
        dec!(1.813) * pbb + jsa_t2 * (ai - dec!(3.24) * pbb)
    } else {
        jsa_max * pbb
    };
    let jsa = ((jsa_base - ga).max(Decimal::ZERO) * taux_communal).floor().min(communal);
    let red_fi = ((imposable - dec!(40000)).max(Decimal::ZERO) * dec!(0.0075)).min(dec!(1500)).floor();
    let public_service = (imposable * dec!(0.01)).min(ps_max).floor();
    let impot_an = (communal + etat - jsa - red_fi).max(Decimal::ZERO) + public_service;
    let impot_mens = (impot_an / dec!(12)).round_dp(2);
    let taux_imp = if brut > Decimal::ZERO { (impot_mens / brut).round_dp(4) } else { Decimal::ZERO };
    cotisations.push(LigneCotisation {
        code: "SE_SKATT".into(),
        libelle: ctx.libelle("SE_SKATT", "Inkomstskatt — Impôt (communal + État)"),
        base: brut, taux_sal: taux_imp, montant_sal: impot_mens,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Impôt sur le revenu".into(),
        explication: ctx.expl("SE_SKATT",
            "Impôt sur le revenu {annee} (annualisé, salarié de moins de 66 ans).\n\n\
            Revenu annuel : {g} SEK − grundavdrag {ga} SEK = {ri} SEK imposables\n\
            Impôt communal moyen {tc} % → {co} SEK\n\
            Impôt d'État 20 % au-delà de {sk} SEK → {et} SEK\n\
            − jobbskatteavdrag {jsa} SEK − réduction pour revenu d'activité {rfi} SEK\n\
            + redevance public service {ps} SEK\n\
            = {ia} SEK/an, {im} SEK/mois.\n\n\
            Source : Skatteverket, Teknisk beskrivning SKV 433.")
            .replace("{annee}", &annee.to_string())
            .replace("{g}", &format!("{:.0}", g))
            .replace("{ga}", &format!("{:.0}", ga))
            .replace("{ri}", &format!("{:.0}", imposable))
            .replace("{tc}", &format!("{:.2}", taux_communal * dec!(100)))
            .replace("{sk}", &format!("{:.0}", skiktgrans))
            .replace("{co}", &format!("{:.0}", communal))
            .replace("{et}", &format!("{:.0}", etat))
            .replace("{jsa}", &format!("{:.0}", jsa))
            .replace("{rfi}", &format!("{:.0}", red_fi))
            .replace("{ps}", &format!("{:.0}", public_service))
            .replace("{ia}", &format!("{:.0}", impot_an))
            .replace("{im}", &format!("{:.2}", impot_mens)),
        loi_ref: Some(ctx.loi_ref("Inkomstskattelagen (1999:1229)")),
    });

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations, brut,
        net_imposable: net_a_payer, net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "SEK".into(), absence: None, heures_sup: None, conges: None, frais_professionnels: Vec::new(), avantages_nature: Vec::new(), salarie,
    }
}
