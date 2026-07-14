// Helper partagé : bulletin « année non couverte » (lacune honnête).
// Renvoyé quand un pays n'a pas encore de données sourcées pour la date demandée.
// Aucun chiffre inventé : net = brut, une seule ligne d'information.
//
// i18n : le libellé et la phrase de conclusion vivent sous la clé
// PAYS_NON_COUVERT (i18n::cotisations, repli France du dispatcher) ; la 1re
// ligne « Pays : disponibilité » est traduite par i18n::non_couvert::message,
// à partir du `code_pays` (le `message` français reste le repli natif).

use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn bulletin_non_couvert(
    salarie: Salarie,
    brut: Decimal,
    devise: &str,
    code_pays: &str,
    message: &str,
    ctx: &ContextPaie,
) -> Bulletin {
    let premiere_ligne = if ctx.lang == "fr" {
        message.to_string()
    } else {
        crate::i18n::non_couvert::message(code_pays, &ctx.lang)
            .unwrap_or_else(|| message.to_string())
    };
    let conclusion = ctx.expl(
        "PAYS_NON_COUVERT",
        "Aucun chiffre n'est appliqué en l'absence de source officielle \
        pour cette date (lacune assumée, rien d'inventé).",
    );
    let ligne = LigneCotisation {
        code: "PAYS_NON_COUVERT".into(),
        libelle: ctx.libelle("PAYS_NON_COUVERT", "Données indisponibles pour cette année"),
        base: brut,
        taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Information".into(),
        explication: format!("{premiere_ligne}\n\n{conclusion}"),
        loi_ref: None,
    };
    Bulletin {
        cotisations: vec![ligne],
        brut,
        net_imposable: brut,
        net_a_payer: brut,
        cout_total_employeur: brut,
        devise: devise.into(),
        absence: None,
        heures_sup: None, conges: None,
        salarie,
    }
}
