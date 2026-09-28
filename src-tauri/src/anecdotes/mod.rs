// Histoire des cotisations et prélèvements : anecdotes, acteurs, controverses.
//
// L'explication d'une ligne dit CE QUE FAIT la cotisation ; ce module raconte
// d'où elle vient, qui l'a portée et qui l'a combattue. Les deux restent
// séparés : l'explication est technique et chiffrée (placeholders, calcul du
// mois), l'anecdote est un texte fixe, rattaché au CODE de la ligne.
//
// Transport : `generer_bulletin` ajoute l'anecdote à la fin de l'explication,
// derrière le séparateur `SEPARATEUR` (U+0002). Le front l'en détache (comme le
// détail « aide au poste » derrière U+0001) et l'affiche dans un bloc distinct.
// Aucune structure de données n'est touchée : `LigneCotisation` garde sa forme.
//
// Règle de rédaction — la même que pour les chiffres : rien d'inventé. On ne
// retient que des faits établis (lois, dates, acteurs, votations, controverses
// documentées) ; pas de citation incertaine ni de chiffre approximatif. Un code
// sans anecdote sûre n'en reçoit pas : l'absence vaut mieux qu'une légende.
//
// Langues : les six du menu (fr, en, de, nl, it, es), un fichier par langue,
// chacun traduction fidèle du français (`fr.rs`, texte de référence). Le test
// `memes_codes_dans_toutes_les_langues` exige que chaque langue couvre
// exactement les mêmes codes : une anecdote ajoutée en français sans ses
// traductions casse le test. Une langue inconnue ne reçoit rien (et non le
// français) — un bulletin affiché dans une langue ne doit pas en changer en
// cours de lecture.

mod de;
mod en;
mod es;
mod fr;
mod it;
mod nl;

/// Séparateur entre l'explication et l'anecdote (caractère de contrôle STX).
pub const SEPARATEUR: char = '\u{2}';

/// Table des anecdotes d'une langue, ou None si la langue n'est pas couverte.
fn table(lang: &str) -> Option<&'static [(&'static str, &'static str)]> {
    Some(match lang {
        "fr" => fr::TEXTES,
        "en" => en::TEXTES,
        "de" => de::TEXTES,
        "nl" => nl::TEXTES,
        "it" => it::TEXTES,
        "es" => es::TEXTES,
        _ => return None,
    })
}

/// Anecdote historique d'une cotisation, ou None.
pub fn anecdote(code: &str, lang: &str) -> Option<&'static str> {
    // Variantes d'une même cotisation : même histoire.
    let code = match code {
        c if c.starts_with("IT_ADD_REG_") => "IT_ADD_REG",   // une par région (IT_ADD_REG_LO…)
        c if c.starts_with("IT_ESONERO_") => "IT_ESONERO",   // une par année
        "IT_NASPI_TERMINE" => "IT_NASPI",
        "DE_PV_KINDERLOS"  => "DE_PFLEGEVERSICHERUNG",
        "PT_FGCT"          => "PT_FCT",
        "QC_RRQ2"          => "CA_RPC2",
        c => c,
    };
    table(lang)?.iter().find(|(c, _)| *c == code).map(|(_, t)| *t)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LANGUES: [&str; 6] = ["fr", "en", "de", "nl", "it", "es"];

    #[test]
    fn variantes_et_langue_inconnue() {
        assert!(anecdote("CSG_DEDUCTIBLE", "fr").is_some());
        assert!(anecdote("CSG_DEDUCTIBLE", "pt").is_none());
        assert_eq!(anecdote("IT_ADD_REG_LO", "fr"), anecdote("IT_ADD_REG", "fr"));
        assert_eq!(anecdote("IT_ADD_REG_LO", "de"), anecdote("IT_ADD_REG", "de"));
        assert!(anecdote("CODE_INCONNU", "fr").is_none());
    }

    /// Chaque langue couvre exactement les codes du français, sans doublon ;
    /// aucun texte vide, aucun séparateur, aucun texte resté en français.
    #[test]
    fn memes_codes_dans_toutes_les_langues() {
        let codes = |l: &str| {
            let mut v: Vec<&str> = table(l).unwrap().iter().map(|(c, _)| *c).collect();
            let n = v.len();
            v.sort();
            v.dedup();
            assert_eq!(v.len(), n, "{l} : code en double");
            v
        };
        let fr = codes("fr");
        for l in LANGUES {
            assert_eq!(codes(l), fr, "{l} : codes différents du français");
            for (c, t) in table(l).unwrap() {
                assert!(!t.trim().is_empty(), "{l}/{c} : texte vide");
                assert!(!t.contains(SEPARATEUR), "{l}/{c} : séparateur dans le texte");
                if l != "fr" {
                    assert_ne!(Some(*t), anecdote(c, "fr"), "{l}/{c} : texte resté en français");
                }
            }
        }
    }
}
