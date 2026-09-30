// Traductions des cotisations françaises.
//
// Deux tables : libellés (`t_libelle`) et explications (`t_explication`).
// Clé = code de cotisation (ou clé synthétique pour les variantes Fillon).
// Retourne None si la paire (clé, langue) n'est pas couverte → l'appelant
// retombe sur le texte français natif.
//
// Les explications dynamiques contiennent des placeholders nommés
// (`{pmss}`, `{annee}`, `{coeff}`…) substitués par l'appelant.

/// Libellé traduit d'une cotisation, ou None si non couvert.
pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "SS_MALADIE" => match lang {
            "en" => "Health, maternity, disability and death insurance",
            "de" => "Kranken-, Mutterschafts-, Invaliditäts- und Todesfallversicherung",
            "nl" => "Ziekte-, moederschaps-, invaliditeits- en overlijdensverzekering",
            "it" => "Assicurazione malattia, maternità, invalidità, morte",
            "es" => "Seguro de enfermedad, maternidad, invalidez y muerte",
            _ => return None,
        },
        "SS_VIEILLESSE_PLAF" => match lang {
            "en" => "Old-age insurance (capped)",
            "de" => "Altersversicherung (gedeckelt)",
            "nl" => "Ouderdomsverzekering (geplafonneerd)",
            "it" => "Assicurazione vecchiaia (con massimale)",
            "es" => "Seguro de vejez (con tope)",
            _ => return None,
        },
        "SS_VIEILLESSE_DEPLAF" => match lang {
            "en" => "Old-age insurance (uncapped)",
            "de" => "Altersversicherung (ungedeckelt)",
            "nl" => "Ouderdomsverzekering (zonder plafond)",
            "it" => "Assicurazione vecchiaia (senza massimale)",
            "es" => "Seguro de vejez (sin tope)",
            _ => return None,
        },
        "FAMILLE" => match lang {
            "en" => "Family allowances",
            "de" => "Familienbeihilfen",
            "nl" => "Kinderbijslag",
            "it" => "Assegni familiari",
            "es" => "Prestaciones familiares",
            _ => return None,
        },
        "AT_MP" => match lang {
            "en" => "Occupational accidents / occupational diseases",
            "de" => "Arbeitsunfälle / Berufskrankheiten",
            "nl" => "Arbeidsongevallen / beroepsziekten",
            "it" => "Infortuni sul lavoro / malattie professionali",
            "es" => "Accidentes de trabajo / enfermedades profesionales",
            _ => return None,
        },
        "CHOMAGE" => match lang {
            "en" => "Unemployment insurance",
            "de" => "Arbeitslosenversicherung",
            "nl" => "Werkloosheidsverzekering",
            "it" => "Assicurazione contro la disoccupazione",
            "es" => "Seguro de desempleo",
            _ => return None,
        },
        "CSG_DEDUCTIBLE" => match lang {
            "en" => "Deductible CSG",
            "de" => "Abziehbare CSG",
            "nl" => "Aftrekbare CSG",
            "it" => "CSG deducibile",
            "es" => "CSG deducible",
            _ => return None,
        },
        "CSG_NON_DEDUCTIBLE" => match lang {
            "en" => "Non-deductible CSG",
            "de" => "Nicht abziehbare CSG",
            "nl" => "Niet-aftrekbare CSG",
            "it" => "CSG non deducibile",
            "es" => "CSG no deducible",
            _ => return None,
        },
        // CRDS : acronyme conservé tel quel dans toutes les langues → pas d'entrée.
        "AGIRC_ARRCO_T1" => match lang {
            "en" => "AGIRC-ARRCO Band 1",
            "de" => "AGIRC-ARRCO Tranche 1",
            "nl" => "AGIRC-ARRCO Schijf 1",
            "it" => "AGIRC-ARRCO Fascia 1",
            "es" => "AGIRC-ARRCO Tramo 1",
            _ => return None,
        },
        "AGIRC_ARRCO_T2" => match lang {
            "en" => "AGIRC-ARRCO Band 2",
            "de" => "AGIRC-ARRCO Tranche 2",
            "nl" => "AGIRC-ARRCO Schijf 2",
            "it" => "AGIRC-ARRCO Fascia 2",
            "es" => "AGIRC-ARRCO Tramo 2",
            _ => return None,
        },
        "AGIRC_ARRCO_CEG_T1" => match lang {
            "en" => "General Equilibrium Contribution (T1)",
            "de" => "Allgemeiner Ausgleichsbeitrag (T1)",
            "nl" => "Algemene evenwichtsbijdrage (T1)",
            "it" => "Contributo di equilibrio generale (T1)",
            "es" => "Contribución de equilibrio general (T1)",
            _ => return None,
        },
        "PREVOYANCE_CADRE_MIN" => match lang {
            "en" => "Minimum executive death-and-disability cover (art. 7 CCN 1947)",
            "de" => "Mindestvorsorge für Führungskräfte (Art. 7 CCN 1947)",
            "nl" => "Minimale voorzorgsverzekering kaderleden (art. 7 CCN 1947)",
            "it" => "Previdenza minima quadri (art. 7 CCN 1947)",
            "es" => "Previsión mínima de ejecutivos (art. 7 CCN 1947)",
            _ => return None,
        },
        "ALSACE_MOSELLE_MALADIE" => match lang {
            "en" => "Supplementary health insurance Alsace-Moselle (local scheme)",
            "de" => "Ergänzende Krankenversicherung Elsass-Mosel (Lokalregime)",
            "nl" => "Aanvullende ziekteverzekering Elzas-Moezel (lokaal stelsel)",
            "it" => "Assicurazione malattia integrativa Alsazia-Mosella (regime locale)",
            "es" => "Seguro de enfermedad complementario Alsacia-Mosela (régimen local)",
            _ => return None,
        },
        "REDUCTION_FILLON" => match lang {
            "en" => "General reduction of employer contributions",
            "de" => "Allgemeine Senkung der Arbeitgeberbeiträge",
            "nl" => "Algemene vermindering van werkgeversbijdragen",
            "it" => "Riduzione generale dei contributi a carico del datore di lavoro",
            "es" => "Reducción general de las cotizaciones patronales",
            _ => return None,
        },
        "ESAT_AIDE_POSTE" => match lang {
            "en" => "Job support grant — ESAT (State)",
            "de" => "Arbeitsplatzhilfe — ESAT (Staat)",
            "nl" => "Werkpleksteun — ESAT (Staat)",
            "it" => "Aiuto al posto — ESAT (Stato)",
            "es" => "Ayuda al puesto — ESAT (Estado)",
            _ => return None,
        },
        "ESAT_COMPENSATION" => match lang {
            "en" => "State compensation of charges on the job support grant",
            "de" => "Staatlicher Ausgleich der Abgaben auf die Arbeitsplatzhilfe",
            "nl" => "Staatscompensatie van de lasten op de werkpleksteun",
            "it" => "Compensazione statale degli oneri sull'aiuto al posto",
            "es" => "Compensación estatal de las cargas sobre la ayuda al puesto",
            _ => return None,
        },
        "IDCC16_REPAS_UNIQUE" => match lang {
            "en" => "Single meal allowance",
            "de" => "Einzelmahlzeitzulage",
            "nl" => "Vergoeding enkele maaltijd",
            "it" => "Indennità di pasto unico",
            "es" => "Dieta de comida única",
            _ => return None,
        },
        "IDCC16_REPAS_UNIQUE_NUIT" => match lang {
            "en" => "Single night meal allowance",
            "de" => "Einzelmahlzeitzulage Nacht",
            "nl" => "Vergoeding enkele nachtmaaltijd",
            "it" => "Indennità di pasto unico notturno",
            "es" => "Dieta de comida única nocturna",
            _ => return None,
        },
        "IDCC16_INDEMNITE_SPECIALE" => match lang {
            "en" => "Special allowance (meal)",
            "de" => "Sonderzulage (Mahlzeit)",
            "nl" => "Bijzondere vergoeding (maaltijd)",
            "it" => "Indennità speciale (pasto)",
            "es" => "Dieta especial (comida)",
            _ => return None,
        },
        "IDCC16_CASSE_CROUTE" => match lang {
            "en" => "Snack allowance",
            "de" => "Imbisszulage",
            "nl" => "Vergoeding tussendoortje",
            "it" => "Indennità di spuntino",
            "es" => "Dieta de tentempié",
            _ => return None,
        },
        "AN_REPAS" => match lang {
            "en" => "Benefit in kind — meals",
            "de" => "Sachbezug — Mahlzeiten",
            "nl" => "Voordeel in natura — maaltijden",
            "it" => "Fringe benefit — pasti",
            "es" => "Retribución en especie — comidas",
            _ => return None,
        },
        "AN_LOGEMENT" => match lang {
            "en" => "Benefit in kind — housing",
            "de" => "Sachbezug — Wohnung",
            "nl" => "Voordeel in natura — huisvesting",
            "it" => "Fringe benefit — alloggio",
            "es" => "Retribución en especie — vivienda",
            _ => return None,
        },
        "AN_VEHICULE" => match lang {
            "en" => "Benefit in kind — vehicle",
            "de" => "Sachbezug — Fahrzeug",
            "nl" => "Voordeel in natura — voertuig",
            "it" => "Fringe benefit — veicolo",
            "es" => "Retribución en especie — vehículo",
            _ => return None,
        },
        "AN_NTIC" => match lang {
            "en" => "Benefit in kind — digital tools",
            "de" => "Sachbezug — digitale Geräte",
            "nl" => "Voordeel in natura — digitale middelen",
            "it" => "Fringe benefit — strumenti digitali",
            "es" => "Retribución en especie — herramientas digitales",
            _ => return None,
        },
        "AN_AUTRE" => match lang {
            "en" => "Benefit in kind — other",
            "de" => "Sachbezug — sonstige",
            "nl" => "Voordeel in natura — overige",
            "it" => "Fringe benefit — altro",
            "es" => "Retribución en especie — otra",
            _ => return None,
        },
        "AIDE_POSTE_EA" => match lang {
            "en" => "Employment support grant — adapted enterprise (State/ASP)",
            "de" => "Beschäftigungszuschuss — angepasstes Unternehmen (Staat/ASP)",
            "nl" => "Tewerkstellingssteun — aangepaste onderneming (Staat/ASP)",
            "it" => "Aiuto al posto di lavoro — impresa adattata (Stato/ASP)",
            "es" => "Ayuda al puesto — empresa adaptada (Estado/ASP)",
            _ => return None,
        },
        "REDUC_SAL_HS" => match lang {
            "en" => "Employee contribution reduction — overtime/extra hours",
            "de" => "Senkung der Arbeitnehmerbeiträge — Über-/Mehrstunden",
            "nl" => "Vermindering werknemersbijdragen — over-/meeruren",
            "it" => "Riduzione contributi del dipendente — ore supplementari/complementari",
            "es" => "Reducción de cotizaciones del trabajador — horas extra/complementarias",
            _ => return None,
        },
        "DFP_HS" => match lang {
            "en" => "Flat-rate employer deduction (overtime)",
            "de" => "Pauschaler Arbeitgeberabzug (Überstunden)",
            "nl" => "Forfaitaire werkgeversaftrek (overuren)",
            "it" => "Deduzione forfettaria datoriale (ore supplementari)",
            "es" => "Deducción a tanto alzado del empleador (horas extra)",
            _ => return None,
        },
        "FPT_CNRACL" => match lang {
            "en" => "CNRACL — Main pension (local civil service)",
            "de" => "CNRACL — Hauptrente (kommunaler öffentlicher Dienst)",
            "nl" => "CNRACL — Hoofdpensioen (lokale ambtenaren)",
            "it" => "CNRACL — Pensione principale (funzione pubblica territoriale)",
            "es" => "CNRACL — Pensión principal (función pública territorial)",
            _ => return None,
        },
        "PAYS_NON_COUVERT" => match lang {
            "en" => "Data unavailable for this year",
            "de" => "Keine Daten für dieses Jahr verfügbar",
            "nl" => "Geen gegevens beschikbaar voor dit jaar",
            "it" => "Dati non disponibili per quest'anno",
            "es" => "Datos no disponibles para este año",
            _ => return None,
        },
        _ => return None,
    })
}

/// Explication traduite d'une cotisation (ou gabarit pour les dynamiques),
/// ou None si non couvert. `key` peut être un code de cotisation ou une clé
/// synthétique (variantes Fillon).
pub fn t_explication(key: &str, lang: &str) -> Option<&'static str> {
    Some(match key {
        "SS_MALADIE" => match lang {
            "en" => "The employee health contribution was abolished on 1 January 2018 \
                (LFSS 2018). In return, the CSG was raised by 1.7 points. \
                This shift aimed to increase net pay without raising the employer's cost. \
                The employer share funds the health branch of the national health insurance.",
            "de" => "Der Arbeitnehmerbeitrag zur Krankenversicherung wurde zum 1. Januar 2018 \
                abgeschafft (LFSS 2018). Im Gegenzug wurde die CSG um 1,7 Punkte erhöht. \
                Diese Umstellung sollte das Nettogehalt erhöhen, ohne die Arbeitgeberkosten zu steigern. \
                Der Arbeitgeberanteil finanziert den Krankenzweig der Krankenkasse.",
            "nl" => "De werknemersbijdrage voor ziekte werd op 1 januari 2018 afgeschaft \
                (LFSS 2018). Als tegenprestatie werd de CSG met 1,7 punt verhoogd. \
                Deze verschuiving wilde het nettoloon verhogen zonder de werkgeverskost te verhogen. \
                Het werkgeversaandeel financiert de ziektetak van de ziekteverzekering.",
            "it" => "Il contributo malattia a carico del dipendente è stato soppresso il 1° gennaio 2018 \
                (LFSS 2018). In cambio, la CSG è stata aumentata di 1,7 punti. \
                Questo passaggio mirava ad aumentare lo stipendio netto senza accrescere il costo del datore di lavoro. \
                La quota a carico del datore finanzia il ramo malattia dell'assicurazione sanitaria.",
            "es" => "La cotización de enfermedad a cargo del trabajador se suprimió el 1 de enero de 2018 \
                (LFSS 2018). A cambio, la CSG aumentó 1,7 puntos. \
                Este cambio buscaba aumentar el salario neto sin incrementar el coste del empleador. \
                La parte patronal financia la rama de enfermedad del seguro de salud.",
            _ => return None,
        },
        // Dynamique — placeholders {pmss} {annee}
        "SS_VIEILLESSE_PLAF" => match lang {
            "en" => "This pension contribution is capped at the Monthly Social Security Ceiling \
                (PMSS = {pmss} € in {annee}). Above it, only the uncapped contribution applies. \
                The French pay-as-you-go system, created in 1945 by GPRF ordinance, \
                guarantees a pension calculated on the best 25 years (private-sector employees).",
            "de" => "Dieser Rentenbeitrag ist auf die monatliche Beitragsbemessungsgrenze der \
                Sozialversicherung begrenzt (PMSS = {pmss} € in {annee}). Darüber hinaus gilt nur \
                der ungedeckelte Beitrag. Das französische Umlagesystem, 1945 durch Verordnung der GPRF \
                geschaffen, garantiert eine Rente, die auf den besten 25 Jahren berechnet wird \
                (Beschäftigte der Privatwirtschaft).",
            "nl" => "Deze pensioenbijdrage is beperkt tot het maandelijkse plafond van de sociale \
                zekerheid (PMSS = {pmss} € in {annee}). Daarboven geldt alleen de bijdrage zonder plafond. \
                Het Franse omslagstelsel, in 1945 opgericht bij ordonnantie van de GPRF, waarborgt een \
                pensioen berekend op de beste 25 jaren (werknemers uit de privésector).",
            "it" => "Questo contributo pensionistico è limitato al massimale mensile della previdenza \
                sociale (PMSS = {pmss} € nel {annee}). Oltre tale soglia si applica solo il contributo \
                senza massimale. Il sistema francese a ripartizione, creato nel 1945 con ordinanza del GPRF, \
                garantisce una pensione calcolata sui 25 anni migliori (lavoratori del settore privato).",
            "es" => "Esta cotización de jubilación está limitada al tope mensual de la Seguridad Social \
                (PMSS = {pmss} € en {annee}). Por encima, solo se aplica la cotización sin tope. \
                El sistema francés de reparto, creado en 1945 por ordenanza del GPRF, garantiza una \
                pensión calculada sobre los 25 mejores años (asalariados del sector privado).",
            _ => return None,
        },
        "SS_VIEILLESSE_DEPLAF" => match lang {
            "en" => "Applies to the entire gross salary, with no ceiling. \
                A solidarity contribution: high earners contribute proportionally more \
                to fund a system whose pensions are capped. \
                Principle of universality of Social Security (1946 Preamble).",
            "de" => "Gilt für das gesamte Bruttogehalt, ohne Obergrenze. \
                Solidaritätsbeitrag: hohe Gehälter tragen anteilig mehr bei, \
                um ein System zu finanzieren, dessen Renten gedeckelt sind. \
                Grundsatz der Universalität der Sozialversicherung (Präambel von 1946).",
            "nl" => "Geldt op het volledige brutoloon, zonder plafond. \
                Solidariteitsbijdrage: hoge lonen dragen evenredig meer bij \
                om een stelsel te financieren waarvan de pensioenen geplafonneerd zijn. \
                Beginsel van universaliteit van de sociale zekerheid (Preambule van 1946).",
            "it" => "Si applica sull'intero stipendio lordo, senza massimale. \
                Contributo solidale: i redditi alti contribuiscono proporzionalmente di più \
                per finanziare un sistema le cui pensioni sono soggette a massimale. \
                Principio di universalità della previdenza sociale (Preambolo del 1946).",
            "es" => "Se aplica sobre la totalidad del salario bruto, sin tope. \
                Cotización solidaria: los salarios altos contribuyen proporcionalmente más \
                para financiar un sistema cuyas pensiones tienen tope. \
                Principio de universalidad de la Seguridad Social (Preámbulo de 1946).",
            _ => return None,
        },
        "FAMILLE" => match lang {
            "en" => "Funds family benefits (allowances, nurseries, childcare support). \
                Reduced rate of 3.45% for salaries ≤ 3.5 SMIC (full rate: 5.25%). \
                A French pro-birth policy dating from the interwar period, institutionalised in 1945.",
            "de" => "Finanziert Familienleistungen (Beihilfen, Kinderkrippen, Betreuungshilfe). \
                Ermäßigter Satz von 3,45 % für Gehälter ≤ 3,5 SMIC (voller Satz: 5,25 %). \
                Französische Geburtenförderpolitik aus der Zwischenkriegszeit, 1945 institutionalisiert.",
            "nl" => "Financiert gezinsuitkeringen (toelagen, kinderdagverblijven, opvanghulp). \
                Verlaagd tarief van 3,45% voor lonen ≤ 3,5 SMIC (vol tarief: 5,25%). \
                Frans geboortebevorderingsbeleid uit het interbellum, geïnstitutionaliseerd in 1945.",
            "it" => "Finanzia le prestazioni familiari (assegni, asili nido, sostegno alla custodia). \
                Aliquota ridotta del 3,45% per le retribuzioni ≤ 3,5 SMIC (aliquota piena: 5,25%). \
                Politica natalista francese risalente al periodo tra le due guerre, istituzionalizzata nel 1945.",
            "es" => "Financia las prestaciones familiares (subsidios, guarderías, ayuda al cuidado). \
                Tipo reducido del 3,45% para salarios ≤ 3,5 SMIC (tipo pleno: 5,25%). \
                Política natalista francesa del período de entreguerras, institucionalizada en 1945.",
            _ => return None,
        },
        "AT_MP" => match lang {
            "en" => "Rate set by the CARSAT according to the company's risk code \
                (sector of activity, past claims). Entirely borne by the employer: \
                principle of employer liability introduced by the Act of 9 April 1898, \
                the first social law recognising the employer's liability without proven fault.",
            "de" => "Satz von der CARSAT nach dem Risikocode des Unternehmens festgelegt \
                (Branche, frühere Schadensfälle). Vollständig vom Arbeitgeber getragen: \
                Grundsatz der Arbeitgeberhaftung, eingeführt durch das Gesetz vom 9. April 1898, \
                das erste Sozialgesetz, das die Haftung des Arbeitgebers ohne nachgewiesenes Verschulden anerkennt.",
            "nl" => "Tarief vastgesteld door de CARSAT volgens de risicocode van de onderneming \
                (sector, schadeverleden). Volledig ten laste van de werkgever: \
                beginsel van werkgeversaansprakelijkheid ingevoerd door de wet van 9 april 1898, \
                de eerste sociale wet die de aansprakelijkheid van de werkgever zonder bewezen fout erkent.",
            "it" => "Aliquota fissata dalla CARSAT in base al codice di rischio dell'impresa \
                (settore di attività, sinistrosità passata). Interamente a carico del datore di lavoro: \
                principio di responsabilità datoriale introdotto dalla legge del 9 aprile 1898, \
                prima legge sociale a riconoscere la responsabilità del datore senza colpa provata.",
            "es" => "Tipo fijado por la CARSAT según el código de riesgo de la empresa \
                (sector de actividad, siniestralidad pasada). Íntegramente a cargo del empleador: \
                principio de responsabilidad patronal instaurado por la ley del 9 de abril de 1898, \
                primera ley social que reconoce la responsabilidad del empleador sin culpa probada.",
            _ => return None,
        },
        "CHOMAGE" => match lang {
            "en" => "Since 2018, the employee unemployment contribution has been abolished \
                and offset by the CSG increase. Only the employer share remains, \
                capped at 4 PMSS. Unemployment insurance (UNEDIC) has been managed jointly \
                by unions and employers since 1958.",
            "de" => "Seit 2018 ist der Arbeitnehmerbeitrag zur Arbeitslosenversicherung abgeschafft \
                und durch die CSG-Erhöhung ausgeglichen. Nur der Arbeitgeberanteil bleibt bestehen, \
                begrenzt auf 4 PMSS. Die Arbeitslosenversicherung (UNEDIC) wird seit 1958 paritätisch verwaltet.",
            "nl" => "Sinds 2018 is de werknemersbijdrage voor werkloosheid afgeschaft \
                en gecompenseerd door de CSG-verhoging. Alleen het werkgeversaandeel blijft bestaan, \
                geplafonneerd op 4 PMSS. De werkloosheidsverzekering (UNEDIC) wordt sinds 1958 paritair beheerd.",
            "it" => "Dal 2018 il contributo disoccupazione a carico del dipendente è stato soppresso \
                e compensato dall'aumento della CSG. Resta solo la quota a carico del datore, \
                con massimale a 4 PMSS. L'assicurazione contro la disoccupazione (UNEDIC) è gestita \
                in modo paritetico dal 1958.",
            "es" => "Desde 2018, la cotización de desempleo a cargo del trabajador se ha suprimido \
                y compensado con la subida de la CSG. Solo subsiste la parte patronal, \
                con tope de 4 PMSS. El seguro de desempleo (UNEDIC) se gestiona de forma paritaria desde 1958.",
            _ => return None,
        },
        "CSG_DEDUCTIBLE" => match lang {
            "en" => "The CSG (General Social Contribution) was created in 1991 \
                by Michel Rocard to diversify the funding of Social Security \
                beyond salaried work (capital income included). The deductible portion \
                is subtracted from income taxable for income tax. \
                The base is 98.25% of gross (a 1.75% allowance for professional expenses).",
            "de" => "Die CSG (Allgemeiner Sozialbeitrag) wurde 1991 von Michel Rocard geschaffen, \
                um die Finanzierung der Sozialversicherung über die Erwerbsarbeit hinaus zu \
                diversifizieren (einschließlich Kapitalerträge). Der abziehbare Teil wird vom \
                einkommensteuerpflichtigen Einkommen abgezogen. \
                Die Bemessungsgrundlage beträgt 98,25 % des Bruttos (1,75 % Pauschale für Werbungskosten).",
            "nl" => "De CSG (Algemene Sociale Bijdrage) werd in 1991 ingevoerd door Michel Rocard \
                om de financiering van de sociale zekerheid te diversifiëren voorbij de loonarbeid \
                (kapitaalinkomsten inbegrepen). Het aftrekbare deel wordt afgetrokken van het belastbaar inkomen. \
                De grondslag bedraagt 98,25% van het bruto (1,75% aftrek voor beroepskosten).",
            "it" => "La CSG (Contributo Sociale Generalizzato) è stata creata nel 1991 da Michel Rocard \
                per diversificare il finanziamento della previdenza sociale oltre il lavoro dipendente \
                (redditi da capitale inclusi). La parte deducibile è sottratta dal reddito imponibile IRPEF. \
                La base è il 98,25% del lordo (abbattimento dell'1,75% per spese professionali).",
            "es" => "La CSG (Contribución Social Generalizada) fue creada en 1991 por Michel Rocard \
                para diversificar la financiación de la Seguridad Social más allá del trabajo asalariado \
                (rentas del capital incluidas). La parte deducible se resta de la renta sujeta al IRPF. \
                La base es el 98,25% del bruto (reducción del 1,75% por gastos profesionales).",
            _ => return None,
        },
        "CSG_NON_DEDUCTIBLE" => match lang {
            "en" => "Fraction of CSG not deductible from taxable income: it amounts to \
                a pure tax on the salary. Raised by 1.7 points in 2018 (LFSS 2018) \
                in exchange for the abolition of the employee health and unemployment contributions.",
            "de" => "Anteil der CSG, der nicht vom steuerpflichtigen Einkommen abziehbar ist: \
                er stellt eine reine Steuer auf das Gehalt dar. 2018 um 1,7 Punkte erhöht (LFSS 2018) \
                im Gegenzug zur Abschaffung der Arbeitnehmerbeiträge zur Kranken- und Arbeitslosenversicherung.",
            "nl" => "Fractie van de CSG die niet aftrekbaar is van het belastbaar inkomen: \
                het is een zuivere belasting op het loon. In 2018 met 1,7 punt verhoogd (LFSS 2018) \
                in ruil voor de afschaffing van de werknemersbijdragen ziekte en werkloosheid.",
            "it" => "Frazione di CSG non deducibile dal reddito imponibile: costituisce \
                un'imposta secca sullo stipendio. Aumentata di 1,7 punti nel 2018 (LFSS 2018) \
                in cambio della soppressione dei contributi malattia e disoccupazione a carico del dipendente.",
            "es" => "Fracción de la CSG no deducible de la renta imponible: constituye \
                un impuesto puro sobre el salario. Aumentada 1,7 puntos en 2018 (LFSS 2018) \
                a cambio de la supresión de las cotizaciones de enfermedad y desempleo del trabajador.",
            _ => return None,
        },
        "CRDS" => match lang {
            "en" => "The CRDS (Contribution to the Repayment of the Social Debt, 0.5%) \
                was created in 1996 by Alain Juppé to repay the Social Security debt \
                through the CADES. Meant to last 13 years, it still exists. \
                Not deductible from income tax.",
            "de" => "Die CRDS (Beitrag zur Tilgung der Sozialschuld, 0,5 %) wurde 1996 \
                von Alain Juppé geschaffen, um die Schulden der Sozialversicherung über die CADES \
                zu tilgen. Eigentlich auf 13 Jahre angelegt, besteht sie bis heute. \
                Nicht von der Einkommensteuer abziehbar.",
            "nl" => "De CRDS (Bijdrage tot terugbetaling van de sociale schuld, 0,5%) \
                werd in 1996 ingevoerd door Alain Juppé om de schuld van de sociale zekerheid \
                via de CADES af te lossen. Bedoeld voor 13 jaar, bestaat ze nog steeds. \
                Niet aftrekbaar van de inkomstenbelasting.",
            "it" => "La CRDS (Contributo al Rimborso del Debito Sociale, 0,5%) è stata creata \
                nel 1996 da Alain Juppé per rimborsare il debito della previdenza sociale \
                tramite la CADES. Prevista per durare 13 anni, esiste tuttora. \
                Non deducibile dall'IRPEF.",
            "es" => "La CRDS (Contribución al Reembolso de la Deuda Social, 0,5%) fue creada \
                en 1996 por Alain Juppé para reembolsar la deuda de la Seguridad Social \
                a través de la CADES. Prevista para durar 13 años, todavía existe. \
                No deducible del IRPF.",
            _ => return None,
        },
        // Dynamique — placeholder {pmss}
        "AGIRC_ARRCO_T1" => match lang {
            "en" => "AGIRC-ARRCO: 2019 merger of the executive (AGIRC, 1947) and \
                non-executive (ARRCO, 1961) schemes. Points-based system. \
                Band 1 = salary up to the PMSS ({pmss} €).",
            "de" => "AGIRC-ARRCO: Fusion 2019 der Systeme für Führungskräfte (AGIRC, 1947) \
                und Nicht-Führungskräfte (ARRCO, 1961). Punktesystem. \
                Tranche 1 = Gehalt bis zur PMSS ({pmss} €).",
            "nl" => "AGIRC-ARRCO: fusie in 2019 van de stelsels voor kaderleden (AGIRC, 1947) \
                en niet-kaderleden (ARRCO, 1961). Puntensysteem. \
                Schijf 1 = loon tot de PMSS ({pmss} €).",
            "it" => "AGIRC-ARRCO: fusione nel 2019 dei regimi quadri (AGIRC, 1947) \
                e non quadri (ARRCO, 1961). Sistema a punti. \
                Fascia 1 = retribuzione fino al PMSS ({pmss} €).",
            "es" => "AGIRC-ARRCO: fusión en 2019 de los regímenes de ejecutivos (AGIRC, 1947) \
                y no ejecutivos (ARRCO, 1961). Sistema por puntos. \
                Tramo 1 = salario hasta el PMSS ({pmss} €).",
            _ => return None,
        },
        "AGIRC_ARRCO_T2" => match lang {
            "en" => "Band 2: portion of salary between 1 and 8 PMSS. \
                Higher rate as it targets mid-to-high salaries. \
                Managed jointly (unions and employers).",
            "de" => "Tranche 2: Gehaltsanteil zwischen 1 und 8 PMSS. \
                Höherer Satz, da auf mittlere bis hohe Gehälter ausgerichtet. \
                Paritätisch verwaltet (Gewerkschaften und Arbeitgeber).",
            "nl" => "Schijf 2: gedeelte van het loon tussen 1 en 8 PMSS. \
                Hoger tarief omdat het gericht is op midden- tot hoge lonen. \
                Paritair beheerd (vakbonden en werkgevers).",
            "it" => "Fascia 2: quota di retribuzione tra 1 e 8 PMSS. \
                Aliquota più elevata perché mira alle retribuzioni medio-alte. \
                Gestita in modo paritetico (sindacati e datori di lavoro).",
            "es" => "Tramo 2: fracción del salario entre 1 y 8 PMSS. \
                Tipo más elevado porque se dirige a los salarios medios y altos. \
                Gestionado de forma paritaria (sindicatos y patronal).",
            _ => return None,
        },
        "AGIRC_ARRCO_CEG_T1" => match lang {
            "en" => "Contribution that does not generate points, intended for the financial \
                balance of the AGIRC-ARRCO scheme. Created during the 2019 merger.",
            "de" => "Beitrag, der keine Punkte generiert und dem finanziellen Gleichgewicht \
                des AGIRC-ARRCO-Systems dient. Bei der Fusion 2019 geschaffen.",
            "nl" => "Bijdrage die geen punten genereert, bedoeld voor het financiële evenwicht \
                van het AGIRC-ARRCO-stelsel. Ingevoerd bij de fusie van 2019.",
            "it" => "Contributo che non genera punti, destinato all'equilibrio finanziario \
                del regime AGIRC-ARRCO. Creato in occasione della fusione del 2019.",
            "es" => "Contribución que no genera puntos, destinada al equilibrio financiero \
                del régimen AGIRC-ARRCO. Creada con motivo de la fusión de 2019.",
            _ => return None,
        },
        "PREVOYANCE_CADRE_MIN" => match lang {
            "en" => "The National Collective Agreement for Executives (14/03/1947) \
                requires employers to pay a minimum contribution of 1.5% on band A \
                to fund executives' death cover. An employer obligation unique \
                in Europe, the outcome of post-war bargaining.",
            "de" => "Der Nationale Tarifvertrag der Führungskräfte (14.03.1947) \
                verpflichtet Arbeitgeber zu einem Mindestbeitrag von 1,5 % auf Tranche A \
                zur Finanzierung der Todesfallvorsorge der Führungskräfte. Eine in Europa \
                einzigartige Arbeitgeberpflicht, Ergebnis der Nachkriegsverhandlungen.",
            "nl" => "De Nationale Collectieve Overeenkomst voor Kaderleden (14/03/1947) \
                verplicht werkgevers tot een minimumbijdrage van 1,5% op schijf A \
                om de overlijdensdekking van kaderleden te financieren. Een in Europa \
                unieke werkgeversverplichting, resultaat van naoorlogse onderhandelingen.",
            "it" => "Il Contratto Collettivo Nazionale dei Quadri (14/03/1947) \
                impone ai datori di lavoro un contributo minimo dell'1,5% sulla fascia A \
                per finanziare la copertura morte dei quadri. Obbligo datoriale unico \
                in Europa, frutto della contrattazione del dopoguerra.",
            "es" => "El Convenio Colectivo Nacional de Ejecutivos (14/03/1947) \
                impone a los empleadores una cotización mínima del 1,5% sobre el tramo A \
                para financiar la cobertura de fallecimiento de los ejecutivos. Obligación patronal única \
                en Europa, resultado de la negociación de posguerra.",
            _ => return None,
        },
        "ALSACE_MOSELLE_MALADIE" => match lang {
            "en" => "The Alsace-Moselle local scheme (local law) provides compulsory \
                supplementary health cover to employees of the Bas-Rhin (67), Haut-Rhin (68) \
                and Moselle (57) departments. This contribution, employee-only, is levied \
                on top of the general scheme. It funds reimbursement at 90% (vs. 70% in the \
                general scheme), with no co-payment for hospital stays. This scheme stems from \
                Bismarckian law in force since 1871, retained when Alsace-Lorraine returned to \
                France in 1919 (Act of 1 June 1924). Rate set each year by the local scheme's board: \
                1.50% from 01/01/2012 to 31/03/2022, 1.30% since 01/04/2022 (unchanged in 2026).",
            "de" => "Das Lokalregime Elsass-Mosel (Lokalrecht) bietet den Beschäftigten der \
                Departements Bas-Rhin (67), Haut-Rhin (68) und Moselle (57) eine obligatorische \
                ergänzende Krankenversicherung. Dieser nur vom Arbeitnehmer getragene Beitrag wird \
                zusätzlich zum allgemeinen System erhoben. Er finanziert eine Erstattung von 90 % \
                (gegenüber 70 % im allgemeinen System), ohne Selbstbeteiligung bei Krankenhausaufenthalten. \
                Dieses Regime geht auf das seit 1871 geltende bismarcksche Recht zurück, beibehalten bei \
                der Rückkehr Elsass-Lothringens zu Frankreich 1919 (Gesetz vom 1. Juni 1924). \
                Satz jährlich vom Verwaltungsrat des Lokalregimes festgelegt: \
                1,50 % vom 01.01.2012 bis 31.03.2022, 1,30 % seit 01.04.2022 (2026 unverändert).",
            "nl" => "Het lokale stelsel van de Elzas-Moezel (lokaal recht) biedt een verplichte \
                aanvullende ziekteverzekering aan werknemers van de departementen Bas-Rhin (67), \
                Haut-Rhin (68) en Moezel (57). Deze uitsluitend door de werknemer gedragen bijdrage \
                wordt bovenop het algemene stelsel geheven. Ze financiert een terugbetaling van 90% \
                (tegenover 70% in het algemene stelsel), zonder remgeld voor ziekenhuisopnames. \
                Dit stelsel stamt uit het sinds 1871 geldende bismarckiaanse recht, behouden bij de \
                terugkeer van Elzas-Lotharingen naar Frankrijk in 1919 (wet van 1 juni 1924). \
                Tarief jaarlijks vastgesteld door de raad van bestuur van het lokale stelsel: \
                1,50% van 01/01/2012 tot 31/03/2022, 1,30% sinds 01/04/2022 (ongewijzigd in 2026).",
            "it" => "Il regime locale dell'Alsazia-Mosella (diritto locale) offre una copertura \
                malattia integrativa obbligatoria ai dipendenti dei dipartimenti del Bas-Rhin (67), \
                Haut-Rhin (68) e Mosella (57). Questo contributo, esclusivamente a carico del dipendente, \
                è prelevato in aggiunta al regime generale. Finanzia un rimborso al 90% (contro il 70% \
                del regime generale), senza ticket per i ricoveri. Questo regime deriva dal diritto \
                bismarckiano vigente dal 1871, mantenuto al ritorno dell'Alsazia-Lorena alla Francia \
                nel 1919 (legge del 1° giugno 1924). Aliquota fissata ogni anno dal consiglio \
                di amministrazione del regime locale: 1,50% dal 01/01/2012 al 31/03/2022, 1,30% \
                dal 01/04/2022 (invariata nel 2026).",
            "es" => "El régimen local de Alsacia-Mosela (derecho local) ofrece una cobertura \
                de enfermedad complementaria obligatoria a los trabajadores de los departamentos del \
                Bajo Rin (67), Alto Rin (68) y Mosela (57). Esta cotización, únicamente a cargo del \
                trabajador, se recauda además del régimen general. Financia un reembolso del 90% \
                (frente al 70% del régimen general), sin copago para las hospitalizaciones. Este régimen \
                procede del derecho bismarckiano vigente desde 1871, mantenido al volver Alsacia-Lorena \
                a Francia en 1919 (ley del 1 de junio de 1924). Tipo fijado cada año por el consejo \
                de administración del régimen local: 1,50% del 01/01/2012 al 31/03/2022, 1,30% \
                desde el 01/04/2022 (sin cambios en 2026).",
            _ => return None,
        },
        // ── Réduction Fillon — gabarits dynamiques ────────────────────────────
        // Placeholders : {tmin} {tdelta} {tmax} {p} {seuil} {smic} {brut}
        //                {inner_disp} {coeff} {montant} {seuil_eur} {etp_info}
        "REDUCTION_FILLON_PUISSANCE" => match lang {
            "en" => "[ Monthly calculation — CSS art. L241-13 ]\n\
                \n\
                Formula: C = Tmin + (Tdelta × D^P)\n\
                D = (1/2) × (threshold × monthly SMIC / gross salary − 1)\n\
                \n\
                Parameters: Tmin={tmin}  Tdelta={tdelta}  Tmax={tmax}  P={p}  Threshold={seuil}×SMIC\n\
                \n\
                D = (1/2) × ({seuil} × {smic} / {brut} − 1)\n\
                  = {inner_disp}\n\
                \n\
                C = {tmin} + ({tdelta} × {inner_disp}^{p})\n\
                  = {coeff}\n\
                \n\
                ── Monthly reduction ───────────────────────────────\n\
                Reduction = gross salary × C\n\
                          = {brut} × {coeff}\n\
                          = {montant} €\n\
                ────────────────────────────────────────────────────\n\
                \n\
                Vanishes at {seuil} × SMIC = {seuil_eur} €/month.{etp_info}\n\
                Fillon Act of 17/01/2003: relief on employer contributions for low wages.",
            "de" => "[ Monatliche Berechnung — CSS Art. L241-13 ]\n\
                \n\
                Formel: C = Tmin + (Tdelta × D^P)\n\
                D = (1/2) × (Schwelle × monatlicher SMIC / Bruttogehalt − 1)\n\
                \n\
                Parameter: Tmin={tmin}  Tdelta={tdelta}  Tmax={tmax}  P={p}  Schwelle={seuil}×SMIC\n\
                \n\
                D = (1/2) × ({seuil} × {smic} / {brut} − 1)\n\
                  = {inner_disp}\n\
                \n\
                C = {tmin} + ({tdelta} × {inner_disp}^{p})\n\
                  = {coeff}\n\
                \n\
                ── Monatliche Senkung ──────────────────────────────\n\
                Senkung = Bruttogehalt × C\n\
                        = {brut} × {coeff}\n\
                        = {montant} €\n\
                ────────────────────────────────────────────────────\n\
                \n\
                Entfällt bei {seuil} × SMIC = {seuil_eur} €/Monat.{etp_info}\n\
                Fillon-Gesetz vom 17.01.2003: Entlastung der Arbeitgeberbeiträge bei niedrigen Löhnen.",
            "nl" => "[ Maandelijkse berekening — CSS art. L241-13 ]\n\
                \n\
                Formule: C = Tmin + (Tdelta × D^P)\n\
                D = (1/2) × (drempel × maandelijkse SMIC / brutoloon − 1)\n\
                \n\
                Parameters: Tmin={tmin}  Tdelta={tdelta}  Tmax={tmax}  P={p}  Drempel={seuil}×SMIC\n\
                \n\
                D = (1/2) × ({seuil} × {smic} / {brut} − 1)\n\
                  = {inner_disp}\n\
                \n\
                C = {tmin} + ({tdelta} × {inner_disp}^{p})\n\
                  = {coeff}\n\
                \n\
                ── Maandelijkse vermindering ───────────────────────\n\
                Vermindering = brutoloon × C\n\
                             = {brut} × {coeff}\n\
                             = {montant} €\n\
                ────────────────────────────────────────────────────\n\
                \n\
                Vervalt bij {seuil} × SMIC = {seuil_eur} €/maand.{etp_info}\n\
                Fillon-wet van 17/01/2003: vermindering van werkgeverslasten op lage lonen.",
            "it" => "[ Calcolo mensile — CSS art. L241-13 ]\n\
                \n\
                Formula: C = Tmin + (Tdelta × D^P)\n\
                D = (1/2) × (soglia × SMIC mensile / retribuzione lorda − 1)\n\
                \n\
                Parametri: Tmin={tmin}  Tdelta={tdelta}  Tmax={tmax}  P={p}  Soglia={seuil}×SMIC\n\
                \n\
                D = (1/2) × ({seuil} × {smic} / {brut} − 1)\n\
                  = {inner_disp}\n\
                \n\
                C = {tmin} + ({tdelta} × {inner_disp}^{p})\n\
                  = {coeff}\n\
                \n\
                ── Riduzione mensile ───────────────────────────────\n\
                Riduzione = retribuzione lorda × C\n\
                          = {brut} × {coeff}\n\
                          = {montant} €\n\
                ────────────────────────────────────────────────────\n\
                \n\
                Si annulla a {seuil} × SMIC = {seuil_eur} €/mese.{etp_info}\n\
                Legge Fillon del 17/01/2003: alleggerimento degli oneri datoriali sui bassi salari.",
            "es" => "[ Cálculo mensual — CSS art. L241-13 ]\n\
                \n\
                Fórmula: C = Tmin + (Tdelta × D^P)\n\
                D = (1/2) × (umbral × SMIC mensual / salario bruto − 1)\n\
                \n\
                Parámetros: Tmin={tmin}  Tdelta={tdelta}  Tmax={tmax}  P={p}  Umbral={seuil}×SMIC\n\
                \n\
                D = (1/2) × ({seuil} × {smic} / {brut} − 1)\n\
                  = {inner_disp}\n\
                \n\
                C = {tmin} + ({tdelta} × {inner_disp}^{p})\n\
                  = {coeff}\n\
                \n\
                ── Reducción mensual ───────────────────────────────\n\
                Reducción = salario bruto × C\n\
                          = {brut} × {coeff}\n\
                          = {montant} €\n\
                ────────────────────────────────────────────────────\n\
                \n\
                Se anula en {seuil} × SMIC = {seuil_eur} €/mes.{etp_info}\n\
                Ley Fillon del 17/01/2003: alivio de las cargas patronales sobre los salarios bajos.",
            _ => return None,
        },
        "REDUCTION_FILLON_LINEAIRE" => match lang {
            "en" => "[ Monthly calculation — old linear formula 2015-2018 ]\n\
                \n\
                Formula: C = (Tmax / 0.6) × (threshold × SMIC / gross − 1)\n\
                  = ({tmax} / 0.6) × ({seuil} × {smic} / {brut} − 1)\n\
                  = {coeff}\n\
                \n\
                ── Monthly reduction ───────────────────────────────\n\
                Reduction = gross salary × C\n\
                          = {brut} × {coeff}\n\
                          = {montant} €\n\
                ────────────────────────────────────────────────────\n\
                \n\
                Vanishes at {seuil} × SMIC = {seuil_eur} €/month.{etp_info}",
            "de" => "[ Monatliche Berechnung — alte lineare Formel 2015-2018 ]\n\
                \n\
                Formel: C = (Tmax / 0,6) × (Schwelle × SMIC / Brutto − 1)\n\
                  = ({tmax} / 0,6) × ({seuil} × {smic} / {brut} − 1)\n\
                  = {coeff}\n\
                \n\
                ── Monatliche Senkung ──────────────────────────────\n\
                Senkung = Bruttogehalt × C\n\
                        = {brut} × {coeff}\n\
                        = {montant} €\n\
                ────────────────────────────────────────────────────\n\
                \n\
                Entfällt bei {seuil} × SMIC = {seuil_eur} €/Monat.{etp_info}",
            "nl" => "[ Maandelijkse berekening — oude lineaire formule 2015-2018 ]\n\
                \n\
                Formule: C = (Tmax / 0,6) × (drempel × SMIC / bruto − 1)\n\
                  = ({tmax} / 0,6) × ({seuil} × {smic} / {brut} − 1)\n\
                  = {coeff}\n\
                \n\
                ── Maandelijkse vermindering ───────────────────────\n\
                Vermindering = brutoloon × C\n\
                             = {brut} × {coeff}\n\
                             = {montant} €\n\
                ────────────────────────────────────────────────────\n\
                \n\
                Vervalt bij {seuil} × SMIC = {seuil_eur} €/maand.{etp_info}",
            "it" => "[ Calcolo mensile — vecchia formula lineare 2015-2018 ]\n\
                \n\
                Formula: C = (Tmax / 0,6) × (soglia × SMIC / lordo − 1)\n\
                  = ({tmax} / 0,6) × ({seuil} × {smic} / {brut} − 1)\n\
                  = {coeff}\n\
                \n\
                ── Riduzione mensile ───────────────────────────────\n\
                Riduzione = retribuzione lorda × C\n\
                          = {brut} × {coeff}\n\
                          = {montant} €\n\
                ────────────────────────────────────────────────────\n\
                \n\
                Si annulla a {seuil} × SMIC = {seuil_eur} €/mese.{etp_info}",
            "es" => "[ Cálculo mensual — antigua fórmula lineal 2015-2018 ]\n\
                \n\
                Fórmula: C = (Tmax / 0,6) × (umbral × SMIC / bruto − 1)\n\
                  = ({tmax} / 0,6) × ({seuil} × {smic} / {brut} − 1)\n\
                  = {coeff}\n\
                \n\
                ── Reducción mensual ───────────────────────────────\n\
                Reducción = salario bruto × C\n\
                          = {brut} × {coeff}\n\
                          = {montant} €\n\
                ────────────────────────────────────────────────────\n\
                \n\
                Se anula en {seuil} × SMIC = {seuil_eur} €/mes.{etp_info}",
            _ => return None,
        },
        // Fragment temps partiel — placeholders {etp} {smic}
        "REDUCTION_FILLON_ETP" => match lang {
            "en" => "\n⚠ Part-time {etp} % — prorated SMIC: {smic} € (§670 BOSS)",
            "de" => "\n⚠ Teilzeit {etp} % — anteiliger SMIC: {smic} € (§670 BOSS)",
            "nl" => "\n⚠ Deeltijds {etp} % — geprorateerde SMIC: {smic} € (§670 BOSS)",
            "it" => "\n⚠ Tempo parziale {etp} % — SMIC proporzionato: {smic} € (§670 BOSS)",
            "es" => "\n⚠ Tiempo parcial {etp} % — SMIC prorrateado: {smic} € (§670 BOSS)",
            _ => return None,
        },
        // Correction du SMIC en cas d'absence — placeholders {ratio} {smic}
        "REDUCTION_FILLON_ABSENCE" => match lang {
            "en" => "\n⚠ Absence: SMIC corrected pro rata to pay (× {ratio}) → {smic} € (CSS art. D241-7 IV)",
            "de" => "\n⚠ Abwesenheit: SMIC anteilig zum Entgelt korrigiert (× {ratio}) → {smic} € (CSS Art. D241-7 IV)",
            "nl" => "\n⚠ Afwezigheid: SMIC gecorrigeerd naar rato van het loon (× {ratio}) → {smic} € (CSS art. D241-7 IV)",
            "it" => "\n⚠ Assenza: SMIC corretto in proporzione alla retribuzione (× {ratio}) → {smic} € (CSS art. D241-7 IV)",
            "es" => "\n⚠ Ausencia: SMIC corregido a prorrata de la remuneración (× {ratio}) → {smic} € (CSS art. D241-7 IV)",
            _ => return None,
        },
        // Fragment plafond proratisé temps partiel — placeholders {etp} {pmss}
        "PMSS_ETP_NOTE" => match lang {
            "en" => "\n⚠ Part-time {etp} % — prorated SSC ceiling: {pmss} € (reduced ceiling, CSS art. L242-1)",
            "de" => "\n⚠ Teilzeit {etp} % — anteilige Beitragsbemessungsgrenze: {pmss} € (CSS Art. L242-1)",
            "nl" => "\n⚠ Deeltijds {etp} % — geprorateerd plafond: {pmss} € (verlaagd plafond, CSS art. L242-1)",
            "it" => "\n⚠ Tempo parziale {etp} % — massimale proporzionato: {pmss} € (massimale ridotto, CSS art. L242-1)",
            "es" => "\n⚠ Tiempo parcial {etp} % — tope prorrateado: {pmss} € (tope reducido, CSS art. L242-1)",
            _ => return None,
        },
        "ESAT_AIDE_POSTE" => match lang {
            "en" => "An ESAT worker receives a guaranteed pay of between {min} and {max} of the \
                minimum wage ({smic} € for this working time). It consists of a share funded by the \
                ESAT, at least 5 % of the minimum wage (here {part} €), and a job support grant funded \
                by the State, at most {amax} of the minimum wage (here {aide} €), which falls by 0.5 \
                point per point of ESAT share above 20 % of the minimum wage. The grant is paid to \
                the ESAT and must appear on the payslip: it does not change the worker's net pay, it \
                reduces the cost borne by the ESAT.",
            "de" => "Ein Beschäftigter eines ESAT erhält eine garantierte Vergütung zwischen {min} \
                und {max} des Mindestlohns ({smic} € für diese Arbeitszeit). Sie besteht aus einem \
                vom ESAT finanzierten Anteil, mindestens 5 % des Mindestlohns (hier {part} €), und \
                einer staatlichen Arbeitsplatzhilfe von höchstens {amax} des Mindestlohns (hier \
                {aide} €), die je Prozentpunkt ESAT-Anteil über 20 % um 0,5 Punkte sinkt. Die Hilfe \
                wird an das ESAT gezahlt und muss auf der Abrechnung stehen: Sie ändert nicht den \
                Nettolohn, sie senkt die Kosten des ESAT.",
            "nl" => "Een werker in een ESAT ontvangt een gegarandeerde vergoeding tussen {min} en \
                {max} van het minimumloon ({smic} € voor deze arbeidsduur). Die bestaat uit een door \
                het ESAT gefinancierd deel, minstens 5 % van het minimumloon (hier {part} €), en een \
                werkpleksteun van de Staat, hoogstens {amax} van het minimumloon (hier {aide} €), die \
                per punt ESAT-deel boven 20 % met 0,5 punt daalt. De steun wordt aan het ESAT betaald \
                en moet op de loonstrook staan: hij verandert het nettoloon niet, hij verlaagt de \
                kosten van het ESAT.",
            "it" => "Il lavoratore di un ESAT percepisce una retribuzione garantita tra {min} e {max} \
                del salario minimo ({smic} € per questo orario). Si compone di una quota a carico \
                dell'ESAT, almeno il 5 % del salario minimo (qui {part} €), e di un aiuto al posto \
                finanziato dallo Stato, al massimo {amax} del salario minimo (qui {aide} €), che cala \
                di 0,5 punti per ogni punto di quota ESAT oltre il 20 %. L'aiuto è versato all'ESAT e \
                deve figurare in busta paga: non cambia il netto del lavoratore, riduce il costo \
                sostenuto dall'ESAT.",
            "es" => "El trabajador de un ESAT percibe una remuneración garantizada de entre {min} y \
                {max} del salario mínimo ({smic} € para esta jornada). Se compone de una parte \
                financiada por el ESAT, al menos el 5 % del salario mínimo (aquí {part} €), y de una \
                ayuda al puesto financiada por el Estado, como máximo {amax} del salario mínimo (aquí \
                {aide} €), que baja 0,5 puntos por cada punto de parte ESAT por encima del 20 %. La \
                ayuda se abona al ESAT y debe figurar en la nómina: no cambia el neto del trabajador, \
                reduce el coste que soporta el ESAT.",
            _ => return None,
        },
        "ESAT_COMPENSATION" => match lang {
            "en" => "The State reimburses the ESAT for all compulsory employer contributions due on \
                the share of the guaranteed pay equal to the job support grant: health, old-age, \
                family allowances, work accidents and supplementary pension. Here: {aide} € of grant \
                × {taux} of employer rates = {comp} €. FNAL, the mobility levy and occupational \
                health remain borne by the ESAT. No unemployment or AGS contribution and no general \
                reduction: an ESAT worker has no employment contract.",
            "de" => "Der Staat erstattet dem ESAT alle Pflichtbeiträge des Arbeitgebers auf den Teil \
                der garantierten Vergütung, der der Arbeitsplatzhilfe entspricht: Kranken-, Renten-, \
                Familien-, Unfall- und Zusatzrentenversicherung. Hier: {aide} € Hilfe × {taux} \
                Arbeitgebersätze = {comp} €. FNAL, Mobilitätsabgabe und Arbeitsmedizin trägt das \
                ESAT selbst. Kein Beitrag zur Arbeitslosenversicherung oder AGS und keine allgemeine \
                Senkung: ein ESAT-Beschäftigter hat keinen Arbeitsvertrag.",
            "nl" => "De Staat vergoedt het ESAT alle verplichte werkgeversbijdragen op het deel van \
                de gegarandeerde vergoeding dat gelijk is aan de werkpleksteun: ziekte, ouderdom, \
                gezinsbijslag, arbeidsongevallen en aanvullend pensioen. Hier: {aide} € steun × \
                {taux} werkgeverstarieven = {comp} €. FNAL, mobiliteitsheffing en arbeidsgeneeskunde \
                blijven ten laste van het ESAT. Geen werkloosheids- of AGS-bijdrage en geen algemene \
                vermindering: een ESAT-werker heeft geen arbeidscontract.",
            "it" => "Lo Stato rimborsa all'ESAT tutti i contributi datoriali obbligatori dovuti sulla \
                quota della retribuzione garantita pari all'aiuto al posto: malattia, vecchiaia, \
                assegni familiari, infortuni sul lavoro e previdenza complementare. Qui: {aide} € di \
                aiuto × {taux} di aliquote datoriali = {comp} €. FNAL, contributo mobilità e medicina \
                del lavoro restano a carico dell'ESAT. Nessun contributo di disoccupazione o AGS e \
                nessuna riduzione generale: il lavoratore di un ESAT non ha un contratto di lavoro.",
            "es" => "El Estado reembolsa al ESAT todas las cotizaciones patronales obligatorias \
                debidas sobre la parte de la remuneración garantizada igual a la ayuda al puesto: \
                enfermedad, vejez, prestaciones familiares, accidentes de trabajo y pensión \
                complementaria. Aquí: {aide} € de ayuda × {taux} de tipos patronales = {comp} €. El \
                FNAL, la contribución de movilidad y la medicina del trabajo quedan a cargo del ESAT. \
                Sin cotización por desempleo ni AGS y sin reducción general: el trabajador de un ESAT \
                no tiene contrato de trabajo.",
            _ => return None,
        },
        "IDCC16_FRAIS" => match lang {
            "en" => "Flat-rate allowance of the road transport collective agreement (IDCC 0016), \
                due for: {condition} ({article} of the protocol of 30/04/1974). {n} × {u} € = \
                {montant} €. It is a reimbursement of business expenses, not a wage: excluded from \
                contributions and CSG within the flat-rate limits of the order of 20/12/2002, exempt \
                from income tax (CGI art. 81, 1°). It is paid net, after the net pay of the salary, \
                without affecting taxable net pay.",
            "de" => "Pauschale Zulage des Tarifvertrags Straßentransport (IDCC 0016), geschuldet für: \
                {condition} ({article} des Protokolls vom 30.04.1974). {n} × {u} € = {montant} €. \
                Es ist eine Erstattung beruflicher Kosten, kein Lohn: beitrags- und CSG-frei im \
                Rahmen der Pauschalen des Erlasses vom 20.12.2002, einkommensteuerfrei (CGI Art. 81, \
                1°). Sie wird netto nach dem Nettolohn gezahlt, ohne das steuerpflichtige Netto zu \
                berühren.",
            "nl" => "Forfaitaire vergoeding uit de cao wegvervoer (IDCC 0016), verschuldigd voor: \
                {condition} ({article} van het protocol van 30/04/1974). {n} × {u} € = {montant} €. \
                Het is een terugbetaling van beroepskosten, geen loon: vrij van bijdragen en CSG \
                binnen de forfaits van het besluit van 20/12/2002, vrij van inkomstenbelasting (CGI \
                art. 81, 1°). Ze wordt netto betaald, na het nettoloon, zonder het belastbare netto \
                te raken.",
            "it" => "Indennità forfettaria del contratto collettivo dei trasporti su strada (IDCC 0016), \
                dovuta per: {condition} ({article} del protocollo del 30/04/1974). {n} × {u} € = \
                {montant} €. È un rimborso di spese professionali, non una retribuzione: esente da \
                contributi e CSG entro i forfait del decreto del 20/12/2002, esente da imposta sul \
                reddito (CGI art. 81, 1°). È versata al netto, dopo il netto in busta, senza toccare \
                il netto imponibile.",
            "es" => "Dieta a tanto alzado del convenio colectivo del transporte por carretera (IDCC \
                0016), debida por: {condition} ({article} del protocolo del 30/04/1974). {n} × {u} € = \
                {montant} €. Es un reembolso de gastos profesionales, no un salario: exento de \
                cotizaciones y CSG dentro de los límites de la orden del 20/12/2002, exento del \
                impuesto sobre la renta (CGI art. 81, 1°). Se abona en neto, tras el neto del \
                salario, sin afectar al neto imponible.",
            _ => return None,
        },
        "IDCC16_FRAIS_SANS_BAREME" => match lang {
            "en" => "Flat-rate allowance of the road transport collective agreement (IDCC 0016), \
                due for: {condition} ({article} of the protocol of 30/04/1974). No scale is \
                integrated for this date: the simulator covers the amounts in force since \
                01/12/2022. Amount left at 0 rather than invented.",
            "de" => "Pauschale Zulage des Tarifvertrags Straßentransport (IDCC 0016), geschuldet für: \
                {condition} ({article} des Protokolls vom 30.04.1974). Für dieses Datum ist kein \
                Tarif hinterlegt: der Simulator deckt die seit dem 01.12.2022 geltenden Beträge ab. \
                Betrag bei 0 belassen statt erfunden.",
            "nl" => "Forfaitaire vergoeding uit de cao wegvervoer (IDCC 0016), verschuldigd voor: \
                {condition} ({article} van het protocol van 30/04/1974). Voor deze datum is geen \
                barema opgenomen: de simulator dekt de bedragen sinds 01/12/2022. Bedrag op 0 \
                gelaten in plaats van verzonnen.",
            "it" => "Indennità forfettaria del contratto collettivo dei trasporti su strada (IDCC 0016), \
                dovuta per: {condition} ({article} del protocollo del 30/04/1974). Nessuna tabella \
                integrata per questa data: il simulatore copre gli importi in vigore dal 01/12/2022. \
                Importo lasciato a 0 anziché inventato.",
            "es" => "Dieta a tanto alzado del convenio colectivo del transporte por carretera (IDCC \
                0016), debida por: {condition} ({article} del protocolo del 30/04/1974). No hay \
                baremo integrado para esta fecha: el simulador cubre los importes vigentes desde el \
                01/12/2022. Importe dejado en 0 en lugar de inventado.",
            _ => return None,
        },
        "IDCC16_REPAS_UNIQUE_COND" => match lang {
            "en" => "travel within the trucking zone around Paris",
            "de" => "Fahrten in der Rollfuhrzone um Paris",
            "nl" => "verplaatsingen in de vrachtzone rond Parijs",
            "it" => "spostamenti nella zona di trasporto intorno a Parigi",
            "es" => "desplazamientos en la zona de acarreo alrededor de París",
            _ => return None,
        },
        "IDCC16_REPAS_UNIQUE_NUIT_COND" => match lang {
            "en" => "a shift with at least 4 hours of actual work between 10 pm and 7 am",
            "de" => "ein Dienst mit mindestens 4 Stunden effektiver Arbeit zwischen 22 und 7 Uhr",
            "nl" => "een dienst met minstens 4 uur effectief werk tussen 22 en 7 uur",
            "it" => "un servizio con almeno 4 ore di lavoro effettivo tra le 22 e le 7",
            "es" => "un servicio con al menos 4 horas de trabajo efectivo entre las 22 h y las 7 h",
            _ => return None,
        },
        "IDCC16_INDEMNITE_SPECIALE_COND" => match lang {
            "en" => "a working span fully covering 11 am-2:30 pm or 6:30-10 pm without a break of at least 1 hour",
            "de" => "eine Arbeitsspanne, die 11-14:30 Uhr oder 18:30-22 Uhr ganz abdeckt, ohne Pause von mindestens 1 Stunde",
            "nl" => "een amplitude die 11-14.30 uur of 18.30-22 uur volledig dekt zonder pauze van minstens 1 uur",
            "it" => "un'ampiezza che copre interamente 11-14.30 o 18.30-22 senza pausa di almeno 1 ora",
            "es" => "una amplitud que cubre por completo 11 h-14.30 h o 18.30 h-22 h sin pausa de al menos 1 hora",
            _ => return None,
        },
        "IDCC16_CASSE_CROUTE_COND" => match lang {
            "en" => "starting the shift before 5 am because of travel",
            "de" => "Dienstbeginn vor 5 Uhr wegen einer Fahrt",
            "nl" => "aanvang van de dienst vóór 5 uur wegens een verplaatsing",
            "it" => "inizio del servizio prima delle 5 a causa di uno spostamento",
            "es" => "inicio del servicio antes de las 5 h por un desplazamiento",
            _ => return None,
        },
        "AN_REPAS" => match lang {
            "en" => "Meals provided by the employer, valued at the flat rate per meal of the order of 25/02/2025 (one day = two meals), less the employee's contribution. In a staff canteen or company restaurant, the benefit is disregarded if the employee pays at least half the flat rate.",
            "de" => "Vom Arbeitgeber gestellte Mahlzeiten, bewertet mit dem Pauschalbetrag je Mahlzeit des Erlasses vom 25.02.2025 (ein Tag = zwei Mahlzeiten), abzüglich der Zuzahlung des Beschäftigten. In einer Kantine oder einem Betriebsrestaurant bleibt der Vorteil außer Ansatz, wenn der Beschäftigte mindestens die Hälfte der Pauschale zahlt.",
            "nl" => "Maaltijden verstrekt door de werkgever, gewaardeerd tegen het forfait per maaltijd van het besluit van 25/02/2025 (een dag = twee maaltijden), min de bijdrage van de werknemer. In een bedrijfskantine of bedrijfsrestaurant wordt het voordeel verwaarloosd als de werknemer minstens de helft van het forfait betaalt.",
            "it" => "Pasti forniti dal datore di lavoro, valutati al forfait per pasto del decreto del 25/02/2025 (una giornata = due pasti), dedotta la partecipazione del dipendente. In mensa o ristorante aziendale il beneficio è trascurato se il dipendente paga almeno la metà del forfait.",
            "es" => "Comidas facilitadas por el empleador, valoradas al tanto alzado por comida de la orden del 25/02/2025 (un día = dos comidas), menos la aportación del trabajador. En comedor o restaurante de empresa, la retribución se desprecia si el trabajador paga al menos la mitad del tanto alzado.",
            _ => return None,
        },
        "AN_LOGEMENT" => match lang {
            "en" => "Housing provided by the employer. Monthly flat rate according to gross pay excluding benefits (8 brackets expressed as a fraction of the social security ceiling) and the number of main rooms, water, gas, electricity, heating and garage included; beyond one room, the amount applies per room. Otherwise, rental value and actual ancillary costs. Employee contribution deducted.",
            "de" => "Vom Arbeitgeber gestellte Wohnung. Monatliche Pauschale nach dem Bruttolohn ohne Sachbezüge (8 Stufen als Bruchteil der Beitragsbemessungsgrenze) und der Zahl der Haupträume, Wasser, Gas, Strom, Heizung und Garage inbegriffen; ab zwei Räumen gilt der Betrag je Raum. Andernfalls Mietwert und tatsächliche Nebenkosten. Zuzahlung des Beschäftigten abgezogen.",
            "nl" => "Huisvesting verstrekt door de werkgever. Maandelijks forfait volgens het brutoloon zonder voordelen (8 schijven uitgedrukt als fractie van het socialezekerheidsplafond) en het aantal hoofdvertrekken, water, gas, elektriciteit, verwarming en garage inbegrepen; boven één vertrek geldt het bedrag per vertrek. Anders huurwaarde en werkelijke bijkomende kosten. Bijdrage van de werknemer afgetrokken.",
            "it" => "Alloggio fornito dal datore di lavoro. Forfait mensile secondo la retribuzione lorda esclusi i benefit (8 fasce espresse come frazione del massimale di previdenza sociale) e il numero di vani principali, acqua, gas, elettricità, riscaldamento e garage compresi; oltre un vano l'importo vale per vano. In alternativa, valore locativo e accessori reali. Partecipazione del dipendente dedotta.",
            "es" => "Vivienda facilitada por el empleador. Tanto alzado mensual según la remuneración bruta sin retribuciones en especie (8 tramos expresados como fracción del tope de la seguridad social) y el número de habitaciones principales, agua, gas, electricidad, calefacción y garaje incluidos; a partir de dos habitaciones el importe se aplica por habitación. Si no, valor de alquiler y gastos accesorios reales. Aportación del trabajador deducida.",
            _ => return None,
        },
        "AN_VEHICULE" => match lang {
            "en" => "Vehicle used privately. Annual flat rate, taken monthly: purchased vehicle, 15 % of the purchase cost incl. VAT (10 % over 5 years), 20 % (15 %) fuel included; leased vehicle, 50 % of the total annual cost, 67 % fuel included. Vehicles provided before 01/02/2025: 9 % (6 %), 12 % (9 %); lease 30 %, 40 %. Fully electric vehicle: capped 70 % allowance if provided between 01/02/2025 and 31/12/2027 and meeting the eco-score, capped 50 % if provided between 01/01/2020 and 31/01/2025; charging electricity is not counted. Employee contribution deducted.",
            "de" => "Privat genutztes Fahrzeug. Jahrespauschale, monatlich angesetzt: gekauftes Fahrzeug 15 % der Anschaffungskosten inkl. MwSt. (10 % über 5 Jahre), 20 % (15 %) mit Kraftstoff; geleastes Fahrzeug 50 % der jährlichen Gesamtkosten, 67 % mit Kraftstoff. Vor dem 01.02.2025 überlassene Fahrzeuge: 9 % (6 %), 12 % (9 %); Leasing 30 %, 40 %. Reines Elektrofahrzeug: gedeckelter Abschlag von 70 % bei Überlassung zwischen 01.02.2025 und 31.12.2027 und erfülltem Öko-Score, gedeckelte 50 % bei Überlassung zwischen 01.01.2020 und 31.01.2025; Ladestrom wird nicht angesetzt. Zuzahlung des Beschäftigten abgezogen.",
            "nl" => "Voertuig voor privégebruik. Jaarlijks forfait, per maand genomen: gekocht voertuig 15 % van de aankoopprijs incl. btw (10 % boven 5 jaar), 20 % (15 %) met brandstof; geleasd voertuig 50 % van de totale jaarkosten, 67 % met brandstof. Voertuigen ter beschikking gesteld vóór 01/02/2025: 9 % (6 %), 12 % (9 %); lease 30 %, 40 %. Volledig elektrisch voertuig: begrensde aftrek van 70 % bij terbeschikkingstelling tussen 01/02/2025 en 31/12/2027 met eco-score, begrensde 50 % tussen 01/01/2020 en 31/01/2025; laadstroom telt niet mee. Bijdrage van de werknemer afgetrokken.",
            "it" => "Veicolo usato a titolo privato. Forfait annuo, preso per dodicesimi: veicolo acquistato 15 % del costo d'acquisto IVA inclusa (10 % oltre 5 anni), 20 % (15 %) carburante compreso; veicolo a noleggio 50 % del costo globale annuo, 67 % carburante compreso. Veicoli messi a disposizione prima del 01/02/2025: 9 % (6 %), 12 % (9 %); noleggio 30 %, 40 %. Veicolo 100 % elettrico: abbattimento del 70 % con massimale se messo a disposizione tra il 01/02/2025 e il 31/12/2027 con eco-score, 50 % con massimale tra il 01/01/2020 e il 31/01/2025; l'elettricità di ricarica non è conteggiata. Partecipazione del dipendente dedotta.",
            "es" => "Vehículo de uso privado. Tanto alzado anual, por dozavas partes: vehículo comprado, 15 % del coste de compra IVA incluido (10 % con más de 5 años), 20 % (15 %) con carburante; vehículo alquilado, 50 % del coste global anual, 67 % con carburante. Vehículos puestos a disposición antes del 01/02/2025: 9 % (6 %), 12 % (9 %); alquiler 30 %, 40 %. Vehículo 100 % eléctrico: reducción del 70 % con tope si se puso a disposición entre el 01/02/2025 y el 31/12/2027 y cumple la ecopuntuación, 50 % con tope entre el 01/01/2020 y el 31/01/2025; la electricidad de recarga no se computa. Aportación del trabajador deducida.",
            _ => return None,
        },
        "AN_NTIC" => match lang {
            "en" => "Digital tools (computer, phone, subscription) used privately: 10 % of the purchase cost incl. VAT or of the annual subscription incl. VAT, taken monthly. Employee contribution deducted. Strictly professional use is not a benefit.",
            "de" => "Digitale Geräte (Computer, Telefon, Abonnement) zur privaten Nutzung: 10 % der Anschaffungskosten oder des Jahresabonnements inkl. MwSt., monatlich angesetzt. Zuzahlung des Beschäftigten abgezogen. Rein berufliche Nutzung ist kein Vorteil.",
            "nl" => "Digitale middelen (computer, telefoon, abonnement) voor privégebruik: 10 % van de aankoopprijs of het jaarabonnement incl. btw, per maand genomen. Bijdrage van de werknemer afgetrokken. Strikt beroepsmatig gebruik is geen voordeel.",
            "it" => "Strumenti digitali (computer, telefono, abbonamento) usati a titolo privato: 10 % del costo d'acquisto o dell'abbonamento annuo IVA inclusa, per dodicesimi. Partecipazione del dipendente dedotta. Un uso strettamente professionale non è un beneficio.",
            "es" => "Herramientas digitales (ordenador, teléfono, suscripción) de uso privado: 10 % del coste de compra o de la suscripción anual IVA incluido, por dozavas partes. Aportación del trabajador deducida. Un uso estrictamente profesional no es retribución.",
            _ => return None,
        },
        "AN_AUTRE" => match lang {
            "en" => "Goods or services provided free or at a reduced price, outside the flat-rate scales: valued at their actual value, less the employee's contribution.",
            "de" => "Unentgeltlich oder verbilligt überlassene Waren oder Dienstleistungen außerhalb der Pauschalen: mit ihrem tatsächlichen Wert bewertet, abzüglich der Zuzahlung des Beschäftigten.",
            "nl" => "Goederen of diensten gratis of tegen verlaagde prijs verstrekt, buiten de forfaits: gewaardeerd tegen hun werkelijke waarde, min de bijdrage van de werknemer.",
            "it" => "Beni o servizi forniti gratuitamente o a prezzo ridotto, fuori dai forfait: valutati al loro valore reale, dedotta la partecipazione del dipendente.",
            "es" => "Bienes o servicios facilitados gratis o a precio reducido, fuera de los baremos: valorados por su valor real, menos la aportación del trabajador.",
            _ => return None,
        },
        "AN_SANS_BAREME" => match lang {
            "en" => "No scale is integrated for this date: the simulator covers the flat rates in force since 2025. Amount left at 0 rather than invented.",
            "de" => "Für dieses Datum ist kein Tarif hinterlegt: der Simulator deckt die seit 2025 geltenden Pauschalen ab. Betrag bei 0 belassen statt erfunden.",
            "nl" => "Voor deze datum is geen barema opgenomen: de simulator dekt de forfaits sinds 2025. Bedrag op 0 gelaten in plaats van verzonnen.",
            "it" => "Nessuna tabella integrata per questa data: il simulatore copre i forfait in vigore dal 2025. Importo lasciato a 0 anziché inventato.",
            "es" => "No hay baremo integrado para esta fecha: el simulador cubre los tantos alzados vigentes desde 2025. Importe dejado en 0 en lugar de inventado.",
            _ => return None,
        },
        "AN_REPAS_NEGLIGE" => match lang {
            "en" => "Here, the contribution reaches half the flat rate: the benefit is disregarded.",
            "de" => "Hier erreicht die Zuzahlung die Hälfte der Pauschale: der Vorteil bleibt außer Ansatz.",
            "nl" => "Hier bereikt de bijdrage de helft van het forfait: het voordeel wordt verwaarloosd.",
            "it" => "Qui la partecipazione raggiunge la metà del forfait: il beneficio è trascurato.",
            "es" => "Aquí la aportación alcanza la mitad del tanto alzado: la retribución se desprecia.",
            _ => return None,
        },
        "AIDE_POSTE_EA" => match lang {
            "en" => "The employment support grant is State financial aid, paid to the employer by \
                the Agency for Services and Payment (ASP), for employing a disabled worker in an \
                adapted enterprise. Annual lump sum per full-time equivalent, paid monthly and \
                prorated to working time. Amount depends on the worker's age bracket. In case of \
                sick leave or accident, the absent share is reduced to 30 % of the gross hourly \
                minimum wage. This aid does not change the employee's gross or net pay: it reduces \
                the real cost borne by the employer.",
            "de" => "Der Beschäftigungszuschuss ist eine staatliche Finanzhilfe, die dem Arbeitgeber \
                von der Agentur für Dienstleistungen und Zahlungen (ASP) für die Beschäftigung eines \
                schwerbehinderten Arbeitnehmers in einem angepassten Unternehmen gezahlt wird. \
                Jährlicher Pauschalbetrag je Vollzeitäquivalent, monatlich gezahlt und nach der \
                Arbeitszeit anteilig berechnet. Höhe je nach Altersgruppe. Bei Krankheit oder Unfall \
                wird der Abwesenheitsanteil auf 30 % des Brutto-Mindeststundenlohns gekürzt. Diese \
                Hilfe ändert weder Brutto- noch Nettolohn: sie senkt die tatsächlichen Arbeitgeberkosten.",
            "nl" => "De tewerkstellingssteun is een financiële steun van de Staat, betaald aan de \
                werkgever door het Agentschap voor Diensten en Betalingen (ASP), voor het in dienst \
                nemen van een werknemer met een handicap in een aangepaste onderneming. Jaarlijks \
                forfait per voltijdequivalent, maandelijks uitbetaald en geproratiseerd naar arbeidstijd. \
                Bedrag afhankelijk van de leeftijdscategorie. Bij ziekte of ongeval wordt het afwezige \
                deel verlaagd tot 30 % van het bruto minimumuurloon. Deze steun wijzigt het bruto- noch \
                het nettoloon: ze verlaagt de werkelijke kosten voor de werkgever.",
            "it" => "L'aiuto al posto di lavoro è un sostegno finanziario dello Stato, versato al \
                datore di lavoro dall'Agenzia per i servizi e i pagamenti (ASP), per l'assunzione di \
                un lavoratore disabile in un'impresa adattata. Importo forfettario annuo per equivalente \
                a tempo pieno, erogato mensilmente e proporzionato all'orario di lavoro. Importo secondo \
                la fascia di età. In caso di malattia o infortunio, la quota di assenza è ridotta al 30 % \
                del salario minimo orario lordo. Questo aiuto non modifica né il lordo né il netto del \
                dipendente: riduce il costo reale a carico del datore di lavoro.",
            "es" => "La ayuda al puesto es una ayuda financiera del Estado, abonada al empleador por \
                la Agencia de Servicios y Pagos (ASP), por emplear a un trabajador con discapacidad en \
                una empresa adaptada. Importe anual a tanto alzado por equivalente a tiempo completo, \
                pagado mensualmente y prorrateado al tiempo de trabajo. Importe según el tramo de edad. \
                En caso de baja por enfermedad o accidente, la parte ausente se reduce al 30 % del \
                salario mínimo bruto por hora. Esta ayuda no modifica ni el bruto ni el neto del \
                trabajador: reduce el coste real soportado por el empleador.",
            _ => return None,
        },
        "REDUC_SAL_HS" => match lang {
            "en" => "Reduction of employee old-age (basic and supplementary) social security \
                contributions on overtime and extra-hour pay, capped at 11.31 % (law of 24/12/2018, \
                Social Security Code art. L241-17). It is deducted from employee contributions: it \
                increases the net pay.",
            "de" => "Senkung der Arbeitnehmerbeiträge zur Rentenversicherung (Grund- und \
                Zusatzversicherung) auf die Vergütung von Über- und Mehrstunden, begrenzt auf 11,31 % \
                (Gesetz vom 24.12.2018, Sozialgesetzbuch Art. L241-17). Sie wird von den \
                Arbeitnehmerbeiträgen abgezogen und erhöht den Nettolohn.",
            "nl" => "Vermindering van de werknemersbijdragen voor het ouderdomspensioen (basis en \
                aanvullend) op de vergoeding van over- en meeruren, beperkt tot 11,31 % (wet van \
                24/12/2018, Sociale Zekerheidswet art. L241-17). Ze wordt afgetrokken van de \
                werknemersbijdragen en verhoogt het nettoloon.",
            "it" => "Riduzione dei contributi previdenziali del dipendente per la vecchiaia (di base \
                e complementare) sulla retribuzione delle ore supplementari e complementari, nel \
                limite dell'11,31 % (legge del 24/12/2018, Codice della previdenza sociale art. \
                L241-17). È dedotta dai contributi del dipendente: aumenta il netto.",
            "es" => "Reducción de las cotizaciones del trabajador a la jubilación (de base y \
                complementaria) sobre la remuneración de las horas extra y complementarias, con un \
                límite del 11,31 % (ley de 24/12/2018, Código de la Seguridad Social art. L241-17). \
                Se deduce de las cotizaciones del trabajador: aumenta el salario neto.",
            _ => return None,
        },
        "DFP_HS" => match lang {
            "en" => "Flat-rate reduction of employer contributions per overtime hour (Social \
                Security Code art. L241-18): €1.50 in companies with fewer than 20 employees, €0.50 \
                from 20 employees. It lowers the real cost borne by the employer, with no effect on \
                the employee's net pay.",
            "de" => "Pauschale Senkung der Arbeitgeberbeiträge je Überstunde (Sozialgesetzbuch \
                Art. L241-18): 1,50 € in Unternehmen mit weniger als 20 Beschäftigten, 0,50 € ab \
                20 Beschäftigten. Sie senkt die tatsächlichen Arbeitgeberkosten, ohne den Nettolohn \
                zu beeinflussen.",
            "nl" => "Forfaitaire vermindering van de werkgeversbijdragen per overuur (Sociale \
                Zekerheidswet art. L241-18): 1,50 € in ondernemingen met minder dan 20 werknemers, \
                0,50 € vanaf 20 werknemers. Ze verlaagt de werkelijke werkgeverskosten, zonder \
                invloed op het nettoloon.",
            "it" => "Riduzione forfettaria dei contributi datoriali per ogni ora supplementare \
                (Codice della previdenza sociale art. L241-18): 1,50 € nelle imprese con meno di 20 \
                dipendenti, 0,50 € a partire da 20 dipendenti. Riduce il costo reale a carico del \
                datore di lavoro, senza effetto sul netto del dipendente.",
            "es" => "Reducción a tanto alzado de las cotizaciones patronales por hora extra (Código \
                de la Seguridad Social art. L241-18): 1,50 € en empresas de menos de 20 trabajadores, \
                0,50 € a partir de 20. Reduce el coste real del empleador, sin efecto sobre el neto \
                del trabajador.",
            _ => return None,
        },
        // Dynamique — placeholders {ts_pct} {tp_pct}
        "FPT_CNRACL" => match lang {
            "en" => "The CNRACL (Caisse Nationale de Retraite des Agents des Collectivités \
                Locales) is the mandatory pension scheme for established local civil servants. \
                It replaces both the general scheme's old-age insurance (CARSAT) and the \
                AGIRC-ARRCO supplementary pension — the civil servant therefore contributes to a \
                single fund for both basic and supplementary pension.\n\n\
                Notable differences from the private sector:\n\
                • Pension computed on the last 6 months (index-based salary), not on the best \
                25 years as in the private sector\n\
                • Target replacement rate: 75 % after 41 years and 3 quarters (2016)\n\
                • No funding: pay-as-you-go scheme\n\n\
                Phase-in 2012-2019 (decree no. 2011-291):\n\
                2016: 10.29 % — 2017: 10.56 % — 2018: 10.83 % — 2019+: 11.10 %\n\
                Employer (local authority) rate: 30.65 % until 2024, then raised by 3 points a year \
                (Decree 2025-86): 34.65 % in 2025, 37.65 % in 2026, 40.65 % in 2027, 43.65 % in 2028 (vs ≈ 16 % total in the private \
                sector)\n\n\
                Applied rates: employee {ts_pct} % — employer {tp_pct} %.",
            "de" => "Die CNRACL (Caisse Nationale de Retraite des Agents des Collectivités \
                Locales) ist das Pflichtrentensystem der verbeamteten Bediensteten der \
                Gebietskörperschaften. Sie ersetzt sowohl die Altersversicherung des \
                Allgemeinsystems (CARSAT) als auch die Zusatzrente AGIRC-ARRCO — der Beamte zahlt \
                also für Grund- und Zusatzrente in eine einzige Kasse ein.\n\n\
                Wesentliche Unterschiede zum Privatsektor:\n\
                • Rente auf Basis der letzten 6 Monate (Besoldung), nicht der besten 25 Jahre wie \
                im Privatsektor\n\
                • Ziel-Ersatzquote: 75 % nach 41 Jahren und 3 Quartalen (2016)\n\
                • Keine Kapitaldeckung: Umlagesystem\n\n\
                Stufenweiser Anstieg 2012-2019 (Dekret Nr. 2011-291):\n\
                2016: 10,29 % — 2017: 10,56 % — 2018: 10,83 % — 2019+: 11,10 %\n\
                Satz der Körperschaft: 30,65 % bis 2024, dann jährlich um 3 Punkte angehoben (Dekret 2025-86): 34,65 % 2025, 37,65 % 2026, 40,65 % 2027, 43,65 % 2028 (vs. ≈ 16 % gesamt im Privatsektor)\n\n\
                Angewandte Sätze: Bediensteter {ts_pct} % — Körperschaft {tp_pct} %.",
            "nl" => "De CNRACL (Caisse Nationale de Retraite des Agents des Collectivités \
                Locales) is het verplichte pensioenstelsel van de vastbenoemde ambtenaren van de \
                lokale besturen. Het vervangt zowel de ouderdomsverzekering van het algemene \
                stelsel (CARSAT) als het aanvullend pensioen AGIRC-ARRCO — de ambtenaar draagt dus \
                bij aan één enkele kas voor basis- en aanvullend pensioen.\n\n\
                Belangrijke verschillen met de privésector:\n\
                • Pensioen berekend op de laatste 6 maanden (weddeschaal), niet op de beste \
                25 jaar zoals in de privé\n\
                • Beoogde vervangingsratio: 75 % na 41 jaar en 3 kwartalen (2016)\n\
                • Geen kapitalisatie: repartitiestelsel\n\n\
                Geleidelijke stijging 2012-2019 (decreet nr. 2011-291):\n\
                2016: 10,29 % — 2017: 10,56 % — 2018: 10,83 % — 2019+: 11,10 %\n\
                Tarief bestuur: 30,65 % tot en met 2024, daarna jaarlijks met 3 punten verhoogd (decreet 2025-86): 34,65 % in 2025, 37,65 % in 2026, 40,65 % in 2027, 43,65 % in 2028 (vs ≈ 16 % totaal in de privé)\n\n\
                Toegepaste tarieven: ambtenaar {ts_pct} % — bestuur {tp_pct} %.",
            "it" => "La CNRACL (Caisse Nationale de Retraite des Agents des Collectivités \
                Locales) è il regime pensionistico obbligatorio dei funzionari territoriali di \
                ruolo. Sostituisce sia l'assicurazione vecchiaia del regime generale (CARSAT) sia \
                la pensione complementare AGIRC-ARRCO — il funzionario versa quindi a un'unica \
                cassa per la pensione di base e complementare.\n\n\
                Differenze notevoli rispetto al settore privato:\n\
                • Pensione calcolata sugli ultimi 6 mesi (trattamento indiciario), non sui \
                25 anni migliori come nel privato\n\
                • Tasso di sostituzione obiettivo: 75 % dopo 41 anni e 3 trimestri (2016)\n\
                • Nessuna capitalizzazione: sistema a ripartizione\n\n\
                Aumento graduale 2012-2019 (decreto n. 2011-291):\n\
                2016: 10,29 % — 2017: 10,56 % — 2018: 10,83 % — 2019+: 11,10 %\n\
                Aliquota ente: 30,65 % fino al 2024, poi aumentata di 3 punti l'anno (decreto 2025-86): 34,65 % nel 2025, 37,65 % nel 2026, 40,65 % nel 2027, 43,65 % nel 2028 (vs ≈ 16 % totale nel privato)\n\n\
                Aliquote applicate: agente {ts_pct} % — ente {tp_pct} %.",
            "es" => "La CNRACL (Caisse Nationale de Retraite des Agents des Collectivités \
                Locales) es el régimen obligatorio de jubilación de los funcionarios territoriales \
                titulares. Sustituye a la vez al seguro de vejez del régimen general (CARSAT) y a \
                la pensión complementaria AGIRC-ARRCO — el funcionario cotiza por tanto a una sola \
                caja para su pensión básica y complementaria.\n\n\
                Diferencias notables con el sector privado:\n\
                • Pensión calculada sobre los últimos 6 meses (salario indiciario), no sobre los \
                25 mejores años como en el privado\n\
                • Tasa de sustitución objetivo: 75 % tras 41 años y 3 trimestres (2016)\n\
                • Sin capitalización: sistema de reparto\n\n\
                Subida progresiva 2012-2019 (decreto n.º 2011-291):\n\
                2016: 10,29 % — 2017: 10,56 % — 2018: 10,83 % — 2019+: 11,10 %\n\
                Tipo de la entidad: 30,65 % hasta 2024, luego elevado 3 puntos al año (decreto 2025-86): 34,65 % en 2025, 37,65 % en 2026, 40,65 % en 2027, 43,65 % en 2028 (vs ≈ 16 % total en el privado)\n\n\
                Tipos aplicados: agente {ts_pct} % — entidad {tp_pct} %.",
            _ => return None,
        },
        // Phrase générique « lacune assumée » (la 1re ligne, par pays, vit dans
        // i18n::non_couvert).
        "PAYS_NON_COUVERT" => match lang {
            "en" => "No figures are applied in the absence of an official source for this date \
                (acknowledged gap, nothing invented).",
            "de" => "Mangels amtlicher Quelle für dieses Datum werden keine Zahlen angewendet \
                (eingestandene Lücke, nichts erfunden).",
            "nl" => "Bij gebrek aan een officiële bron voor deze datum worden geen cijfers \
                toegepast (erkende leemte, niets verzonnen).",
            "it" => "In assenza di una fonte ufficiale per questa data non viene applicata alcuna \
                cifra (lacuna dichiarata, nulla di inventato).",
            "es" => "A falta de fuente oficial para esta fecha no se aplica ninguna cifra \
                (laguna asumida, nada inventado).",
            _ => return None,
        },
        _ => return None,
    })
}
