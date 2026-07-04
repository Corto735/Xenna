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
    // ── Codes et sigles français : nom officiel conservé + glose traduite ──
    ("Code général de la fonction publique", [
        "Code général de la fonction publique (General Civil Service Code)",
        "Code général de la fonction publique (Allgemeines Beamtengesetzbuch)",
        "Code général de la fonction publique (Algemeen wetboek overheidsdienst)",
        "Code général de la fonction publique (Codice generale della funzione pubblica)",
        "Code général de la fonction publique (Código general de la función pública)",
    ]),
    ("Code de la sécurité sociale", [
        "Code de la sécurité sociale (Social Security Code)",
        "Code de la sécurité sociale (Sozialversicherungsgesetzbuch)",
        "Code de la sécurité sociale (Wetboek sociale zekerheid)",
        "Code de la sécurité sociale (Codice della sicurezza sociale)",
        "Code de la sécurité sociale (Código de la Seguridad Social)",
    ]),
    ("Code du travail", [
        "Code du travail (French Labour Code)",
        "Code du travail (frz. Arbeitsgesetzbuch)",
        "Code du travail (Frans arbeidswetboek)",
        "Code du travail (Codice del lavoro francese)",
        "Code du travail (Código laboral francés)",
    ]),
    ("CSS LU", [
        "CSS LU (Luxembourg Social Security Code)",
        "CSS LU (luxemburgisches Sozialversicherungsgesetzbuch)",
        "CSS LU (Luxemburgs wetboek sociale zekerheid)",
        "CSS LU (Codice lussemburghese della sicurezza sociale)",
        "CSS LU (Código luxemburgués de la Seguridad Social)",
    ]),
    ("CSS art.", [
        "CSS (Social Security Code) art.",
        "CSS (Sozialversicherungsgesetzbuch) Art.",
        "CSS (Wetboek sociale zekerheid) art.",
        "CSS (Codice della sicurezza sociale) art.",
        "CSS (Código de la Seguridad Social) art.",
    ]),
    ("CT art.", [
        "CT (French Labour Code) art.",
        "CT (frz. Arbeitsgesetzbuch) Art.",
        "CT (Frans arbeidswetboek) art.",
        "CT (Codice del lavoro francese) art.",
        "CT (Código laboral francés) art.",
    ]),
    ("CGFP art.", [
        "CGFP (General Civil Service Code) art.",
        "CGFP (Allgemeines Beamtengesetzbuch) Art.",
        "CGFP (Algemeen wetboek overheidsdienst) art.",
        "CGFP (Codice generale della funzione pubblica) art.",
        "CGFP (Código general de la función pública) art.",
    ]),
    ("BOSS", [
        "BOSS (official Social Security bulletin)",
        "BOSS (amtliches Bulletin der Sozialversicherung)",
        "BOSS (officieel bulletin van de sociale zekerheid)",
        "BOSS (bollettino ufficiale della sicurezza sociale)",
        "BOSS (boletín oficial de la Seguridad Social)",
    ]),
    ("URSSAF", [
        "URSSAF (social security collection body)",
        "URSSAF (Beitragseinzugsstelle der Sozialversicherung)",
        "URSSAF (inningsorgaan sociale zekerheid)",
        "URSSAF (ente di riscossione dei contributi)",
        "URSSAF (organismo de recaudación de cotizaciones)",
    ]),
    ("ANI 17/11/2017", [
        "ANI (national interprofessional agreement) of 17/11/2017",
        "ANI (nationales branchenübergreifendes Abkommen) vom 17/11/2017",
        "ANI (nationaal interprofessioneel akkoord) van 17/11/2017",
        "ANI (accordo nazionale interprofessionale) del 17/11/2017",
        "ANI (acuerdo nacional interprofesional) del 17/11/2017",
    ]),
    // ── Références québécoises / canadiennes (noms officiels bilingues) ──
    ("Loi sur les normes du travail", [
        "Loi sur les normes du travail (Act respecting labour standards)",
        "Loi sur les normes du travail (Gesetz über Arbeitsnormen)",
        "Loi sur les normes du travail (Wet op de arbeidsnormen)",
        "Loi sur les normes du travail (Legge sulle norme del lavoro)",
        "Loi sur les normes du travail (Ley sobre normas laborales)",
    ]),
    ("Règlement sur l'assurance-emploi", [
        "Règlement sur l'assurance-emploi (Employment Insurance Regulations)",
        "Règlement sur l'assurance-emploi (Verordnung über die Beschäftigungsversicherung)",
        "Règlement sur l'assurance-emploi (Verordening werkverzekering)",
        "Règlement sur l'assurance-emploi (Regolamento sull'assicurazione occupazionale)",
        "Règlement sur l'assurance-emploi (Reglamento del seguro de empleo)",
    ]),
    ("Accord Canada-Québec", [
        "Canada-Québec Agreement", "Abkommen Kanada-Québec", "Akkoord Canada-Québec",
        "Accordo Canada-Québec", "Acuerdo Canadá-Quebec",
    ]),
    (" sur le RQAP", [
        " on the RQAP", " zum RQAP", " over het RQAP", " sul RQAP", " sobre el RQAP",
    ]),
    ("Règlement sur le", [
        "Regulations on the", "Verordnung über den", "Verordening over de",
        "Regolamento sul", "Reglamento sobre el",
    ]),
    ("Règlement", [
        "Regulations", "Verordnung", "Verordening", "Regolamento", "Reglamento",
    ]),
    ("Formulaire", ["Form", "Formular", "Formulier", "Modulo", "Formulario"]),
    // ── Mots descriptifs complémentaires ──
    ("réd. structurelle patronale", [
        "structural employer reduction", "strukturelle Arbeitgeberermäßigung",
        "structurele werkgeversvermindering", "riduzione strutturale datoriale",
        "reducción estructural patronal",
    ]),
    ("CCNL applicable", [
        "applicable CCNL", "anwendbarer CCNL", "toepasselijke CCNL",
        "CCNL applicabile", "CCNL aplicable",
    ]),
    ("(plan Juppé)", [
        "(Juppé plan)", "(Juppé-Plan)", "(plan-Juppé)", "(piano Juppé)", "(plan Juppé)",
    ]),
    ("(statut général FP)", [
        "(general civil-service statute)", "(allgemeines Beamtenstatut)",
        "(algemeen ambtenarenstatuut)", "(statuto generale della funzione pubblica)",
        "(estatuto general de la función pública)",
    ]),
    ("(barème)", ["(scale)", "(Tarif)", "(schaal)", "(scala)", "(escala)"]),
    ("(Finlande)", ["(Finland)", "(Finnland)", "(Finland)", "(Finlandia)", "(Finlandia)"]),
    ("(unification BBG)", [
        "(BBG unification)", "(BBG-Vereinheitlichung)", "(BBG-unificatie)",
        "(unificazione BBG)", "(unificación BBG)",
    ]),
    ("annuels", ["annual", "jährlich", "jaarlijkse", "annuali", "anuales"]),
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
