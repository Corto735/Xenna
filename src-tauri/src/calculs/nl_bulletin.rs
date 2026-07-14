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
    let lib = ctx.libelle(code, libelle);
    let explication = ctx.expl("NL_PAT_GENERIC",
        "{libelle} — premie patronale (werkgeversheffing).\n\n\
        Taux : {tp} %\n\
        Assiette : {base} € (min(brut, maximumpremieloon mensuel {plaf} €))\n\
        Employeur : {mp} €\n\n\
        Base légale : Wfsv / Zorgverzekeringswet.")
        .replace("{libelle}", &lib)
        .replace("{tp}", &format!("{:.2}", tp * dec!(100)))
        .replace("{base}", &format!("{:.2}", base))
        .replace("{plaf}", &format!("{:.2}", plafond_mensuel))
        .replace("{mp}", &format!("{:.2}", (base * tp).round_dp(2)));
    LigneCotisation {
        code: code.into(),
        libelle: lib,
        base,
        taux_sal: Decimal::ZERO,
        montant_sal: Decimal::ZERO,
        taux_pat: tp,
        montant_pat: (base * tp).round_dp(2),
        categorie: "Cotisations patronales".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("Wfsv / Zvw")),
    }
}

pub fn generer_bulletin_nl(salarie: Salarie, ctx: &ContextPaie) -> Bulletin {
    let brut  = salarie.salaire_brut;
    let annee = ctx.date_paie.year();

    // Année non encore sourcée → lacune honnête : pas de calcul, message explicite.
    let Some(params) = nl_parametres(annee) else {
        let ligne = LigneCotisation {
            code: "NL_NON_COUVERT".into(),
            libelle: ctx.libelle("NL_NON_COUVERT", "Pays-Bas — données indisponibles pour cette année"),
            base: brut,
            taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
            taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
            categorie: "Information".into(),
            explication: ctx.expl("NL_NON_COUVERT",
                "Les données néerlandaises ne sont disponibles que pour 2026 (pilote).\n\
                L'année {annee} sera ajoutée après sourcing officiel (Belastingdienst).\n\
                Aucun chiffre n'est inventé en l'absence de source.")
                .replace("{annee}", &annee.to_string()),
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
            heures_sup: None, conges: None,
            salarie,
        };
    };

    let plafond_mensuel = (params.max_premieloon / dec!(12)).round_dp(2);

    let mut cotisations = Vec::new();
    // Côté salarié : loonheffing (impôt + premies volksverzekeringen − crédits).
    if let Some(l) = nl_loonheffing(brut, annee, ctx) {
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
        heures_sup: None, conges: None,
        salarie,
    }
}
