// Traductions États-Unis (codes `US_*`). Placeholders préservés (substitués par
// l'appelant) : {plaf_an} {plaf_m} {annee} {seuil} {seuil_m} {rev} {ded} {imp}
// {ia} {im} {teff} {nom} {desc} {loi}.
// Les libellés « Social Security », « Medicare », « Additional Medicare »,
// « California SDI » sont des noms propres → non traduits (repli FR, whitelistés).

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "US_FUTA" => match lang {
            "en" => "FUTA — Federal unemployment (employer)",
            "de" => "FUTA — Bundesarbeitslosenversicherung (Arbeitgeber)",
            "nl" => "FUTA — Federale werkloosheid (werkgever)",
            "it" => "FUTA — Disoccupazione federale (datore di lavoro)",
            "es" => "FUTA — Desempleo federal (empleador)",
            _ => return None,
        },
        "US_IMPOT_FED" => match lang {
            "en" => "Federal income tax — withholding {annee}",
            "de" => "Bundeseinkommensteuer — Einbehalt {annee}",
            "nl" => "Federale inkomstenbelasting — inhouding {annee}",
            "it" => "Imposta federale sul reddito — ritenuta {annee}",
            "es" => "Impuesto federal sobre la renta — retención {annee}",
            _ => return None,
        },
        "US_IMPOT_STATE" => match lang {
            "en" => "State income tax {nom} — withholding {annee}",
            "de" => "Einkommensteuer des Bundesstaates {nom} — Einbehalt {annee}",
            "nl" => "Deelstaatinkomstenbelasting {nom} — inhouding {annee}",
            "it" => "Imposta statale sul reddito {nom} — ritenuta {annee}",
            "es" => "Impuesto estatal sobre la renta {nom} — retención {annee}",
            _ => return None,
        },
        _ => return None,
    })
}

pub fn t_explication(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "US_SS" => match lang {
            "en" => "Social Security (Old-Age, Survivors and Disability Insurance). Rate 6.2 % \
                employee + 6.2 % employer, on pay capped at the annual wage base ({plaf_an} $/yr, \
                i.e. {plaf_m} $/month in {annee}). Above it, no more SS. Legal basis: 26 U.S.C. §3101(a) / §3111(a).",
            "de" => "Social Security (Alters-, Hinterbliebenen- und Invaliditätsversicherung). Satz \
                6,2 % Arbeitnehmer + 6,2 % Arbeitgeber, auf das bis zur jährlichen Beitragsbemessungsgrenze \
                gedeckelte Entgelt ({plaf_an} $/Jahr, d. h. {plaf_m} $/Monat in {annee}). Darüber keine SS. \
                Rechtsgrundlage: 26 U.S.C. §3101(a) / §3111(a).",
            "nl" => "Social Security (ouderdoms-, nabestaanden- en invaliditeitsverzekering). Tarief \
                6,2 % werknemer + 6,2 % werkgever, op loon begrensd tot de jaarlijkse wage base \
                ({plaf_an} $/jaar, ofwel {plaf_m} $/maand in {annee}). Daarboven geen SS meer. \
                Wettelijke basis: 26 U.S.C. §3101(a) / §3111(a).",
            "it" => "Social Security (assicurazione vecchiaia, superstiti e invalidità). Aliquota \
                6,2 % dipendente + 6,2 % datore, sulla retribuzione limitata al massimale annuo \
                ({plaf_an} $/anno, cioè {plaf_m} $/mese nel {annee}). Oltre, nessun contributo SS. \
                Base giuridica: 26 U.S.C. §3101(a) / §3111(a).",
            "es" => "Social Security (seguro de vejez, supervivencia e invalidez). Tipo 6,2 % \
                trabajador + 6,2 % empleador, sobre la retribución limitada a la base anual \
                ({plaf_an} $/año, es decir {plaf_m} $/mes en {annee}). Por encima, sin SS. \
                Base legal: 26 U.S.C. §3101(a) / §3111(a).",
            _ => return None,
        },
        "US_MEDICARE" => match lang {
            "en" => "Medicare (health insurance for seniors). Rate 1.45 % employee + 1.45 % employer, \
                no wage cap. Legal basis: 26 U.S.C. §3101(b) / §3111(b).",
            "de" => "Medicare (Krankenversicherung für Senioren). Satz 1,45 % Arbeitnehmer + 1,45 % \
                Arbeitgeber, ohne Bemessungsgrenze. Rechtsgrundlage: 26 U.S.C. §3101(b) / §3111(b).",
            "nl" => "Medicare (ziektekostenverzekering voor senioren). Tarief 1,45 % werknemer + \
                1,45 % werkgever, zonder loonplafond. Wettelijke basis: 26 U.S.C. §3101(b) / §3111(b).",
            "it" => "Medicare (assicurazione sanitaria per anziani). Aliquota 1,45 % dipendente + \
                1,45 % datore, senza massimale. Base giuridica: 26 U.S.C. §3101(b) / §3111(b).",
            "es" => "Medicare (seguro de salud para mayores). Tipo 1,45 % trabajador + 1,45 % \
                empleador, sin tope salarial. Base legal: 26 U.S.C. §3101(b) / §3111(b).",
            _ => return None,
        },
        "US_ADD_MEDICARE" => match lang {
            "en" => "Additional Medicare surtax of 0.9 % borne by the employee alone, on the portion \
                of pay above {seuil} $/yr ({seuil_m} $/month). The employer does not contribute. \
                Legal basis: 26 U.S.C. §3101(b)(2).",
            "de" => "Zusätzliche Medicare-Steuer von 0,9 % allein zulasten des Arbeitnehmers, auf den \
                Entgeltanteil über {seuil} $/Jahr ({seuil_m} $/Monat). Der Arbeitgeber zahlt nicht. \
                Rechtsgrundlage: 26 U.S.C. §3101(b)(2).",
            "nl" => "Aanvullende Medicare-heffing van 0,9 % uitsluitend ten laste van de werknemer, op \
                het loondeel boven {seuil} $/jaar ({seuil_m} $/maand). De werkgever draagt niet bij. \
                Wettelijke basis: 26 U.S.C. §3101(b)(2).",
            "it" => "Sovrattassa Medicare aggiuntiva dello 0,9 % a carico del solo dipendente, sulla \
                quota di retribuzione oltre {seuil} $/anno ({seuil_m} $/mese). Il datore non contribuisce. \
                Base giuridica: 26 U.S.C. §3101(b)(2).",
            "es" => "Recargo Medicare adicional del 0,9 % a cargo únicamente del trabajador, sobre la \
                parte de retribución superior a {seuil} $/año ({seuil_m} $/mes). El empleador no cotiza. \
                Base legal: 26 U.S.C. §3101(b)(2).",
            _ => return None,
        },
        "US_FUTA" => match lang {
            "en" => "Federal Unemployment Tax Act: federal unemployment, 100 % employer. Nominal rate \
                6.0 % reduced to 0.6 % effective via the state credit (5.4 %), on the first 7,000 $/yr. \
                State unemployment (SUTA), at an experience-rated variable rate, is not modelled. \
                Legal basis: 26 U.S.C. §3301.",
            "de" => "Federal Unemployment Tax Act: Bundesarbeitslosenversicherung, 100 % Arbeitgeber. \
                Nominalsatz 6,0 %, effektiv 0,6 % dank Staatskredit (5,4 %), auf die ersten 7.000 $/Jahr. \
                Die Arbeitslosenversicherung der Bundesstaaten (SUTA) mit variablem Satz wird nicht \
                modelliert. Rechtsgrundlage: 26 U.S.C. §3301.",
            "nl" => "Federal Unemployment Tax Act: federale werkloosheid, 100 % werkgever. Nominaal \
                tarief 6,0 %, effectief 0,6 % dankzij het staatskrediet (5,4 %), op de eerste 7.000 $/jaar. \
                De deelstaatwerkloosheid (SUTA) met variabel tarief wordt niet gemodelleerd. \
                Wettelijke basis: 26 U.S.C. §3301.",
            "it" => "Federal Unemployment Tax Act: disoccupazione federale, 100 % datore. Aliquota \
                nominale 6,0 % ridotta allo 0,6 % effettivo grazie al credito statale (5,4 %), sui primi \
                7.000 $/anno. La disoccupazione statale (SUTA), ad aliquota variabile, non è modellata. \
                Base giuridica: 26 U.S.C. §3301.",
            "es" => "Federal Unemployment Tax Act: desempleo federal, 100 % empleador. Tipo nominal \
                6,0 % reducido al 0,6 % efectivo por el crédito estatal (5,4 %), sobre los primeros \
                7.000 $/año. El desempleo estatal (SUTA), de tipo variable, no se modela. \
                Base legal: 26 U.S.C. §3301.",
            _ => return None,
        },
        "US_CA_SDI" => match lang {
            "en" => "California State Disability Insurance: 1.2 % borne by the employee in 2025, with \
                no wage cap since 01/01/2024 (SB 951). Funds disability insurance and paid family leave \
                (PFL). Legal basis: California Unemployment Insurance Code §984.",
            "de" => "California State Disability Insurance: 1,2 % zulasten des Arbeitnehmers 2025, ohne \
                Bemessungsgrenze seit 01.01.2024 (SB 951). Finanziert Invaliditätsversicherung und \
                bezahlten Familienurlaub (PFL). Rechtsgrundlage: California Unemployment Insurance Code §984.",
            "nl" => "California State Disability Insurance: 1,2 % ten laste van de werknemer in 2025, \
                zonder loonplafond sinds 01-01-2024 (SB 951). Financiert arbeidsongeschiktheids­verzekering \
                en betaald familieverlof (PFL). Wettelijke basis: California Unemployment Insurance Code §984.",
            "it" => "California State Disability Insurance: 1,2 % a carico del dipendente nel 2025, senza \
                massimale dal 01/01/2024 (SB 951). Finanzia l'assicurazione invalidità e il congedo \
                familiare retribuito (PFL). Base giuridica: California Unemployment Insurance Code §984.",
            "es" => "California State Disability Insurance: 1,2 % a cargo del trabajador en 2025, sin tope \
                salarial desde el 01/01/2024 (SB 951). Financia el seguro de invalidez y el permiso \
                familiar retribuido (PFL). Base legal: California Unemployment Insurance Code §984.",
            _ => return None,
        },
        "US_IMPOT_FED" => match lang {
            "en" => "Federal income tax withholding, single filer with no dependents.\nAnnualised \
                income: {rev} $\nStandard deduction: − {ded} $\nTaxable income: {imp} $\nBrackets {annee}: \
                10/12/22/24/32/35/37 %\nAnnual tax: {ia} $ / 12 = {im} $/month\nEffective rate: {teff} %\n\
                Legal basis: 26 U.S.C. §1 and §63.",
            "de" => "Bundeseinkommensteuer-Einbehalt, alleinstehend ohne Angehörige.\nJahresbasiertes \
                Einkommen: {rev} $\nStandardabzug: − {ded} $\nZu versteuerndes Einkommen: {imp} $\n\
                Tarif {annee}: 10/12/22/24/32/35/37 %\nJahressteuer: {ia} $ / 12 = {im} $/Monat\n\
                Effektivsatz: {teff} %\nRechtsgrundlage: 26 U.S.C. §1 und §63.",
            "nl" => "Federale inkomstenbelasting-inhouding, alleenstaand zonder personen ten laste.\n\
                Jaarinkomen: {rev} $\nStandaardaftrek: − {ded} $\nBelastbaar inkomen: {imp} $\n\
                Schijven {annee}: 10/12/22/24/32/35/37 %\nJaarbelasting: {ia} $ / 12 = {im} $/maand\n\
                Effectief tarief: {teff} %\nWettelijke basis: 26 U.S.C. §1 en §63.",
            "it" => "Ritenuta dell'imposta federale sul reddito, single senza persone a carico.\n\
                Reddito annualizzato: {rev} $\nDeduzione standard: − {ded} $\nReddito imponibile: {imp} $\n\
                Scaglioni {annee}: 10/12/22/24/32/35/37 %\nImposta annua: {ia} $ / 12 = {im} $/mese\n\
                Aliquota effettiva: {teff} %\nBase giuridica: 26 U.S.C. §1 e §63.",
            "es" => "Retención del impuesto federal sobre la renta, soltero sin personas a cargo.\n\
                Renta anualizada: {rev} $\nDeducción estándar: − {ded} $\nRenta imponible: {imp} $\n\
                Tramos {annee}: 10/12/22/24/32/35/37 %\nImpuesto anual: {ia} $ / 12 = {im} $/mes\n\
                Tipo efectivo: {teff} %\nBase legal: 26 U.S.C. §1 y §63.",
            _ => return None,
        },
        "US_IMPOT_STATE" => match lang {
            "en" => "State income tax — {nom} ({desc}), single filer with no dependents.\nAnnualised \
                income: {rev} $\nTaxable income: {imp} $ (after {ded} $ deduction)\nAnnual tax: {ia} $ / 12 \
                = {im} $/month\nEffective rate: {teff} %\nLegal basis: {loi}.",
            "de" => "Einkommensteuer des Bundesstaates — {nom} ({desc}), alleinstehend ohne Angehörige.\n\
                Jahresbasiertes Einkommen: {rev} $\nZu versteuerndes Einkommen: {imp} $ (nach Abzug {ded} $)\n\
                Jahressteuer: {ia} $ / 12 = {im} $/Monat\nEffektivsatz: {teff} %\nRechtsgrundlage: {loi}.",
            "nl" => "Deelstaatinkomstenbelasting — {nom} ({desc}), alleenstaand zonder personen ten laste.\n\
                Jaarinkomen: {rev} $\nBelastbaar inkomen: {imp} $ (na aftrek {ded} $)\nJaarbelasting: {ia} $ \
                / 12 = {im} $/maand\nEffectief tarief: {teff} %\nWettelijke basis: {loi}.",
            "it" => "Imposta statale sul reddito — {nom} ({desc}), single senza persone a carico.\n\
                Reddito annualizzato: {rev} $\nReddito imponibile: {imp} $ (dopo deduzione {ded} $)\n\
                Imposta annua: {ia} $ / 12 = {im} $/mese\nAliquota effettiva: {teff} %\nBase giuridica: {loi}.",
            "es" => "Impuesto estatal sobre la renta — {nom} ({desc}), soltero sin personas a cargo.\n\
                Renta anualizada: {rev} $\nRenta imponible: {imp} $ (tras deducción {ded} $)\nImpuesto \
                anual: {ia} $ / 12 = {im} $/mes\nTipo efectivo: {teff} %\nBase legal: {loi}.",
            _ => return None,
        },
        _ => return None,
    })
}
