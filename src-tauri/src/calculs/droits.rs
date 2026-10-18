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
//   - Sécurité sociale : indemnités journalières maladie. Trois mois à ce
//     rythme ouvrent-ils le droit (150 h sur 3 mois, ou 1 015 SMIC horaires
//     sur 6 mois, art. R313-3 CSS) ? Et combien par jour, au salaire de ce mois
//     (50 % du salaire journalier de base, plafonné, art. R323-4) ?
//   - chômage : part du mois dans les 910 h d'affiliation, jours d'allocation
//     qu'il ajoute (jours calendaires × 0,75), allocation journalière si ce
//     salaire était celui des douze derniers mois (SJR = assiette × 12 ÷ 365) ;
//   - congés payés : 2,5 jours ouvrables par mois, 2 pour la part d'un arrêt
//     maladie non professionnel (L3141-5-1) ; le dixième du brut en donne le
//     plancher d'indemnité. Pas une cotisation, mais un droit que le mois ouvre.
//
// Paramètres : migration 0148.
//
// Hors champ : trimestres assimilés (maladie, chômage…), majorations, CET,
// régimes spéciaux et fonction publique.

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use crate::db::ContextPaie;
use rust_decimal_macros::dec;
use chrono::Datelike;
use crate::models::{Bulletin, ChomageDroits, CongesDroits, CpfResult, DroitsResult, SsDroits};

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
        ss: ss(b, ctx, assiette_trimestres),
        chomage: chomage(b, ctx, base("CHOMAGE")),
        conges: conges(b, ctx),
    })
}

/// Heures du mois : durée légale (151,67 h) × quotité, plus les heures
/// supplémentaires et complémentaires saisies.
fn heures_mois(b: &Bulletin) -> Decimal {
    let s = &b.salarie;
    let sup = s.heures_supp_25 + s.heures_supp_50 + s.heures_comp_10 + s.heures_comp_25
        + s.heures_struct_25 + s.heures_struct_50;
    let h = 151.67 * s.etp / 100.0 + sup;
    Decimal::try_from(h).unwrap_or(Decimal::ZERO).round_dp(2)
}

/// Indemnités journalières maladie. L'assiette est celle de la vieillesse
/// déplafonnée : le salaire réellement soumis à cotisations ce mois.
fn ss(b: &Bulletin, ctx: &ContextPaie, assiette: Decimal) -> Option<SsDroits> {
    if b.cotisations.iter().all(|c| c.code != "SS_MALADIE") || assiette <= Decimal::ZERO {
        return None;
    }
    let h3 = ctx.plafond("IJSS_HEURES_3MOIS")?;
    let n_smic = ctx.plafond("IJSS_SMIC_6MOIS")?;
    let taux = ctx.plafond("IJSS_TAUX")?;
    let coeff = ctx.plafond("IJSS_PLAFOND_SMIC")?;
    let smic_h = ctx.plafond("SMIC_HORAIRE")?;
    let heures = heures_mois(b);
    let salaire_requis = (n_smic * smic_h).round_dp(2);
    let ouvre = heures * dec!(3) >= h3 || assiette * dec!(6) >= salaire_requis;
    let plafond = coeff * ctx.smic_mensuel;
    let retenu = assiette.min(plafond);
    // Salaire journalier de base : trois mois de salaire ÷ 91,25.
    let sjb = retenu * dec!(3) / dec!(91.25);
    Some(SsDroits {
        heures_mois: heures,
        heures_requises_3mois: h3,
        salaire_requis_6mois: salaire_requis,
        ouvre,
        ij_jour: (sjb * taux).round_dp(2),
        plafonne: assiette > plafond,
        coeff_plafond: coeff,
    })
}

/// Assurance chômage. Pas de ligne chômage (ESAT, expatrié…) : pas de droit.
fn chomage(b: &Bulletin, ctx: &ContextPaie, assiette: Decimal) -> Option<ChomageDroits> {
    if assiette <= Decimal::ZERO {
        return None;
    }
    let h_req = ctx.plafond("CHOMAGE_HEURES_AFFILIATION")?;
    let j_req = ctx.plafond("CHOMAGE_JOURS_AFFILIATION")?;
    let coeff = ctx.plafond("CHOMAGE_COEFF_DUREE")?;
    let heures = heures_mois(b);
    if heures <= Decimal::ZERO {
        return None;
    }
    let mois_pour_ouvrir = (h_req / heures).ceil().to_i64().unwrap_or(0);
    let d = ctx.date_paie;
    let suivant = if d.month() == 12 {
        chrono::NaiveDate::from_ymd_opt(d.year() + 1, 1, 1)
    } else {
        chrono::NaiveDate::from_ymd_opt(d.year(), d.month() + 1, 1)
    }?;
    let jours_mois = (suivant - chrono::NaiveDate::from_ymd_opt(d.year(), d.month(), 1)?).num_days();
    let sjr = (assiette * dec!(12) / dec!(365)).round_dp(2);

    // Montant : barème relevé depuis le 1er juillet 2025 seulement.
    let quotite = Decimal::try_from(b.salarie.etp / 100.0).unwrap_or(Decimal::ONE).min(Decimal::ONE);
    let are_jour = (|| {
        let pf = ctx.plafond("ARE_PARTIE_FIXE")? * quotite;
        let mini = ctx.plafond("ARE_MINIMUM")? * quotite;
        let t1 = ctx.plafond("ARE_TAUX_PROPORTIONNEL")?;
        let t2 = ctx.plafond("ARE_TAUX_UNIQUE")?;
        let plaf = ctx.plafond("ARE_PLAFOND_SJR")?;
        let aj = (sjr * t1 + pf).max(sjr * t2).max(mini).min(sjr * plaf);
        Some(aj.round_dp(2))
    })();
    let degressive = match (are_jour, ctx.plafond("ARE_SEUIL_DEGRESSIVITE")) {
        (Some(aj), Some(seuil)) => aj > seuil,
        _ => false,
    };
    Some(ChomageDroits {
        heures_mois: heures,
        heures_requises: h_req,
        jours_requis: j_req.to_i64().unwrap_or(130),
        mois_pour_ouvrir,
        jours_mois,
        coeff_duree: coeff,
        jours_indemnises: (Decimal::from(jours_mois) * coeff).round_dp(2),
        sjr,
        are_jour,
        are_mois: are_jour.map(|aj| aj * dec!(30)),
        degressive,
    })
}

/// Congés payés acquis. La part du mois passée en arrêt se lit sur la retenue
/// d'absence rapportée au brut plein : elle ne dépend pas de la méthode de
/// décompte choisie.
fn conges(b: &Bulletin, ctx: &ContextPaie) -> Option<CongesDroits> {
    let plein = ctx.plafond("CP_JOURS_MOIS")?;
    let (jours, reduit) = match &b.absence {
        Some(a) if a.brut_mensuel > Decimal::ZERO && a.retenue > Decimal::ZERO => {
            let part = (a.retenue.abs() / a.brut_mensuel).min(Decimal::ONE);
            let pendant = match a.type_arret.as_str() {
                // AT/MP : assimilé à du travail effectif, sans limite d'un an
                // depuis la loi du 22 avril 2024 (L3141-5, 5°).
                "pro" => Some(plein),
                // Maladie non professionnelle : 2 jours par mois, depuis le
                // 24 avril 2024 ; rien avant.
                "maladie" => Some(ctx.plafond("CP_JOURS_MOIS_MALADIE").unwrap_or(Decimal::ZERO)),
                _ => Some(Decimal::ZERO),
            }?;
            let j = plein * (Decimal::ONE - part) + pendant * part;
            (j.round_dp(2), pendant < plein)
        }
        _ => (plein, false),
    };
    Some(CongesDroits {
        jours,
        reduit_absence: reduit,
        dixieme: (b.brut / dec!(10)).round_dp(2),
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
