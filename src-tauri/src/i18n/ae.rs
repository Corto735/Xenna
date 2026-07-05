// Traductions Émirats arabes unis (codes `AE_*`). Placeholders préservés
// (substitués par l'appelant) : {ts} {tp} {base}. « GPSSA » sigle conservé.

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    Some(match code {
        "AE_GPSSA" => match lang {
            "en" => "GPSSA — Pension (Emirati national)", "de" => "GPSSA — Rente (emiratischer Staatsbürger)",
            "nl" => "GPSSA — Pensioen (Emiraats staatsburger)", "it" => "GPSSA — Pensione (cittadino emiratino)",
            "es" => "GPSSA — Pensión (nacional emiratí)", _ => return None,
        },
        "AE_EXPAT" => match lang {
            "en" => "No contribution (expatriate)", "de" => "Kein Beitrag (Expatriate)",
            "nl" => "Geen bijdrage (expat)", "it" => "Nessun contributo (espatriato)",
            "es" => "Sin cotización (expatriado)", _ => return None,
        },
        _ => return None,
    })
}

pub fn t_explication(key: &str, lang: &str) -> Option<&'static str> {
    Some(match key {
        "AE_GPSSA" => match lang {
            "en" => "GPSSA (General Pension and Social Security Authority) — pension scheme for Emirati nationals. \
                Employee {ts} % + employer {tp} % on the contributory salary (capped at 50,000 AED); the State adds 2.5 %. \
                Base {base} AED. Legal basis: Federal Decree-Law 57/2023.",
            "de" => "GPSSA (allgemeine Renten- und Sozialversicherungsbehörde) — Rentensystem für emiratische Staatsbürger. \
                Arbeitnehmer {ts} % + Arbeitgeber {tp} % auf das beitragspflichtige Gehalt (gedeckelt bei 50.000 AED); der Staat legt 2,5 % dazu. \
                Basis {base} AED. Rechtsgrundlage: Federal Decree-Law 57/2023.",
            "nl" => "GPSSA (algemene pensioen- en socialezekerheidsautoriteit) — pensioenregeling voor Emiraatse staatsburgers. \
                Werknemer {ts} % + werkgever {tp} % over het bijdrageloon (gemaximeerd op 50.000 AED); de staat voegt 2,5 % toe. \
                Grondslag {base} AED. Wettelijke basis: Federal Decree-Law 57/2023.",
            "it" => "GPSSA (autorità generale per le pensioni e la previdenza sociale) — regime pensionistico per i cittadini emiratini. \
                Dipendente {ts} % + datore {tp} % sulla retribuzione contributiva (massimale 50.000 AED); lo Stato aggiunge 2,5 %. \
                Base {base} AED. Base giuridica: Federal Decree-Law 57/2023.",
            "es" => "GPSSA (Autoridad General de Pensiones y Seguridad Social) — régimen de pensiones para nacionales emiratíes. \
                Empleado {ts} % + empleador {tp} % sobre el salario cotizable (tope 50 000 AED); el Estado aporta 2,5 %. \
                Base {base} AED. Base legal: Federal Decree-Law 57/2023.",
            _ => return None,
        },
        "AE_EXPAT" => match lang {
            "en" => "The United Arab Emirates levy neither personal income tax nor social contributions on expatriate \
                employees: net equals gross. The GPSSA pension scheme applies only to Emirati nationals (tick the option).",
            "de" => "Die Vereinigten Arabischen Emirate erheben bei Expatriate-Beschäftigten weder Einkommensteuer noch \
                Sozialbeiträge: netto gleich brutto. Das GPSSA-Rentensystem gilt nur für emiratische Staatsbürger (Option ankreuzen).",
            "nl" => "De Verenigde Arabische Emiraten heffen bij expat-werknemers geen inkomstenbelasting noch sociale \
                premies: netto is gelijk aan bruto. De GPSSA-pensioenregeling geldt alleen voor Emiraatse staatsburgers (optie aanvinken).",
            "it" => "Gli Emirati Arabi Uniti non applicano né imposta sul reddito né contributi sociali ai lavoratori \
                espatriati: il netto è pari al lordo. Il regime pensionistico GPSSA riguarda solo i cittadini emiratini (spuntare l'opzione).",
            "es" => "Los Emiratos Árabes Unidos no aplican impuesto sobre la renta ni cotizaciones sociales a los empleados \
                expatriados: el neto es igual al bruto. El régimen de pensiones GPSSA solo afecta a los nacionales emiratíes (marcar la opción).",
            _ => return None,
        },
        _ => return None,
    })
}
