use chrono::Datelike;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};
use super::nl_loonheffing::{nl_loonheffing, nl_parametres};

/// Cotisation patronale néerlandaise (werknemersverzekeringen / Zvw), assiette
/// plafonnée au maximumpremieloon mensuel. 100 % patronale → n'affecte pas le net.
fn cotisation_pat(
    code: &str, libelle: &str, brut: Decimal, plafond_mensuel: Decimal, ctx: &ContextPaie,
) -> LigneCotisation {
    let tp   = ctx.taux_pat(code);
    let base = brut.min(plafond_mensuel);
    LigneCotisation {
        code: code.into(),
        libelle: libelle.into(),
        base,
        taux_sal: Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat: tp,
        montant_pat: (base * tp).round_dp(2),
        categorie: "Cotisations patronales".into(),
        explication: format!(
            "{libelle} — premie patronale (werkgeversheffing).\n\n\
            Taux : {tp:.2} %\n\
            Assiette : {base:.2} € (min(brut, maximumpremieloon mensuel {plaf:.2} €))\n\
            Employeur : {mp:.2} €\n\n\
            Base légale : Wfsv / Zorgverzekeringswet.",
            tp = tp * dec!(100), base = base, plaf = plafond_mensuel,
            mp = (base * tp).round_dp(2),
        ),
        loi_ref: Some("Wfsv / Zvw".into()),
    }
}

pub fn generer_bulletin_nl(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    // Année non encore sourcée → lacune honnête : pas de calcul, message explicite.
    let Some(params) = nl_parametres(annee) else {
        let ligne = LigneCotisation {
            code: "NL_NON_COUVERT".into(),
            libelle: "Pays-Bas — données indisponibles pour cette année".into(),
            base: brut,
            taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
            taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
            categorie: "Information".into(),
            explication: format!(
                "Les données néerlandaises ne sont disponibles que pour 2026 (pilote).\n\
                L'année {annee} sera ajoutée après sourcing officiel (Belastingdienst).\n\
                Aucun chiffre n'est inventé en l'absence de source."
            ),
            loi_ref: None,
        };
        return Bulletin {
            cotisations: vec![ligne],
            brut,
            net_imposable: brut,
            net_a_payer: brut,
            cout_total_employeur: brut,
            devise: "EUR".into(),
            absence: None,
            salarie,
        };
    };

    let plafond_mensuel = (params.max_premieloon / dec!(12)).round_dp(2);

    let mut cotisations = Vec::new();
    // Côté salarié : loonheffing (impôt + premies volksverzekeringen − crédits).
    if let Some(l) = nl_loonheffing(brut, annee) {
        cotisations.push(l);
    }
    // Côté patronal : werknemersverzekeringen + Zvw (plafonnées).
    cotisations.push(cotisation_pat("NL_ZVW",       "Zvw — Assurance santé",        brut, plafond_mensuel, ctx));
    cotisations.push(cotisation_pat("NL_AWF",       "AWf — Chômage (WW)",            brut, plafond_mensuel, ctx));
    cotisations.push(cotisation_pat("NL_AOF",       "Aof — Invalidité (WIA)",        brut, plafond_mensuel, ctx));
    cotisations.push(cotisation_pat("NL_WHK",       "Whk — WGA + Ziektewet",         brut, plafond_mensuel, ctx));
    cotisations.push(cotisation_pat("NL_OPSLAG_KO", "Opslag kinderopvang",           brut, plafond_mensuel, ctx));

    let total_sal: Decimal = cotisations.iter().map(|c| c.montant_sal).sum();
    let total_pat: Decimal = cotisations.iter().map(|c| c.montant_pat).sum();
    let net_a_payer = (brut - total_sal).round_dp(2);

    Bulletin {
        cotisations,
        brut,
        net_imposable: net_a_payer,
        net_a_payer,
        cout_total_employeur: (brut + total_pat).round_dp(2),
        devise: "EUR".into(),
        absence: None,
        salarie,
    }
}
