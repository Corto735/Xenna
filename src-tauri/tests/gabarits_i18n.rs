//! Parité des placeholders entre un gabarit français et ses traductions.
//!
//! Le test `i18n` attrape un `{…}` resté NON substitué dans un texte traduit ;
//! il ne voit pas celui qui MANQUE. Or c'est le défaut qui se produit en
//! pratique : on enrichit le gabarit français d'un `ctx.expl("CLE", "…{x}…")`,
//! on oublie les cinq traductions, et le lecteur étranger perd l'information
//! sans qu'aucun test ne bronche (cas de REDUCTION_FILLON_* : `{abs_info}`,
//! `{annee}` et `{ref_smic}` absents des cinq langues).
//!
//! Méthode : on relit les sources, on retient chaque appel `.expl("CLE", "…")`
//! dont le texte français est un littéral, et on exige que chaque traduction
//! (`i18n::t_explication`) porte EXACTEMENT les mêmes placeholders. Les rares
//! appels à clé ou texte calculés échappent à l'analyse ; leur nombre est borné
//! pour qu'un glissement massif vers cette forme ne passe pas inaperçu.

use std::collections::BTreeSet;
use std::path::Path;

use xenna_paie_lib::i18n::t_explication;

const LANGUES: [&str; 5] = ["en", "de", "nl", "it", "es"];

/// Dette connue au 23/10/2026 : placeholders du français absents des cinq
/// traductions (information perdue pour le lecteur étranger : année du barème,
/// plafond, mention temps partiel…). Tolérés tels quels ; un NOUVEL écart fait
/// échouer le test, et une entrée comblée doit être retirée d'ici.
const DETTE: &[(&str, &[&str])] = &[
    ("AGIRC_ARRCO_T1", &["etp_info"]),
    ("AGIRC_ARRCO_T2", &["etp_info"]),
    ("AT_SV", &["hbgl"]),
    ("BG_DANAK", &["annee", "sym"]),
    ("BG_OSIG", &["pl", "sym"]),
    ("CHOMAGE", &["etp_info"]),
    ("CY_FOROS", &["annee", "bareme"]),
    ("CY_SI", &["plaf"]),
    ("CZ_DAN", &["annee", "seuil"]),
    ("EE_TULUMAKS", &["abatt_txt", "annee"]),
    ("GR_EFKA", &["pl"]),
    ("HR_POREZ", &["annee"]),
    ("HU_SZJA", &["annee"]),
    ("IE_PAYE", &["annee"]),
    ("IE_USC", &["annee", "seuils"]),
    ("LT_GPM", &["annee", "bareme"]),
    ("LV_IIN", &["annee", "mni"]),
    ("MT_SSC", &["pl"]),
    ("MT_TAX", &["annee"]),
    ("SI_DOHODNINA", &["ab", "annee", "seuils"]),
    ("SS_VIEILLESSE_PLAF", &["etp_info"]),
];

/// Appels `.expl(…)` dont la clé ou le texte français n'est pas un littéral.
const NON_ANALYSABLES_MAX: usize = 6;

fn sources(dir: &Path, out: &mut Vec<(String, String)>) {
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push((p.display().to_string(), std::fs::read_to_string(&p).unwrap()));
        }
    }
}

/// Décode un littéral de chaîne Rust ordinaire commençant à `s[0] == '"'`.
/// Rend le texte et la longueur consommée.
fn litteral(s: &str) -> (String, usize) {
    let b: Vec<char> = s.chars().collect();
    let (mut i, mut out) = (1, String::new());
    while i < b.len() {
        match b[i] {
            '"' => return (out, b[..=i].iter().map(|c| c.len_utf8()).sum()),
            '\\' => {
                i += 1;
                match b[i] {
                    // Continuation de ligne : saute le saut et l'indentation suivante.
                    '\n' => {
                        while i + 1 < b.len() && b[i + 1].is_whitespace() { i += 1; }
                    }
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    'r' => out.push('\r'),
                    '0' => out.push('\0'),
                    'u' => {
                        let fin = (i..b.len()).find(|&k| b[k] == '}').unwrap();
                        let hex: String = b[i + 2..fin].iter().collect();
                        out.push(char::from_u32(u32::from_str_radix(&hex, 16).unwrap()).unwrap());
                        i = fin;
                    }
                    'x' => {
                        let hex: String = b[i + 1..i + 3].iter().collect();
                        out.push(u8::from_str_radix(&hex, 16).unwrap() as char);
                        i += 2;
                    }
                    c => out.push(c), // \" \\ \'
                }
            }
            c => out.push(c),
        }
        i += 1;
    }
    panic!("littéral non terminé");
}

/// Placeholders nommés `{mot}` (minuscules, chiffres, soulignés).
fn placeholders(s: &str) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    let mut reste = s;
    while let Some(d) = reste.find('{') {
        let apres = &reste[d + 1..];
        match apres.find('}') {
            Some(f) if f > 0
                && apres[..f].chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') =>
            {
                set.insert(apres[..f].to_string());
                reste = &apres[f + 1..];
            }
            _ => reste = apres,
        }
    }
    set
}

#[test]
fn traductions_memes_placeholders_que_le_francais() {
    let racine = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut fichiers = Vec::new();
    sources(&racine, &mut fichiers);

    let (mut analyses, mut non_analysables, mut ecarts) = (0usize, Vec::new(), Vec::new());
    let mut dette_vue = BTreeSet::new();

    for (chemin, texte) in &fichiers {
        let mut reste = texte.as_str();
        while let Some(pos) = reste.find(".expl(") {
            let ligne = texte[..texte.len() - reste.len() + pos].lines().count();
            reste = &reste[pos + ".expl(".len()..];
            let r = reste.trim_start();
            // Clé littérale "CLE", puis texte français littéral.
            let cle = r.strip_prefix('"').and_then(|x| x.find('"').map(|f| &x[..f]));
            let Some(cle) = cle.filter(|c| c.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')) else {
                non_analysables.push(format!("{chemin}:{ligne}"));
                continue;
            };
            let r = r[cle.len() + 2..].trim_start().strip_prefix(',').unwrap_or("").trim_start();
            if !r.starts_with('"') {
                non_analysables.push(format!("{chemin}:{ligne}"));
                continue;
            }
            let (fr, _) = litteral(r);
            let attendus = placeholders(&fr);
            analyses += 1;
            for lang in LANGUES {
                let Some(tr) = t_explication(cle, lang) else { continue };
                let trouves = placeholders(tr);
                if trouves != attendus {
                    let manque: Vec<_> = attendus.difference(&trouves).collect();
                    let en_trop: Vec<_> = trouves.difference(&attendus).collect();
                    let dette = DETTE.iter().find(|(k, _)| *k == cle).map(|(_, v)| *v);
                    if en_trop.is_empty() && dette.is_some_and(|v| v.iter().copied().eq(manque.iter().map(|m| m.as_str()))) {
                        dette_vue.insert(cle);
                        continue;
                    }
                    ecarts.push(format!(
                        "{cle}/{lang} ({chemin}:{ligne}) — manquants {manque:?}, en trop {en_trop:?}"
                    ));
                }
            }
        }
    }

    assert!(analyses > 100, "analyse des sources suspecte : {analyses} appels seulement");
    assert!(
        non_analysables.len() <= NON_ANALYSABLES_MAX,
        "{} appels .expl() non analysables (max {NON_ANALYSABLES_MAX}) :\n{}",
        non_analysables.len(),
        non_analysables.join("\n")
    );
    let soldees: Vec<_> = DETTE.iter().map(|(k, _)| *k).filter(|k| !dette_vue.contains(k)).collect();
    assert!(soldees.is_empty(), "dette comblée, à retirer de DETTE : {soldees:?}");
    assert!(ecarts.is_empty(), "{} écarts de placeholders :\n{}", ecarts.len(), ecarts.join("\n"));
}
