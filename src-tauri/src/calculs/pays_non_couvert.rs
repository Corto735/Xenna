// Helper partagé : bulletin « année non couverte » (lacune honnête).
// Renvoyé quand un pays n'a pas encore de données sourcées pour la date demandée.
// Aucun chiffre inventé : net = brut, une seule ligne d'information.

use rust_decimal::Decimal;
use crate::models::{Bulletin, LigneCotisation, Salarie};

pub fn bulletin_non_couvert(salarie: Salarie, brut: Decimal, devise: &str, message: &str) -> Bulletin {
    let ligne = LigneCotisation {
        code: "PAYS_NON_COUVERT".into(),
        libelle: "Données indisponibles pour cette année".into(),
        base: brut,
        taux_sal: Decimal::ZERO, montant_sal: Decimal::ZERO,
        taux_pat: Decimal::ZERO, montant_pat: Decimal::ZERO,
        categorie: "Information".into(),
        explication: format!(
            "{message}\n\nAucun chiffre n'est appliqué en l'absence de source officielle \
            pour cette date (lacune assumée, rien d'inventé)."
        ),
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
        salarie,
    }
}
