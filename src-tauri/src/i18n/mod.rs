// Traductions curées des libellés et explications de cotisations.
//
// Le français est la langue native du code (rédigée en dur dans les
// fonctions de cotisation) : il sert de repli. Les autres langues
// (en, de, nl, it, es — celles du menu 🌐 LANGUE) sont fournies ici,
// indexées par le `code` stable de chaque cotisation.
//
// Les explications DYNAMIQUES (PMSS, dates, formule Fillon) sont stockées
// sous forme de gabarits à placeholders nommés `{pmss}`, `{annee}`, `{coeff}`…
// identiques au gabarit français, et substitués côté appelant via `.replace`.
//
// Organisation : un module par pays (codes préfixés `IT_`, `CH_`, …) plus
// `cotisations` (France, codes sans préfixe pays). Le dispatcher ci-dessous
// route par préfixe de code. `categories` et `refs` couvrent respectivement
// les en-têtes de catégorie et les fragments descriptifs des références légales.

pub mod cotisations; // France (codes sans préfixe pays)
pub mod it;
pub mod ch;
pub mod lu;
pub mod es;
pub mod pt;
pub mod divers; // pays « mono-fichier » (AD, AT, BG, CY, CZ, DK, EE, FI, GR, HR,
                // HU, IE, LV, LT, MT, SI, SE, RO, PL, SK, NZ, NL, MC, CN, UK,
                // AU, BE, CA + provinces)
pub mod kr;     // Corée du Sud
pub mod jp;     // Japon
pub mod de;     // Allemagne
pub mod non_couvert; // messages « année non couverte » par pays
pub mod refs;

/// Préfixe pays d'un code de cotisation (`IT_IVS` → `IT`).
fn prefixe(code: &str) -> &str {
    code.split_once('_').map(|(p, _)| p).unwrap_or(code)
}

/// Libellé traduit d'une cotisation, ou None si non couvert → repli français.
///
/// On route par préfixe vers le module pays ; si celui-ci ne couvre pas le code,
/// on retombe sur la table France. Cela résout aussi la collision de préfixe
/// `AT_MP` (France) vs `AT_*` (Autriche) : `divers` renvoie None → France répond.
pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    let par_pays = match prefixe(code) {
        "IT" => it::t_libelle(code, lang),
        "CH" => ch::t_libelle(code, lang),
        "LU" => lu::t_libelle(code, lang),
        "ES" => es::t_libelle(code, lang),
        "PT" => pt::t_libelle(code, lang),
        "KR" => kr::t_libelle(code, lang),
        "JP" => jp::t_libelle(code, lang),
        "DE" => de::t_libelle(code, lang),
        "AD" | "AT" | "BG" | "CY" | "CZ" | "DK" | "EE" | "FI" | "GR" | "HR" | "HU"
        | "IE" | "LV" | "LT" | "MT" | "SI" | "SE" | "RO" | "PL" | "SK" | "NZ" | "NL"
        | "MC" | "CN" | "UK" | "AU" | "BE"
        | "CA" | "ON" | "QC" | "AB" | "BC" | "MB" | "NB" | "NS" | "NT" | "NU" | "PE"
        | "YT" => divers::t_libelle(code, lang),
        _ => None,
    };
    par_pays.or_else(|| cotisations::t_libelle(code, lang)) // France (défaut + repli)
}

/// Explication traduite (ou gabarit à placeholders), ou None → repli français.
pub fn t_explication(key: &str, lang: &str) -> Option<&'static str> {
    let par_pays = match prefixe(key) {
        "IT" => it::t_explication(key, lang),
        "CH" => ch::t_explication(key, lang),
        "LU" => lu::t_explication(key, lang),
        "ES" => es::t_explication(key, lang),
        "PT" => pt::t_explication(key, lang),
        "KR" => kr::t_explication(key, lang),
        "JP" => jp::t_explication(key, lang),
        "DE" => de::t_explication(key, lang),
        "AD" | "AT" | "BG" | "CY" | "CZ" | "DK" | "EE" | "FI" | "GR" | "HR" | "HU"
        | "IE" | "LV" | "LT" | "MT" | "SI" | "SE" | "RO" | "PL" | "SK" | "NZ" | "NL"
        | "MC" | "CN" | "UK" | "AU" | "BE"
        | "CA" | "ON" | "QC" | "AB" | "BC" | "MB" | "NB" | "NS" | "NT" | "NU" | "PE"
        | "YT" => divers::t_explication(key, lang),
        _ => None,
    };
    par_pays.or_else(|| cotisations::t_explication(key, lang)) // France (défaut + repli)
}

/// Référence légale avec mots descriptifs traduits, citations conservées.
/// Renvoie une nouvelle chaîne (substitution de fragments). En `fr` ou si rien
/// ne matche, renvoie le texte d'origine inchangé.
pub fn t_loi_ref(fr: &str, lang: &str) -> String {
    refs::traduire(fr, lang)
}
