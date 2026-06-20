// Traductions Corée du Sud (codes `KR_*`). Libellés + explications en/de/nl/it/es.
//
// Convention identique à `divers.rs` : lignes `[&str; 5]` indexées par langue,
// placeholders nommés (`{ts}`, `{base}`, …) substitués côté appelant après lookup.
// Les termes coréens (국민연금, 소득세…) et les références légales restent intacts.

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    let i = match lang { "en" => 0, "de" => 1, "nl" => 2, "it" => 3, "es" => 4, _ => return None };
    let row: [&str; 5] = match code {
        "KR_NPS" => ["국민연금 — National pension", "국민연금 — Nationale Rente", "국민연금 — Nationaal pensioen", "국민연금 — Pensione nazionale", "국민연금 — Pensión nacional"],
        "KR_NHI" => ["건강보험 — Health insurance", "건강보험 — Krankenversicherung", "건강보험 — Ziektekostenverzekering", "건강보험 — Assicurazione sanitaria", "건강보험 — Seguro de salud"],
        "KR_LTC" => ["장기요양보험 — Long-term care", "장기요양보험 — Pflegeversicherung", "장기요양보험 — Langdurige zorg", "장기요양보험 — Assistenza a lungo termine", "장기요양보험 — Dependencia"],
        "KR_EI" => ["고용보험 — Employment insurance", "고용보험 — Arbeitslosenversicherung", "고용보험 — Werkloosheidsverzekering", "고용보험 — Assicurazione disoccupazione", "고용보험 — Seguro de empleo"],
        "KR_SANJAE" => ["산재보험 — Work accidents (employer)", "산재보험 — Arbeitsunfälle (Arbeitgeber)", "산재보험 — Arbeidsongevallen (werkgever)", "산재보험 — Infortuni sul lavoro (datore di lavoro)", "산재보험 — Accidentes laborales (empleador)"],
        "KR_INCOME_TAX" => ["소득세 — Income tax", "소득세 — Einkommensteuer", "소득세 — Inkomstenbelasting", "소득세 — Imposta sul reddito", "소득세 — Impuesto sobre la renta"],
        "KR_LOCAL_TAX" => ["지방소득세 — Local income tax (10 %)", "지방소득세 — Lokale Einkommensteuer (10 %)", "지방소득세 — Lokale inkomstenbelasting (10 %)", "지방소득세 — Imposta locale sul reddito (10 %)", "지방소득세 — Impuesto local sobre la renta (10 %)"],
        _ => return None,
    };
    Some(row[i])
}

pub fn t_explication(key: &str, lang: &str) -> Option<&'static str> {
    let i = match lang { "en" => 0, "de" => 1, "nl" => 2, "it" => 3, "es" => 4, _ => return None };
    let row: [&str; 5] = match key {
        "KR_NPS" => [
            "국민연금 — pension. {ts} % empl / {tp} % empr.\nBase capped at 6,370,000 ₩/month → {base} ₩.\nEmployee: {ms} ₩.\n\nLegal basis: 국민연금법.",
            "국민연금 — Rente. {ts} % AN / {tp} % AG.\nBemessungsgrundlage gedeckelt auf 6 370 000 ₩/Monat → {base} ₩.\nArbeitnehmer: {ms} ₩.\n\nRechtsgrundlage: 국민연금법.",
            "국민연금 — pensioen. {ts} % wn / {tp} % wg.\nGrondslag begrensd tot 6.370.000 ₩/maand → {base} ₩.\nWerknemer: {ms} ₩.\n\nWettelijke basis: 국민연금법.",
            "국민연금 — pensione. {ts} % dip / {tp} % dat.\nBase limitata a 6 370 000 ₩/mese → {base} ₩.\nDipendente: {ms} ₩.\n\nBase giuridica: 국민연금법.",
            "국민연금 — pensión. {ts} % trab / {tp} % empr.\nBase limitada a 6 370 000 ₩/mes → {base} ₩.\nTrabajador: {ms} ₩.\n\nBase legal: 국민연금법.",
        ],
        "KR_NHI" => [
            "건강보험 — health. {ts} % each. Employee: {ms} ₩.\n\nLegal basis: 국민건강보험법.",
            "건강보험 — Gesundheit. {ts} % je. Arbeitnehmer: {ms} ₩.\n\nRechtsgrundlage: 국민건강보험법.",
            "건강보험 — gezondheid. {ts} % elk. Werknemer: {ms} ₩.\n\nWettelijke basis: 국민건강보험법.",
            "건강보험 — sanità. {ts} % ciascuno. Dipendente: {ms} ₩.\n\nBase giuridica: 국민건강보험법.",
            "건강보험 — salud. {ts} % cada uno. Trabajador: {ms} ₩.\n\nBase legal: 국민건강보험법.",
        ],
        "KR_LTC" => [
            "장기요양보험 — long-term care. {lt} % of the health premium.\nBase: employee health premium {b} ₩ → {ms} ₩.\n\nLegal basis: 노인장기요양보험법.",
            "장기요양보험 — Langzeitpflege. {lt} % der Krankenversicherungsprämie.\nGrundlage: AN-Krankenprämie {b} ₩ → {ms} ₩.\n\nRechtsgrundlage: 노인장기요양보험법.",
            "장기요양보험 — langdurige zorg. {lt} % van de zorgpremie.\nGrondslag: zorgpremie werknemer {b} ₩ → {ms} ₩.\n\nWettelijke basis: 노인장기요양보험법.",
            "장기요양보험 — assistenza a lungo termine. {lt} % del premio sanitario.\nBase: premio sanitario dipendente {b} ₩ → {ms} ₩.\n\nBase giuridica: 노인장기요양보험법.",
            "장기요양보험 — cuidados de larga duración. {lt} % de la prima sanitaria.\nBase: prima sanitaria del trabajador {b} ₩ → {ms} ₩.\n\nBase legal: 노인장기요양보험법.",
        ],
        "KR_EI" => [
            "고용보험 — employment. Employee {ts} % / employer {tp} %.\n\nLegal basis: 고용보험법.",
            "고용보험 — Beschäftigung. Arbeitnehmer {ts} % / Arbeitgeber {tp} %.\n\nRechtsgrundlage: 고용보험법.",
            "고용보험 — werk. Werknemer {ts} % / werkgever {tp} %.\n\nWettelijke basis: 고용보험법.",
            "고용보험 — occupazione. Dipendente {ts} % / datore di lavoro {tp} %.\n\nBase giuridica: 고용보험법.",
            "고용보험 — empleo. Trabajador {ts} % / empleador {tp} %.\n\nBase legal: 고용보험법.",
        ],
        "KR_SANJAE" => [
            "산재보험 — work accidents, 100 % employer. ≈ {tp} % (average).\n\nLegal basis: 고용보험법/산재.",
            "산재보험 — Arbeitsunfälle, 100 % Arbeitgeber. ≈ {tp} % (Durchschnitt).\n\nRechtsgrundlage: 고용보험법/산재.",
            "산재보험 — arbeidsongevallen, 100 % werkgever. ≈ {tp} % (gemiddeld).\n\nWettelijke basis: 고용보험법/산재.",
            "산재보험 — infortuni sul lavoro, 100 % datore di lavoro. ≈ {tp} % (medio).\n\nBase giuridica: 고용보험법/산재.",
            "산재보험 — accidentes laborales, 100 % empleador. ≈ {tp} % (medio).\n\nBase legal: 고용보험법/산재.",
        ],
        "KR_INCOME_TAX" => [
            "소득세 — national tax (annualised).\n\n총급여 {g} ₩ − 근로소득공제 {ded} ₩ − 기본공제 1,500,000 ₩\n= taxable income {tx} ₩\nScale 6→45 %: {tb} ₩ − 근로소득세액공제 {cr} ₩\n= {na} ₩/yr / 12 = {nm} ₩/month.\n\nLegal basis: 소득세법.",
            "소득세 — nationale Steuer (annualisiert).\n\n총급여 {g} ₩ − 근로소득공제 {ded} ₩ − 기본공제 1 500 000 ₩\n= zu versteuerndes Einkommen {tx} ₩\nTarif 6→45 %: {tb} ₩ − 근로소득세액공제 {cr} ₩\n= {na} ₩/Jahr / 12 = {nm} ₩/Monat.\n\nRechtsgrundlage: 소득세법.",
            "소득세 — nationale belasting (op jaarbasis).\n\n총급여 {g} ₩ − 근로소득공제 {ded} ₩ − 기본공제 1.500.000 ₩\n= belastbaar inkomen {tx} ₩\nSchaal 6→45 %: {tb} ₩ − 근로소득세액공제 {cr} ₩\n= {na} ₩/jr / 12 = {nm} ₩/maand.\n\nWettelijke basis: 소득세법.",
            "소득세 — imposta nazionale (annualizzata).\n\n총급여 {g} ₩ − 근로소득공제 {ded} ₩ − 기본공제 1 500 000 ₩\n= reddito imponibile {tx} ₩\nScaglioni 6→45 %: {tb} ₩ − 근로소득세액공제 {cr} ₩\n= {na} ₩/anno / 12 = {nm} ₩/mese.\n\nBase giuridica: 소득세법.",
            "소득세 — impuesto nacional (anualizado).\n\n총급여 {g} ₩ − 근로소득공제 {ded} ₩ − 기본공제 1 500 000 ₩\n= renta imponible {tx} ₩\nEscala 6→45 %: {tb} ₩ − 근로소득세액공제 {cr} ₩\n= {na} ₩/año / 12 = {nm} ₩/mes.\n\nBase legal: 소득세법.",
        ],
        "KR_LOCAL_TAX" => [
            "지방소득세 — local tax = 10 % of national tax.\n{n} ₩ × 10 % = {l} ₩/month.\n\nLegal basis: 지방세법.",
            "지방소득세 — lokale Steuer = 10 % der nationalen Steuer.\n{n} ₩ × 10 % = {l} ₩/Monat.\n\nRechtsgrundlage: 지방세법.",
            "지방소득세 — lokale belasting = 10 % van de nationale belasting.\n{n} ₩ × 10 % = {l} ₩/maand.\n\nWettelijke basis: 지방세법.",
            "지방소득세 — imposta locale = 10 % dell'imposta nazionale.\n{n} ₩ × 10 % = {l} ₩/mese.\n\nBase giuridica: 지방세법.",
            "지방소득세 — impuesto local = 10 % del impuesto nacional.\n{n} ₩ × 10 % = {l} ₩/mes.\n\nBase legal: 지방세법.",
        ],
        _ => return None,
    };
    Some(row[i])
}
