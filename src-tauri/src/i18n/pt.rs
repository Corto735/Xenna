// Traductions Portugal (codes `PT_*`). Placeholders nommés :
// {ts_pct} {tp_pct} {brut} {ms} {mp} {total} {tot} (cotisations) ;
// {annee} {nb_tr} {rend_a} {ded} {ss} {df} {base_irs} {irs_a} {irs_m} {teff} (IRS).

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "PT_AT_SEG" => match lang {
            "en" => "Acidentes de Trabalho — work accident insurance",
            "de" => "Acidentes de Trabalho — Arbeitsunfallversicherung",
            "nl" => "Acidentes de Trabalho — arbeidsongevallenverzekering",
            "it" => "Acidentes de Trabalho — assicurazione infortuni sul lavoro",
            "es" => "Acidentes de Trabalho — seguro de accidentes de trabajo",
            _ => return None,
        },
        "PT_IRS" => match lang {
            "en" => "IRS — Withholding {annee}",
            "de" => "IRS — Quellensteuer {annee}",
            "nl" => "IRS — Inhouding {annee}",
            "it" => "IRS — Ritenuta alla fonte {annee}",
            "es" => "IRS — Retención en origen {annee}",
            _ => return None,
        },
        _ => return None,
    })
}

pub fn t_explication(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "PT_SS" => match lang {
            "en" => "Main general-scheme contribution (TSU): sickness/maternity, disability, \
                pension, survivors, unemployment. Base = full gross, no cap.\nEmployee: {ts_pct} % \
                × {brut} € = {ms} € | Employer: {tp_pct} % = {mp} € | Total: {total} % = {tot} €.\n\
                Stable since 2013. Lei 110/2009 art. 53-54.",
            "de" => "Hauptbeitrag des Allgemeinsystems (TSU): Kranken/Mutterschaft, Invalidität, \
                Rente, Hinterbliebene, Arbeitslosigkeit. Bemessung = volles Brutto, ohne \
                Obergrenze.\nArbeitnehmer: {ts_pct} % × {brut} € = {ms} € | Arbeitgeber: {tp_pct} % \
                = {mp} € | Gesamt: {total} % = {tot} €.\nStabil seit 2013. Lei 110/2009 art. 53-54.",
            "nl" => "Hoofdbijdrage van het algemene stelsel (TSU): ziekte/moederschap, \
                invaliditeit, pensioen, nabestaanden, werkloosheid. Grondslag = volledig bruto, \
                geen plafond.\nWerknemer: {ts_pct} % × {brut} € = {ms} € | Werkgever: {tp_pct} % = \
                {mp} € | Totaal: {total} % = {tot} €.\nStabiel sinds 2013. Lei 110/2009 art. 53-54.",
            "it" => "Contributo principale del regime generale (TSU): malattia/maternità, \
                invalidità, pensione, superstiti, disoccupazione. Base = lordo intero, senza \
                massimale.\nDipendente: {ts_pct} % × {brut} € = {ms} € | Datore: {tp_pct} % = {mp} € \
                | Totale: {total} % = {tot} €.\nStabile dal 2013. Lei 110/2009 art. 53-54.",
            "es" => "Cotización principal del régimen general (TSU): enfermedad/maternidad, \
                invalidez, jubilación, supervivencia, desempleo. Base = bruto íntegro, sin tope.\n\
                Trabajador: {ts_pct} % × {brut} € = {ms} € | Empleador: {tp_pct} % = {mp} € | Total: \
                {total} % = {tot} €.\nEstable desde 2013. Lei 110/2009 art. 53-54.",
            _ => return None,
        },
        "PT_AT_SEG" => match lang {
            "en" => "Mandatory work-accident and occupational-disease insurance, 100 % employer. \
                Rate {tp_pct} % indicative (services); from 0.5 % (office) to 10 %+ (construction). \
                Employer: {mp} €. Lei 98/2009 art. 79.",
            "de" => "Pflicht-Arbeitsunfall- und Berufskrankheitenversicherung, 100 % Arbeitgeber. \
                Satz {tp_pct} % indikativ (Dienstleistung); von 0,5 % (Büro) bis 10 %+ (Bau). \
                Arbeitgeber: {mp} €. Lei 98/2009 art. 79.",
            "nl" => "Verplichte arbeidsongevallen- en beroepsziekteverzekering, 100 % werkgever. \
                Tarief {tp_pct} % indicatief (diensten); van 0,5 % (kantoor) tot 10 %+ (bouw). \
                Werkgever: {mp} €. Lei 98/2009 art. 79.",
            "it" => "Assicurazione obbligatoria infortuni sul lavoro e malattie professionali, \
                100 % datoriale. Aliquota {tp_pct} % indicativa (terziario); da 0,5 % (ufficio) a \
                10 %+ (edilizia). Datore: {mp} €. Lei 98/2009 art. 79.",
            "es" => "Seguro obligatorio de accidentes de trabajo y enfermedades profesionales, \
                100 % patronal. Tipo {tp_pct} % indicativo (servicios); de 0,5 % (oficina) a 10 %+ \
                (construcción). Empleador: {mp} €. Lei 98/2009 art. 79.",
            _ => return None,
        },
        "PT_FCT" => match lang {
            "en" => "Fund covering 50 % of severance pay on insolvency (permanent contracts after \
                01/10/2013). 100 % employer: {tp_pct} %. Employer: {mp} €. DL 210/2015 art. 4.",
            "de" => "Fonds, der 50 % der Abfindung bei Insolvenz abdeckt (unbefristete Verträge \
                nach dem 01.10.2013). 100 % Arbeitgeber: {tp_pct} %. Arbeitgeber: {mp} €. DL \
                210/2015 art. 4.",
            "nl" => "Fonds dat 50 % van de ontslagvergoeding dekt bij insolventie (vaste contracten \
                na 01-10-2013). 100 % werkgever: {tp_pct} %. Werkgever: {mp} €. DL 210/2015 art. 4.",
            "it" => "Fondo che copre il 50 % dell'indennità di licenziamento in caso di insolvenza \
                (contratti a tempo indeterminato dopo il 01/10/2013). 100 % datoriale: {tp_pct} %. \
                Datore: {mp} €. DL 210/2015 art. 4.",
            "es" => "Fondo que cubre el 50 % de la indemnización por despido en caso de insolvencia \
                (contratos indefinidos posteriores al 01/10/2013). 100 % patronal: {tp_pct} %. \
                Empleador: {mp} €. DL 210/2015 art. 4.",
            _ => return None,
        },
        "PT_FGCT" => match lang {
            "en" => "Fund guaranteeing the remaining 50 % of severance not covered by the FCT. \
                100 % employer: {tp_pct} %. Employer: {mp} €. DL 210/2015 art. 5.",
            "de" => "Fonds, der die restlichen 50 % der vom FCT nicht gedeckten Abfindung \
                garantiert. 100 % Arbeitgeber: {tp_pct} %. Arbeitgeber: {mp} €. DL 210/2015 art. 5.",
            "nl" => "Fonds dat de resterende 50 % van de ontslagvergoeding garandeert die niet door \
                het FCT wordt gedekt. 100 % werkgever: {tp_pct} %. Werkgever: {mp} €. DL 210/2015 \
                art. 5.",
            "it" => "Fondo che garantisce il restante 50 % dell'indennità non coperta dal FCT. \
                100 % datoriale: {tp_pct} %. Datore: {mp} €. DL 210/2015 art. 5.",
            "es" => "Fondo que garantiza el 50 % restante de la indemnización no cubierta por el \
                FCT. 100 % patronal: {tp_pct} %. Empleador: {mp} €. DL 210/2015 art. 5.",
            _ => return None,
        },
        "PT_IRS" => match lang {
            "en" => "Monthly withholding (retenção na fonte) of IRS (personal income tax). The \
                employer (substituto tributário) withholds an advance on the annual IRS, reconciled \
                via the Modelo 3 return (April).\n[ Calculation {annee} — CIRS art. 68 scale, \
                {nb_tr} bands ]\nMonthly gross: {brut} €\nEstimated annual income: {rend_a} € \
                (× 12)\nSpecific deduction: − {ded} € (max(SS {ss} €, flat {df} €))\nAnnual taxable \
                base: {base_irs} €\nAnnual IRS: {irs_a} €\nMonthly withholding: {irs_m} € (÷ 12)\n\
                Effective rate: {teff} %\nApproximation by annualised scale; official AT tables \
                apply. Legal basis: CIRS art. 99 + AT tables {annee}.",
            "de" => "Monatlicher Quellensteuerabzug (retenção na fonte) der IRS (Einkommensteuer). \
                Der Arbeitgeber (substituto tributário) behält einen Vorschuss auf die Jahres-IRS \
                ein, abgerechnet über die Erklärung Modelo 3 (April).\n[ Berechnung {annee} — Tarif \
                CIRS art. 68, {nb_tr} Stufen ]\nMonatsbrutto: {brut} €\nGeschätztes Jahreseinkommen: \
                {rend_a} € (× 12)\nSpezifischer Abzug: − {ded} € (max(SS {ss} €, Pauschale {df} €))\n\
                Jährliche Bemessungsgrundlage: {base_irs} €\nJahres-IRS: {irs_a} €\nMonatlicher \
                Abzug: {irs_m} € (÷ 12)\nEffektivsatz: {teff} %\nNäherung über Jahrestarif; offizielle \
                AT-Tabellen gelten. Rechtsgrundlage: CIRS art. 99 + AT-Tabellen {annee}.",
            "nl" => "Maandelijkse bronheffing (retenção na fonte) van de IRS (inkomstenbelasting). \
                De werkgever (substituto tributário) houdt een voorschot op de jaarlijkse IRS in, \
                verrekend via de aangifte Modelo 3 (april).\n[ Berekening {annee} — schaal CIRS art. \
                68, {nb_tr} schijven ]\nMaandbruto: {brut} €\nGeschat jaarinkomen: {rend_a} € \
                (× 12)\nSpecifieke aftrek: − {ded} € (max(SS {ss} €, forfait {df} €))\nJaarlijkse \
                belastbare basis: {base_irs} €\nJaarlijkse IRS: {irs_a} €\nMaandelijkse inhouding: \
                {irs_m} € (÷ 12)\nEffectief tarief: {teff} %\nBenadering via jaartarief; officiële \
                AT-tabellen gelden. Rechtsgrond: CIRS art. 99 + AT-tabellen {annee}.",
            "it" => "Ritenuta mensile alla fonte (retenção na fonte) dell'IRS (imposta sul \
                reddito). Il datore (substituto tributário) trattiene un anticipo sull'IRS annua, \
                conguagliato con la dichiarazione Modelo 3 (aprile).\n[ Calcolo {annee} — scala CIRS \
                art. 68, {nb_tr} scaglioni ]\nLordo mensile: {brut} €\nReddito annuo stimato: \
                {rend_a} € (× 12)\nDeduzione specifica: − {ded} € (max(SS {ss} €, forfait {df} €))\n\
                Base imponibile annua: {base_irs} €\nIRS annua: {irs_a} €\nRitenuta mensile: \
                {irs_m} € (÷ 12)\nAliquota effettiva: {teff} %\nApprossimazione su scala annualizzata; \
                valgono le tabelle ufficiali AT. Base legale: CIRS art. 99 + tabelle AT {annee}.",
            "es" => "Retención mensual en origen (retenção na fonte) del IRS (impuesto sobre la \
                renta). El empleador (substituto tributário) retiene un anticipo del IRS anual, \
                regularizado con la declaración Modelo 3 (abril).\n[ Cálculo {annee} — escala CIRS \
                art. 68, {nb_tr} tramos ]\nBruto mensual: {brut} €\nRenta anual estimada: {rend_a} € \
                (× 12)\nDeducción específica: − {ded} € (máx(SS {ss} €, fijo {df} €))\nBase imponible \
                anual: {base_irs} €\nIRS anual: {irs_a} €\nRetención mensual: {irs_m} € (÷ 12)\nTipo \
                efectivo: {teff} %\nAproximación por escala anualizada; rigen las tablas oficiales \
                AT. Base legal: CIRS art. 99 + tablas AT {annee}.",
            _ => return None,
        },
        _ => return None,
    })
}
