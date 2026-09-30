// Frais professionnels conventionnels versés en net — indemnités de repas des
// ouvriers de l'IDCC 0016 (transports routiers).
//
// Un remboursement de frais professionnels n'est pas un salaire : il est exclu
// de l'assiette des cotisations et de la CSG dans la limite des forfaits de
// l'arrêté du 20/12/2002 (CSS art. L242-1), et exonéré d'impôt sur le revenu
// (CGI art. 81, 1°). Il se paie donc en bas de bulletin, après le net : il ne
// touche ni le brut, ni les cotisations, ni le net imposable, seulement le net
// à payer. Les montants conventionnels intégrés restent sous la limite du
// forfait URSSAF de leur situation (repas sur le lieu de travail, hors des
// locaux) : aucun excédent à réintégrer dans l'assiette.
//
// Montants : protocole du 30/04/1974 relatif aux frais de déplacement des
// ouvriers (annexe I), revalorisé par avenants — en base (plafond_reference,
// codes IDCC16_*), depuis le 01/12/2022. Avant, aucun barème : la ligne est
// rendue à 0 avec une explication plutôt qu'inventée.

use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{LigneFrais, Salarie};

pub const IDCC_TRANSPORT_ROUTIER: &str = "0016";

/// (code, libellé, condition d'attribution, article du protocole).
const INDEMNITES: [(&str, &str, &str, &str); 4] = [
    ("IDCC16_REPAS_UNIQUE", "Indemnité de repas unique",
     "déplacement dans la zone de camionnage autour de Paris", "art. 4"),
    ("IDCC16_REPAS_UNIQUE_NUIT", "Indemnité de repas unique de nuit",
     "service comportant au moins 4 heures de travail effectif entre 22 h et 7 h", "art. 12"),
    ("IDCC16_INDEMNITE_SPECIALE", "Indemnité spéciale (repas)",
     "amplitude couvrant entièrement 11 h-14 h 30 ou 18 h 30-22 h sans coupure d'au moins 1 heure", "art. 7"),
    ("IDCC16_CASSE_CROUTE", "Indemnité de casse-croûte",
     "prise de service avant 5 heures en raison d'un déplacement", "art. 5"),
];

fn nombre(n: f64) -> Decimal {
    format!("{:.2}", n.clamp(0.0, 1000.0)).parse().unwrap_or(Decimal::ZERO)
}

/// Lignes de frais du mois, ou rien si la convention appliquée n'est pas
/// l'IDCC 0016 ou si aucune indemnité n'est saisie.
pub fn lignes_frais(salarie: &Salarie, ctx: &ContextPaie) -> Vec<LigneFrais> {
    if salarie.convention_idcc.as_deref() != Some(IDCC_TRANSPORT_ROUTIER) {
        return Vec::new();
    }
    let Some(saisie) = &salarie.indemnites_repas else { return Vec::new() };
    let nombres = [saisie.repas_unique, saisie.repas_unique_nuit, saisie.speciale, saisie.casse_croute];

    INDEMNITES.iter().zip(nombres).filter_map(|(&(code, libelle, condition, article), n)| {
        let n = nombre(n);
        if n <= Decimal::ZERO {
            return None;
        }
        let unitaire = ctx.plafond(code);
        let montant = unitaire.map(|u| (u * n).round_dp(2)).unwrap_or(Decimal::ZERO);
        let explication = match unitaire {
            Some(_) => ctx.expl("IDCC16_FRAIS",
                "Indemnité forfaitaire de la convention des transports routiers (IDCC 0016), \
                due pour : {condition} ({article} du protocole du 30/04/1974). {n} × {u} € = \
                {montant} €. C'est un remboursement de frais professionnels, pas un salaire : \
                exclu des cotisations et de la CSG dans la limite des forfaits de l'arrêté du \
                20/12/2002, exonéré d'impôt sur le revenu (CGI art. 81, 1°). Il est versé en net, \
                après le net à payer du salaire, sans toucher au net imposable."),
            None => ctx.expl("IDCC16_FRAIS_SANS_BAREME",
                "Indemnité forfaitaire de la convention des transports routiers (IDCC 0016), \
                due pour : {condition} ({article} du protocole du 30/04/1974). Aucun barème \
                n'est intégré pour cette date : le simulateur couvre les montants en vigueur \
                depuis le 01/12/2022. Montant laissé à 0 plutôt qu'inventé."),
        }
        .replace("{condition}", &ctx.expl(&format!("{code}_COND"), condition))
        .replace("{article}", article)
        .replace("{n}", &n.normalize().to_string())
        .replace("{u}", &unitaire.unwrap_or_default().to_string())
        .replace("{montant}", &montant.to_string());

        Some(LigneFrais {
            code:             code.into(),
            libelle:          ctx.libelle(code, libelle),
            nombre:           n,
            montant_unitaire: unitaire,
            montant,
            explication,
            loi_ref: Some(ctx.loi_ref("Convention du 21/12/1950 (IDCC 0016), protocole du 30/04/1974, annexe I — Arrêté du 20/12/2002")),
        })
    }).collect()
}

/// Total des frais versés en net.
pub fn total(lignes: &[LigneFrais]) -> Decimal {
    lignes.iter().map(|l| l.montant).sum::<Decimal>().round_dp(2)
}
