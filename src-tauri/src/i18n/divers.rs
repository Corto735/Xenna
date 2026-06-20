// Traductions des pays « mono-fichier » (AD, AT, BG, CY, CZ, DK, EE, FI, GR, HR,
// HU, IE, LV, LT, MT, SI, SE, RO, PL, SK, NZ, NL, MC, CN, UK).
//
// Libellés par code ; explications par code OU via une clé synthétique générique
// (`XX_GENERIC`) pour les pays dont toutes les lignes partagent un même gabarit.
// Placeholders nommés (`{ts}`, `{tp}`, `{libelle}`, …) identiques aux 6 langues,
// substitués côté appelant. None → repli français.

pub fn t_libelle(code: &str, lang: &str) -> Option<&'static str> {
    let i = match lang { "en" => 0, "de" => 1, "nl" => 2, "it" => 3, "es" => 4, _ => return None };
    let row: [&str; 5] = match code {
        // ── Andorre ──
        "AD_CASS" => ["CASS — Social security", "CASS — Sozialversicherung", "CASS — Sociale zekerheid", "CASS — Sicurezza sociale", "CASS — Seguridad social"],
        "AD_IRPF" => ["IRPF — Income tax", "IRPF — Einkommensteuer", "IRPF — Inkomstenbelasting", "IRPF — Imposta sul reddito", "IRPF — Impuesto sobre la renta"],
        // ── Autriche ──
        "AT_SV" => ["Sozialversicherung — Social contributions", "Sozialversicherung — Sozialbeiträge", "Sozialversicherung — Sociale bijdragen", "Sozialversicherung — Contributi sociali", "Sozialversicherung — Cotizaciones sociales"],
        "AT_LOHNSTEUER" => ["Lohnsteuer — Income tax", "Lohnsteuer — Einkommensteuer", "Lohnsteuer — Inkomstenbelasting", "Lohnsteuer — Imposta sul reddito", "Lohnsteuer — Impuesto sobre la renta"],
        // ── Bulgarie ──
        "BG_OSIG" => ["Осигуровки — Social contributions", "Осигуровки — Sozialbeiträge", "Осигуровки — Sociale bijdragen", "Осигуровки — Contributi sociali", "Осигуровки — Cotizaciones sociales"],
        "BG_DANAK" => ["Данък върху доходите — Income tax (10 %)", "Данък върху доходите — Einkommensteuer (10 %)", "Данък върху доходите — Inkomstenbelasting (10 %)", "Данък върху доходите — Imposta sul reddito (10 %)", "Данък върху доходите — Impuesto sobre la renta (10 %)"],
        // ── Chypre ──
        "CY_SI" => ["Κοινωνικές Ασφαλίσεις — Social insurance", "Κοινωνικές Ασφαλίσεις — Sozialversicherung", "Κοινωνικές Ασφαλίσεις — Sociale verzekering", "Κοινωνικές Ασφαλίσεις — Assicurazione sociale", "Κοινωνικές Ασφαλίσεις — Seguro social"],
        "CY_GESY" => ["ΓΕΣΥ — National health system", "ΓΕΣΥ — Nationales Gesundheitssystem", "ΓΕΣΥ — Nationaal gezondheidssysteem", "ΓΕΣΥ — Sistema sanitario nazionale", "ΓΕΣΥ — Sistema nacional de salud"],
        "CY_FOROS" => ["Φόρος εισοδήματος — Income tax", "Φόρος εισοδήματος — Einkommensteuer", "Φόρος εισοδήματος — Inkomstenbelasting", "Φόρος εισοδήματος — Imposta sul reddito", "Φόρος εισοδήματος — Impuesto sobre la renta"],
        // ── Tchéquie ──
        "CZ_SOCIAL" => ["Sociální pojištění — Social security", "Sociální pojištění — Sozialversicherung", "Sociální pojištění — Sociale zekerheid", "Sociální pojištění — Sicurezza sociale", "Sociální pojištění — Seguridad social"],
        "CZ_ZDRAVOTNI" => ["Zdravotní pojištění — Health insurance", "Zdravotní pojištění — Krankenversicherung", "Zdravotní pojištění — Ziektekostenverzekering", "Zdravotní pojištění — Assicurazione malattia", "Zdravotní pojištění — Seguro de enfermedad"],
        "CZ_DAN" => ["Daň z příjmů — Income tax", "Daň z příjmů — Einkommensteuer", "Daň z příjmů — Inkomstenbelasting", "Daň z příjmů — Imposta sul reddito", "Daň z příjmů — Impuesto sobre la renta"],
        // ── Danemark ──
        "DK_AM" => ["AM-bidrag — Labour market contribution", "AM-bidrag — Arbeitsmarktbeitrag", "AM-bidrag — Arbeidsmarktbijdrage", "AM-bidrag — Contributo mercato del lavoro", "AM-bidrag — Contribución al mercado laboral"],
        "DK_ATP" => ["ATP — Supplementary pension", "ATP — Zusatzrente", "ATP — Aanvullend pensioen", "ATP — Pensione complementare", "ATP — Pensión complementaria"],
        "DK_INDKOMSTSKAT" => ["Indkomstskat — Income tax", "Indkomstskat — Einkommensteuer", "Indkomstskat — Inkomstenbelasting", "Indkomstskat — Imposta sul reddito", "Indkomstskat — Impuesto sobre la renta"],
        // ── Estonie ──
        "EE_TOOTUS" => ["Töötuskindlustusmakse — Unemployment", "Töötuskindlustusmakse — Arbeitslosigkeit", "Töötuskindlustusmakse — Werkloosheid", "Töötuskindlustusmakse — Disoccupazione", "Töötuskindlustusmakse — Desempleo"],
        "EE_KOGUMISPENSION" => ["Kogumispension — 2nd-pillar pension", "Kogumispension — Rente 2. Säule", "Kogumispension — Pensioen 2e pijler", "Kogumispension — Pensione 2º pilastro", "Kogumispension — Pensión 2º pilar"],
        "EE_SOTSIAALMAKS" => ["Sotsiaalmaks — Social charge (employer)", "Sotsiaalmaks — Sozialabgabe (Arbeitgeber)", "Sotsiaalmaks — Sociale last (werkgever)", "Sotsiaalmaks — Onere sociale (datore di lavoro)", "Sotsiaalmaks — Carga social (empleador)"],
        "EE_TULUMAKS" => ["Tulumaks — Income tax (22 %)", "Tulumaks — Einkommensteuer (22 %)", "Tulumaks — Inkomstenbelasting (22 %)", "Tulumaks — Imposta sul reddito (22 %)", "Tulumaks — Impuesto sobre la renta (22 %)"],
        // ── Finlande ──
        "FI_TYEL" => ["TyEL — Pension", "TyEL — Rente", "TyEL — Pensioen", "TyEL — Pensione", "TyEL — Pensión"],
        "FI_TYOTTOMYYS" => ["Työttömyysvakuutus — Unemployment", "Työttömyysvakuutus — Arbeitslosigkeit", "Työttömyysvakuutus — Werkloosheid", "Työttömyysvakuutus — Disoccupazione", "Työttömyysvakuutus — Desempleo"],
        "FI_SAIRAANHOITO" => ["Sairaanhoitomaksu — Healthcare", "Sairaanhoitomaksu — Gesundheitsversorgung", "Sairaanhoitomaksu — Gezondheidszorg", "Sairaanhoitomaksu — Assistenza sanitaria", "Sairaanhoitomaksu — Asistencia sanitaria"],
        "FI_TYONANTAJA_SV" => ["Health insurance (employer)", "Krankenversicherung (Arbeitgeber)", "Ziektekostenverzekering (werkgever)", "Assicurazione malattia (datore di lavoro)", "Seguro de enfermedad (empleador)"],
        "FI_PAIVARAHA" => ["Päivärahamaksu — Daily allowance", "Päivärahamaksu — Tagegeldbeitrag", "Päivärahamaksu — Dagvergoeding", "Päivärahamaksu — Indennità giornaliera", "Päivärahamaksu — Subsidio diario"],
        "FI_TULOVERO" => ["Tulovero — Tax (state + municipal)", "Tulovero — Steuer (Staat + Gemeinde)", "Tulovero — Belasting (staat + gemeente)", "Tulovero — Imposta (statale + comunale)", "Tulovero — Impuesto (estatal + municipal)"],
        // ── Grèce ──
        "GR_EFKA" => ["EFKA — Social contributions", "EFKA — Sozialbeiträge", "EFKA — Sociale bijdragen", "EFKA — Contributi sociali", "EFKA — Cotizaciones sociales"],
        "GR_FOROS" => ["Φόρος εισοδήματος — Income tax", "Φόρος εισοδήματος — Einkommensteuer", "Φόρος εισοδήματος — Inkomstenbelasting", "Φόρος εισοδήματος — Imposta sul reddito", "Φόρος εισοδήματος — Impuesto sobre la renta"],
        // ── Croatie ──
        "HR_MIROVINSKO" => ["Mirovinsko osiguranje — Pension", "Mirovinsko osiguranje — Rente", "Mirovinsko osiguranje — Pensioen", "Mirovinsko osiguranje — Pensione", "Mirovinsko osiguranje — Pensión"],
        "HR_ZDRAVSTVENO" => ["Zdravstveno osiguranje — Health (employer)", "Zdravstveno osiguranje — Gesundheit (Arbeitgeber)", "Zdravstveno osiguranje — Gezondheid (werkgever)", "Zdravstveno osiguranje — Sanità (datore di lavoro)", "Zdravstveno osiguranje — Salud (empleador)"],
        "HR_POREZ" => ["Porez na dohodak — Income tax", "Porez na dohodak — Einkommensteuer", "Porez na dohodak — Inkomstenbelasting", "Porez na dohodak — Imposta sul reddito", "Porez na dohodak — Impuesto sobre la renta"],
        // ── Hongrie ──
        "HU_TB" => ["Társadalombiztosítás — Social contribution", "Társadalombiztosítás — Sozialbeitrag", "Társadalombiztosítás — Sociale bijdrage", "Társadalombiztosítás — Contributo sociale", "Társadalombiztosítás — Cotización social"],
        "HU_SZOCHO" => ["Social contribution tax (employer)", "Sozialbeitragssteuer (Arbeitgeber)", "Sociale bijdrageheffing (werkgever)", "Imposta sul contributo sociale (datore di lavoro)", "Impuesto de cotización social (empleador)"],
        "HU_SZJA" => ["SZJA — Income tax (15 %)", "SZJA — Einkommensteuer (15 %)", "SZJA — Inkomstenbelasting (15 %)", "SZJA — Imposta sul reddito (15 %)", "SZJA — Impuesto sobre la renta (15 %)"],
        // ── Irlande ──
        "IE_PRSI" => ["PRSI (Class A) — Social contribution", "PRSI (Class A) — Sozialbeitrag", "PRSI (Class A) — Sociale bijdrage", "PRSI (Class A) — Contributo sociale", "PRSI (Class A) — Cotización social"],
        "IE_PAYE" => ["Income Tax (PAYE)", "Income Tax (PAYE) — Einkommensteuer", "Income Tax (PAYE) — Inkomstenbelasting", "Income Tax (PAYE) — Imposta sul reddito", "Income Tax (PAYE) — Impuesto sobre la renta"],
        // IE_USC : identique 6 langues → None (repli fr)
        // ── Lettonie ──
        "LV_VSAOI" => ["VSAOI — Mandatory social contributions", "VSAOI — Pflichtsozialbeiträge", "VSAOI — Verplichte sociale bijdragen", "VSAOI — Contributi sociali obbligatori", "VSAOI — Cotizaciones sociales obligatorias"],
        "LV_IIN" => ["IIN — Income tax", "IIN — Einkommensteuer", "IIN — Inkomstenbelasting", "IIN — Imposta sul reddito", "IIN — Impuesto sobre la renta"],
        // ── Lituanie ──
        "LT_SODRA" => ["Sodra — Social contributions", "Sodra — Sozialbeiträge", "Sodra — Sociale bijdragen", "Sodra — Contributi sociali", "Sodra — Cotizaciones sociales"],
        "LT_GPM" => ["GPM — Income tax", "GPM — Einkommensteuer", "GPM — Inkomstenbelasting", "GPM — Imposta sul reddito", "GPM — Impuesto sobre la renta"],
        // ── Malte (seul en diffère) ──
        "MT_SSC" => ["Social Security Contributions (Class 1)", "Social Security Contributions (Klassi 1)", "Social Security Contributions (Klassi 1)", "Social Security Contributions (Klassi 1)", "Social Security Contributions (Klassi 1)"],
        "MT_TAX" => ["Income Tax", "Income Tax — Einkommensteuer", "Income Tax — Inkomstenbelasting", "Income Tax — Imposta sul reddito", "Income Tax — Impuesto sobre la renta"],
        // ── Slovénie ──
        "SI_PRISPEVKI" => ["Prispevki — Social contributions", "Prispevki — Sozialbeiträge", "Prispevki — Sociale bijdragen", "Prispevki — Contributi sociali", "Prispevki — Cotizaciones sociales"],
        "SI_DOHODNINA" => ["Dohodnina — Income tax", "Dohodnina — Einkommensteuer", "Dohodnina — Inkomstenbelasting", "Dohodnina — Imposta sul reddito", "Dohodnina — Impuesto sobre la renta"],
        // ── Suède ──
        "SE_ARBETSGIVARAVGIFT" => ["Arbetsgivaravgifter — employer contributions", "Arbetsgivaravgifter — Arbeitgeberbeiträge", "Arbetsgivaravgifter — werkgeversbijdragen", "Arbetsgivaravgifter — contributi del datore di lavoro", "Arbetsgivaravgifter — cotizaciones patronales"],
        "SE_SKATT" => ["Inkomstskatt — Tax (municipal + state)", "Inkomstskatt — Steuer (Gemeinde + Staat)", "Inkomstskatt — Belasting (gemeente + staat)", "Inkomstskatt — Imposta (comunale + statale)", "Inkomstskatt — Impuesto (municipal + estatal)"],
        // ── Roumanie ──
        "RO_CAS" => ["CAS — Pension", "CAS — Rente", "CAS — Pensioen", "CAS — Pensione", "CAS — Pensión"],
        "RO_CASS" => ["CASS — Health insurance", "CASS — Krankenversicherung", "CASS — Ziektekostenverzekering", "CASS — Assicurazione malattia", "CASS — Seguro de enfermedad"],
        "RO_CAM" => ["CAM — Work contribution (employer)", "CAM — Arbeitsbeitrag (Arbeitgeber)", "CAM — Arbeidsbijdrage (werkgever)", "CAM — Contributo lavoro (datore di lavoro)", "CAM — Contribución laboral (empleador)"],
        "RO_IMPOZIT" => ["Impozit pe venit — Income tax (10 %)", "Impozit pe venit — Einkommensteuer (10 %)", "Impozit pe venit — Inkomstenbelasting (10 %)", "Impozit pe venit — Imposta sul reddito (10 %)", "Impozit pe venit — Impuesto sobre la renta (10 %)"],
        // ── Pologne ──
        "PL_EMERYTALNE" => ["Emerytalne — Old-age", "Emerytalne — Alter", "Emerytalne — Ouderdom", "Emerytalne — Vecchiaia", "Emerytalne — Vejez"],
        "PL_RENTOWE" => ["Rentowe — Disability/survivors", "Rentowe — Invalidität/Hinterbliebene", "Rentowe — Invaliditeit/nabestaanden", "Rentowe — Invalidità/superstiti", "Rentowe — Invalidez/supervivencia"],
        "PL_CHOROBOWE" => ["Chorobowe — Sickness", "Chorobowe — Krankheit", "Chorobowe — Ziekte", "Chorobowe — Malattia", "Chorobowe — Enfermedad"],
        "PL_WYPADKOWE" => ["Wypadkowe — Accidents (employer)", "Wypadkowe — Unfälle (Arbeitgeber)", "Wypadkowe — Ongevallen (werkgever)", "Wypadkowe — Infortuni (datore di lavoro)", "Wypadkowe — Accidentes (empleador)"],
        "PL_FP" => ["Fundusz Pracy — Labour Fund (employer)", "Fundusz Pracy — Arbeitsfonds (Arbeitgeber)", "Fundusz Pracy — Arbeidsfonds (werkgever)", "Fundusz Pracy — Fondo lavoro (datore di lavoro)", "Fundusz Pracy — Fondo de trabajo (empleador)"],
        "PL_FGSP" => ["FGŚP — Guaranteed Benefits Fund (employer)", "FGŚP — Garantiefonds (Arbeitgeber)", "FGŚP — Garantiefonds (werkgever)", "FGŚP — Fondo garanzia (datore di lavoro)", "FGŚP — Fondo de garantía (empleador)"],
        "PL_ZDROWOTNE" => ["Składka zdrowotna — Health insurance (9 %)", "Składka zdrowotna — Krankenversicherung (9 %)", "Składka zdrowotna — Ziektekostenverzekering (9 %)", "Składka zdrowotna — Assicurazione malattia (9 %)", "Składka zdrowotna — Seguro de enfermedad (9 %)"],
        "PL_PIT" => ["PIT — Income tax", "PIT — Einkommensteuer", "PIT — Inkomstenbelasting", "PIT — Imposta sul reddito", "PIT — Impuesto sobre la renta"],
        // ── Slovaquie ──
        "SK_ZDRAVOTNE" => ["Zdravotné poistenie — Health insurance", "Zdravotné poistenie — Krankenversicherung", "Zdravotné poistenie — Ziektekostenverzekering", "Zdravotné poistenie — Assicurazione malattia", "Zdravotné poistenie — Seguro de enfermedad"],
        "SK_SOCIALNE" => ["Sociálne poistenie — Social security", "Sociálne poistenie — Sozialversicherung", "Sociálne poistenie — Sociale zekerheid", "Sociálne poistenie — Sicurezza sociale", "Sociálne poistenie — Seguridad social"],
        "SK_DAN" => ["Daň z príjmov — Income tax", "Daň z príjmov — Einkommensteuer", "Daň z príjmov — Inkomstenbelasting", "Daň z príjmov — Imposta sul reddito", "Daň z príjmov — Impuesto sobre la renta"],
        // ── Nouvelle-Zélande ──
        "NZ_PAYE" => ["PAYE — Income tax", "PAYE — Einkommensteuer", "PAYE — Inkomstenbelasting", "PAYE — Imposta sul reddito", "PAYE — Impuesto sobre la renta"],
        "NZ_ACC" => ["ACC earner's levy — Accident insurance", "ACC earner's levy — Unfallversicherung", "ACC earner's levy — Ongevallenverzekering", "ACC earner's levy — Assicurazione infortuni", "ACC earner's levy — Seguro de accidentes"],
        "NZ_KIWISAVER_EMP" => ["KiwiSaver — Pension (employer, default 3 %)", "KiwiSaver — Rente (Arbeitgeber, Standard 3 %)", "KiwiSaver — Pensioen (werkgever, standaard 3 %)", "KiwiSaver — Pensione (datore di lavoro, predefinito 3 %)", "KiwiSaver — Pensión (empleador, por defecto 3 %)"],
        // ── Pays-Bas ──
        "NL_ZVW" => ["Zvw — Health insurance", "Zvw — Krankenversicherung", "Zvw — Zorgverzekering", "Zvw — Assicurazione sanitaria", "Zvw — Seguro de salud"],
        "NL_AWF" => ["AWf — Unemployment (WW)", "AWf — Arbeitslosigkeit (WW)", "AWf — Werkloosheid (WW)", "AWf — Disoccupazione (WW)", "AWf — Desempleo (WW)"],
        "NL_AOF" => ["Aof — Disability (WIA)", "Aof — Invalidität (WIA)", "Aof — Arbeidsongeschiktheid (WIA)", "Aof — Invalidità (WIA)", "Aof — Invalidez (WIA)"],
        // NL_WHK : identique 6 langues → None
        "NL_OPSLAG_KO" => ["Childcare surcharge (Opslag kinderopvang)", "Kinderbetreuungszuschlag (Opslag kinderopvang)", "Opslag kinderopvang", "Supplemento asili nido (Opslag kinderopvang)", "Recargo guardería (Opslag kinderopvang)"],
        "NL_LOONHEFFING" => ["Loonheffing — Income tax + premiums", "Loonheffing — Einkommensteuer + Beiträge", "Loonheffing — Loonbelasting + premies", "Loonheffing — Imposta + contributi", "Loonheffing — Impuesto + cotizaciones"],
        "NL_NON_COUVERT" => ["Netherlands — data unavailable for this year", "Niederlande — Daten für dieses Jahr nicht verfügbar", "Nederland — gegevens niet beschikbaar voor dit jaar", "Paesi Bassi — dati non disponibili per quest'anno", "Países Bajos — datos no disponibles para este año"],
        // ── Monaco ──
        "MC_CAR" => ["CAR — Pension", "CAR — Rente", "CAR — Pensioen", "CAR — Pensione", "CAR — Pensión"],
        "MC_CCSS" => ["CCSS — Health/family", "CCSS — Kranken/Familie", "CCSS — Ziekte/gezin", "CCSS — Malattia/famiglia", "CCSS — Enfermedad/familia"],
        "MC_CHOM" => ["Unemployment", "Arbeitslosigkeit", "Werkloosheid", "Disoccupazione", "Desempleo"],
        // ── Chine ──
        "CN_YANGLAO" => ["养老保险 — Pension insurance", "养老保险 — Rentenversicherung", "养老保险 — Pensioenverzekering", "养老保险 — Assicurazione pensione", "养老保险 — Seguro de pensión"],
        "CN_YILIAO" => ["医疗保险 — Health insurance", "医疗保险 — Krankenversicherung", "医疗保险 — Ziektekostenverzekering", "医疗保险 — Assicurazione malattia", "医疗保险 — Seguro de enfermedad"],
        "CN_SHIYE" => ["失业保险 — Unemployment insurance", "失业保险 — Arbeitslosenversicherung", "失业保险 — Werkloosheidsverzekering", "失业保险 — Assicurazione disoccupazione", "失业保险 — Seguro de desempleo"],
        "CN_GONGSHANG" => ["工伤保险 — Work accidents", "工伤保险 — Arbeitsunfälle", "工伤保险 — Arbeidsongevallen", "工伤保险 — Infortuni sul lavoro", "工伤保险 — Accidentes laborales"],
        "CN_SHENGYU" => ["生育保险 — Maternity insurance", "生育保险 — Mutterschaftsversicherung", "生育保险 — Moederschapsverzekering", "生育保险 — Assicurazione maternità", "生育保险 — Seguro de maternidad"],
        "CN_GONGJIJIN" => ["住房公积金 — Mandatory housing fund", "住房公积金 — Pflicht-Wohnungsfonds", "住房公积金 — Verplicht huisvestingsfonds", "住房公积金 — Fondo casa obbligatorio", "住房公积金 — Fondo de vivienda obligatorio"],
        "CN_IIT" => ["个人所得税 — Income tax (IIT)", "个人所得税 — Einkommensteuer (IIT)", "个人所得税 — Inkomstenbelasting (IIT)", "个人所得税 — Imposta sul reddito (IIT)", "个人所得税 — Impuesto sobre la renta (IIT)"],
        // ── Royaume-Uni (placeholder {fy}) ──
        "UK_NI_SAL" => ["National Insurance Class 1 — employee {fy}", "National Insurance Class 1 — Arbeitnehmer {fy}", "National Insurance Class 1 — werknemer {fy}", "National Insurance Class 1 — dipendente {fy}", "National Insurance Class 1 — trabajador {fy}"],
        "UK_NI_PAT" => ["National Insurance Class 1 — employer {fy}", "National Insurance Class 1 — Arbeitgeber {fy}", "National Insurance Class 1 — werkgever {fy}", "National Insurance Class 1 — datore di lavoro {fy}", "National Insurance Class 1 — empleador {fy}"],
        "UK_INCOME_TAX" => ["Income Tax PAYE — withholding {fy}", "Income Tax PAYE — Einbehalt {fy}", "Income Tax PAYE — inhouding {fy}", "Income Tax PAYE — ritenuta {fy}", "Income Tax PAYE — retención {fy}"],
        // ── Australie ──
        "AU_INCOME_TAX" => ["Income tax (PAYG)", "Income tax — Einkommensteuer (PAYG)", "Income tax — Inkomstenbelasting (PAYG)", "Income tax — Imposta sul reddito (PAYG)", "Income tax — Impuesto sobre la renta (PAYG)"],
        "AU_MEDICARE" => ["Medicare levy — Health contribution (2 %)", "Medicare levy — Gesundheitsbeitrag (2 %)", "Medicare levy — Gezondheidsbijdrage (2 %)", "Medicare levy — Contributo sanitario (2 %)", "Medicare levy — Contribución sanitaria (2 %)"],
        "AU_SUPER" => ["Superannuation Guarantee — Pension (employer)", "Superannuation Guarantee — Rente (Arbeitgeber)", "Superannuation Guarantee — Pensioen (werkgever)", "Superannuation Guarantee — Pensione (datore di lavoro)", "Superannuation Guarantee — Pensión (empleador)"],
        // ── Belgique ──
        "BE_ONSS_SAL" => ["ONSS — personal employee contribution", "ONSS — persönlicher Arbeitnehmerbeitrag", "ONSS (RSZ) — persoonlijke werknemersbijdrage", "ONSS — contributo personale del dipendente", "ONSS — cotización personal del trabajador"],
        "BE_ONSS_PAT" => ["ONSS — employer contribution (global rate)", "ONSS — Arbeitgeberbeitrag (Gesamtsatz)", "ONSS (RSZ) — werkgeversbijdrage (globaal tarief)", "ONSS — contributo datoriale (aliquota globale)", "ONSS — cotización patronal (tipo global)"],
        "BE_BONUS_EMPLOI" => ["Work bonus — ONSS contribution reduction {annee}", "Beschäftigungsbonus — ONSS-Beitragsermäßigung {annee}", "Werkbonus — ONSS-bijdragevermindering {annee}", "Bonus occupazione — riduzione contributi ONSS {annee}", "Bono de empleo — reducción de cotizaciones ONSS {annee}"],
        "BE_RED_STRUCT" => ["Structural employer reduction {annee}", "Strukturelle Arbeitgeberermäßigung {annee}", "Structurele werkgeversvermindering {annee}", "Riduzione strutturale datoriale {annee}", "Reducción estructural patronal {annee}"],
        "BE_PP" => ["Professional withholding (PP/BV) {annee} — {rl}", "Berufssteuervorabzug (PP/BV) {annee} — {rl}", "Bedrijfsvoorheffing (PP/BV) {annee} — {rl}", "Ritenuta professionale (PP/BV) {annee} — {rl}", "Retención profesional (PP/BV) {annee} — {rl}"],
        "BE_REG_WALLONIE" => ["Wallonia (+9 % regional surcharges)", "Wallonien (+9 % regionale Zuschläge)", "Wallonië (+9 % regionale opcentiemen)", "Vallonia (+9 % addizionali regionali)", "Valonia (+9 % recargos regionales)"],
        "BE_REG_FLANDRE" => ["Flanders (Flemish reduction −{korting} %)", "Flandern (flämische Ermäßigung −{korting} %)", "Vlaanderen (Vlaamse korting −{korting} %)", "Fiandre (riduzione fiamminga −{korting} %)", "Flandes (reducción flamenca −{korting} %)"],
        "BE_REG_BXL" => ["Brussels-Capital (no surcharge)", "Brüssel-Hauptstadt (kein Zuschlag)", "Brussel-Hoofdstad (geen opcentiemen)", "Bruxelles-Capitale (nessuna addizionale)", "Bruselas-Capital (sin recargo)"],
        // ── Canada ──
        "CA_RPC" => ["CPP — Canada Pension Plan", "CPP/RPC — Canada Pension Plan (Rente)", "CPP/RPC — Canada Pension Plan (pensioen)", "CPP/RPC — Canada Pension Plan (pensione)", "CPP/RPC — Plan de Pensiones de Canadá"],
        "CA_RPC2" => ["CPP2 — Additional enhancement (phase 2)", "CPP2/RPC2 — Zusatzaufstockung (Phase 2)", "CPP2/RPC2 — Aanvullende verhoging (fase 2)", "CPP2/RPC2 — Potenziamento aggiuntivo (fase 2)", "CPP2/RPC2 — Mejora adicional (fase 2)"],
        "CA_AE" => ["EI — Employment Insurance (general scheme)", "EI/AE — Arbeitslosenversicherung (allgemeines System)", "EI/AE — Werkloosheidsverzekering (algemeen stelsel)", "EI/AE — Assicurazione contro la disoccupazione (regime generale)", "EI/AE — Seguro de empleo (régimen general)"],
        "CA_IMPOT_FED" => ["Federal tax — {annee} withholding", "Bundessteuer — Einbehalt {annee}", "Federale belasting — inhouding {annee}", "Imposta federale — ritenuta {annee}", "Impuesto federal — retención {annee}"],
        "ON_IMPOT_PROV" => ["Ontario provincial tax — {annee} withholding", "Provinzsteuer Ontario — Einbehalt {annee}", "Provinciale belasting Ontario — inhouding {annee}", "Imposta provinciale Ontario — ritenuta {annee}", "Impuesto provincial de Ontario — retención {annee}"],
        "QC_IMPOT_PROV" => ["Quebec provincial tax — {annee} withholding", "Provinzsteuer Québec — Einbehalt {annee}", "Provinciale belasting Québec — inhouding {annee}", "Imposta provinciale Québec — ritenuta {annee}", "Impuesto provincial de Quebec — retención {annee}"],
        "AB_IMPOT_PROV" | "BC_IMPOT_PROV" | "MB_IMPOT_PROV" | "NB_IMPOT_PROV" | "NL_IMPOT_PROV"
        | "NS_IMPOT_PROV" | "NT_IMPOT_PROV" | "NU_IMPOT_PROV" | "PE_IMPOT_PROV" | "SK_IMPOT_PROV"
        | "YT_IMPOT_PROV" => ["{nom} provincial tax — {annee} withholding", "Provinzsteuer {nom} — Einbehalt {annee}", "Provinciale belasting {nom} — inhouding {annee}", "Imposta provinciale {nom} — ritenuta {annee}", "Impuesto provincial {nom} — retención {annee}"],
        // ── Québec ──
        "QC_RRQ" => ["QPP — Quebec Pension Plan", "QPP/RRQ — Quebec Pension Plan (Rente)", "QPP/RRQ — Quebec Pension Plan (pensioen)", "QPP/RRQ — Quebec Pension Plan (pensione)", "QPP/RRQ — Plan de Rentas de Quebec"],
        "QC_RRQ2" => ["QPP2 — Additional enhancement (phase 2)", "QPP2/RRQ2 — Zusatzaufstockung (Phase 2)", "QPP2/RRQ2 — Aanvullende verhoging (fase 2)", "QPP2/RRQ2 — Potenziamento aggiuntivo (fase 2)", "QPP2/RRQ2 — Mejora adicional (fase 2)"],
        "QC_AE" => ["EI — Employment Insurance (reduced Quebec rate)", "EI/AE — Arbeitslosenversicherung (reduzierter Québec-Satz)", "EI/AE — Werkloosheidsverzekering (verlaagd Québec-tarief)", "EI/AE — Assicurazione disoccupazione (aliquota ridotta Québec)", "EI/AE — Seguro de empleo (tipo reducido de Quebec)"],
        "QC_RQAP" => ["QPIP — Parental insurance (Quebec)", "QPIP/RQAP — Elternversicherung (Québec)", "QPIP/RQAP — Ouderschapsverzekering (Québec)", "QPIP/RQAP — Assicurazione parentale (Québec)", "QPIP/RQAP — Seguro parental (Quebec)"],
        "QC_FSS" => ["HSF — Health Services Fund (Quebec)", "HSF/FSS — Gesundheitsdienstefonds (Québec)", "HSF/FSS — Fonds voor gezondheidsdiensten (Québec)", "HSF/FSS — Fondo per i servizi sanitari (Québec)", "HSF/FSS — Fondo de servicios de salud (Quebec)"],
        "QC_CNT" => ["Labour-standards contribution (CNESST)", "Arbeitsnormenbeitrag (CNESST)", "Bijdrage arbeidsnormen (CNESST)", "Contributo norme del lavoro (CNESST)", "Contribución a las normas laborales (CNESST)"],
        _ => return None,
    };
    Some(row[i])
}

pub fn t_explication(key: &str, lang: &str) -> Option<&'static str> {
    let i = match lang { "en" => 0, "de" => 1, "nl" => 2, "it" => 3, "es" => 4, _ => return None };
    let row: [&str; 5] = match key {
        // ── Gabarits génériques par pays (placeholders {libelle} {ts} {tp} [{ms}]) ──
        "CZ_GENERIC" | "EE_GENERIC" | "RO_GENERIC" => [
            "{libelle}. Employee {ts} % / employer {tp} %.",
            "{libelle}. Arbeitnehmer {ts} % / Arbeitgeber {tp} %.",
            "{libelle}. Werknemer {ts} % / werkgever {tp} %.",
            "{libelle}. Dipendente {ts} % / datore di lavoro {tp} %.",
            "{libelle}. Trabajador {ts} % / empleador {tp} %.",
        ],
        "FI_GENERIC" => [
            "{libelle}. Employee {ts} % / employer {tp} %. Employee: {ms} €.",
            "{libelle}. Arbeitnehmer {ts} % / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.",
            "{libelle}. Werknemer {ts} % / werkgever {tp} %. Werknemer: {ms} €.",
            "{libelle}. Dipendente {ts} % / datore di lavoro {tp} %. Dipendente: {ms} €.",
            "{libelle}. Trabajador {ts} % / empleador {tp} %. Trabajador: {ms} €.",
        ],
        "MC_GENERIC" => [
            "{libelle} — Caisses Sociales de Monaco.\nEmployee {ts} % / employer {tp} %. Employee: {ms} €.\n\nNote: Monaco levies no income tax on residents (except French nationals — 1963 tax treaty).",
            "{libelle} — Caisses Sociales de Monaco.\nArbeitnehmer {ts} % / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.\n\nHinweis: Monaco erhebt keine Einkommensteuer auf Gebietsansässige (außer französische Staatsangehörige — Steuerabkommen 1963).",
            "{libelle} — Caisses Sociales de Monaco.\nWerknemer {ts} % / werkgever {tp} %. Werknemer: {ms} €.\n\nNoot: Monaco heft geen inkomstenbelasting op inwoners (behalve Franse staatsburgers — belastingverdrag 1963).",
            "{libelle} — Caisses Sociales de Monaco.\nDipendente {ts} % / datore di lavoro {tp} %. Dipendente: {ms} €.\n\nNota: Monaco non preleva imposte sul reddito dei residenti (salvo cittadini francesi — convenzione fiscale 1963).",
            "{libelle} — Caisses Sociales de Monaco.\nTrabajador {ts} % / empleador {tp} %. Trabajador: {ms} €.\n\nNota: Mónaco no grava la renta de los residentes (salvo nacionales franceses — convenio fiscal de 1963).",
        ],
        "PL_GENERIC" => [
            "{libelle} — ZUS.\nRate: {tsp} % empl / {tpp} % empr. Base: {base} PLN.\nEmployee: {ms} PLN | Employer: {mp} PLN.\n\nLegal basis: Ustawa o systemie ubezpieczeń społecznych.",
            "{libelle} — ZUS.\nSatz: {tsp} % AN / {tpp} % AG. Grundlage: {base} PLN.\nArbeitnehmer: {ms} PLN | Arbeitgeber: {mp} PLN.\n\nRechtsgrundlage: Ustawa o systemie ubezpieczeń społecznych.",
            "{libelle} — ZUS.\nTarief: {tsp} % wn / {tpp} % wg. Grondslag: {base} PLN.\nWerknemer: {ms} PLN | Werkgever: {mp} PLN.\n\nWettelijke basis: Ustawa o systemie ubezpieczeń społecznych.",
            "{libelle} — ZUS.\nAliquota: {tsp} % dip / {tpp} % dat. Base: {base} PLN.\nDipendente: {ms} PLN | Datore di lavoro: {mp} PLN.\n\nBase giuridica: Ustawa o systemie ubezpieczeń społecznych.",
            "{libelle} — ZUS.\nTipo: {tsp} % trab / {tpp} % empr. Base: {base} PLN.\nTrabajador: {ms} PLN | Empleador: {mp} PLN.\n\nBase legal: Ustawa o systemie ubezpieczeń społecznych.",
        ],
        "NL_PAT_GENERIC" => [
            "{libelle} — employer premium (werkgeversheffing).\n\nRate: {tp} %\nBase: {base} € (min(gross, monthly maximumpremieloon {plaf} €))\nEmployer: {mp} €\n\nLegal basis: Wfsv / Zorgverzekeringswet.",
            "{libelle} — Arbeitgeberprämie (werkgeversheffing).\n\nSatz: {tp} %\nGrundlage: {base} € (min(brutto, monatliches maximumpremieloon {plaf} €))\nArbeitgeber: {mp} €\n\nRechtsgrundlage: Wfsv / Zorgverzekeringswet.",
            "{libelle} — werkgeverspremie (werkgeversheffing).\n\nTarief: {tp} %\nGrondslag: {base} € (min(bruto, maandelijks maximumpremieloon {plaf} €))\nWerkgever: {mp} €\n\nWettelijke basis: Wfsv / Zorgverzekeringswet.",
            "{libelle} — premio datoriale (werkgeversheffing).\n\nAliquota: {tp} %\nBase: {base} € (min(lordo, maximumpremieloon mensile {plaf} €))\nDatore di lavoro: {mp} €\n\nBase giuridica: Wfsv / Zorgverzekeringswet.",
            "{libelle} — prima patronal (werkgeversheffing).\n\nTipo: {tp} %\nBase: {base} € (mín(bruto, maximumpremieloon mensual {plaf} €))\nEmpleador: {mp} €\n\nBase legal: Wfsv / Zorgverzekeringswet.",
        ],
        "CN_GENERIC" => [
            "{expl}\nClamped base: ¥{base} (gross ¥{brut}, min ¥{min}–max ¥{max})\nEmployee: {ts_pct} % = ¥{ms} | Employer: {tp_pct} % = ¥{mp}",
            "{expl}\nBegrenzte Grundlage: ¥{base} (brutto ¥{brut}, min ¥{min}–max ¥{max})\nArbeitnehmer: {ts_pct} % = ¥{ms} | Arbeitgeber: {tp_pct} % = ¥{mp}",
            "{expl}\nBegrensde grondslag: ¥{base} (bruto ¥{brut}, min ¥{min}–max ¥{max})\nWerknemer: {ts_pct} % = ¥{ms} | Werkgever: {tp_pct} % = ¥{mp}",
            "{expl}\nBase limitata: ¥{base} (lordo ¥{brut}, min ¥{min}–max ¥{max})\nDipendente: {ts_pct} % = ¥{ms} | Datore di lavoro: {tp_pct} % = ¥{mp}",
            "{expl}\nBase limitada: ¥{base} (bruto ¥{brut}, mín ¥{min}–máx ¥{max})\nTrabajador: {ts_pct} % = ¥{ms} | Empleador: {tp_pct} % = ¥{mp}",
        ],

        // ── Andorre ──
        "AD_CASS" => [
            "CASS — social security (general branch + pension).\nEmployee {ts} % / employer {tp} %. Employee: {ms} €.\n\nLegal basis: Llei 17/2008.",
            "CASS — Sozialversicherung (allgemeiner Zweig + Rente).\nArbeitnehmer {ts} % / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.\n\nRechtsgrundlage: Llei 17/2008.",
            "CASS — sociale zekerheid (algemene tak + pensioen).\nWerknemer {ts} % / werkgever {tp} %. Werknemer: {ms} €.\n\nWettelijke basis: Llei 17/2008.",
            "CASS — sicurezza sociale (ramo generale + pensione).\nDipendente {ts} % / datore di lavoro {tp} %. Dipendente: {ms} €.\n\nBase giuridica: Llei 17/2008.",
            "CASS — seguridad social (rama general + pensión).\nTrabajador {ts} % / empleador {tp} %. Trabajador: {ms} €.\n\nBase legal: Llei 17/2008.",
        ],
        "AD_IRPF" => [
            "IRPF — income tax (annualised).\n\nAnnual income: {ra} €\n• 0 % up to 24,000 €\n• 5 % from 24,001 to 40,000 €\n• 10 % above 40,000 €\n= {ia} €/year / 12 = {im} €/month.\n\nLegal basis: Llei 5/2014 (IRPF).",
            "IRPF — Einkommensteuer (auf Jahresbasis).\n\nJahreseinkommen: {ra} €\n• 0 % bis 24.000 €\n• 5 % von 24.001 bis 40.000 €\n• 10 % über 40.000 €\n= {ia} €/Jahr / 12 = {im} €/Monat.\n\nRechtsgrundlage: Llei 5/2014 (IRPF).",
            "IRPF — inkomstenbelasting (op jaarbasis).\n\nJaarinkomen: {ra} €\n• 0 % tot 24.000 €\n• 5 % van 24.001 tot 40.000 €\n• 10 % boven 40.000 €\n= {ia} €/jaar / 12 = {im} €/maand.\n\nWettelijke basis: Llei 5/2014 (IRPF).",
            "IRPF — imposta sul reddito (annualizzata).\n\nReddito annuo: {ra} €\n• 0 % fino a 24.000 €\n• 5 % da 24.001 a 40.000 €\n• 10 % oltre 40.000 €\n= {ia} €/anno / 12 = {im} €/mese.\n\nBase giuridica: Llei 5/2014 (IRPF).",
            "IRPF — impuesto sobre la renta (anualizado).\n\nRenta anual: {ra} €\n• 0 % hasta 24.000 €\n• 5 % de 24.001 a 40.000 €\n• 10 % por encima de 40.000 €\n= {ia} €/año / 12 = {im} €/mes.\n\nBase legal: Llei 5/2014 (IRPF).",
        ],
        // ── Autriche ──
        "AT_SV" => [
            "Sozialversicherung — employee {ts} % / employer {tp} % (pension PV, health KV, unemployment ALV, AK, WBF). Base capped at 6,450 €/month (Höchstbeitragsgrundlage). Employee: {ms} €.",
            "Sozialversicherung — Arbeitnehmer {ts} % / Arbeitgeber {tp} % (Pension PV, Kranken KV, Arbeitslosen ALV, AK, WBF). Bemessungsgrundlage gedeckelt auf 6.450 €/Monat (Höchstbeitragsgrundlage). Arbeitnehmer: {ms} €.",
            "Sozialversicherung — werknemer {ts} % / werkgever {tp} % (pensioen PV, ziekte KV, werkloosheid ALV, AK, WBF). Grondslag begrensd op 6.450 €/maand (Höchstbeitragsgrundlage). Werknemer: {ms} €.",
            "Sozialversicherung — dipendente {ts} % / datore di lavoro {tp} % (pensione PV, malattia KV, disoccupazione ALV, AK, WBF). Base limitata a 6.450 €/mese (Höchstbeitragsgrundlage). Dipendente: {ms} €.",
            "Sozialversicherung — trabajador {ts} % / empleador {tp} % (pensión PV, enfermedad KV, desempleo ALV, AK, WBF). Base limitada a 6.450 €/mes (Höchstbeitragsgrundlage). Trabajador: {ms} €.",
        ],
        "AT_LOHNSTEUER" => [
            "Income tax 2025 (annualised).\n\nBase = (gross − employee SV) × 12 = {b} €\nScale 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(thresholds 13,308 / 21,617 / 35,836 / 69,166 / 103,072 / 1,000,000 €)\n→ {im} €/month.\n\nNote: 13th/14th salary (Sonderzahlungen) and credits not modelled (conservative net).\nSource: BMF.",
            "Einkommensteuer 2025 (auf Jahresbasis).\n\nBemessung = (brutto − AN-SV) × 12 = {b} €\nTarif 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(Grenzen 13.308 / 21.617 / 35.836 / 69.166 / 103.072 / 1.000.000 €)\n→ {im} €/Monat.\n\nHinweis: 13./14. Gehalt (Sonderzahlungen) und Absetzbeträge nicht modelliert (vorsichtiger Nettowert).\nQuelle: BMF.",
            "Inkomstenbelasting 2025 (op jaarbasis).\n\nGrondslag = (bruto − werknemers-SV) × 12 = {b} €\nSchaal 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(drempels 13.308 / 21.617 / 35.836 / 69.166 / 103.072 / 1.000.000 €)\n→ {im} €/maand.\n\nNoot: 13e/14e maand (Sonderzahlungen) en kortingen niet gemodelleerd (voorzichtig netto).\nBron: BMF.",
            "Imposta sul reddito 2025 (annualizzata).\n\nBase = (lordo − SV dipendente) × 12 = {b} €\nScala 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(soglie 13.308 / 21.617 / 35.836 / 69.166 / 103.072 / 1.000.000 €)\n→ {im} €/mese.\n\nNota: 13ª/14ª mensilità (Sonderzahlungen) e detrazioni non modellate (netto prudente).\nFonte: BMF.",
            "Impuesto sobre la renta 2025 (anualizado).\n\nBase = (bruto − SV trabajador) × 12 = {b} €\nEscala 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(umbrales 13.308 / 21.617 / 35.836 / 69.166 / 103.072 / 1.000.000 €)\n→ {im} €/mes.\n\nNota: pagas 13ª/14ª (Sonderzahlungen) y deducciones no modeladas (neto prudente).\nFuente: BMF.",
        ],
        // ── Bulgarie ──
        "BG_OSIG" => [
            "Social contributions — employee {ts} % / employer {tp} % (pension, health NZOK, 2nd pillar). Base capped at 3,750 BGN/month. Employee: {ms} BGN.",
            "Sozialbeiträge — Arbeitnehmer {ts} % / Arbeitgeber {tp} % (Rente, Kranken NZOK, 2. Säule). Bemessungsgrundlage gedeckelt auf 3.750 BGN/Monat. Arbeitnehmer: {ms} BGN.",
            "Sociale bijdragen — werknemer {ts} % / werkgever {tp} % (pensioen, ziekte NZOK, 2e pijler). Grondslag begrensd op 3.750 BGN/maand. Werknemer: {ms} BGN.",
            "Contributi sociali — dipendente {ts} % / datore di lavoro {tp} % (pensione, malattia NZOK, 2º pilastro). Base limitata a 3.750 BGN/mese. Dipendente: {ms} BGN.",
            "Cotizaciones sociales — trabajador {ts} % / empleador {tp} % (pensión, enfermedad NZOK, 2º pilar). Base limitada a 3.750 BGN/mes. Trabajador: {ms} BGN.",
        ],
        "BG_DANAK" => [
            "Income tax 2025: flat 10 %.\n\nBase = gross − employee contributions = {b} BGN → {im} BGN/month.\n\nSource: НАП (NRA).",
            "Einkommensteuer 2025: pauschal 10 %.\n\nBemessung = brutto − AN-Beiträge = {b} BGN → {im} BGN/Monat.\n\nQuelle: НАП (NRA).",
            "Inkomstenbelasting 2025: vlak 10 %.\n\nGrondslag = bruto − werknemersbijdragen = {b} BGN → {im} BGN/maand.\n\nBron: НАП (NRA).",
            "Imposta sul reddito 2025: proporzionale 10 %.\n\nBase = lordo − contributi dipendente = {b} BGN → {im} BGN/mese.\n\nFonte: НАП (NRA).",
            "Impuesto sobre la renta 2025: plano 10 %.\n\nBase = bruto − cotizaciones del trabajador = {b} BGN → {im} BGN/mes.\n\nFuente: НАП (NRA).",
        ],
        // ── Chypre ──
        "CY_SI" => [
            "Social insurance — employee {ts} % / employer {tp} %. Base capped at 5,551 €/month.",
            "Sozialversicherung — Arbeitnehmer {ts} % / Arbeitgeber {tp} %. Bemessungsgrundlage gedeckelt auf 5.551 €/Monat.",
            "Sociale verzekering — werknemer {ts} % / werkgever {tp} %. Grondslag begrensd op 5.551 €/maand.",
            "Assicurazione sociale — dipendente {ts} % / datore di lavoro {tp} %. Base limitata a 5.551 €/mese.",
            "Seguro social — trabajador {ts} % / empleador {tp} %. Base limitada a 5.551 €/mes.",
        ],
        "CY_GESY" => [
            "GESY (health) — employee {ts} % / employer {tp} %.",
            "GESY (Gesundheit) — Arbeitnehmer {ts} % / Arbeitgeber {tp} %.",
            "GESY (gezondheid) — werknemer {ts} % / werkgever {tp} %.",
            "GESY (sanità) — dipendente {ts} % / datore di lavoro {tp} %.",
            "GESY (salud) — trabajador {ts} % / empleador {tp} %.",
        ],
        "CY_FOROS" => [
            "Income tax 2025 (annualised).\n\nBase = (gross − contributions) × 12 = {b} €\nScale 0 / 20 / 25 / 30 / 35 % (thresholds 19,500 / 28,000 / 36,300 / 60,000 €)\n→ {im} €/month.\n\nSource: Τμήμα Φορολογίας.",
            "Einkommensteuer 2025 (auf Jahresbasis).\n\nBemessung = (brutto − Beiträge) × 12 = {b} €\nTarif 0 / 20 / 25 / 30 / 35 % (Grenzen 19.500 / 28.000 / 36.300 / 60.000 €)\n→ {im} €/Monat.\n\nQuelle: Τμήμα Φορολογίας.",
            "Inkomstenbelasting 2025 (op jaarbasis).\n\nGrondslag = (bruto − bijdragen) × 12 = {b} €\nSchaal 0 / 20 / 25 / 30 / 35 % (drempels 19.500 / 28.000 / 36.300 / 60.000 €)\n→ {im} €/maand.\n\nBron: Τμήμα Φορολογίας.",
            "Imposta sul reddito 2025 (annualizzata).\n\nBase = (lordo − contributi) × 12 = {b} €\nScala 0 / 20 / 25 / 30 / 35 % (soglie 19.500 / 28.000 / 36.300 / 60.000 €)\n→ {im} €/mese.\n\nFonte: Τμήμα Φορολογίας.",
            "Impuesto sobre la renta 2025 (anualizado).\n\nBase = (bruto − cotizaciones) × 12 = {b} €\nEscala 0 / 20 / 25 / 30 / 35 % (umbrales 19.500 / 28.000 / 36.300 / 60.000 €)\n→ {im} €/mes.\n\nFuente: Τμήμα Φορολογίας.",
        ],
        // ── Tchéquie ──
        "CZ_DAN" => [
            "Income tax 2025.\n\n15 % up to 139,671 CZK/month, 23 % above = {ib} CZK\n− sleva na poplatníka 2,570 CZK = {im} CZK/month.\n\nSource: Finanční správa.",
            "Einkommensteuer 2025.\n\n15 % bis 139.671 CZK/Monat, 23 % darüber = {ib} CZK\n− sleva na poplatníka 2.570 CZK = {im} CZK/Monat.\n\nQuelle: Finanční správa.",
            "Inkomstenbelasting 2025.\n\n15 % tot 139.671 CZK/maand, 23 % daarboven = {ib} CZK\n− sleva na poplatníka 2.570 CZK = {im} CZK/maand.\n\nBron: Finanční správa.",
            "Imposta sul reddito 2025.\n\n15 % fino a 139.671 CZK/mese, 23 % oltre = {ib} CZK\n− sleva na poplatníka 2.570 CZK = {im} CZK/mese.\n\nFonte: Finanční správa.",
            "Impuesto sobre la renta 2025.\n\n15 % hasta 139.671 CZK/mes, 23 % por encima = {ib} CZK\n− sleva na poplatníka 2.570 CZK = {im} CZK/mes.\n\nFuente: Finanční správa.",
        ],
        // ── Danemark ──
        "DK_AM" => [
            "AM-bidrag — 8 % of gross salary, deducted before tax.\nAmount: {am} DKK.\n\nLegal basis: Arbejdsmarkedsbidragsloven.",
            "AM-bidrag — 8 % des Bruttolohns, vor Steuer einbehalten.\nBetrag: {am} DKK.\n\nRechtsgrundlage: Arbejdsmarkedsbidragsloven.",
            "AM-bidrag — 8 % van het brutoloon, ingehouden vóór belasting.\nBedrag: {am} DKK.\n\nWettelijke basis: Arbejdsmarkedsbidragsloven.",
            "AM-bidrag — 8 % della retribuzione lorda, trattenuto prima delle imposte.\nImporto: {am} DKK.\n\nBase giuridica: Arbejdsmarkedsbidragsloven.",
            "AM-bidrag — 8 % del salario bruto, retenido antes del impuesto.\nImporte: {am} DKK.\n\nBase legal: Arbejdsmarkedsbidragsloven.",
        ],
        "DK_ATP" => [
            "ATP — labour market supplementary pension (flat rate).\nFull-time 2025: {a} DKK/month employee (2/3 employer).\n\nLegal basis: ATP-loven.",
            "ATP — Arbeitsmarkt-Zusatzrente (Pauschale).\nVollzeit 2025: {a} DKK/Monat Arbeitnehmer (2/3 Arbeitgeber).\n\nRechtsgrundlage: ATP-loven.",
            "ATP — aanvullend arbeidsmarktpensioen (forfait).\nVoltijd 2025: {a} DKK/maand werknemer (2/3 werkgever).\n\nWettelijke basis: ATP-loven.",
            "ATP — pensione complementare del mercato del lavoro (forfait).\nTempo pieno 2025: {a} DKK/mese dipendente (2/3 datore di lavoro).\n\nBase giuridica: ATP-loven.",
            "ATP — pensión complementaria del mercado laboral (tanto alzado).\nTiempo completo 2025: {a} DKK/mes trabajador (2/3 empleador).\n\nBase legal: ATP-loven.",
        ],
        "DK_INDKOMSTSKAT" => [
            "Income tax — bundskat 12.01 % + average kommuneskat 25.1 % (= 37.11 %)\non income after AM-bidrag, ATP and personfradrag (4,300 DKK/month).\n+ topskat 15 % above {ts} DKK/month (income after AM).\nTaxable base: {tx} DKK → {ib} DKK; topskat {tk} DKK.\n= {im} DKK/month.\n\nLegal basis: Personskatteloven (2025). Kommuneskat = national average.",
            "Einkommensteuer — bundskat 12,01 % + durchschnittliche kommuneskat 25,1 % (= 37,11 %)\nauf das Einkommen nach AM-bidrag, ATP und personfradrag (4.300 DKK/Monat).\n+ topskat 15 % über {ts} DKK/Monat (Einkommen nach AM).\nBemessungsgrundlage: {tx} DKK → {ib} DKK; topskat {tk} DKK.\n= {im} DKK/Monat.\n\nRechtsgrundlage: Personskatteloven (2025). Kommuneskat = Landesdurchschnitt.",
            "Inkomstenbelasting — bundskat 12,01 % + gemiddelde kommuneskat 25,1 % (= 37,11 %)\nop het inkomen na AM-bidrag, ATP en personfradrag (4.300 DKK/maand).\n+ topskat 15 % boven {ts} DKK/maand (inkomen na AM).\nBelastbare grondslag: {tx} DKK → {ib} DKK; topskat {tk} DKK.\n= {im} DKK/maand.\n\nWettelijke basis: Personskatteloven (2025). Kommuneskat = landelijk gemiddelde.",
            "Imposta sul reddito — bundskat 12,01 % + kommuneskat media 25,1 % (= 37,11 %)\nsul reddito dopo AM-bidrag, ATP e personfradrag (4.300 DKK/mese).\n+ topskat 15 % oltre {ts} DKK/mese (reddito dopo AM).\nBase imponibile: {tx} DKK → {ib} DKK; topskat {tk} DKK.\n= {im} DKK/mese.\n\nBase giuridica: Personskatteloven (2025). Kommuneskat = media nazionale.",
            "Impuesto sobre la renta — bundskat 12,01 % + kommuneskat media 25,1 % (= 37,11 %)\nsobre la renta tras AM-bidrag, ATP y personfradrag (4.300 DKK/mes).\n+ topskat 15 % por encima de {ts} DKK/mes (renta tras AM).\nBase imponible: {tx} DKK → {ib} DKK; topskat {tk} DKK.\n= {im} DKK/mes.\n\nBase legal: Personskatteloven (2025). Kommuneskat = media nacional.",
        ],
        // ── Estonie ──
        "EE_TULUMAKS" => [
            "Income tax 2025: flat 22 %.\n\nAnnual income {g} € − employee contributions − basic allowance {ab} €\n= taxable base {b} € → {im} €/month.\n\nTapering basic allowance (7,848 € if ≤ 14,400 €/yr, nil if ≥ 25,200 €/yr).\nSource: Maksu- ja Tolliamet.",
            "Einkommensteuer 2025: einheitlich 22 %.\n\nJahreseinkommen {g} € − AN-Beiträge − Grundfreibetrag {ab} €\n= Bemessungsgrundlage {b} € → {im} €/Monat.\n\nGleitender Grundfreibetrag (7.848 € bei ≤ 14.400 €/Jahr, 0 bei ≥ 25.200 €/Jahr).\nQuelle: Maksu- ja Tolliamet.",
            "Inkomstenbelasting 2025: vlak 22 %.\n\nJaarinkomen {g} € − werknemersbijdragen − basisaftrek {ab} €\n= belastbare grondslag {b} € → {im} €/maand.\n\nAflopende basisaftrek (7.848 € bij ≤ 14.400 €/jr, nul bij ≥ 25.200 €/jr).\nBron: Maksu- ja Tolliamet.",
            "Imposta sul reddito 2025: aliquota unica 22 %.\n\nReddito annuo {g} € − contributi dipendente − detrazione di base {ab} €\n= base imponibile {b} € → {im} €/mese.\n\nDetrazione di base decrescente (7.848 € se ≤ 14.400 €/anno, nulla se ≥ 25.200 €/anno).\nFonte: Maksu- ja Tolliamet.",
            "Impuesto sobre la renta 2025: tipo único 22 %.\n\nRenta anual {g} € − cotizaciones del trabajador − mínimo exento {ab} €\n= base imponible {b} € → {im} €/mes.\n\nMínimo exento decreciente (7.848 € si ≤ 14.400 €/año, nulo si ≥ 25.200 €/año).\nFuente: Maksu- ja Tolliamet.",
        ],
        // ── Finlande ──
        "FI_PAIVARAHA" => [
            "Päivärahamaksu — 0.88 % (only if annual income ≥ 17,255 €). Deductible.\nAnnual income: {g} € → {m} €/month.",
            "Päivärahamaksu — 0,88 % (nur bei Jahreseinkommen ≥ 17.255 €). Abzugsfähig.\nJahreseinkommen: {g} € → {m} €/Monat.",
            "Päivärahamaksu — 0,88 % (alleen bij jaarinkomen ≥ 17.255 €). Aftrekbaar.\nJaarinkomen: {g} € → {m} €/maand.",
            "Päivärahamaksu — 0,88 % (solo se reddito annuo ≥ 17.255 €). Deducibile.\nReddito annuo: {g} € → {m} €/mese.",
            "Päivärahamaksu — 0,88 % (solo si renta anual ≥ 17.255 €). Deducible.\nRenta anual: {g} € → {m} €/mes.",
        ],
        "FI_TULOVERO" => [
            "Income tax 2026 (annualised).\n\nTaxable income: {g} € − deductible contributions {ded} € = {tx} €\nState scale: 12.64 % / 19 % / 30.25 % / 33.25 % / 37.5 %\n(thresholds 21,200 / 32,600 / 40,100 / 52,100 €) → {et} €\nAverage municipal tax 7.50 % → {co} €\n= {im} €/month.\n\nNote: työtulovähennys / perusvähennys credits not modelled (conservative net).\nLegal basis: Tuloverolaki.",
            "Einkommensteuer 2026 (auf Jahresbasis).\n\nZu versteuern: {g} € − abzugsfähige Beiträge {ded} € = {tx} €\nStaatstarif: 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n(Grenzen 21.200 / 32.600 / 40.100 / 52.100 €) → {et} €\nDurchschn. Gemeindesteuer 7,50 % → {co} €\n= {im} €/Monat.\n\nHinweis: työtulovähennys / perusvähennys nicht modelliert (vorsichtiger Nettowert).\nRechtsgrundlage: Tuloverolaki.",
            "Inkomstenbelasting 2026 (op jaarbasis).\n\nBelastbaar inkomen: {g} € − aftrekbare bijdragen {ded} € = {tx} €\nRijksschaal: 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n(drempels 21.200 / 32.600 / 40.100 / 52.100 €) → {et} €\nGemiddelde gemeentebelasting 7,50 % → {co} €\n= {im} €/maand.\n\nNoot: työtulovähennys / perusvähennys niet gemodelleerd (voorzichtig netto).\nWettelijke basis: Tuloverolaki.",
            "Imposta sul reddito 2026 (annualizzata).\n\nReddito imponibile: {g} € − contributi deducibili {ded} € = {tx} €\nScala statale: 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n(soglie 21.200 / 32.600 / 40.100 / 52.100 €) → {et} €\nImposta comunale media 7,50 % → {co} €\n= {im} €/mese.\n\nNota: crediti työtulovähennys / perusvähennys non modellati (netto prudente).\nBase giuridica: Tuloverolaki.",
            "Impuesto sobre la renta 2026 (anualizado).\n\nRenta imponible: {g} € − cotizaciones deducibles {ded} € = {tx} €\nEscala estatal: 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n(umbrales 21.200 / 32.600 / 40.100 / 52.100 €) → {et} €\nImpuesto municipal medio 7,50 % → {co} €\n= {im} €/mes.\n\nNota: créditos työtulovähennys / perusvähennys no modelados (neto prudente).\nFuente: Tuloverolaki.",
        ],
        // ── Grèce ──
        "GR_EFKA" => [
            "EFKA — employee {ts} % / employer {tp} % (pension, health, supplementary). Base capped at 7,572.62 €/month. Employee: {ms} €.",
            "EFKA — Arbeitnehmer {ts} % / Arbeitgeber {tp} % (Rente, Kranken, Zusatz). Bemessungsgrundlage gedeckelt auf 7.572,62 €/Monat. Arbeitnehmer: {ms} €.",
            "EFKA — werknemer {ts} % / werkgever {tp} % (pensioen, ziekte, aanvullend). Grondslag begrensd op 7.572,62 €/maand. Werknemer: {ms} €.",
            "EFKA — dipendente {ts} % / datore di lavoro {tp} % (pensione, malattia, complementare). Base limitata a 7.572,62 €/mese. Dipendente: {ms} €.",
            "EFKA — trabajador {ts} % / empleador {tp} % (pensión, enfermedad, complementaria). Base limitada a 7.572,62 €/mes. Trabajador: {ms} €.",
        ],
        "GR_FOROS" => [
            "Income tax 2025 (annualised).\n\nBase = (gross − EFKA) × 12 = {b} €\nScale 9 / 22 / 28 / 36 / 44 % (thresholds 10,000 / 20,000 / 30,000 / 40,000 €)\n− employee relief 777 € → {im} €/month.\n\nNote: child increases not modelled (conservative net).\nSource: AADE.",
            "Einkommensteuer 2025 (auf Jahresbasis).\n\nBemessung = (brutto − EFKA) × 12 = {b} €\nTarif 9 / 22 / 28 / 36 / 44 % (Grenzen 10.000 / 20.000 / 30.000 / 40.000 €)\n− AN-Ermäßigung 777 € → {im} €/Monat.\n\nHinweis: Kinderzuschläge nicht modelliert (vorsichtiger Nettowert).\nQuelle: AADE.",
            "Inkomstenbelasting 2025 (op jaarbasis).\n\nGrondslag = (bruto − EFKA) × 12 = {b} €\nSchaal 9 / 22 / 28 / 36 / 44 % (drempels 10.000 / 20.000 / 30.000 / 40.000 €)\n− werknemerskorting 777 € → {im} €/maand.\n\nNoot: kindverhogingen niet gemodelleerd (voorzichtig netto).\nBron: AADE.",
            "Imposta sul reddito 2025 (annualizzata).\n\nBase = (lordo − EFKA) × 12 = {b} €\nScala 9 / 22 / 28 / 36 / 44 % (soglie 10.000 / 20.000 / 30.000 / 40.000 €)\n− riduzione dipendente 777 € → {im} €/mese.\n\nNota: maggiorazioni per figli non modellate (netto prudente).\nFonte: AADE.",
            "Impuesto sobre la renta 2025 (anualizado).\n\nBase = (bruto − EFKA) × 12 = {b} €\nEscala 9 / 22 / 28 / 36 / 44 % (umbrales 10.000 / 20.000 / 30.000 / 40.000 €)\n− reducción del trabajador 777 € → {im} €/mes.\n\nNota: incrementos por hijos no modelados (neto prudente).\nFuente: AADE.",
        ],
        // ── Croatie ──
        "HR_MIROVINSKO" => [
            "Pension — {ts} % employee (1st pillar 15 % + 2nd pillar 5 %). Employee: {ms} €.",
            "Rente — {ts} % Arbeitnehmer (1. Säule 15 % + 2. Säule 5 %). Arbeitnehmer: {ms} €.",
            "Pensioen — {ts} % werknemer (1e pijler 15 % + 2e pijler 5 %). Werknemer: {ms} €.",
            "Pensione — {ts} % dipendente (1º pilastro 15 % + 2º pilastro 5 %). Dipendente: {ms} €.",
            "Pensión — {ts} % trabajador (1er pilar 15 % + 2º pilar 5 %). Trabajador: {ms} €.",
        ],
        "HR_ZDRAVSTVENO" => [
            "Health insurance — {tp} % borne by the employer.",
            "Krankenversicherung — {tp} % zu Lasten des Arbeitgebers.",
            "Ziektekostenverzekering — {tp} % ten laste van de werkgever.",
            "Assicurazione malattia — {tp} % a carico del datore di lavoro.",
            "Seguro de enfermedad — {tp} % a cargo del empleador.",
        ],
        "HR_POREZ" => [
            "Income tax 2025.\n\nBase = gross − pension − allowance 600 € = {b} €\n20 % up to 5,000 €/month, 30 % above → {im} €/month.\n\nNote: representative municipal rates. Source: Porezna uprava.",
            "Einkommensteuer 2025.\n\nBemessung = brutto − Rente − Freibetrag 600 € = {b} €\n20 % bis 5.000 €/Monat, 30 % darüber → {im} €/Monat.\n\nHinweis: repräsentative Gemeindesätze. Quelle: Porezna uprava.",
            "Inkomstenbelasting 2025.\n\nGrondslag = bruto − pensioen − aftrek 600 € = {b} €\n20 % tot 5.000 €/maand, 30 % daarboven → {im} €/maand.\n\nNoot: representatieve gemeentetarieven. Bron: Porezna uprava.",
            "Imposta sul reddito 2025.\n\nBase = lordo − pensione − detrazione 600 € = {b} €\n20 % fino a 5.000 €/mese, 30 % oltre → {im} €/mese.\n\nNota: aliquote comunali rappresentative. Fonte: Porezna uprava.",
            "Impuesto sobre la renta 2025.\n\nBase = bruto − pensión − reducción 600 € = {b} €\n20 % hasta 5.000 €/mes, 30 % por encima → {im} €/mes.\n\nNota: tipos municipales representativos. Fuente: Porezna uprava.",
        ],
        // ── Hongrie ──
        "HU_TB" => [
            "TB — {ts} % employee (pension 10 % + health 7 % + unemployment 1.5 %). Employee: {ms} HUF.",
            "TB — {ts} % Arbeitnehmer (Rente 10 % + Kranken 7 % + Arbeitslosen 1,5 %). Arbeitnehmer: {ms} HUF.",
            "TB — {ts} % werknemer (pensioen 10 % + ziekte 7 % + werkloosheid 1,5 %). Werknemer: {ms} HUF.",
            "TB — {ts} % dipendente (pensione 10 % + malattia 7 % + disoccupazione 1,5 %). Dipendente: {ms} HUF.",
            "TB — {ts} % trabajador (pensión 10 % + enfermedad 7 % + desempleo 1,5 %). Trabajador: {ms} HUF.",
        ],
        "HU_SZOCHO" => [
            "Szocho — {tp} % borne by the employer.",
            "Szocho — {tp} % zu Lasten des Arbeitgebers.",
            "Szocho — {tp} % ten laste van de werkgever.",
            "Szocho — {tp} % a carico del datore di lavoro.",
            "Szocho — {tp} % a cargo del empleador.",
        ],
        "HU_SZJA" => [
            "Income tax 2025: flat 15 % → {im} HUF/month.\n\nNote: family allowances and young/mothers exemptions not modelled (conservative net).\nSource: NAV.",
            "Einkommensteuer 2025: einheitlich 15 % → {im} HUF/Monat.\n\nHinweis: Familienfreibeträge und Befreiungen für Junge/Mütter nicht modelliert (vorsichtiger Nettowert).\nQuelle: NAV.",
            "Inkomstenbelasting 2025: vlak 15 % → {im} HUF/maand.\n\nNoot: gezinsaftrekken en vrijstellingen jongeren/moeders niet gemodelleerd (voorzichtig netto).\nBron: NAV.",
            "Imposta sul reddito 2025: aliquota unica 15 % → {im} HUF/mese.\n\nNota: detrazioni familiari ed esenzioni giovani/madri non modellate (netto prudente).\nFonte: NAV.",
            "Impuesto sobre la renta 2025: tipo único 15 % → {im} HUF/mes.\n\nNota: deducciones familiares y exenciones jóvenes/madres no modeladas (neto prudente).\nFuente: NAV.",
        ],
        // ── Irlande ──
        "IE_PRSI" => [
            "PRSI Class A — employee {ts} % / employer {tp} %. Employee: {ms} €.",
            "PRSI Class A — Arbeitnehmer {ts} % / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.",
            "PRSI Class A — werknemer {ts} % / werkgever {tp} %. Werknemer: {ms} €.",
            "PRSI Class A — dipendente {ts} % / datore di lavoro {tp} %. Dipendente: {ms} €.",
            "PRSI Class A — trabajador {ts} % / empleador {tp} %. Trabajador: {ms} €.",
        ],
        "IE_USC" => [
            "USC 2025: 0.5 % / 2 % / 3 % / 8 % (thresholds 12,012 / 27,382 / 70,044 €).\nAnnual income {g} € → {im} €/month.",
            "USC 2025: 0,5 % / 2 % / 3 % / 8 % (Grenzen 12.012 / 27.382 / 70.044 €).\nJahreseinkommen {g} € → {im} €/Monat.",
            "USC 2025: 0,5 % / 2 % / 3 % / 8 % (drempels 12.012 / 27.382 / 70.044 €).\nJaarinkomen {g} € → {im} €/maand.",
            "USC 2025: 0,5 % / 2 % / 3 % / 8 % (soglie 12.012 / 27.382 / 70.044 €).\nReddito annuo {g} € → {im} €/mese.",
            "USC 2025: 0,5 % / 2 % / 3 % / 8 % (umbrales 12.012 / 27.382 / 70.044 €).\nRenta anual {g} € → {im} €/mes.",
        ],
        "IE_PAYE" => [
            "Income tax 2025 (annualised).\n\n20 % up to 44,000 €/yr, 40 % above − credits 4,000 € (personal + PAYE)\nAnnual income {g} € → {im} €/month.\n\nNote: single-employee credits. Source: Revenue.",
            "Einkommensteuer 2025 (auf Jahresbasis).\n\n20 % bis 44.000 €/Jahr, 40 % darüber − Absetzbeträge 4.000 € (persönlich + PAYE)\nJahreseinkommen {g} € → {im} €/Monat.\n\nHinweis: Absetzbeträge eines alleinstehenden Arbeitnehmers. Quelle: Revenue.",
            "Inkomstenbelasting 2025 (op jaarbasis).\n\n20 % tot 44.000 €/jr, 40 % daarboven − kortingen 4.000 € (persoonlijk + PAYE)\nJaarinkomen {g} € → {im} €/maand.\n\nNoot: kortingen van een alleenstaande werknemer. Bron: Revenue.",
            "Imposta sul reddito 2025 (annualizzata).\n\n20 % fino a 44.000 €/anno, 40 % oltre − crediti 4.000 € (personale + PAYE)\nReddito annuo {g} € → {im} €/mese.\n\nNota: crediti di un dipendente celibe. Fonte: Revenue.",
            "Impuesto sobre la renta 2025 (anualizado).\n\n20 % hasta 44.000 €/año, 40 % por encima − créditos 4.000 € (personal + PAYE)\nRenta anual {g} € → {im} €/mes.\n\nNota: créditos de un trabajador soltero. Fuente: Revenue.",
        ],
        // ── Lettonie ──
        "LV_VSAOI" => [
            "VSAOI — employee {ts} % / employer {tp} % (pension, health, unemployment, maternity, accidents). Employee: {ms} €.",
            "VSAOI — Arbeitnehmer {ts} % / Arbeitgeber {tp} % (Rente, Kranken, Arbeitslosen, Mutterschaft, Unfälle). Arbeitnehmer: {ms} €.",
            "VSAOI — werknemer {ts} % / werkgever {tp} % (pensioen, ziekte, werkloosheid, moederschap, ongevallen). Werknemer: {ms} €.",
            "VSAOI — dipendente {ts} % / datore di lavoro {tp} % (pensione, malattia, disoccupazione, maternità, infortuni). Dipendente: {ms} €.",
            "VSAOI — trabajador {ts} % / empleador {tp} % (pensión, enfermedad, desempleo, maternidad, accidentes). Trabajador: {ms} €.",
        ],
        "LV_IIN" => [
            "Income tax 2025.\n\nBase = gross − VSAOI {vs} € − tax-free minimum 510 € = {b} €\nRate 25.5 % (up to 8,775 €/month) then 33 % above → {iin} €/month.\n\nSource: Valsts ieņēmumu dienests.",
            "Einkommensteuer 2025.\n\nBemessung = brutto − VSAOI {vs} € − Steuerfreibetrag 510 € = {b} €\nSatz 25,5 % (bis 8.775 €/Monat) dann 33 % darüber → {iin} €/Monat.\n\nQuelle: Valsts ieņēmumu dienests.",
            "Inkomstenbelasting 2025.\n\nGrondslag = bruto − VSAOI {vs} € − belastingvrij minimum 510 € = {b} €\nTarief 25,5 % (tot 8.775 €/maand) dan 33 % daarboven → {iin} €/maand.\n\nBron: Valsts ieņēmumu dienests.",
            "Imposta sul reddito 2025.\n\nBase = lordo − VSAOI {vs} € − minimo esente 510 € = {b} €\nAliquota 25,5 % (fino a 8.775 €/mese) poi 33 % oltre → {iin} €/mese.\n\nFonte: Valsts ieņēmumu dienests.",
            "Impuesto sobre la renta 2025.\n\nBase = bruto − VSAOI {vs} € − mínimo exento 510 € = {b} €\nTipo 25,5 % (hasta 8.775 €/mes) luego 33 % por encima → {iin} €/mes.\n\nFuente: Valsts ieņēmumu dienests.",
        ],
        // ── Lituanie ──
        "LT_SODRA" => [
            "Sodra — employee {ts} % (pension, health/PSD, maternity) / employer {tp} %. Employee: {ms} €.",
            "Sodra — Arbeitnehmer {ts} % (Rente, Kranken/PSD, Mutterschaft) / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.",
            "Sodra — werknemer {ts} % (pensioen, ziekte/PSD, moederschap) / werkgever {tp} %. Werknemer: {ms} €.",
            "Sodra — dipendente {ts} % (pensione, malattia/PSD, maternità) / datore di lavoro {tp} %. Dipendente: {ms} €.",
            "Sodra — trabajador {ts} % (pensión, enfermedad/PSD, maternidad) / empleador {tp} %. Trabajador: {ms} €.",
        ],
        "LT_GPM" => [
            "Income tax 2025 (GPM).\n\nTapering tax-free amount (NPD): {npd} €\nBase = gross − NPD = {b} €\nRate 20 % (up to ≈ 10,540 €/month) then 32 % above → {gpm} €/month.\n\nSource: VMI.",
            "Einkommensteuer 2025 (GPM).\n\nGleitender Freibetrag (NPD): {npd} €\nBemessung = brutto − NPD = {b} €\nSatz 20 % (bis ≈ 10.540 €/Monat) dann 32 % darüber → {gpm} €/Monat.\n\nQuelle: VMI.",
            "Inkomstenbelasting 2025 (GPM).\n\nAflopend belastingvrij bedrag (NPD): {npd} €\nGrondslag = bruto − NPD = {b} €\nTarief 20 % (tot ≈ 10.540 €/maand) dan 32 % daarboven → {gpm} €/maand.\n\nBron: VMI.",
            "Imposta sul reddito 2025 (GPM).\n\nImporto esente decrescente (NPD): {npd} €\nBase = lordo − NPD = {b} €\nAliquota 20 % (fino a ≈ 10.540 €/mese) poi 32 % oltre → {gpm} €/mese.\n\nFonte: VMI.",
            "Impuesto sobre la renta 2025 (GPM).\n\nImporte exento decreciente (NPD): {npd} €\nBase = bruto − NPD = {b} €\nTipo 20 % (hasta ≈ 10.540 €/mes) luego 32 % por encima → {gpm} €/mes.\n\nFuente: VMI.",
        ],
        // ── Malte ──
        "MT_SSC" => [
            "SSC — employee {ts} % / employer {tp} %. Base capped at 2,306.58 €/month (≈ 27,679 €/yr). Employee: {ms} €.",
            "SSC — Arbeitnehmer {ts} % / Arbeitgeber {tp} %. Bemessungsgrundlage gedeckelt auf 2.306,58 €/Monat (≈ 27.679 €/Jahr). Arbeitnehmer: {ms} €.",
            "SSC — werknemer {ts} % / werkgever {tp} %. Grondslag begrensd op 2.306,58 €/maand (≈ 27.679 €/jr). Werknemer: {ms} €.",
            "SSC — dipendente {ts} % / datore di lavoro {tp} %. Base limitata a 2.306,58 €/mese (≈ 27.679 €/anno). Dipendente: {ms} €.",
            "SSC — trabajador {ts} % / empleador {tp} %. Base limitada a 2.306,58 €/mes (≈ 27.679 €/año). Trabajador: {ms} €.",
        ],
        "MT_TAX" => [
            "Income tax 2025 (single scale, annualised).\n\nBase = gross × 12 = {b} €\n0 % up to 12,000 €, then 15 % / 25 % / 35 % (abatements 1,800 / 3,400 / 9,400 €)\n→ {im} €/month.\n\nSource: Commissioner for Revenue.",
            "Einkommensteuer 2025 (Single-Tarif, auf Jahresbasis).\n\nBemessung = brutto × 12 = {b} €\n0 % bis 12.000 €, dann 15 % / 25 % / 35 % (Abzüge 1.800 / 3.400 / 9.400 €)\n→ {im} €/Monat.\n\nQuelle: Commissioner for Revenue.",
            "Inkomstenbelasting 2025 (single-schaal, op jaarbasis).\n\nGrondslag = bruto × 12 = {b} €\n0 % tot 12.000 €, dan 15 % / 25 % / 35 % (aftrekken 1.800 / 3.400 / 9.400 €)\n→ {im} €/maand.\n\nBron: Commissioner for Revenue.",
            "Imposta sul reddito 2025 (scala single, annualizzata).\n\nBase = lordo × 12 = {b} €\n0 % fino a 12.000 €, poi 15 % / 25 % / 35 % (abbattimenti 1.800 / 3.400 / 9.400 €)\n→ {im} €/mese.\n\nFonte: Commissioner for Revenue.",
            "Impuesto sobre la renta 2025 (escala single, anualizado).\n\nBase = bruto × 12 = {b} €\n0 % hasta 12.000 €, luego 15 % / 25 % / 35 % (reducciones 1.800 / 3.400 / 9.400 €)\n→ {im} €/mes.\n\nFuente: Commissioner for Revenue.",
        ],
        // ── Slovénie ──
        "SI_PRISPEVKI" => [
            "Prispevki — employee {ts} % (pension/disability 15.5 %, health 6.36 %, unemployment 0.14 %, parental 0.10 %) / employer {tp} %. Employee: {ms} €.",
            "Prispevki — Arbeitnehmer {ts} % (Rente/Invalidität 15,5 %, Kranken 6,36 %, Arbeitslosen 0,14 %, Eltern 0,10 %) / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.",
            "Prispevki — werknemer {ts} % (pensioen/invaliditeit 15,5 %, ziekte 6,36 %, werkloosheid 0,14 %, ouderschap 0,10 %) / werkgever {tp} %. Werknemer: {ms} €.",
            "Prispevki — dipendente {ts} % (pensione/invalidità 15,5 %, malattia 6,36 %, disoccupazione 0,14 %, parentale 0,10 %) / datore di lavoro {tp} %. Dipendente: {ms} €.",
            "Prispevki — trabajador {ts} % (pensión/invalidez 15,5 %, enfermedad 6,36 %, desempleo 0,14 %, parental 0,10 %) / empleador {tp} %. Trabajador: {ms} €.",
        ],
        "SI_DOHODNINA" => [
            "Income tax 2025 (annualised).\n\nBase = (gross − contributions) × 12 − allowance 5,000 € = {b} €\nScale 16 / 26 / 33 / 39 / 50 % (thresholds 9,210 / 27,089 / 54,179 / 78,016 €)\n→ {im} €/month.\n\nNote: increased low-income allowance not modelled (conservative net).\nSource: FURS.",
            "Einkommensteuer 2025 (auf Jahresbasis).\n\nBemessung = (brutto − Beiträge) × 12 − Freibetrag 5.000 € = {b} €\nTarif 16 / 26 / 33 / 39 / 50 % (Grenzen 9.210 / 27.089 / 54.179 / 78.016 €)\n→ {im} €/Monat.\n\nHinweis: erhöhter Freibetrag für niedrige Einkommen nicht modelliert (vorsichtiger Nettowert).\nQuelle: FURS.",
            "Inkomstenbelasting 2025 (op jaarbasis).\n\nGrondslag = (bruto − bijdragen) × 12 − aftrek 5.000 € = {b} €\nSchaal 16 / 26 / 33 / 39 / 50 % (drempels 9.210 / 27.089 / 54.179 / 78.016 €)\n→ {im} €/maand.\n\nNoot: verhoogde aftrek voor lage inkomens niet gemodelleerd (voorzichtig netto).\nBron: FURS.",
            "Imposta sul reddito 2025 (annualizzata).\n\nBase = (lordo − contributi) × 12 − detrazione 5.000 € = {b} €\nScala 16 / 26 / 33 / 39 / 50 % (soglie 9.210 / 27.089 / 54.179 / 78.016 €)\n→ {im} €/mese.\n\nNota: detrazione maggiorata per redditi bassi non modellata (netto prudente).\nFonte: FURS.",
            "Impuesto sobre la renta 2025 (anualizado).\n\nBase = (bruto − cotizaciones) × 12 − reducción 5.000 € = {b} €\nEscala 16 / 26 / 33 / 39 / 50 % (umbrales 9.210 / 27.089 / 54.179 / 78.016 €)\n→ {im} €/mes.\n\nNota: reducción incrementada para rentas bajas no modelada (neto prudente).\nFuente: FURS.",
        ],
        // ── Suède ──
        "SE_ARBETSGIVARAVGIFT" => [
            "Arbetsgivaravgifter — {tp} % borne by the employer (pension, health, parental, accident, labour market, general payroll tax).\n\nEmployee side: the allmän pensionsavgift (7 %) is fully offset by a tax reduction (net nil effect) → not shown.",
            "Arbetsgivaravgifter — {tp} % zu Lasten des Arbeitgebers (Rente, Kranken, Eltern, Unfall, Arbeitsmarkt, allgemeine Lohnsteuer).\n\nArbeitnehmerseite: die allmän pensionsavgift (7 %) wird durch eine Steuerermäßigung vollständig ausgeglichen (Nettoeffekt null) → nicht angezeigt.",
            "Arbetsgivaravgifter — {tp} % ten laste van de werkgever (pensioen, ziekte, ouderschap, ongeval, arbeidsmarkt, algemene loonheffing).\n\nWerknemerszijde: de allmän pensionsavgift (7 %) wordt volledig gecompenseerd door een belastingvermindering (netto nul) → niet getoond.",
            "Arbetsgivaravgifter — {tp} % a carico del datore di lavoro (pensione, malattia, parentale, infortuni, mercato del lavoro, imposta generale sui salari).\n\nLato dipendente: l'allmän pensionsavgift (7 %) è interamente compensata da una riduzione d'imposta (effetto netto nullo) → non mostrata.",
            "Arbetsgivaravgifter — {tp} % a cargo del empleador (pensión, enfermedad, parental, accidente, mercado laboral, impuesto general sobre salarios).\n\nLado del trabajador: la allmän pensionsavgift (7 %) se compensa íntegramente con una reducción fiscal (efecto neto nulo) → no se muestra.",
        ],
        "SE_SKATT" => [
            "Income tax 2025 (annualised).\n\nAnnual income: {g} SEK\nAverage municipal tax 32.41 % → {co} SEK\nState tax 20 % above 625,800 SEK/yr → {et} SEK\n= {im} SEK/month.\n\nNote: grundavdrag and jobbskatteavdrag not modelled (conservative net).\nSource: Skatteverket.",
            "Einkommensteuer 2025 (auf Jahresbasis).\n\nJahreseinkommen: {g} SEK\nDurchschn. Gemeindesteuer 32,41 % → {co} SEK\nStaatssteuer 20 % über 625.800 SEK/Jahr → {et} SEK\n= {im} SEK/Monat.\n\nHinweis: grundavdrag und jobbskatteavdrag nicht modelliert (vorsichtiger Nettowert).\nQuelle: Skatteverket.",
            "Inkomstenbelasting 2025 (op jaarbasis).\n\nJaarinkomen: {g} SEK\nGemiddelde gemeentebelasting 32,41 % → {co} SEK\nRijksbelasting 20 % boven 625.800 SEK/jr → {et} SEK\n= {im} SEK/maand.\n\nNoot: grundavdrag en jobbskatteavdrag niet gemodelleerd (voorzichtig netto).\nBron: Skatteverket.",
            "Imposta sul reddito 2025 (annualizzata).\n\nReddito annuo: {g} SEK\nImposta comunale media 32,41 % → {co} SEK\nImposta statale 20 % oltre 625.800 SEK/anno → {et} SEK\n= {im} SEK/mese.\n\nNota: grundavdrag e jobbskatteavdrag non modellati (netto prudente).\nFonte: Skatteverket.",
            "Impuesto sobre la renta 2025 (anualizado).\n\nRenta anual: {g} SEK\nImpuesto municipal medio 32,41 % → {co} SEK\nImpuesto estatal 20 % por encima de 625.800 SEK/año → {et} SEK\n= {im} SEK/mes.\n\nNota: grundavdrag y jobbskatteavdrag no modelados (neto prudente).\nFuente: Skatteverket.",
        ],
        // ── Roumanie ──
        "RO_IMPOZIT" => [
            "Income tax {annee}: flat 10 % (since 2018).\n\nBase = gross − CAS 25 % − CASS 10 % = {b} RON → {im} RON/month.\n\nNote: personal deduction (low wages) not modelled (conservative net).\nSource: ANAF.",
            "Einkommensteuer {annee}: pauschal 10 % (seit 2018).\n\nBemessung = brutto − CAS 25 % − CASS 10 % = {b} RON → {im} RON/Monat.\n\nHinweis: persönlicher Abzug (niedrige Löhne) nicht modelliert (vorsichtiger Nettowert).\nQuelle: ANAF.",
            "Inkomstenbelasting {annee}: vlak 10 % (sinds 2018).\n\nGrondslag = bruto − CAS 25 % − CASS 10 % = {b} RON → {im} RON/maand.\n\nNoot: persoonlijke aftrek (lage lonen) niet gemodelleerd (voorzichtig netto).\nBron: ANAF.",
            "Imposta sul reddito {annee}: proporzionale 10 % (dal 2018).\n\nBase = lordo − CAS 25 % − CASS 10 % = {b} RON → {im} RON/mese.\n\nNota: detrazione personale (bassi salari) non modellata (netto prudente).\nFonte: ANAF.",
            "Impuesto sobre la renta {annee}: plano 10 % (desde 2018).\n\nBase = bruto − CAS 25 % − CASS 10 % = {b} RON → {im} RON/mes.\n\nNota: deducción personal (salarios bajos) no modelada (neto prudente).\nFuente: ANAF.",
        ],
        // ── Pologne ──
        "PL_ZDROWOTNE" => [
            "Składka zdrowotna — 9 % of the base (gross − employee social ZUS).\nBase: {b} PLN → {s} PLN/month. Non-deductible from PIT since 2022.\n\nLegal basis: Ustawa o świadczeniach opieki zdrowotnej.",
            "Składka zdrowotna — 9 % der Grundlage (brutto − AN-Sozial-ZUS).\nGrundlage: {b} PLN → {s} PLN/Monat. Seit 2022 nicht von der PIT abzugsfähig.\n\nRechtsgrundlage: Ustawa o świadczeniach opieki zdrowotnej.",
            "Składka zdrowotna — 9 % van de grondslag (bruto − sociale ZUS werknemer).\nGrondslag: {b} PLN → {s} PLN/maand. Sinds 2022 niet aftrekbaar van PIT.\n\nWettelijke basis: Ustawa o świadczeniach opieki zdrowotnej.",
            "Składka zdrowotna — 9 % della base (lordo − ZUS sociale dipendente).\nBase: {b} PLN → {s} PLN/mese. Non deducibile dal PIT dal 2022.\n\nBase giuridica: Ustawa o świadczeniach opieki zdrowotnej.",
            "Składka zdrowotna — 9 % de la base (bruto − ZUS social del trabajador).\nBase: {b} PLN → {s} PLN/mes. No deducible del PIT desde 2022.\n\nBase legal: Ustawa o świadczeniach opieki zdrowotnej.",
        ],
        "PL_PIT" => [
            "Income tax (PIT) 2025 — annualised.\n\nAnnual income: {ba} PLN − social ZUS {za} PLN − KUP {kup} PLN\n= taxable base {tx} PLN\nScale: 12 % up to 120,000 PLN, 32 % above; − tax-reducing amount 3,600 PLN.\n= {pa} PLN/yr / 12 = {pm} PLN/month.\n\nLegal basis: Ustawa o PIT.",
            "Einkommensteuer (PIT) 2025 — auf Jahresbasis.\n\nJahreseinkommen: {ba} PLN − Sozial-ZUS {za} PLN − KUP {kup} PLN\n= Bemessungsgrundlage {tx} PLN\nTarif: 12 % bis 120.000 PLN, 32 % darüber; − Steuerminderungsbetrag 3.600 PLN.\n= {pa} PLN/Jahr / 12 = {pm} PLN/Monat.\n\nRechtsgrundlage: Ustawa o PIT.",
            "Inkomstenbelasting (PIT) 2025 — op jaarbasis.\n\nJaarinkomen: {ba} PLN − sociale ZUS {za} PLN − KUP {kup} PLN\n= belastbare grondslag {tx} PLN\nSchaal: 12 % tot 120.000 PLN, 32 % daarboven; − belastingverlagend bedrag 3.600 PLN.\n= {pa} PLN/jr / 12 = {pm} PLN/maand.\n\nWettelijke basis: Ustawa o PIT.",
            "Imposta sul reddito (PIT) 2025 — annualizzata.\n\nReddito annuo: {ba} PLN − ZUS sociale {za} PLN − KUP {kup} PLN\n= base imponibile {tx} PLN\nScala: 12 % fino a 120.000 PLN, 32 % oltre; − importo riduttore 3.600 PLN.\n= {pa} PLN/anno / 12 = {pm} PLN/mese.\n\nBase giuridica: Ustawa o PIT.",
            "Impuesto sobre la renta (PIT) 2025 — anualizado.\n\nRenta anual: {ba} PLN − ZUS social {za} PLN − KUP {kup} PLN\n= base imponible {tx} PLN\nEscala: 12 % hasta 120.000 PLN, 32 % por encima; − importe reductor 3.600 PLN.\n= {pa} PLN/año / 12 = {pm} PLN/mes.\n\nBase legal: Ustawa o PIT.",
        ],
        // ── Slovaquie ──
        "SK_ZDRAVOTNE" => [
            "Health insurance — employee {ts} % / employer {tp} %.",
            "Krankenversicherung — Arbeitnehmer {ts} % / Arbeitgeber {tp} %.",
            "Ziektekostenverzekering — werknemer {ts} % / werkgever {tp} %.",
            "Assicurazione malattia — dipendente {ts} % / datore di lavoro {tp} %.",
            "Seguro de enfermedad — trabajador {ts} % / empleador {tp} %.",
        ],
        "SK_SOCIALNE" => [
            "Social security — employee {ts} % / employer {tp} %. Base capped at 15,730 €/month.",
            "Sozialversicherung — Arbeitnehmer {ts} % / Arbeitgeber {tp} %. Bemessungsgrundlage gedeckelt auf 15.730 €/Monat.",
            "Sociale zekerheid — werknemer {ts} % / werkgever {tp} %. Grondslag begrensd op 15.730 €/maand.",
            "Sicurezza sociale — dipendente {ts} % / datore di lavoro {tp} %. Base limitata a 15.730 €/mese.",
            "Seguridad social — trabajador {ts} % / empleador {tp} %. Base limitada a 15.730 €/mes.",
        ],
        "SK_DAN" => [
            "Income tax 2025.\n\nBase = gross − employee contributions − tax-free part 479.48 € = {b} €\n19 % up to 4,036.79 €/month, 25 % above → {im} €/month.\n\nNote: tapering of the tax-free part not modelled (conservative net).\nSource: Finančná správa.",
            "Einkommensteuer 2025.\n\nBemessung = brutto − AN-Beiträge − steuerfreier Teil 479,48 € = {b} €\n19 % bis 4.036,79 €/Monat, 25 % darüber → {im} €/Monat.\n\nHinweis: Abschmelzung des steuerfreien Teils nicht modelliert (vorsichtiger Nettowert).\nQuelle: Finančná správa.",
            "Inkomstenbelasting 2025.\n\nGrondslag = bruto − werknemersbijdragen − belastingvrij deel 479,48 € = {b} €\n19 % tot 4.036,79 €/maand, 25 % daarboven → {im} €/maand.\n\nNoot: afbouw van het belastingvrije deel niet gemodelleerd (voorzichtig netto).\nBron: Finančná správa.",
            "Imposta sul reddito 2025.\n\nBase = lordo − contributi dipendente − parte esente 479,48 € = {b} €\n19 % fino a 4.036,79 €/mese, 25 % oltre → {im} €/mese.\n\nNota: decrescenza della parte esente non modellata (netto prudente).\nFonte: Finančná správa.",
            "Impuesto sobre la renta 2025.\n\nBase = bruto − cotizaciones del trabajador − parte exenta 479,48 € = {b} €\n19 % hasta 4.036,79 €/mes, 25 % por encima → {im} €/mes.\n\nNota: decrecimiento de la parte exenta no modelado (neto prudente).\nFuente: Finančná správa.",
        ],
        // ── Nouvelle-Zélande ──
        "NZ_PAYE" => [
            "Income tax (PAYE) — fiscal year {fy0}-{fy1}, no tax-free band.\n\nEstimated annual income: {rev} $ → {imp} $/yr / 12 = {mens} $/month.\n\nLegal basis: Income Tax Act 2007.",
            "Einkommensteuer (PAYE) — Steuerjahr {fy0}-{fy1}, ohne Freibetrag.\n\nGeschätztes Jahreseinkommen: {rev} $ → {imp} $/Jahr / 12 = {mens} $/Monat.\n\nRechtsgrundlage: Income Tax Act 2007.",
            "Inkomstenbelasting (PAYE) — belastingjaar {fy0}-{fy1}, zonder belastingvrije schijf.\n\nGeschat jaarinkomen: {rev} $ → {imp} $/jr / 12 = {mens} $/maand.\n\nWettelijke basis: Income Tax Act 2007.",
            "Imposta sul reddito (PAYE) — anno fiscale {fy0}-{fy1}, senza fascia esente.\n\nReddito annuo stimato: {rev} $ → {imp} $/anno / 12 = {mens} $/mese.\n\nBase giuridica: Income Tax Act 2007.",
            "Impuesto sobre la renta (PAYE) — año fiscal {fy0}-{fy1}, sin tramo exento.\n\nRenta anual estimada: {rev} $ → {imp} $/año / 12 = {mens} $/mes.\n\nBase legal: Income Tax Act 2007.",
        ],
        "NZ_ACC" => [
            "ACC earner's levy — accident cover, {t} % of gross salary (year {fy0}-{fy1}).\nBase capped at {cap} $/yr. Amount: {m} $/month.\n\nLegal basis: Accident Compensation Act 2001.",
            "ACC earner's levy — Unfallschutz, {t} % des Bruttolohns (Jahr {fy0}-{fy1}).\nBemessung gedeckelt auf {cap} $/Jahr. Betrag: {m} $/Monat.\n\nRechtsgrundlage: Accident Compensation Act 2001.",
            "ACC earner's levy — ongevallendekking, {t} % van het brutoloon (jaar {fy0}-{fy1}).\nGrondslag begrensd op {cap} $/jr. Bedrag: {m} $/maand.\n\nWettelijke basis: Accident Compensation Act 2001.",
            "ACC earner's levy — copertura infortuni, {t} % della retribuzione lorda (anno {fy0}-{fy1}).\nBase limitata a {cap} $/anno. Importo: {m} $/mese.\n\nBase giuridica: Accident Compensation Act 2001.",
            "ACC earner's levy — cobertura de accidentes, {t} % del salario bruto (año {fy0}-{fy1}).\nBase limitada a {cap} $/año. Importe: {m} $/mes.\n\nBase legal: Accident Compensation Act 2001.",
        ],
        "NZ_KIWISAVER_EMP" => [
            "KiwiSaver — retirement savings, default employer contribution {t} %, paid on top.\nOptional depending on employee enrolment.\nEmployer: {mp} $/month.\n\nLegal basis: KiwiSaver Act 2006.",
            "KiwiSaver — Altersvorsorge, Arbeitgeberbeitrag standardmäßig {t} %, zusätzlich gezahlt.\nOptional je nach Beitritt des Arbeitnehmers.\nArbeitgeber: {mp} $/Monat.\n\nRechtsgrundlage: KiwiSaver Act 2006.",
            "KiwiSaver — pensioensparen, standaard werkgeversbijdrage {t} %, bovenop betaald.\nOptioneel afhankelijk van aanmelding werknemer.\nWerkgever: {mp} $/maand.\n\nWettelijke basis: KiwiSaver Act 2006.",
            "KiwiSaver — risparmio pensionistico, contributo datore di lavoro predefinito {t} %, versato in aggiunta.\nOpzionale secondo l'adesione del dipendente.\nDatore di lavoro: {mp} $/mese.\n\nBase giuridica: KiwiSaver Act 2006.",
            "KiwiSaver — ahorro para la jubilación, cotización del empleador por defecto {t} %, pagada adicionalmente.\nOpcional según la adhesión del trabajador.\nEmpleador: {mp} $/mes.\n\nBase legal: KiwiSaver Act 2006.",
        ],
        // ── Pays-Bas ──
        "NL_NON_COUVERT" => [
            "Dutch data is only available for 2026 (pilot).\nYear {annee} will be added after official sourcing (Belastingdienst).\nNo figure is invented in the absence of a source.",
            "Niederländische Daten sind nur für 2026 verfügbar (Pilot).\nDas Jahr {annee} wird nach offizieller Quellenprüfung (Belastingdienst) ergänzt.\nOhne Quelle wird keine Zahl erfunden.",
            "Nederlandse gegevens zijn alleen beschikbaar voor 2026 (pilot).\nHet jaar {annee} wordt toegevoegd na officiële bronvermelding (Belastingdienst).\nZonder bron wordt geen cijfer verzonnen.",
            "I dati olandesi sono disponibili solo per il 2026 (pilota).\nL'anno {annee} sarà aggiunto dopo reperimento ufficiale (Belastingdienst).\nNessuna cifra è inventata in assenza di fonte.",
            "Los datos neerlandeses solo están disponibles para 2026 (piloto).\nEl año {annee} se añadirá tras el sourcing oficial (Belastingdienst).\nNo se inventa ninguna cifra en ausencia de fuente.",
        ],
        // ── Chine (sous-phrases {expl}) ──
        "CN_YANGLAO" => [
            "Mandatory pension contribution. Empl 8 % + empr 16 % = 24 % total. 社会保险法 art. 12.",
            "Pflicht-Rentenbeitrag. AN 8 % + AG 16 % = 24 % gesamt. 社会保险法 Art. 12.",
            "Verplichte pensioenbijdrage. Wn 8 % + wg 16 % = 24 % totaal. 社会保险法 art. 12.",
            "Contributo pensione obbligatorio. Dip 8 % + dat 16 % = 24 % totale. 社会保险法 art. 12.",
            "Cotización de pensión obligatoria. Trab 8 % + empr 16 % = 24 % total. 社会保险法 art. 12.",
        ],
        "CN_YILIAO" => [
            "Health insurance. Empl 2 % + empr 8 % = 10 % total. 社会保险法 art. 23.",
            "Krankenversicherung. AN 2 % + AG 8 % = 10 % gesamt. 社会保险法 Art. 23.",
            "Ziektekostenverzekering. Wn 2 % + wg 8 % = 10 % totaal. 社会保险法 art. 23.",
            "Assicurazione malattia. Dip 2 % + dat 8 % = 10 % totale. 社会保险法 art. 23.",
            "Seguro de enfermedad. Trab 2 % + empr 8 % = 10 % total. 社会保险法 art. 23.",
        ],
        "CN_SHIYE" => [
            "Unemployment insurance. Empl 0.5 % + empr 0.5 % = 1 % total. 社会保险法 art. 44.",
            "Arbeitslosenversicherung. AN 0,5 % + AG 0,5 % = 1 % gesamt. 社会保险法 Art. 44.",
            "Werkloosheidsverzekering. Wn 0,5 % + wg 0,5 % = 1 % totaal. 社会保险法 art. 44.",
            "Assicurazione disoccupazione. Dip 0,5 % + dat 0,5 % = 1 % totale. 社会保险法 art. 44.",
            "Seguro de desempleo. Trab 0,5 % + empr 0,5 % = 1 % total. 社会保险法 art. 44.",
        ],
        "CN_GONGSHANG" => [
            "100 % employer. Beijing general rate 0.4 %. 社会保险法 art. 33.",
            "100 % Arbeitgeber. Pekinger Allgemeinsatz 0,4 %. 社会保险法 Art. 33.",
            "100 % werkgever. Algemeen tarief Peking 0,4 %. 社会保险法 art. 33.",
            "100 % datore di lavoro. Aliquota generale Pechino 0,4 %. 社会保险法 art. 33.",
            "100 % empleador. Tipo general Pekín 0,4 %. 社会保险法 art. 33.",
        ],
        "CN_SHENGYU" => [
            "100 % employer. Beijing rate 0.8 %. 社会保险法 art. 53.",
            "100 % Arbeitgeber. Pekinger Satz 0,8 %. 社会保险法 Art. 53.",
            "100 % werkgever. Tarief Peking 0,8 %. 社会保险法 art. 53.",
            "100 % datore di lavoro. Aliquota Pechino 0,8 %. 社会保险法 art. 53.",
            "100 % empleador. Tipo Pekín 0,8 %. 社会保险法 art. 53.",
        ],
        "CN_GONGJIJIN" => [
            "Housing fund: empl 12 % + empr 12 % = 24 % total. Beijing 2024. Individual savings available for purchase/rent. 住房公积金管理条例.",
            "Wohnungsfonds: AN 12 % + AG 12 % = 24 % gesamt. Peking 2024. Individuelles Guthaben für Kauf/Miete verfügbar. 住房公积金管理条例.",
            "Huisvestingsfonds: wn 12 % + wg 12 % = 24 % totaal. Peking 2024. Individueel spaargeld beschikbaar voor koop/huur. 住房公积金管理条例.",
            "Fondo casa: dip 12 % + dat 12 % = 24 % totale. Pechino 2024. Risparmio individuale disponibile per acquisto/affitto. 住房公积金管理条例.",
            "Fondo de vivienda: trab 12 % + empr 12 % = 24 % total. Pekín 2024. Ahorro individual disponible para compra/alquiler. 住房公积金管理条例.",
        ],
        "CN_IIT" => [
            "个人所得税 — income tax (2018 reform).\n\nMonthly gross: ¥{brut}\n− Employee social contributions: ¥{cot}\n− Personal deduction: ¥{dp}/month\n= Monthly taxable base: ¥{bm}\n× 12 = Annual base: ¥{ba}\n\nAnnual IIT (brackets 3/10/20/25/30/35/45 %): ¥{ia}\nMonthly withholding: ¥{ia} / 12 = ¥{mens}\nEffective monthly rate: {teff} %\n\nLegal basis: 个人所得税法 (Law 31/08/2018); 国税发〔2018〕164号.",
            "个人所得税 — Einkommensteuer (Reform 2018).\n\nMonatsbrutto: ¥{brut}\n− AN-Sozialbeiträge: ¥{cot}\n− Persönlicher Abzug: ¥{dp}/Monat\n= Monatliche Bemessungsgrundlage: ¥{bm}\n× 12 = Jahresgrundlage: ¥{ba}\n\nJahres-IIT (Stufen 3/10/20/25/30/35/45 %): ¥{ia}\nMonatlicher Einbehalt: ¥{ia} / 12 = ¥{mens}\nEffektiver Monatssatz: {teff} %\n\nRechtsgrundlage: 个人所得税法 (Gesetz 31.08.2018); 国税发〔2018〕164号.",
            "个人所得税 — inkomstenbelasting (hervorming 2018).\n\nMaandbruto: ¥{brut}\n− Sociale bijdragen werknemer: ¥{cot}\n− Persoonlijke aftrek: ¥{dp}/maand\n= Maandelijkse belastbare grondslag: ¥{bm}\n× 12 = Jaargrondslag: ¥{ba}\n\nJaarlijkse IIT (schijven 3/10/20/25/30/35/45 %): ¥{ia}\nMaandelijkse inhouding: ¥{ia} / 12 = ¥{mens}\nEffectief maandtarief: {teff} %\n\nWettelijke basis: 个人所得税法 (wet 31-08-2018); 国税发〔2018〕164号.",
            "个人所得税 — imposta sul reddito (riforma 2018).\n\nLordo mensile: ¥{brut}\n− Contributi sociali dipendente: ¥{cot}\n− Detrazione personale: ¥{dp}/mese\n= Base mensile imponibile: ¥{bm}\n× 12 = Base annua: ¥{ba}\n\nIIT annua (scaglioni 3/10/20/25/30/35/45 %): ¥{ia}\nRitenuta mensile: ¥{ia} / 12 = ¥{mens}\nAliquota effettiva mensile: {teff} %\n\nBase giuridica: 个人所得税法 (legge 31/08/2018); 国税发〔2018〕164号.",
            "个人所得税 — impuesto sobre la renta (reforma 2018).\n\nBruto mensual: ¥{brut}\n− Cotizaciones sociales del trabajador: ¥{cot}\n− Deducción personal: ¥{dp}/mes\n= Base mensual imponible: ¥{bm}\n× 12 = Base anual: ¥{ba}\n\nIIT anual (tramos 3/10/20/25/30/35/45 %): ¥{ia}\nRetención mensual: ¥{ia} / 12 = ¥{mens}\nTipo efectivo mensual: {teff} %\n\nBase legal: 个人所得税法 (ley 31/08/2018); 国税发〔2018〕164号.",
        ],
        // ── Royaume-Uni ──
        "UK_NI_SAL" => [
            "National Insurance Class 1 — employee share.\n\nBand [PT – UEL] ({ts_pct} %): £{pt} – £{uel}/month\n→ base {tp} × {ts_pct} % = £{m1}\nUpper band (> UEL, 2 %): £{uel}/month\n→ base {th} × 2 % = £{m2}\n\nTotal employee NI: £{tot}\nEffective rate: {teff} %\n\nLegal basis: NIA 2014; Finance Act 2024.",
            "National Insurance Class 1 — Arbeitnehmeranteil.\n\nBand [PT – UEL] ({ts_pct} %): £{pt} – £{uel}/Monat\n→ Grundlage {tp} × {ts_pct} % = £{m1}\nOberes Band (> UEL, 2 %): £{uel}/Monat\n→ Grundlage {th} × 2 % = £{m2}\n\nGesamt AN-NI: £{tot}\nEffektivsatz: {teff} %\n\nRechtsgrundlage: NIA 2014; Finance Act 2024.",
            "National Insurance Class 1 — werknemersdeel.\n\nSchijf [PT – UEL] ({ts_pct} %): £{pt} – £{uel}/maand\n→ grondslag {tp} × {ts_pct} % = £{m1}\nHoge schijf (> UEL, 2 %): £{uel}/maand\n→ grondslag {th} × 2 % = £{m2}\n\nTotaal werknemers-NI: £{tot}\nEffectief tarief: {teff} %\n\nWettelijke basis: NIA 2014; Finance Act 2024.",
            "National Insurance Class 1 — quota dipendente.\n\nFascia [PT – UEL] ({ts_pct} %): £{pt} – £{uel}/mese\n→ base {tp} × {ts_pct} % = £{m1}\nFascia alta (> UEL, 2 %): £{uel}/mese\n→ base {th} × 2 % = £{m2}\n\nTotale NI dipendente: £{tot}\nAliquota effettiva: {teff} %\n\nBase giuridica: NIA 2014; Finance Act 2024.",
            "National Insurance Class 1 — parte del trabajador.\n\nTramo [PT – UEL] ({ts_pct} %): £{pt} – £{uel}/mes\n→ base {tp} × {ts_pct} % = £{m1}\nTramo alto (> UEL, 2 %): £{uel}/mes\n→ base {th} × 2 % = £{m2}\n\nTotal NI trabajador: £{tot}\nTipo efectivo: {teff} %\n\nBase legal: NIA 2014; Finance Act 2024.",
        ],
        "UK_NI_PAT" => [
            "National Insurance Class 1 — employer share.\n\nRate: {tp_pct} % on salary > ST (£{st}/month)\nTaxable base: £{base} × {tp_pct} % = £{tot}\nNo upper cap on the employer side.\nEffective rate on gross salary: {teff} %\n\nLegal basis: NIA 2014; Finance Act 2024.",
            "National Insurance Class 1 — Arbeitgeberanteil.\n\nSatz: {tp_pct} % auf Gehalt > ST (£{st}/Monat)\nBemessungsgrundlage: £{base} × {tp_pct} % = £{tot}\nKeine Obergrenze auf Arbeitgeberseite.\nEffektivsatz auf Bruttogehalt: {teff} %\n\nRechtsgrundlage: NIA 2014; Finance Act 2024.",
            "National Insurance Class 1 — werkgeversdeel.\n\nTarief: {tp_pct} % op loon > ST (£{st}/maand)\nBelastbare grondslag: £{base} × {tp_pct} % = £{tot}\nGeen bovengrens aan werkgeverszijde.\nEffectief tarief op brutoloon: {teff} %\n\nWettelijke basis: NIA 2014; Finance Act 2024.",
            "National Insurance Class 1 — quota datore di lavoro.\n\nAliquota: {tp_pct} % su retribuzione > ST (£{st}/mese)\nBase imponibile: £{base} × {tp_pct} % = £{tot}\nNessun massimale lato datore di lavoro.\nAliquota effettiva sul lordo: {teff} %\n\nBase giuridica: NIA 2014; Finance Act 2024.",
            "National Insurance Class 1 — parte del empleador.\n\nTipo: {tp_pct} % sobre salario > ST (£{st}/mes)\nBase imponible: £{base} × {tp_pct} % = £{tot}\nSin límite superior por el lado del empleador.\nTipo efectivo sobre salario bruto: {teff} %\n\nBase legal: NIA 2014; Finance Act 2024.",
        ],
        "UK_INCOME_TAX" => [
            "Income Tax PAYE (monthly withholding).\n\nEstimated annual income: £{rev} → band: {tl}\nPersonal Allowance: £{pa}/yr (tax-free)\nBasic Rate 20 %: up to £{br}/yr\nHigher Rate 40 %: £{br} – £{hr}/yr\nAdditional Rate 45 %: above £{hr}/yr\n\nEstimated annual tax: £{ia} / 12 = £{im}/month\nEffective monthly rate: {teff} %\n\nLegal basis: Income Tax Act 2007; Finance Act 2024.",
            "Income Tax PAYE (monatlicher Einbehalt).\n\nGeschätztes Jahreseinkommen: £{rev} → Stufe: {tl}\nPersonal Allowance: £{pa}/Jahr (steuerfrei)\nBasic Rate 20 %: bis £{br}/Jahr\nHigher Rate 40 %: £{br} – £{hr}/Jahr\nAdditional Rate 45 %: über £{hr}/Jahr\n\nGeschätzte Jahressteuer: £{ia} / 12 = £{im}/Monat\nEffektiver Monatssatz: {teff} %\n\nRechtsgrundlage: Income Tax Act 2007; Finance Act 2024.",
            "Income Tax PAYE (maandelijkse inhouding).\n\nGeschat jaarinkomen: £{rev} → schijf: {tl}\nPersonal Allowance: £{pa}/jr (belastingvrij)\nBasic Rate 20 %: tot £{br}/jr\nHigher Rate 40 %: £{br} – £{hr}/jr\nAdditional Rate 45 %: boven £{hr}/jr\n\nGeschatte jaarbelasting: £{ia} / 12 = £{im}/maand\nEffectief maandtarief: {teff} %\n\nWettelijke basis: Income Tax Act 2007; Finance Act 2024.",
            "Income Tax PAYE (ritenuta mensile).\n\nReddito annuo stimato: £{rev} → fascia: {tl}\nPersonal Allowance: £{pa}/anno (esente)\nBasic Rate 20 %: fino a £{br}/anno\nHigher Rate 40 %: £{br} – £{hr}/anno\nAdditional Rate 45 %: oltre £{hr}/anno\n\nImposta annua stimata: £{ia} / 12 = £{im}/mese\nAliquota effettiva mensile: {teff} %\n\nBase giuridica: Income Tax Act 2007; Finance Act 2024.",
            "Income Tax PAYE (retención mensual).\n\nRenta anual estimada: £{rev} → tramo: {tl}\nPersonal Allowance: £{pa}/año (exento)\nBasic Rate 20 %: hasta £{br}/año\nHigher Rate 40 %: £{br} – £{hr}/año\nAdditional Rate 45 %: por encima de £{hr}/año\n\nImpuesto anual estimado: £{ia} / 12 = £{im}/mes\nTipo efectivo mensual: {teff} %\n\nBase legal: Income Tax Act 2007; Finance Act 2024.",
        ],
        // ── UK : libellés de tranche {tl} ──
        "UK_TL_PA" => [
            "within the Personal Allowance (0 %)", "innerhalb der Personal Allowance (0 %)",
            "binnen de Personal Allowance (0 %)", "entro la Personal Allowance (0 %)",
            "dentro de la Personal Allowance (0 %)",
        ],
        "UK_TL_HIGHER_PARTIAL" => [
            "partial Higher Rate (40 %)", "teilweiser Higher Rate (40 %)",
            "gedeeltelijk Higher Rate (40 %)", "Higher Rate parziale (40 %)",
            "Higher Rate parcial (40 %)",
        ],
        // ── Australie ──
        "AU_INCOME_TAX" => [
            "Resident income tax — {fy0}-{fy1} tax-year scale.\n\nEstimated annual income: {rev} $ → tax {imp} $/yr / 12 = {mens} $/month.\nTax-free threshold: 18,200 $. LITO/LMITO offsets not modelled (conservative net).\n\nLegal basis: Income Tax Assessment Act 1997.",
            "Einkommensteuer für Ansässige — Tarif des Steuerjahres {fy0}-{fy1}.\n\nGeschätztes Jahreseinkommen: {rev} $ → Steuer {imp} $/Jahr / 12 = {mens} $/Monat.\nSteuerfreibetrag: 18 200 $. LITO/LMITO-Anrechnungen nicht modelliert (vorsichtiger Netto).\n\nRechtsgrundlage: Income Tax Assessment Act 1997.",
            "Inkomstenbelasting voor inwoners — schaal belastingjaar {fy0}-{fy1}.\n\nGeschat jaarinkomen: {rev} $ → belasting {imp} $/jr / 12 = {mens} $/maand.\nBelastingvrije voet: 18.200 $. LITO/LMITO-kortingen niet gemodelleerd (voorzichtig netto).\n\nWettelijke basis: Income Tax Assessment Act 1997.",
            "Imposta sul reddito dei residenti — scaglioni dell'anno fiscale {fy0}-{fy1}.\n\nReddito annuo stimato: {rev} $ → imposta {imp} $/anno / 12 = {mens} $/mese.\nFascia esente: 18 200 $. Detrazioni LITO/LMITO non modellate (netto prudente).\n\nBase giuridica: Income Tax Assessment Act 1997.",
            "Impuesto sobre la renta de residentes — escala del año fiscal {fy0}-{fy1}.\n\nRenta anual estimada: {rev} $ → impuesto {imp} $/año / 12 = {mens} $/mes.\nTramo exento: 18 200 $. Deducciones LITO/LMITO no modeladas (neto prudente).\n\nBase legal: Income Tax Assessment Act 1997.",
        ],
        "AU_MEDICARE" => [
            "Medicare levy — 2 % of taxable income (public health funding).\nAmount: {m} $/month. Low-income reductions and surcharge (MLS) not modelled.\n\nLegal basis: Medicare Levy Act 1986.",
            "Medicare levy — 2 % des steuerpflichtigen Einkommens (Finanzierung des öffentlichen Gesundheitswesens).\nBetrag: {m} $/Monat. Ermäßigungen für Geringverdiener und Zuschlag (MLS) nicht modelliert.\n\nRechtsgrundlage: Medicare Levy Act 1986.",
            "Medicare levy — 2 % van het belastbaar inkomen (financiering volksgezondheid).\nBedrag: {m} $/maand. Verlagingen voor lage inkomens en toeslag (MLS) niet gemodelleerd.\n\nWettelijke basis: Medicare Levy Act 1986.",
            "Medicare levy — 2 % del reddito imponibile (finanziamento della sanità pubblica).\nImporto: {m} $/mese. Riduzioni per bassi redditi e sovrattassa (MLS) non modellate.\n\nBase giuridica: Medicare Levy Act 1986.",
            "Medicare levy — 2 % de la renta imponible (financiación de la sanidad pública).\nImporte: {m} $/mes. Reducciones para rentas bajas y recargo (MLS) no modelados.\n\nBase legal: Medicare Levy Act 1986.",
        ],
        "AU_SUPER" => [
            "Superannuation Guarantee — pension, 100 % employer, paid on top of salary.\nTax-year rate: {t} %. Base capped at the maximum contribution base.\nEmployer: {mp} $/month.\n\nLegal basis: SGAA 1992.",
            "Superannuation Guarantee — Rente, 100 % Arbeitgeber, zusätzlich zum Gehalt gezahlt.\nSatz des Steuerjahres: {t} %. Bemessungsgrundlage auf die maximum contribution base begrenzt.\nArbeitgeber: {mp} $/Monat.\n\nRechtsgrundlage: SGAA 1992.",
            "Superannuation Guarantee — pensioen, 100 % werkgever, bovenop het salaris betaald.\nTarief van het belastingjaar: {t} %. Grondslag begrensd tot de maximum contribution base.\nWerkgever: {mp} $/maand.\n\nWettelijke basis: SGAA 1992.",
            "Superannuation Guarantee — pensione, 100 % datore di lavoro, versata in aggiunta alla retribuzione.\nAliquota dell'anno fiscale: {t} %. Base limitata alla maximum contribution base.\nDatore di lavoro: {mp} $/mese.\n\nBase giuridica: SGAA 1992.",
            "Superannuation Guarantee — pensión, 100 % empleador, abonada además del salario.\nTipo del año fiscal: {t} %. Base limitada a la maximum contribution base.\nEmpleador: {mp} $/mes.\n\nBase legal: SGAA 1992.",
        ],
        // ── Belgique ──
        "BE_ONSS_SAL" => [
            "Personal ONSS contribution of 13.07 % on gross salary. Covers: sickness-disability, pension, unemployment, work accidents, family allowances. Base: full gross salary, no cap.\n\nEmployee: {ts_pct} % × {brut} € = {ms} €\nRate stable since 2003. Legal basis: Law of 27/06/1969; annual ONSS Royal Decrees.",
            "Persönlicher ONSS-Beitrag von 13,07 % auf das Bruttogehalt. Deckt ab: Krankheit-Invalidität, Rente, Arbeitslosigkeit, Arbeitsunfälle, Familienbeihilfen. Bemessungsgrundlage: volles Bruttogehalt, ohne Obergrenze.\n\nArbeitnehmer: {ts_pct} % × {brut} € = {ms} €\nSatz stabil seit 2003. Rechtsgrundlage: Gesetz vom 27.06.1969; jährliche ONSS-Erlasse.",
            "Persoonlijke ONSS-bijdrage van 13,07 % op het brutoloon. Dekt: ziekte-invaliditeit, pensioen, werkloosheid, arbeidsongevallen, kinderbijslag. Grondslag: volledig brutoloon, zonder plafond.\n\nWerknemer: {ts_pct} % × {brut} € = {ms} €\nTarief stabiel sinds 2003. Wettelijke basis: wet van 27/06/1969; jaarlijkse RSZ-besluiten.",
            "Contributo personale ONSS del 13,07 % sulla retribuzione lorda. Copre: malattia-invalidità, pensione, disoccupazione, infortuni sul lavoro, assegni familiari. Base: retribuzione lorda intera, senza massimale.\n\nDipendente: {ts_pct} % × {brut} € = {ms} €\nAliquota stabile dal 2003. Base giuridica: legge del 27/06/1969; decreti reali ONSS annuali.",
            "Cotización personal ONSS del 13,07 % sobre el salario bruto. Cubre: enfermedad-invalidez, pensión, desempleo, accidentes laborales, prestaciones familiares. Base: salario bruto íntegro, sin tope.\n\nTrabajador: {ts_pct} % × {brut} € = {ms} €\nTipo estable desde 2003. Base legal: ley de 27/06/1969; decretos reales ONSS anuales.",
        ],
        "BE_ONSS_PAT" => [
            "Global ONSS employer contribution ({tp_pct} % of gross). Groups: pension (8.86 %), sickness-disability (5.90 %), unemployment (1.46 %), family allowances (5.25 %), miscellaneous. The structural reduction (BE_RED_STRUCT) is applied separately. Base: full gross salary, no cap.\n\nEmployer: {tp_pct} % × {brut} € = {mp} €\nLegal basis: Law of 27/06/1969; annual ONSS Royal Decrees.",
            "Globaler ONSS-Arbeitgeberbeitrag ({tp_pct} % des Brutto). Umfasst: Rente (8,86 %), Krankheit-Invalidität (5,90 %), Arbeitslosigkeit (1,46 %), Familienbeihilfen (5,25 %), Sonstiges. Die strukturelle Ermäßigung (BE_RED_STRUCT) wird separat angewandt. Bemessungsgrundlage: volles Bruttogehalt, ohne Obergrenze.\n\nArbeitgeber: {tp_pct} % × {brut} € = {mp} €\nRechtsgrundlage: Gesetz vom 27.06.1969; jährliche ONSS-Erlasse.",
            "Globale ONSS-werkgeversbijdrage ({tp_pct} % van het bruto). Omvat: pensioen (8,86 %), ziekte-invaliditeit (5,90 %), werkloosheid (1,46 %), kinderbijslag (5,25 %), diverse. De structurele vermindering (BE_RED_STRUCT) wordt apart toegepast. Grondslag: volledig brutoloon, zonder plafond.\n\nWerkgever: {tp_pct} % × {brut} € = {mp} €\nWettelijke basis: wet van 27/06/1969; jaarlijkse RSZ-besluiten.",
            "Contributo datoriale globale ONSS ({tp_pct} % del lordo). Raggruppa: pensione (8,86 %), malattia-invalidità (5,90 %), disoccupazione (1,46 %), assegni familiari (5,25 %), varie. La riduzione strutturale (BE_RED_STRUCT) è applicata separatamente. Base: retribuzione lorda intera, senza massimale.\n\nDatore di lavoro: {tp_pct} % × {brut} € = {mp} €\nBase giuridica: legge del 27/06/1969; decreti reali ONSS annuali.",
            "Cotización patronal global ONSS ({tp_pct} % del bruto). Agrupa: pensión (8,86 %), enfermedad-invalidez (5,90 %), desempleo (1,46 %), prestaciones familiares (5,25 %), varios. La reducción estructural (BE_RED_STRUCT) se aplica por separado. Base: salario bruto íntegro, sin tope.\n\nEmpleador: {tp_pct} % × {brut} € = {mp} €\nBase legal: ley de 27/06/1969; decretos reales ONSS anuales.",
        ],
        "BE_BONUS_EMPLOI" => [
            "Monthly reduction of personal ONSS contributions (13.07 %) for low-wage workers. The amount tapers off between the low and high thresholds.\n\n{annee}: low threshold {sb} €/yr — high threshold {sh} €/yr — max {mm} €/month\nEstimated annual salary: {ann} € → monthly reduction: {m} €\nIndicative effective rate: {teff} %\n\nThe reduction is deducted from the ONSS contribution owed by the worker. Legal basis: Law of 20/12/1999; annual ONSS Royal Decrees.",
            "Monatliche Ermäßigung der persönlichen ONSS-Beiträge (13,07 %) für Geringverdiener. Der Betrag ist degressiv zwischen unterer und oberer Schwelle.\n\n{annee}: untere Schwelle {sb} €/Jahr — obere Schwelle {sh} €/Jahr — max. {mm} €/Monat\nGeschätztes Jahresgehalt: {ann} € → monatliche Ermäßigung: {m} €\nIndikativer effektiver Satz: {teff} %\n\nDie Ermäßigung wird vom ONSS-Beitrag des Arbeitnehmers abgezogen. Rechtsgrundlage: Gesetz vom 20.12.1999; jährliche ONSS-Erlasse.",
            "Maandelijkse vermindering van de persoonlijke ONSS-bijdragen (13,07 %) voor laagbetaalde werknemers. Het bedrag is degressief tussen de lage en de hoge drempel.\n\n{annee}: lage drempel {sb} €/jr — hoge drempel {sh} €/jr — max {mm} €/maand\nGeschat jaarloon: {ann} € → maandelijkse vermindering: {m} €\nIndicatief effectief tarief: {teff} %\n\nDe vermindering wordt afgetrokken van de door de werknemer verschuldigde ONSS-bijdrage. Wettelijke basis: wet van 20/12/1999; jaarlijkse RSZ-besluiten.",
            "Riduzione mensile dei contributi personali ONSS (13,07 %) per i lavoratori a basso salario. L'importo è decrescente tra la soglia bassa e quella alta.\n\n{annee}: soglia bassa {sb} €/anno — soglia alta {sh} €/anno — max {mm} €/mese\nSalario annuo stimato: {ann} € → riduzione mensile: {m} €\nAliquota effettiva indicativa: {teff} %\n\nLa riduzione è detratta dal contributo ONSS dovuto dal lavoratore. Base giuridica: legge del 20/12/1999; decreti reali ONSS annuali.",
            "Reducción mensual de las cotizaciones personales ONSS (13,07 %) para trabajadores con salarios bajos. El importe es decreciente entre el umbral bajo y el alto.\n\n{annee}: umbral bajo {sb} €/año — umbral alto {sh} €/año — máx {mm} €/mes\nSalario anual estimado: {ann} € → reducción mensual: {m} €\nTipo efectivo indicativo: {teff} %\n\nLa reducción se deduce de la cotización ONSS adeudada por el trabajador. Base legal: ley de 20/12/1999; decretos reales ONSS anuales.",
        ],
        "BE_RED_STRUCT" => [
            "Monthly reduction of employer ONSS contributions (Royal Decree 16/05/2003). Flat amount if salary ≤ threshold, tapering down to 1.5 × threshold.\n\n{annee}: full amount {mp} €/month — threshold {seuil} €/yr\nEstimated annual salary: {ann} € → monthly reduction: {m} €\n\nThe reduction is offset against total employer ONSS contributions. Legal basis: Royal Decree 16/05/2003 (structural reduction) + annual ONSS Royal Decrees.",
            "Monatliche Ermäßigung der ONSS-Arbeitgeberbeiträge (Königlicher Erlass 16.05.2003). Pauschalbetrag bei Gehalt ≤ Schwelle, degressiv bis 1,5 × Schwelle.\n\n{annee}: voller Betrag {mp} €/Monat — Schwelle {seuil} €/Jahr\nGeschätztes Jahresgehalt: {ann} € → monatliche Ermäßigung: {m} €\n\nDie Ermäßigung wird mit den gesamten ONSS-Arbeitgeberbeiträgen verrechnet. Rechtsgrundlage: Königlicher Erlass 16.05.2003 (strukturelle Ermäßigung) + jährliche ONSS-Erlasse.",
            "Maandelijkse vermindering van de ONSS-werkgeversbijdragen (KB 16/05/2003). Forfaitair bedrag als loon ≤ drempel, degressief tot 1,5 × drempel.\n\n{annee}: vol bedrag {mp} €/maand — drempel {seuil} €/jr\nGeschat jaarloon: {ann} € → maandelijkse vermindering: {m} €\n\nDe vermindering wordt verrekend met het totaal van de RSZ-werkgeversbijdragen. Wettelijke basis: KB 16/05/2003 (structurele vermindering) + jaarlijkse RSZ-besluiten.",
            "Riduzione mensile dei contributi datoriali ONSS (Regio Decreto 16/05/2003). Importo forfettario se la retribuzione ≤ soglia, decrescente fino a 1,5 × soglia.\n\n{annee}: importo pieno {mp} €/mese — soglia {seuil} €/anno\nSalario annuo stimato: {ann} € → riduzione mensile: {m} €\n\nLa riduzione è scomputata dal totale dei contributi datoriali ONSS. Base giuridica: Regio Decreto 16/05/2003 (riduzione strutturale) + decreti reali ONSS annuali.",
            "Reducción mensual de las cotizaciones patronales ONSS (Real Decreto 16/05/2003). Importe a tanto alzado si el salario ≤ umbral, decreciente hasta 1,5 × umbral.\n\n{annee}: importe pleno {mp} €/mes — umbral {seuil} €/año\nSalario anual estimado: {ann} € → reducción mensual: {m} €\n\nLa reducción se descuenta del total de las cotizaciones patronales ONSS. Base legal: Real Decreto 16/05/2003 (reducción estructural) + decretos reales ONSS anuales.",
        ],
        "BE_PP" => [
            "Monthly withholding at source of personal income tax (IPP/PB). The employer must apply this withholding (art. 270 ff. CIR92). Adjusted on the annual income-tax return.\n\n[ {annee} calculation — region: {rl} ]\nMonthly gross salary: {brut} €\nEstimated annual income: {brut_a} € (× 12)\nProfessional-expense allowance (30 %, cap): − {fp} €\nEstimated net taxable income: {rn} €\nGross annual IPP: {ipp} €\nBasic allowance: − IPP({exo} €) deducted\nRegional coefficient: × {fac}\nAnnual withholding: {ppa} €\nMonthly withholding: {ppm} € (÷ 12)\nEffective rate: {teff} %\n\nNote: annualised-scale approximation. The official SPF Finances tables account for family situation and other deductions.\nLegal basis: CIR92 art. 130-145 + SPF Finances circulars {annee}.",
            "Monatlicher Quellenabzug der Einkommensteuer natürlicher Personen (IPP/PB). Der Arbeitgeber muss diesen Abzug vornehmen (Art. 270 ff. CIR92). Ausgleich bei der jährlichen IPP-Erklärung.\n\n[ Berechnung {annee} — Region: {rl} ]\nMonatliches Bruttogehalt: {brut} €\nGeschätztes Jahreseinkommen: {brut_a} € (× 12)\nWerbungskostenpauschale (30 %, Obergrenze): − {fp} €\nGeschätztes steuerpflichtiges Nettoeinkommen: {rn} €\nBrutto-Jahres-IPP: {ipp} €\nGrundfreibetrag: − IPP({exo} €) abgezogen\nRegionaler Koeffizient: × {fac}\nJährlicher Abzug: {ppa} €\nMonatlicher Abzug: {ppm} € (÷ 12)\nEffektiver Satz: {teff} %\n\nHinweis: Näherung über annualisierten Tarif. Die offiziellen Tabellen des SPF Finances berücksichtigen die Familiensituation und weitere Abzüge.\nRechtsgrundlage: CIR92 Art. 130-145 + Rundschreiben SPF Finances {annee}.",
            "Maandelijkse inhouding aan de bron van de personenbelasting (PB/IPP). De werkgever moet deze inhouding verrichten (art. 270 e.v. WIB92). Verrekening bij de jaarlijkse PB-aangifte.\n\n[ Berekening {annee} — gewest: {rl} ]\nMaandelijks brutoloon: {brut} €\nGeschat jaarinkomen: {brut_a} € (× 12)\nForfaitaire beroepskosten (30 %, plafond): − {fp} €\nGeschat netto belastbaar inkomen: {rn} €\nBruto jaarlijkse PB: {ipp} €\nBelastingvrije som: − PB({exo} €) afgetrokken\nGewestelijke coëfficiënt: × {fac}\nJaarlijkse voorheffing: {ppa} €\nMaandelijkse voorheffing: {ppm} € (÷ 12)\nEffectief tarief: {teff} %\n\nOpmerking: benadering via geannualiseerd tarief. De officiële tabellen van de FOD Financiën houden rekening met de gezinssituatie en andere aftrekken.\nWettelijke basis: WIB92 art. 130-145 + circulaires FOD Financiën {annee}.",
            "Ritenuta mensile alla fonte dell'imposta sulle persone fisiche (IPP/PB). Il datore di lavoro è tenuto a effettuare questa ritenuta (art. 270 e segg. CIR92). Conguaglio in sede di dichiarazione IPP annuale.\n\n[ Calcolo {annee} — regione: {rl} ]\nRetribuzione lorda mensile: {brut} €\nReddito annuo stimato: {brut_a} € (× 12)\nDeduzione forfettaria spese professionali (30 %, massimale): − {fp} €\nReddito netto imponibile stimato: {rn} €\nIPP lorda annua: {ipp} €\nDetrazione di base: − IPP({exo} €) dedotta\nCoefficiente regionale: × {fac}\nRitenuta annua: {ppa} €\nRitenuta mensile: {ppm} € (÷ 12)\nAliquota effettiva: {teff} %\n\nNota: approssimazione tramite scaglioni annualizzati. Le tabelle ufficiali del SPF Finances tengono conto della situazione familiare e di altre detrazioni.\nBase giuridica: CIR92 art. 130-145 + circolari SPF Finances {annee}.",
            "Retención mensual en origen del impuesto sobre las personas físicas (IPP/PB). El empleador está obligado a practicar esta retención (art. 270 y ss. CIR92). Regularización en la declaración anual del IPP.\n\n[ Cálculo {annee} — región: {rl} ]\nSalario bruto mensual: {brut} €\nRenta anual estimada: {brut_a} € (× 12)\nDeducción a tanto alzado por gastos profesionales (30 %, tope): − {fp} €\nRenta neta imponible estimada: {rn} €\nIPP bruto anual: {ipp} €\nMínimo exento: − IPP({exo} €) deducido\nCoeficiente regional: × {fac}\nRetención anual: {ppa} €\nRetención mensual: {ppm} € (÷ 12)\nTipo efectivo: {teff} %\n\nNota: aproximación mediante escala anualizada. Las tablas oficiales del SPF Finances tienen en cuenta la situación familiar y otras deducciones.\nBase legal: CIR92 art. 130-145 + circulares SPF Finances {annee}.",
        ],
        // ── Canada ──
        "CA_RPC" => [
            "The Canada Pension Plan (CPP) is the mandatory federal retirement scheme, in force since 1966 (R.S.C. 1985, c. C-8). It pays retirement, disability and survivor pensions.\n\n[ {an} calculation ]\nPensionable earnings = min(gross, YMPE/12) − basic exemption\n= min({brut}, {mga}) − {exo} = {pens} CAD\nRate: {ts} % employee = {tp} % employer (matched rates)\n\nThe gradual enhancement 2019-2023 raised the rate from 4.95 % to 5.95 %. Earnings above the YMPE ({mga} CAD/month) earn no pension under the base plan (see CPP2 for phase 2). The basic exemption ({exo} CAD/month = 3,500 CAD/yr) applies to everyone.",
            "Der Canada Pension Plan (CPP/RPC) ist das obligatorische föderale Rentensystem, in Kraft seit 1966 (R.S.C. 1985, c. C-8). Er zahlt Alters-, Invaliditäts- und Hinterbliebenenrenten.\n\n[ Berechnung {an} ]\nRentenfähiges Einkommen = min(brutto, YMPE/12) − Grundfreibetrag\n= min({brut}, {mga}) − {exo} = {pens} CAD\nSatz: {ts} % Arbeitnehmer = {tp} % Arbeitgeber (paritätisch)\n\nDie schrittweise Erhöhung 2019-2023 hob den Satz von 4,95 % auf 5,95 %. Einkommen über dem YMPE ({mga} CAD/Monat) begründen im Grundsystem keinen Rentenanspruch (siehe CPP2 für Phase 2). Der Grundfreibetrag ({exo} CAD/Monat = 3 500 CAD/Jahr) gilt für alle.",
            "Het Canada Pension Plan (CPP/RPC) is het verplichte federale pensioenstelsel, van kracht sinds 1966 (R.S.C. 1985, c. C-8). Het keert ouderdoms-, invaliditeits- en nabestaandenpensioenen uit.\n\n[ Berekening {an} ]\nPensioengevend inkomen = min(bruto, YMPE/12) − basisvrijstelling\n= min({brut}, {mga}) − {exo} = {pens} CAD\nTarief: {ts} % werknemer = {tp} % werkgever (gelijke tarieven)\n\nDe geleidelijke verhoging 2019-2023 bracht het tarief van 4,95 % naar 5,95 %. Inkomen boven de YMPE ({mga} CAD/maand) geeft geen pensioenrecht in het basisstelsel (zie CPP2 voor fase 2). De basisvrijstelling ({exo} CAD/maand = 3.500 CAD/jr) geldt voor iedereen.",
            "Il Canada Pension Plan (CPP/RPC) è il regime pensionistico federale obbligatorio, in vigore dal 1966 (R.S.C. 1985, c. C-8). Eroga pensioni di vecchiaia, invalidità e superstiti.\n\n[ Calcolo {an} ]\nRetribuzione pensionabile = min(lordo, YMPE/12) − franchigia di base\n= min({brut}, {mga}) − {exo} = {pens} CAD\nAliquota: {ts} % dipendente = {tp} % datore di lavoro (aliquote appaiate)\n\nIl potenziamento graduale 2019-2023 ha portato l'aliquota dal 4,95 % al 5,95 %. I redditi oltre lo YMPE ({mga} CAD/mese) non danno diritto a pensione nel regime di base (vedi CPP2 per la fase 2). La franchigia di base ({exo} CAD/mese = 3 500 CAD/anno) si applica a tutti.",
            "El Canada Pension Plan (CPP/RPC) es el régimen de jubilación federal obligatorio, en vigor desde 1966 (R.S.C. 1985, c. C-8). Paga pensiones de jubilación, invalidez y supervivencia.\n\n[ Cálculo {an} ]\nGanancias pensionables = mín(bruto, YMPE/12) − exención de base\n= mín({brut}, {mga}) − {exo} = {pens} CAD\nTipo: {ts} % trabajador = {tp} % empleador (tipos emparejados)\n\nLa mejora gradual 2019-2023 elevó el tipo del 4,95 % al 5,95 %. Las ganancias por encima del YMPE ({mga} CAD/mes) no generan derecho a pensión en el régimen base (véase CPP2 para la fase 2). La exención de base ({exo} CAD/mes = 3 500 CAD/año) se aplica a todos.",
        ],
        "CA_RPC2" => [
            "CPP2 (phase 2 of the CPP enhancement) applies to the earnings band between the YMPE ({mga} CAD/month) and the YAMPE ({mgap2} CAD/month).\n\nAdditional earnings {an}: {base2} CAD\nRate: 4.00 % employee + 4.00 % employer (no basic exemption)\n\nCPP2 targets middle-to-high earners who already contribute the CPP base maximum. Over time, the CPP2 pension will provide income replacement above the base plan alone. Introduced by Budget Implementation Act, 2018, No. 1 (S.C. 2018, c. 12).",
            "CPP2 (Phase 2 der CPP-Aufstockung) gilt für das Einkommensband zwischen YMPE ({mga} CAD/Monat) und YAMPE ({mgap2} CAD/Monat).\n\nZusätzliches Einkommen {an}: {base2} CAD\nSatz: 4,00 % Arbeitnehmer + 4,00 % Arbeitgeber (ohne Grundfreibetrag)\n\nCPP2 richtet sich an mittlere bis hohe Einkommen, die bereits das CPP-Grundmaximum zahlen. Langfristig wird die CPP2-Rente einen höheren Einkommensersatz als das Grundsystem allein bieten. Eingeführt durch den Budget Implementation Act, 2018, No. 1 (S.C. 2018, c. 12).",
            "CPP2 (fase 2 van de CPP-verhoging) is van toepassing op de inkomensschijf tussen de YMPE ({mga} CAD/maand) en de YAMPE ({mgap2} CAD/maand).\n\nAanvullend inkomen {an}: {base2} CAD\nTarief: 4,00 % werknemer + 4,00 % werkgever (zonder basisvrijstelling)\n\nCPP2 richt zich op midden- tot hoge inkomens die al het CPP-basismaximum betalen. Op termijn biedt het CPP2-pensioen een hogere inkomensvervanging dan het basisstelsel alleen. Ingevoerd bij de Budget Implementation Act, 2018, No. 1 (S.C. 2018, c. 12).",
            "Il CPP2 (fase 2 del potenziamento CPP) si applica alla fascia di reddito compresa tra lo YMPE ({mga} CAD/mese) e lo YAMPE ({mgap2} CAD/mese).\n\nReddito aggiuntivo {an}: {base2} CAD\nAliquota: 4,00 % dipendente + 4,00 % datore di lavoro (senza franchigia di base)\n\nIl CPP2 si rivolge ai redditi medio-alti che già versano il massimale CPP di base. Nel tempo, la pensione CPP2 garantirà una sostituzione del reddito superiore al solo regime di base. Introdotto dal Budget Implementation Act, 2018, No. 1 (S.C. 2018, c. 12).",
            "El CPP2 (fase 2 de la mejora del CPP) se aplica al tramo de ganancias comprendido entre el YMPE ({mga} CAD/mes) y el YAMPE ({mgap2} CAD/mes).\n\nGanancias adicionales {an}: {base2} CAD\nTipo: 4,00 % trabajador + 4,00 % empleador (sin exención de base)\n\nEl CPP2 se dirige a rentas medias-altas que ya cotizan el máximo del CPP base. Con el tiempo, la pensión CPP2 ofrecerá una sustitución de renta superior a la del régimen base por sí solo. Introducido por la Budget Implementation Act, 2018, No. 1 (S.C. 2018, c. 12).",
        ],
        "CA_AE" => [
            "Employment Insurance (EI) is the federal unemployment benefit scheme, governed by the Employment Insurance Act (S.C. 1996, c. 23). It pays regular (unemployment), special (sickness, maternity, paternity, caregiver) and work-sharing benefits.\n\nMIE {an}: {maga} CAD/month ({magaa} CAD/yr)\nEmployee rate: {ts} % — Employer rate: {tp} % (= employee × 1.4)\n\nThe 1.4 multiplier is set by s. 68 of the EIA. The employer pays more to fund the overall risk borne by the scheme. Quebec workers pay a reduced rate because the QPIP covers their parental benefits (see QC_AE rate).",
            "Die Employment Insurance (EI/AE) ist das föderale Arbeitslosenleistungssystem, geregelt durch den Employment Insurance Act (S.C. 1996, c. 23). Sie zahlt reguläre (Arbeitslosigkeit), besondere (Krankheit, Mutterschaft, Vaterschaft, Pflege) und Kurzarbeitsleistungen.\n\nMIE {an}: {maga} CAD/Monat ({magaa} CAD/Jahr)\nArbeitnehmersatz: {ts} % — Arbeitgebersatz: {tp} % (= Arbeitnehmer × 1,4)\n\nDer Faktor 1,4 ist in s. 68 des EIA festgelegt. Der Arbeitgeber zahlt mehr, um das vom System getragene Gesamtrisiko zu finanzieren. Arbeitnehmer in Québec zahlen einen reduzierten Satz, da das QPIP ihre Elternleistungen abdeckt (siehe Satz QC_AE).",
            "De Employment Insurance (EI/AE) is het federale werkloosheidsstelsel, geregeld door de Employment Insurance Act (S.C. 1996, c. 23). Het keert reguliere (werkloosheid), bijzondere (ziekte, moederschap, vaderschap, mantelzorg) en werkdelingsuitkeringen uit.\n\nMIE {an}: {maga} CAD/maand ({magaa} CAD/jr)\nWerknemerstarief: {ts} % — Werkgeverstarief: {tp} % (= werknemer × 1,4)\n\nDe vermenigvuldiger 1,4 is vastgelegd in s. 68 van de EIA. De werkgever betaalt meer om het totale risico van het stelsel te financieren. Werknemers in Québec betalen een verlaagd tarief omdat het QPIP hun ouderschapsuitkeringen dekt (zie tarief QC_AE).",
            "L'Employment Insurance (EI/AE) è il regime federale di indennità di disoccupazione, disciplinato dall'Employment Insurance Act (S.C. 1996, c. 23). Eroga prestazioni ordinarie (disoccupazione), speciali (malattia, maternità, paternità, caregiver) e di lavoro condiviso.\n\nMIE {an}: {maga} CAD/mese ({magaa} CAD/anno)\nAliquota dipendente: {ts} % — Aliquota datore di lavoro: {tp} % (= dipendente × 1,4)\n\nIl moltiplicatore 1,4 è fissato dall'art. 68 dell'EIA. Il datore di lavoro versa di più per finanziare il rischio complessivo del regime. I lavoratori del Québec pagano un'aliquota ridotta perché il QPIP copre le loro prestazioni parentali (vedi aliquota QC_AE).",
            "El Employment Insurance (EI/AE) es el régimen federal de prestaciones por desempleo, regido por la Employment Insurance Act (S.C. 1996, c. 23). Paga prestaciones ordinarias (desempleo), especiales (enfermedad, maternidad, paternidad, cuidadores) y de trabajo compartido.\n\nMIE {an}: {maga} CAD/mes ({magaa} CAD/año)\nTipo trabajador: {ts} % — Tipo empleador: {tp} % (= trabajador × 1,4)\n\nEl multiplicador 1,4 lo fija el art. 68 de la EIA. El empleador cotiza más para financiar el riesgo global asumido por el régimen. Los trabajadores de Quebec pagan un tipo reducido porque el QPIP cubre sus prestaciones parentales (véase el tipo QC_AE).",
        ],
        "CA_IMPOT_FED" => [
            "Monthly federal income-tax withholding. The employer acts as a withholding agent (source deductions — form TD1).\n\n[ {an} calculation — federal scale ]\nEstimated annual income: {rev} CAD\nGross annual tax: {ib} CAD\nPersonal credit (BPA): − {cred} CAD\nNet annual tax: {inet} CAD\nMonthly withholding: {mens} CAD (÷ 12)\nEffective rate: {teff} %\n\n{an} scale: 15/20.5/26/29/33 %. The Basic Personal Amount ({mpb} CAD) yields a 15 % credit = {cred} CAD/yr. Adjusted in December or via the annual T1 return.",
            "Monatlicher Einbehalt der Bundeseinkommensteuer. Der Arbeitgeber handelt als Steuerabzugsverpflichteter (Quellenabzüge — Formular TD1).\n\n[ Berechnung {an} — Bundestarif ]\nGeschätztes Jahreseinkommen: {rev} CAD\nBrutto-Jahressteuer: {ib} CAD\nPersönliche Gutschrift (BPA): − {cred} CAD\nNetto-Jahressteuer: {inet} CAD\nMonatlicher Einbehalt: {mens} CAD (÷ 12)\nEffektiver Satz: {teff} %\n\nTarif {an}: 15/20,5/26/29/33 %. Der Grundbetrag ({mpb} CAD) ergibt eine Gutschrift von 15 % = {cred} CAD/Jahr. Ausgleich im Dezember oder über die jährliche T1-Erklärung.",
            "Maandelijkse inhouding federale inkomstenbelasting. De werkgever treedt op als inhoudingsplichtige (bronheffingen — formulier TD1).\n\n[ Berekening {an} — federale schaal ]\nGeschat jaarinkomen: {rev} CAD\nBruto jaarbelasting: {ib} CAD\nPersoonlijk krediet (BPA): − {cred} CAD\nNetto jaarbelasting: {inet} CAD\nMaandelijkse inhouding: {mens} CAD (÷ 12)\nEffectief tarief: {teff} %\n\nSchaal {an}: 15/20,5/26/29/33 %. Het basisbedrag ({mpb} CAD) levert een krediet van 15 % = {cred} CAD/jr op. Verrekening in december of via de jaarlijkse T1-aangifte.",
            "Ritenuta mensile dell'imposta federale sul reddito. Il datore di lavoro agisce come sostituto d'imposta (ritenute alla fonte — modulo TD1).\n\n[ Calcolo {an} — scaglioni federali ]\nReddito annuo stimato: {rev} CAD\nImposta lorda annua: {ib} CAD\nCredito personale (BPA): − {cred} CAD\nImposta netta annua: {inet} CAD\nRitenuta mensile: {mens} CAD (÷ 12)\nAliquota effettiva: {teff} %\n\nScaglioni {an}: 15/20,5/26/29/33 %. L'importo personale di base ({mpb} CAD) genera un credito del 15 % = {cred} CAD/anno. Conguaglio a dicembre o tramite la dichiarazione annuale T1.",
            "Retención mensual del impuesto federal sobre la renta. El empleador actúa como agente de retención (retenciones en origen — formulario TD1).\n\n[ Cálculo {an} — escala federal ]\nRenta anual estimada: {rev} CAD\nImpuesto bruto anual: {ib} CAD\nCrédito personal (BPA): − {cred} CAD\nImpuesto neto anual: {inet} CAD\nRetención mensual: {mens} CAD (÷ 12)\nTipo efectivo: {teff} %\n\nEscala {an}: 15/20,5/26/29/33 %. El importe personal básico ({mpb} CAD) genera un crédito del 15 % = {cred} CAD/año. Regularización en diciembre o mediante la declaración anual T1.",
        ],
        "ON_IMPOT_PROV" => [
            "Monthly Ontario provincial income-tax withholding (reference province outside Quebec). 2024 scale: 5.05/9.15/11.16/12.16/13.16 %. Ontario BPA 2024: 11,865 CAD → credit of 599.18 CAD/yr.\n\nEstimated annual income: {rev} CAD\nGross annual tax: {ib} CAD\nBPA credit: − 599.18 CAD\nNet annual tax: {inet} CAD\nMonthly withholding: {mens} CAD\nEffective rate: {teff} %\n\nNote: not applicable in Quebec (which has its own separate tax). The other provinces (BC, AB, excl. QC) have their own scales — use Ontario as a general approximation.",
            "Monatlicher Einbehalt der Provinzsteuer Ontario (Referenzprovinz außerhalb Québecs). Tarif 2024: 5,05/9,15/11,16/12,16/13,16 %. Ontario-Grundbetrag 2024: 11 865 CAD → Gutschrift 599,18 CAD/Jahr.\n\nGeschätztes Jahreseinkommen: {rev} CAD\nBrutto-Jahressteuer: {ib} CAD\nGrundbetrag-Gutschrift: − 599,18 CAD\nNetto-Jahressteuer: {inet} CAD\nMonatlicher Einbehalt: {mens} CAD\nEffektiver Satz: {teff} %\n\nHinweis: in Québec nicht anwendbar (eigene separate Steuer). Die übrigen Provinzen (BC, AB, ohne QC) haben eigene Tarife — Ontario als allgemeine Näherung verwenden.",
            "Maandelijkse inhouding provinciale belasting Ontario (referentieprovincie buiten Québec). Schaal 2024: 5,05/9,15/11,16/12,16/13,16 %. Ontario-basisbedrag 2024: 11.865 CAD → krediet van 599,18 CAD/jr.\n\nGeschat jaarinkomen: {rev} CAD\nBruto jaarbelasting: {ib} CAD\nBasiskrediet: − 599,18 CAD\nNetto jaarbelasting: {inet} CAD\nMaandelijkse inhouding: {mens} CAD\nEffectief tarief: {teff} %\n\nOpmerking: niet van toepassing in Québec (eigen aparte belasting). De overige provincies (BC, AB, excl. QC) hebben eigen schalen — gebruik Ontario als algemene benadering.",
            "Ritenuta mensile dell'imposta provinciale dell'Ontario (provincia di riferimento fuori dal Québec). Scaglioni 2024: 5,05/9,15/11,16/12,16/13,16 %. Importo personale di base Ontario 2024: 11 865 CAD → credito di 599,18 CAD/anno.\n\nReddito annuo stimato: {rev} CAD\nImposta lorda annua: {ib} CAD\nCredito di base: − 599,18 CAD\nImposta netta annua: {inet} CAD\nRitenuta mensile: {mens} CAD\nAliquota effettiva: {teff} %\n\nNota: non applicabile in Québec (che ha un'imposta propria separata). Le altre province (BC, AB, escl. QC) hanno scaglioni propri — usare l'Ontario come approssimazione generale.",
            "Retención mensual del impuesto provincial de Ontario (provincia de referencia fuera de Quebec). Escala 2024: 5,05/9,15/11,16/12,16/13,16 %. Importe personal básico de Ontario 2024: 11 865 CAD → crédito de 599,18 CAD/año.\n\nRenta anual estimada: {rev} CAD\nImpuesto bruto anual: {ib} CAD\nCrédito básico: − 599,18 CAD\nImpuesto neto anual: {inet} CAD\nRetención mensual: {mens} CAD\nTipo efectivo: {teff} %\n\nNota: no aplicable en Quebec (que tiene su propio impuesto separado). Las demás provincias (BC, AB, excl. QC) tienen sus propias escalas — usar Ontario como aproximación general.",
        ],
        "QC_IMPOT_PROV" => [
            "Quebec collects its own provincial tax directly (unique in Canada) through Revenu Québec, unlike the other provinces where the CRA collects both taxes together.\n\n{an} scale: 14/19/24/25.75 %. Quebec BPA: {mpb} CAD → credit: {cred} CAD/yr.\n\n[ Calculation ]\nEstimated annual income: {rev} CAD\nGross annual tax: {ib} CAD\nBPA credit: − {cred} CAD\nNet annual tax: {inet} CAD\nMonthly withholding: {mens} CAD\nEffective rate: {teff} %\n\nThe employer issues the RL-1 slip instead of the T4. The Quebec worker files two returns: T1 (federal) + TP-1 (provincial).",
            "Québec erhebt seine eigene Provinzsteuer direkt (einzigartig in Kanada) über Revenu Québec, anders als die übrigen Provinzen, wo die CRA beide Steuern gemeinsam erhebt.\n\nTarif {an}: 14/19/24/25,75 %. Québec-Grundbetrag: {mpb} CAD → Gutschrift: {cred} CAD/Jahr.\n\n[ Berechnung ]\nGeschätztes Jahreseinkommen: {rev} CAD\nBrutto-Jahressteuer: {ib} CAD\nGrundbetrag-Gutschrift: − {cred} CAD\nNetto-Jahressteuer: {inet} CAD\nMonatlicher Einbehalt: {mens} CAD\nEffektiver Satz: {teff} %\n\nDer Arbeitgeber stellt den Beleg RL-1 statt des T4 aus. Der Arbeitnehmer in Québec gibt zwei Erklärungen ab: T1 (Bund) + TP-1 (Provinz).",
            "Québec int zijn eigen provinciale belasting rechtstreeks (uniek in Canada) via Revenu Québec, anders dan de overige provincies waar de CRA beide belastingen samen int.\n\nSchaal {an}: 14/19/24/25,75 %. Québec-basisbedrag: {mpb} CAD → krediet: {cred} CAD/jr.\n\n[ Berekening ]\nGeschat jaarinkomen: {rev} CAD\nBruto jaarbelasting: {ib} CAD\nBasiskrediet: − {cred} CAD\nNetto jaarbelasting: {inet} CAD\nMaandelijkse inhouding: {mens} CAD\nEffectief tarief: {teff} %\n\nDe werkgever reikt het RL-1-attest uit in plaats van de T4. De werknemer in Québec dient twee aangiften in: T1 (federaal) + TP-1 (provinciaal).",
            "Il Québec riscuote la propria imposta provinciale direttamente (unico in Canada) tramite Revenu Québec, a differenza delle altre province dove la CRA riscuote entrambe le imposte insieme.\n\nScaglioni {an}: 14/19/24/25,75 %. Importo di base Québec: {mpb} CAD → credito: {cred} CAD/anno.\n\n[ Calcolo ]\nReddito annuo stimato: {rev} CAD\nImposta lorda annua: {ib} CAD\nCredito di base: − {cred} CAD\nImposta netta annua: {inet} CAD\nRitenuta mensile: {mens} CAD\nAliquota effettiva: {teff} %\n\nIl datore di lavoro emette il modulo RL-1 al posto del T4. Il lavoratore del Québec presenta due dichiarazioni: T1 (federale) + TP-1 (provinciale).",
            "Quebec recauda su propio impuesto provincial directamente (único en Canadá) a través de Revenu Québec, a diferencia de las demás provincias donde la CRA recauda ambos impuestos conjuntamente.\n\nEscala {an}: 14/19/24/25,75 %. Importe básico de Quebec: {mpb} CAD → crédito: {cred} CAD/año.\n\n[ Cálculo ]\nRenta anual estimada: {rev} CAD\nImpuesto bruto anual: {ib} CAD\nCrédito básico: − {cred} CAD\nImpuesto neto anual: {inet} CAD\nRetención mensual: {mens} CAD\nTipo efectivo: {teff} %\n\nEl empleador emite el comprobante RL-1 en lugar del T4. El trabajador de Quebec presenta dos declaraciones: T1 (federal) + TP-1 (provincial).",
        ],
        "AB_IMPOT_PROV" | "BC_IMPOT_PROV" | "MB_IMPOT_PROV" | "NB_IMPOT_PROV" | "NL_IMPOT_PROV"
        | "NS_IMPOT_PROV" | "NT_IMPOT_PROV" | "NU_IMPOT_PROV" | "PE_IMPOT_PROV" | "SK_IMPOT_PROV"
        | "YT_IMPOT_PROV" => [
            "Monthly provincial income-tax withholding — {nom}.\n{annee} scale: {tranches_desc}\n\n[ Calculation ]\nEstimated annual income: {revenu} CAD\nGross annual tax: {ib} CAD\nBPA credit: − {bpa} CAD\nNet annual tax: {inet} CAD\nMonthly withholding: {mens} CAD (÷ 12)\nEffective rate: {teff} %\n\nWithheld together with federal tax by the employer (withholding agent). Adjusted via the annual T1 return (CRA) and, where applicable, a complementary provincial return.",
            "Monatlicher Einbehalt der Provinzsteuer — {nom}.\nTarif {annee}: {tranches_desc}\n\n[ Berechnung ]\nGeschätztes Jahreseinkommen: {revenu} CAD\nBrutto-Jahressteuer: {ib} CAD\nGrundbetrag-Gutschrift: − {bpa} CAD\nNetto-Jahressteuer: {inet} CAD\nMonatlicher Einbehalt: {mens} CAD (÷ 12)\nEffektiver Satz: {teff} %\n\nWird zusammen mit der Bundessteuer vom Arbeitgeber einbehalten (Steuerabzugsverpflichteter). Ausgleich über die jährliche T1-Erklärung (CRA) und ggf. eine ergänzende Provinzerklärung.",
            "Maandelijkse inhouding provinciale belasting — {nom}.\nSchaal {annee}: {tranches_desc}\n\n[ Berekening ]\nGeschat jaarinkomen: {revenu} CAD\nBruto jaarbelasting: {ib} CAD\nBasiskrediet: − {bpa} CAD\nNetto jaarbelasting: {inet} CAD\nMaandelijkse inhouding: {mens} CAD (÷ 12)\nEffectief tarief: {teff} %\n\nSamen met de federale belasting ingehouden door de werkgever (inhoudingsplichtige). Verrekening via de jaarlijkse T1-aangifte (CRA) en, indien van toepassing, een aanvullende provinciale aangifte.",
            "Ritenuta mensile dell'imposta provinciale — {nom}.\nScaglioni {annee}: {tranches_desc}\n\n[ Calcolo ]\nReddito annuo stimato: {revenu} CAD\nImposta lorda annua: {ib} CAD\nCredito di base: − {bpa} CAD\nImposta netta annua: {inet} CAD\nRitenuta mensile: {mens} CAD (÷ 12)\nAliquota effettiva: {teff} %\n\nTrattenuta insieme all'imposta federale dal datore di lavoro (sostituto d'imposta). Conguaglio tramite la dichiarazione annuale T1 (CRA) e, se applicabile, una dichiarazione provinciale complementare.",
            "Retención mensual del impuesto provincial — {nom}.\nEscala {annee}: {tranches_desc}\n\n[ Cálculo ]\nRenta anual estimada: {revenu} CAD\nImpuesto bruto anual: {ib} CAD\nCrédito básico: − {bpa} CAD\nImpuesto neto anual: {inet} CAD\nRetención mensual: {mens} CAD (÷ 12)\nTipo efectivo: {teff} %\n\nRetenido junto con el impuesto federal por el empleador (agente de retención). Regularización mediante la declaración anual T1 (CRA) y, si procede, una declaración provincial complementaria.",
        ],
        // ── Québec ──
        "QC_RRQ" => [
            "The Quebec Pension Plan (QPP) is Quebec's equivalent of the CPP, but managed independently by Retraite Québec since 1966 (CQLR, c. R-9). Quebec workers contribute to the QPP, not the federal CPP.\n\n[ {an} calculation ]\nPensionable earnings = min(gross, YMPE/12) − basic exemption\n= min({brut}, {mga}) − {exo} = {pens} CAD\nRate {an}: {ts} % employee = {tp} % employer\n\nSince 2019, the QPP has been gradually enhanced (like the CPP): the rate rose each year from 5.55 % (2019) to 6.40 % (2023+). The QPP rate is slightly higher than the CPP for the same dates, owing to demographics and the history of the Quebec fund.",
            "Der Quebec Pension Plan (QPP/RRQ) ist das québecische Pendant zum CPP, wird aber seit 1966 unabhängig von Retraite Québec verwaltet (CQLR, c. R-9). Arbeitnehmer in Québec zahlen in den QPP, nicht in den föderalen CPP.\n\n[ Berechnung {an} ]\nRentenfähiges Einkommen = min(brutto, YMPE/12) − Grundfreibetrag\n= min({brut}, {mga}) − {exo} = {pens} CAD\nSatz {an}: {ts} % Arbeitnehmer = {tp} % Arbeitgeber\n\nSeit 2019 wird der QPP schrittweise aufgestockt (wie der CPP): Der Satz stieg jährlich von 5,55 % (2019) auf 6,40 % (2023+). Der QPP-Satz liegt für dieselben Zeiträume leicht über dem CPP, bedingt durch die Demografie und die Historie des québecischen Fonds.",
            "Het Quebec Pension Plan (QPP/RRQ) is het Québecse equivalent van het CPP, maar wordt sinds 1966 onafhankelijk beheerd door Retraite Québec (CQLR, c. R-9). Werknemers in Québec dragen bij aan het QPP, niet aan het federale CPP.\n\n[ Berekening {an} ]\nPensioengevend inkomen = min(bruto, YMPE/12) − basisvrijstelling\n= min({brut}, {mga}) − {exo} = {pens} CAD\nTarief {an}: {ts} % werknemer = {tp} % werkgever\n\nSinds 2019 wordt het QPP geleidelijk verhoogd (zoals het CPP): het tarief steeg elk jaar van 5,55 % (2019) naar 6,40 % (2023+). Het QPP-tarief ligt voor dezelfde data iets hoger dan het CPP, door de demografie en de geschiedenis van het Québecse fonds.",
            "Il Quebec Pension Plan (QPP/RRQ) è l'equivalente québecchese del CPP, ma gestito indipendentemente da Retraite Québec dal 1966 (CQLR, c. R-9). I lavoratori del Québec versano al QPP e non al CPP federale.\n\n[ Calcolo {an} ]\nRetribuzione pensionabile = min(lordo, YMPE/12) − franchigia di base\n= min({brut}, {mga}) − {exo} = {pens} CAD\nAliquota {an}: {ts} % dipendente = {tp} % datore di lavoro\n\nDal 2019 il QPP è potenziato gradualmente (come il CPP): l'aliquota è salita ogni anno dal 5,55 % (2019) al 6,40 % (2023+). L'aliquota QPP è leggermente superiore al CPP per le stesse date, per via della demografia e della storia del fondo québecchese.",
            "El Quebec Pension Plan (QPP/RRQ) es el equivalente quebequés del CPP, pero gestionado de forma independiente por Retraite Québec desde 1966 (CQLR, c. R-9). Los trabajadores de Quebec cotizan al QPP y no al CPP federal.\n\n[ Cálculo {an} ]\nGanancias pensionables = mín(bruto, YMPE/12) − exención de base\n= mín({brut}, {mga}) − {exo} = {pens} CAD\nTipo {an}: {ts} % trabajador = {tp} % empleador\n\nDesde 2019, el QPP se mejora progresivamente (igual que el CPP): el tipo subió cada año del 5,55 % (2019) al 6,40 % (2023+). El tipo del QPP es ligeramente superior al del CPP en las mismas fechas, por la demografía y la historia del fondo quebequés.",
        ],
        "QC_RRQ2" => [
            "Phase 2 of the QPP enhancement applies to earnings between the YMPE ({mga} CAD/month) and the YAMPE ({mgap2} CAD/month) at a 4 % rate. Additional earnings {an}: {base2} CAD. Identical to CPP2 except managed by Retraite Québec.",
            "Phase 2 der QPP-Aufstockung gilt für Einkommen zwischen YMPE ({mga} CAD/Monat) und YAMPE ({mgap2} CAD/Monat) zu einem Satz von 4 %. Zusätzliches Einkommen {an}: {base2} CAD. Identisch mit CPP2, jedoch von Retraite Québec verwaltet.",
            "Fase 2 van de QPP-verhoging is van toepassing op inkomen tussen de YMPE ({mga} CAD/maand) en de YAMPE ({mgap2} CAD/maand) tegen een tarief van 4 %. Aanvullend inkomen {an}: {base2} CAD. Identiek aan CPP2, maar beheerd door Retraite Québec.",
            "La fase 2 del potenziamento del QPP si applica ai redditi tra lo YMPE ({mga} CAD/mese) e lo YAMPE ({mgap2} CAD/mese) con aliquota del 4 %. Reddito aggiuntivo {an}: {base2} CAD. Identico al CPP2 ma gestito da Retraite Québec.",
            "La fase 2 de la mejora del QPP se aplica a las ganancias entre el YMPE ({mga} CAD/mes) y el YAMPE ({mgap2} CAD/mes) a un tipo del 4 %. Ganancias adicionales {an}: {base2} CAD. Idéntico al CPP2 pero gestionado por Retraite Québec.",
        ],
        "QC_AE" => [
            "Quebec workers pay a reduced EI rate under s. 69 of the Employment Insurance Act, because the QPIP covers parental benefits (maternity, paternity, parental, adoption).\n\nRate {an}: {ts} % employee + {tp} % employer (= employee × 1.4)\nvs. general scheme: differential of about 0.35 pp (employee)\n\nThis reduction reflects the federal → provincial transfer of responsibility for parental benefits, thanks to the 2005 Canada-Quebec agreement that created the QPIP.",
            "Arbeitnehmer in Québec zahlen gemäß s. 69 des Employment Insurance Act einen reduzierten EI-Satz, da das QPIP die Elternleistungen (Mutterschaft, Vaterschaft, Eltern, Adoption) übernimmt.\n\nSatz {an}: {ts} % Arbeitnehmer + {tp} % Arbeitgeber (= Arbeitnehmer × 1,4)\nggü. allgemeinem System: Differenz von etwa 0,35 PP (Arbeitnehmer)\n\nDiese Ermäßigung spiegelt die Übertragung der Zuständigkeit für Elternleistungen vom Bund auf die Provinz wider, dank des Abkommens Kanada-Québec von 2005, das das QPIP schuf.",
            "Werknemers in Québec betalen een verlaagd EI-tarief op grond van s. 69 van de Employment Insurance Act, omdat het QPIP de ouderschapsuitkeringen (moederschap, vaderschap, ouderschap, adoptie) dekt.\n\nTarief {an}: {ts} % werknemer + {tp} % werkgever (= werknemer × 1,4)\nt.o.v. algemeen stelsel: verschil van ongeveer 0,35 pp (werknemer)\n\nDeze verlaging weerspiegelt de overdracht van bevoegdheid voor ouderschapsuitkeringen van federaal → provinciaal, dankzij het akkoord Canada-Québec van 2005 dat het QPIP creëerde.",
            "I lavoratori del Québec pagano un'aliquota EI ridotta ai sensi dell'art. 69 dell'Employment Insurance Act, perché il QPIP copre le prestazioni parentali (maternità, paternità, parentale, adozione).\n\nAliquota {an}: {ts} % dipendente + {tp} % datore di lavoro (= dipendente × 1,4)\nvs. regime generale: differenziale di circa 0,35 pp (dipendente)\n\nQuesta riduzione riflette il trasferimento di responsabilità federale → provinciale per le prestazioni parentali, grazie all'accordo Canada-Québec del 2005 che ha creato il QPIP.",
            "Los trabajadores de Quebec pagan un tipo de EI reducido en virtud del art. 69 de la Employment Insurance Act, porque el QPIP cubre las prestaciones parentales (maternidad, paternidad, parental, adopción).\n\nTipo {an}: {ts} % trabajador + {tp} % empleador (= trabajador × 1,4)\nfrente al régimen general: diferencial de aproximadamente 0,35 pp (trabajador)\n\nEsta reducción refleja la transferencia de competencia federal → provincial para las prestaciones parentales, gracias al acuerdo Canadá-Quebec de 2005 que creó el QPIP.",
        ],
        "QC_RQAP" => [
            "The Quebec Parental Insurance Plan (QPIP) replaced the federal EI parental benefits for Quebecers as of 1 January 2006 (CQLR, c. A-29.011). It offers more generous terms than EI.\n\nQPIP ceiling {an}: {plaf} CAD/month ({plafa} CAD/yr)\nRate: {ts} % employee + {tp} % employer\n\nBenefits covered (basic plan):\n• Maternity: 18 weeks at 70 % of income\n• Paternity: 5 weeks at 70 %\n• Parental: 40 weeks (or 25 wks in the enhanced plan at 75 %)\n• Adoption: 37 weeks at 70 %\n\nThe contribution rate is lower than EI because parental benefits have a lower actuarial cost than regular benefits.",
            "Der Quebec Parental Insurance Plan (QPIP/RQAP) ersetzte ab dem 1. Januar 2006 die Elternleistungen der föderalen EI für die Québecer (CQLR, c. A-29.011). Er bietet großzügigere Bedingungen als die EI.\n\nQPIP-Höchstgrenze {an}: {plaf} CAD/Monat ({plafa} CAD/Jahr)\nSatz: {ts} % Arbeitnehmer + {tp} % Arbeitgeber\n\nAbgedeckte Leistungen (Basisplan):\n• Mutterschaft: 18 Wochen zu 70 % des Einkommens\n• Vaterschaft: 5 Wochen zu 70 %\n• Elternzeit: 40 Wochen (oder 25 Wochen im aufgestockten Plan zu 75 %)\n• Adoption: 37 Wochen zu 70 %\n\nDer Beitragssatz ist niedriger als bei der EI, da Elternleistungen geringere versicherungsmathematische Kosten haben als reguläre Leistungen.",
            "Het Quebec Parental Insurance Plan (QPIP/RQAP) verving vanaf 1 januari 2006 de federale EI-ouderschapsuitkeringen voor Québecers (CQLR, c. A-29.011). Het biedt gunstiger voorwaarden dan de EI.\n\nQPIP-plafond {an}: {plaf} CAD/maand ({plafa} CAD/jr)\nTarief: {ts} % werknemer + {tp} % werkgever\n\nGedekte uitkeringen (basisplan):\n• Moederschap: 18 weken tegen 70 % van het inkomen\n• Vaderschap: 5 weken tegen 70 %\n• Ouderschap: 40 weken (of 25 weken in het verhoogde plan tegen 75 %)\n• Adoptie: 37 weken tegen 70 %\n\nHet bijdragetarief is lager dan de EI omdat ouderschapsuitkeringen lagere actuariële kosten hebben dan reguliere uitkeringen.",
            "Il Quebec Parental Insurance Plan (QPIP/RQAP) ha sostituito le prestazioni parentali dell'EI federale per i québecchesi dal 1° gennaio 2006 (CQLR, c. A-29.011). Offre condizioni più generose dell'EI.\n\nMassimale QPIP {an}: {plaf} CAD/mese ({plafa} CAD/anno)\nAliquota: {ts} % dipendente + {tp} % datore di lavoro\n\nPrestazioni coperte (piano base):\n• Maternità: 18 settimane al 70 % del reddito\n• Paternità: 5 settimane al 70 %\n• Parentale: 40 settimane (o 25 sett. nel piano potenziato al 75 %)\n• Adozione: 37 settimane al 70 %\n\nL'aliquota contributiva è inferiore all'EI perché le prestazioni parentali hanno un costo attuariale minore rispetto a quelle ordinarie.",
            "El Quebec Parental Insurance Plan (QPIP/RQAP) sustituyó las prestaciones parentales del EI federal para los quebequeses desde el 1 de enero de 2006 (CQLR, c. A-29.011). Ofrece condiciones más generosas que el EI.\n\nTope QPIP {an}: {plaf} CAD/mes ({plafa} CAD/año)\nTipo: {ts} % trabajador + {tp} % empleador\n\nPrestaciones cubiertas (plan básico):\n• Maternidad: 18 semanas al 70 % de la renta\n• Paternidad: 5 semanas al 70 %\n• Parental: 40 semanas (o 25 sem. en el plan mejorado al 75 %)\n• Adopción: 37 semanas al 70 %\n\nEl tipo de cotización es inferior al EI porque las prestaciones parentales tienen un coste actuarial menor que las ordinarias.",
        ],
        "QC_FSS" => [
            "The Health Services Fund (HSF/FSS) is an employer contribution unique to Quebec, paid to Revenu Québec, funding the public health-insurance scheme (CQLR, c. R-5).\n\nDisplayed rate: {tp} % (indicative — mid-range payroll, services sector)\nActual rate depends on the company's total annual payroll:\n• Payroll ≤ 1,000,000 CAD: 1.65 %\n• Payroll 1,000,001–6,000,000 CAD: between 1.65 % and 4.26 % (progressive)\n• Payroll > 6,000,000 CAD (services): 4.26 %\n• Payroll > 6,000,000 CAD (primary/manufacturing sector): 1.25 %\n\nNo per-employee cap — base = full salary. Declared and paid via the RL-1 / TP-64.3.",
            "Der Gesundheitsdienstefonds (HSF/FSS) ist ein in Kanada nur in Québec erhobener Arbeitgeberbeitrag, gezahlt an Revenu Québec, der das öffentliche Krankenversicherungssystem finanziert (CQLR, c. R-5).\n\nAngezeigter Satz: {tp} % (Richtwert — mittlere Lohnsumme, Dienstleistungssektor)\nTatsächlicher Satz je nach jährlicher Gesamtlohnsumme des Unternehmens:\n• Lohnsumme ≤ 1 000 000 CAD: 1,65 %\n• Lohnsumme 1 000 001–6 000 000 CAD: zwischen 1,65 % und 4,26 % (progressiv)\n• Lohnsumme > 6 000 000 CAD (Dienstleistungen): 4,26 %\n• Lohnsumme > 6 000 000 CAD (Primär-/verarbeitender Sektor): 1,25 %\n\nKeine Obergrenze pro Arbeitnehmer — Bemessungsgrundlage = gesamtes Gehalt. Erklärt und gezahlt über den Beleg RL-1 / TP-64.3.",
            "Het Fonds voor gezondheidsdiensten (HSF/FSS) is een werkgeversbijdrage die in Canada uniek is voor Québec, betaald aan Revenu Québec, ter financiering van het openbare ziektekostenstelsel (CQLR, c. R-5).\n\nWeergegeven tarief: {tp} % (indicatief — gemiddelde loonsom, dienstensector)\nWerkelijk tarief afhankelijk van de totale jaarlijkse loonsom van de onderneming:\n• Loonsom ≤ 1.000.000 CAD: 1,65 %\n• Loonsom 1.000.001–6.000.000 CAD: tussen 1,65 % en 4,26 % (progressief)\n• Loonsom > 6.000.000 CAD (diensten): 4,26 %\n• Loonsom > 6.000.000 CAD (primaire/maakindustrie): 1,25 %\n\nGeen plafond per werknemer — grondslag = volledig salaris. Aangegeven en betaald via het RL-1 / TP-64.3.",
            "Il Fondo per i servizi sanitari (HSF/FSS) è un contributo datoriale esclusivo del Québec, versato a Revenu Québec, che finanzia il regime pubblico di assicurazione malattia (CQLR, c. R-5).\n\nAliquota visualizzata: {tp} % (indicativa — monte salari medio, settore servizi)\nAliquota reale secondo il monte salari annuo totale dell'impresa:\n• Monte salari ≤ 1 000 000 CAD: 1,65 %\n• Monte salari 1 000 001–6 000 000 CAD: tra 1,65 % e 4,26 % (progressiva)\n• Monte salari > 6 000 000 CAD (servizi): 4,26 %\n• Monte salari > 6 000 000 CAD (settore primario/manifatturiero): 1,25 %\n\nNessun massimale per dipendente — base = retribuzione totale. Dichiarato e pagato tramite il modulo RL-1 / TP-64.3.",
            "El Fondo de servicios de salud (HSF/FSS) es una cotización patronal exclusiva de Quebec, abonada a Revenu Québec, que financia el régimen público de seguro de enfermedad (CQLR, c. R-5).\n\nTipo mostrado: {tp} % (indicativo — masa salarial intermedia, sector servicios)\nTipo real según la masa salarial anual total de la empresa:\n• Masa ≤ 1 000 000 CAD: 1,65 %\n• Masa 1 000 001–6 000 000 CAD: entre 1,65 % y 4,26 % (progresivo)\n• Masa > 6 000 000 CAD (servicios): 4,26 %\n• Masa > 6 000 000 CAD (sector primario/manufacturero): 1,25 %\n\nSin tope por trabajador — base = salario íntegro. Declarado y pagado mediante el RL-1 / TP-64.3.",
        ],
        "QC_CNT" => [
            "Employer contribution of 0.06 % paid to the CNESST (Commission for standards, equity, health and safety at work), formerly the CNT (Labour Standards Commission). It funds labour-standards inspection, support for injured workers and rights promotion. Cap identical to the QPIP-MIE ({plafa} CAD/yr). Very low financial impact — often included in administrative costs.",
            "Arbeitgeberbeitrag von 0,06 % an die CNESST (Kommission für Normen, Gleichheit, Gesundheit und Sicherheit am Arbeitsplatz), ehemals CNT (Kommission für Arbeitsnormen). Finanziert die Kontrolle der Arbeitsnormen, die Unterstützung verletzter Arbeitnehmer und die Förderung von Rechten. Obergrenze identisch mit dem QPIP-MIE ({plafa} CAD/Jahr). Sehr geringe finanzielle Auswirkung — oft in den Verwaltungskosten enthalten.",
            "Werkgeversbijdrage van 0,06 % aan de CNESST (Commissie voor normen, gelijkheid, gezondheid en veiligheid op het werk), voorheen de CNT (Commissie voor arbeidsnormen). Financiert de inspectie van arbeidsnormen, steun aan gewonde werknemers en de bevordering van rechten. Plafond identiek aan het QPIP-MIE ({plafa} CAD/jr). Zeer lage financiële impact — vaak opgenomen in de administratiekosten.",
            "Contributo datoriale dello 0,06 % versato alla CNESST (Commissione per le norme, l'equità, la salute e la sicurezza sul lavoro), già CNT (Commissione delle norme del lavoro). Finanzia l'ispezione delle norme del lavoro, il sostegno ai lavoratori infortunati e la promozione dei diritti. Massimale identico al MIE-QPIP ({plafa} CAD/anno). Impatto finanziario molto basso — spesso incluso nei costi amministrativi.",
            "Cotización patronal del 0,06 % abonada a la CNESST (Comisión de normas, equidad, salud y seguridad en el trabajo), antes CNT (Comisión de normas laborales). Financia la inspección de las normas laborales, la ayuda a los trabajadores lesionados y la promoción de derechos. Tope idéntico al MIE-QPIP ({plafa} CAD/año). Impacto financiero muy bajo — a menudo incluido en los gastos de administración.",
        ],
        _ => return None,
    };
    Some(row[i])
}
