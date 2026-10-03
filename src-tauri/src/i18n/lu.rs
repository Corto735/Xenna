// Traductions Luxembourg (codes `LU_*`). Placeholders nommés {taux} {plafond} {annee}.

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "LU_IMPOT" => match lang {
            "en" => "Income tax — wage withholding (tax class 1)",
            "de" => "Einkommensteuer — Lohnsteuerabzug (Steuerklasse 1)",
            "nl" => "Inkomstenbelasting — loonbelasting (belastingklasse 1)",
            "it" => "Imposta sul reddito — ritenuta sui salari (classe 1)",
            "es" => "Impuesto sobre la renta — retención salarial (clase 1)",
            _ => return None,
        },
        "LU_AP" => match lang {
            "en" => "AP — Pension insurance",
            "de" => "AP — Rentenversicherung",
            "nl" => "AP — Pensioenverzekering",
            "it" => "AP — Assicurazione pensione",
            "es" => "AP — Seguro de pensión",
            _ => return None,
        },
        "LU_AM" => match lang {
            "en" => "AM — Health-maternity insurance (CNS)",
            "de" => "AM — Kranken-Mutterschaftsversicherung (CNS)",
            "nl" => "AM — Ziekte-moederschapsverzekering (CNS)",
            "it" => "AM — Assicurazione malattia-maternità (CNS)",
            "es" => "AM — Seguro de enfermedad-maternidad (CNS)",
            _ => return None,
        },
        "LU_AD" => match lang {
            "en" => "AD — Long-term care insurance",
            "de" => "AD — Pflegeversicherung",
            "nl" => "AD — Langdurigezorgverzekering",
            "it" => "AD — Assicurazione dipendenza",
            "es" => "AD — Seguro de dependencia",
            _ => return None,
        },
        "LU_AA" => match lang {
            "en" => "AA — Accident insurance (AAA)",
            "de" => "AA — Unfallversicherung (AAA)",
            "nl" => "AA — Ongevallenverzekering (AAA)",
            "it" => "AA — Assicurazione infortuni (AAA)",
            "es" => "AA — Seguro de accidentes (AAA)",
            _ => return None,
        },
        "LU_ME" => match lang {
            "en" => "ME — Employers' mutual fund",
            "de" => "ME — Arbeitgeber-Ausgleichskasse",
            "nl" => "ME — Werkgeversmutualiteit",
            "it" => "ME — Mutua dei datori di lavoro",
            "es" => "ME — Mutualidad de empleadores",
            _ => return None,
        },
        _ => return None,
    })
}

pub fn t_explication(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "LU_IMPOT" => match lang {
            "en" => "Wage tax withholding {annee}, tax class 1 (single, no children).\n\nTaxable income: {b} € − pension and health contributions − flat expense allowance 540 € − flat special expenses 480 € = {r} €/yr\nScale 0 to 42 % → {i} €; employment fund contribution {fe} % → {ife} €\n− employee tax credit and CO2 credit {cr} € = {a} €/yr, {m} €/month.\n\nSource: ACD (basic scale, CIS and CI-CO2).",
            "de" => "Lohnsteuerabzug {annee}, Steuerklasse 1 (ledig, ohne Kinder).\n\nSteuerpflichtiges Einkommen: {b} € − Renten- und Krankenversicherungsbeiträge − Werbungskostenpauschale 540 € − Sonderausgabenpauschale 480 € = {r} €/Jahr\nTarif 0 bis 42 % → {i} €; Beitrag zum Beschäftigungsfonds {fe} % → {ife} €\n− Steuerkredit für Arbeitnehmer und CO2-Steuerkredit {cr} € = {a} €/Jahr, {m} €/Monat.\n\nQuelle: ACD (Grundtarif, CIS und CI-CO2).",
            "nl" => "Loonbelastinginhouding {annee}, belastingklasse 1 (alleenstaand, zonder kinderen).\n\nBelastbaar inkomen: {b} € − pensioen- en ziektebijdragen − forfaitaire beroepskosten 540 € − forfaitaire bijzondere uitgaven 480 € = {r} €/jr\nSchaal 0 tot 42 % → {i} €; bijdrage aan het werkgelegenheidsfonds {fe} % → {ife} €\n− belastingkrediet werknemer en CO2-krediet {cr} € = {a} €/jr, {m} €/maand.\n\nBron: ACD (basistarief, CIS en CI-CO2).",
            "it" => "Ritenuta d'imposta sui salari {annee}, classe d'imposta 1 (celibe, senza figli).\n\nReddito imponibile: {b} € − contributi pensione e malattia − spese di produzione forfettarie 540 € − spese speciali forfettarie 480 € = {r} €/anno\nScaglioni da 0 a 42 % → {i} €; contributo al fondo per l'occupazione {fe} % → {ife} €\n− credito d'imposta per dipendenti e credito CO2 {cr} € = {a} €/anno, {m} €/mese.\n\nFonte: ACD (tariffa di base, CIS e CI-CO2).",
            "es" => "Retención del impuesto sobre salarios {annee}, clase impositiva 1 (soltero, sin hijos).\n\nRenta imponible: {b} € − cotizaciones de pensión y enfermedad − gastos de obtención a tanto alzado 540 € − gastos especiales a tanto alzado 480 € = {r} €/año\nEscala de 0 a 42 % → {i} €; contribución al fondo para el empleo {fe} % → {ife} €\n− crédito fiscal del asalariado y crédito CO2 {cr} € = {a} €/año, {m} €/mes.\n\nFuente: ACD (tarifa básica, CIS y CI-CO2).",
            _ => return None,
        },
        "LU_AP" => match lang {
            "en" => "Mandatory pension insurance (CNAP, CSS LU Book II), pay-as-you-go. Rate 8 % \
                employee and 8 % employer until 2025, 8.5 % each since 1 January 2026 (pension reform); the State adds a third. Base capped at 5 × SSM \
                (≈ {plafond} €/month in {annee}). Full pension after 40 years; retirement at 65 \
                (or 57 early).",
            "de" => "Obligatorische Rentenversicherung (CNAP, CSS LU Buch II), Umlageverfahren. \
                Satz 8 % AN und 8 % AG bis 2025, je 8,5 % seit dem 1. Januar 2026 (Rentenreform); der Staat ergänzt ein Drittel. Bemessung \
                gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in {annee}). Volle Rente nach 40 Jahren; \
                Rente mit 65 (oder 57 vorgezogen).",
            "nl" => "Verplichte pensioenverzekering (CNAP, CSS LU Boek II), omslagstelsel. Tarief \
                8 % wn en 8 % wg tot 2025, elk 8,5 % sinds 1 januari 2026 (pensioenhervorming); de Staat voegt een derde toe. Grondslag begrensd op \
                5 × SSM (≈ {plafond} €/maand in {annee}). Volledig pensioen na 40 jaar; pensioen op \
                65 (of 57 vervroegd).",
            "it" => "Assicurazione pensione obbligatoria (CNAP, CSS LU Libro II), a ripartizione. \
                Aliquota 8 % dipendente e 8 % datore fino al 2025, 8,5 % ciascuno dal 1° gennaio 2026 (riforma delle pensioni); lo Stato aggiunge un terzo. Base \
                limitata a 5 × SSM (≈ {plafond} €/mese in {annee}). Pensione piena dopo 40 anni; \
                pensione a 65 (o 57 anticipata).",
            "es" => "Seguro de pensión obligatorio (CNAP, CSS LU Libro II), por reparto. Tipo 8 % \
                trabajador y 8 % empleador hasta 2025, 8,5 % cada uno desde el 1 de enero de 2026 (reforma de pensiones); el Estado añade un tercio. Base limitada a \
                5 × SSM (≈ {plafond} €/mes en {annee}). Pensión completa tras 40 años; jubilación a \
                los 65 (o 57 anticipada).",
            _ => return None,
        },
        "LU_AM" => match lang {
            "en" => "Health-maternity insurance (CNS): healthcare + cash benefits (100 % of pay for \
                52 weeks, then 80 % up to 78). Contribution 3.05 % (care 2.80 % + cash 0.25 %). Base \
                capped at 5 × SSM (≈ {plafond} €/month in {annee}). Generalised third-party payment \
                since 2010.",
            "de" => "Kranken-Mutterschaftsversicherung (CNS): Gesundheitsversorgung + Geldleistungen \
                (100 % des Lohns 52 Wochen, dann 80 % bis 78). Beitrag 3,05 % (Pflege 2,80 % + Geld \
                0,25 %). Bemessung gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in {annee}). \
                Sachleistungsprinzip seit 2010.",
            "nl" => "Ziekte-moederschapsverzekering (CNS): zorg + uitkeringen (100 % van het loon 52 \
                weken, daarna 80 % tot 78). Bijdrage 3,05 % (zorg 2,80 % + uitkering 0,25 %). \
                Grondslag begrensd op 5 × SSM (≈ {plafond} €/maand in {annee}). Algemeen \
                derdebetalerssysteem sinds 2010.",
            "it" => "Assicurazione malattia-maternità (CNS): assistenza sanitaria + indennità \
                (100 % della retribuzione 52 settimane, poi 80 % fino a 78). Contributo 3,05 % (cure \
                2,80 % + indennità 0,25 %). Base limitata a 5 × SSM (≈ {plafond} €/mese in {annee}). \
                Terzo pagante generalizzato dal 2010.",
            "es" => "Seguro de enfermedad-maternidad (CNS): asistencia sanitaria + prestaciones \
                (100 % del salario 52 semanas, luego 80 % hasta 78). Cotización 3,05 % (atención \
                2,80 % + prestaciones 0,25 %). Base limitada a 5 × SSM (≈ {plafond} €/mes en \
                {annee}). Tercero pagador generalizado desde 2010.",
            _ => return None,
        },
        "LU_AD" => match lang {
            "en" => "Long-term care insurance (law of 19/06/1998): benefits for dependent persons. \
                Notably 100 % employee contribution (1.40 %), no employer share. No ceiling: base = gross \
                minus an allowance of a quarter of the minimum wage ({abattement} €/month in {annee}). Managed by CNS.",
            "de" => "Pflegeversicherung (Gesetz vom 19.06.1998): Leistungen für pflegebedürftige \
                Personen. Besonderheit: 100 % Arbeitnehmerbeitrag (1,40 %), kein Arbeitgeberanteil. \
                Keine Obergrenze: Bemessungsgrundlage = Brutto abzüglich eines Freibetrags von einem Viertel des Mindestlohns ({abattement} €/Monat in {annee}). Verwaltung CNS.",
            "nl" => "Langdurigezorgverzekering (wet 19-06-1998): uitkeringen voor zorgafhankelijke \
                personen. Bijzonderheid: 100 % werknemersbijdrage (1,40 %), geen werkgeversdeel. \
                Geen plafond: grondslag = brutoloon min een aftrek van een kwart van het minimumloon ({abattement} €/maand in {annee}). Beheer CNS.",
            "it" => "Assicurazione dipendenza (legge 19/06/1998): prestazioni per persone non \
                autonome. Particolarità: contributo 100 % dipendente (1,40 %), senza quota \
                datoriale. Nessun massimale: base = lordo meno una franchigia di un quarto del salario minimo ({abattement} €/mese in {annee}). Gestione CNS.",
            "es" => "Seguro de dependencia (ley 19/06/1998): prestaciones para personas \
                dependientes. Particularidad: cotización 100 % del trabajador (1,40 %), sin parte \
                patronal. Sin tope: base = bruto menos una deducción de un cuarto del salario mínimo ({abattement} €/mes en {annee}). Gestión CNS.",
            _ => return None,
        },
        "LU_AA" => match lang {
            "en" => "Mandatory accident insurance (AAA), work accidents and occupational diseases, \
                100 % employer. Rate {taux} % indicative (services); 3–10× higher in high-risk \
                sectors. Capped at 5 × SSM (≈ {plafond} €/month in {annee}). CSS LU Book III.",
            "de" => "Obligatorische Unfallversicherung (AAA), Arbeitsunfälle und Berufskrankheiten, \
                100 % Arbeitgeber. Satz {taux} % indikativ (Dienstleistung); in Risikobranchen \
                3–10× höher. Gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in {annee}). CSS LU Buch III.",
            "nl" => "Verplichte ongevallenverzekering (AAA), arbeidsongevallen en beroepsziekten, \
                100 % werkgever. Tarief {taux} % indicatief (diensten); 3–10× hoger in \
                risicosectoren. Begrensd op 5 × SSM (≈ {plafond} €/maand in {annee}). CSS LU Boek III.",
            "it" => "Assicurazione infortuni obbligatoria (AAA), infortuni sul lavoro e malattie \
                professionali, 100 % datoriale. Aliquota {taux} % indicativa (terziario); 3–10× più \
                alta nei settori a rischio. Limitata a 5 × SSM (≈ {plafond} €/mese in {annee}). \
                CSS LU Libro III.",
            "es" => "Seguro de accidentes obligatorio (AAA), accidentes laborales y enfermedades \
                profesionales, 100 % patronal. Tipo {taux} % indicativo (servicios); 3–10× más alto \
                en sectores de riesgo. Limitado a 5 × SSM (≈ {plafond} €/mes en {annee}). CSS LU \
                Libro III.",
            _ => return None,
        },
        "LU_ME" => match lang {
            "en" => "Employers' mutual fund (CCSS): solidarity scheme reimbursing employers for \
                continued pay (sick days 1–77), CNS taking over from day 78. Rate {taux} % \
                indicative (services). Capped at 5 × SSM (≈ {plafond} €/month in {annee}).",
            "de" => "Arbeitgeber-Ausgleichskasse (CCSS): Solidarsystem, das Arbeitgebern die \
                Lohnfortzahlung erstattet (Krankheitstage 1–77), CNS übernimmt ab Tag 78. Satz \
                {taux} % indikativ (Dienstleistung). Gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in \
                {annee}).",
            "nl" => "Werkgeversmutualiteit (CCSS): solidariteitsmechanisme dat werkgevers het \
                doorbetaalde loon vergoedt (ziektedagen 1–77), CNS neemt over vanaf dag 78. Tarief \
                {taux} % indicatief (diensten). Begrensd op 5 × SSM (≈ {plafond} €/maand in {annee}).",
            "it" => "Mutua dei datori di lavoro (CCSS): meccanismo di solidarietà che rimborsa ai \
                datori la retribuzione mantenuta (giorni di malattia 1–77), la CNS subentra dal 78°. \
                Aliquota {taux} % indicativa (terziario). Limitata a 5 × SSM (≈ {plafond} €/mese in \
                {annee}).",
            "es" => "Mutualidad de empleadores (CCSS): mecanismo de solidaridad que reembolsa a los \
                empleadores el salario mantenido (días de baja 1–77), la CNS toma el relevo desde el \
                78. Tipo {taux} % indicativo (servicios). Limitada a 5 × SSM (≈ {plafond} €/mes en \
                {annee}).",
            _ => return None,
        },
        _ => return None,
    })
}
