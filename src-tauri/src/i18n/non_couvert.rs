// Messages « année non couverte » par pays (1re ligne de l'explication
// PAYS_NON_COUVERT). Le français natif vit chez l'appelant (calculs/*_bulletin.rs) ;
// ici les 5 langues du menu 🌐, assemblées « Nom du pays: disponibilité ».
// La phrase de conclusion (« lacune assumée ») vit dans cotisations::t_explication
// sous la clé PAYS_NON_COUVERT.

/// Message traduit pour un pays non couvert à la date demandée,
/// ou None si le couple (code pays, langue) n'est pas couvert → repli français.
pub fn message(code_pays: &str, lang: &str) -> Option<String> {
    let i = match lang { "en" => 0, "de" => 1, "nl" => 2, "it" => 3, "es" => 4, _ => return None };

    let pays: [&str; 5] = match code_pays {
        "AD" => ["Andorra", "Andorra", "Andorra", "Andorra", "Andorra"],
        "AT" => ["Austria", "Österreich", "Oostenrijk", "Austria", "Austria"],
        "AU" => ["Australia", "Australien", "Australië", "Australia", "Australia"],
        "BG" => ["Bulgaria", "Bulgarien", "Bulgarije", "Bulgaria", "Bulgaria"],
        "CY" => ["Cyprus", "Zypern", "Cyprus", "Cipro", "Chipre"],
        "CZ" => ["Czechia", "Tschechien", "Tsjechië", "Cechia", "Chequia"],
        "DK" => ["Denmark", "Dänemark", "Denemarken", "Danimarca", "Dinamarca"],
        "EE" => ["Estonia", "Estland", "Estland", "Estonia", "Estonia"],
        "FI" => ["Finland", "Finnland", "Finland", "Finlandia", "Finlandia"],
        "GR" => ["Greece", "Griechenland", "Griekenland", "Grecia", "Grecia"],
        "HR" => ["Croatia", "Kroatien", "Kroatië", "Croazia", "Croacia"],
        "HU" => ["Hungary", "Ungarn", "Hongarije", "Ungheria", "Hungría"],
        "IE" => ["Ireland", "Irland", "Ierland", "Irlanda", "Irlanda"],
        "KR" => ["South Korea", "Südkorea", "Zuid-Korea", "Corea del Sud", "Corea del Sur"],
        "LT" => ["Lithuania", "Litauen", "Litouwen", "Lituania", "Lituania"],
        "LV" => ["Latvia", "Lettland", "Letland", "Lettonia", "Letonia"],
        "MC" => ["Monaco", "Monaco", "Monaco", "Monaco", "Mónaco"],
        "MT" => ["Malta", "Malta", "Malta", "Malta", "Malta"],
        "NZ" => ["New Zealand", "Neuseeland", "Nieuw-Zeeland", "Nuova Zelanda", "Nueva Zelanda"],
        "PL" => ["Poland", "Polen", "Polen", "Polonia", "Polonia"],
        "RO" => ["Romania", "Rumänien", "Roemenië", "Romania", "Rumanía"],
        "SE" => ["Sweden", "Schweden", "Zweden", "Svezia", "Suecia"],
        "SI" => ["Slovenia", "Slowenien", "Slovenië", "Slovenia", "Eslovenia"],
        "SK" => ["Slovakia", "Slowakei", "Slowakije", "Slovacchia", "Eslovaquia"],
        _ => return None,
    };

    let dispo: [&str; 5] = match code_pays {
        // Groupe « données disponibles pour 2025 et 2026 »
        "AT" | "BG" | "CY" | "CZ" | "EE" | "GR" | "HR" | "HU" | "IE" | "LT" | "LV"
        | "MT" | "SE" | "SI" | "SK" => [
            "data available for 2025 and 2026.",
            "Daten verfügbar für 2025 und 2026.",
            "gegevens beschikbaar voor 2025 en 2026.",
            "dati disponibili per il 2025 e il 2026.",
            "datos disponibles para 2025 y 2026.",
        ],
        "DK" | "KR" => [
            "data available for 2024-2026.",
            "Daten verfügbar für 2024-2026.",
            "gegevens beschikbaar voor 2024-2026.",
            "dati disponibili per il periodo 2024-2026.",
            "datos disponibles para 2024-2026.",
        ],
        "MC" => [
            "data available for 2020-2026.",
            "Daten verfügbar für 2020-2026.",
            "gegevens beschikbaar voor 2020-2026.",
            "dati disponibili per il periodo 2020-2026.",
            "datos disponibles para 2020-2026.",
        ],
        "FI" => [
            "data available for 2026.",
            "Daten verfügbar für 2026.",
            "gegevens beschikbaar voor 2026.",
            "dati disponibili per il 2026.",
            "datos disponibles para 2026.",
        ],
        "PL" => [
            "data available for 2025.",
            "Daten verfügbar für 2025.",
            "gegevens beschikbaar voor 2025.",
            "dati disponibili per il 2025.",
            "datos disponibles para 2025.",
        ],
        "RO" => [
            "data available from 2018 (OUG 79/2017 reform).",
            "Daten verfügbar ab 2018 (Reform OUG 79/2017).",
            "gegevens beschikbaar vanaf 2018 (hervorming OUG 79/2017).",
            "dati disponibili dal 2018 (riforma OUG 79/2017).",
            "datos disponibles desde 2018 (reforma OUG 79/2017).",
        ],
        "AD" => [
            "data available from 2015 (creation of the IRPF).",
            "Daten verfügbar ab 2015 (Einführung der IRPF).",
            "gegevens beschikbaar vanaf 2015 (invoering van de IRPF).",
            "dati disponibili dal 2015 (creazione dell'IRPF).",
            "datos disponibles desde 2015 (creación del IRPF).",
        ],
        "AU" | "NZ" => [
            "data available for fiscal years 2014-15 to 2025-26.",
            "Daten verfügbar für die Steuerjahre 2014-15 bis 2025-26.",
            "gegevens beschikbaar voor de belastingjaren 2014-15 tot 2025-26.",
            "dati disponibili per gli anni fiscali dal 2014-15 al 2025-26.",
            "datos disponibles para los ejercicios fiscales 2014-15 a 2025-26.",
        ],
        _ => return None,
    };

    Some(format!("{}: {}", pays[i], dispo[i]))
}
