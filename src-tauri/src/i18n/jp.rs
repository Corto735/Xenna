// Traductions Japon (codes `JP_*`). Libellés + explications en/de/nl/it/es.
//
// Convention identique à `divers.rs` : lignes `[&str; 5]` indexées par langue,
// placeholders nommés (`{an}`, `{ts}`, `{plaf}`, …) substitués côté appelant.
// Les termes japonais (健康保険, 所得税…) et les références légales restent intacts.

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    let i = match lang { "en" => 0, "de" => 1, "nl" => 2, "it" => 3, "es" => 4, _ => return None };
    let row: [&str; 5] = match code {
        "JP_KENPO" => ["健康保険 — Health insurance (Kyokai Kenpo Tokyo)", "健康保険 — Krankenversicherung (Kyokai Kenpo Tokyo)", "健康保険 — Ziektekostenverzekering (Kyokai Kenpo Tokyo)", "健康保険 — Assicurazione malattia (Kyokai Kenpo Tokyo)", "健康保険 — Seguro de enfermedad (Kyokai Kenpo Tokyo)"],
        "JP_KAIGO" => ["介護保険 — Long-term care (≥ 40 yrs)", "介護保険 — Pflegeversicherung (≥ 40 J.)", "介護保険 — Langdurige zorg (≥ 40 jr)", "介護保険 — Assistenza a lungo termine (≥ 40 anni)", "介護保険 — Cuidados de larga duración (≥ 40 años)"],
        "JP_KOSEI" => ["厚生年金保険 — Employees' pension insurance", "厚生年金保険 — Arbeitnehmer-Rentenversicherung", "厚生年金保険 — Werknemerspensioenverzekering", "厚生年金保険 — Assicurazione pensione dei lavoratori", "厚生年金保険 — Seguro de pensión de los trabajadores"],
        "JP_KOYO" => ["雇用保険 — Employment insurance (unemployment)", "雇用保険 — Arbeitslosenversicherung", "雇用保険 — Werkloosheidsverzekering", "雇用保険 — Assicurazione disoccupazione", "雇用保険 — Seguro de empleo (desempleo)"],
        "JP_ROUSAI" => ["労災保険 — Work accidents (office)", "労災保険 — Arbeitsunfälle (Büro)", "労災保険 — Arbeidsongevallen (kantoor)", "労災保険 — Infortuni sul lavoro (ufficio)", "労災保険 — Accidentes laborales (oficina)"],
        "JP_SHOTOKUZEI" => ["所得税 — Income tax + reconstruction surtax", "所得税 — Einkommensteuer + Wiederaufbau-Zuschlag", "所得税 — Inkomstenbelasting + wederopbouwtoeslag", "所得税 — Imposta sul reddito + sovrattassa ricostruzione", "所得税 — Impuesto sobre la renta + recargo de reconstrucción"],
        "JP_JUMINZEI" => ["住民税 — Local tax (estimate)", "住民税 — Kommunalsteuer (Schätzung)", "住民税 — Lokale belasting (schatting)", "住民税 — Imposta locale (stima)", "住民税 — Impuesto local (estimación)"],
        _ => return None,
    };
    Some(row[i])
}

pub fn t_explication(key: &str, lang: &str) -> Option<&'static str> {
    let i = match lang { "en" => 0, "de" => 1, "nl" => 2, "it" => 3, "es" => 4, _ => return None };
    let row: [&str; 5] = match key {
        "JP_KENPO" => [
            "Employee health insurance (健康保険) — Kyokai Kenpo Tokyo {an}.\n\nRate: {ts} % empl + {tp} % empr = {tot} % total\nCeiling 標準報酬月額: ¥{plaf}/month\nBase: ¥{base} (min(gross, ceiling))\nEmployee: ¥{ms} | Employer: ¥{mp}\n\nLegal basis: 健康保険法.",
            "Arbeitnehmer-Krankenversicherung (健康保険) — Kyokai Kenpo Tokyo {an}.\n\nSatz: {ts} % AN + {tp} % AG = {tot} % gesamt\nObergrenze 標準報酬月額: ¥{plaf}/Monat\nGrundlage: ¥{base} (min(brutto, Obergrenze))\nArbeitnehmer: ¥{ms} | Arbeitgeber: ¥{mp}\n\nRechtsgrundlage: 健康保険法.",
            "Werknemersziektekostenverzekering (健康保険) — Kyokai Kenpo Tokyo {an}.\n\nTarief: {ts} % wn + {tp} % wg = {tot} % totaal\nPlafond 標準報酬月額: ¥{plaf}/maand\nGrondslag: ¥{base} (min(bruto, plafond))\nWerknemer: ¥{ms} | Werkgever: ¥{mp}\n\nWettelijke basis: 健康保険法.",
            "Assicurazione malattia dipendenti (健康保険) — Kyokai Kenpo Tokyo {an}.\n\nAliquota: {ts} % dip + {tp} % dat = {tot} % totale\nMassimale 標準報酬月額: ¥{plaf}/mese\nBase: ¥{base} (min(lordo, massimale))\nDipendente: ¥{ms} | Datore di lavoro: ¥{mp}\n\nBase giuridica: 健康保険法.",
            "Seguro de enfermedad de los trabajadores (健康保険) — Kyokai Kenpo Tokyo {an}.\n\nTipo: {ts} % trab + {tp} % empr = {tot} % total\nTope 標準報酬月額: ¥{plaf}/mes\nBase: ¥{base} (mín(bruto, tope))\nTrabajador: ¥{ms} | Empleador: ¥{mp}\n\nBase legal: 健康保険法.",
        ],
        "JP_KAIGO" => [
            "Long-term care insurance (介護保険) — for ages 40-64.\n\nNational rate {an}: {ts} % empl + {tp} % empr = {tot} % total\nSame ceiling as 健康保険: ¥{plaf}/month\nBase: ¥{base} | Employee: ¥{ms} | Employer: ¥{mp}\n\nLegal basis: 介護保険法.",
            "Pflegeversicherung (介護保険) — für 40-64-Jährige.\n\nLandessatz {an}: {ts} % AN + {tp} % AG = {tot} % gesamt\nGleiche Obergrenze wie 健康保険: ¥{plaf}/Monat\nGrundlage: ¥{base} | Arbeitnehmer: ¥{ms} | Arbeitgeber: ¥{mp}\n\nRechtsgrundlage: 介護保険法.",
            "Langdurigezorgverzekering (介護保険) — voor 40-64-jarigen.\n\nLandelijk tarief {an}: {ts} % wn + {tp} % wg = {tot} % totaal\nZelfde plafond als 健康保険: ¥{plaf}/maand\nGrondslag: ¥{base} | Werknemer: ¥{ms} | Werkgever: ¥{mp}\n\nWettelijke basis: 介護保険法.",
            "Assicurazione assistenza a lungo termine (介護保険) — per i 40-64 anni.\n\nAliquota nazionale {an}: {ts} % dip + {tp} % dat = {tot} % totale\nStesso massimale di 健康保険: ¥{plaf}/mese\nBase: ¥{base} | Dipendente: ¥{ms} | Datore di lavoro: ¥{mp}\n\nBase giuridica: 介護保険法.",
            "Seguro de cuidados de larga duración (介護保険) — para 40-64 años.\n\nTipo nacional {an}: {ts} % trab + {tp} % empr = {tot} % total\nMismo tope que 健康保険: ¥{plaf}/mes\nBase: ¥{base} | Trabajador: ¥{ms} | Empleador: ¥{mp}\n\nBase legal: 介護保険法.",
        ],
        "JP_KOSEI" => [
            "Mandatory employees' pension insurance (厚生年金保険).\n\nSingle national rate (since Oct. 2017): {ts} % empl + {tp} % empr = {tot} %\nCeiling 標準報酬月額: ¥{plaf}/month (grade 32)\nBase: ¥{base} | Employee: ¥{ms} | Employer: ¥{mp}\n\nLegal basis: 厚生年金保険法.",
            "Obligatorische Arbeitnehmer-Rentenversicherung (厚生年金保険).\n\nEinheitlicher Landessatz (seit Okt. 2017): {ts} % AN + {tp} % AG = {tot} %\nObergrenze 標準報酬月額: ¥{plaf}/Monat (Stufe 32)\nGrundlage: ¥{base} | Arbeitnehmer: ¥{ms} | Arbeitgeber: ¥{mp}\n\nRechtsgrundlage: 厚生年金保険法.",
            "Verplichte werknemerspensioenverzekering (厚生年金保険).\n\nUniform landelijk tarief (sinds okt. 2017): {ts} % wn + {tp} % wg = {tot} %\nPlafond 標準報酬月額: ¥{plaf}/maand (graad 32)\nGrondslag: ¥{base} | Werknemer: ¥{ms} | Werkgever: ¥{mp}\n\nWettelijke basis: 厚生年金保険法.",
            "Assicurazione pensione obbligatoria dei dipendenti (厚生年金保険).\n\nAliquota nazionale unica (da ott. 2017): {ts} % dip + {tp} % dat = {tot} %\nMassimale 標準報酬月額: ¥{plaf}/mese (grado 32)\nBase: ¥{base} | Dipendente: ¥{ms} | Datore di lavoro: ¥{mp}\n\nBase giuridica: 厚生年金保険法.",
            "Seguro de pensión obligatorio de los trabajadores (厚生年金保険).\n\nTipo nacional único (desde oct. 2017): {ts} % trab + {tp} % empr = {tot} %\nTope 標準報酬月額: ¥{plaf}/mes (grado 32)\nBase: ¥{base} | Trabajador: ¥{ms} | Empleador: ¥{mp}\n\nBase legal: 厚生年金保険法.",
        ],
        "JP_KOYO" => [
            "Employment insurance (雇用保険) — 一般の事業 (general sector) 2024.\n\nRate: employee {ts} % + employer {tp} % = {tot} % total\nBase: full gross salary, no ceiling.\nEmployee: ¥{ms} | Employer: ¥{mp}\n\nLegal basis: 雇用保険法.",
            "Arbeitslosenversicherung (雇用保険) — 一般の事業 (allgemeiner Sektor) 2024.\n\nSatz: Arbeitnehmer {ts} % + Arbeitgeber {tp} % = {tot} % gesamt\nGrundlage: volles Bruttogehalt, ohne Obergrenze.\nArbeitnehmer: ¥{ms} | Arbeitgeber: ¥{mp}\n\nRechtsgrundlage: 雇用保険法.",
            "Werkloosheidsverzekering (雇用保険) — 一般の事業 (algemene sector) 2024.\n\nTarief: werknemer {ts} % + werkgever {tp} % = {tot} % totaal\nGrondslag: volledig brutoloon, zonder plafond.\nWerknemer: ¥{ms} | Werkgever: ¥{mp}\n\nWettelijke basis: 雇用保険法.",
            "Assicurazione disoccupazione (雇用保険) — 一般の事業 (settore generale) 2024.\n\nAliquota: dipendente {ts} % + datore di lavoro {tp} % = {tot} % totale\nBase: retribuzione lorda intera, senza massimale.\nDipendente: ¥{ms} | Datore di lavoro: ¥{mp}\n\nBase giuridica: 雇用保険法.",
            "Seguro de empleo (雇用保険) — 一般の事業 (sector general) 2024.\n\nTipo: trabajador {ts} % + empleador {tp} % = {tot} % total\nBase: salario bruto íntegro, sin tope.\nTrabajador: ¥{ms} | Empleador: ¥{mp}\n\nBase legal: 雇用保険法.",
        ],
        "JP_ROUSAI" => [
            "Work-accident insurance (労働者災害補償保険).\n100 % borne by the employer. Office/general-services rate 2024: {tp} %.\nEmployer: ¥{mp}\n\nLegal basis: 労働者災害補償保険法.",
            "Arbeitsunfallversicherung (労働者災害補償保険).\n100 % vom Arbeitgeber getragen. Satz Büro/allg. Dienste 2024: {tp} %.\nArbeitgeber: ¥{mp}\n\nRechtsgrundlage: 労働者災害補償保険法.",
            "Arbeidsongevallenverzekering (労働者災害補償保険).\n100 % voor rekening van de werkgever. Tarief kantoor/algemene diensten 2024: {tp} %.\nWerkgever: ¥{mp}\n\nWettelijke basis: 労働者災害補償保険法.",
            "Assicurazione infortuni sul lavoro (労働者災害補償保険).\n100 % a carico del datore di lavoro. Aliquota ufficio/servizi generali 2024: {tp} %.\nDatore di lavoro: ¥{mp}\n\nBase giuridica: 労働者災害補償保険法.",
            "Seguro de accidentes laborales (労働者災害補償保険).\n100 % a cargo del empleador. Tipo oficina/servicios generales 2024: {tp} %.\nEmpleador: ¥{mp}\n\nBase legal: 労働者災害補償保険法.",
        ],
        "JP_SHOTOKUZEI" => [
            "所得税 — national income tax (monthly withholding 源泉徴収).\n\nEstimated annual gross income: ¥{rev}\n− 給与所得控除 (employment deduction): ¥{de}\n− 基礎控除 (basic deduction): ¥{db}\n= Taxable income: ¥{ri}\n\nGross 所得税: ¥{sz}\n+ 復興特別所得税 (2.1 %): ¥{fk}\n= Annual total: ¥{ta} / 12 = ¥{mens}/month\nEffective rate: {teff} %\n\nLegal basis: 所得税法 art. 28, 89; 復興特別所得税法 (Law 02/12/2011).",
            "所得税 — nationale Einkommensteuer (monatlicher Einbehalt 源泉徴収).\n\nGeschätztes Jahresbruttoeinkommen: ¥{rev}\n− 給与所得控除 (Beschäftigungsabzug): ¥{de}\n− 基礎控除 (Grundabzug): ¥{db}\n= Zu versteuerndes Einkommen: ¥{ri}\n\nBrutto-所得税: ¥{sz}\n+ 復興特別所得税 (2,1 %): ¥{fk}\n= Jahressumme: ¥{ta} / 12 = ¥{mens}/Monat\nEffektiver Satz: {teff} %\n\nRechtsgrundlage: 所得税法 Art. 28, 89; 復興特別所得税法 (Gesetz 02.12.2011).",
            "所得税 — nationale inkomstenbelasting (maandelijkse inhouding 源泉徴収).\n\nGeschat bruto jaarinkomen: ¥{rev}\n− 給与所得控除 (arbeidsaftrek): ¥{de}\n− 基礎控除 (basisaftrek): ¥{db}\n= Belastbaar inkomen: ¥{ri}\n\nBruto 所得税: ¥{sz}\n+ 復興特別所得税 (2,1 %): ¥{fk}\n= Jaartotaal: ¥{ta} / 12 = ¥{mens}/maand\nEffectief tarief: {teff} %\n\nWettelijke basis: 所得税法 art. 28, 89; 復興特別所得税法 (wet 02-12-2011).",
            "所得税 — imposta nazionale sul reddito (ritenuta mensile 源泉徴収).\n\nReddito lordo annuo stimato: ¥{rev}\n− 給与所得控除 (detrazione da lavoro): ¥{de}\n− 基礎控除 (detrazione di base): ¥{db}\n= Reddito imponibile: ¥{ri}\n\n所得税 lorda: ¥{sz}\n+ 復興特別所得税 (2,1 %): ¥{fk}\n= Totale annuo: ¥{ta} / 12 = ¥{mens}/mese\nAliquota effettiva: {teff} %\n\nBase giuridica: 所得税法 art. 28, 89; 復興特別所得税法 (legge 02/12/2011).",
            "所得税 — impuesto nacional sobre la renta (retención mensual 源泉徴収).\n\nRenta bruta anual estimada: ¥{rev}\n− 給与所得控除 (deducción por empleo): ¥{de}\n− 基礎控除 (deducción básica): ¥{db}\n= Renta imponible: ¥{ri}\n\n所得税 bruto: ¥{sz}\n+ 復興特別所得税 (2,1 %): ¥{fk}\n= Total anual: ¥{ta} / 12 = ¥{mens}/mes\nTipo efectivo: {teff} %\n\nBase legal: 所得税法 art. 28, 89; 復興特別所得税法 (ley 02/12/2011).",
        ],
        "JP_JUMINZEI" => [
            "住民税 — local tax levied by the municipality (monthly estimate).\n\nApplied rate: 10 % flat (8 % prefectural + 2 % municipal — 地方税法).\nBase: estimated taxable income ¥{ri} (gross − employment deduction)\n= ¥{ta}/yr / 12 = ¥{mens}/month\nEffective rate: {teff} %\n\nNote: in practice, 住民税 is computed in June of year N+1 on year-N income. This monthly estimate is indicative.\nLegal basis: 地方税法.",
            "住民税 — von der Gebietskörperschaft erhobene Kommunalsteuer (monatliche Schätzung).\n\nAngewandter Satz: 10 % pauschal (8 % präfektural + 2 % kommunal — 地方税法).\nGrundlage: geschätztes zu versteuerndes Einkommen ¥{ri} (brutto − Beschäftigungsabzug)\n= ¥{ta}/Jahr / 12 = ¥{mens}/Monat\nEffektiver Satz: {teff} %\n\nHinweis: In der Praxis wird 住民税 im Juni des Jahres N+1 auf das Einkommen des Jahres N berechnet. Diese monatliche Schätzung ist indikativ.\nRechtsgrundlage: 地方税法.",
            "住民税 — door de gemeente geheven lokale belasting (maandelijkse schatting).\n\nToegepast tarief: 10 % vlak (8 % prefecturaal + 2 % gemeentelijk — 地方税法).\nGrondslag: geschat belastbaar inkomen ¥{ri} (bruto − arbeidsaftrek)\n= ¥{ta}/jr / 12 = ¥{mens}/maand\nEffectief tarief: {teff} %\n\nOpmerking: in de praktijk wordt 住民税 in juni van jaar N+1 berekend over het inkomen van jaar N. Deze maandelijkse schatting is indicatief.\nWettelijke basis: 地方税法.",
            "住民税 — imposta locale riscossa dall'ente locale (stima mensile).\n\nAliquota applicata: 10 % fissa (8 % prefetturale + 2 % comunale — 地方税法).\nBase: reddito imponibile stimato ¥{ri} (lordo − detrazione da lavoro)\n= ¥{ta}/anno / 12 = ¥{mens}/mese\nAliquota effettiva: {teff} %\n\nNota: in pratica, la 住民税 è calcolata a giugno dell'anno N+1 sui redditi dell'anno N. Questa stima mensile è indicativa.\nBase giuridica: 地方税法.",
            "住民税 — impuesto local recaudado por la entidad local (estimación mensual).\n\nTipo aplicado: 10 % plano (8 % prefectural + 2 % municipal — 地方税法).\nBase: renta imponible estimada ¥{ri} (bruto − deducción por empleo)\n= ¥{ta}/año / 12 = ¥{mens}/mes\nTipo efectivo: {teff} %\n\nNota: en la práctica, el 住民税 se calcula en junio del año N+1 sobre la renta del año N. Esta estimación mensual es indicativa.\nBase legal: 地方税法.",
        ],
        _ => return None,
    };
    Some(row[i])
}
