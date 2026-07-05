// Traductions Brésil (codes `BR_*`). Placeholders préservés (substitués par
// l'appelant) : {teto} {inss} {teff} {base} {taux} {irrf} {tp}.
// « INSS », « IRRF », « FGTS » sont des sigles conservés.

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "BR_INSS" => match lang {
            "en" => "INSS — Social security", "de" => "INSS — Sozialversicherung",
            "nl" => "INSS — Sociale zekerheid", "it" => "INSS — Previdenza sociale",
            "es" => "INSS — Seguridad social", _ => return None,
        },
        "BR_IRRF" => match lang {
            "en" => "IRRF — Withholding income tax", "de" => "IRRF — Einbehaltene Einkommensteuer",
            "nl" => "IRRF — Ingehouden inkomstenbelasting", "it" => "IRRF — Ritenuta d'imposta sul reddito",
            "es" => "IRRF — Impuesto sobre la renta retenido", _ => return None,
        },
        "BR_INSS_PAT" => match lang {
            "en" => "INSS — Employer contribution", "de" => "INSS — Arbeitgeberbeitrag",
            "nl" => "INSS — Werkgeversbijdrage", "it" => "INSS — Contributo del datore",
            "es" => "INSS — Aportación patronal", _ => return None,
        },
        "BR_FGTS" => match lang {
            "en" => "FGTS — Severance guarantee fund (employer)", "de" => "FGTS — Abfindungsgarantiefonds (Arbeitgeber)",
            "nl" => "FGTS — Ontslaggarantiefonds (werkgever)", "it" => "FGTS — Fondo di garanzia (datore)",
            "es" => "FGTS — Fondo de garantía (empleador)", _ => return None,
        },
        _ => return None,
    })
}

pub fn t_explication(key: &str, lang: &str) -> Option<&'static str> {
    Some(match key {
        "BR_INSS" => match lang {
            "en" => "INSS (National Social Security Institute) — progressive employee contribution by brackets \
                (7.5 / 9 / 12 / 14 %), capped at the teto of {teto} R$. Contribution {inss} R$ (effective rate {teff} %). \
                Legal basis: Lei 8.212/1991.",
            "de" => "INSS (nationales Sozialversicherungsinstitut) — progressiver Arbeitnehmerbeitrag nach Stufen \
                (7,5 / 9 / 12 / 14 %), gedeckelt beim teto von {teto} R$. Beitrag {inss} R$ (effektiver Satz {teff} %). \
                Rechtsgrundlage: Lei 8.212/1991.",
            "nl" => "INSS (nationaal socialezekerheidsinstituut) — progressieve werknemersbijdrage per schijf \
                (7,5 / 9 / 12 / 14 %), gemaximeerd op het teto van {teto} R$. Bijdrage {inss} R$ (effectief tarief {teff} %). \
                Wettelijke basis: Lei 8.212/1991.",
            "it" => "INSS (istituto nazionale di previdenza sociale) — contributo dipendente progressivo per scaglioni \
                (7,5 / 9 / 12 / 14 %), con massimale al teto di {teto} R$. Contributo {inss} R$ (aliquota effettiva {teff} %). \
                Base giuridica: Lei 8.212/1991.",
            "es" => "INSS (Instituto Nacional del Seguro Social) — aportación del empleado progresiva por tramos \
                (7,5 / 9 / 12 / 14 %), con tope en el teto de {teto} R$. Aportación {inss} R$ (tipo efectivo {teff} %). \
                Base legal: Lei 8.212/1991.",
            _ => return None,
        },
        "BR_IRRF" => match lang {
            "en" => "IRRF (income tax withheld at source) — base = gross − max(INSS, simplified discount 564.80 R$) = {base} R$; \
                progressive monthly scale, marginal bracket {taux} %. Tax {irrf} R$. Legal basis: Lei 7.713/1988.",
            "de" => "IRRF (an der Quelle einbehaltene Einkommensteuer) — Basis = brutto − max(INSS, Pauschalabzug 564,80 R$) = {base} R$; \
                progressive Monatstabelle, Grenzstufe {taux} %. Steuer {irrf} R$. Rechtsgrundlage: Lei 7.713/1988.",
            "nl" => "IRRF (aan de bron ingehouden inkomstenbelasting) — grondslag = bruto − max(INSS, forfaitaire aftrek 564,80 R$) = {base} R$; \
                progressieve maandtabel, marginale schijf {taux} %. Belasting {irrf} R$. Wettelijke basis: Lei 7.713/1988.",
            "it" => "IRRF (imposta sul reddito trattenuta alla fonte) — base = lordo − max(INSS, sconto forfettario 564,80 R$) = {base} R$; \
                scala mensile progressiva, scaglione marginale {taux} %. Imposta {irrf} R$. Base giuridica: Lei 7.713/1988.",
            "es" => "IRRF (impuesto sobre la renta retenido en la fuente) — base = bruto − max(INSS, descuento simplificado 564,80 R$) = {base} R$; \
                tabla mensual progresiva, tramo marginal {taux} %. Impuesto {irrf} R$. Base legal: Lei 7.713/1988.",
            _ => return None,
        },
        "BR_INSS_PAT" => match lang {
            "en" => "Employer INSS — 20 % on payroll. RAT (risk) and terceiros (Sistema S) — up to +8.8 % — not detailed. \
                Rate {tp} % × {base} R$. Legal basis: Lei 8.212/1991 art. 22.",
            "de" => "Arbeitgeber-INSS — 20 % auf die Lohnsumme. RAT (Risiko) und terceiros (Sistema S) — bis +8,8 % — nicht detailliert. \
                Satz {tp} % × {base} R$. Rechtsgrundlage: Lei 8.212/1991 Art. 22.",
            "nl" => "Werkgevers-INSS — 20 % over de loonsom. RAT (risico) en terceiros (Sistema S) — tot +8,8 % — niet uitgesplitst. \
                Tarief {tp} % × {base} R$. Wettelijke basis: Lei 8.212/1991 art. 22.",
            "it" => "INSS datore — 20 % sul monte salari. RAT (rischio) e terceiros (Sistema S) — fino a +8,8 % — non dettagliati. \
                Aliquota {tp} % × {base} R$. Base giuridica: Lei 8.212/1991 art. 22.",
            "es" => "INSS patronal — 20 % sobre la masa salarial. RAT (riesgo) y terceiros (Sistema S) — hasta +8,8 % — no detallados. \
                Tipo {tp} % × {base} R$. Base legal: Lei 8.212/1991 art. 22.",
            _ => return None,
        },
        "BR_FGTS" => match lang {
            "en" => "FGTS (Length-of-Service Guarantee Fund) — 8 % employer deposited to the worker's linked account \
                (does not reduce net pay; released on dismissal/home purchase). Rate {tp} % × {base} R$. Legal basis: Lei 8.036/1990.",
            "de" => "FGTS (Garantiefonds für Dienstzeit) — 8 % Arbeitgeber auf das verknüpfte Konto des Arbeitnehmers \
                (mindert den Nettolohn nicht; frei bei Kündigung/Immobilienkauf). Satz {tp} % × {base} R$. Rechtsgrundlage: Lei 8.036/1990.",
            "nl" => "FGTS (garantiefonds diensttijd) — 8 % werkgever gestort op de gekoppelde rekening van de werknemer \
                (verlaagt het netto niet; vrij bij ontslag/woningaankoop). Tarief {tp} % × {base} R$. Wettelijke basis: Lei 8.036/1990.",
            "it" => "FGTS (fondo di garanzia per l'anzianità) — 8 % datore versato sul conto collegato del lavoratore \
                (non riduce il netto; sbloccato al licenziamento/acquisto casa). Aliquota {tp} % × {base} R$. Base giuridica: Lei 8.036/1990.",
            "es" => "FGTS (Fondo de Garantía por Tiempo de Servicio) — 8 % empleador depositado en la cuenta vinculada del trabajador \
                (no reduce el neto; liberado al despido/compra de vivienda). Tipo {tp} % × {base} R$. Base legal: Lei 8.036/1990.",
            _ => return None,
        },
        _ => return None,
    })
}
