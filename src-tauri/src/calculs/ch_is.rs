use rust_decimal::Decimal;
use std::str::FromStr;
use crate::db::ContextPaie;
use crate::models::LigneCotisation;

// ── Seuils de salaire mensuel (CHF) ──────────────────────────────────────────
// 27 éléments correspondant aux 27 colonnes des tables cantonales.
const THRESHOLDS: &[u32] = &[
    0, 1900, 2000, 2100, 2200, 2400, 2600, 2800, 3000,
    3200, 3400, 3600, 3800, 4000, 4500, 5000, 5500, 6000,
    6500, 7000, 8000, 9000, 10000, 12000, 15000, 20000, 30000,
];

// ── Taux A0 par canton (célibataire, sans enfant) — valeurs 2025 ─────────────
// Ordre alphabétique des codes à 2 lettres.
// Chaque tableau a exactement 27 valeurs (une par seuil dans THRESHOLDS).
const CANTON_A0: &[(&str, &str, &[f32; 27])] = &[
    ("AG", "Argovie",              &[0.0, 0.0, 3.0, 4.5, 6.0, 7.5, 9.0, 10.5, 12.0, 13.0, 14.5, 15.5, 16.5, 17.5, 19.0, 20.5, 22.0, 23.5, 24.5, 25.5, 27.0, 28.5, 30.0, 32.0, 34.0, 36.5, 39.0]),
    ("AI", "Appenzell Rh.-Int.",   &[0.0, 0.0, 1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 5.5, 6.5, 7.0, 7.5, 8.0, 8.5, 9.5, 10.5, 11.5, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.5, 20.0, 22.0, 24.0]),
    ("AR", "Appenzell Rh.-Ext.",   &[0.0, 0.0, 2.0, 3.0, 4.5, 6.0, 7.5, 8.5, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.5, 18.0, 19.5, 20.5, 21.5, 22.5, 24.0, 25.5, 26.5, 28.5, 30.5, 33.0, 35.5]),
    ("BE", "Berne",                &[0.0, 0.0, 4.0, 5.5, 7.0, 9.0, 11.0, 12.5, 14.5, 15.5, 17.0, 18.0, 19.0, 20.5, 22.0, 23.5, 25.0, 26.5, 27.5, 28.5, 30.5, 32.0, 33.0, 35.0, 37.0, 39.5, 42.0]),
    ("BL", "Bâle-Campagne",        &[0.0, 0.0, 3.0, 4.5, 6.0, 7.5, 9.0, 10.5, 12.0, 13.0, 14.5, 15.5, 16.5, 17.5, 19.0, 20.5, 22.0, 23.5, 24.5, 25.5, 27.0, 28.5, 30.0, 32.0, 34.0, 36.5, 39.0]),
    ("BS", "Bâle-Ville",           &[0.0, 0.0, 2.5, 4.0, 5.5, 7.0, 8.5, 10.0, 11.5, 12.5, 13.5, 14.5, 15.5, 16.5, 18.0, 19.5, 21.0, 22.5, 23.5, 24.5, 26.0, 27.5, 29.0, 30.5, 32.5, 34.5, 37.0]),
    ("FR", "Fribourg",             &[0.0, 0.0, 3.0, 4.5, 5.5, 7.5, 9.0, 10.5, 12.0, 13.0, 14.5, 15.5, 16.5, 17.5, 19.0, 20.5, 22.0, 23.5, 24.5, 25.5, 27.0, 28.5, 30.0, 32.0, 34.0, 36.5, 39.0]),
    ("GE", "Genève",               &[0.0, 0.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.5, 10.0, 11.0, 11.5, 12.0, 13.0, 14.0, 15.0, 16.0, 16.5, 17.0, 18.0, 19.0, 19.5, 20.5, 21.5, 23.0, 24.5]),
    ("GL", "Glaris",               &[0.0, 0.0, 2.0, 3.5, 4.5, 6.0, 7.5, 8.5, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.5, 18.0, 19.5, 20.5, 21.5, 22.5, 24.0, 25.5, 26.5, 28.5, 30.5, 33.0, 35.5]),
    ("GR", "Grisons",              &[0.0, 0.0, 2.5, 4.0, 5.5, 7.0, 8.5, 10.0, 11.5, 12.5, 14.0, 15.0, 16.0, 17.0, 18.5, 20.0, 21.5, 22.5, 23.5, 24.5, 26.0, 27.5, 29.0, 30.5, 32.5, 35.0, 37.5]),
    ("JU", "Jura",                 &[0.0, 0.0, 4.5, 6.0, 7.5, 10.0, 12.0, 13.5, 15.5, 17.0, 18.5, 19.5, 21.0, 22.5, 24.0, 25.5, 27.0, 28.5, 29.5, 31.0, 33.0, 34.5, 36.0, 38.0, 40.0, 42.5, 45.0]),
    ("LU", "Lucerne",              &[0.0, 0.0, 2.5, 3.5, 5.0, 6.5, 8.0, 9.5, 11.0, 12.0, 13.5, 14.5, 15.0, 16.0, 17.5, 19.0, 20.5, 22.0, 23.0, 24.0, 25.5, 27.0, 28.0, 30.0, 32.0, 34.5, 37.0]),
    ("NE", "Neuchâtel",            &[0.0, 0.0, 3.5, 5.0, 6.5, 8.5, 10.5, 12.0, 13.5, 15.0, 16.5, 17.5, 18.5, 20.0, 21.5, 23.0, 24.5, 26.0, 27.0, 28.0, 30.0, 31.5, 33.0, 35.0, 37.0, 39.5, 42.0]),
    ("NW", "Nidwald",              &[0.0, 0.0, 1.5, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 9.5, 10.5, 11.0, 12.5, 13.5, 14.5, 15.5, 16.0, 17.0, 18.0, 19.0, 20.0, 21.5, 23.0, 25.0, 27.5]),
    ("OW", "Obwald",               &[0.0, 0.0, 1.5, 2.5, 3.5, 5.0, 6.0, 7.0, 8.5, 9.5, 10.5, 11.5, 12.0, 13.0, 14.5, 15.5, 16.5, 17.5, 18.5, 19.5, 21.0, 22.0, 23.0, 24.5, 26.5, 28.5, 31.0]),
    ("SG", "Saint-Gall",           &[0.0, 0.0, 2.0, 3.5, 5.0, 6.5, 8.0, 9.5, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.5, 18.5, 20.0, 21.5, 22.5, 23.5, 25.0, 26.5, 27.5, 29.5, 31.5, 34.0, 36.5]),
    ("SH", "Schaffhouse",          &[0.0, 0.0, 2.5, 4.0, 5.5, 7.0, 8.5, 10.0, 11.5, 12.5, 14.0, 15.0, 16.0, 17.0, 18.5, 20.0, 21.5, 22.5, 23.5, 24.5, 26.0, 27.5, 29.0, 31.0, 33.0, 35.5, 38.0]),
    ("SO", "Soleure",              &[0.0, 0.0, 2.5, 4.0, 5.5, 7.0, 8.5, 10.0, 11.5, 12.5, 14.0, 15.0, 16.0, 17.0, 18.5, 20.0, 21.5, 22.5, 23.5, 24.5, 26.0, 27.5, 29.0, 30.5, 32.5, 35.0, 37.5]),
    ("SZ", "Schwyz",               &[0.0, 0.0, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5, 10.5, 11.0, 11.5, 13.0, 14.0, 15.0, 16.0, 16.5, 17.5, 18.5, 19.5, 20.5, 22.0, 23.5, 25.5, 27.5]),
    ("TG", "Thurgovie",            &[0.0, 0.0, 2.0, 3.5, 4.5, 6.0, 7.5, 8.5, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.5, 17.5, 19.0, 20.5, 21.5, 22.5, 24.0, 25.5, 27.0, 28.5, 30.5, 33.0, 35.5]),
    ("TI", "Tessin",               &[0.0, 0.0, 2.0, 3.0, 4.5, 6.0, 7.5, 8.5, 10.0, 11.0, 12.0, 13.0, 14.0, 14.5, 16.0, 17.5, 18.5, 19.5, 20.5, 21.5, 23.0, 24.0, 25.0, 26.5, 28.5, 30.5, 33.0]),
    ("UR", "Uri",                  &[0.0, 0.0, 2.0, 3.0, 4.0, 5.5, 7.0, 8.0, 9.5, 10.5, 11.5, 12.5, 13.5, 14.5, 16.0, 17.0, 18.5, 19.5, 20.5, 21.5, 23.0, 24.5, 25.5, 27.5, 29.5, 31.5, 34.0]),
    ("VD", "Vaud",                 &[0.0, 0.0, 2.5, 3.5, 4.5, 6.0, 7.5, 8.5, 10.0, 11.0, 12.0, 13.0, 13.5, 14.5, 15.5, 17.0, 18.0, 19.5, 20.0, 21.0, 22.5, 23.5, 24.5, 26.0, 28.0, 30.0, 32.5]),
    ("VS", "Valais",               &[0.0, 0.0, 2.0, 3.0, 4.0, 5.5, 7.0, 8.0, 9.0, 10.0, 11.0, 11.5, 12.5, 13.0, 14.5, 15.5, 16.5, 17.5, 18.5, 19.5, 21.0, 22.0, 23.0, 24.5, 26.0, 28.0, 30.5]),
    ("ZG", "Zoug",                 &[0.0, 0.0, 1.0, 1.5, 2.0, 3.0, 3.5, 4.5, 5.0, 5.5, 6.0, 6.5, 7.0, 7.5, 8.5, 9.5, 10.5, 11.0, 12.0, 12.5, 13.5, 14.5, 15.5, 16.5, 18.0, 19.5, 21.5]),
    ("ZH", "Zurich",               &[0.0, 0.0, 3.0, 4.5, 5.5, 7.0, 8.5, 10.0, 11.5, 12.5, 14.0, 15.0, 16.0, 17.0, 18.5, 20.0, 21.5, 22.5, 23.5, 24.5, 26.0, 27.5, 28.5, 30.5, 32.5, 35.0, 37.5]),
];

// ── Multiplicateurs de tarif ──────────────────────────────────────────────────
// Appliqués au taux A0 pour obtenir le taux du tarif demandé.
const TARIF_MULT: &[(&str, f32)] = &[
    ("A0", 1.000),
    ("A1", 0.875),
    ("A2", 0.762),
    ("B0", 0.800),
    ("B1", 0.710),
    ("B2", 0.620),
    ("C0", 0.922),
    ("C1", 0.832),
    ("C2", 0.742),
    ("H0", 0.862),
    ("H1", 0.778),
    ("H2", 0.690),
];

// ── Libellés longs des tarifs ─────────────────────────────────────────────────
const TARIF_LABELS: &[(&str, &str)] = &[
    ("A0", "Célibataire, sans enfant"),
    ("A1", "Célibataire, 1 enfant à charge"),
    ("A2", "Célibataire, 2 enfants ou plus à charge"),
    ("B0", "Marié·e, 1 seul revenu, sans enfant"),
    ("B1", "Marié·e, 1 seul revenu, 1 enfant à charge"),
    ("B2", "Marié·e, 1 seul revenu, 2 enfants ou plus à charge"),
    ("C0", "Marié·e, 2 revenus, sans enfant"),
    ("C1", "Marié·e, 2 revenus, 1 enfant à charge"),
    ("C2", "Marié·e, 2 revenus, 2 enfants ou plus à charge"),
    ("H0", "Monoparental·e, sans enfant supplémentaire"),
    ("H1", "Monoparental·e, 1 enfant supplémentaire"),
    ("H2", "Monoparental·e, 2 enfants supplémentaires ou plus"),
];

/// Calcule l'impôt à la source (IS) pour un salarié soumis à la retenue à la source
/// en Suisse selon le barème cantonal ORIS 2025.
///
/// # Arguments
/// * `canton`          — code à 2 lettres du canton de travail (ex. "GE", "ZH")
/// * `tarif`           — code tarif ORIS (A0, A1, A2, B0, B1, B2, C0, C1, C2, H0, H1, H2)
/// * `salaire_mensuel` — salaire brut mensuel en CHF
///
/// Retourne `None` si le taux résultant est nul (salaire sous le seuil d'imposition).
pub fn calculer_is(canton: &str, tarif: &str, salaire_mensuel: Decimal, ctx: &ContextPaie) -> Option<LigneCotisation> {
    // Trouver la ligne du canton
    let (_, libelle_canton, taux_a0) = CANTON_A0.iter()
        .find(|(code, _, _)| *code == canton)?;

    // Trouver le multiplicateur du tarif
    let &(_, mult) = TARIF_MULT.iter()
        .find(|(t, _)| *t == tarif)?;

    // Trouver le libellé long du tarif
    let tarif_label = TARIF_LABELS.iter()
        .find(|(t, _)| *t == tarif)
        .map(|(_, l)| *l)
        .unwrap_or(tarif);

    // Convertir le salaire en u32 pour comparer aux seuils
    let sal_u32: u32 = salaire_mensuel.trunc().to_string().parse::<u32>().unwrap_or(0);

    // Trouver l'index du plus grand seuil <= salaire
    let idx = THRESHOLDS.iter()
        .rposition(|&s| s <= sal_u32)
        .unwrap_or(0);

    // Taux A0 brut (en %)
    let taux_a0_pct: f32 = taux_a0[idx];

    // Appliquer le multiplicateur de tarif
    let taux_final_pct: f32 = taux_a0_pct * mult;

    // Taux nul → pas d'IS (salaire sous le seuil d'imposition du canton)
    if taux_final_pct == 0.0 {
        return None;
    }

    // Convertir en Decimal (fraction décimale, pas en %)
    let taux_decimal = Decimal::from_str(&format!("{:.6}", taux_final_pct / 100.0))
        .unwrap_or(Decimal::ZERO);

    let montant = (salaire_mensuel * taux_decimal).round_dp(2);

    let libelle = ctx.libelle("CH_IS", "Impôt à la source — Tarif {tarif} — {libelle_canton}")
        .replace("{tarif}", tarif)
        .replace("{libelle_canton}", libelle_canton);
    let explication = ctx.expl("CH_IS",
        "L'impôt à la source (IS, Quellensteuer) est prélevé directement sur le salaire des \
        travailleurs étrangers sans permis d'établissement (C) résidant en Suisse, ainsi que des \
        frontaliers et des non-résidents (art. 83-90a LIFD). Il se substitue à la déclaration \
        fiscale ordinaire pour les revenus inférieurs à CHF 120 000/an (seuil de rectification \
        facultative, selon le canton).\
        \n\n\
        [ Paramètres retenus ]\n\
        Canton de travail : {canton} — {libelle_canton}\n\
        Tarif ORIS : {tarif} — {tarif_label}\n\
        Salaire mensuel brut : CHF {sal}\n\
        Seuil de barème retenu : CHF {seuil}\n\
        Taux A0 de base (tarif barème) : {taux_a0} %\n\
        Multiplicateur tarif {tarif} : × {mult}\n\
        Taux retenu : {taux_a0} % × {mult} = {taux_final} %\n\
        Montant IS = CHF {sal} × {taux_final} % = CHF {montant}\
        \n\n\
        La retenue est calculée mois par mois sur le salaire brut du mois considéré. \
        En cas de revenus irréguliers (primes, 13e mois, heures supplémentaires), \
        les cantons peuvent prévoir des règles de lissage. \
        Depuis la réforme RAS (ORIS du 12/11/2014, RS 642.118.2), un barème unique \
        par tarif s'applique à l'ensemble du territoire cantonal.")
        .replace("{canton}", canton)
        .replace("{libelle_canton}", libelle_canton)
        .replace("{tarif_label}", tarif_label)
        .replace("{tarif}", tarif)
        .replace("{sal}", &format!("{:.2}", salaire_mensuel))
        .replace("{seuil}", &THRESHOLDS[idx].to_string())
        .replace("{taux_a0}", &format!("{:.1}", taux_a0_pct))
        .replace("{mult}", &format!("{:.3}", mult))
        .replace("{taux_final}", &format!("{:.4}", taux_final_pct))
        .replace("{montant}", &format!("{:.2}", montant));
    Some(LigneCotisation {
        code:        "CH_IS".into(),
        libelle,
        base:        salaire_mensuel,
        taux_sal:    taux_decimal,
        montant_sal: montant,
        taux_pat:    Decimal::ZERO,
        montant_pat: Decimal::ZERO,
        categorie:   "Impôt à la source".into(),
        explication,
        loi_ref: Some(ctx.loi_ref("Art. 83-90a LIFD (RS 642.11) — ORIS (RS 642.118.2)")),
    })
}
