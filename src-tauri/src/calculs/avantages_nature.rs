// Avantages en nature — évaluation pour le calcul des cotisations.
//
// Un avantage en nature est un bien ou un service fourni par l'employeur,
// gratuitement ou moyennant une participation inférieure à sa valeur, pour un
// usage privé. C'est une rémunération (CSS art. L242-1) : sa valeur s'ajoute
// au brut soumis aux cotisations, à la CSG/CRDS et à l'impôt sur le revenu,
// puis se retient sur le net, puisqu'elle n'est pas payée en espèces.
//
// Évaluation : arrêté du 25/02/2025 (en vigueur le 01/02/2025, remplace
// l'arrêté du 10/12/2002), forfaits revalorisés au 1er janvier.
//   Repas     : forfait par repas ; participation du salarié déduite. En
//               cantine ou restaurant d'entreprise, l'avantage est négligé si
//               le salarié paie au moins la moitié du forfait.
//   Logement  : forfait mensuel selon la rémunération (tranches du PSS) et le
//               nombre de pièces principales, avantages accessoires compris
//               (eau, gaz, électricité, chauffage, garage) ; ou valeur réelle
//               (valeur locative + accessoires). Participation déduite.
//   Véhicule  : forfait annuel en % du coût d'achat TTC ou du coût global
//               annuel de location, selon la date de mise à disposition
//               (avant / depuis le 01/02/2025), l'âge du véhicule et la prise
//               en charge du carburant ; abattement plafonné pour les véhicules
//               électriques ; électricité de recharge non comptée.
//   NTIC      : 10 % du coût d'achat TTC ou de l'abonnement annuel TTC.
//   Autre     : valeur réelle.
// Les forfaits annuels se prennent par douzième. Les montants datés vivent en
// base (plafond_reference, codes AN_*) depuis 2025 ; avant, la ligne est rendue
// à 0 avec une explication plutôt qu'inventée.

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::db::ContextPaie;
use crate::models::{AvantageNatureInput, LigneAvantage, Salarie};

fn dec_de(x: f64) -> Decimal {
    format!("{:.2}", x.max(0.0)).parse().unwrap_or(Decimal::ZERO)
}

fn eur(x: Decimal) -> String {
    format!("{} €", x.round_dp(2))
}

fn pct(x: Decimal) -> String {
    format!("{} %", (x * dec!(100)).normalize())
}

/// Première date d'application des forfaits véhicule de l'arrêté du 25/02/2025.
fn bascule_vehicule() -> NaiveDate {
    NaiveDate::from_ymd_opt(2025, 2, 1).unwrap()
}

/// Tranche du barème logement (1 à 8) selon la rémunération brute mensuelle
/// hors avantages, rapportée au plafond de la sécurité sociale.
pub fn tranche_logement(remuneration: Decimal, pss: Decimal) -> usize {
    const BORNES: [Decimal; 7] = [dec!(0.5), dec!(0.6), dec!(0.7), dec!(0.9), dec!(1.1), dec!(1.3), dec!(1.5)];
    BORNES.iter().position(|b| remuneration < *b * pss).map(|i| i + 1).unwrap_or(8)
}

/// Taux du forfait annuel véhicule : (taux, libellé de la règle).
pub fn taux_vehicule(location: bool, carburant: bool, plus_de_5_ans: bool, avant_fevrier_2025: bool) -> Decimal {
    match (location, avant_fevrier_2025, carburant, plus_de_5_ans) {
        (true,  true,  false, _)     => dec!(0.30),
        (true,  true,  true,  _)     => dec!(0.40),
        (true,  false, false, _)     => dec!(0.50),
        (true,  false, true,  _)     => dec!(0.67),
        (false, true,  false, false) => dec!(0.09),
        (false, true,  false, true)  => dec!(0.06),
        (false, true,  true,  false) => dec!(0.12),
        (false, true,  true,  true)  => dec!(0.09),
        (false, false, false, false) => dec!(0.15),
        (false, false, false, true)  => dec!(0.10),
        (false, false, true,  false) => dec!(0.20),
        (false, false, true,  true)  => dec!(0.15),
    }
}

struct Evaluation {
    montant: Decimal,
    calcul: String,
    /// Clé d'explication spécifique (absence de barème, avantage négligé…).
    cle: Option<&'static str>,
}

fn sans_bareme() -> Evaluation {
    Evaluation { montant: Decimal::ZERO, calcul: "—".into(), cle: Some("AN_SANS_BAREME") }
}

fn moins_participation(brut: Decimal, participation: Decimal, calcul: String) -> Evaluation {
    let montant = (brut - participation).max(Decimal::ZERO).round_dp(2);
    let calcul = if participation > Decimal::ZERO {
        format!("{calcul} − {} = {}", eur(participation), eur(montant))
    } else {
        format!("{calcul} = {}", eur(montant))
    };
    Evaluation { montant, calcul, cle: None }
}

fn repas(a: &AvantageNatureInput, ctx: &ContextPaie) -> Evaluation {
    let Some(forfait) = ctx.plafond("AN_REPAS") else { return sans_bareme() };
    let n = dec_de(a.nombre);
    let brut = (n * forfait).round_dp(2);
    let participation = dec_de(a.participation);
    // Cantine : participation d'au moins la moitié du forfait → avantage négligé.
    if a.cantine && brut > Decimal::ZERO && participation * dec!(2) >= brut {
        return Evaluation {
            montant: Decimal::ZERO,
            calcul: format!("{} ≥ 50 % × {} → 0,00 €", eur(participation), eur(brut)),
            cle: Some("AN_REPAS_NEGLIGE"),
        };
    }
    moins_participation(brut, participation, format!("{} × {}", n.normalize(), eur(forfait)))
}

fn logement(a: &AvantageNatureInput, remuneration: Decimal, ctx: &ContextPaie) -> Evaluation {
    let participation = dec_de(a.participation);
    if a.methode.as_deref() == Some("reel") {
        let valeur = dec_de(a.montant);
        return moins_participation(valeur, participation, eur(valeur));
    }
    let t = tranche_logement(remuneration, ctx.pmss);
    let pieces = a.nombre.round().max(1.0) as i64;
    let (code, facteur) = if pieces == 1 {
        (format!("AN_LOG_1P_T{t}"), Decimal::ONE)
    } else {
        (format!("AN_LOG_PP_T{t}"), Decimal::from(pieces))
    };
    let Some(unitaire) = ctx.plafond(&code) else { return sans_bareme() };
    let brut = (unitaire * facteur).round_dp(2);
    let calcul = if pieces == 1 { format!("T{t} : {}", eur(unitaire)) }
                 else { format!("T{t} : {pieces} × {}", eur(unitaire)) };
    moins_participation(brut, participation, calcul)
}

fn vehicule(a: &AvantageNatureInput, ctx: &ContextPaie) -> Evaluation {
    let location = a.mode.as_deref() == Some("location");
    let mad = a.mise_a_disposition.as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .unwrap_or(ctx.date_paie);
    let avant = mad < bascule_vehicule();
    // Électricité de recharge non comptée : pas de forfait « carburant ».
    let carburant = a.carburant && !a.electrique;
    let taux = taux_vehicule(location, carburant, a.plus_de_5_ans, avant);
    let cout = dec_de(a.cout);
    let annuel = (cout * taux).round_dp(2);
    let mut calcul = format!("{} × {}", eur(cout), pct(taux));

    let mut annuel_net = annuel;
    if a.electrique {
        let debut_50 = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let fin_70 = NaiveDate::from_ymd_opt(2027, 12, 31).unwrap();
        let regle = if !avant && mad <= fin_70 && a.eco_score { Some(("AN_VE_PLAF_70", dec!(0.70))) }
                    else if avant && mad >= debut_50 { Some(("AN_VE_PLAF_50", dec!(0.50))) }
                    else { None };
        if let Some((code, taux_abatt)) = regle {
            let Some(plafond) = ctx.plafond(code) else { return sans_bareme() };
            let abattement = (annuel * taux_abatt).min(plafond).round_dp(2);
            annuel_net = annuel - abattement;
            calcul = format!("({calcul} = {}) − min({} ; {})", eur(annuel), pct(taux_abatt), eur(plafond));
        }
    }
    let mensuel = (annuel_net / dec!(12)).round_dp(2);
    moins_participation(mensuel, dec_de(a.participation), format!("{calcul} ÷ 12"))
}

fn ntic(a: &AvantageNatureInput) -> Evaluation {
    let cout = dec_de(a.cout);
    let mensuel = (cout * dec!(0.10) / dec!(12)).round_dp(2);
    moins_participation(mensuel, dec_de(a.participation), format!("{} × 10 % ÷ 12", eur(cout)))
}

fn autre(a: &AvantageNatureInput) -> Evaluation {
    let valeur = dec_de(a.montant);
    moins_participation(valeur, dec_de(a.participation), eur(valeur))
}

/// (code, libellé français, explication française).
fn textes(nature: &str) -> Option<(&'static str, &'static str, &'static str)> {
    Some(match nature {
        "repas" => ("AN_REPAS", "Avantage en nature — repas",
            "Repas fournis par l'employeur, évalués au forfait par repas de l'arrêté du \
            25/02/2025 (une journée = deux repas), participation du salarié déduite. En cantine \
            ou restaurant d'entreprise, l'avantage est négligé si le salarié paie au moins la \
            moitié du forfait."),
        "logement" => ("AN_LOGEMENT", "Avantage en nature — logement",
            "Logement fourni par l'employeur. Forfait mensuel selon la rémunération brute hors \
            avantages (8 tranches exprimées en fraction du plafond de la sécurité sociale) et le \
            nombre de pièces principales, eau, gaz, électricité, chauffage et garage compris ; \
            au-delà d'une pièce, le montant s'entend par pièce. À défaut, valeur locative et \
            accessoires réels. Participation du salarié déduite."),
        "vehicule" => ("AN_VEHICULE", "Avantage en nature — véhicule",
            "Véhicule utilisé à titre privé. Forfait annuel, pris par douzième : véhicule acheté, \
            15 % du coût d'achat TTC (10 % au-delà de 5 ans), 20 % (15 %) carburant compris ; \
            véhicule loué, 50 % du coût global annuel, 67 % carburant compris. Véhicules mis à \
            disposition avant le 01/02/2025 : 9 % (6 %), 12 % (9 %) ; location 30 %, 40 %. \
            Véhicule 100 % électrique : abattement de 70 % plafonné s'il est mis à disposition \
            entre le 01/02/2025 et le 31/12/2027 et respecte l'éco-score, 50 % plafonné s'il \
            l'a été entre le 01/01/2020 et le 31/01/2025 ; l'électricité de recharge n'est pas \
            comptée. Participation du salarié déduite."),
        "ntic" => ("AN_NTIC", "Avantage en nature — outils numériques",
            "Outils issus des nouvelles technologies (ordinateur, téléphone, abonnement) utilisés \
            à titre privé : 10 % du coût d'achat TTC ou de l'abonnement annuel TTC, pris par \
            douzième. Participation du salarié déduite. Un usage strictement professionnel ne \
            constitue pas un avantage."),
        "autre" => ("AN_AUTRE", "Avantage en nature — autre",
            "Bien ou service fourni gratuitement ou à prix réduit, hors barème forfaitaire : \
            évalué à sa valeur réelle, participation du salarié déduite."),
        _ => return None,
    })
}

/// Avantages en nature valorisés pour le mois. `remuneration` = brut en
/// espèces du mois (hors avantages), qui fixe la tranche du barème logement.
pub fn valoriser(salarie: &Salarie, remuneration: Decimal, ctx: &ContextPaie) -> Vec<LigneAvantage> {
    salarie.avantages_nature.iter().filter_map(|a| {
        let (code, libelle, expl) = textes(&a.nature)?;
        let ev = match a.nature.as_str() {
            "repas"    => repas(a, ctx),
            "logement" => logement(a, remuneration, ctx),
            "vehicule" => vehicule(a, ctx),
            "ntic"     => ntic(a),
            _          => autre(a),
        };
        let mut explication = ctx.expl(code, expl);
        if let Some(cle) = ev.cle {
            explication.push(' ');
            explication.push_str(&ctx.expl(cle, match cle {
                "AN_REPAS_NEGLIGE" => "Ici, la participation atteint la moitié du forfait : \
                    l'avantage est négligé.",
                _ => "Aucun barème n'est intégré pour cette date : le simulateur couvre les \
                    forfaits en vigueur depuis 2025. Montant laissé à 0 plutôt qu'inventé.",
            }));
        }
        let libelle = match (&a.libelle, code) {
            (Some(l), "AN_AUTRE") if !l.trim().is_empty() =>
                format!("{} ({})", ctx.libelle(code, libelle), l.trim()),
            _ => ctx.libelle(code, libelle),
        };
        Some(LigneAvantage {
            code: code.into(),
            libelle,
            montant: ev.montant,
            calcul: ev.calcul,
            explication,
            loi_ref: Some(ctx.loi_ref("CSS art. L242-1 — Arrêté du 25/02/2025")),
        })
    }).collect()
}

/// Total des avantages en nature du mois.
pub fn total(lignes: &[LigneAvantage]) -> Decimal {
    lignes.iter().map(|l| l.montant).sum::<Decimal>().round_dp(2)
}
