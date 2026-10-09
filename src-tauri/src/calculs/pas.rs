// Prélèvement à la source de l'impôt sur le revenu (France, FPT).
//
// Art. 204 H du CGI : le taux est soit personnalisé (calculé par
// l'administration à partir de la dernière déclaration et transmis à
// l'employeur), soit, à défaut, le taux de la grille par défaut qui
// correspond à la base mensuelle de prélèvement. Dans les deux cas c'est un
// taux UNIQUE, appliqué à la TOTALITÉ de la base : la grille dit quel taux
// retenir, elle ne découpe pas la base en tranches. (Le simulateur a longtemps
// fait l'inverse — calcul progressif tranche par tranche, côté front — ce qui
// sous-estimait le PAS et enseignait une règle fausse.)
//
// Taux par défaut seulement :
//   - la grille dépend du domicile du salarié (métropole ou hors de France ;
//     Guadeloupe, La Réunion, Martinique ; Guyane, Mayotte) — III-1 a à c ;
//   - contrat court (CDD ou mission ≤ 2 mois, deux premiers mois d'embauche) :
//     un abattement d'un demi-SMIC mensuel net imposable est retranché de
//     l'assiette ; le taux se lit sur l'assiette réduite et s'y applique
//     (III-1-d, BOI-IR-PAS-20-20-30-10 § 270).
// Un taux personnalisé l'emporte toujours : ni grille, ni abattement.
//
// Base retenue : le net imposable du bulletin. Hors champ : revenus de
// remplacement versés par un tiers, périodicité autre que mensuelle.

use rust_decimal::Decimal;
use crate::db::ContextPaie;
use crate::models::{PasResult, Salarie, TranchePas};

pub fn calculer(salarie: &Salarie, net_imposable: Decimal, ctx: &ContextPaie) -> Option<PasResult> {
    // Avant 2019, pas de grille en base : le PAS n'existait pas, et un taux
    // personnalisé n'aurait pas de sens non plus.
    if ctx.pas_grille.is_empty() {
        return None;
    }
    let base = net_imposable.max(Decimal::ZERO);
    let zone = match salarie.pas_zone.as_deref() {
        Some("grm") => "GRM",
        Some("gm") => "GM",
        _ => "METROPOLE",
    };
    // Grille de la zone ; repli sur la métropole si la zone manquait en base.
    let (grille, source) = ctx.pas_grilles.get(zone)
        .cloned()
        .unwrap_or_else(|| (ctx.pas_grille.clone(), ctx.pas_source.clone()));
    let zone = if ctx.pas_grilles.contains_key(zone) { zone } else { "METROPOLE" };

    let (taux, origine, abattement) = match salarie.taux_pas {
        Some(t) if t >= Decimal::ZERO => (t, "personnalise", Decimal::ZERO),
        _ => {
            let abattement = if salarie.contrat_court {
                ctx.plafond("PAS_ABATTEMENT_CONTRAT_COURT").unwrap_or(Decimal::ZERO).min(base)
            } else {
                Decimal::ZERO
            };
            (taux_grille(&grille, base - abattement), "defaut", abattement)
        }
    };
    let assiette = base - abattement;
    Some(PasResult {
        base,
        abattement,
        assiette,
        zone: zone.into(),
        taux,
        montant: (assiette * taux).round_dp(2),
        origine: origine.into(),
        grille,
        source,
    })
}

/// Taux de la plage telle que borne_min ≤ base < borne_max.
fn taux_grille(grille: &[TranchePas], base: Decimal) -> Decimal {
    grille.iter()
        .find(|t| base >= t.borne_min && t.borne_max.map_or(true, |m| base < m))
        .map(|t| t.taux)
        .unwrap_or(Decimal::ZERO)
}
