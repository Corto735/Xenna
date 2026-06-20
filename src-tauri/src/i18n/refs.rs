// Traduction des références légales (`loi_ref`) — mots descriptifs uniquement.
//
// Règle (décision produit) : on traduit seulement les MOTS descriptifs
// (Loi, Décret, Convention, Ordonnance, Accord…) ; les CITATIONS (numéros,
// articles « art. L241-3 », « SGB V §241-242 », noms de lois étrangères) restent
// INTACTES. La plupart des références non françaises sont déjà rédigées dans la
// langue nationale (grec, hongrois, tchèque, chinois…) et ne contiennent aucun
// fragment français : elles traversent donc cette fonction inchangées.
//
// Les remplacements sont appliqués dans l'ordre (fragments les plus longs
// d'abord) pour éviter qu'un fragment court n'altère un fragment long.

/// Fragments descriptifs français → traductions [en, de, nl, it, es].
/// Ordre = longueur décroissante / spécificité (les phrases avant les mots).
const FRAGMENTS: &[(&str, [&str; 5])] = &[
    ("Accord national interprofessionnel", [
        "National interprofessional agreement",
        "Nationales branchenübergreifendes Abkommen",
        "Nationaal interprofessioneel akkoord",
        "Accordo nazionale interprofessionale",
        "Acuerdo nacional interprofesional",
    ]),
    ("portant introduction de l'assurance dépendance", [
        "introducing long-term care insurance",
        "zur Einführung der Pflegeversicherung",
        "tot invoering van de afhankelijkheidsverzekering",
        "che introduce l'assicurazione dipendenza",
        "que introduce el seguro de dependencia",
    ]),
    ("suppression cotisation sal.", [
        "abolition of employee contribution",
        "Abschaffung des Arbeitnehmerbeitrags",
        "afschaffing werknemersbijdrage",
        "soppressione contributo dipendente",
        "supresión cotización trabajador",
    ]),
    ("réformé par loi", [
        "reformed by law",
        "reformiert durch Gesetz",
        "hervormd bij wet",
        "riformato dalla legge",
        "reformado por ley",
    ]),
    ("réforme retraites", [
        "pension reform", "Rentenreform", "pensioenhervorming",
        "riforma pensioni", "reforma de pensiones",
    ]),
    ("Caisses Sociales de Monaco", [
        "Monaco Social Funds", "Sozialkassen von Monaco", "Sociale kassen van Monaco",
        "Casse Sociali di Monaco", "Cajas Sociales de Mónaco",
    ]),
    ("Congé paternité", [
        "Paternity leave", "Vaterschaftsurlaub", "Vaderschapsverlof",
        "Congedo di paternità", "Permiso de paternidad",
    ]),
    ("Loi locale du", [
        "Local law of", "Lokales Gesetz vom", "Lokale wet van",
        "Legge locale del", "Ley local del",
    ]),
    ("Loi locale", [
        "Local law", "Lokales Gesetz", "Lokale wet", "Legge locale", "Ley local",
    ]),
    ("Convention UNEDIC", [
        "UNEDIC agreement", "UNEDIC-Abkommen", "UNEDIC-overeenkomst",
        "Convenzione UNEDIC", "Convenio UNEDIC",
    ]),
    ("Convention du", [
        "Agreement of", "Abkommen vom", "Overeenkomst van",
        "Convenzione del", "Convenio del",
    ]),
    ("Convention", [
        "Agreement", "Abkommen", "Overeenkomst", "Convenzione", "Convenio",
    ]),
    ("Circulaires", [
        "circulars", "Rundschreiben", "circulaires", "circolari", "circulares",
    ]),
    ("Circulaire", [
        "circular", "Rundschreiben", "circulaire", "circolare", "circular",
    ]),
    ("Ordonnance", ["Order", "Verordnung", "Verordening", "Ordinanza", "Orden"]),
    ("Décret", ["Decree", "Dekret", "Decreet", "Decreto", "Decreto"]),
    ("Arrêté du", [
        "Decree of", "Erlass vom", "Besluit van", "Decreto del", "Orden del",
    ]),
    ("Arrêté", ["Decree", "Erlass", "Besluit", "Decreto", "Orden"]),
    ("Loi du", ["Law of", "Gesetz vom", "Wet van", "Legge del", "Ley del"]),
    ("Loi", ["Law", "Gesetz", "Wet", "Legge", "Ley"]),
    ("Réforme", ["Reform", "Reform", "Hervorming", "Riforma", "Reforma"]),
    ("réformée", ["reformed", "reformiert", "hervormd", "riformata", "reformada"]),
    ("créée par", [
        "created by", "geschaffen von", "gecreëerd door", "creata da", "creada por",
    ]),
    ("selon contrat", [
        "depending on contract", "je nach Vertrag", "afhankelijk van contract",
        "secondo contratto", "según contrato",
    ]),
    ("Article", ["Article", "Artikel", "Artikel", "Articolo", "Artículo"]),
    ("Livre ", ["Book ", "Buch ", "Boek ", "Libro ", "Libro "]),
    ("annuelles", ["annual", "jährlich", "jaarlijkse", "annuali", "anuales"]),
    ("(soins)", [
        "(care)", "(Sachleistungen)", "(zorg)", "(prestazioni)", "(asistencia)",
    ]),
    ("(indemnités)", [
        "(benefits)", "(Geldleistungen)", "(uitkeringen)", "(indennità)", "(prestaciones)",
    ]),
    ("et s.", ["et seq.", "ff.", "e.v.", "e segg.", "y ss."]),
    (" et ", [" and ", " und ", " en ", " e ", " y "]),
    (" ou ", [" or ", " oder ", " of ", " o ", " o "]),
    (" du ", [" of ", " vom ", " van ", " del ", " del "]),
];

/// Traduit les mots descriptifs d'une référence légale, citations conservées.
/// `lang` ∈ {en, de, nl, it, es} ; tout autre code renvoie le texte inchangé.
pub fn traduire(fr: &str, lang: &str) -> String {
    let idx = match lang {
        "en" => 0,
        "de" => 1,
        "nl" => 2,
        "it" => 3,
        "es" => 4,
        _ => return fr.to_string(),
    };
    let mut s = fr.to_string();
    for (from, tos) in FRAGMENTS {
        if s.contains(from) {
            s = s.replace(from, tos[idx]);
        }
    }
    s
}
