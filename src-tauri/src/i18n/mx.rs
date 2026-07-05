// Traductions Mexique (codes `MX_*`). Placeholders préservés (substitués par
// l'appelant) : {ts} {tp} {base} {ms} {uma} {annee} {li} {taux} {isr} {sub} {isrnet}.
// « IMSS », « INFONAVIT », « SAR », « ISR », « UMA » sont des noms propres conservés.

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "MX_IMSS" => match lang {
            "en" => "IMSS — Employee contributions", "de" => "IMSS — Arbeitnehmerbeiträge",
            "nl" => "IMSS — Werknemersbijdragen", "it" => "IMSS — Contributi del dipendente",
            "es" => "IMSS — Cuotas obrero", _ => return None,
        },
        "MX_IMSS_EXC" => match lang {
            "en" => "IMSS — Excess (> 3 UMA)", "de" => "IMSS — Überschuss (> 3 UMA)",
            "nl" => "IMSS — Overschot (> 3 UMA)", "it" => "IMSS — Eccedenza (> 3 UMA)",
            "es" => "IMSS — Excedente (> 3 UMA)", _ => return None,
        },
        "MX_ISR" => match lang {
            "en" => "ISR — Income tax (withholding {annee})", "de" => "ISR — Einkommensteuer (Einbehalt {annee})",
            "nl" => "ISR — Inkomstenbelasting (inhouding {annee})", "it" => "ISR — Imposta sul reddito (ritenuta {annee})",
            "es" => "ISR — Impuesto sobre la renta (retención {annee})", _ => return None,
        },
        "MX_INFONAVIT" => match lang {
            "en" => "INFONAVIT — Housing (employer)", "de" => "INFONAVIT — Wohnen (Arbeitgeber)",
            "nl" => "INFONAVIT — Huisvesting (werkgever)", "it" => "INFONAVIT — Alloggio (datore di lavoro)",
            "es" => "INFONAVIT — Vivienda (empleador)", _ => return None,
        },
        "MX_RETIRO" => match lang {
            "en" => "Retiro (SAR) — Retirement (employer)", "de" => "Retiro (SAR) — Altersvorsorge (Arbeitgeber)",
            "nl" => "Retiro (SAR) — Pensioen (werkgever)", "it" => "Retiro (SAR) — Pensione (datore di lavoro)",
            "es" => "Retiro (SAR) — Jubilación (empleador)", _ => return None,
        },
        _ => return None,
    })
}

pub fn t_explication(key: &str, lang: &str) -> Option<&'static str> {
    Some(match key {
        "MX_IMSS" => match lang {
            "en" => "IMSS (Mexican Social Security Institute) — aggregated employee share: sickness/maternity, \
                disability/life, severance/old-age. Rate {ts} % × {base} $ = {ms} $. Legal basis: Ley del Seguro Social art. 25-36.",
            "de" => "IMSS (mexikanisches Sozialversicherungsinstitut) — aggregierter Arbeitnehmeranteil: Kranken/Mutterschaft, \
                Invalidität/Leben, Abfindung/Alter. Satz {ts} % × {base} $ = {ms} $. Rechtsgrundlage: Ley del Seguro Social art. 25-36.",
            "nl" => "IMSS (Mexicaans socialezekerheidsinstituut) — geaggregeerd werknemersdeel: ziekte/moederschap, \
                invaliditeit/leven, ontslag/ouderdom. Tarief {ts} % × {base} $ = {ms} $. Wettelijke basis: Ley del Seguro Social art. 25-36.",
            "it" => "IMSS (istituto messicano di previdenza sociale) — quota dipendente aggregata: malattia/maternità, \
                invalidità/vita, cessazione/vecchiaia. Aliquota {ts} % × {base} $ = {ms} $. Base giuridica: Ley del Seguro Social art. 25-36.",
            "es" => "IMSS (Instituto Mexicano del Seguro Social) — cuota obrero agregada: enfermedad/maternidad, \
                invalidez/vida, cesantía/vejez. Tipo {ts} % × {base} $ = {ms} $. Base legal: Ley del Seguro Social art. 25-36.",
            _ => return None,
        },
        "MX_EXC" => match lang {
            "en" => "Employee contribution on the pay above 3 UMA ({base} $ = gross − 3 × {uma} $). Rate {ts} % = {ms} $. \
                Legal basis: Ley del Seguro Social art. 106.",
            "de" => "Arbeitnehmerbeitrag auf das Entgelt über 3 UMA ({base} $ = brutto − 3 × {uma} $). Satz {ts} % = {ms} $. \
                Rechtsgrundlage: Ley del Seguro Social art. 106.",
            "nl" => "Werknemersbijdrage op het loon boven 3 UMA ({base} $ = bruto − 3 × {uma} $). Tarief {ts} % = {ms} $. \
                Wettelijke basis: Ley del Seguro Social art. 106.",
            "it" => "Contributo dipendente sulla retribuzione oltre 3 UMA ({base} $ = lordo − 3 × {uma} $). Aliquota {ts} % = {ms} $. \
                Base giuridica: Ley del Seguro Social art. 106.",
            "es" => "Cuota obrero sobre la parte de salario superior a 3 UMA ({base} $ = bruto − 3 × {uma} $). Tipo {ts} % = {ms} $. \
                Base legal: Ley del Seguro Social art. 106.",
            _ => return None,
        },
        "MX_ISR" => match lang {
            "en" => "Income tax (monthly withholding, art. 96 LISR).\nBase: {base} $\nBracket: lower limit {li} $, marginal rate {taux} %\n\
                Gross ISR: {isr} $\n− employment subsidy: {sub} $ (up to 406.83 $ for income ≤ 9,081 $)\nNet ISR: {isrnet} $\n\
                Legal basis: Ley del ISR art. 96; DOF 01/05/2024.",
            "de" => "Einkommensteuer (monatlicher Einbehalt, Art. 96 LISR).\nBasis: {base} $\nStufe: Untergrenze {li} $, Grenzsatz {taux} %\n\
                Brutto-ISR: {isr} $\n− Beschäftigungszuschuss: {sub} $ (bis 406,83 $ bei Einkommen ≤ 9.081 $)\nNetto-ISR: {isrnet} $\n\
                Rechtsgrundlage: Ley del ISR art. 96; DOF 01/05/2024.",
            "nl" => "Inkomstenbelasting (maandelijkse inhouding, art. 96 LISR).\nGrondslag: {base} $\nSchijf: ondergrens {li} $, marginaal tarief {taux} %\n\
                Bruto ISR: {isr} $\n− arbeidssubsidie: {sub} $ (tot 406,83 $ bij inkomen ≤ 9.081 $)\nNetto ISR: {isrnet} $\n\
                Wettelijke basis: Ley del ISR art. 96; DOF 01/05/2024.",
            "it" => "Imposta sul reddito (ritenuta mensile, art. 96 LISR).\nBase: {base} $\nScaglione: limite inferiore {li} $, aliquota marginale {taux} %\n\
                ISR lordo: {isr} $\n− sussidio all'impiego: {sub} $ (fino a 406,83 $ per reddito ≤ 9.081 $)\nISR netto: {isrnet} $\n\
                Base giuridica: Ley del ISR art. 96; DOF 01/05/2024.",
            "es" => "Impuesto sobre la renta (retención mensual, art. 96 LISR).\nBase: {base} $\nTramo: límite inferior {li} $, tipo marginal {taux} %\n\
                ISR bruto: {isr} $\n− subsidio al empleo: {sub} $ (hasta 406,83 $ para ingreso ≤ 9.081 $)\nISR neto: {isrnet} $\n\
                Base legal: Ley del ISR art. 96; DOF 01/05/2024.",
            _ => return None,
        },
        "MX_INFONAVIT" => match lang {
            "en" => "INFONAVIT (National Workers' Housing Fund) — 5 % employer on pay, funds workers' housing. \
                Rate {tp} % × {base} $. Legal basis: Ley del INFONAVIT art. 29.",
            "de" => "INFONAVIT (nationaler Wohnungsfonds der Arbeitnehmer) — 5 % Arbeitgeber auf das Entgelt, \
                finanziert Arbeiterwohnungen. Satz {tp} % × {base} $. Rechtsgrundlage: Ley del INFONAVIT art. 29.",
            "nl" => "INFONAVIT (nationaal huisvestingsfonds voor werknemers) — 5 % werkgever op het loon, \
                financiert werknemershuisvesting. Tarief {tp} % × {base} $. Wettelijke basis: Ley del INFONAVIT art. 29.",
            "it" => "INFONAVIT (fondo nazionale per l'alloggio dei lavoratori) — 5 % datore sulla retribuzione, \
                finanzia l'alloggio dei lavoratori. Aliquota {tp} % × {base} $. Base giuridica: Ley del INFONAVIT art. 29.",
            "es" => "INFONAVIT (Instituto del Fondo Nacional de la Vivienda) — 5 % empleador sobre el salario, \
                financia la vivienda de los trabajadores. Tipo {tp} % × {base} $. Base legal: Ley del INFONAVIT art. 29.",
            _ => return None,
        },
        "MX_RETIRO" => match lang {
            "en" => "Retiro (Retirement Savings System, SAR) — 2 % employer on pay, to the Afore fund. \
                Rate {tp} % × {base} $. Legal basis: Ley del Seguro Social art. 168.",
            "de" => "Retiro (Altersvorsorgesystem, SAR) — 2 % Arbeitgeber auf das Entgelt, an den Afore-Fonds. \
                Satz {tp} % × {base} $. Rechtsgrundlage: Ley del Seguro Social art. 168.",
            "nl" => "Retiro (pensioenspaarsysteem, SAR) — 2 % werkgever op het loon, naar het Afore-fonds. \
                Tarief {tp} % × {base} $. Wettelijke basis: Ley del Seguro Social art. 168.",
            "it" => "Retiro (sistema di risparmio previdenziale, SAR) — 2 % datore sulla retribuzione, verso il fondo Afore. \
                Aliquota {tp} % × {base} $. Base giuridica: Ley del Seguro Social art. 168.",
            "es" => "Retiro (Sistema de Ahorro para el Retiro, SAR) — 2 % empleador sobre el salario, a la Afore. \
                Tipo {tp} % × {base} $. Base legal: Ley del Seguro Social art. 168.",
            _ => return None,
        },
        _ => return None,
    })
}
