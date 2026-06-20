// Traductions Suisse (codes `CH_*`). Libellés + explications en/de/nl/it/es.
// Placeholders nommés identiques aux 6 langues, substitués côté appelant.

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "CH_AVS" => match lang {
            "en" => "AVS — Old-age and survivors insurance",
            "de" => "AVS — Alters- und Hinterlassenenversicherung",
            "nl" => "AVS — Ouderdoms- en nabestaandenverzekering",
            "it" => "AVS — Assicurazione vecchiaia e superstiti",
            "es" => "AVS — Seguro de vejez y supervivencia",
            _ => return None,
        },
        "CH_AI" => match lang {
            "en" => "AI — Disability insurance",
            "de" => "AI — Invalidenversicherung",
            "nl" => "AI — Invaliditeitsverzekering",
            "it" => "AI — Assicurazione invalidità",
            "es" => "AI — Seguro de invalidez",
            _ => return None,
        },
        "CH_APG" => match lang {
            "en" => "APG — Loss-of-earnings allowances",
            "de" => "APG — Erwerbsersatzordnung",
            "nl" => "APG — Inkomstenvervangende uitkeringen",
            "it" => "APG — Indennità per perdita di guadagno",
            "es" => "APG — Subsidios por pérdida de ganancia",
            _ => return None,
        },
        "CH_AC" => match lang {
            "en" => "AC — Unemployment insurance",
            "de" => "AC — Arbeitslosenversicherung",
            "nl" => "AC — Werkloosheidsverzekering",
            "it" => "AC — Assicurazione contro la disoccupazione",
            "es" => "AC — Seguro de desempleo",
            _ => return None,
        },
        "CH_AANP" => match lang {
            "en" => "AANP — Non-occupational accident insurance",
            "de" => "NBU — Nichtberufsunfallversicherung",
            "nl" => "AANP — Verzekering niet-beroepsongevallen",
            "it" => "AINP — Assicurazione infortuni non professionali",
            "es" => "AANP — Seguro de accidentes no profesionales",
            _ => return None,
        },
        "CH_AAP" => match lang {
            "en" => "AAP — Occupational accident insurance",
            "de" => "BU — Berufsunfallversicherung",
            "nl" => "AAP — Verzekering beroepsongevallen",
            "it" => "AAP — Assicurazione infortuni professionali",
            "es" => "AAP — Seguro de accidentes profesionales",
            _ => return None,
        },
        "CH_IJM" => match lang {
            "en" => "IJM — Daily sickness benefits (collective plan)",
            "de" => "KTG — Krankentaggeld (Kollektivvertrag)",
            "nl" => "IJM — Dagvergoeding ziekte (collectief plan)",
            "it" => "IJM — Indennità giornaliera malattia (piano collettivo)",
            "es" => "IJM — Subsidio diario de enfermedad (plan colectivo)",
            _ => return None,
        },
        "CH_LPP" => match lang {
            "en" => "LPP — Occupational pension (2nd pillar)",
            "de" => "BVG — Berufliche Vorsorge (2. Säule)",
            "nl" => "LPP — Beroepspensioen (2e pijler)",
            "it" => "LPP — Previdenza professionale (2º pilastro)",
            "es" => "LPP — Previsión profesional (2º pilar)",
            _ => return None,
        },
        "CH_IS" => match lang {
            "en" => "Withholding tax — Tariff {tarif} — {libelle_canton}",
            "de" => "Quellensteuer — Tarif {tarif} — {libelle_canton}",
            "nl" => "Bronbelasting — Tarief {tarif} — {libelle_canton}",
            "it" => "Imposta alla fonte — Tariffa {tarif} — {libelle_canton}",
            "es" => "Impuesto en origen — Tarifa {tarif} — {libelle_canton}",
            _ => return None,
        },
        _ => return None,
    })
}

pub fn t_explication(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "CH_AVS" => match lang {
            "en" => "AVS (1st pillar, LAVS 1948) is pay-as-you-go and pays a pension from age 65 \
                (AVS 21 reform, 01/01/2024). Rate 8.70 % since 2020 (RFFA funding) — 4.35 % each. \
                Base = full gross, no cap. Max pension 2025: CHF 2,590/month; min: CHF 1,225.",
            "de" => "Die AVS (1. Säule, LAVS 1948) ist umlagefinanziert und zahlt ab 65 eine Rente \
                (Reform AHV 21, 01.01.2024). Satz 8,70 % seit 2020 (STAF-Finanzierung) — je 4,35 %. \
                Bemessung = volles Brutto, ohne Obergrenze. Max. Rente 2025: CHF 2'590/Monat; \
                min.: CHF 1'225.",
            "nl" => "De AVS (1e pijler, LAVS 1948) is omslaggefinancierd en betaalt pensioen vanaf \
                65 (hervorming AHV 21, 01-01-2024). Tarief 8,70 % sinds 2020 (RFFA-financiering) — \
                elk 4,35 %. Grondslag = volledig bruto, geen plafond. Max. pensioen 2025: \
                CHF 2.590/maand; min.: CHF 1.225.",
            "it" => "L'AVS (1º pilastro, LAVS 1948) è a ripartizione e versa una rendita dai 65 \
                anni (riforma AVS 21, 01/01/2024). Aliquota 8,70 % dal 2020 (finanziamento RFFA) — \
                4,35 % ciascuno. Base = lordo totale, senza massimale. Rendita max 2025: \
                CHF 2'590/mese; min: CHF 1'225.",
            "es" => "El AVS (1er pilar, LAVS 1948) es de reparto y paga una pensión desde los 65 \
                años (reforma AVS 21, 01/01/2024). Tipo 8,70 % desde 2020 (financiación RFFA) — \
                4,35 % cada uno. Base = bruto total, sin tope. Pensión máx 2025: CHF 2.590/mes; \
                mín: CHF 1.225.",
            _ => return None,
        },
        "CH_AI" => match lang {
            "en" => "AI (LAI 1959) supplements the 1st pillar: pensions and rehabilitation \
                measures for lastingly reduced earning capacity. Stable rate 1.40 % (0.70 % each), \
                full gross with no cap. The 2022 AI revision prioritises reintegration \
                ('rehabilitation before pension').",
            "de" => "Die IV (IVG 1959) ergänzt die 1. Säule: Renten und Eingliederungsmassnahmen \
                bei dauerhaft verminderter Erwerbsfähigkeit. Stabiler Satz 1,40 % (je 0,70 %), \
                volles Brutto ohne Obergrenze. Die IV-Revision 2022 priorisiert die Eingliederung \
                ('Eingliederung vor Rente').",
            "nl" => "De AI (LAI 1959) vult de 1e pijler aan: uitkeringen en re-integratiemaatregelen \
                bij duurzaam verminderd verdienvermogen. Stabiel tarief 1,40 % (elk 0,70 %), \
                volledig bruto zonder plafond. De AI-herziening 2022 geeft prioriteit aan \
                re-integratie ('re-integratie vóór rente').",
            "it" => "L'AI (LAI 1959) integra il 1º pilastro: rendite e misure di reinserimento per \
                capacità di guadagno durevolmente ridotta. Aliquota stabile 1,40 % (0,70 % \
                ciascuno), lordo totale senza massimale. La revisione AI 2022 dà priorità al \
                reinserimento ('riabilitazione prima della rendita').",
            "es" => "El AI (LAI 1959) complementa el 1er pilar: rentas y medidas de reinserción por \
                capacidad de ganancia duraderamente reducida. Tipo estable 1,40 % (0,70 % cada \
                uno), bruto total sin tope. La revisión AI 2022 prioriza la reinserción ('la \
                rehabilitación antes que la renta').",
            _ => return None,
        },
        "CH_APG" => match lang {
            "en" => "APG (LAPG) compensate loss of earnings during military/civil/civil-protection \
                service, maternity and, since 2021, paternity leave (2 weeks). Rate 0.50 % \
                (0.25 % each), full gross with no cap. Often grouped AVS/AI/APG = 10.60 % \
                (5.30 % each).",
            "de" => "Die EO (EOG) ersetzt den Erwerbsausfall bei Militär-/Zivil-/Zivilschutzdienst, \
                Mutterschaft und seit 2021 Vaterschaftsurlaub (2 Wochen). Satz 0,50 % (je 0,25 %), \
                volles Brutto ohne Obergrenze. Oft AHV/IV/EO = 10,60 % (je 5,30 %).",
            "nl" => "De APG (LAPG) compenseert inkomstenverlies tijdens militaire/civiele/civiele-\
                beschermingsdienst, moederschap en sinds 2021 vaderschapsverlof (2 weken). Tarief \
                0,50 % (elk 0,25 %), volledig bruto zonder plafond. Vaak AVS/AI/APG = 10,60 % \
                (elk 5,30 %).",
            "it" => "Le APG (LAPG) compensano la perdita di guadagno durante servizio militare/\
                civile/protezione civile, maternità e, dal 2021, congedo di paternità (2 settimane). \
                Aliquota 0,50 % (0,25 % ciascuno), lordo totale senza massimale. Spesso AVS/AI/APG \
                = 10,60 % (5,30 % ciascuno).",
            "es" => "Los APG (LAPG) compensan la pérdida de ganancia durante el servicio militar/\
                civil/protección civil, maternidad y, desde 2021, permiso de paternidad (2 \
                semanas). Tipo 0,50 % (0,25 % cada uno), bruto total sin tope. A menudo AVS/AI/APG \
                = 10,60 % (5,30 % cada uno).",
            _ => return None,
        },
        "CH_AC" => match lang {
            "en" => "AC (LACI 1982) pays 70–80 % of insured earnings. Rate 2.20 % (1.10 % each), \
                capped at CHF 148,200/yr (CHF {plafond} /month); above that, no contribution. \
                Administered by SECO, cantons and funds.",
            "de" => "Die ALV (AVIG 1982) zahlt 70–80 % des versicherten Verdienstes. Satz 2,20 % \
                (je 1,10 %), gedeckelt auf CHF 148'200/Jahr (CHF {plafond} /Monat); darüber kein \
                Beitrag. Verwaltet von SECO, Kantonen und Kassen.",
            "nl" => "De AC (LACI 1982) betaalt 70–80 % van het verzekerde loon. Tarief 2,20 % \
                (elk 1,10 %), begrensd op CHF 148.200/jr (CHF {plafond} /maand); daarboven geen \
                bijdrage. Beheerd door SECO, kantons en kassen.",
            "it" => "L'AC (LADI 1982) versa il 70–80 % del guadagno assicurato. Aliquota 2,20 % \
                (1,10 % ciascuno), limitata a CHF 148'200/anno (CHF {plafond} /mese); oltre, nessun \
                contributo. Gestita da SECO, cantoni e casse.",
            "es" => "El AC (LACI 1982) paga el 70–80 % de la ganancia asegurada. Tipo 2,20 % \
                (1,10 % cada uno), limitado a CHF 148.200/año (CHF {plafond} /mes); por encima, sin \
                cotización. Administrado por SECO, cantones y cajas.",
            _ => return None,
        },
        "CH_AANP" => match lang {
            "en" => "LAA distinguishes occupational (AAP, employer) and non-occupational (AANP, \
                employee) accidents. AANP covers off-work accidents. Employee-borne. Base capped at \
                CHF 148,200/yr (CHF {plafond} /month). Rate {taux} % set by the insurer \
                (SUVA/private) — office indicative.",
            "de" => "Das UVG unterscheidet Berufsunfälle (BU, Arbeitgeber) und Nichtberufsunfälle \
                (NBU, Arbeitnehmer). Die NBU deckt Unfälle ausserhalb der Arbeit. Vom Arbeitnehmer \
                getragen. Bemessung gedeckelt auf CHF 148'200/Jahr (CHF {plafond} /Monat). Satz \
                {taux} % vom Versicherer (SUVA/privat) — Büro indikativ.",
            "nl" => "De LAA onderscheidt beroepsongevallen (AAP, werkgever) en niet-\
                beroepsongevallen (AANP, werknemer). AANP dekt ongevallen buiten het werk. Door \
                werknemer gedragen. Grondslag begrensd op CHF 148.200/jr (CHF {plafond} /maand). \
                Tarief {taux} % bepaald door verzekeraar (SUVA/privé) — kantoor indicatief.",
            "it" => "La LAINF distingue infortuni professionali (AAP, datore) e non professionali \
                (AINP, dipendente). L'AINP copre gli infortuni fuori dal lavoro. A carico del \
                dipendente. Base limitata a CHF 148'200/anno (CHF {plafond} /mese). Aliquota \
                {taux} % fissata dall'assicuratore (SUVA/privato) — ufficio indicativo.",
            "es" => "La LAA distingue accidentes profesionales (AAP, empleador) y no profesionales \
                (AANP, trabajador). El AANP cubre accidentes fuera del trabajo. A cargo del \
                trabajador. Base limitada a CHF 148.200/año (CHF {plafond} /mes). Tipo {taux} % \
                fijado por el asegurador (SUVA/privado) — oficina indicativo.",
            _ => return None,
        },
        "CH_AAP" => match lang {
            "en" => "AAP covers occupational accidents and diseases arising at work. 100 % \
                employer. Base capped at CHF 148,200/yr (CHF {plafond} /month). Rate {taux} % set \
                by SUVA/insurer per risk class (NOGA code). Services indicative.",
            "de" => "Die BU deckt Berufsunfälle und -krankheiten am Arbeitsplatz. 100 % \
                Arbeitgeber. Bemessung gedeckelt auf CHF 148'200/Jahr (CHF {plafond} /Monat). Satz \
                {taux} % von SUVA/Versicherer nach Risikoklasse (NOGA-Code). Dienstleistung \
                indikativ.",
            "nl" => "AAP dekt beroepsongevallen en -ziekten op het werk. 100 % werkgever. \
                Grondslag begrensd op CHF 148.200/jr (CHF {plafond} /maand). Tarief {taux} % \
                bepaald door SUVA/verzekeraar per risicoklasse (NOGA-code). Diensten indicatief.",
            "it" => "L'AAP copre infortuni e malattie professionali sul lavoro. 100 % datore. Base \
                limitata a CHF 148'200/anno (CHF {plafond} /mese). Aliquota {taux} % fissata da \
                SUVA/assicuratore per classe di rischio (codice NOGA). Terziario indicativo.",
            "es" => "El AAP cubre accidentes y enfermedades profesionales en el trabajo. 100 % \
                empleador. Base limitada a CHF 148.200/año (CHF {plafond} /mes). Tipo {taux} % \
                fijado por SUVA/asegurador según clase de riesgo (código NOGA). Servicios \
                indicativo.",
            _ => return None,
        },
        "CH_IJM" => match lang {
            "en" => "Switzerland has no mandatory daily sickness benefit (only basic LAMal is). \
                Employers take out a collective plan (LCA or LAMal art. 67-77) covering ~80 % of \
                pay for 720-730 days. Contractual funding; indicative rate 1.50 % (0.75 % each), \
                variable.",
            "de" => "Die Schweiz kennt kein obligatorisches Krankentaggeld (nur die KVG-\
                Grundversicherung). Arbeitgeber schliessen einen Kollektivvertrag (VVG oder KVG \
                Art. 67-77) ab, der ~80 % des Lohns für 720-730 Tage deckt. Vertragliche \
                Finanzierung; Richtsatz 1,50 % (je 0,75 %), variabel.",
            "nl" => "Zwitserland kent geen verplichte ziektedaguitkering (alleen de basis-LAMal). \
                Werkgevers sluiten een collectief plan (LCA of LAMal art. 67-77) dat ~80 % van het \
                loon 720-730 dagen dekt. Contractuele financiering; indicatief tarief 1,50 % \
                (elk 0,75 %), variabel.",
            "it" => "La Svizzera non ha un'indennità giornaliera malattia obbligatoria (solo la \
                LAMal di base). I datori stipulano un piano collettivo (LCA o LAMal art. 67-77) che \
                copre ~80 % della retribuzione per 720-730 giorni. Finanziamento convenzionale; \
                aliquota indicativa 1,50 % (0,75 % ciascuno), variabile.",
            "es" => "Suiza no tiene un subsidio diario de enfermedad obligatorio (solo la LAMal \
                básica). Los empleadores contratan un plan colectivo (LCA o LAMal art. 67-77) que \
                cubre ~80 % del salario durante 720-730 días. Financiación convencional; tipo \
                indicativo 1,50 % (0,75 % cada uno), variable.",
            _ => return None,
        },
        "CH_LPP" => match lang {
            "en" => "LPP (RS 831.40, 1985) is the 2nd pillar; mandatory above CHF 22,680/yr (2025 \
                threshold).\n\n[ Coordinated salary {annee} ]\nCoordinated salary = max(CHF \
                {coord_min}, gross − coordination deduction)\n  = max(CHF {coord_min}, {brut} − CHF \
                {coord_ded}) = CHF {coord}\nCapped at CHF {coord_max}/month.\n\nLegal minimum rate \
                by age (art. 16): 25-34 → 7 %; 35-44 → 10 %; 45-54 → 15 %; 55-65 → 18 % (half \
                each). Deductible contributions; funded (capitalisation).",
            "de" => "Das BVG (SR 831.40, 1985) ist die 2. Säule; obligatorisch ab CHF 22'680/Jahr \
                (Schwelle 2025).\n\n[ Koordinierter Lohn {annee} ]\nKoordinierter Lohn = max(CHF \
                {coord_min}, brutto − Koordinationsabzug)\n  = max(CHF {coord_min}, {brut} − CHF \
                {coord_ded}) = CHF {coord}\nGedeckelt auf CHF {coord_max}/Monat.\n\nGesetzlicher \
                Mindestsatz nach Alter (Art. 16): 25-34 → 7 %; 35-44 → 10 %; 45-54 → 15 %; 55-65 → \
                18 % (je hälftig). Abzugsfähige Beiträge; Kapitaldeckungsverfahren.",
            "nl" => "De LPP (SR 831.40, 1985) is de 2e pijler; verplicht boven CHF 22.680/jr \
                (drempel 2025).\n\n[ Gecoördineerd loon {annee} ]\nGecoördineerd loon = max(CHF \
                {coord_min}, bruto − coördinatieaftrek)\n  = max(CHF {coord_min}, {brut} − CHF \
                {coord_ded}) = CHF {coord}\nBegrensd op CHF {coord_max}/maand.\n\nWettelijk \
                minimumtarief naar leeftijd (art. 16): 25-34 → 7 %; 35-44 → 10 %; 45-54 → 15 %; \
                55-65 → 18 % (elk de helft). Aftrekbare bijdragen; kapitaaldekking.",
            "it" => "La LPP (RS 831.40, 1985) è il 2º pilastro; obbligatoria oltre CHF 22'680/anno \
                (soglia 2025).\n\n[ Salario coordinato {annee} ]\nSalario coordinato = max(CHF \
                {coord_min}, lordo − deduzione di coordinamento)\n  = max(CHF {coord_min}, {brut} − \
                CHF {coord_ded}) = CHF {coord}\nLimitato a CHF {coord_max}/mese.\n\nAliquota minima \
                legale per età (art. 16): 25-34 → 7 %; 35-44 → 10 %; 45-54 → 15 %; 55-65 → 18 % \
                (metà ciascuno). Contributi deducibili; gestione a capitalizzazione.",
            "es" => "La LPP (RS 831.40, 1985) es el 2º pilar; obligatoria por encima de CHF \
                22.680/año (umbral 2025).\n\n[ Salario coordinado {annee} ]\nSalario coordinado = \
                máx(CHF {coord_min}, bruto − deducción de coordinación)\n  = máx(CHF {coord_min}, \
                {brut} − CHF {coord_ded}) = CHF {coord}\nLimitado a CHF {coord_max}/mes.\n\nTipo \
                mínimo legal por edad (art. 16): 25-34 → 7 %; 35-44 → 10 %; 45-54 → 15 %; 55-65 → \
                18 % (mitad cada uno). Cotizaciones deducibles; gestión por capitalización.",
            _ => return None,
        },
        "CH_IS" => match lang {
            "en" => "Withholding tax (IS, Quellensteuer) is levied directly on the pay of foreign \
                workers without a settlement permit (C), cross-border and non-resident workers \
                (art. 83-90a LIFD). It replaces the ordinary tax return below CHF 120,000/yr.\n\n\
                [ Parameters ]\nCanton: {canton} — {libelle_canton}\nORIS tariff: {tarif} — \
                {tarif_label}\nMonthly gross: CHF {sal}\nScale threshold: CHF {seuil}\nBase A0 rate: \
                {taux_a0} %\nTariff multiplier {tarif}: × {mult}\nApplied rate: {taux_a0} % × {mult} \
                = {taux_final} %\nIS amount = CHF {sal} × {taux_final} % = CHF {montant}\n\n\
                Computed monthly on the gross pay (smoothing rules may apply to irregular income). \
                Since the RAS reform (ORIS 12/11/2014), a single tariff scale applies across the \
                canton.",
            "de" => "Die Quellensteuer (IS) wird direkt auf den Lohn ausländischer Arbeitnehmer \
                ohne Niederlassungsbewilligung (C), Grenzgänger und Nichtansässiger erhoben (Art. \
                83-90a LIFD). Sie ersetzt die ordentliche Steuererklärung unter CHF 120'000/Jahr.\n\n\
                [ Parameter ]\nKanton: {canton} — {libelle_canton}\nORIS-Tarif: {tarif} — \
                {tarif_label}\nMonatsbrutto: CHF {sal}\nTarifschwelle: CHF {seuil}\nA0-Basissatz: \
                {taux_a0} %\nTarifmultiplikator {tarif}: × {mult}\nAngewandter Satz: {taux_a0} % × \
                {mult} = {taux_final} %\nQuellensteuer = CHF {sal} × {taux_final} % = CHF {montant}\n\n\
                Monatlich auf das Bruttogehalt berechnet (Glättungsregeln bei unregelmässigem \
                Einkommen). Seit der RAS-Reform (ORIS 12.11.2014) gilt ein einheitlicher Tarif pro \
                Kanton.",
            "nl" => "De bronbelasting (IS, Quellensteuer) wordt rechtstreeks ingehouden op het loon \
                van buitenlandse werknemers zonder vestigingsvergunning (C), grensarbeiders en niet-\
                ingezetenen (art. 83-90a LIFD). Ze vervangt de gewone aangifte onder CHF 120.000/jr.\n\n\
                [ Parameters ]\nKanton: {canton} — {libelle_canton}\nORIS-tarief: {tarif} — \
                {tarif_label}\nMaandbruto: CHF {sal}\nSchaaldrempel: CHF {seuil}\nA0-basistarief: \
                {taux_a0} %\nTariefvermenigvuldiger {tarif}: × {mult}\nToegepast tarief: {taux_a0} % \
                × {mult} = {taux_final} %\nIS-bedrag = CHF {sal} × {taux_final} % = CHF {montant}\n\n\
                Maandelijks berekend op het brutoloon (afvlakkingsregels bij onregelmatig inkomen). \
                Sinds de RAS-hervorming (ORIS 12-11-2014) geldt één tariefschaal per kanton.",
            "it" => "L'imposta alla fonte (IS, Quellensteuer) è prelevata direttamente sul salario \
                dei lavoratori stranieri senza permesso di domicilio (C), frontalieri e non \
                residenti (art. 83-90a LIFD). Sostituisce la dichiarazione ordinaria sotto i CHF \
                120'000/anno.\n\n[ Parametri ]\nCantone: {canton} — {libelle_canton}\nTariffa ORIS: \
                {tarif} — {tarif_label}\nLordo mensile: CHF {sal}\nSoglia scala: CHF {seuil}\n\
                Aliquota base A0: {taux_a0} %\nMoltiplicatore tariffa {tarif}: × {mult}\nAliquota \
                applicata: {taux_a0} % × {mult} = {taux_final} %\nImporto IS = CHF {sal} × \
                {taux_final} % = CHF {montant}\n\nCalcolata mensilmente sul lordo (regole di \
                livellamento per redditi irregolari). Dalla riforma RAS (ORIS 12/11/2014) si applica \
                un'unica scala tariffaria per cantone.",
            "es" => "El impuesto en origen (IS, Quellensteuer) se retiene directamente del salario \
                de trabajadores extranjeros sin permiso de establecimiento (C), fronterizos y no \
                residentes (art. 83-90a LIFD). Sustituye a la declaración ordinaria por debajo de \
                CHF 120.000/año.\n\n[ Parámetros ]\nCantón: {canton} — {libelle_canton}\nTarifa \
                ORIS: {tarif} — {tarif_label}\nBruto mensual: CHF {sal}\nUmbral de escala: CHF \
                {seuil}\nTipo base A0: {taux_a0} %\nMultiplicador tarifa {tarif}: × {mult}\nTipo \
                aplicado: {taux_a0} % × {mult} = {taux_final} %\nImporte IS = CHF {sal} × \
                {taux_final} % = CHF {montant}\n\nCalculado mensualmente sobre el bruto (reglas de \
                suavizado para ingresos irregulares). Desde la reforma RAS (ORIS 12/11/2014) se \
                aplica una única escala por cantón.",
            _ => return None,
        },
        _ => return None,
    })
}
