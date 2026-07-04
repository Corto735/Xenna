// Traductions Espagne (codes `ES_*`). Placeholders nommés communs :
// {base_min} {base_max} {annee} {base} {ts_pct} {tp_pct} {ms} {mp} {total} {tot} {brut}.

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "ES_CC" => match lang {
            "en" => "Contingencias Comunes — sickness, maternity, pension",
            "de" => "Contingencias Comunes — Kranken, Mutterschaft, Rente",
            "nl" => "Contingencias Comunes — ziekte, moederschap, pensioen",
            "it" => "Contingencias Comunes — malattia, maternità, pensione",
            "es" => "Contingencias Comunes — enfermedad, maternidad, jubilación",
            _ => return None,
        },
        "ES_DESEMPLEO" => match lang {
            "en" => "Desempleo — unemployment insurance (permanent contract)",
            "de" => "Desempleo — Arbeitslosenversicherung (unbefristet)",
            "nl" => "Desempleo — werkloosheidsverzekering (vast contract)",
            "it" => "Desempleo — assicurazione disoccupazione (contratto indeterminato)",
            "es" => "Desempleo — seguro de desempleo (contrato indefinido)",
            _ => return None,
        },
        "ES_FP" => match lang {
            "en" => "Formación Profesional — continuing vocational training",
            "de" => "Formación Profesional — berufliche Weiterbildung",
            "nl" => "Formación Profesional — voortgezette beroepsopleiding",
            "it" => "Formación Profesional — formazione professionale continua",
            "es" => "Formación Profesional — formación profesional continua",
            _ => return None,
        },
        "ES_FOGASA" => match lang {
            "en" => "FOGASA — Wage Guarantee Fund",
            "de" => "FOGASA — Lohngarantiefonds",
            "nl" => "FOGASA — Loongarantiefonds",
            "it" => "FOGASA — Fondo di garanzia salariale",
            "es" => "FOGASA — Fondo de Garantía Salarial",
            _ => return None,
        },
        "ES_MEI" => match lang {
            "en" => "MEI — Intergenerational Equity Mechanism {annee}",
            "de" => "MEI — Mechanismus der Generationengerechtigkeit {annee}",
            "nl" => "MEI — Intergenerationeel gelijkheidsmechanisme {annee}",
            "it" => "MEI — Meccanismo di equità intergenerazionale {annee}",
            "es" => "MEI — Mecanismo de Equidad Intergeneracional {annee}",
            _ => return None,
        },
        _ => return None,
    })
}

pub fn t_explication(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "ES_CC" => match lang {
            "en" => "Main contribution of the Spanish general scheme (sickness, maternity, \
                disability, pension, death/survivors). Base bounded between {base_min} € and \
                {base_max} € in {annee}; applied: {base} €.\nEmployee: {ts_pct} % = {ms} € | \
                Employer: {tp_pct} % = {mp} € | Total: {total} % = {tot} €.\nLGSS art. 143-144. \
                Rates stable since 2015 (4.70 + 23.60 = 28.30 %).",
            "de" => "Hauptbeitrag des spanischen Allgemeinsystems (Kranken, Mutterschaft, \
                Invalidität, Rente, Tod/Hinterbliebene). Bemessung zwischen {base_min} € und \
                {base_max} € in {annee}; angewandt: {base} €.\nArbeitnehmer: {ts_pct} % = {ms} € | \
                Arbeitgeber: {tp_pct} % = {mp} € | Gesamt: {total} % = {tot} €.\nLGSS Art. 143-144. \
                Sätze seit 2015 stabil (4,70 + 23,60 = 28,30 %).",
            "nl" => "Hoofdbijdrage van het Spaanse algemene stelsel (ziekte, moederschap, \
                invaliditeit, pensioen, overlijden/nabestaanden). Grondslag tussen {base_min} € en \
                {base_max} € in {annee}; toegepast: {base} €.\nWerknemer: {ts_pct} % = {ms} € | \
                Werkgever: {tp_pct} % = {mp} € | Totaal: {total} % = {tot} €.\nLGSS art. 143-144. \
                Tarieven stabiel sinds 2015 (4,70 + 23,60 = 28,30 %).",
            "it" => "Contributo principale del regime generale spagnolo (malattia, maternità, \
                invalidità, pensione, morte/superstiti). Base compresa tra {base_min} € e \
                {base_max} € nel {annee}; applicata: {base} €.\nDipendente: {ts_pct} % = {ms} € | \
                Datore: {tp_pct} % = {mp} € | Totale: {total} % = {tot} €.\nLGSS art. 143-144. \
                Aliquote stabili dal 2015 (4,70 + 23,60 = 28,30 %).",
            "es" => "Cotización principal del régimen general español (enfermedad, maternidad, \
                incapacidad, jubilación, muerte/supervivencia). Base acotada entre {base_min} € y \
                {base_max} € en {annee}; aplicada: {base} €.\nTrabajador: {ts_pct} % = {ms} € | \
                Empleador: {tp_pct} % = {mp} € | Total: {total} % = {tot} €.\nLGSS art. 143-144. \
                Tipos estables desde 2015 (4,70 + 23,60 = 28,30 %).",
            _ => return None,
        },
        "ES_DESEMPLEO" => match lang {
            "en" => "Unemployment contribution for permanent contracts (SEPE). Employee: {ts_pct} % \
                × {base} € = {ms} € | Employer: {tp_pct} % = {mp} € | Total: {total} % = {tot} €.\n\
                LGSS art. 270.",
            "de" => "Arbeitslosenbeitrag für unbefristete Verträge (SEPE). Arbeitnehmer: {ts_pct} % \
                × {base} € = {ms} € | Arbeitgeber: {tp_pct} % = {mp} € | Gesamt: {total} % = \
                {tot} €.\nLGSS Art. 270.",
            "nl" => "Werkloosheidsbijdrage voor vaste contracten (SEPE). Werknemer: {ts_pct} % × \
                {base} € = {ms} € | Werkgever: {tp_pct} % = {mp} € | Totaal: {total} % = {tot} €.\n\
                LGSS art. 270.",
            "it" => "Contributo disoccupazione per contratti a tempo indeterminato (SEPE). \
                Dipendente: {ts_pct} % × {base} € = {ms} € | Datore: {tp_pct} % = {mp} € | Totale: \
                {total} % = {tot} €.\nLGSS art. 270.",
            "es" => "Cotización por desempleo para contratos indefinidos (SEPE). Trabajador: \
                {ts_pct} % × {base} € = {ms} € | Empleador: {tp_pct} % = {mp} € | Total: {total} % \
                = {tot} €.\nLGSS art. 270.",
            _ => return None,
        },
        "ES_FOGASA" => match lang {
            "en" => "Wage guarantee fund for unpaid salaries upon employer insolvency. 100 % \
                employer: {tp_pct} % = {mp} €. Stable at 0.20 %. ET (RDL 2/2015) art. 33.",
            "de" => "Lohngarantiefonds für unbezahlte Löhne bei Arbeitgeberinsolvenz. 100 % \
                Arbeitgeber: {tp_pct} % = {mp} €. Stabil bei 0,20 %. ET (RDL 2/2015) Art. 33.",
            "nl" => "Loongarantiefonds voor onbetaalde lonen bij insolventie werkgever. 100 % \
                werkgever: {tp_pct} % = {mp} €. Stabiel op 0,20 %. ET (RDL 2/2015) art. 33.",
            "it" => "Fondo di garanzia salariale per retribuzioni non pagate in caso di insolvenza \
                del datore. 100 % datoriale: {tp_pct} % = {mp} €. Stabile allo 0,20 %. ET (RDL \
                2/2015) art. 33.",
            "es" => "Fondo de garantía salarial para salarios impagados por insolvencia del \
                empleador. 100 % patronal: {tp_pct} % = {mp} €. Estable en 0,20 %. ET (RDL 2/2015) \
                art. 33.",
            _ => return None,
        },
        "ES_FP" => match lang {
            "en" => "Funds continuing vocational training (FUNDAE). Employee: {ts_pct} % — \
                Employer: {tp_pct} % — Total: {total} %.\nEmployee: {ms} € — Employer: {mp} €. LGSS \
                art. 7 and DA 19a.",
            "de" => "Finanziert berufliche Weiterbildung (FUNDAE). Arbeitnehmer: {ts_pct} % — \
                Arbeitgeber: {tp_pct} % — Gesamt: {total} %.\nArbeitnehmer: {ms} € — Arbeitgeber: \
                {mp} €. LGSS Art. 7 und DA 19a.",
            "nl" => "Financiert voortgezette beroepsopleiding (FUNDAE). Werknemer: {ts_pct} % — \
                Werkgever: {tp_pct} % — Totaal: {total} %.\nWerknemer: {ms} € — Werkgever: {mp} €. \
                LGSS art. 7 en DA 19a.",
            "it" => "Finanzia la formazione professionale continua (FUNDAE). Dipendente: {ts_pct} % \
                — Datore: {tp_pct} % — Totale: {total} %.\nDipendente: {ms} € — Datore: {mp} €. \
                LGSS art. 7 e DA 19a.",
            "es" => "Financia la formación profesional continua (FUNDAE). Trabajador: {ts_pct} % — \
                Empleador: {tp_pct} % — Total: {total} %.\nTrabajador: {ms} € — Empleador: {mp} €. \
                LGSS art. 7 y DA 19a.",
            _ => return None,
        },
        "ES_MEI" => match lang {
            "en" => "Additional contribution (Ley 21/2021) feeding the pension Reserve Fund; rate \
                rising until 2032. {annee}: employee {ts_pct} % + employer {tp_pct} % = {total} %.\n\
                Employee: {ms} € — Employer: {mp} €. In force since 01/01/2023.",
            "de" => "Zusatzbeitrag (Ley 21/2021) zur Speisung des Renten-Reservefonds; steigender \
                Satz bis 2032. {annee}: Arbeitnehmer {ts_pct} % + Arbeitgeber {tp_pct} % = \
                {total} %.\nArbeitnehmer: {ms} € — Arbeitgeber: {mp} €. In Kraft seit 01.01.2023.",
            "nl" => "Aanvullende bijdrage (Ley 21/2021) voor het pensioenreservefonds; stijgend \
                tarief tot 2032. {annee}: werknemer {ts_pct} % + werkgever {tp_pct} % = {total} %.\n\
                Werknemer: {ms} € — Werkgever: {mp} €. Van kracht sinds 01-01-2023.",
            "it" => "Contributo aggiuntivo (Ley 21/2021) che alimenta il Fondo di riserva delle \
                pensioni; aliquota crescente fino al 2032. {annee}: dipendente {ts_pct} % + datore \
                {tp_pct} % = {total} %.\nDipendente: {ms} € — Datore: {mp} €. In vigore dal \
                01/01/2023.",
            "es" => "Cotización adicional (Ley 21/2021) que alimenta el Fondo de Reserva de las \
                pensiones; tipo creciente hasta 2032. {annee}: trabajador {ts_pct} % + empleador \
                {tp_pct} % = {total} %.\nTrabajador: {ms} € — Empleador: {mp} €. En vigor desde el \
                01/01/2023.",
            _ => return None,
        },
        _ => return None,
    })
}
