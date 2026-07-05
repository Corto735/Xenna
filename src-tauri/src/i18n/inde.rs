// Traductions Inde (codes `IN_*`). Placeholders préservés (substitués par
// l'appelant) : {ts} {tp} {base} {regime} {annuel} {imposable} {marginal} {ann} {mens}.
// « EPF », « ESI », « TDS », « Professional Tax » conservés (termes indiens).

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "IN_EPF" => match lang {
            "en" => "EPF — Provident fund", "de" => "EPF — Vorsorgefonds",
            "nl" => "EPF — Voorzorgsfonds", "it" => "EPF — Fondo di previdenza",
            "es" => "EPF — Fondo de previsión", _ => return None,
        },
        "IN_ESI" => match lang {
            "en" => "ESI — Health insurance", "de" => "ESI — Krankenversicherung",
            "nl" => "ESI — Ziektekostenverzekering", "it" => "ESI — Assicurazione malattia",
            "es" => "ESI — Seguro de enfermedad", _ => return None,
        },
        "IN_PT" => match lang {
            "en" => "Professional Tax (Karnataka state)", "de" => "Berufssteuer (Karnataka)",
            "nl" => "Beroepsbelasting (Karnataka)", "it" => "Imposta professionale (Karnataka)",
            "es" => "Impuesto profesional (Karnataka)", _ => return None,
        },
        "IN_IMPOT" => match lang {
            "en" => "Income tax (TDS)", "de" => "Einkommensteuer (TDS)",
            "nl" => "Inkomstenbelasting (TDS)", "it" => "Imposta sul reddito (TDS)",
            "es" => "Impuesto sobre la renta (TDS)", _ => return None,
        },
        _ => return None,
    })
}

pub fn t_explication(key: &str, lang: &str) -> Option<&'static str> {
    Some(match key {
        "IN_EPF" => match lang {
            "en" => "EPF (Employees' Provident Fund) — employee {ts} % + employer {tp} % on the base capped at the \
                statutory minimum of 15,000 INR (many employers contribute above it). Base {base} INR. Legal basis: EPF & MP Act 1952.",
            "de" => "EPF (Vorsorgefonds der Arbeitnehmer) — Arbeitnehmer {ts} % + Arbeitgeber {tp} % auf die beim \
                gesetzlichen Minimum von 15.000 INR gedeckelte Basis. Basis {base} INR. Rechtsgrundlage: EPF & MP Act 1952.",
            "nl" => "EPF (voorzorgsfonds werknemers) — werknemer {ts} % + werkgever {tp} % over de grondslag, gemaximeerd op \
                het wettelijke minimum van 15.000 INR. Grondslag {base} INR. Wettelijke basis: EPF & MP Act 1952.",
            "it" => "EPF (fondo di previdenza dei dipendenti) — dipendente {ts} % + datore {tp} % sulla base con massimale \
                al minimo legale di 15.000 INR. Base {base} INR. Base giuridica: EPF & MP Act 1952.",
            "es" => "EPF (fondo de previsión de los empleados) — empleado {ts} % + empleador {tp} % sobre la base con tope \
                en el mínimo legal de 15 000 INR. Base {base} INR. Base legal: EPF & MP Act 1952.",
            _ => return None,
        },
        "IN_ESI" => match lang {
            "en" => "ESI (Employees' State Insurance) — health/maternity insurance, due if monthly gross ≤ 21,000 INR. \
                Employee {ts} % + employer {tp} % × {base} INR. Legal basis: ESI Act 1948.",
            "de" => "ESI (staatliche Arbeitnehmerversicherung) — Kranken-/Mutterschaftsversicherung, fällig bei Monatsbrutto ≤ 21.000 INR. \
                Arbeitnehmer {ts} % + Arbeitgeber {tp} % × {base} INR. Rechtsgrundlage: ESI Act 1948.",
            "nl" => "ESI (staatsverzekering werknemers) — ziekte-/moederschapsverzekering, verschuldigd bij maandbruto ≤ 21.000 INR. \
                Werknemer {ts} % + werkgever {tp} % × {base} INR. Wettelijke basis: ESI Act 1948.",
            "it" => "ESI (assicurazione statale dei dipendenti) — assicurazione malattia/maternità, dovuta se lordo mensile ≤ 21.000 INR. \
                Dipendente {ts} % + datore {tp} % × {base} INR. Base giuridica: ESI Act 1948.",
            "es" => "ESI (seguro estatal de los empleados) — seguro de enfermedad/maternidad, debido si el bruto mensual ≤ 21 000 INR. \
                Empleado {ts} % + empleador {tp} % × {base} INR. Base legal: ESI Act 1948.",
            _ => return None,
        },
        "IN_PT" => match lang {
            "en" => "Professional Tax — a state levy on the exercise of a profession. Karnataka: flat 200 INR/month above \
                25,000 INR of salary (amount and threshold vary by state). Legal basis: Karnataka Tax on Professions Act 1976.",
            "de" => "Berufssteuer — eine bundesstaatliche Abgabe auf die Berufsausübung. Karnataka: pauschal 200 INR/Monat über \
                25.000 INR Gehalt (Betrag und Schwelle je Bundesstaat verschieden). Rechtsgrundlage: Karnataka Tax on Professions Act 1976.",
            "nl" => "Beroepsbelasting — een deelstaatheffing op de beroepsuitoefening. Karnataka: forfaitair 200 INR/maand boven \
                25.000 INR salaris (bedrag en drempel verschillen per staat). Wettelijke basis: Karnataka Tax on Professions Act 1976.",
            "it" => "Imposta professionale — un prelievo statale sull'esercizio di una professione. Karnataka: forfait 200 INR/mese oltre \
                25.000 INR di salario (importo e soglia variano per stato). Base giuridica: Karnataka Tax on Professions Act 1976.",
            "es" => "Impuesto profesional — un gravamen estatal sobre el ejercicio de una profesión. Karnataka: 200 INR/mes fijos por encima \
                de 25 000 INR de salario (importe y umbral varían según el estado). Base legal: Karnataka Tax on Professions Act 1976.",
            _ => return None,
        },
        "IN_IMPOT" => match lang {
            "en" => "Income tax (monthly TDS), {regime}.\nAnnualised income: {annuel} INR\n\
                − standard deduction, taxable income {imposable} INR\nMarginal bracket {marginal} %\n\
                Annual tax (incl. 4 % cess): {ann} INR → monthly {mens} INR\n87A rebate applied below threshold. Legal basis: Income-tax Act 1961.",
            "de" => "Einkommensteuer (monatlicher TDS), {regime}.\nHochgerechnetes Einkommen: {annuel} INR\n\
                − Pauschalabzug, zu versteuerndes Einkommen {imposable} INR\nGrenzstufe {marginal} %\n\
                Jahressteuer (inkl. 4 % Cess): {ann} INR → monatlich {mens} INR\n87A-Rabatt unter der Schwelle. Rechtsgrundlage: Income-tax Act 1961.",
            "nl" => "Inkomstenbelasting (maandelijkse TDS), {regime}.\nGeannualiseerd inkomen: {annuel} INR\n\
                − standaardaftrek, belastbaar inkomen {imposable} INR\nMarginale schijf {marginal} %\n\
                Jaarbelasting (incl. 4 % cess): {ann} INR → maandelijks {mens} INR\n87A-korting onder de drempel. Wettelijke basis: Income-tax Act 1961.",
            "it" => "Imposta sul reddito (TDS mensile), {regime}.\nReddito annualizzato: {annuel} INR\n\
                − deduzione standard, reddito imponibile {imposable} INR\nScaglione marginale {marginal} %\n\
                Imposta annua (incl. cess 4 %): {ann} INR → mensile {mens} INR\nDetrazione 87A sotto soglia. Base giuridica: Income-tax Act 1961.",
            "es" => "Impuesto sobre la renta (TDS mensual), {regime}.\nRenta anualizada: {annuel} INR\n\
                − deducción estándar, renta imponible {imposable} INR\nTramo marginal {marginal} %\n\
                Impuesto anual (incl. cess 4 %): {ann} INR → mensual {mens} INR\nRebaja 87A por debajo del umbral. Base legal: Income-tax Act 1961.",
            _ => return None,
        },
        _ => return None,
    })
}
