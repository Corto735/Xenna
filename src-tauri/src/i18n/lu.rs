// Traductions Luxembourg (codes `LU_*`). Placeholders nommés {taux} {plafond} {annee}.

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
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
        "LU_AP" => match lang {
            "en" => "Mandatory pension insurance (CNAP, CSS LU Book II), pay-as-you-go. Rate 16 % \
                total (8 % employee, 8 % employer); the State adds a third. Base capped at 5 × SSM \
                (≈ {plafond} €/month in {annee}). Full pension after 40 years; retirement at 65 \
                (or 57 early).",
            "de" => "Obligatorische Rentenversicherung (CNAP, CSS LU Buch II), Umlageverfahren. \
                Satz 16 % gesamt (8 % AN, 8 % AG); der Staat ergänzt ein Drittel. Bemessung \
                gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in {annee}). Volle Rente nach 40 Jahren; \
                Rente mit 65 (oder 57 vorgezogen).",
            "nl" => "Verplichte pensioenverzekering (CNAP, CSS LU Boek II), omslagstelsel. Tarief \
                16 % totaal (8 % wn, 8 % wg); de Staat voegt een derde toe. Grondslag begrensd op \
                5 × SSM (≈ {plafond} €/maand in {annee}). Volledig pensioen na 40 jaar; pensioen op \
                65 (of 57 vervroegd).",
            "it" => "Assicurazione pensione obbligatoria (CNAP, CSS LU Libro II), a ripartizione. \
                Aliquota 16 % totale (8 % dipendente, 8 % datore); lo Stato aggiunge un terzo. Base \
                limitata a 5 × SSM (≈ {plafond} €/mese in {annee}). Pensione piena dopo 40 anni; \
                pensione a 65 (o 57 anticipata).",
            "es" => "Seguro de pensión obligatorio (CNAP, CSS LU Libro II), por reparto. Tipo 16 % \
                total (8 % trabajador, 8 % empleador); el Estado añade un tercio. Base limitada a \
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
                Notably 100 % employee contribution (1.40 %), no employer share. Capped at 5 × SSM \
                (≈ {plafond} €/month in {annee}). Managed by CNS.",
            "de" => "Pflegeversicherung (Gesetz vom 19.06.1998): Leistungen für pflegebedürftige \
                Personen. Besonderheit: 100 % Arbeitnehmerbeitrag (1,40 %), kein Arbeitgeberanteil. \
                Gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in {annee}). Verwaltung CNS.",
            "nl" => "Langdurigezorgverzekering (wet 19-06-1998): uitkeringen voor zorgafhankelijke \
                personen. Bijzonderheid: 100 % werknemersbijdrage (1,40 %), geen werkgeversdeel. \
                Begrensd op 5 × SSM (≈ {plafond} €/maand in {annee}). Beheer CNS.",
            "it" => "Assicurazione dipendenza (legge 19/06/1998): prestazioni per persone non \
                autonome. Particolarità: contributo 100 % dipendente (1,40 %), senza quota \
                datoriale. Limitata a 5 × SSM (≈ {plafond} €/mese in {annee}). Gestione CNS.",
            "es" => "Seguro de dependencia (ley 19/06/1998): prestaciones para personas \
                dependientes. Particularidad: cotización 100 % del trabajador (1,40 %), sin parte \
                patronal. Limitada a 5 × SSM (≈ {plafond} €/mes en {annee}). Gestión CNS.",
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
