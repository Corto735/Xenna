// Droits à la retraite ouverts par le mois (France, secteur privé).
//
// Le salarié voit ses cotisations partir ; il voit rarement ce qu'elles lui
// rapportent. Trois choses, toutes lues sur le bulletin (bases des lignes) et
// dans la base (paramètres datés et sourcés, migration 0141) :
//
//   - régime de base : le salaire « porté au compte », c'est-à-dire l'assiette
//     de la vieillesse plafonnée, qui entrera dans le salaire annuel moyen ;
//   - trimestres : un trimestre par tranche de 150 h au SMIC horaire du
//     1er janvier (art. R351-9 CSS), 4 au plus par an. Un mois isolé ne
//     « valide » rien à lui seul : la règle est annuelle. On dit donc combien
//     de trimestres donnerait ce salaire sur douze mois, et ce que pèse le mois ;
//   - Agirc-Arrco : points = assiette × taux de calcul des points ÷ prix
//     d'achat du point. Le taux APPELÉ (127 % du taux de calcul) n'entre pas
//     ici : sa part au-delà de 100 % n'ouvre aucun droit.
//
//   - CPF : alimentation annuelle en euros (500 €, 800 € en ESAT ou pour un
//     travailleur handicapé en entreprise adaptée), au prorata sous le
//     mi-temps ; un mois en vaut le douzième (migration 0147).
//
// Hors champ : trimestres assimilés (maladie, chômage…), majorations, CET,
// régimes spéciaux et fonction publique.

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use crate::db::ContextPaie;
use rust_decimal_macros::dec;
use crate::models::{Bulletin, CpfResult, DroitsResult};

pub fn calculer(b: &Bulletin, ctx: &ContextPaie) -> Option<DroitsResult> {
    let base = |code: &str| b.cotisations.iter()
        .find(|c| c.code == code)
        .map(|c| c.base)
        .unwrap_or(Decimal::ZERO);

    let prix_achat = ctx.plafond("AA_PRIX_ACHAT_POINT")?;
    let valeur_point = ctx.plafond("AA_VALEUR_POINT")?;
    let taux_t1 = ctx.plafond("AA_TAUX_POINTS_T1")?;
    let taux_t2 = ctx.plafond("AA_TAUX_POINTS_T2").unwrap_or(Decimal::ZERO);
    let heures = ctx.plafond("RETRAITE_HEURES_TRIMESTRE")?;
    let smic_janvier = ctx.smic_horaire_janvier?;
    if prix_achat <= Decimal::ZERO || smic_janvier <= Decimal::ZERO {
        return None;
    }

    let assiette_t1 = base("AGIRC_ARRCO_T1");
    let assiette_t2 = base("AGIRC_ARRCO_T2");
    let points = ((assiette_t1 * taux_t1 + assiette_t2 * taux_t2) / prix_achat).round_dp(2);

    let assiette_trimestres = base("SS_VIEILLESSE_DEPLAF");
    let seuil = (heures * smic_janvier).round_dp(2);
    let part = (assiette_trimestres / seuil).round_dp(2);
    let sur_un_an = (assiette_trimestres * Decimal::from(12) / seuil).floor();
    let trimestres_an = sur_un_an.to_u8().unwrap_or(4).min(4);

    Some(DroitsResult {
        salaire_porte_au_compte: base("SS_VIEILLESSE_PLAF"),
        assiette_trimestres,
        seuil_trimestre: seuil,
        smic_horaire_janvier: smic_janvier,
        heures_trimestre: heures,
        trimestres_an,
        part_trimestre_mois: part,
        assiette_t1,
        assiette_t2,
        taux_points_t1: taux_t1,
        taux_points_t2: taux_t2,
        prix_achat_point: prix_achat,
        valeur_point,
        points,
        rente_annuelle: (points * valeur_point).round_dp(2),
        cpf: cpf(b, ctx),
    })
}

/// Centime supérieur (art. R6323-1 : « arrondis au centime d'euro supérieur »).
fn centime_sup(d: Decimal) -> Decimal {
    (d * dec!(100)).ceil() / dec!(100)
}

/// Alimentation du CPF qu'ouvre ce temps de travail. La règle est annuelle :
/// au moins la moitié de la durée légale sur l'année donne le montant plein,
/// en deçà le prorata. Le mois affiché en est le douzième.
fn cpf(b: &Bulletin, ctx: &ContextPaie) -> Option<CpfResult> {
    let base = ctx.plafond("CPF_ALIMENTATION")?;
    let plafond = ctx.plafond("CPF_PLAFOND")?;
    let majore = ctx.plafond("CPF_ALIMENTATION_MAJOREE")?;
    let plafond_majore = ctx.plafond("CPF_PLAFOND_MAJORE")?;
    let s = &b.salarie;
    let etp = Decimal::try_from(s.etp / 100.0).unwrap_or(Decimal::ONE).round_dp(6);
    let (regime, annuel, plaf, prorata) = if s.esat {
        // Par année d'admission, à temps plein ou partiel (R6323-27).
        ("esat", majore, plafond_majore, false)
    } else if etp < dec!(0.5) {
        ("general", centime_sup(base * etp), plafond, true)
    } else if s.entreprise_adaptee {
        ("handicap", majore, plafond_majore, false)
    } else {
        ("general", base, plafond, false)
    };
    Some(CpfResult {
        regime: regime.into(),
        mois: centime_sup(annuel / dec!(12)),
        annuel,
        plafond: plaf,
        majore,
        plafond_majore,
        prorata,
    })
}
