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
        "AIDE_POSTE_EA" => match lang {
            "en" => "Employment support grant — adapted enterprise (State/ASP)",
            "de" => "Beschäftigungszuschuss — angepasstes Unternehmen (Staat/ASP)",
            "nl" => "Tewerkstellingssteun — aangepaste onderneming (Staat/ASP)",
            "it" => "Aiuto al posto di lavoro — impresa adattata (Stato/ASP)",
            "es" => "Ayuda al puesto — empresa adaptada (Estado/ASP)",
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
                France in 1919 (Act of 1 June 1924). Rate 1.50% until 30/06/2018, then 1.30% \
                from 01/07/2018 (LFSS 2018).",
            "de" => "Das Lokalregime Elsass-Mosel (Lokalrecht) bietet den Beschäftigten der \
                Departements Bas-Rhin (67), Haut-Rhin (68) und Moselle (57) eine obligatorische \
                ergänzende Krankenversicherung. Dieser nur vom Arbeitnehmer getragene Beitrag wird \
                zusätzlich zum allgemeinen System erhoben. Er finanziert eine Erstattung von 90 % \
                (gegenüber 70 % im allgemeinen System), ohne Selbstbeteiligung bei Krankenhausaufenthalten. \
                Dieses Regime geht auf das seit 1871 geltende bismarcksche Recht zurück, beibehalten bei \
                der Rückkehr Elsass-Lothringens zu Frankreich 1919 (Gesetz vom 1. Juni 1924). \
                Satz 1,50 % bis 30.06.2018, dann 1,30 % ab 01.07.2018 (LFSS 2018).",
            "nl" => "Het lokale stelsel van de Elzas-Moezel (lokaal recht) biedt een verplichte \
                aanvullende ziekteverzekering aan werknemers van de departementen Bas-Rhin (67), \
                Haut-Rhin (68) en Moezel (57). Deze uitsluitend door de werknemer gedragen bijdrage \
                wordt bovenop het algemene stelsel geheven. Ze financiert een terugbetaling van 90% \
                (tegenover 70% in het algemene stelsel), zonder remgeld voor ziekenhuisopnames. \
                Dit stelsel stamt uit het sinds 1871 geldende bismarckiaanse recht, behouden bij de \
                terugkeer van Elzas-Lotharingen naar Frankrijk in 1919 (wet van 1 juni 1924). \
                Tarief 1,50% tot 30/06/2018, daarna 1,30% vanaf 01/07/2018 (LFSS 2018).",
            "it" => "Il regime locale dell'Alsazia-Mosella (diritto locale) offre una copertura \
                malattia integrativa obbligatoria ai dipendenti dei dipartimenti del Bas-Rhin (67), \
                Haut-Rhin (68) e Mosella (57). Questo contributo, esclusivamente a carico del dipendente, \
                è prelevato in aggiunta al regime generale. Finanzia un rimborso al 90% (contro il 70% \
                del regime generale), senza ticket per i ricoveri. Questo regime deriva dal diritto \
                bismarckiano vigente dal 1871, mantenuto al ritorno dell'Alsazia-Lorena alla Francia \
                nel 1919 (legge del 1° giugno 1924). Aliquota 1,50% fino al 30/06/2018, poi 1,30% \
                dal 01/07/2018 (LFSS 2018).",
            "es" => "El régimen local de Alsacia-Mosela (derecho local) ofrece una cobertura \
                de enfermedad complementaria obligatoria a los trabajadores de los departamentos del \
                Bajo Rin (67), Alto Rin (68) y Mosela (57). Esta cotización, únicamente a cargo del \
                trabajador, se recauda además del régimen general. Financia un reembolso del 90% \
                (frente al 70% del régimen general), sin copago para las hospitalizaciones. Este régimen \
                procede del derecho bismarckiano vigente desde 1871, mantenido al volver Alsacia-Lorena \
                a Francia en 1919 (ley del 1 de junio de 1924). Tipo 1,50% hasta el 30/06/2018, luego \
                1,30% desde el 01/07/2018 (LFSS 2018).",
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
        // Fragment plafond proratisé temps partiel — placeholders {etp} {pmss}
        "PMSS_ETP_NOTE" => match lang {
            "en" => "\n⚠ Part-time {etp} % — prorated SSC ceiling: {pmss} € (reduced ceiling, CSS art. L242-1)",
            "de" => "\n⚠ Teilzeit {etp} % — anteilige Beitragsbemessungsgrenze: {pmss} € (CSS Art. L242-1)",
            "nl" => "\n⚠ Deeltijds {etp} % — geprorateerd plafond: {pmss} € (verlaagd plafond, CSS art. L242-1)",
            "it" => "\n⚠ Tempo parziale {etp} % — massimale proporzionato: {pmss} € (massimale ridotto, CSS art. L242-1)",
            "es" => "\n⚠ Tiempo parcial {etp} % — tope prorrateado: {pmss} € (tope reducido, CSS art. L242-1)",
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
        _ => return None,
    })
}
