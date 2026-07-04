// Traductions Italie (codes `IT_*`). Libellés + explications en/de/nl/it/es.
//
// Convention : on renvoie None quand la traduction est identique au français
// (terme propre INPS, acronyme…) → l'appelant retombe sur le texte fr.
// Les placeholders nommés (`{annee}`, `{massimale}`, …) sont identiques dans les
// 6 langues et substitués côté appelant après lookup.

/// Clé de traduction stable d'une ligne (les addizionali régionales partagent
/// la clé `IT_ADD_REG`, le nom de région étant substitué via placeholder).
fn cle(code: &str) -> &str {
    if code.starts_with("IT_ADD_REG") {
        "IT_ADD_REG"
    } else {
        code
    }
}

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match cle(code) {
        "IT_NASPI" => match lang {
            "en" => "NASpI — Unemployment insurance (ordinary contribution)",
            "de" => "NASpI — Arbeitslosenversicherung (ordentlicher Beitrag)",
            "nl" => "NASpI — Werkloosheidsverzekering (gewone bijdrage)",
            "it" => "NASpI — Assicurazione disoccupazione (contributo ordinario)",
            "es" => "NASpI — Seguro de desempleo (cotización ordinaria)",
            _ => return None,
        },
        "IT_NASPI_TERMINE" => match lang {
            "en" => "NASpI — Additional fixed-term contribution (+1.40 % empr)",
            "de" => "NASpI — Zusatzbeitrag befristet (+1,40 % AG)",
            "nl" => "NASpI — Aanvullende bijdrage tijdelijk (+1,40 % wg)",
            "it" => "NASpI — Contributo addizionale a termine (+1,40 % dat.)",
            "es" => "NASpI — Cotización adicional temporal (+1,40 % empr)",
            _ => return None,
        },
        "IT_MALATTIA" => match lang {
            "en" => "Malattia — Daily sickness allowance (INPS)",
            "de" => "Malattia — Krankengeld (INPS)",
            "nl" => "Malattia — Dagvergoeding ziekte (INPS)",
            "it" => "Malattia — Indennità giornaliere (INPS)",
            "es" => "Malattia — Subsidio diario por enfermedad (INPS)",
            _ => return None,
        },
        "IT_MATERNITA" => match lang {
            "en" => "Maternità / Paternità — Parental leave (INPS)",
            "de" => "Maternità / Paternità — Elternzeit (INPS)",
            "nl" => "Maternità / Paternità — Ouderschapsverlof (INPS)",
            "it" => "Maternità / Paternità — Congedi parentali (INPS)",
            "es" => "Maternità / Paternità — Permisos parentales (INPS)",
            _ => return None,
        },
        "IT_TFR" => match lang {
            "en" => "TFR — Trattamento Fine Rapporto (monthly accrual)",
            "de" => "TFR — Trattamento Fine Rapporto (monatliche Rückstellung)",
            "nl" => "TFR — Trattamento Fine Rapporto (maandelijkse opbouw)",
            "it" => "TFR — Trattamento Fine Rapporto (accantonamento mensile)",
            "es" => "TFR — Trattamento Fine Rapporto (provisión mensual)",
            _ => return None,
        },
        "IT_ESONERO_2022" => match lang {
            "en" => "Contribution relief H2 2022 (−0.80 % IVS)",
            "de" => "Beitragsentlastung H2 2022 (−0,80 % IVS)",
            "nl" => "Bijdragevermindering H2 2022 (−0,80 % IVS)",
            "it" => "Esonero contributivo H2 2022 (−0,80 % IVS)",
            "es" => "Exoneración de cotización H2 2022 (−0,80 % IVS)",
            _ => return None,
        },
        "IT_ESONERO_2023" => match lang {
            "en" => "Contribution relief 2023 (−{taux_pp} % IVS)",
            "de" => "Beitragsentlastung 2023 (−{taux_pp} % IVS)",
            "nl" => "Bijdragevermindering 2023 (−{taux_pp} % IVS)",
            "it" => "Esonero contributivo 2023 (−{taux_pp} % IVS)",
            "es" => "Exoneración de cotización 2023 (−{taux_pp} % IVS)",
            _ => return None,
        },
        "IT_ESONERO_2024" => match lang {
            "en" => "Contribution relief 2024 (−{taux_pp} % IVS)",
            "de" => "Beitragsentlastung 2024 (−{taux_pp} % IVS)",
            "nl" => "Bijdragevermindering 2024 (−{taux_pp} % IVS)",
            "it" => "Esonero contributivo 2024 (−{taux_pp} % IVS)",
            "es" => "Exoneración de cotización 2024 (−{taux_pp} % IVS)",
            _ => return None,
        },
        "IT_IRPEF" => match lang {
            "en" => "IRPEF — Withholding {annee}",
            "de" => "IRPEF — Quellensteuer {annee}",
            "nl" => "IRPEF — Inhouding {annee}",
            "it" => "IRPEF — Ritenuta alla fonte {annee}",
            "es" => "IRPEF — Retención en origen {annee}",
            _ => return None,
        },
        "IT_ADD_REG" => match lang {
            "en" => "Regional IRPEF surcharge — {libelle_region}",
            "de" => "Regionaler IRPEF-Zuschlag — {libelle_region}",
            "nl" => "Regionale IRPEF-toeslag — {libelle_region}",
            "it" => "Addizionale regionale IRPEF — {libelle_region}",
            "es" => "Recargo regional del IRPEF — {libelle_region}",
            _ => return None,
        },
        "IT_IVS" => match lang {
            "en" => "IVS — Disability, old-age and survivors' pension",
            "de" => "IVS — Invaliditäts-, Alters- und Hinterbliebenenrente",
            "nl" => "IVS — Invaliditeits-, ouderdoms- en nabestaandenpensioen",
            "it" => "IVS — Invalidità, Vecchiaia, Superstiti",
            "es" => "IVS — Invalidez, vejez y supervivencia",
            _ => return None,
        },
        "IT_FONDO_GARANZIA" => match lang {
            "en" => "TFR Guarantee Fund — INPS (L. 297/1982)",
            "de" => "TFR-Garantiefonds — INPS (G. 297/1982)",
            "nl" => "TFR-garantiefonds — INPS (W. 297/1982)",
            "it" => "Fondo di Garanzia TFR — INPS (L. 297/1982)",
            "es" => "Fondo de Garantía TFR — INPS (L. 297/1982)",
            _ => return None,
        },
        "IT_INAIL" => match lang {
            "en" => "INAIL — Occupational accident and disease insurance",
            "de" => "INAIL — Arbeitsunfall- und Berufskrankheitenversicherung",
            "nl" => "INAIL — Arbeidsongevallen- en beroepsziekteverzekering",
            "it" => "INAIL — Assicurazione Infortuni e Malattie Professionali",
            "es" => "INAIL — Seguro de accidentes y enfermedades profesionales",
            _ => return None,
        },
        _ => return None,
    })
}

pub fn t_explication(key: &str, lang: &str) -> Option<&'static str> {
    Some(match cle(key) {
        "IT_IVS" => match lang {
            "en" => "IVS (Invalidità, Vecchiaia, Superstiti) is Italy's mandatory pension \
                contribution (INPS), governed by L. 335/1995. Total rate 33 % = 9.19 % employee \
                + 23.81 % employer, stable since the 1990s. Massimale contributivo {annee}: \
                {massimale} €/month ({annuel} €/yr), applying only to workers with no INPS \
                seniority at 31/12/1995; pre-1996 employees contribute on the full salary.",
            "de" => "IVS (Invalidità, Vecchiaia, Superstiti) ist Italiens obligatorischer \
                Rentenbeitrag (INPS), geregelt durch L. 335/1995. Gesamtsatz 33 % = 9,19 % \
                Arbeitnehmer + 23,81 % Arbeitgeber, seit den 1990ern stabil. Massimale \
                contributivo {annee}: {massimale} €/Monat ({annuel} €/Jahr), nur für \
                Arbeitnehmer ohne INPS-Anwartschaft zum 31.12.1995; vor 1996 Beschäftigte \
                zahlen auf das volle Gehalt.",
            "nl" => "IVS (Invalidità, Vecchiaia, Superstiti) is de verplichte Italiaanse \
                pensioenbijdrage (INPS), geregeld door L. 335/1995. Totaaltarief 33 % = 9,19 % \
                werknemer + 23,81 % werkgever, stabiel sinds de jaren 90. Massimale contributivo \
                {annee}: {massimale} €/maand ({annuel} €/jr), alleen voor werknemers zonder \
                INPS-anciënniteit op 31-12-1995; vóór 1996 betalen op het volledige loon.",
            "it" => "L'IVS (Invalidità, Vecchiaia, Superstiti) è il contributo pensionistico \
                obbligatorio italiano (INPS), disciplinato dalla L. 335/1995. Aliquota totale \
                33 % = 9,19 % dipendente + 23,81 % datore di lavoro, stabile dagli anni '90. \
                Massimale contributivo {annee}: {massimale} €/mese ({annuel} €/anno), applicabile \
                ai soli lavoratori senza anzianità INPS al 31/12/1995; gli iscritti ante 1996 \
                contribuiscono sull'intera retribuzione.",
            "es" => "El IVS (Invalidità, Vecchiaia, Superstiti) es la cotización de jubilación \
                obligatoria italiana (INPS), regida por la L. 335/1995. Tipo total 33 % = 9,19 % \
                trabajador + 23,81 % empleador, estable desde los años 90. Massimale contributivo \
                {annee}: {massimale} €/mes ({annuel} €/año), aplicable solo a trabajadores sin \
                antigüedad INPS al 31/12/1995; los anteriores a 1996 cotizan sobre el salario íntegro.",
            _ => return None,
        },
        "IT_NASPI" => match lang {
            "en" => "NASpI (D.Lgs. 22/2015, Jobs Act) indemnifies dismissed employees, \
                proportional to the average wage of the last 4 years (duration = half the \
                contributed weeks, max 24 months). The employee contribution (0.30 %) was \
                abolished on 01/01/2013 (L. 228/2012): only the ordinary employer contribution \
                (1.61 %) remains, with no cap.",
            "de" => "NASpI (D.Lgs. 22/2015, Jobs Act) entschädigt entlassene Arbeitnehmer, \
                anteilig zum Durchschnittslohn der letzten 4 Jahre (Dauer = halbe Beitragswochen, \
                max. 24 Monate). Der Arbeitnehmerbeitrag (0,30 %) ist seit 01.01.2013 abgeschafft \
                (L. 228/2012): es bleibt nur der ordentliche Arbeitgeberbeitrag (1,61 %), ohne \
                Obergrenze.",
            "nl" => "NASpI (D.Lgs. 22/2015, Jobs Act) vergoedt ontslagen werknemers, evenredig \
                aan het gemiddelde loon van de laatste 4 jaar (duur = helft van de bijdrageweken, \
                max. 24 maanden). De werknemersbijdrage (0,30 %) is sinds 01-01-2013 afgeschaft \
                (L. 228/2012): alleen de gewone werkgeversbijdrage (1,61 %) blijft, zonder plafond.",
            "it" => "La NASpI (D.Lgs. 22/2015, Jobs Act) indennizza i dipendenti licenziati, in \
                proporzione alla retribuzione media degli ultimi 4 anni (durata = metà delle \
                settimane contribuite, max 24 mesi). Il contributo dipendente (0,30 %) è soppresso \
                dal 01/01/2013 (L. 228/2012): resta solo il contributo ordinario datoriale \
                (1,61 %), senza massimale.",
            "es" => "La NASpI (D.Lgs. 22/2015, Jobs Act) indemniza a los trabajadores despedidos, \
                en proporción al salario medio de los últimos 4 años (duración = mitad de las \
                semanas cotizadas, máx. 24 meses). La cotización del trabajador (0,30 %) se \
                suprimió el 01/01/2013 (L. 228/2012): solo queda la cotización ordinaria patronal \
                (1,61 %), sin tope.",
            _ => return None,
        },
        "IT_NASPI_TERMINE" => match lang {
            "en" => "Employer surcharge of 1.40 % on fixed-term contracts (L. 92/2012 art. 2 \
                c. 28-29), refunded if the contract is converted to permanent within 6 months. \
                Not applicable to replacement, seasonal, apprentice or intermittent contracts.",
            "de" => "Arbeitgeberzuschlag von 1,40 % auf befristete Verträge (L. 92/2012 Art. 2 \
                Abs. 28-29), erstattet bei Umwandlung in unbefristet binnen 6 Monaten. Nicht für \
                Vertretungs-, Saison-, Ausbildungs- oder intermittierende Verträge.",
            "nl" => "Werkgeverstoeslag van 1,40 % op tijdelijke contracten (L. 92/2012 art. 2 \
                c. 28-29), terugbetaald bij omzetting naar vast binnen 6 maanden. Niet voor \
                vervangings-, seizoens-, leerling- of oproepcontracten.",
            "it" => "Maggiorazione datoriale dell'1,40 % sui contratti a termine (L. 92/2012 \
                art. 2 c. 28-29), restituita in caso di trasformazione a tempo indeterminato \
                entro 6 mesi. Non si applica a sostituzione, stagionali, apprendisti o intermittenti.",
            "es" => "Recargo patronal del 1,40 % sobre contratos temporales (L. 92/2012 art. 2 \
                c. 28-29), reembolsado si se transforma en indefinido en 6 meses. No aplicable a \
                sustitución, temporeros, aprendices ni intermitentes.",
            _ => return None,
        },
        "IT_MALATTIA" => match lang {
            "en" => "Funds INPS daily allowances from the 4th day of leave (first 3 days — \
                carenza — borne by employer or CCNL). Allowance: 50 % of daily wage from day 4 \
                to 20, 66.66 % from day 21 to 180. Rate 2.22 % indicative (commercio/industria), \
                varies by CCNL.",
            "de" => "Finanziert INPS-Krankengeld ab dem 4. Tag (erste 3 Tage — carenza — vom \
                Arbeitgeber oder CCNL getragen). Leistung: 50 % des Tageslohns vom 4.–20. Tag, \
                66,66 % vom 21.–180. Satz 2,22 % indikativ (commercio/industria), je nach CCNL.",
            "nl" => "Financiert INPS-dagvergoeding vanaf de 4e ziektedag (eerste 3 dagen — \
                carenza — door werkgever of CCNL gedragen). Vergoeding: 50 % van het dagloon \
                dag 4-20, 66,66 % dag 21-180. Tarief 2,22 % indicatief (commercio/industria), \
                varieert per CCNL.",
            "it" => "Finanzia le indennità giornaliere INPS dal 4° giorno di assenza (primi 3 \
                giorni — carenza — a carico del datore o del CCNL). Indennità: 50 % della \
                retribuzione giornaliera dal 4° al 20° giorno, 66,66 % dal 21° al 180°. Aliquota \
                2,22 % indicativa (commercio/industria), variabile per CCNL.",
            "es" => "Financia los subsidios diarios del INPS desde el 4º día de baja (los 3 \
                primeros — carenza — a cargo del empleador o del CCNL). Subsidio: 50 % del \
                salario diario del día 4 al 20, 66,66 % del 21 al 180. Tipo 2,22 % indicativo \
                (commercio/industria), variable según CCNL.",
            _ => return None,
        },
        "IT_MATERNITA" => match lang {
            "en" => "Funds INPS parental leave: maternity 5 months at 80 %, mandatory paternity \
                10 days at 80 %, parental leave up to 6 months/parent (L. 207/2024: 80 % 1st \
                month, 60 % 2nd). 100 % employer contribution (0.46 %), stable.",
            "de" => "Finanziert INPS-Elternzeit: Mutterschaft 5 Monate zu 80 %, Pflicht-\
                Vaterschaft 10 Tage zu 80 %, Elternzeit bis 6 Monate/Elternteil (L. 207/2024: \
                80 % 1. Monat, 60 % 2.). 100 % Arbeitgeberbeitrag (0,46 %), stabil.",
            "nl" => "Financiert INPS-ouderschapsverlof: moederschap 5 maanden tegen 80 %, \
                verplicht vaderschap 10 dagen tegen 80 %, ouderschapsverlof tot 6 maanden/ouder \
                (L. 207/2024: 80 % 1e maand, 60 % 2e). 100 % werkgeversbijdrage (0,46 %), stabiel.",
            "it" => "Finanzia i congedi parentali INPS: maternità 5 mesi all'80 %, paternità \
                obbligatoria 10 giorni all'80 %, congedo parentale fino a 6 mesi/genitore \
                (L. 207/2024: 80 % 1° mese, 60 % 2°). Contributo 100 % datoriale (0,46 %), stabile.",
            "es" => "Financia los permisos parentales del INPS: maternidad 5 meses al 80 %, \
                paternidad obligatoria 10 días al 80 %, permiso parental hasta 6 meses/progenitor \
                (L. 207/2024: 80 % 1er mes, 60 % 2º). Cotización 100 % patronal (0,46 %), estable.",
            _ => return None,
        },
        "IT_FONDO_GARANZIA" => match lang {
            "en" => "The Fondo di Garanzia (INPS) guarantees TFR payment if the employer is \
                insolvent (L. 297/1982 art. 2). Employer contribution 0.20 %, paid via F24. \
                Distinct from the direct transfer to the Fondo Tesoreria INPS (mandatory > 50 \
                employees since 2007).",
            "de" => "Der Fondo di Garanzia (INPS) sichert die TFR-Zahlung bei Insolvenz des \
                Arbeitgebers (L. 297/1982 Art. 2). Arbeitgeberbeitrag 0,20 %, gezahlt über F24. \
                Unterscheidet sich von der direkten Zahlung an den Fondo Tesoreria INPS (Pflicht \
                > 50 Beschäftigte seit 2007).",
            "nl" => "Het Fondo di Garanzia (INPS) waarborgt de TFR-betaling bij insolventie van \
                de werkgever (L. 297/1982 art. 2). Werkgeversbijdrage 0,20 %, betaald via F24. \
                Te onderscheiden van de directe storting aan het Fondo Tesoreria INPS (verplicht \
                > 50 werknemers sinds 2007).",
            "it" => "Il Fondo di Garanzia (INPS) garantisce il pagamento del TFR in caso di \
                insolvenza del datore (L. 297/1982 art. 2). Contributo datoriale 0,20 %, versato \
                tramite F24. Distinto dal versamento diretto al Fondo Tesoreria INPS (obbligatorio \
                > 50 dipendenti dal 2007).",
            "es" => "El Fondo di Garanzia (INPS) garantiza el pago del TFR si el empleador es \
                insolvente (L. 297/1982 art. 2). Cotización patronal 0,20 %, abonada vía F24. \
                Distinto del ingreso directo al Fondo Tesoreria INPS (obligatorio > 50 \
                trabajadores desde 2007).",
            _ => return None,
        },
        "IT_INAIL" => match lang {
            "en" => "Work accident and occupational disease insurance (INAIL), mandatory and \
                100 % employer (DPR 1124/1965). Rate {taux} % indicative (office/terziario); the \
                actual rate depends on the ATECO voce di tariffa, claims history (±28 %) and \
                prevention measures. Auto-liquidazione on 16 February.",
            "de" => "Arbeitsunfall- und Berufskrankheitenversicherung (INAIL), Pflicht und 100 % \
                Arbeitgeber (DPR 1124/1965). Satz {taux} % indikativ (Büro/terziario); der \
                tatsächliche Satz hängt von der ATECO voce di tariffa, der Schadenshistorie \
                (±28 %) und Präventionsmaßnahmen ab. Auto-liquidazione am 16. Februar.",
            "nl" => "Verzekering arbeidsongevallen en beroepsziekten (INAIL), verplicht en 100 % \
                werkgever (DPR 1124/1965). Tarief {taux} % indicatief (kantoor/terziario); het \
                werkelijke tarief hangt af van de ATECO voce di tariffa, schadeverleden (±28 %) \
                en preventiemaatregelen. Auto-liquidazione op 16 februari.",
            "it" => "Assicurazione infortuni sul lavoro e malattie professionali (INAIL), \
                obbligatoria e 100 % datoriale (DPR 1124/1965). Aliquota {taux} % indicativa \
                (ufficio/terziario); l'aliquota reale dipende dalla voce di tariffa ATECO, dalla \
                sinistrosità (±28 %) e dalle misure di prevenzione. Autoliquidazione al 16 febbraio.",
            "es" => "Seguro de accidentes de trabajo y enfermedades profesionales (INAIL), \
                obligatorio y 100 % patronal (DPR 1124/1965). Tipo {taux} % indicativo \
                (oficina/terziario); el tipo real depende de la voce di tariffa ATECO, la \
                siniestralidad (±28 %) y las medidas de prevención. Auto-liquidazione el 16 de febrero.",
            _ => return None,
        },
        "IT_TFR" => match lang {
            "en" => "TFR (L. 297/1982) is deferred pay: monthly provision of {montant} € \
                (6.91 % = 1/13.5 of annual gross), paid at contract end. Destination by size \
                (≤ 50: with employer; > 50: Fondo Tesoreria INPS or pension fund). Revaluation \
                75 % ISTAT + 1.5 %.",
            "de" => "TFR (L. 297/1982) ist aufgeschobenes Entgelt: monatliche Rückstellung von \
                {montant} € (6,91 % = 1/13,5 des Jahresbruttos), bei Vertragsende ausgezahlt. Ziel \
                je nach Größe (≤ 50: beim Arbeitgeber; > 50: Fondo Tesoreria INPS oder \
                Pensionsfonds). Aufwertung 75 % ISTAT + 1,5 %.",
            "nl" => "TFR (L. 297/1982) is uitgesteld loon: maandelijkse voorziening van \
                {montant} € (6,91 % = 1/13,5 van het jaarbruto), uitbetaald bij einde contract. \
                Bestemming naar grootte (≤ 50: bij werkgever; > 50: Fondo Tesoreria INPS of \
                pensioenfonds). Herwaardering 75 % ISTAT + 1,5 %.",
            "it" => "Il TFR (L. 297/1982) è retribuzione differita: accantonamento mensile di \
                {montant} € (6,91 % = 1/13,5 del lordo annuo), pagato a fine rapporto. \
                Destinazione per dimensione (≤ 50: presso il datore; > 50: Fondo Tesoreria INPS \
                o fondo pensione). Rivalutazione 75 % ISTAT + 1,5 %.",
            "es" => "El TFR (L. 297/1982) es retribución diferida: provisión mensual de \
                {montant} € (6,91 % = 1/13,5 del bruto anual), pagada al final del contrato. \
                Destino según tamaño (≤ 50: en el empleador; > 50: Fondo Tesoreria INPS o fondo \
                de pensiones). Revalorización 75 % ISTAT + 1,5 %.",
            _ => return None,
        },
        "IT_ESONERO_2022" => match lang {
            "en" => "Temporary 0.80 pt reduction of the employee IVS contribution, July–December \
                2022 if reddito ≤ 35,000 € (DL 115/2022, conv. L. 142/2022). The negative amount \
                increases net pay.",
            "de" => "Vorübergehende Senkung um 0,80 Pkt. des AN-IVS-Beitrags, Juli–Dezember 2022 \
                bei reddito ≤ 35.000 € (DL 115/2022, umgew. L. 142/2022). Der negative Betrag \
                erhöht das Netto.",
            "nl" => "Tijdelijke verlaging van 0,80 pt van de werknemers-IVS-bijdrage, \
                juli–december 2022 bij reddito ≤ 35.000 € (DL 115/2022, omgez. L. 142/2022). Het \
                negatieve bedrag verhoogt het netto.",
            "it" => "Riduzione temporanea di 0,80 pt del contributo IVS dipendente, luglio–\
                dicembre 2022 se reddito ≤ 35.000 € (DL 115/2022, conv. L. 142/2022). L'importo \
                negativo aumenta il netto.",
            "es" => "Reducción temporal de 0,80 pt de la cotización IVS del trabajador, julio–\
                diciembre 2022 si reddito ≤ 35.000 € (DL 115/2022, conv. L. 142/2022). El importe \
                negativo aumenta el neto.",
            _ => return None,
        },
        "IT_ESONERO_2023" => match lang {
            "en" => "Reduction of {taux_pp} pts on employee IVS contribution (2023): −3 pts if \
                reddito ≤ 25,000 €, −2 pts if 25,001–35,000 €. Estimated reddito: {reddito} €/yr \
                → {taux_pp} pts. L. 197/2022 art. 1 c. 281-286.",
            "de" => "Senkung um {taux_pp} Pkt. beim AN-IVS-Beitrag (2023): −3 bei reddito ≤ \
                25.000 €, −2 bei 25.001–35.000 €. Geschätzter reddito: {reddito} €/Jahr → \
                {taux_pp} Pkt. L. 197/2022 Art. 1 Abs. 281-286.",
            "nl" => "Verlaging van {taux_pp} pt op de werknemers-IVS-bijdrage (2023): −3 bij \
                reddito ≤ 25.000 €, −2 bij 25.001–35.000 €. Geschatte reddito: {reddito} €/jr → \
                {taux_pp} pt. L. 197/2022 art. 1 c. 281-286.",
            "it" => "Riduzione di {taux_pp} pt sul contributo IVS dipendente (2023): −3 se \
                reddito ≤ 25.000 €, −2 se 25.001–35.000 €. Reddito stimato: {reddito} €/anno → \
                {taux_pp} pt. L. 197/2022 art. 1 c. 281-286.",
            "es" => "Reducción de {taux_pp} pt en la cotización IVS del trabajador (2023): −3 si \
                reddito ≤ 25.000 €, −2 si 25.001–35.000 €. Reddito estimado: {reddito} €/año → \
                {taux_pp} pt. L. 197/2022 art. 1 c. 281-286.",
            _ => return None,
        },
        "IT_ESONERO_2024" => match lang {
            "en" => "Reduction of {taux_pp} pts on employee IVS (2024): −7 if reddito ≤ 25,000 €, \
                −6 if 25,001–35,000 €. Estimated reddito: {reddito} €/yr → {taux_pp} pts. \
                L. 213/2023 art. 1 cc. 15-17.",
            "de" => "Senkung um {taux_pp} Pkt. (2024): −7 bei reddito ≤ 25.000 €, −6 bei \
                25.001–35.000 €. Geschätzter reddito: {reddito} €/Jahr → {taux_pp} Pkt. \
                L. 213/2023 Art. 1 cc. 15-17.",
            "nl" => "Verlaging van {taux_pp} pt (2024): −7 bij reddito ≤ 25.000 €, −6 bij \
                25.001–35.000 €. Geschatte reddito: {reddito} €/jr → {taux_pp} pt. L. 213/2023 \
                art. 1 cc. 15-17.",
            "it" => "Riduzione di {taux_pp} pt (2024): −7 se reddito ≤ 25.000 €, −6 se \
                25.001–35.000 €. Reddito stimato: {reddito} €/anno → {taux_pp} pt. L. 213/2023 \
                art. 1 cc. 15-17.",
            "es" => "Reducción de {taux_pp} pt (2024): −7 si reddito ≤ 25.000 €, −6 si \
                25.001–35.000 €. Reddito estimado: {reddito} €/año → {taux_pp} pt. L. 213/2023 \
                art. 1 cc. 15-17.",
            _ => return None,
        },
        "IT_BONUS_CUNEO" => match lang {
            "en" => "Monthly tax benefit paid by the employer (sostituto d'imposta) under the \
                taglio del cuneo fiscale. {annee}: {desc} Estimated annual reddito: {reddito} € \
                → annual bonus: {bonus_a} € → monthly: {bonus_m} €. Paid directly on the payslip \
                (advance on tax credit) and reconciled in the annual 730/Redditi return.",
            "de" => "Monatlicher Steuervorteil, vom Arbeitgeber (sostituto d'imposta) im Rahmen \
                des taglio del cuneo fiscale gezahlt. {annee}: {desc} Geschätzter Jahres-reddito: \
                {reddito} € → Jahresbonus: {bonus_a} € → monatlich: {bonus_m} €. Direkt auf der \
                Lohnabrechnung gezahlt (Vorschuss auf Steuergutschrift) und in der jährlichen \
                730/Redditi-Erklärung abgerechnet.",
            "nl" => "Maandelijks belastingvoordeel betaald door de werkgever (sostituto \
                d'imposta) in het kader van de taglio del cuneo fiscale. {annee}: {desc} Geschatte \
                jaarlijkse reddito: {reddito} € → jaarbonus: {bonus_a} € → maandelijks: \
                {bonus_m} €. Rechtstreeks op de loonbrief betaald (voorschot op belastingkrediet) \
                en verrekend in de jaarlijkse 730/Redditi-aangifte.",
            "it" => "Beneficio fiscale mensile erogato dal datore (sostituto d'imposta) a titolo \
                di taglio del cuneo fiscale. {annee}: {desc} Reddito annuo stimato: {reddito} € → \
                bonus annuo: {bonus_a} € → mensile: {bonus_m} €. Erogato direttamente in busta \
                paga (anticipo su credito d'imposta) e conguagliato nella dichiarazione annuale \
                730/Redditi.",
            "es" => "Beneficio fiscal mensual abonado por el empleador (sostituto d'imposta) en \
                concepto de taglio del cuneo fiscale. {annee}: {desc} Reddito anual estimado: \
                {reddito} € → bono anual: {bonus_a} € → mensual: {bonus_m} €. Abonado directamente \
                en la nómina (anticipo de crédito fiscal) y regularizado en la declaración anual \
                730/Redditi.",
            _ => return None,
        },
        "IT_BONUS_CUNEO_DESC_2024" => match lang {
            "en" => "L. 213/2023: bonus 1,200 €/yr for reddito ≤ 35,000 €.",
            "de" => "L. 213/2023: Bonus 1.200 €/Jahr bei reddito ≤ 35.000 €.",
            "nl" => "L. 213/2023: bonus 1.200 €/jr bij reddito ≤ 35.000 €.",
            "it" => "L. 213/2023: bonus 1.200 €/anno per reddito ≤ 35.000 €.",
            "es" => "L. 213/2023: bono 1.200 €/año para reddito ≤ 35.000 €.",
            _ => return None,
        },
        "IT_BONUS_CUNEO_DESC_2025" => match lang {
            "en" => "L. 207/2024: bonus 7.1 % × reddito (max 1,400 €) if reddito ≤ 20,000 €; \
                fixed detrazione 1,000 € if reddito 20,001–40,000 €.",
            "de" => "L. 207/2024: Bonus 7,1 % × reddito (max. 1.400 €) bei reddito ≤ 20.000 €; \
                feste detrazione 1.000 € bei reddito 20.001–40.000 €.",
            "nl" => "L. 207/2024: bonus 7,1 % × reddito (max 1.400 €) bij reddito ≤ 20.000 €; \
                vaste detrazione 1.000 € bij reddito 20.001–40.000 €.",
            "it" => "L. 207/2024: bonus 7,1 % × reddito (max 1.400 €) se reddito ≤ 20.000 €; \
                detrazione fissa 1.000 € se reddito 20.001–40.000 €.",
            "es" => "L. 207/2024: bono 7,1 % × reddito (máx 1.400 €) si reddito ≤ 20.000 €; \
                detrazione fija 1.000 € si reddito 20.001–40.000 €.",
            _ => return None,
        },
        "IT_IRPEF" => match lang {
            "en" => "IRPEF (Imposta sul Reddito delle Persone Fisiche) is the progressive income \
                tax governed by the TUIR (DPR 917/1986). The employer (sostituto d'imposta) makes \
                a monthly estimated withholding, reconciled via the 730/Redditi return. \
                [ Calculation {annee} — {nb_tranches}-band scale ] Estimated annual income: \
                {reddito} €; gross annual IRPEF: {irpef_b} €; employee detrazione: − {det} €; net \
                annual IRPEF: {irpef_n} €; monthly IRPEF: {irpef_m} € (÷ 12); effective rate: \
                {teff} %. Regional and municipal addizionali are computed separately.",
            "de" => "IRPEF (Imposta sul Reddito delle Persone Fisiche) ist die progressive \
                Einkommensteuer nach dem TUIR (DPR 917/1986). Der Arbeitgeber (sostituto \
                d'imposta) nimmt einen monatlichen Schätzabzug vor, abgerechnet über die \
                730/Redditi-Erklärung. [ Berechnung {annee} — Tarif mit {nb_tranches} Stufen ] \
                Geschätztes Jahreseinkommen: {reddito} €; Brutto-IRPEF/Jahr: {irpef_b} €; \
                detrazione: − {det} €; Netto-IRPEF/Jahr: {irpef_n} €; IRPEF/Monat: {irpef_m} € \
                (÷ 12); Effektivsatz: {teff} %. Regionale und kommunale addizionali werden separat \
                berechnet.",
            "nl" => "IRPEF (Imposta sul Reddito delle Persone Fisiche) is de progressieve \
                inkomstenbelasting volgens de TUIR (DPR 917/1986). De werkgever (sostituto \
                d'imposta) houdt maandelijks een geschat bedrag in, verrekend via de \
                730/Redditi-aangifte. [ Berekening {annee} — schaal met {nb_tranches} schijven ] \
                Geschat jaarinkomen: {reddito} €; bruto IRPEF/jaar: {irpef_b} €; detrazione: \
                − {det} €; netto IRPEF/jaar: {irpef_n} €; IRPEF/maand: {irpef_m} € (÷ 12); \
                effectief tarief: {teff} %. Regionale en gemeentelijke addizionali worden apart \
                berekend.",
            "it" => "L'IRPEF (Imposta sul Reddito delle Persone Fisiche) è l'imposta progressiva \
                sul reddito, disciplinata dal TUIR (DPR 917/1986). Il datore (sostituto d'imposta) \
                effettua una ritenuta mensile stimata, conguagliata con il modello 730/Redditi. \
                [ Calcolo {annee} — scaglioni {nb_tranches} ] Reddito annuo stimato: {reddito} €; \
                IRPEF lorda annua: {irpef_b} €; detrazione lavoro dip.: − {det} €; IRPEF netta \
                annua: {irpef_n} €; IRPEF mensile: {irpef_m} € (÷ 12); aliquota effettiva: \
                {teff} %. Le addizionali regionale e comunale sono calcolate separatamente.",
            "es" => "El IRPEF (Imposta sul Reddito delle Persone Fisiche) es el impuesto \
                progresivo sobre la renta, regido por el TUIR (DPR 917/1986). El empleador \
                (sostituto d'imposta) practica una retención mensual estimada, regularizada con el \
                modelo 730/Redditi. [ Cálculo {annee} — escala de {nb_tranches} tramos ] Reddito \
                anual estimado: {reddito} €; IRPEF bruto anual: {irpef_b} €; detrazione: − {det} €; \
                IRPEF neto anual: {irpef_n} €; IRPEF mensual: {irpef_m} € (÷ 12); tipo efectivo: \
                {teff} %. Las addizionali regional y municipal se calculan por separado.",
            _ => return None,
        },
        "IT_ADD_REG" => match lang {
            "en" => "The regional IRPEF addizionale is a tax levied by the Region on top of \
                national IRPEF (art. 50 D.Lgs. 446/1997), with a rate set yearly by the Region. \
                Region: {libelle} (code {code}); base rate: {taux_pct} %; estimated annual reddito: \
                {reddito} €; annual addizionale: {addiz_a} €; indicative monthly: {addiz_m} €. In \
                practice withheld over 11 months (Mar–Nov of year N+1).",
            "de" => "Die regionale IRPEF-addizionale ist eine Steuer der Region zusätzlich zur \
                nationalen IRPEF (Art. 50 D.Lgs. 446/1997), mit jährlich von der Region \
                festgelegtem Satz. Region: {libelle} (Code {code}); Basissatz: {taux_pct} %; \
                geschätzter Jahres-reddito: {reddito} €; addizionale/Jahr: {addiz_a} €; \
                Richtwert/Monat: {addiz_m} €. In der Praxis über 11 Monate einbehalten \
                (Mär.–Nov. des Jahres N+1).",
            "nl" => "De regionale IRPEF-addizionale is een belasting van de Regio bovenop de \
                nationale IRPEF (art. 50 D.Lgs. 446/1997), met een tarief dat de Regio jaarlijks \
                vaststelt. Regio: {libelle} (code {code}); basistarief: {taux_pct} %; geschatte \
                jaarlijkse reddito: {reddito} €; addizionale/jaar: {addiz_a} €; indicatief/maand: \
                {addiz_m} €. In de praktijk ingehouden over 11 maanden (mrt–nov van jaar N+1).",
            "it" => "L'addizionale regionale IRPEF è un'imposta prelevata dalla Regione in \
                aggiunta all'IRPEF nazionale (art. 50 D.Lgs. 446/1997), con aliquota fissata \
                annualmente dalla Regione. Regione: {libelle} (codice {code}); aliquota base: \
                {taux_pct} %; reddito annuo stimato: {reddito} €; addizionale annua: {addiz_a} €; \
                mensile indicativa: {addiz_m} €. In pratica trattenuta in 11 mensilità \
                (mar.–nov. dell'anno N+1).",
            "es" => "El addizionale regional del IRPEF es un impuesto recaudado por la Región \
                además del IRPEF nacional (art. 50 D.Lgs. 446/1997), con un tipo fijado anualmente \
                por la Región. Región: {libelle} (código {code}); tipo base: {taux_pct} %; reddito \
                anual estimado: {reddito} €; addizionale anual: {addiz_a} €; mensual indicativa: \
                {addiz_m} €. En la práctica se retiene en 11 mensualidades (mar.–nov. del año N+1).",
            _ => return None,
        },
        _ => return None,
    })
}
