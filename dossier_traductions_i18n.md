# Xenna Paie — Dossier de traduction i18n (6 langues)

> Objectif : traduire **tous les pays et leurs détails** (libellés + explications des lignes
> de bulletin) dans les **6 langues** du menu langue, plus les **noms de pays**.
> Langues : `fr` (référence) · `en` · `de` · `nl` (Vlaams) · `it` · `es`.
>
> **Phase de documentation préalable** — à valider avant câblage. Aucun code modifié.
> Généré le 19/06/2026.
>
> Règles : les **placeholders `{x}`** des gabarits d'explication sont **identiques dans les 6
> langues** (substitués côté Rust). Les **références légales** (n° de loi, « Llei… », loi_ref)
> ne sont **pas** traduites. La France est déjà couverte (`i18n/cotisations.rs`) → non reprise.

---

## 1. Noms de pays et sous-régions (dictionnaire front)

| clé (fr) | en | de | nl | it | es |
|----------|----|----|----|----|----|
| France | France | Frankreich | Frankrijk | Francia | Francia |
| Fonction publique | Civil service | Öffentlicher Dienst | Overheid | Pubblico impiego | Función pública |
| Entreprise adaptée (AAP) | Adapted enterprise (AAP) | Inklusionsbetrieb (AAP) | Aangepast bedrijf (AAP) | Impresa adattata (AAP) | Empresa adaptada (AAP) |
| Alsace-Moselle | Alsace-Moselle | Elsass-Mosel | Alsace-Moselle | Alsazia-Mosella | Alsacia-Mosela |
| Allemagne | Germany | Deutschland | Duitsland | Germania | Alemania |
| Andorre | Andorra | Andorra | Andorra | Andorra | Andorra |
| Angleterre | England | England | Engeland | Inghilterra | Inglaterra |
| Autriche | Austria | Österreich | Oostenrijk | Austria | Austria |
| Belgique | Belgium | Belgien | België | Belgio | Bélgica |
| Flandres | Flanders | Flandern | Vlaanderen | Fiandre | Flandes |
| Wallonie | Wallonia | Wallonien | Wallonië | Vallonia | Valonia |
| Bruxelles | Brussels | Brüssel | Brussel | Bruxelles | Bruselas |
| Bulgarie | Bulgaria | Bulgarien | Bulgarije | Bulgaria | Bulgaria |
| Chypre | Cyprus | Zypern | Cyprus | Cipro | Chipre |
| Croatie | Croatia | Kroatien | Kroatië | Croazia | Croacia |
| Danemark | Denmark | Dänemark | Denemarken | Danimarca | Dinamarca |
| Espagne | Spain | Spanien | Spanje | Spagna | España |
| Estonie | Estonia | Estland | Estland | Estonia | Estonia |
| Finlande | Finland | Finnland | Finland | Finlandia | Finlandia |
| Grèce | Greece | Griechenland | Griekenland | Grecia | Grecia |
| Hongrie | Hungary | Ungarn | Hongarije | Ungheria | Hungría |
| Irlande | Ireland | Irland | Ierland | Irlanda | Irlanda |
| Italie | Italy | Italien | Italië | Italia | Italia |
| Lettonie | Latvia | Lettland | Letland | Lettonia | Letonia |
| Lituanie | Lithuania | Litauen | Litouwen | Lituania | Lituania |
| Luxembourg | Luxembourg | Luxemburg | Luxemburg | Lussemburgo | Luxemburgo |
| Malte | Malta | Malta | Malta | Malta | Malta |
| Monaco | Monaco | Monaco | Monaco | Monaco | Mónaco |
| Pays-Bas | Netherlands | Niederlande | Nederland | Paesi Bassi | Países Bajos |
| Pologne | Poland | Polen | Polen | Polonia | Polonia |
| Portugal | Portugal | Portugal | Portugal | Portogallo | Portugal |
| Roumanie | Romania | Rumänien | Roemenië | Romania | Rumanía |
| Slovaquie | Slovakia | Slowakei | Slowakije | Slovacchia | Eslovaquia |
| Slovénie | Slovenia | Slowenien | Slovenië | Slovenia | Eslovenia |
| Suède | Sweden | Schweden | Zweden | Svezia | Suecia |
| Suisse | Switzerland | Schweiz | Zwitserland | Svizzera | Suiza |
| Tchéquie | Czechia | Tschechien | Tsjechië | Cechia | Chequia |
| Canada | Canada | Kanada | Canada | Canada | Canadá |
| Québec | Quebec | Québec | Quebec | Québec | Quebec |
| Japon | Japan | Japan | Japan | Giappone | Japón |
| Chine | China | China | China | Cina | China |
| Corée du Sud | South Korea | Südkorea | Zuid-Korea | Corea del Sud | Corea del Sur |
| Australie | Australia | Australien | Australië | Australia | Australia |
| Nouvelle-Zélande | New Zealand | Neuseeland | Nieuw-Zeeland | Nuova Zelanda | Nueva Zelanda |

---

## 2. Libellés de catégories (champ `categorie`)

> Ces libellés servent d'en-têtes de regroupement des lignes. Certains pays utilisent
> déjà une langue locale (it). Traductions proposées (FR de référence) :

| fr | en | de | nl | it | es |
|----|----|----|----|----|----|
| Sécurité sociale | Social security | Sozialversicherung | Sociale zekerheid | Sicurezza sociale | Seguridad social |
| Assurance maladie | Health insurance | Krankenversicherung | Ziektekostenverzekering | Assicurazione malattia | Seguro de enfermedad |
| Assurance pension | Pension insurance | Rentenversicherung | Pensioenverzekering | Assicurazione pensione | Seguro de pensión |
| Assurance chômage | Unemployment insurance | Arbeitslosenversicherung | Werkloosheidsverzekering | Assicurazione disoccupazione | Seguro de desempleo |
| Assurance dépendance | Long-term care insurance | Pflegeversicherung | Zorgverzekering (langdurig) | Assicurazione dipendenza | Seguro de dependencia |
| Assurance accidents | Accident insurance | Unfallversicherung | Ongevallenverzekering | Assicurazione infortuni | Seguro de accidentes |
| Accidents du travail | Work accidents | Arbeitsunfälle | Arbeidsongevallen | Infortuni sul lavoro | Accidentes laborales |
| Retraite | Pension | Rente | Pensioen | Pensione | Jubilación |
| Retraite complémentaire | Supplementary pension | Zusatzrente | Aanvullend pensioen | Pensione complementare | Pensión complementaria |
| Prévoyance | Occupational benefits | Vorsorge | Voorzorg | Previdenza | Previsión |
| Prévoyance (LPP) | Occupational pension (LPP) | Berufliche Vorsorge (BVG) | Beroepsvoorzorg (LPP) | Previdenza professionale (LPP) | Previsión profesional (LPP) |
| Prévoyance maladie | Health provision | Krankenvorsorge | Ziektevoorzorg | Previdenza malattia | Previsión de enfermedad |
| 1er pilier | First pillar | Erste Säule | Eerste pijler | Primo pilastro | Primer pilar |
| CSG/CRDS | CSG/CRDS | CSG/CRDS | CSG/CRDS | CSG/CRDS | CSG/CRDS |
| Impôt sur le revenu | Income tax | Einkommensteuer | Inkomstenbelasting | Imposta sul reddito | Impuesto sobre la renta |
| Impôt à la source | Withholding tax | Quellensteuer | Bronbelasting | Imposta alla fonte | Retención en origen |
| Impôt fédéral | Federal tax | Bundessteuer | Federale belasting | Imposta federale | Impuesto federal |
| Impôt provincial | Provincial tax | Provinzsteuer | Provinciale belasting | Imposta provinciale | Impuesto provincial |
| Taxe locale | Local tax | Kommunalsteuer | Lokale belasting | Imposta locale | Impuesto local |
| Formation professionnelle | Vocational training | Berufsbildung | Beroepsopleiding | Formazione professionale | Formación profesional |
| Garantie salariale | Wage guarantee | Lohngarantie | Loongarantie | Garanzia salariale | Garantía salarial |
| Garantie emploi | Employment guarantee | Beschäftigungsgarantie | Werkgelegenheidsgarantie | Garanzia occupazione | Garantía de empleo |
| Aide à l'emploi | Employment support | Beschäftigungshilfe | Werkgelegenheidssteun | Sostegno all'occupazione | Ayuda al empleo |
| Allègement | Relief | Entlastung | Verlichting | Sgravio | Reducción |
| Réduction patronale | Employer relief | Arbeitgeberentlastung | Werkgeversvermindering | Sgravio datore di lavoro | Reducción patronal |
| Réduction salariale | Employee relief | Arbeitnehmerentlastung | Werknemersvermindering | Sgravio dipendente | Reducción salarial |
| Mutualité des employeurs | Employers' mutual fund | Arbeitgeber-Ausgleichskasse | Werkgeversfonds | Mutua datori di lavoro | Mutualidad de empleadores |
| Parentalité Québec | Quebec parental insurance | Elternversicherung Québec | Ouderschapsverzekering Québec | Assicurazione parentale Québec | Seguro parental de Quebec |
| Santé Québec | Quebec health | Gesundheit Québec | Gezondheid Québec | Sanità Québec | Salud Quebec |
| Réserve retraite | Pension reserve | Rentenrücklage | Pensioenreserve | Riserva pensione | Reserva de pensión |
| Bonus IRPEF | IRPEF bonus | IRPEF-Bonus | IRPEF-bonus | Bonus IRPEF | Bono IRPEF |
| Autres | Other | Sonstige | Overige | Altro | Otros |
| Information | Information | Information | Informatie | Informazione | Información |

---

## 3. Détail par pays (libellés + gabarits d'explication)

> Format par ligne de cotisation : `#### <CODE>` puis `libelle` (6 langues) et `explication`
> (gabarit, un bloc par langue, placeholders `{x}` préservés).

### 🇦🇩 Andorre

#### AD_CASS
- **libelle** — fr: `CASS — Sécurité sociale` · en: `CASS — Social security` · de: `CASS — Sozialversicherung` · nl: `CASS — Sociale zekerheid` · it: `CASS — Sicurezza sociale` · es: `CASS — Seguridad social`
- **explication** (placeholders `{ts} {tp} {ms}`) :
  - fr: `CASS — sécurité sociale (branche générale + retraite).\nSalarié {ts} % / employeur {tp} %. Salarié : {ms} €.\n\nBase légale : Llei 17/2008.`
  - en: `CASS — social security (general branch + pension).\nEmployee {ts} % / employer {tp} %. Employee: {ms} €.\n\nLegal basis: Llei 17/2008.`
  - de: `CASS — Sozialversicherung (allgemeiner Zweig + Rente).\nArbeitnehmer {ts} % / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.\n\nRechtsgrundlage: Llei 17/2008.`
  - nl: `CASS — sociale zekerheid (algemene tak + pensioen).\nWerknemer {ts} % / werkgever {tp} %. Werknemer: {ms} €.\n\nWettelijke basis: Llei 17/2008.`
  - it: `CASS — sicurezza sociale (ramo generale + pensione).\nDipendente {ts} % / datore di lavoro {tp} %. Dipendente: {ms} €.\n\nBase giuridica: Llei 17/2008.`
  - es: `CASS — seguridad social (rama general + pensión).\nTrabajador {ts} % / empleador {tp} %. Trabajador: {ms} €.\n\nBase legal: Llei 17/2008.`

#### AD_IRPF
- **libelle** — fr: `IRPF — Impôt sur le revenu` · en: `IRPF — Income tax` · de: `IRPF — Einkommensteuer` · nl: `IRPF — Inkomstenbelasting` · it: `IRPF — Imposta sul reddito` · es: `IRPF — Impuesto sobre la renta`
- **explication** (placeholders `{ra} {ia} {im}`) :
  - fr: `IRPF — impôt sur le revenu (annualisé).\n\nRevenu annuel : {ra} €\n• 0 % jusqu'à 24 000 €\n• 5 % de 24 001 à 40 000 €\n• 10 % au-delà de 40 000 €\n= {ia} €/an / 12 = {im} €/mois.\n\nBase légale : Llei 5/2014 (IRPF).`
  - en: `IRPF — income tax (annualised).\n\nAnnual income: {ra} €\n• 0 % up to 24,000 €\n• 5 % from 24,001 to 40,000 €\n• 10 % above 40,000 €\n= {ia} €/year / 12 = {im} €/month.\n\nLegal basis: Llei 5/2014 (IRPF).`
  - de: `IRPF — Einkommensteuer (auf Jahresbasis).\n\nJahreseinkommen: {ra} €\n• 0 % bis 24.000 €\n• 5 % von 24.001 bis 40.000 €\n• 10 % über 40.000 €\n= {ia} €/Jahr / 12 = {im} €/Monat.\n\nRechtsgrundlage: Llei 5/2014 (IRPF).`
  - nl: `IRPF — inkomstenbelasting (op jaarbasis).\n\nJaarinkomen: {ra} €\n• 0 % tot 24.000 €\n• 5 % van 24.001 tot 40.000 €\n• 10 % boven 40.000 €\n= {ia} €/jaar / 12 = {im} €/maand.\n\nWettelijke basis: Llei 5/2014 (IRPF).`
  - it: `IRPF — imposta sul reddito (annualizzata).\n\nReddito annuo: {ra} €\n• 0 % fino a 24.000 €\n• 5 % da 24.001 a 40.000 €\n• 10 % oltre 40.000 €\n= {ia} €/anno / 12 = {im} €/mese.\n\nBase giuridica: Llei 5/2014 (IRPF).`
  - es: `IRPF — impuesto sobre la renta (anualizado).\n\nRenta anual: {ra} €\n• 0 % hasta 24.000 €\n• 5 % de 24.001 a 40.000 €\n• 10 % por encima de 40.000 €\n= {ia} €/año / 12 = {im} €/mes.\n\nBase legal: Llei 5/2014 (IRPF).`

### 🇦🇹 Autriche

#### AT_SV
- **libelle** — fr: `Sozialversicherung — Cotisations sociales` · en: `Sozialversicherung — Social contributions` · de: `Sozialversicherung — Sozialbeiträge` · nl: `Sozialversicherung — Sociale bijdragen` · it: `Sozialversicherung — Contributi sociali` · es: `Sozialversicherung — Cotizaciones sociales`
- **explication** (placeholders `{ts} {tp} {ms}`) :
  - fr: `Sozialversicherung — salarié {ts} % / employeur {tp} % (retraite PV, maladie KV, chômage ALV, AK, WBF). Assiette plafonnée à 6 450 €/mois (Höchstbeitragsgrundlage). Salarié : {ms} €.`
  - en: `Sozialversicherung — employee {ts} % / employer {tp} % (pension PV, health KV, unemployment ALV, AK, WBF). Base capped at 6,450 €/month (Höchstbeitragsgrundlage). Employee: {ms} €.`
  - de: `Sozialversicherung — Arbeitnehmer {ts} % / Arbeitgeber {tp} % (Pension PV, Kranken KV, Arbeitslosen ALV, AK, WBF). Bemessungsgrundlage gedeckelt auf 6.450 €/Monat (Höchstbeitragsgrundlage). Arbeitnehmer: {ms} €.`
  - nl: `Sozialversicherung — werknemer {ts} % / werkgever {tp} % (pensioen PV, ziekte KV, werkloosheid ALV, AK, WBF). Grondslag begrensd op 6.450 €/maand (Höchstbeitragsgrundlage). Werknemer: {ms} €.`
  - it: `Sozialversicherung — dipendente {ts} % / datore di lavoro {tp} % (pensione PV, malattia KV, disoccupazione ALV, AK, WBF). Base limitata a 6.450 €/mese (Höchstbeitragsgrundlage). Dipendente: {ms} €.`
  - es: `Sozialversicherung — trabajador {ts} % / empleador {tp} % (pensión PV, enfermedad KV, desempleo ALV, AK, WBF). Base limitada a 6.450 €/mes (Höchstbeitragsgrundlage). Trabajador: {ms} €.`

#### AT_LOHNSTEUER
- **libelle** — fr: `Lohnsteuer — Impôt sur le revenu` · en: `Lohnsteuer — Income tax` · de: `Lohnsteuer — Einkommensteuer` · nl: `Lohnsteuer — Inkomstenbelasting` · it: `Lohnsteuer — Imposta sul reddito` · es: `Lohnsteuer — Impuesto sobre la renta`
- **explication** (placeholders `{b} {im}`) :
  - fr: `Impôt sur le revenu 2025 (annualisé).\n\nBase = (brut − SV salarié) × 12 = {b} €\nBarème 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(seuils 13 308 / 21 617 / 35 836 / 69 166 / 103 072 / 1 000 000 €)\n→ {im} €/mois.\n\nNote : 13ᵉ/14ᵉ mois (Sonderzahlungen) et crédits non modélisés (net prudent).\nSource : BMF.`
  - en: `Income tax 2025 (annualised).\n\nBase = (gross − employee SV) × 12 = {b} €\nScale 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(thresholds 13,308 / 21,617 / 35,836 / 69,166 / 103,072 / 1,000,000 €)\n→ {im} €/month.\n\nNote: 13th/14th salary (Sonderzahlungen) and credits not modelled (conservative net).\nSource: BMF.`
  - de: `Einkommensteuer 2025 (auf Jahresbasis).\n\nBemessung = (brutto − AN-SV) × 12 = {b} €\nTarif 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(Grenzen 13.308 / 21.617 / 35.836 / 69.166 / 103.072 / 1.000.000 €)\n→ {im} €/Monat.\n\nHinweis: 13./14. Gehalt (Sonderzahlungen) und Absetzbeträge nicht modelliert (vorsichtiger Nettowert).\nQuelle: BMF.`
  - nl: `Inkomstenbelasting 2025 (op jaarbasis).\n\nGrondslag = (bruto − werknemers-SV) × 12 = {b} €\nSchaal 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(drempels 13.308 / 21.617 / 35.836 / 69.166 / 103.072 / 1.000.000 €)\n→ {im} €/maand.\n\nNoot: 13e/14e maand (Sonderzahlungen) en kortingen niet gemodelleerd (voorzichtig netto).\nBron: BMF.`
  - it: `Imposta sul reddito 2025 (annualizzata).\n\nBase = (lordo − SV dipendente) × 12 = {b} €\nScala 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(soglie 13.308 / 21.617 / 35.836 / 69.166 / 103.072 / 1.000.000 €)\n→ {im} €/mese.\n\nNota: 13ª/14ª mensilità (Sonderzahlungen) e detrazioni non modellate (netto prudente).\nFonte: BMF.`
  - es: `Impuesto sobre la renta 2025 (anualizado).\n\nBase = (bruto − SV trabajador) × 12 = {b} €\nEscala 0 / 20 / 30 / 40 / 48 / 50 / 55 %\n(umbrales 13.308 / 21.617 / 35.836 / 69.166 / 103.072 / 1.000.000 €)\n→ {im} €/mes.\n\nNota: pagas 13ª/14ª (Sonderzahlungen) y deducciones no modeladas (neto prudente).\nFuente: BMF.`

### 🇧🇬 Bulgarie

#### BG_OSIG
- **libelle** — fr: `Осигуровки — Cotisations sociales` · en: `Осигуровки — Social contributions` · de: `Осигуровки — Sozialbeiträge` · nl: `Осигуровки — Sociale bijdragen` · it: `Осигуровки — Contributi sociali` · es: `Осигуровки — Cotizaciones sociales`
- **explication** (placeholders `{ts} {tp} {ms}`) :
  - fr: `Cotisations sociales — salarié {ts} % / employeur {tp} % (retraite, maladie NZOK, 2ᵉ pilier). Assiette plafonnée à 3 750 BGN/mois. Salarié : {ms} BGN.`
  - en: `Social contributions — employee {ts} % / employer {tp} % (pension, health NZOK, 2nd pillar). Base capped at 3,750 BGN/month. Employee: {ms} BGN.`
  - de: `Sozialbeiträge — Arbeitnehmer {ts} % / Arbeitgeber {tp} % (Rente, Kranken NZOK, 2. Säule). Bemessungsgrundlage gedeckelt auf 3.750 BGN/Monat. Arbeitnehmer: {ms} BGN.`
  - nl: `Sociale bijdragen — werknemer {ts} % / werkgever {tp} % (pensioen, ziekte NZOK, 2e pijler). Grondslag begrensd op 3.750 BGN/maand. Werknemer: {ms} BGN.`
  - it: `Contributi sociali — dipendente {ts} % / datore di lavoro {tp} % (pensione, malattia NZOK, 2º pilastro). Base limitata a 3.750 BGN/mese. Dipendente: {ms} BGN.`
  - es: `Cotizaciones sociales — trabajador {ts} % / empleador {tp} % (pensión, enfermedad NZOK, 2º pilar). Base limitada a 3.750 BGN/mes. Trabajador: {ms} BGN.`

#### BG_DANAK
- **libelle** — fr: `Данък върху доходите — Impôt sur le revenu (10 %)` · en: `Данък върху доходите — Income tax (10 %)` · de: `Данък върху доходите — Einkommensteuer (10 %)` · nl: `Данък върху доходите — Inkomstenbelasting (10 %)` · it: `Данък върху доходите — Imposta sul reddito (10 %)` · es: `Данък върху доходите — Impuesto sobre la renta (10 %)`
- **explication** (placeholders `{b} {im}`) :
  - fr: `Impôt sur le revenu 2025 : 10 % proportionnel.\n\nBase = brut − cotisations salariales = {b} BGN → {im} BGN/mois.\n\nSource : НАП (NRA).`
  - en: `Income tax 2025: flat 10 %.\n\nBase = gross − employee contributions = {b} BGN → {im} BGN/month.\n\nSource: НАП (NRA).`
  - de: `Einkommensteuer 2025: pauschal 10 %.\n\nBemessung = brutto − AN-Beiträge = {b} BGN → {im} BGN/Monat.\n\nQuelle: НАП (NRA).`
  - nl: `Inkomstenbelasting 2025: vlak 10 %.\n\nGrondslag = bruto − werknemersbijdragen = {b} BGN → {im} BGN/maand.\n\nBron: НАП (NRA).`
  - it: `Imposta sul reddito 2025: proporzionale 10 %.\n\nBase = lordo − contributi dipendente = {b} BGN → {im} BGN/mese.\n\nFonte: НАП (NRA).`
  - es: `Impuesto sobre la renta 2025: plano 10 %.\n\nBase = bruto − cotizaciones del trabajador = {b} BGN → {im} BGN/mes.\n\nFuente: НАП (NRA).`

### 🇨🇾 Chypre

#### CY_SI
- **libelle** — fr: `Κοινωνικές Ασφαλίσεις — Assurance sociale` · en: `Κοινωνικές Ασφαλίσεις — Social insurance` · de: `Κοινωνικές Ασφαλίσεις — Sozialversicherung` · nl: `Κοινωνικές Ασφαλίσεις — Sociale verzekering` · it: `Κοινωνικές Ασφαλίσεις — Assicurazione sociale` · es: `Κοινωνικές Ασφαλίσεις — Seguro social`
- **explication** (placeholders `{ts} {tp}`) :
  - fr: `Assurance sociale — salarié {ts} % / employeur {tp} %. Assiette plafonnée à 5 551 €/mois.`
  - en: `Social insurance — employee {ts} % / employer {tp} %. Base capped at 5,551 €/month.`
  - de: `Sozialversicherung — Arbeitnehmer {ts} % / Arbeitgeber {tp} %. Bemessungsgrundlage gedeckelt auf 5.551 €/Monat.`
  - nl: `Sociale verzekering — werknemer {ts} % / werkgever {tp} %. Grondslag begrensd op 5.551 €/maand.`
  - it: `Assicurazione sociale — dipendente {ts} % / datore di lavoro {tp} %. Base limitata a 5.551 €/mese.`
  - es: `Seguro social — trabajador {ts} % / empleador {tp} %. Base limitada a 5.551 €/mes.`

#### CY_GESY
- **libelle** — fr: `ΓΕΣΥ — Système national de santé` · en: `ΓΕΣΥ — National health system` · de: `ΓΕΣΥ — Nationales Gesundheitssystem` · nl: `ΓΕΣΥ — Nationaal gezondheidssysteem` · it: `ΓΕΣΥ — Sistema sanitario nazionale` · es: `ΓΕΣΥ — Sistema nacional de salud`
- **explication** (placeholders `{ts} {tp}`) :
  - fr: `GESY (santé) — salarié {ts} % / employeur {tp} %.`
  - en: `GESY (health) — employee {ts} % / employer {tp} %.`
  - de: `GESY (Gesundheit) — Arbeitnehmer {ts} % / Arbeitgeber {tp} %.`
  - nl: `GESY (gezondheid) — werknemer {ts} % / werkgever {tp} %.`
  - it: `GESY (sanità) — dipendente {ts} % / datore di lavoro {tp} %.`
  - es: `GESY (salud) — trabajador {ts} % / empleador {tp} %.`

#### CY_FOROS
- **libelle** — fr: `Φόρος εισοδήματος — Impôt sur le revenu` · en: `Φόρος εισοδήματος — Income tax` · de: `Φόρος εισοδήματος — Einkommensteuer` · nl: `Φόρος εισοδήματος — Inkomstenbelasting` · it: `Φόρος εισοδήματος — Imposta sul reddito` · es: `Φόρος εισοδήματος — Impuesto sobre la renta`
- **explication** (placeholders `{b} {im}`) :
  - fr: `Impôt sur le revenu 2025 (annualisé).\n\nBase = (brut − cotisations) × 12 = {b} €\nBarème 0 / 20 / 25 / 30 / 35 % (seuils 19 500 / 28 000 / 36 300 / 60 000 €)\n→ {im} €/mois.\n\nSource : Τμήμα Φορολογίας.`
  - en: `Income tax 2025 (annualised).\n\nBase = (gross − contributions) × 12 = {b} €\nScale 0 / 20 / 25 / 30 / 35 % (thresholds 19,500 / 28,000 / 36,300 / 60,000 €)\n→ {im} €/month.\n\nSource: Τμήμα Φορολογίας.`
  - de: `Einkommensteuer 2025 (auf Jahresbasis).\n\nBemessung = (brutto − Beiträge) × 12 = {b} €\nTarif 0 / 20 / 25 / 30 / 35 % (Grenzen 19.500 / 28.000 / 36.300 / 60.000 €)\n→ {im} €/Monat.\n\nQuelle: Τμήμα Φορολογίας.`
  - nl: `Inkomstenbelasting 2025 (op jaarbasis).\n\nGrondslag = (bruto − bijdragen) × 12 = {b} €\nSchaal 0 / 20 / 25 / 30 / 35 % (drempels 19.500 / 28.000 / 36.300 / 60.000 €)\n→ {im} €/maand.\n\nBron: Τμήμα Φορολογίας.`
  - it: `Imposta sul reddito 2025 (annualizzata).\n\nBase = (lordo − contributi) × 12 = {b} €\nScala 0 / 20 / 25 / 30 / 35 % (soglie 19.500 / 28.000 / 36.300 / 60.000 €)\n→ {im} €/mese.\n\nFonte: Τμήμα Φορολογίας.`
  - es: `Impuesto sobre la renta 2025 (anualizado).\n\nBase = (bruto − cotizaciones) × 12 = {b} €\nEscala 0 / 20 / 25 / 30 / 35 % (umbrales 19.500 / 28.000 / 36.300 / 60.000 €)\n→ {im} €/mes.\n\nFuente: Τμήμα Φορολογίας.`

### 🇨🇿 Tchéquie

> Note câblage : `CZ_SOCIAL` et `CZ_ZDRAVOTNI` partagent le gabarit générique `ligne_cot`
> (placeholder `{libelle}` = le libellé déjà traduit + `{ts} {tp}`).

#### CZ_SOCIAL
- **libelle** — fr: `Sociální pojištění — Sécurité sociale` · en: `Sociální pojištění — Social security` · de: `Sociální pojištění — Sozialversicherung` · nl: `Sociální pojištění — Sociale zekerheid` · it: `Sociální pojištění — Sicurezza sociale` · es: `Sociální pojištění — Seguridad social`

#### CZ_ZDRAVOTNI
- **libelle** — fr: `Zdravotní pojištění — Assurance maladie` · en: `Zdravotní pojištění — Health insurance` · de: `Zdravotní pojištění — Krankenversicherung` · nl: `Zdravotní pojištění — Ziektekostenverzekering` · it: `Zdravotní pojištění — Assicurazione malattia` · es: `Zdravotní pojištění — Seguro de enfermedad`

#### CZ (gabarit générique `ligne_cot`) — explication
- **explication** (placeholders `{libelle} {ts} {tp}`) :
  - fr: `{libelle}. Salarié {ts} % / employeur {tp} %.`
  - en: `{libelle}. Employee {ts} % / employer {tp} %.`
  - de: `{libelle}. Arbeitnehmer {ts} % / Arbeitgeber {tp} %.`
  - nl: `{libelle}. Werknemer {ts} % / werkgever {tp} %.`
  - it: `{libelle}. Dipendente {ts} % / datore di lavoro {tp} %.`
  - es: `{libelle}. Trabajador {ts} % / empleador {tp} %.`

#### CZ_DAN
- **libelle** — fr: `Daň z příjmů — Impôt sur le revenu` · en: `Daň z příjmů — Income tax` · de: `Daň z příjmů — Einkommensteuer` · nl: `Daň z příjmů — Inkomstenbelasting` · it: `Daň z příjmů — Imposta sul reddito` · es: `Daň z příjmů — Impuesto sobre la renta`
- **explication** (placeholders `{ib} {im}`) :
  - fr: `Impôt sur le revenu 2025.\n\n15 % jusqu'à 139 671 CZK/mois, 23 % au-delà = {ib} CZK\n− sleva na poplatníka 2 570 CZK = {im} CZK/mois.\n\nSource : Finanční správa.`
  - en: `Income tax 2025.\n\n15 % up to 139,671 CZK/month, 23 % above = {ib} CZK\n− sleva na poplatníka 2,570 CZK = {im} CZK/month.\n\nSource: Finanční správa.`
  - de: `Einkommensteuer 2025.\n\n15 % bis 139.671 CZK/Monat, 23 % darüber = {ib} CZK\n− sleva na poplatníka 2.570 CZK = {im} CZK/Monat.\n\nQuelle: Finanční správa.`
  - nl: `Inkomstenbelasting 2025.\n\n15 % tot 139.671 CZK/maand, 23 % daarboven = {ib} CZK\n− sleva na poplatníka 2.570 CZK = {im} CZK/maand.\n\nBron: Finanční správa.`
  - it: `Imposta sul reddito 2025.\n\n15 % fino a 139.671 CZK/mese, 23 % oltre = {ib} CZK\n− sleva na poplatníka 2.570 CZK = {im} CZK/mese.\n\nFonte: Finanční správa.`
  - es: `Impuesto sobre la renta 2025.\n\n15 % hasta 139.671 CZK/mes, 23 % por encima = {ib} CZK\n− sleva na poplatníka 2.570 CZK = {im} CZK/mes.\n\nFuente: Finanční správa.`

### 🇩🇰 Danemark

#### DK_AM
- **libelle** — fr: `AM-bidrag — Contribution marché du travail` · en: `AM-bidrag — Labour market contribution` · de: `AM-bidrag — Arbeitsmarktbeitrag` · nl: `AM-bidrag — Arbeidsmarktbijdrage` · it: `AM-bidrag — Contributo mercato del lavoro` · es: `AM-bidrag — Contribución al mercado laboral`
- **explication** (placeholders `{am}`) :
  - fr: `AM-bidrag — 8 % du salaire brut, prélevé avant l'impôt.\nMontant : {am} DKK.\n\nBase légale : Arbejdsmarkedsbidragsloven.`
  - en: `AM-bidrag — 8 % of gross salary, deducted before tax.\nAmount: {am} DKK.\n\nLegal basis: Arbejdsmarkedsbidragsloven.`
  - de: `AM-bidrag — 8 % des Bruttolohns, vor Steuer einbehalten.\nBetrag: {am} DKK.\n\nRechtsgrundlage: Arbejdsmarkedsbidragsloven.`
  - nl: `AM-bidrag — 8 % van het brutoloon, ingehouden vóór belasting.\nBedrag: {am} DKK.\n\nWettelijke basis: Arbejdsmarkedsbidragsloven.`
  - it: `AM-bidrag — 8 % della retribuzione lorda, trattenuto prima delle imposte.\nImporto: {am} DKK.\n\nBase giuridica: Arbejdsmarkedsbidragsloven.`
  - es: `AM-bidrag — 8 % del salario bruto, retenido antes del impuesto.\nImporte: {am} DKK.\n\nBase legal: Arbejdsmarkedsbidragsloven.`

#### DK_ATP
- **libelle** — fr: `ATP — Pension complémentaire` · en: `ATP — Supplementary pension` · de: `ATP — Zusatzrente` · nl: `ATP — Aanvullend pensioen` · it: `ATP — Pensione complementare` · es: `ATP — Pensión complementaria`
- **explication** (placeholders `{a}`) :
  - fr: `ATP — pension complémentaire du marché du travail (forfait).\nTemps plein 2025 : {a} DKK/mois salarié (2/3 employeur).\n\nBase légale : ATP-loven.`
  - en: `ATP — labour market supplementary pension (flat rate).\nFull-time 2025: {a} DKK/month employee (2/3 employer).\n\nLegal basis: ATP-loven.`
  - de: `ATP — Arbeitsmarkt-Zusatzrente (Pauschale).\nVollzeit 2025: {a} DKK/Monat Arbeitnehmer (2/3 Arbeitgeber).\n\nRechtsgrundlage: ATP-loven.`
  - nl: `ATP — aanvullend arbeidsmarktpensioen (forfait).\nVoltijd 2025: {a} DKK/maand werknemer (2/3 werkgever).\n\nWettelijke basis: ATP-loven.`
  - it: `ATP — pensione complementare del mercato del lavoro (forfait).\nTempo pieno 2025: {a} DKK/mese dipendente (2/3 datore di lavoro).\n\nBase giuridica: ATP-loven.`
  - es: `ATP — pensión complementaria del mercado laboral (tanto alzado).\nTiempo completo 2025: {a} DKK/mes trabajador (2/3 empleador).\n\nBase legal: ATP-loven.`

#### DK_INDKOMSTSKAT
- **libelle** — fr: `Indkomstskat — Impôt sur le revenu` · en: `Indkomstskat — Income tax` · de: `Indkomstskat — Einkommensteuer` · nl: `Indkomstskat — Inkomstenbelasting` · it: `Indkomstskat — Imposta sul reddito` · es: `Indkomstskat — Impuesto sobre la renta`
- **explication** (placeholders `{ts} {tx} {ib} {tk} {im}`) :
  - fr: `Impôt sur le revenu — bundskat 12,01 % + kommuneskat moyen 25,1 % (= 37,11 %)\nsur le revenu après AM-bidrag, ATP et personfradrag (4 300 DKK/mois).\n+ topskat 15 % au-delà de {ts} DKK/mois (revenu après AM).\nBase imposable : {tx} DKK → {ib} DKK ; topskat {tk} DKK.\n= {im} DKK/mois.\n\nBase légale : Personskatteloven (2025). Kommuneskat = moyenne nationale.`
  - en: `Income tax — bundskat 12.01 % + average kommuneskat 25.1 % (= 37.11 %)\non income after AM-bidrag, ATP and personfradrag (4,300 DKK/month).\n+ topskat 15 % above {ts} DKK/month (income after AM).\nTaxable base: {tx} DKK → {ib} DKK; topskat {tk} DKK.\n= {im} DKK/month.\n\nLegal basis: Personskatteloven (2025). Kommuneskat = national average.`
  - de: `Einkommensteuer — bundskat 12,01 % + durchschnittliche kommuneskat 25,1 % (= 37,11 %)\nauf das Einkommen nach AM-bidrag, ATP und personfradrag (4.300 DKK/Monat).\n+ topskat 15 % über {ts} DKK/Monat (Einkommen nach AM).\nBemessungsgrundlage: {tx} DKK → {ib} DKK; topskat {tk} DKK.\n= {im} DKK/Monat.\n\nRechtsgrundlage: Personskatteloven (2025). Kommuneskat = Landesdurchschnitt.`
  - nl: `Inkomstenbelasting — bundskat 12,01 % + gemiddelde kommuneskat 25,1 % (= 37,11 %)\nop het inkomen na AM-bidrag, ATP en personfradrag (4.300 DKK/maand).\n+ topskat 15 % boven {ts} DKK/maand (inkomen na AM).\nBelastbare grondslag: {tx} DKK → {ib} DKK; topskat {tk} DKK.\n= {im} DKK/maand.\n\nWettelijke basis: Personskatteloven (2025). Kommuneskat = landelijk gemiddelde.`
  - it: `Imposta sul reddito — bundskat 12,01 % + kommuneskat media 25,1 % (= 37,11 %)\nsul reddito dopo AM-bidrag, ATP e personfradrag (4.300 DKK/mese).\n+ topskat 15 % oltre {ts} DKK/mese (reddito dopo AM).\nBase imponibile: {tx} DKK → {ib} DKK; topskat {tk} DKK.\n= {im} DKK/mese.\n\nBase giuridica: Personskatteloven (2025). Kommuneskat = media nazionale.`
  - es: `Impuesto sobre la renta — bundskat 12,01 % + kommuneskat media 25,1 % (= 37,11 %)\nsobre la renta tras AM-bidrag, ATP y personfradrag (4.300 DKK/mes).\n+ topskat 15 % por encima de {ts} DKK/mes (renta tras AM).\nBase imponible: {tx} DKK → {ib} DKK; topskat {tk} DKK.\n= {im} DKK/mes.\n\nBase legal: Personskatteloven (2025). Kommuneskat = media nacional.`

### 🇪🇪 Estonie

> Gabarit générique `ligne_cot` (placeholders `{libelle} {ts} {tp}`) — même phrase que Tchéquie :
> fr `{libelle}. Salarié {ts} % / employeur {tp} %.` · en `{libelle}. Employee {ts} % / employer {tp} %.` · de `{libelle}. Arbeitnehmer {ts} % / Arbeitgeber {tp} %.` · nl `{libelle}. Werknemer {ts} % / werkgever {tp} %.` · it `{libelle}. Dipendente {ts} % / datore di lavoro {tp} %.` · es `{libelle}. Trabajador {ts} % / empleador {tp} %.`

#### EE_TOOTUS
- **libelle** — fr: `Töötuskindlustusmakse — Chômage` · en: `Töötuskindlustusmakse — Unemployment` · de: `Töötuskindlustusmakse — Arbeitslosigkeit` · nl: `Töötuskindlustusmakse — Werkloosheid` · it: `Töötuskindlustusmakse — Disoccupazione` · es: `Töötuskindlustusmakse — Desempleo`

#### EE_KOGUMISPENSION
- **libelle** — fr: `Kogumispension — Retraite 2ᵉ pilier` · en: `Kogumispension — 2nd-pillar pension` · de: `Kogumispension — Rente 2. Säule` · nl: `Kogumispension — Pensioen 2e pijler` · it: `Kogumispension — Pensione 2º pilastro` · es: `Kogumispension — Pensión 2º pilar`

#### EE_SOTSIAALMAKS
- **libelle** — fr: `Sotsiaalmaks — Charge sociale (employeur)` · en: `Sotsiaalmaks — Social charge (employer)` · de: `Sotsiaalmaks — Sozialabgabe (Arbeitgeber)` · nl: `Sotsiaalmaks — Sociale last (werkgever)` · it: `Sotsiaalmaks — Onere sociale (datore di lavoro)` · es: `Sotsiaalmaks — Carga social (empleador)`

#### EE_TULUMAKS
- **libelle** — fr: `Tulumaks — Impôt sur le revenu (22 %)` · en: `Tulumaks — Income tax (22 %)` · de: `Tulumaks — Einkommensteuer (22 %)` · nl: `Tulumaks — Inkomstenbelasting (22 %)` · it: `Tulumaks — Imposta sul reddito (22 %)` · es: `Tulumaks — Impuesto sobre la renta (22 %)`
- **explication** (placeholders `{g} {ab} {b} {im}`) :
  - fr: `Impôt sur le revenu 2025 : 22 % (taux unique).\n\nRevenu annuel {g} € − cotisations salariales − abattement de base {ab} €\n= base imposable {b} € → {im} €/mois.\n\nAbattement de base dégressif (7 848 € si ≤ 14 400 €/an, nul si ≥ 25 200 €/an).\nSource : Maksu- ja Tolliamet.`
  - en: `Income tax 2025: flat 22 %.\n\nAnnual income {g} € − employee contributions − basic allowance {ab} €\n= taxable base {b} € → {im} €/month.\n\nTapering basic allowance (7,848 € if ≤ 14,400 €/yr, nil if ≥ 25,200 €/yr).\nSource: Maksu- ja Tolliamet.`
  - de: `Einkommensteuer 2025: einheitlich 22 %.\n\nJahreseinkommen {g} € − AN-Beiträge − Grundfreibetrag {ab} €\n= Bemessungsgrundlage {b} € → {im} €/Monat.\n\nGleitender Grundfreibetrag (7.848 € bei ≤ 14.400 €/Jahr, 0 bei ≥ 25.200 €/Jahr).\nQuelle: Maksu- ja Tolliamet.`
  - nl: `Inkomstenbelasting 2025: vlak 22 %.\n\nJaarinkomen {g} € − werknemersbijdragen − basisaftrek {ab} €\n= belastbare grondslag {b} € → {im} €/maand.\n\nAflopende basisaftrek (7.848 € bij ≤ 14.400 €/jr, nul bij ≥ 25.200 €/jr).\nBron: Maksu- ja Tolliamet.`
  - it: `Imposta sul reddito 2025: aliquota unica 22 %.\n\nReddito annuo {g} € − contributi dipendente − detrazione di base {ab} €\n= base imponibile {b} € → {im} €/mese.\n\nDetrazione di base decrescente (7.848 € se ≤ 14.400 €/anno, nulla se ≥ 25.200 €/anno).\nFonte: Maksu- ja Tolliamet.`
  - es: `Impuesto sobre la renta 2025: tipo único 22 %.\n\nRenta anual {g} € − cotizaciones del trabajador − mínimo exento {ab} €\n= base imponible {b} € → {im} €/mes.\n\nMínimo exento decreciente (7.848 € si ≤ 14.400 €/año, nulo si ≥ 25.200 €/año).\nFuente: Maksu- ja Tolliamet.`

### 🇫🇮 Finlande

> Gabarit générique `ligne_cot` (placeholders `{libelle} {ts} {tp} {ms}`) :
> fr `{libelle}. Salarié {ts} % / employeur {tp} %. Salarié : {ms} €.` · en `{libelle}. Employee {ts} % / employer {tp} %. Employee: {ms} €.` · de `{libelle}. Arbeitnehmer {ts} % / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.` · nl `{libelle}. Werknemer {ts} % / werkgever {tp} %. Werknemer: {ms} €.` · it `{libelle}. Dipendente {ts} % / datore di lavoro {tp} %. Dipendente: {ms} €.` · es `{libelle}. Trabajador {ts} % / empleador {tp} %. Trabajador: {ms} €.`

#### FI_TYEL
- **libelle** — fr: `TyEL — Retraite` · en: `TyEL — Pension` · de: `TyEL — Rente` · nl: `TyEL — Pensioen` · it: `TyEL — Pensione` · es: `TyEL — Pensión`

#### FI_TYOTTOMYYS
- **libelle** — fr: `Työttömyysvakuutus — Chômage` · en: `Työttömyysvakuutus — Unemployment` · de: `Työttömyysvakuutus — Arbeitslosigkeit` · nl: `Työttömyysvakuutus — Werkloosheid` · it: `Työttömyysvakuutus — Disoccupazione` · es: `Työttömyysvakuutus — Desempleo`

#### FI_SAIRAANHOITO
- **libelle** — fr: `Sairaanhoitomaksu — Soins de santé` · en: `Sairaanhoitomaksu — Healthcare` · de: `Sairaanhoitomaksu — Gesundheitsversorgung` · nl: `Sairaanhoitomaksu — Gezondheidszorg` · it: `Sairaanhoitomaksu — Assistenza sanitaria` · es: `Sairaanhoitomaksu — Asistencia sanitaria`

#### FI_TYONANTAJA_SV
- **libelle** — fr: `Sairausvakuutus employeur` · en: `Health insurance (employer)` · de: `Krankenversicherung (Arbeitgeber)` · nl: `Ziektekostenverzekering (werkgever)` · it: `Assicurazione malattia (datore di lavoro)` · es: `Seguro de enfermedad (empleador)`

#### FI_PAIVARAHA
- **libelle** — fr: `Päivärahamaksu — Indemnités journalières` · en: `Päivärahamaksu — Daily allowance` · de: `Päivärahamaksu — Tagegeldbeitrag` · nl: `Päivärahamaksu — Dagvergoeding` · it: `Päivärahamaksu — Indennità giornaliera` · es: `Päivärahamaksu — Subsidio diario`
- **explication** (placeholders `{g} {m}`) :
  - fr: `Päivärahamaksu — 0,88 % (uniquement si revenu annuel ≥ 17 255 €). Déductible.\nRevenu annuel : {g} € → {m} €/mois.`
  - en: `Päivärahamaksu — 0.88 % (only if annual income ≥ 17,255 €). Deductible.\nAnnual income: {g} € → {m} €/month.`
  - de: `Päivärahamaksu — 0,88 % (nur bei Jahreseinkommen ≥ 17.255 €). Abzugsfähig.\nJahreseinkommen: {g} € → {m} €/Monat.`
  - nl: `Päivärahamaksu — 0,88 % (alleen bij jaarinkomen ≥ 17.255 €). Aftrekbaar.\nJaarinkomen: {g} € → {m} €/maand.`
  - it: `Päivärahamaksu — 0,88 % (solo se reddito annuo ≥ 17.255 €). Deducibile.\nReddito annuo: {g} € → {m} €/mese.`
  - es: `Päivärahamaksu — 0,88 % (solo si renta anual ≥ 17.255 €). Deducible.\nRenta anual: {g} € → {m} €/mes.`

#### FI_TULOVERO
- **libelle** — fr: `Tulovero — Impôt (État + communal)` · en: `Tulovero — Tax (state + municipal)` · de: `Tulovero — Steuer (Staat + Gemeinde)` · nl: `Tulovero — Belasting (staat + gemeente)` · it: `Tulovero — Imposta (statale + comunale)` · es: `Tulovero — Impuesto (estatal + municipal)`
- **explication** (placeholders `{g} {ded} {tx} {et} {co} {im}`) :
  - fr: `Impôt sur le revenu 2026 (annualisé).\n\nRevenu imposable : {g} € − cotisations déductibles {ded} € = {tx} €\nBarème d'État : 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n(seuils 21 200 / 32 600 / 40 100 / 52 100 €) → {et} €\nImpôt communal moyen 7,50 % → {co} €\n= {im} €/mois.\n\nNote : crédits työtulovähennys / perusvähennys non modélisés (net prudent).\nBase légale : Tuloverolaki.`
  - en: `Income tax 2026 (annualised).\n\nTaxable income: {g} € − deductible contributions {ded} € = {tx} €\nState scale: 12.64 % / 19 % / 30.25 % / 33.25 % / 37.5 %\n(thresholds 21,200 / 32,600 / 40,100 / 52,100 €) → {et} €\nAverage municipal tax 7.50 % → {co} €\n= {im} €/month.\n\nNote: työtulovähennys / perusvähennys credits not modelled (conservative net).\nLegal basis: Tuloverolaki.`
  - de: `Einkommensteuer 2026 (auf Jahresbasis).\n\nZu versteuern: {g} € − abzugsfähige Beiträge {ded} € = {tx} €\nStaatstarif: 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n(Grenzen 21.200 / 32.600 / 40.100 / 52.100 €) → {et} €\nDurchschn. Gemeindesteuer 7,50 % → {co} €\n= {im} €/Monat.\n\nHinweis: työtulovähennys / perusvähennys nicht modelliert (vorsichtiger Nettowert).\nRechtsgrundlage: Tuloverolaki.`
  - nl: `Inkomstenbelasting 2026 (op jaarbasis).\n\nBelastbaar inkomen: {g} € − aftrekbare bijdragen {ded} € = {tx} €\nRijksschaal: 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n(drempels 21.200 / 32.600 / 40.100 / 52.100 €) → {et} €\nGemiddelde gemeentebelasting 7,50 % → {co} €\n= {im} €/maand.\n\nNoot: työtulovähennys / perusvähennys niet gemodelleerd (voorzichtig netto).\nWettelijke basis: Tuloverolaki.`
  - it: `Imposta sul reddito 2026 (annualizzata).\n\nReddito imponibile: {g} € − contributi deducibili {ded} € = {tx} €\nScala statale: 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n(soglie 21.200 / 32.600 / 40.100 / 52.100 €) → {et} €\nImposta comunale media 7,50 % → {co} €\n= {im} €/mese.\n\nNota: crediti työtulovähennys / perusvähennys non modellati (netto prudente).\nBase giuridica: Tuloverolaki.`
  - es: `Impuesto sobre la renta 2026 (anualizado).\n\nRenta imponible: {g} € − cotizaciones deducibles {ded} € = {tx} €\nEscala estatal: 12,64 % / 19 % / 30,25 % / 33,25 % / 37,5 %\n(umbrales 21.200 / 32.600 / 40.100 / 52.100 €) → {et} €\nImpuesto municipal medio 7,50 % → {co} €\n= {im} €/mes.\n\nNota: créditos työtulovähennys / perusvähennys no modelados (neto prudente).\nBase legal: Tuloverolaki.`

### 🇬🇷 Grèce

#### GR_EFKA
- **libelle** — fr: `EFKA — Cotisations sociales` · en: `EFKA — Social contributions` · de: `EFKA — Sozialbeiträge` · nl: `EFKA — Sociale bijdragen` · it: `EFKA — Contributi sociali` · es: `EFKA — Cotizaciones sociales`
- **explication** (placeholders `{ts} {tp} {ms}`) :
  - fr: `EFKA — salarié {ts} % / employeur {tp} % (retraite, maladie, complémentaire). Assiette plafonnée à 7 572,62 €/mois. Salarié : {ms} €.`
  - en: `EFKA — employee {ts} % / employer {tp} % (pension, health, supplementary). Base capped at 7,572.62 €/month. Employee: {ms} €.`
  - de: `EFKA — Arbeitnehmer {ts} % / Arbeitgeber {tp} % (Rente, Kranken, Zusatz). Bemessungsgrundlage gedeckelt auf 7.572,62 €/Monat. Arbeitnehmer: {ms} €.`
  - nl: `EFKA — werknemer {ts} % / werkgever {tp} % (pensioen, ziekte, aanvullend). Grondslag begrensd op 7.572,62 €/maand. Werknemer: {ms} €.`
  - it: `EFKA — dipendente {ts} % / datore di lavoro {tp} % (pensione, malattia, complementare). Base limitata a 7.572,62 €/mese. Dipendente: {ms} €.`
  - es: `EFKA — trabajador {ts} % / empleador {tp} % (pensión, enfermedad, complementaria). Base limitada a 7.572,62 €/mes. Trabajador: {ms} €.`

#### GR_FOROS
- **libelle** — fr: `Φόρος εισοδήματος — Impôt sur le revenu` · en: `Φόρος εισοδήματος — Income tax` · de: `Φόρος εισοδήματος — Einkommensteuer` · nl: `Φόρος εισοδήματος — Inkomstenbelasting` · it: `Φόρος εισοδήματος — Imposta sul reddito` · es: `Φόρος εισοδήματος — Impuesto sobre la renta`
- **explication** (placeholders `{b} {im}`) :
  - fr: `Impôt sur le revenu 2025 (annualisé).\n\nBase = (brut − EFKA) × 12 = {b} €\nBarème 9 / 22 / 28 / 36 / 44 % (seuils 10 000 / 20 000 / 30 000 / 40 000 €)\n− réduction salarié 777 € → {im} €/mois.\n\nNote : majorations pour enfants non modélisées (net prudent).\nSource : AADE.`
  - en: `Income tax 2025 (annualised).\n\nBase = (gross − EFKA) × 12 = {b} €\nScale 9 / 22 / 28 / 36 / 44 % (thresholds 10,000 / 20,000 / 30,000 / 40,000 €)\n− employee relief 777 € → {im} €/month.\n\nNote: child increases not modelled (conservative net).\nSource: AADE.`
  - de: `Einkommensteuer 2025 (auf Jahresbasis).\n\nBemessung = (brutto − EFKA) × 12 = {b} €\nTarif 9 / 22 / 28 / 36 / 44 % (Grenzen 10.000 / 20.000 / 30.000 / 40.000 €)\n− AN-Ermäßigung 777 € → {im} €/Monat.\n\nHinweis: Kinderzuschläge nicht modelliert (vorsichtiger Nettowert).\nQuelle: AADE.`
  - nl: `Inkomstenbelasting 2025 (op jaarbasis).\n\nGrondslag = (bruto − EFKA) × 12 = {b} €\nSchaal 9 / 22 / 28 / 36 / 44 % (drempels 10.000 / 20.000 / 30.000 / 40.000 €)\n− werknemerskorting 777 € → {im} €/maand.\n\nNoot: kindverhogingen niet gemodelleerd (voorzichtig netto).\nBron: AADE.`
  - it: `Imposta sul reddito 2025 (annualizzata).\n\nBase = (lordo − EFKA) × 12 = {b} €\nScala 9 / 22 / 28 / 36 / 44 % (soglie 10.000 / 20.000 / 30.000 / 40.000 €)\n− riduzione dipendente 777 € → {im} €/mese.\n\nNota: maggiorazioni per figli non modellate (netto prudente).\nFonte: AADE.`
  - es: `Impuesto sobre la renta 2025 (anualizado).\n\nBase = (bruto − EFKA) × 12 = {b} €\nEscala 9 / 22 / 28 / 36 / 44 % (umbrales 10.000 / 20.000 / 30.000 / 40.000 €)\n− reducción del trabajador 777 € → {im} €/mes.\n\nNota: incrementos por hijos no modelados (neto prudente).\nFuente: AADE.`

### 🇭🇷 Croatie

#### HR_MIROVINSKO
- **libelle** — fr: `Mirovinsko osiguranje — Retraite` · en: `Mirovinsko osiguranje — Pension` · de: `Mirovinsko osiguranje — Rente` · nl: `Mirovinsko osiguranje — Pensioen` · it: `Mirovinsko osiguranje — Pensione` · es: `Mirovinsko osiguranje — Pensión`
- **explication** (placeholders `{ts} {ms}`) :
  - fr: `Retraite — {ts} % salarié (1er pilier 15 % + 2ᵉ pilier 5 %). Salarié : {ms} €.`
  - en: `Pension — {ts} % employee (1st pillar 15 % + 2nd pillar 5 %). Employee: {ms} €.`
  - de: `Rente — {ts} % Arbeitnehmer (1. Säule 15 % + 2. Säule 5 %). Arbeitnehmer: {ms} €.`
  - nl: `Pensioen — {ts} % werknemer (1e pijler 15 % + 2e pijler 5 %). Werknemer: {ms} €.`
  - it: `Pensione — {ts} % dipendente (1º pilastro 15 % + 2º pilastro 5 %). Dipendente: {ms} €.`
  - es: `Pensión — {ts} % trabajador (1er pilar 15 % + 2º pilar 5 %). Trabajador: {ms} €.`

#### HR_ZDRAVSTVENO
- **libelle** — fr: `Zdravstveno osiguranje — Santé (employeur)` · en: `Zdravstveno osiguranje — Health (employer)` · de: `Zdravstveno osiguranje — Gesundheit (Arbeitgeber)` · nl: `Zdravstveno osiguranje — Gezondheid (werkgever)` · it: `Zdravstveno osiguranje — Sanità (datore di lavoro)` · es: `Zdravstveno osiguranje — Salud (empleador)`
- **explication** (placeholders `{tp}`) :
  - fr: `Assurance maladie — {tp} % à la charge de l'employeur.`
  - en: `Health insurance — {tp} % borne by the employer.`
  - de: `Krankenversicherung — {tp} % zu Lasten des Arbeitgebers.`
  - nl: `Ziektekostenverzekering — {tp} % ten laste van de werkgever.`
  - it: `Assicurazione malattia — {tp} % a carico del datore di lavoro.`
  - es: `Seguro de enfermedad — {tp} % a cargo del empleador.`

#### HR_POREZ
- **libelle** — fr: `Porez na dohodak — Impôt sur le revenu` · en: `Porez na dohodak — Income tax` · de: `Porez na dohodak — Einkommensteuer` · nl: `Porez na dohodak — Inkomstenbelasting` · it: `Porez na dohodak — Imposta sul reddito` · es: `Porez na dohodak — Impuesto sobre la renta`
- **explication** (placeholders `{b} {im}`) :
  - fr: `Impôt sur le revenu 2025.\n\nBase = brut − retraite − abattement 600 € = {b} €\n20 % jusqu'à 5 000 €/mois, 30 % au-delà → {im} €/mois.\n\nNote : taux communaux représentatifs. Source : Porezna uprava.`
  - en: `Income tax 2025.\n\nBase = gross − pension − allowance 600 € = {b} €\n20 % up to 5,000 €/month, 30 % above → {im} €/month.\n\nNote: representative municipal rates. Source: Porezna uprava.`
  - de: `Einkommensteuer 2025.\n\nBemessung = brutto − Rente − Freibetrag 600 € = {b} €\n20 % bis 5.000 €/Monat, 30 % darüber → {im} €/Monat.\n\nHinweis: repräsentative Gemeindesätze. Quelle: Porezna uprava.`
  - nl: `Inkomstenbelasting 2025.\n\nGrondslag = bruto − pensioen − aftrek 600 € = {b} €\n20 % tot 5.000 €/maand, 30 % daarboven → {im} €/maand.\n\nNoot: representatieve gemeentetarieven. Bron: Porezna uprava.`
  - it: `Imposta sul reddito 2025.\n\nBase = lordo − pensione − detrazione 600 € = {b} €\n20 % fino a 5.000 €/mese, 30 % oltre → {im} €/mese.\n\nNota: aliquote comunali rappresentative. Fonte: Porezna uprava.`
  - es: `Impuesto sobre la renta 2025.\n\nBase = bruto − pensión − reducción 600 € = {b} €\n20 % hasta 5.000 €/mes, 30 % por encima → {im} €/mes.\n\nNota: tipos municipales representativos. Fuente: Porezna uprava.`

### 🇭🇺 Hongrie

#### HU_TB
- **libelle** — fr: `Társadalombiztosítás — Cotisation sociale` · en: `Társadalombiztosítás — Social contribution` · de: `Társadalombiztosítás — Sozialbeitrag` · nl: `Társadalombiztosítás — Sociale bijdrage` · it: `Társadalombiztosítás — Contributo sociale` · es: `Társadalombiztosítás — Cotización social`
- **explication** (placeholders `{ts} {ms}`) :
  - fr: `TB — {ts} % salarié (retraite 10 % + maladie 7 % + chômage 1,5 %). Salarié : {ms} HUF.`
  - en: `TB — {ts} % employee (pension 10 % + health 7 % + unemployment 1.5 %). Employee: {ms} HUF.`
  - de: `TB — {ts} % Arbeitnehmer (Rente 10 % + Kranken 7 % + Arbeitslosen 1,5 %). Arbeitnehmer: {ms} HUF.`
  - nl: `TB — {ts} % werknemer (pensioen 10 % + ziekte 7 % + werkloosheid 1,5 %). Werknemer: {ms} HUF.`
  - it: `TB — {ts} % dipendente (pensione 10 % + malattia 7 % + disoccupazione 1,5 %). Dipendente: {ms} HUF.`
  - es: `TB — {ts} % trabajador (pensión 10 % + enfermedad 7 % + desempleo 1,5 %). Trabajador: {ms} HUF.`

#### HU_SZOCHO
- **libelle** — fr: `Szociális hozzájárulási adó (employeur)` · en: `Social contribution tax (employer)` · de: `Sozialbeitragssteuer (Arbeitgeber)` · nl: `Sociale bijdrageheffing (werkgever)` · it: `Imposta sul contributo sociale (datore di lavoro)` · es: `Impuesto de cotización social (empleador)`
- **explication** (placeholders `{tp}`) :
  - fr: `Szocho — {tp} % à la charge de l'employeur.`
  - en: `Szocho — {tp} % borne by the employer.`
  - de: `Szocho — {tp} % zu Lasten des Arbeitgebers.`
  - nl: `Szocho — {tp} % ten laste van de werkgever.`
  - it: `Szocho — {tp} % a carico del datore di lavoro.`
  - es: `Szocho — {tp} % a cargo del empleador.`

#### HU_SZJA
- **libelle** — fr: `SZJA — Impôt sur le revenu (15 %)` · en: `SZJA — Income tax (15 %)` · de: `SZJA — Einkommensteuer (15 %)` · nl: `SZJA — Inkomstenbelasting (15 %)` · it: `SZJA — Imposta sul reddito (15 %)` · es: `SZJA — Impuesto sobre la renta (15 %)`
- **explication** (placeholders `{im}`) :
  - fr: `Impôt sur le revenu 2025 : taux proportionnel unique 15 % → {im} HUF/mois.\n\nNote : abattements familiaux et exonérations jeunes/mères non modélisés (net prudent).\nSource : NAV.`
  - en: `Income tax 2025: flat 15 % → {im} HUF/month.\n\nNote: family allowances and young/mothers exemptions not modelled (conservative net).\nSource: NAV.`
  - de: `Einkommensteuer 2025: einheitlich 15 % → {im} HUF/Monat.\n\nHinweis: Familienfreibeträge und Befreiungen für Junge/Mütter nicht modelliert (vorsichtiger Nettowert).\nQuelle: NAV.`
  - nl: `Inkomstenbelasting 2025: vlak 15 % → {im} HUF/maand.\n\nNoot: gezinsaftrekken en vrijstellingen jongeren/moeders niet gemodelleerd (voorzichtig netto).\nBron: NAV.`
  - it: `Imposta sul reddito 2025: aliquota unica 15 % → {im} HUF/mese.\n\nNota: detrazioni familiari ed esenzioni giovani/madri non modellate (netto prudente).\nFonte: NAV.`
  - es: `Impuesto sobre la renta 2025: tipo único 15 % → {im} HUF/mes.\n\nNota: deducciones familiares y exenciones jóvenes/madres no modeladas (neto prudente).\nFuente: NAV.`

### 🇮🇪 Irlande

#### IE_PRSI
- **libelle** — fr: `PRSI (Class A) — Cotisation sociale` · en: `PRSI (Class A) — Social contribution` · de: `PRSI (Class A) — Sozialbeitrag` · nl: `PRSI (Class A) — Sociale bijdrage` · it: `PRSI (Class A) — Contributo sociale` · es: `PRSI (Class A) — Cotización social`
- **explication** (placeholders `{ts} {tp} {ms}`) :
  - fr: `PRSI Class A — salarié {ts} % / employeur {tp} %. Salarié : {ms} €.`
  - en: `PRSI Class A — employee {ts} % / employer {tp} %. Employee: {ms} €.`
  - de: `PRSI Class A — Arbeitnehmer {ts} % / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.`
  - nl: `PRSI Class A — werknemer {ts} % / werkgever {tp} %. Werknemer: {ms} €.`
  - it: `PRSI Class A — dipendente {ts} % / datore di lavoro {tp} %. Dipendente: {ms} €.`
  - es: `PRSI Class A — trabajador {ts} % / empleador {tp} %. Trabajador: {ms} €.`

#### IE_USC
- **libelle** — fr: `Universal Social Charge (USC)` · en: `Universal Social Charge (USC)` · de: `Universal Social Charge (USC)` · nl: `Universal Social Charge (USC)` · it: `Universal Social Charge (USC)` · es: `Universal Social Charge (USC)`
- **explication** (placeholders `{g} {im}`) :
  - fr: `USC 2025 : 0,5 % / 2 % / 3 % / 8 % (seuils 12 012 / 27 382 / 70 044 €).\nRevenu annuel {g} € → {im} €/mois.`
  - en: `USC 2025: 0.5 % / 2 % / 3 % / 8 % (thresholds 12,012 / 27,382 / 70,044 €).\nAnnual income {g} € → {im} €/month.`
  - de: `USC 2025: 0,5 % / 2 % / 3 % / 8 % (Grenzen 12.012 / 27.382 / 70.044 €).\nJahreseinkommen {g} € → {im} €/Monat.`
  - nl: `USC 2025: 0,5 % / 2 % / 3 % / 8 % (drempels 12.012 / 27.382 / 70.044 €).\nJaarinkomen {g} € → {im} €/maand.`
  - it: `USC 2025: 0,5 % / 2 % / 3 % / 8 % (soglie 12.012 / 27.382 / 70.044 €).\nReddito annuo {g} € → {im} €/mese.`
  - es: `USC 2025: 0,5 % / 2 % / 3 % / 8 % (umbrales 12.012 / 27.382 / 70.044 €).\nRenta anual {g} € → {im} €/mes.`

#### IE_PAYE
- **libelle** — fr: `Income Tax (PAYE) — Impôt sur le revenu` · en: `Income Tax (PAYE)` · de: `Income Tax (PAYE) — Einkommensteuer` · nl: `Income Tax (PAYE) — Inkomstenbelasting` · it: `Income Tax (PAYE) — Imposta sul reddito` · es: `Income Tax (PAYE) — Impuesto sobre la renta`
- **explication** (placeholders `{g} {im}`) :
  - fr: `Impôt sur le revenu 2025 (annualisé).\n\n20 % jusqu'à 44 000 €/an, 40 % au-delà − crédits 4 000 € (personnel + PAYE)\nRevenu annuel {g} € → {im} €/mois.\n\nNote : crédits d'un salarié célibataire. Source : Revenue.`
  - en: `Income tax 2025 (annualised).\n\n20 % up to 44,000 €/yr, 40 % above − credits 4,000 € (personal + PAYE)\nAnnual income {g} € → {im} €/month.\n\nNote: single-employee credits. Source: Revenue.`
  - de: `Einkommensteuer 2025 (auf Jahresbasis).\n\n20 % bis 44.000 €/Jahr, 40 % darüber − Absetzbeträge 4.000 € (persönlich + PAYE)\nJahreseinkommen {g} € → {im} €/Monat.\n\nHinweis: Absetzbeträge eines alleinstehenden Arbeitnehmers. Quelle: Revenue.`
  - nl: `Inkomstenbelasting 2025 (op jaarbasis).\n\n20 % tot 44.000 €/jr, 40 % daarboven − kortingen 4.000 € (persoonlijk + PAYE)\nJaarinkomen {g} € → {im} €/maand.\n\nNoot: kortingen van een alleenstaande werknemer. Bron: Revenue.`
  - it: `Imposta sul reddito 2025 (annualizzata).\n\n20 % fino a 44.000 €/anno, 40 % oltre − crediti 4.000 € (personale + PAYE)\nReddito annuo {g} € → {im} €/mese.\n\nNota: crediti di un dipendente celibe. Fonte: Revenue.`
  - es: `Impuesto sobre la renta 2025 (anualizado).\n\n20 % hasta 44.000 €/año, 40 % por encima − créditos 4.000 € (personal + PAYE)\nRenta anual {g} € → {im} €/mes.\n\nNota: créditos de un trabajador soltero. Fuente: Revenue.`

### 🇱🇻 Lettonie

#### LV_VSAOI
- **libelle** — fr: `VSAOI — Cotisations sociales obligatoires` · en: `VSAOI — Mandatory social contributions` · de: `VSAOI — Pflichtsozialbeiträge` · nl: `VSAOI — Verplichte sociale bijdragen` · it: `VSAOI — Contributi sociali obbligatori` · es: `VSAOI — Cotizaciones sociales obligatorias`
- **explication** (placeholders `{ts} {tp} {ms}`) :
  - fr: `VSAOI — salarié {ts} % / employeur {tp} % (retraite, maladie, chômage, maternité, accidents). Salarié : {ms} €.`
  - en: `VSAOI — employee {ts} % / employer {tp} % (pension, health, unemployment, maternity, accidents). Employee: {ms} €.`
  - de: `VSAOI — Arbeitnehmer {ts} % / Arbeitgeber {tp} % (Rente, Kranken, Arbeitslosen, Mutterschaft, Unfälle). Arbeitnehmer: {ms} €.`
  - nl: `VSAOI — werknemer {ts} % / werkgever {tp} % (pensioen, ziekte, werkloosheid, moederschap, ongevallen). Werknemer: {ms} €.`
  - it: `VSAOI — dipendente {ts} % / datore di lavoro {tp} % (pensione, malattia, disoccupazione, maternità, infortuni). Dipendente: {ms} €.`
  - es: `VSAOI — trabajador {ts} % / empleador {tp} % (pensión, enfermedad, desempleo, maternidad, accidentes). Trabajador: {ms} €.`

#### LV_IIN
- **libelle** — fr: `IIN — Impôt sur le revenu` · en: `IIN — Income tax` · de: `IIN — Einkommensteuer` · nl: `IIN — Inkomstenbelasting` · it: `IIN — Imposta sul reddito` · es: `IIN — Impuesto sobre la renta`
- **explication** (placeholders `{vs} {b} {iin}`) :
  - fr: `Impôt sur le revenu 2025.\n\nBase = brut − VSAOI {vs} € − minimum non imposable 510 € = {b} €\nTaux 25,5 % (jusqu'à 8 775 €/mois) puis 33 % au-delà → {iin} €/mois.\n\nSource : Valsts ieņēmumu dienests.`
  - en: `Income tax 2025.\n\nBase = gross − VSAOI {vs} € − tax-free minimum 510 € = {b} €\nRate 25.5 % (up to 8,775 €/month) then 33 % above → {iin} €/month.\n\nSource: Valsts ieņēmumu dienests.`
  - de: `Einkommensteuer 2025.\n\nBemessung = brutto − VSAOI {vs} € − Steuerfreibetrag 510 € = {b} €\nSatz 25,5 % (bis 8.775 €/Monat) dann 33 % darüber → {iin} €/Monat.\n\nQuelle: Valsts ieņēmumu dienests.`
  - nl: `Inkomstenbelasting 2025.\n\nGrondslag = bruto − VSAOI {vs} € − belastingvrij minimum 510 € = {b} €\nTarief 25,5 % (tot 8.775 €/maand) dan 33 % daarboven → {iin} €/maand.\n\nBron: Valsts ieņēmumu dienests.`
  - it: `Imposta sul reddito 2025.\n\nBase = lordo − VSAOI {vs} € − minimo esente 510 € = {b} €\nAliquota 25,5 % (fino a 8.775 €/mese) poi 33 % oltre → {iin} €/mese.\n\nFonte: Valsts ieņēmumu dienests.`
  - es: `Impuesto sobre la renta 2025.\n\nBase = bruto − VSAOI {vs} € − mínimo exento 510 € = {b} €\nTipo 25,5 % (hasta 8.775 €/mes) luego 33 % por encima → {iin} €/mes.\n\nFuente: Valsts ieņēmumu dienests.`

### 🇱🇹 Lituanie

#### LT_SODRA
- **libelle** — fr: `Sodra — Cotisations sociales` · en: `Sodra — Social contributions` · de: `Sodra — Sozialbeiträge` · nl: `Sodra — Sociale bijdragen` · it: `Sodra — Contributi sociali` · es: `Sodra — Cotizaciones sociales`
- **explication** (placeholders `{ts} {tp} {ms}`) :
  - fr: `Sodra — salarié {ts} % (retraite, maladie/PSD, maternité) / employeur {tp} %. Salarié : {ms} €.`
  - en: `Sodra — employee {ts} % (pension, health/PSD, maternity) / employer {tp} %. Employee: {ms} €.`
  - de: `Sodra — Arbeitnehmer {ts} % (Rente, Kranken/PSD, Mutterschaft) / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.`
  - nl: `Sodra — werknemer {ts} % (pensioen, ziekte/PSD, moederschap) / werkgever {tp} %. Werknemer: {ms} €.`
  - it: `Sodra — dipendente {ts} % (pensione, malattia/PSD, maternità) / datore di lavoro {tp} %. Dipendente: {ms} €.`
  - es: `Sodra — trabajador {ts} % (pensión, enfermedad/PSD, maternidad) / empleador {tp} %. Trabajador: {ms} €.`

#### LT_GPM
- **libelle** — fr: `GPM — Impôt sur le revenu` · en: `GPM — Income tax` · de: `GPM — Einkommensteuer` · nl: `GPM — Inkomstenbelasting` · it: `GPM — Imposta sul reddito` · es: `GPM — Impuesto sobre la renta`
- **explication** (placeholders `{npd} {b} {gpm}`) :
  - fr: `Impôt sur le revenu 2025 (GPM).\n\nNPD (non imposable) dégressif : {npd} €\nBase = brut − NPD = {b} €\nTaux 20 % (jusqu'à ≈ 10 540 €/mois) puis 32 % au-delà → {gpm} €/mois.\n\nSource : VMI.`
  - en: `Income tax 2025 (GPM).\n\nTapering tax-free amount (NPD): {npd} €\nBase = gross − NPD = {b} €\nRate 20 % (up to ≈ 10,540 €/month) then 32 % above → {gpm} €/month.\n\nSource: VMI.`
  - de: `Einkommensteuer 2025 (GPM).\n\nGleitender Freibetrag (NPD): {npd} €\nBemessung = brutto − NPD = {b} €\nSatz 20 % (bis ≈ 10.540 €/Monat) dann 32 % darüber → {gpm} €/Monat.\n\nQuelle: VMI.`
  - nl: `Inkomstenbelasting 2025 (GPM).\n\nAflopend belastingvrij bedrag (NPD): {npd} €\nGrondslag = bruto − NPD = {b} €\nTarief 20 % (tot ≈ 10.540 €/maand) dan 32 % daarboven → {gpm} €/maand.\n\nBron: VMI.`
  - it: `Imposta sul reddito 2025 (GPM).\n\nImporto esente decrescente (NPD): {npd} €\nBase = lordo − NPD = {b} €\nAliquota 20 % (fino a ≈ 10.540 €/mese) poi 32 % oltre → {gpm} €/mese.\n\nFonte: VMI.`
  - es: `Impuesto sobre la renta 2025 (GPM).\n\nImporte exento decreciente (NPD): {npd} €\nBase = bruto − NPD = {b} €\nTipo 20 % (hasta ≈ 10.540 €/mes) luego 32 % por encima → {gpm} €/mes.\n\nFuente: VMI.`

### 🇲🇹 Malte

#### MT_SSC
- **libelle** — fr: `Social Security Contributions (Klassi 1)` · en: `Social Security Contributions (Class 1)` · de: `Social Security Contributions (Klassi 1)` · nl: `Social Security Contributions (Klassi 1)` · it: `Social Security Contributions (Klassi 1)` · es: `Social Security Contributions (Klassi 1)`
- **explication** (placeholders `{ts} {tp} {ms}`) :
  - fr: `SSC — salarié {ts} % / employeur {tp} %. Assiette plafonnée à 2 306,58 €/mois (≈ 27 679 €/an). Salarié : {ms} €.`
  - en: `SSC — employee {ts} % / employer {tp} %. Base capped at 2,306.58 €/month (≈ 27,679 €/yr). Employee: {ms} €.`
  - de: `SSC — Arbeitnehmer {ts} % / Arbeitgeber {tp} %. Bemessungsgrundlage gedeckelt auf 2.306,58 €/Monat (≈ 27.679 €/Jahr). Arbeitnehmer: {ms} €.`
  - nl: `SSC — werknemer {ts} % / werkgever {tp} %. Grondslag begrensd op 2.306,58 €/maand (≈ 27.679 €/jr). Werknemer: {ms} €.`
  - it: `SSC — dipendente {ts} % / datore di lavoro {tp} %. Base limitata a 2.306,58 €/mese (≈ 27.679 €/anno). Dipendente: {ms} €.`
  - es: `SSC — trabajador {ts} % / empleador {tp} %. Base limitada a 2.306,58 €/mes (≈ 27.679 €/año). Trabajador: {ms} €.`

#### MT_TAX
- **libelle** — fr: `Income Tax — Impôt sur le revenu` · en: `Income Tax` · de: `Income Tax — Einkommensteuer` · nl: `Income Tax — Inkomstenbelasting` · it: `Income Tax — Imposta sul reddito` · es: `Income Tax — Impuesto sobre la renta`
- **explication** (placeholders `{b} {im}`) :
  - fr: `Impôt sur le revenu 2025 (barème single, annualisé).\n\nBase = brut × 12 = {b} €\n0 % jusqu'à 12 000 €, puis 15 % / 25 % / 35 % (abattements 1 800 / 3 400 / 9 400 €)\n→ {im} €/mois.\n\nSource : Commissioner for Revenue.`
  - en: `Income tax 2025 (single scale, annualised).\n\nBase = gross × 12 = {b} €\n0 % up to 12,000 €, then 15 % / 25 % / 35 % (abatements 1,800 / 3,400 / 9,400 €)\n→ {im} €/month.\n\nSource: Commissioner for Revenue.`
  - de: `Einkommensteuer 2025 (Single-Tarif, auf Jahresbasis).\n\nBemessung = brutto × 12 = {b} €\n0 % bis 12.000 €, dann 15 % / 25 % / 35 % (Abzüge 1.800 / 3.400 / 9.400 €)\n→ {im} €/Monat.\n\nQuelle: Commissioner for Revenue.`
  - nl: `Inkomstenbelasting 2025 (single-schaal, op jaarbasis).\n\nGrondslag = bruto × 12 = {b} €\n0 % tot 12.000 €, dan 15 % / 25 % / 35 % (aftrekken 1.800 / 3.400 / 9.400 €)\n→ {im} €/maand.\n\nBron: Commissioner for Revenue.`
  - it: `Imposta sul reddito 2025 (scala single, annualizzata).\n\nBase = lordo × 12 = {b} €\n0 % fino a 12.000 €, poi 15 % / 25 % / 35 % (abbattimenti 1.800 / 3.400 / 9.400 €)\n→ {im} €/mese.\n\nFonte: Commissioner for Revenue.`
  - es: `Impuesto sobre la renta 2025 (escala single, anualizado).\n\nBase = bruto × 12 = {b} €\n0 % hasta 12.000 €, luego 15 % / 25 % / 35 % (reducciones 1.800 / 3.400 / 9.400 €)\n→ {im} €/mes.\n\nFuente: Commissioner for Revenue.`

### 🇸🇮 Slovénie

#### SI_PRISPEVKI
- **libelle** — fr: `Prispevki — Cotisations sociales` · en: `Prispevki — Social contributions` · de: `Prispevki — Sozialbeiträge` · nl: `Prispevki — Sociale bijdragen` · it: `Prispevki — Contributi sociali` · es: `Prispevki — Cotizaciones sociales`
- **explication** (placeholders `{ts} {tp} {ms}`) :
  - fr: `Prispevki — salarié {ts} % (retraite/invalidité 15,5 %, maladie 6,36 %, chômage 0,14 %, parental 0,10 %) / employeur {tp} %. Salarié : {ms} €.`
  - en: `Prispevki — employee {ts} % (pension/disability 15.5 %, health 6.36 %, unemployment 0.14 %, parental 0.10 %) / employer {tp} %. Employee: {ms} €.`
  - de: `Prispevki — Arbeitnehmer {ts} % (Rente/Invalidität 15,5 %, Kranken 6,36 %, Arbeitslosen 0,14 %, Eltern 0,10 %) / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.`
  - nl: `Prispevki — werknemer {ts} % (pensioen/invaliditeit 15,5 %, ziekte 6,36 %, werkloosheid 0,14 %, ouderschap 0,10 %) / werkgever {tp} %. Werknemer: {ms} €.`
  - it: `Prispevki — dipendente {ts} % (pensione/invalidità 15,5 %, malattia 6,36 %, disoccupazione 0,14 %, parentale 0,10 %) / datore di lavoro {tp} %. Dipendente: {ms} €.`
  - es: `Prispevki — trabajador {ts} % (pensión/invalidez 15,5 %, enfermedad 6,36 %, desempleo 0,14 %, parental 0,10 %) / empleador {tp} %. Trabajador: {ms} €.`

#### SI_DOHODNINA
- **libelle** — fr: `Dohodnina — Impôt sur le revenu` · en: `Dohodnina — Income tax` · de: `Dohodnina — Einkommensteuer` · nl: `Dohodnina — Inkomstenbelasting` · it: `Dohodnina — Imposta sul reddito` · es: `Dohodnina — Impuesto sobre la renta`
- **explication** (placeholders `{b} {im}`) :
  - fr: `Impôt sur le revenu 2025 (annualisé).\n\nBase = (brut − cotisations) × 12 − abattement 5 000 € = {b} €\nBarème 16 / 26 / 33 / 39 / 50 % (seuils 9 210 / 27 089 / 54 179 / 78 016 €)\n→ {im} €/mois.\n\nNote : abattement majoré pour bas revenus non modélisé (net prudent).\nSource : FURS.`
  - en: `Income tax 2025 (annualised).\n\nBase = (gross − contributions) × 12 − allowance 5,000 € = {b} €\nScale 16 / 26 / 33 / 39 / 50 % (thresholds 9,210 / 27,089 / 54,179 / 78,016 €)\n→ {im} €/month.\n\nNote: increased low-income allowance not modelled (conservative net).\nSource: FURS.`
  - de: `Einkommensteuer 2025 (auf Jahresbasis).\n\nBemessung = (brutto − Beiträge) × 12 − Freibetrag 5.000 € = {b} €\nTarif 16 / 26 / 33 / 39 / 50 % (Grenzen 9.210 / 27.089 / 54.179 / 78.016 €)\n→ {im} €/Monat.\n\nHinweis: erhöhter Freibetrag für niedrige Einkommen nicht modelliert (vorsichtiger Nettowert).\nQuelle: FURS.`
  - nl: `Inkomstenbelasting 2025 (op jaarbasis).\n\nGrondslag = (bruto − bijdragen) × 12 − aftrek 5.000 € = {b} €\nSchaal 16 / 26 / 33 / 39 / 50 % (drempels 9.210 / 27.089 / 54.179 / 78.016 €)\n→ {im} €/maand.\n\nNoot: verhoogde aftrek voor lage inkomens niet gemodelleerd (voorzichtig netto).\nBron: FURS.`
  - it: `Imposta sul reddito 2025 (annualizzata).\n\nBase = (lordo − contributi) × 12 − detrazione 5.000 € = {b} €\nScala 16 / 26 / 33 / 39 / 50 % (soglie 9.210 / 27.089 / 54.179 / 78.016 €)\n→ {im} €/mese.\n\nNota: detrazione maggiorata per redditi bassi non modellata (netto prudente).\nFonte: FURS.`
  - es: `Impuesto sobre la renta 2025 (anualizado).\n\nBase = (bruto − cotizaciones) × 12 − reducción 5.000 € = {b} €\nEscala 16 / 26 / 33 / 39 / 50 % (umbrales 9.210 / 27.089 / 54.179 / 78.016 €)\n→ {im} €/mes.\n\nNota: reducción incrementada para rentas bajas no modelada (neto prudente).\nFuente: FURS.`

### 🇸🇪 Suède

#### SE_ARBETSGIVARAVGIFT
- **libelle** — fr: `Arbetsgivaravgifter — cotisations patronales` · en: `Arbetsgivaravgifter — employer contributions` · de: `Arbetsgivaravgifter — Arbeitgeberbeiträge` · nl: `Arbetsgivaravgifter — werkgeversbijdragen` · it: `Arbetsgivaravgifter — contributi del datore di lavoro` · es: `Arbetsgivaravgifter — cotizaciones patronales`
- **explication** (placeholders `{tp}`) :
  - fr: `Arbetsgivaravgifter — {tp} % à la charge de l'employeur (retraite, maladie, parentalité, accident, marché du travail, taxe générale sur salaires).\n\nCôté salarié : l'allmän pensionsavgift (7 %) est intégralement compensée par une réduction d'impôt (effet net nul) → non affichée.`
  - en: `Arbetsgivaravgifter — {tp} % borne by the employer (pension, health, parental, accident, labour market, general payroll tax).\n\nEmployee side: the allmän pensionsavgift (7 %) is fully offset by a tax reduction (net nil effect) → not shown.`
  - de: `Arbetsgivaravgifter — {tp} % zu Lasten des Arbeitgebers (Rente, Kranken, Eltern, Unfall, Arbeitsmarkt, allgemeine Lohnsteuer).\n\nArbeitnehmerseite: die allmän pensionsavgift (7 %) wird durch eine Steuerermäßigung vollständig ausgeglichen (Nettoeffekt null) → nicht angezeigt.`
  - nl: `Arbetsgivaravgifter — {tp} % ten laste van de werkgever (pensioen, ziekte, ouderschap, ongeval, arbeidsmarkt, algemene loonheffing).\n\nWerknemerszijde: de allmän pensionsavgift (7 %) wordt volledig gecompenseerd door een belastingvermindering (netto nul) → niet getoond.`
  - it: `Arbetsgivaravgifter — {tp} % a carico del datore di lavoro (pensione, malattia, parentale, infortuni, mercato del lavoro, imposta generale sui salari).\n\nLato dipendente: l'allmän pensionsavgift (7 %) è interamente compensata da una riduzione d'imposta (effetto netto nullo) → non mostrata.`
  - es: `Arbetsgivaravgifter — {tp} % a cargo del empleador (pensión, enfermedad, parental, accidente, mercado laboral, impuesto general sobre salarios).\n\nLado del trabajador: la allmän pensionsavgift (7 %) se compensa íntegramente con una reducción fiscal (efecto neto nulo) → no se muestra.`

#### SE_SKATT
- **libelle** — fr: `Inkomstskatt — Impôt (communal + État)` · en: `Inkomstskatt — Tax (municipal + state)` · de: `Inkomstskatt — Steuer (Gemeinde + Staat)` · nl: `Inkomstskatt — Belasting (gemeente + staat)` · it: `Inkomstskatt — Imposta (comunale + statale)` · es: `Inkomstskatt — Impuesto (municipal + estatal)`
- **explication** (placeholders `{g} {co} {et} {im}`) :
  - fr: `Impôt sur le revenu 2025 (annualisé).\n\nRevenu annuel : {g} SEK\nImpôt communal moyen 32,41 % → {co} SEK\nImpôt d'État 20 % au-delà de 625 800 SEK/an → {et} SEK\n= {im} SEK/mois.\n\nNote : grundavdrag et jobbskatteavdrag non modélisés (net prudent).\nSource : Skatteverket.`
  - en: `Income tax 2025 (annualised).\n\nAnnual income: {g} SEK\nAverage municipal tax 32.41 % → {co} SEK\nState tax 20 % above 625,800 SEK/yr → {et} SEK\n= {im} SEK/month.\n\nNote: grundavdrag and jobbskatteavdrag not modelled (conservative net).\nSource: Skatteverket.`
  - de: `Einkommensteuer 2025 (auf Jahresbasis).\n\nJahreseinkommen: {g} SEK\nDurchschn. Gemeindesteuer 32,41 % → {co} SEK\nStaatssteuer 20 % über 625.800 SEK/Jahr → {et} SEK\n= {im} SEK/Monat.\n\nHinweis: grundavdrag und jobbskatteavdrag nicht modelliert (vorsichtiger Nettowert).\nQuelle: Skatteverket.`
  - nl: `Inkomstenbelasting 2025 (op jaarbasis).\n\nJaarinkomen: {g} SEK\nGemiddelde gemeentebelasting 32,41 % → {co} SEK\nRijksbelasting 20 % boven 625.800 SEK/jr → {et} SEK\n= {im} SEK/maand.\n\nNoot: grundavdrag en jobbskatteavdrag niet gemodelleerd (voorzichtig netto).\nBron: Skatteverket.`
  - it: `Imposta sul reddito 2025 (annualizzata).\n\nReddito annuo: {g} SEK\nImposta comunale media 32,41 % → {co} SEK\nImposta statale 20 % oltre 625.800 SEK/anno → {et} SEK\n= {im} SEK/mese.\n\nNota: grundavdrag e jobbskatteavdrag non modellati (netto prudente).\nFonte: Skatteverket.`
  - es: `Impuesto sobre la renta 2025 (anualizado).\n\nRenta anual: {g} SEK\nImpuesto municipal medio 32,41 % → {co} SEK\nImpuesto estatal 20 % por encima de 625.800 SEK/año → {et} SEK\n= {im} SEK/mes.\n\nNota: grundavdrag y jobbskatteavdrag no modelados (neto prudente).\nFuente: Skatteverket.`

### 🇷🇴 Roumanie

> Gabarit générique `ligne_cot` (placeholders `{libelle} {ts} {tp}`) — même phrase que Tchéquie/Estonie.

#### RO_CAS
- **libelle** — fr: `CAS — Pension` · en: `CAS — Pension` · de: `CAS — Rente` · nl: `CAS — Pensioen` · it: `CAS — Pensione` · es: `CAS — Pensión`

#### RO_CASS
- **libelle** — fr: `CASS — Assurance santé` · en: `CASS — Health insurance` · de: `CASS — Krankenversicherung` · nl: `CASS — Ziektekostenverzekering` · it: `CASS — Assicurazione malattia` · es: `CASS — Seguro de enfermedad`

#### RO_CAM
- **libelle** — fr: `CAM — Contribution de travail (employeur)` · en: `CAM — Work contribution (employer)` · de: `CAM — Arbeitsbeitrag (Arbeitgeber)` · nl: `CAM — Arbeidsbijdrage (werkgever)` · it: `CAM — Contributo lavoro (datore di lavoro)` · es: `CAM — Contribución laboral (empleador)`

#### RO_IMPOZIT
- **libelle** — fr: `Impozit pe venit — Impôt sur le revenu (10 %)` · en: `Impozit pe venit — Income tax (10 %)` · de: `Impozit pe venit — Einkommensteuer (10 %)` · nl: `Impozit pe venit — Inkomstenbelasting (10 %)` · it: `Impozit pe venit — Imposta sul reddito (10 %)` · es: `Impozit pe venit — Impuesto sobre la renta (10 %)`
- **explication** (placeholders `{annee} {b} {im}`) :
  - fr: `Impôt sur le revenu {annee} : 10 % proportionnel (flat tax depuis 2018).\n\nBase = brut − CAS 25 % − CASS 10 % = {b} RON → {im} RON/mois.\n\nNote : déduction personnelle (bas salaires) non modélisée (net prudent).\nSource : ANAF.`
  - en: `Income tax {annee}: flat 10 % (since 2018).\n\nBase = gross − CAS 25 % − CASS 10 % = {b} RON → {im} RON/month.\n\nNote: personal deduction (low wages) not modelled (conservative net).\nSource: ANAF.`
  - de: `Einkommensteuer {annee}: pauschal 10 % (seit 2018).\n\nBemessung = brutto − CAS 25 % − CASS 10 % = {b} RON → {im} RON/Monat.\n\nHinweis: persönlicher Abzug (niedrige Löhne) nicht modelliert (vorsichtiger Nettowert).\nQuelle: ANAF.`
  - nl: `Inkomstenbelasting {annee}: vlak 10 % (sinds 2018).\n\nGrondslag = bruto − CAS 25 % − CASS 10 % = {b} RON → {im} RON/maand.\n\nNoot: persoonlijke aftrek (lage lonen) niet gemodelleerd (voorzichtig netto).\nBron: ANAF.`
  - it: `Imposta sul reddito {annee}: proporzionale 10 % (dal 2018).\n\nBase = lordo − CAS 25 % − CASS 10 % = {b} RON → {im} RON/mese.\n\nNota: detrazione personale (bassi salari) non modellata (netto prudente).\nFonte: ANAF.`
  - es: `Impuesto sobre la renta {annee}: plano 10 % (desde 2018).\n\nBase = bruto − CAS 25 % − CASS 10 % = {b} RON → {im} RON/mes.\n\nNota: deducción personal (salarios bajos) no modelada (neto prudente).\nFuente: ANAF.`

### 🇵🇱 Pologne

> Gabarit générique `ligne` (placeholders `{libelle} {tsp} {tpp} {base} {ms} {mp}`) :
> fr `{libelle} — ZUS.\nTaux : {tsp} % sal / {tpp} % pat. Assiette : {base} PLN.\nSalarié : {ms} PLN | Employeur : {mp} PLN.\n\nBase légale : Ustawa o systemie ubezpieczeń społecznych.` ·
> en `{libelle} — ZUS.\nRate: {tsp} % empl / {tpp} % empr. Base: {base} PLN.\nEmployee: {ms} PLN | Employer: {mp} PLN.\n\nLegal basis: Ustawa o systemie ubezpieczeń społecznych.` ·
> de `{libelle} — ZUS.\nSatz: {tsp} % AN / {tpp} % AG. Grundlage: {base} PLN.\nArbeitnehmer: {ms} PLN | Arbeitgeber: {mp} PLN.\n\nRechtsgrundlage: Ustawa o systemie ubezpieczeń społecznych.` ·
> nl `{libelle} — ZUS.\nTarief: {tsp} % wn / {tpp} % wg. Grondslag: {base} PLN.\nWerknemer: {ms} PLN | Werkgever: {mp} PLN.\n\nWettelijke basis: Ustawa o systemie ubezpieczeń społecznych.` ·
> it `{libelle} — ZUS.\nAliquota: {tsp} % dip / {tpp} % dat. Base: {base} PLN.\nDipendente: {ms} PLN | Datore di lavoro: {mp} PLN.\n\nBase giuridica: Ustawa o systemie ubezpieczeń społecznych.` ·
> es `{libelle} — ZUS.\nTipo: {tsp} % trab / {tpp} % empr. Base: {base} PLN.\nTrabajador: {ms} PLN | Empleador: {mp} PLN.\n\nBase legal: Ustawa o systemie ubezpieczeń społecznych.`

#### PL_EMERYTALNE
- **libelle** — fr: `Emerytalne — Vieillesse` · en: `Emerytalne — Old-age` · de: `Emerytalne — Alter` · nl: `Emerytalne — Ouderdom` · it: `Emerytalne — Vecchiaia` · es: `Emerytalne — Vejez`

#### PL_RENTOWE
- **libelle** — fr: `Rentowe — Invalidité/décès` · en: `Rentowe — Disability/survivors` · de: `Rentowe — Invalidität/Hinterbliebene` · nl: `Rentowe — Invaliditeit/nabestaanden` · it: `Rentowe — Invalidità/superstiti` · es: `Rentowe — Invalidez/supervivencia`

#### PL_CHOROBOWE
- **libelle** — fr: `Chorobowe — Maladie` · en: `Chorobowe — Sickness` · de: `Chorobowe — Krankheit` · nl: `Chorobowe — Ziekte` · it: `Chorobowe — Malattia` · es: `Chorobowe — Enfermedad`

#### PL_WYPADKOWE
- **libelle** — fr: `Wypadkowe — Accidents (employeur)` · en: `Wypadkowe — Accidents (employer)` · de: `Wypadkowe — Unfälle (Arbeitgeber)` · nl: `Wypadkowe — Ongevallen (werkgever)` · it: `Wypadkowe — Infortuni (datore di lavoro)` · es: `Wypadkowe — Accidentes (empleador)`

#### PL_FP
- **libelle** — fr: `Fundusz Pracy (employeur)` · en: `Fundusz Pracy — Labour Fund (employer)` · de: `Fundusz Pracy — Arbeitsfonds (Arbeitgeber)` · nl: `Fundusz Pracy — Arbeidsfonds (werkgever)` · it: `Fundusz Pracy — Fondo lavoro (datore di lavoro)` · es: `Fundusz Pracy — Fondo de trabajo (empleador)`

#### PL_FGSP
- **libelle** — fr: `FGŚP (employeur)` · en: `FGŚP — Guaranteed Benefits Fund (employer)` · de: `FGŚP — Garantiefonds (Arbeitgeber)` · nl: `FGŚP — Garantiefonds (werkgever)` · it: `FGŚP — Fondo garanzia (datore di lavoro)` · es: `FGŚP — Fondo de garantía (empleador)`

#### PL_ZDROWOTNE
- **libelle** — fr: `Składka zdrowotna — Assurance maladie (9 %)` · en: `Składka zdrowotna — Health insurance (9 %)` · de: `Składka zdrowotna — Krankenversicherung (9 %)` · nl: `Składka zdrowotna — Ziektekostenverzekering (9 %)` · it: `Składka zdrowotna — Assicurazione malattia (9 %)` · es: `Składka zdrowotna — Seguro de enfermedad (9 %)`
- **explication** (placeholders `{b} {s}`) :
  - fr: `Składka zdrowotna — 9 % de l'assiette (brut − ZUS social salarial).\nAssiette : {b} PLN → {s} PLN/mois. Non déductible du PIT depuis 2022.\n\nBase légale : Ustawa o świadczeniach opieki zdrowotnej.`
  - en: `Składka zdrowotna — 9 % of the base (gross − employee social ZUS).\nBase: {b} PLN → {s} PLN/month. Non-deductible from PIT since 2022.\n\nLegal basis: Ustawa o świadczeniach opieki zdrowotnej.`
  - de: `Składka zdrowotna — 9 % der Grundlage (brutto − AN-Sozial-ZUS).\nGrundlage: {b} PLN → {s} PLN/Monat. Seit 2022 nicht von der PIT abzugsfähig.\n\nRechtsgrundlage: Ustawa o świadczeniach opieki zdrowotnej.`
  - nl: `Składka zdrowotna — 9 % van de grondslag (bruto − sociale ZUS werknemer).\nGrondslag: {b} PLN → {s} PLN/maand. Sinds 2022 niet aftrekbaar van PIT.\n\nWettelijke basis: Ustawa o świadczeniach opieki zdrowotnej.`
  - it: `Składka zdrowotna — 9 % della base (lordo − ZUS sociale dipendente).\nBase: {b} PLN → {s} PLN/mese. Non deducibile dal PIT dal 2022.\n\nBase giuridica: Ustawa o świadczeniach opieki zdrowotnej.`
  - es: `Składka zdrowotna — 9 % de la base (bruto − ZUS social del trabajador).\nBase: {b} PLN → {s} PLN/mes. No deducible del PIT desde 2022.\n\nBase legal: Ustawa o świadczeniach opieki zdrowotnej.`

#### PL_PIT
- **libelle** — fr: `PIT — Impôt sur le revenu` · en: `PIT — Income tax` · de: `PIT — Einkommensteuer` · nl: `PIT — Inkomstenbelasting` · it: `PIT — Imposta sul reddito` · es: `PIT — Impuesto sobre la renta`
- **explication** (placeholders `{ba} {za} {kup} {tx} {pa} {pm}`) :
  - fr: `Impôt sur le revenu (PIT) 2025 — annualisé.\n\nRevenu annuel : {ba} PLN − ZUS social {za} PLN − KUP {kup} PLN\n= base imposable {tx} PLN\nBarème : 12 % jusqu'à 120 000 PLN, 32 % au-delà ; − montant réducteur 3 600 PLN.\n= {pa} PLN/an / 12 = {pm} PLN/mois.\n\nBase légale : Ustawa o PIT.`
  - en: `Income tax (PIT) 2025 — annualised.\n\nAnnual income: {ba} PLN − social ZUS {za} PLN − KUP {kup} PLN\n= taxable base {tx} PLN\nScale: 12 % up to 120,000 PLN, 32 % above; − tax-reducing amount 3,600 PLN.\n= {pa} PLN/yr / 12 = {pm} PLN/month.\n\nLegal basis: Ustawa o PIT.`
  - de: `Einkommensteuer (PIT) 2025 — auf Jahresbasis.\n\nJahreseinkommen: {ba} PLN − Sozial-ZUS {za} PLN − KUP {kup} PLN\n= Bemessungsgrundlage {tx} PLN\nTarif: 12 % bis 120.000 PLN, 32 % darüber; − Steuerminderungsbetrag 3.600 PLN.\n= {pa} PLN/Jahr / 12 = {pm} PLN/Monat.\n\nRechtsgrundlage: Ustawa o PIT.`
  - nl: `Inkomstenbelasting (PIT) 2025 — op jaarbasis.\n\nJaarinkomen: {ba} PLN − sociale ZUS {za} PLN − KUP {kup} PLN\n= belastbare grondslag {tx} PLN\nSchaal: 12 % tot 120.000 PLN, 32 % daarboven; − belastingverlagend bedrag 3.600 PLN.\n= {pa} PLN/jr / 12 = {pm} PLN/maand.\n\nWettelijke basis: Ustawa o PIT.`
  - it: `Imposta sul reddito (PIT) 2025 — annualizzata.\n\nReddito annuo: {ba} PLN − ZUS sociale {za} PLN − KUP {kup} PLN\n= base imponibile {tx} PLN\nScala: 12 % fino a 120.000 PLN, 32 % oltre; − importo riduttore 3.600 PLN.\n= {pa} PLN/anno / 12 = {pm} PLN/mese.\n\nBase giuridica: Ustawa o PIT.`
  - es: `Impuesto sobre la renta (PIT) 2025 — anualizado.\n\nRenta anual: {ba} PLN − ZUS social {za} PLN − KUP {kup} PLN\n= base imponible {tx} PLN\nEscala: 12 % hasta 120.000 PLN, 32 % por encima; − importe reductor 3.600 PLN.\n= {pa} PLN/año / 12 = {pm} PLN/mes.\n\nBase legal: Ustawa o PIT.`

### 🇸🇰 Slovaquie

#### SK_ZDRAVOTNE
- **libelle** — fr: `Zdravotné poistenie — Assurance maladie` · en: `Zdravotné poistenie — Health insurance` · de: `Zdravotné poistenie — Krankenversicherung` · nl: `Zdravotné poistenie — Ziektekostenverzekering` · it: `Zdravotné poistenie — Assicurazione malattia` · es: `Zdravotné poistenie — Seguro de enfermedad`
- **explication** (placeholders `{ts} {tp}`) :
  - fr: `Assurance maladie — salarié {ts} % / employeur {tp} %.`
  - en: `Health insurance — employee {ts} % / employer {tp} %.`
  - de: `Krankenversicherung — Arbeitnehmer {ts} % / Arbeitgeber {tp} %.`
  - nl: `Ziektekostenverzekering — werknemer {ts} % / werkgever {tp} %.`
  - it: `Assicurazione malattia — dipendente {ts} % / datore di lavoro {tp} %.`
  - es: `Seguro de enfermedad — trabajador {ts} % / empleador {tp} %.`

#### SK_SOCIALNE
- **libelle** — fr: `Sociálne poistenie — Sécurité sociale` · en: `Sociálne poistenie — Social security` · de: `Sociálne poistenie — Sozialversicherung` · nl: `Sociálne poistenie — Sociale zekerheid` · it: `Sociálne poistenie — Sicurezza sociale` · es: `Sociálne poistenie — Seguridad social`
- **explication** (placeholders `{ts} {tp}`) :
  - fr: `Sécurité sociale — salarié {ts} % / employeur {tp} %. Assiette plafonnée à 15 730 €/mois.`
  - en: `Social security — employee {ts} % / employer {tp} %. Base capped at 15,730 €/month.`
  - de: `Sozialversicherung — Arbeitnehmer {ts} % / Arbeitgeber {tp} %. Bemessungsgrundlage gedeckelt auf 15.730 €/Monat.`
  - nl: `Sociale zekerheid — werknemer {ts} % / werkgever {tp} %. Grondslag begrensd op 15.730 €/maand.`
  - it: `Sicurezza sociale — dipendente {ts} % / datore di lavoro {tp} %. Base limitata a 15.730 €/mese.`
  - es: `Seguridad social — trabajador {ts} % / empleador {tp} %. Base limitada a 15.730 €/mes.`

#### SK_DAN
- **libelle** — fr: `Daň z príjmov — Impôt sur le revenu` · en: `Daň z príjmov — Income tax` · de: `Daň z príjmov — Einkommensteuer` · nl: `Daň z príjmov — Inkomstenbelasting` · it: `Daň z príjmov — Imposta sul reddito` · es: `Daň z príjmov — Impuesto sobre la renta`
- **explication** (placeholders `{b} {im}`) :
  - fr: `Impôt sur le revenu 2025.\n\nBase = brut − cotisations salariales − part non imposable 479,48 € = {b} €\n19 % jusqu'à 4 036,79 €/mois, 25 % au-delà → {im} €/mois.\n\nNote : dégressivité de la part non imposable non modélisée (net prudent).\nSource : Finančná správa.`
  - en: `Income tax 2025.\n\nBase = gross − employee contributions − tax-free part 479.48 € = {b} €\n19 % up to 4,036.79 €/month, 25 % above → {im} €/month.\n\nNote: tapering of the tax-free part not modelled (conservative net).\nSource: Finančná správa.`
  - de: `Einkommensteuer 2025.\n\nBemessung = brutto − AN-Beiträge − steuerfreier Teil 479,48 € = {b} €\n19 % bis 4.036,79 €/Monat, 25 % darüber → {im} €/Monat.\n\nHinweis: Abschmelzung des steuerfreien Teils nicht modelliert (vorsichtiger Nettowert).\nQuelle: Finančná správa.`
  - nl: `Inkomstenbelasting 2025.\n\nGrondslag = bruto − werknemersbijdragen − belastingvrij deel 479,48 € = {b} €\n19 % tot 4.036,79 €/maand, 25 % daarboven → {im} €/maand.\n\nNoot: afbouw van het belastingvrije deel niet gemodelleerd (voorzichtig netto).\nBron: Finančná správa.`
  - it: `Imposta sul reddito 2025.\n\nBase = lordo − contributi dipendente − parte esente 479,48 € = {b} €\n19 % fino a 4.036,79 €/mese, 25 % oltre → {im} €/mese.\n\nNota: decrescenza della parte esente non modellata (netto prudente).\nFonte: Finančná správa.`
  - es: `Impuesto sobre la renta 2025.\n\nBase = bruto − cotizaciones del trabajador − parte exenta 479,48 € = {b} €\n19 % hasta 4.036,79 €/mes, 25 % por encima → {im} €/mes.\n\nNota: decrecimiento de la parte exenta no modelado (neto prudente).\nFuente: Finančná správa.`

### 🇳🇿 Nouvelle-Zélande

#### NZ_PAYE
- **libelle** — fr: `PAYE — Impôt sur le revenu` · en: `PAYE — Income tax` · de: `PAYE — Einkommensteuer` · nl: `PAYE — Inkomstenbelasting` · it: `PAYE — Imposta sul reddito` · es: `PAYE — Impuesto sobre la renta`
- **explication** (placeholders `{fy0} {fy1} {rev} {imp} {mens}`) :
  - fr: `Impôt sur le revenu (PAYE) — année fiscale {fy0}-{fy1}, sans tranche exonérée.\n\nRevenu annuel estimé : {rev} $ → {imp} $/an / 12 = {mens} $/mois.\n\nBase légale : Income Tax Act 2007.`
  - en: `Income tax (PAYE) — fiscal year {fy0}-{fy1}, no tax-free band.\n\nEstimated annual income: {rev} $ → {imp} $/yr / 12 = {mens} $/month.\n\nLegal basis: Income Tax Act 2007.`
  - de: `Einkommensteuer (PAYE) — Steuerjahr {fy0}-{fy1}, ohne Freibetrag.\n\nGeschätztes Jahreseinkommen: {rev} $ → {imp} $/Jahr / 12 = {mens} $/Monat.\n\nRechtsgrundlage: Income Tax Act 2007.`
  - nl: `Inkomstenbelasting (PAYE) — belastingjaar {fy0}-{fy1}, zonder belastingvrije schijf.\n\nGeschat jaarinkomen: {rev} $ → {imp} $/jr / 12 = {mens} $/maand.\n\nWettelijke basis: Income Tax Act 2007.`
  - it: `Imposta sul reddito (PAYE) — anno fiscale {fy0}-{fy1}, senza fascia esente.\n\nReddito annuo stimato: {rev} $ → {imp} $/anno / 12 = {mens} $/mese.\n\nBase giuridica: Income Tax Act 2007.`
  - es: `Impuesto sobre la renta (PAYE) — año fiscal {fy0}-{fy1}, sin tramo exento.\n\nRenta anual estimada: {rev} $ → {imp} $/año / 12 = {mens} $/mes.\n\nBase legal: Income Tax Act 2007.`

#### NZ_ACC
- **libelle** — fr: `ACC earner's levy — Assurance accidents` · en: `ACC earner's levy — Accident insurance` · de: `ACC earner's levy — Unfallversicherung` · nl: `ACC earner's levy — Ongevallenverzekering` · it: `ACC earner's levy — Assicurazione infortuni` · es: `ACC earner's levy — Seguro de accidentes`
- **explication** (placeholders `{t} {fy0} {fy1} {cap} {m}`) :
  - fr: `ACC earner's levy — couverture accidents, {t} % du salaire brut (année {fy0}-{fy1}).\nAssiette plafonnée à {cap} $/an. Montant : {m} $/mois.\n\nBase légale : Accident Compensation Act 2001.`
  - en: `ACC earner's levy — accident cover, {t} % of gross salary (year {fy0}-{fy1}).\nBase capped at {cap} $/yr. Amount: {m} $/month.\n\nLegal basis: Accident Compensation Act 2001.`
  - de: `ACC earner's levy — Unfallschutz, {t} % des Bruttolohns (Jahr {fy0}-{fy1}).\nBemessung gedeckelt auf {cap} $/Jahr. Betrag: {m} $/Monat.\n\nRechtsgrundlage: Accident Compensation Act 2001.`
  - nl: `ACC earner's levy — ongevallendekking, {t} % van het brutoloon (jaar {fy0}-{fy1}).\nGrondslag begrensd op {cap} $/jr. Bedrag: {m} $/maand.\n\nWettelijke basis: Accident Compensation Act 2001.`
  - it: `ACC earner's levy — copertura infortuni, {t} % della retribuzione lorda (anno {fy0}-{fy1}).\nBase limitata a {cap} $/anno. Importo: {m} $/mese.\n\nBase giuridica: Accident Compensation Act 2001.`
  - es: `ACC earner's levy — cobertura de accidentes, {t} % del salario bruto (año {fy0}-{fy1}).\nBase limitada a {cap} $/año. Importe: {m} $/mes.\n\nBase legal: Accident Compensation Act 2001.`

#### NZ_KIWISAVER_EMP
- **libelle** — fr: `KiwiSaver — Retraite (employeur, défaut 3 %)` · en: `KiwiSaver — Pension (employer, default 3 %)` · de: `KiwiSaver — Rente (Arbeitgeber, Standard 3 %)` · nl: `KiwiSaver — Pensioen (werkgever, standaard 3 %)` · it: `KiwiSaver — Pensione (datore di lavoro, predefinito 3 %)` · es: `KiwiSaver — Pensión (empleador, por defecto 3 %)`
- **explication** (placeholders `{t} {mp}`) :
  - fr: `KiwiSaver — épargne-retraite, cotisation employeur par défaut {t} %, versée en sus.\nOptionnelle selon adhésion du salarié.\nEmployeur : {mp} $/mois.\n\nBase légale : KiwiSaver Act 2006.`
  - en: `KiwiSaver — retirement savings, default employer contribution {t} %, paid on top.\nOptional depending on employee enrolment.\nEmployer: {mp} $/month.\n\nLegal basis: KiwiSaver Act 2006.`
  - de: `KiwiSaver — Altersvorsorge, Arbeitgeberbeitrag standardmäßig {t} %, zusätzlich gezahlt.\nOptional je nach Beitritt des Arbeitnehmers.\nArbeitgeber: {mp} $/Monat.\n\nRechtsgrundlage: KiwiSaver Act 2006.`
  - nl: `KiwiSaver — pensioensparen, standaard werkgeversbijdrage {t} %, bovenop betaald.\nOptioneel afhankelijk van aanmelding werknemer.\nWerkgever: {mp} $/maand.\n\nWettelijke basis: KiwiSaver Act 2006.`
  - it: `KiwiSaver — risparmio pensionistico, contributo datore di lavoro predefinito {t} %, versato in aggiunta.\nOpzionale secondo l'adesione del dipendente.\nDatore di lavoro: {mp} $/mese.\n\nBase giuridica: KiwiSaver Act 2006.`
  - es: `KiwiSaver — ahorro para la jubilación, cotización del empleador por defecto {t} %, pagada adicionalmente.\nOpcional según la adhesión del trabajador.\nEmpleador: {mp} $/mes.\n\nBase legal: KiwiSaver Act 2006.`

### 🇳🇱 Pays-Bas

> `nl_bulletin` est déjà partiellement i18n-conscient (message de lacune). Les lignes patronales
> et le message « non couvert » restent en français.

#### NL — gabarit générique cotisation patronale (`cotisation_pat`)
- **explication** (placeholders `{libelle} {tp} {base} {plaf} {mp}`) :
  - fr: `{libelle} — premie patronale (werkgeversheffing).\n\nTaux : {tp} %\nAssiette : {base} € (min(brut, maximumpremieloon mensuel {plaf} €))\nEmployeur : {mp} €\n\nBase légale : Wfsv / Zorgverzekeringswet.`
  - en: `{libelle} — employer premium (werkgeversheffing).\n\nRate: {tp} %\nBase: {base} € (min(gross, monthly maximumpremieloon {plaf} €))\nEmployer: {mp} €\n\nLegal basis: Wfsv / Zorgverzekeringswet.`
  - de: `{libelle} — Arbeitgeberprämie (werkgeversheffing).\n\nSatz: {tp} %\nGrundlage: {base} € (min(brutto, monatliches maximumpremieloon {plaf} €))\nArbeitgeber: {mp} €\n\nRechtsgrundlage: Wfsv / Zorgverzekeringswet.`
  - nl: `{libelle} — werkgeverspremie (werkgeversheffing).\n\nTarief: {tp} %\nGrondslag: {base} € (min(bruto, maandelijks maximumpremieloon {plaf} €))\nWerkgever: {mp} €\n\nWettelijke basis: Wfsv / Zorgverzekeringswet.`
  - it: `{libelle} — premio datoriale (werkgeversheffing).\n\nAliquota: {tp} %\nBase: {base} € (min(lordo, maximumpremieloon mensile {plaf} €))\nDatore di lavoro: {mp} €\n\nBase giuridica: Wfsv / Zorgverzekeringswet.`
  - es: `{libelle} — prima patronal (werkgeversheffing).\n\nTipo: {tp} %\nBase: {base} € (mín(bruto, maximumpremieloon mensual {plaf} €))\nEmpleador: {mp} €\n\nBase legal: Wfsv / Zorgverzekeringswet.`

#### Libellés NL (patronaux)
- `NL_ZVW` — fr: `Zvw — Assurance santé` · en: `Zvw — Health insurance` · de: `Zvw — Krankenversicherung` · nl: `Zvw — Zorgverzekering` · it: `Zvw — Assicurazione sanitaria` · es: `Zvw — Seguro de salud`
- `NL_AWF` — fr: `AWf — Chômage (WW)` · en: `AWf — Unemployment (WW)` · de: `AWf — Arbeitslosigkeit (WW)` · nl: `AWf — Werkloosheid (WW)` · it: `AWf — Disoccupazione (WW)` · es: `AWf — Desempleo (WW)`
- `NL_AOF` — fr: `Aof — Invalidité (WIA)` · en: `Aof — Disability (WIA)` · de: `Aof — Invalidität (WIA)` · nl: `Aof — Arbeidsongeschiktheid (WIA)` · it: `Aof — Invalidità (WIA)` · es: `Aof — Invalidez (WIA)`
- `NL_WHK` — fr: `Whk — WGA + Ziektewet` (identique dans les 6 langues, termes propres)
- `NL_OPSLAG_KO` — fr: `Opslag kinderopvang` · en: `Childcare surcharge (Opslag kinderopvang)` · de: `Kinderbetreuungszuschlag (Opslag kinderopvang)` · nl: `Opslag kinderopvang` · it: `Supplemento asili nido (Opslag kinderopvang)` · es: `Recargo guardería (Opslag kinderopvang)`

#### NL_LOONHEFFING
> Ligne salariale produite par `nl_loonheffing.rs` (libellé + explication encore en français, à
> extraire lors du câblage : barème box 1 + heffingskortingen). À traduire avec placeholders
> identiques. *(Détail à compléter à la lecture de la fin de `nl_loonheffing.rs`.)*

#### NL_NON_COUVERT (message de lacune)
- **libelle** — fr: `Pays-Bas — données indisponibles pour cette année` · en: `Netherlands — data unavailable for this year` · de: `Niederlande — Daten für dieses Jahr nicht verfügbar` · nl: `Nederland — gegevens niet beschikbaar voor dit jaar` · it: `Paesi Bassi — dati non disponibili per quest'anno` · es: `Países Bajos — datos no disponibles para este año`
- **explication** (placeholders `{annee}`) :
  - fr: `Les données néerlandaises ne sont disponibles que pour 2026 (pilote).\nL'année {annee} sera ajoutée après sourcing officiel (Belastingdienst).\nAucun chiffre n'est inventé en l'absence de source.`
  - en: `Dutch data is only available for 2026 (pilot).\nYear {annee} will be added after official sourcing (Belastingdienst).\nNo figure is invented in the absence of a source.`
  - de: `Niederländische Daten sind nur für 2026 verfügbar (Pilot).\nDas Jahr {annee} wird nach offizieller Quellenprüfung (Belastingdienst) ergänzt.\nOhne Quelle wird keine Zahl erfunden.`
  - nl: `Nederlandse gegevens zijn alleen beschikbaar voor 2026 (pilot).\nHet jaar {annee} wordt toegevoegd na officiële bronvermelding (Belastingdienst).\nZonder bron wordt geen cijfer verzonnen.`
  - it: `I dati olandesi sono disponibili solo per il 2026 (pilota).\nL'anno {annee} sarà aggiunto dopo reperimento ufficiale (Belastingdienst).\nNessuna cifra è inventata in assenza di fonte.`
  - es: `Los datos neerlandeses solo están disponibles para 2026 (piloto).\nEl año {annee} se añadirá tras el sourcing oficial (Belastingdienst).\nNo se inventa ninguna cifra en ausencia de fuente.`

### 🇲🇨 Monaco

> Gabarit générique `ligne` (placeholders `{libelle} {ts} {tp} {ms}`) :
> fr `{libelle} — Caisses Sociales de Monaco.\nSalarié {ts} % / employeur {tp} %. Salarié : {ms} €.\n\nNote : Monaco ne prélève pas d'impôt sur le revenu des résidents (sauf nationaux français — convention fiscale 1963).` ·
> en `{libelle} — Caisses Sociales de Monaco.\nEmployee {ts} % / employer {tp} %. Employee: {ms} €.\n\nNote: Monaco levies no income tax on residents (except French nationals — 1963 tax treaty).` ·
> de `{libelle} — Caisses Sociales de Monaco.\nArbeitnehmer {ts} % / Arbeitgeber {tp} %. Arbeitnehmer: {ms} €.\n\nHinweis: Monaco erhebt keine Einkommensteuer auf Gebietsansässige (außer französische Staatsangehörige — Steuerabkommen 1963).` ·
> nl `{libelle} — Caisses Sociales de Monaco.\nWerknemer {ts} % / werkgever {tp} %. Werknemer: {ms} €.\n\nNoot: Monaco heft geen inkomstenbelasting op inwoners (behalve Franse staatsburgers — belastingverdrag 1963).` ·
> it `{libelle} — Caisses Sociales de Monaco.\nDipendente {ts} % / datore di lavoro {tp} %. Dipendente: {ms} €.\n\nNota: Monaco non preleva imposte sul reddito dei residenti (salvo cittadini francesi — convenzione fiscale 1963).` ·
> es `{libelle} — Caisses Sociales de Monaco.\nTrabajador {ts} % / empleador {tp} %. Trabajador: {ms} €.\n\nNota: Mónaco no grava la renta de los residentes (salvo nacionales franceses — convenio fiscal de 1963).`

- `MC_CAR` — fr: `CAR — Retraite` · en: `CAR — Pension` · de: `CAR — Rente` · nl: `CAR — Pensioen` · it: `CAR — Pensione` · es: `CAR — Pensión`
- `MC_CCSS` — fr: `CCSS — Maladie/famille` · en: `CCSS — Health/family` · de: `CCSS — Kranken/Familie` · nl: `CCSS — Ziekte/gezin` · it: `CCSS — Malattia/famiglia` · es: `CCSS — Enfermedad/familia`
- `MC_CHOM` — fr: `Chômage` · en: `Unemployment` · de: `Arbeitslosigkeit` · nl: `Werkloosheid` · it: `Disoccupazione` · es: `Desempleo`

### 🇨🇳 Chine

> Gabarit générique `ligne` (placeholders `{expl} {base} {brut} {min} {max} {ts_pct} {ms} {tp_pct} {mp}`) :
> fr `{expl}\nBase clampée : ¥{base} (brut ¥{brut}, min ¥{min}–max ¥{max})\nSalarié : {ts_pct} % = ¥{ms} | Employeur : {tp_pct} % = ¥{mp}` ·
> en `{expl}\nClamped base: ¥{base} (gross ¥{brut}, min ¥{min}–max ¥{max})\nEmployee: {ts_pct} % = ¥{ms} | Employer: {tp_pct} % = ¥{mp}` ·
> de `{expl}\nBegrenzte Grundlage: ¥{base} (brutto ¥{brut}, min ¥{min}–max ¥{max})\nArbeitnehmer: {ts_pct} % = ¥{ms} | Arbeitgeber: {tp_pct} % = ¥{mp}` ·
> nl `{expl}\nBegrensde grondslag: ¥{base} (bruto ¥{brut}, min ¥{min}–max ¥{max})\nWerknemer: {ts_pct} % = ¥{ms} | Werkgever: {tp_pct} % = ¥{mp}` ·
> it `{expl}\nBase limitata: ¥{base} (lordo ¥{brut}, min ¥{min}–max ¥{max})\nDipendente: {ts_pct} % = ¥{ms} | Datore di lavoro: {tp_pct} % = ¥{mp}` ·
> es `{expl}\nBase limitada: ¥{base} (bruto ¥{brut}, mín ¥{min}–máx ¥{max})\nTrabajador: {ts_pct} % = ¥{ms} | Empleador: {tp_pct} % = ¥{mp}`
>
> Le sous-texte `{expl}` est l'une des phrases ci-dessous (par cotisation).

- `CN_YANGLAO` — libelle fr `养老保险 — Assurance retraite` · en `养老保险 — Pension insurance` · de `养老保险 — Rentenversicherung` · nl `养老保险 — Pensioenverzekering` · it `养老保险 — Assicurazione pensione` · es `养老保险 — Seguro de pensión`
  - expl `{expl}` — fr: `Cotisation retraite obligatoire. Sal 8 % + pat 16 % = 24 % total. 社会保险法 art. 12.` · en: `Mandatory pension contribution. Empl 8 % + empr 16 % = 24 % total. 社会保险法 art. 12.` · de: `Pflicht-Rentenbeitrag. AN 8 % + AG 16 % = 24 % gesamt. 社会保险法 Art. 12.` · nl: `Verplichte pensioenbijdrage. Wn 8 % + wg 16 % = 24 % totaal. 社会保险法 art. 12.` · it: `Contributo pensione obbligatorio. Dip 8 % + dat 16 % = 24 % totale. 社会保险法 art. 12.` · es: `Cotización de pensión obligatoria. Trab 8 % + empr 16 % = 24 % total. 社会保险法 art. 12.`
- `CN_YILIAO` — libelle fr `医疗保险 — Assurance maladie` · en `医疗保险 — Health insurance` · de `医疗保险 — Krankenversicherung` · nl `医疗保险 — Ziektekostenverzekering` · it `医疗保险 — Assicurazione malattia` · es `医疗保险 — Seguro de enfermedad`
  - expl `{expl}` — fr: `Assurance maladie. Sal 2 % + pat 8 % = 10 % total. 社会保险法 art. 23.` · en: `Health insurance. Empl 2 % + empr 8 % = 10 % total. 社会保险法 art. 23.` · de: `Krankenversicherung. AN 2 % + AG 8 % = 10 % gesamt. 社会保险法 Art. 23.` · nl: `Ziektekostenverzekering. Wn 2 % + wg 8 % = 10 % totaal. 社会保险法 art. 23.` · it: `Assicurazione malattia. Dip 2 % + dat 8 % = 10 % totale. 社会保险法 art. 23.` · es: `Seguro de enfermedad. Trab 2 % + empr 8 % = 10 % total. 社会保险法 art. 23.`
- `CN_SHIYE` — libelle fr `失业保险 — Assurance chômage` · en `失业保险 — Unemployment insurance` · de `失业保险 — Arbeitslosenversicherung` · nl `失业保险 — Werkloosheidsverzekering` · it `失业保险 — Assicurazione disoccupazione` · es `失业保险 — Seguro de desempleo`
  - expl `{expl}` — fr: `Assurance chômage. Sal 0,5 % + pat 0,5 % = 1 % total. 社会保险法 art. 44.` · en: `Unemployment insurance. Empl 0.5 % + empr 0.5 % = 1 % total. 社会保险法 art. 44.` · de: `Arbeitslosenversicherung. AN 0,5 % + AG 0,5 % = 1 % gesamt. 社会保险法 Art. 44.` · nl: `Werkloosheidsverzekering. Wn 0,5 % + wg 0,5 % = 1 % totaal. 社会保险法 art. 44.` · it: `Assicurazione disoccupazione. Dip 0,5 % + dat 0,5 % = 1 % totale. 社会保险法 art. 44.` · es: `Seguro de desempleo. Trab 0,5 % + empr 0,5 % = 1 % total. 社会保险法 art. 44.`
- `CN_GONGSHANG` — libelle fr `工伤保险 — Accidents du travail` · en `工伤保险 — Work accidents` · de `工伤保险 — Arbeitsunfälle` · nl `工伤保险 — Arbeidsongevallen` · it `工伤保险 — Infortuni sul lavoro` · es `工伤保险 — Accidentes laborales`
  - expl `{expl}` — fr: `100 % patronale. Taux Pékin général 0,4 %. 社会保险法 art. 33.` · en: `100 % employer. Beijing general rate 0.4 %. 社会保险法 art. 33.` · de: `100 % Arbeitgeber. Pekinger Allgemeinsatz 0,4 %. 社会保险法 Art. 33.` · nl: `100 % werkgever. Algemeen tarief Peking 0,4 %. 社会保险法 art. 33.` · it: `100 % datore di lavoro. Aliquota generale Pechino 0,4 %. 社会保险法 art. 33.` · es: `100 % empleador. Tipo general Pekín 0,4 %. 社会保险法 art. 33.`
- `CN_SHENGYU` — libelle fr `生育保险 — Assurance maternité` · en `生育保险 — Maternity insurance` · de `生育保险 — Mutterschaftsversicherung` · nl `生育保险 — Moederschapsverzekering` · it `生育保险 — Assicurazione maternità` · es `生育保险 — Seguro de maternidad`
  - expl `{expl}` — fr: `100 % patronale. Taux Pékin 0,8 %. 社会保险法 art. 53.` · en: `100 % employer. Beijing rate 0.8 %. 社会保险法 art. 53.` · de: `100 % Arbeitgeber. Pekinger Satz 0,8 %. 社会保险法 Art. 53.` · nl: `100 % werkgever. Tarief Peking 0,8 %. 社会保险法 art. 53.` · it: `100 % datore di lavoro. Aliquota Pechino 0,8 %. 社会保险法 art. 53.` · es: `100 % empleador. Tipo Pekín 0,8 %. 社会保险法 art. 53.`
- `CN_GONGJIJIN` — libelle fr `住房公积金 — Fonds de logement obligatoire` · en `住房公积金 — Mandatory housing fund` · de `住房公积金 — Pflicht-Wohnungsfonds` · nl `住房公积金 — Verplicht huisvestingsfonds` · it `住房公积金 — Fondo casa obbligatorio` · es `住房公积金 — Fondo de vivienda obligatorio`
  - expl `{expl}` — fr: `Fonds logement : sal 12 % + pat 12 % = 24 % total. Pékin 2024. Épargne individuelle disponible pour achat/loyer. 住房公积金管理条例.` · en: `Housing fund: empl 12 % + empr 12 % = 24 % total. Beijing 2024. Individual savings available for purchase/rent. 住房公积金管理条例.` · de: `Wohnungsfonds: AN 12 % + AG 12 % = 24 % gesamt. Peking 2024. Individuelles Guthaben für Kauf/Miete verfügbar. 住房公积金管理条例.` · nl: `Huisvestingsfonds: wn 12 % + wg 12 % = 24 % totaal. Peking 2024. Individueel spaargeld beschikbaar voor koop/huur. 住房公积金管理条例.` · it: `Fondo casa: dip 12 % + dat 12 % = 24 % totale. Pechino 2024. Risparmio individuale disponibile per acquisto/affitto. 住房公积金管理条例.` · es: `Fondo de vivienda: trab 12 % + empr 12 % = 24 % total. Pekín 2024. Ahorro individual disponible para compra/alquiler. 住房公积金管理条例.`

#### CN_IIT
- **libelle** — fr: `个人所得税 — Impôt sur le revenu (IIT)` · en: `个人所得税 — Income tax (IIT)` · de: `个人所得税 — Einkommensteuer (IIT)` · nl: `个人所得税 — Inkomstenbelasting (IIT)` · it: `个人所得税 — Imposta sul reddito (IIT)` · es: `个人所得税 — Impuesto sobre la renta (IIT)`
- **explication** (placeholders `{brut} {cot} {dp} {bm} {ba} {ia} {mens} {teff}`) :
  - fr: `个人所得税 — impôt sur le revenu (réforme 2018).\n\nBrut mensuel : ¥{brut}\n− Cotisations sociales sal : ¥{cot}\n− Déduction personnelle : ¥{dp}/mois\n= Base mensuelle imposable : ¥{bm}\n× 12 = Base annuelle : ¥{ba}\n\nIIT annuel (tranches 3/10/20/25/30/35/45 %) : ¥{ia}\nRetenue mensuelle : ¥{ia} / 12 = ¥{mens}\nTaux effectif mensuel : {teff} %\n\nBase légale : 个人所得税法 (L. 31/08/2018) ; 国税发〔2018〕164号.`
  - en: `个人所得税 — income tax (2018 reform).\n\nMonthly gross: ¥{brut}\n− Employee social contributions: ¥{cot}\n− Personal deduction: ¥{dp}/month\n= Monthly taxable base: ¥{bm}\n× 12 = Annual base: ¥{ba}\n\nAnnual IIT (brackets 3/10/20/25/30/35/45 %): ¥{ia}\nMonthly withholding: ¥{ia} / 12 = ¥{mens}\nEffective monthly rate: {teff} %\n\nLegal basis: 个人所得税法 (Law 31/08/2018); 国税发〔2018〕164号.`
  - de: `个人所得税 — Einkommensteuer (Reform 2018).\n\nMonatsbrutto: ¥{brut}\n− AN-Sozialbeiträge: ¥{cot}\n− Persönlicher Abzug: ¥{dp}/Monat\n= Monatliche Bemessungsgrundlage: ¥{bm}\n× 12 = Jahresgrundlage: ¥{ba}\n\nJahres-IIT (Stufen 3/10/20/25/30/35/45 %): ¥{ia}\nMonatlicher Einbehalt: ¥{ia} / 12 = ¥{mens}\nEffektiver Monatssatz: {teff} %\n\nRechtsgrundlage: 个人所得税法 (Gesetz 31.08.2018); 国税发〔2018〕164号.`
  - nl: `个人所得税 — inkomstenbelasting (hervorming 2018).\n\nMaandbruto: ¥{brut}\n− Sociale bijdragen werknemer: ¥{cot}\n− Persoonlijke aftrek: ¥{dp}/maand\n= Maandelijkse belastbare grondslag: ¥{bm}\n× 12 = Jaargrondslag: ¥{ba}\n\nJaarlijkse IIT (schijven 3/10/20/25/30/35/45 %): ¥{ia}\nMaandelijkse inhouding: ¥{ia} / 12 = ¥{mens}\nEffectief maandtarief: {teff} %\n\nWettelijke basis: 个人所得税法 (wet 31-08-2018); 国税发〔2018〕164号.`
  - it: `个人所得税 — imposta sul reddito (riforma 2018).\n\nLordo mensile: ¥{brut}\n− Contributi sociali dipendente: ¥{cot}\n− Detrazione personale: ¥{dp}/mese\n= Base mensile imponibile: ¥{bm}\n× 12 = Base annua: ¥{ba}\n\nIIT annua (scaglioni 3/10/20/25/30/35/45 %): ¥{ia}\nRitenuta mensile: ¥{ia} / 12 = ¥{mens}\nAliquota effettiva mensile: {teff} %\n\nBase giuridica: 个人所得税法 (legge 31/08/2018); 国税发〔2018〕164号.`
  - es: `个人所得税 — impuesto sobre la renta (reforma 2018).\n\nBruto mensual: ¥{brut}\n− Cotizaciones sociales del trabajador: ¥{cot}\n− Deducción personal: ¥{dp}/mes\n= Base mensual imponible: ¥{bm}\n× 12 = Base anual: ¥{ba}\n\nIIT anual (tramos 3/10/20/25/30/35/45 %): ¥{ia}\nRetención mensual: ¥{ia} / 12 = ¥{mens}\nTipo efectivo mensual: {teff} %\n\nBase legal: 个人所得税法 (ley 31/08/2018); 国税发〔2018〕164号.`

### 🇬🇧 Royaume-Uni

> Libellés avec placeholder `{fy}` = exercice fiscal « {annee}/{yy} ».

#### UK_NI_SAL
- **libelle** — fr: `National Insurance Class 1 — salarié {fy}` · en: `National Insurance Class 1 — employee {fy}` · de: `National Insurance Class 1 — Arbeitnehmer {fy}` · nl: `National Insurance Class 1 — werknemer {fy}` · it: `National Insurance Class 1 — dipendente {fy}` · es: `National Insurance Class 1 — trabajador {fy}`
- **explication** (placeholders `{ts_pct} {pt} {uel} {tp} {th} {m1} {m2} {tot} {teff}`) :
  - fr: `National Insurance Class 1 — part salariale.\n\nTranche [PT – UEL] ({ts_pct} %) : £{pt} – £{uel}/mois\n→ base {tp} × {ts_pct} % = £{m1}\nTranche haute (> UEL, 2 %) : £{uel}/mois\n→ base {th} × 2 % = £{m2}\n\nTotal NI salarié : £{tot}\nTaux effectif : {teff} %\n\nBase légale : NIA 2014 ; Finance Act 2024.`
  - en: `National Insurance Class 1 — employee share.\n\nBand [PT – UEL] ({ts_pct} %): £{pt} – £{uel}/month\n→ base {tp} × {ts_pct} % = £{m1}\nUpper band (> UEL, 2 %): £{uel}/month\n→ base {th} × 2 % = £{m2}\n\nTotal employee NI: £{tot}\nEffective rate: {teff} %\n\nLegal basis: NIA 2014; Finance Act 2024.`
  - de: `National Insurance Class 1 — Arbeitnehmeranteil.\n\nBand [PT – UEL] ({ts_pct} %): £{pt} – £{uel}/Monat\n→ Grundlage {tp} × {ts_pct} % = £{m1}\nOberes Band (> UEL, 2 %): £{uel}/Monat\n→ Grundlage {th} × 2 % = £{m2}\n\nGesamt AN-NI: £{tot}\nEffektivsatz: {teff} %\n\nRechtsgrundlage: NIA 2014; Finance Act 2024.`
  - nl: `National Insurance Class 1 — werknemersdeel.\n\nSchijf [PT – UEL] ({ts_pct} %): £{pt} – £{uel}/maand\n→ grondslag {tp} × {ts_pct} % = £{m1}\nHoge schijf (> UEL, 2 %): £{uel}/maand\n→ grondslag {th} × 2 % = £{m2}\n\nTotaal werknemers-NI: £{tot}\nEffectief tarief: {teff} %\n\nWettelijke basis: NIA 2014; Finance Act 2024.`
  - it: `National Insurance Class 1 — quota dipendente.\n\nFascia [PT – UEL] ({ts_pct} %): £{pt} – £{uel}/mese\n→ base {tp} × {ts_pct} % = £{m1}\nFascia alta (> UEL, 2 %): £{uel}/mese\n→ base {th} × 2 % = £{m2}\n\nTotale NI dipendente: £{tot}\nAliquota effettiva: {teff} %\n\nBase giuridica: NIA 2014; Finance Act 2024.`
  - es: `National Insurance Class 1 — parte del trabajador.\n\nTramo [PT – UEL] ({ts_pct} %): £{pt} – £{uel}/mes\n→ base {tp} × {ts_pct} % = £{m1}\nTramo alto (> UEL, 2 %): £{uel}/mes\n→ base {th} × 2 % = £{m2}\n\nTotal NI trabajador: £{tot}\nTipo efectivo: {teff} %\n\nBase legal: NIA 2014; Finance Act 2024.`

#### UK_NI_PAT
- **libelle** — fr: `National Insurance Class 1 — employeur {fy}` · en: `National Insurance Class 1 — employer {fy}` · de: `National Insurance Class 1 — Arbeitgeber {fy}` · nl: `National Insurance Class 1 — werkgever {fy}` · it: `National Insurance Class 1 — datore di lavoro {fy}` · es: `National Insurance Class 1 — empleador {fy}`
- **explication** (placeholders `{tp_pct} {st} {base} {tot} {teff}`) :
  - fr: `National Insurance Class 1 — part employeur.\n\nTaux : {tp_pct} % sur salaire > ST (£{st}/mois)\nBase imposable : £{base} × {tp_pct} % = £{tot}\nPas de plafond supérieur côté employeur.\nTaux effectif sur salaire brut : {teff} %\n\nBase légale : NIA 2014 ; Finance Act 2024.`
  - en: `National Insurance Class 1 — employer share.\n\nRate: {tp_pct} % on salary > ST (£{st}/month)\nTaxable base: £{base} × {tp_pct} % = £{tot}\nNo upper cap on the employer side.\nEffective rate on gross salary: {teff} %\n\nLegal basis: NIA 2014; Finance Act 2024.`
  - de: `National Insurance Class 1 — Arbeitgeberanteil.\n\nSatz: {tp_pct} % auf Gehalt > ST (£{st}/Monat)\nBemessungsgrundlage: £{base} × {tp_pct} % = £{tot}\nKeine Obergrenze auf Arbeitgeberseite.\nEffektivsatz auf Bruttogehalt: {teff} %\n\nRechtsgrundlage: NIA 2014; Finance Act 2024.`
  - nl: `National Insurance Class 1 — werkgeversdeel.\n\nTarief: {tp_pct} % op loon > ST (£{st}/maand)\nBelastbare grondslag: £{base} × {tp_pct} % = £{tot}\nGeen bovengrens aan werkgeverszijde.\nEffectief tarief op brutoloon: {teff} %\n\nWettelijke basis: NIA 2014; Finance Act 2024.`
  - it: `National Insurance Class 1 — quota datore di lavoro.\n\nAliquota: {tp_pct} % su retribuzione > ST (£{st}/mese)\nBase imponibile: £{base} × {tp_pct} % = £{tot}\nNessun massimale lato datore di lavoro.\nAliquota effettiva sul lordo: {teff} %\n\nBase giuridica: NIA 2014; Finance Act 2024.`
  - es: `National Insurance Class 1 — parte del empleador.\n\nTipo: {tp_pct} % sobre salario > ST (£{st}/mes)\nBase imponible: £{base} × {tp_pct} % = £{tot}\nSin límite superior por el lado del empleador.\nTipo efectivo sobre salario bruto: {teff} %\n\nBase legal: NIA 2014; Finance Act 2024.`

#### UK_INCOME_TAX
- **libelle** — fr: `Income Tax PAYE — retenue {fy}` · en: `Income Tax PAYE — withholding {fy}` · de: `Income Tax PAYE — Einbehalt {fy}` · nl: `Income Tax PAYE — inhouding {fy}` · it: `Income Tax PAYE — ritenuta {fy}` · es: `Income Tax PAYE — retención {fy}`
- **explication** (placeholders `{rev} {tl} {pa} {br} {hr} {ia} {im} {teff}`; `{tl}` = libellé de tranche, voir variantes ci-dessous) :
  - fr: `Income Tax PAYE (retenue à la source mensuelle).\n\nRevenu annuel estimé : £{rev} → tranche : {tl}\nPersonal Allowance : £{pa}/an (exonéré)\nBasic Rate 20 % : jusqu'à £{br}/an\nHigher Rate 40 % : £{br} – £{hr}/an\nAdditional Rate 45 % : au-delà de £{hr}/an\n\nImpôt annuel estimé : £{ia} / 12 = £{im}/mois\nTaux effectif mensuel : {teff} %\n\nBase légale : Income Tax Act 2007 ; Finance Act 2024.`
  - en: `Income Tax PAYE (monthly withholding).\n\nEstimated annual income: £{rev} → band: {tl}\nPersonal Allowance: £{pa}/yr (tax-free)\nBasic Rate 20 %: up to £{br}/yr\nHigher Rate 40 %: £{br} – £{hr}/yr\nAdditional Rate 45 %: above £{hr}/yr\n\nEstimated annual tax: £{ia} / 12 = £{im}/month\nEffective monthly rate: {teff} %\n\nLegal basis: Income Tax Act 2007; Finance Act 2024.`
  - de: `Income Tax PAYE (monatlicher Einbehalt).\n\nGeschätztes Jahreseinkommen: £{rev} → Stufe: {tl}\nPersonal Allowance: £{pa}/Jahr (steuerfrei)\nBasic Rate 20 %: bis £{br}/Jahr\nHigher Rate 40 %: £{br} – £{hr}/Jahr\nAdditional Rate 45 %: über £{hr}/Jahr\n\nGeschätzte Jahressteuer: £{ia} / 12 = £{im}/Monat\nEffektiver Monatssatz: {teff} %\n\nRechtsgrundlage: Income Tax Act 2007; Finance Act 2024.`
  - nl: `Income Tax PAYE (maandelijkse inhouding).\n\nGeschat jaarinkomen: £{rev} → schijf: {tl}\nPersonal Allowance: £{pa}/jr (belastingvrij)\nBasic Rate 20 %: tot £{br}/jr\nHigher Rate 40 %: £{br} – £{hr}/jr\nAdditional Rate 45 %: boven £{hr}/jr\n\nGeschatte jaarbelasting: £{ia} / 12 = £{im}/maand\nEffectief maandtarief: {teff} %\n\nWettelijke basis: Income Tax Act 2007; Finance Act 2024.`
  - it: `Income Tax PAYE (ritenuta mensile).\n\nReddito annuo stimato: £{rev} → fascia: {tl}\nPersonal Allowance: £{pa}/anno (esente)\nBasic Rate 20 %: fino a £{br}/anno\nHigher Rate 40 %: £{br} – £{hr}/anno\nAdditional Rate 45 %: oltre £{hr}/anno\n\nImposta annua stimata: £{ia} / 12 = £{im}/mese\nAliquota effettiva mensile: {teff} %\n\nBase giuridica: Income Tax Act 2007; Finance Act 2024.`
  - es: `Income Tax PAYE (retención mensual).\n\nRenta anual estimada: £{rev} → tramo: {tl}\nPersonal Allowance: £{pa}/año (exento)\nBasic Rate 20 %: hasta £{br}/año\nHigher Rate 40 %: £{br} – £{hr}/año\nAdditional Rate 45 %: por encima de £{hr}/año\n\nImpuesto anual estimado: £{ia} / 12 = £{im}/mes\nTipo efectivo mensual: {teff} %\n\nBase legal: Income Tax Act 2007; Finance Act 2024.`
- **`{tl}` (libellé de tranche)** :
  - `dans la Personal Allowance (0 %)` → en `within the Personal Allowance (0 %)` · de `innerhalb der Personal Allowance (0 %)` · nl `binnen de Personal Allowance (0 %)` · it `entro la Personal Allowance (0 %)` · es `dentro de la Personal Allowance (0 %)`
  - `Basic Rate (20 %)` (identique) · `Higher Rate partielle (40 %)` → en `partial Higher Rate (40 %)` · de `teilweiser Higher Rate (40 %)` · nl `gedeeltelijk Higher Rate (40 %)` · it `Higher Rate parziale (40 %)` · es `Higher Rate parcial (40 %)` · `Additional Rate (45 %)` (identique)

### 🇫🇷 Entreprise adaptée (rattaché France — déjà routé via `ctx`)

> `AIDE_POSTE_EA` appelle déjà `ctx.libelle`/`ctx.expl` → la traduction doit être ajoutée dans
> `i18n/cotisations.rs` (domaine France). Texte à traduire :

#### AIDE_POSTE_EA
- **libelle** — fr: `Aide au poste — entreprise adaptée (État/ASP)` · en: `Job support grant — adapted enterprise (State/ASP)` · de: `Arbeitsplatzhilfe — Inklusionsbetrieb (Staat/ASP)` · nl: `Werkplekondersteuning — aangepast bedrijf (Staat/ASP)` · it: `Aiuto al posto — impresa adattata (Stato/ASP)` · es: `Ayuda al puesto — empresa adaptada (Estado/ASP)`
- **explication** (texte statique, sans placeholder ; le bloc `\u{1}{…}` JSON de détail n'est PAS traduit) :
  - fr: `L'aide au poste est une aide financière de l'État, versée à l'employeur par l'Agence de services et de paiement (ASP), au titre de l'emploi d'un travailleur handicapé (RQTH) en entreprise adaptée. Forfait annuel par équivalent temps plein, versé par douzième et proratisé au temps de travail. Montant selon la tranche d'âge du salarié : 18 230 € (< 50 ans), 18 465 € (50-55 ans), 18 941 € (56 ans et +) depuis le 01/11/2024. En cas d'arrêt maladie ou accident, la part absente est minorée à 30 % du SMIC horaire brut. Cette aide ne modifie ni le brut, ni le net du salarié : elle réduit le coût réel supporté par l'employeur.`
  - en: `The job support grant is State financial aid paid to the employer by the Agence de services et de paiement (ASP) for employing a disabled worker (RQTH) in an adapted enterprise. Annual flat amount per full-time equivalent, paid in twelfths and pro-rated to working time. Amount by employee age band: €18,230 (< 50), €18,465 (50-55), €18,941 (56+) since 01/11/2024. In case of sick leave or accident, the absent share is reduced to 30 % of the gross hourly SMIC. This grant changes neither the gross nor the net pay: it reduces the real cost borne by the employer.`
  - de: `Die Arbeitsplatzhilfe ist eine staatliche Förderung, die dem Arbeitgeber von der Agence de services et de paiement (ASP) für die Beschäftigung eines schwerbehinderten Arbeitnehmers (RQTH) in einem Inklusionsbetrieb gezahlt wird. Jährlicher Pauschalbetrag je Vollzeitäquivalent, in Zwölfteln gezahlt und nach Arbeitszeit anteilig. Betrag je Altersgruppe: 18.230 € (< 50), 18.465 € (50-55), 18.941 € (56+) seit 01.11.2024. Bei Krankheit oder Unfall wird der abwesende Anteil auf 30 % des Brutto-Stunden-SMIC gekürzt. Diese Hilfe ändert weder Brutto noch Netto des Arbeitnehmers: sie senkt die tatsächlichen Arbeitgeberkosten.`
  - nl: `De werkplekondersteuning is staatssteun die de werkgever ontvangt van de Agence de services et de paiement (ASP) voor het in dienst nemen van een werknemer met een handicap (RQTH) in een aangepast bedrijf. Jaarlijks vast bedrag per voltijdsequivalent, in twaalfden uitbetaald en naar arbeidstijd verrekend. Bedrag per leeftijdsgroep: € 18.230 (< 50), € 18.465 (50-55), € 18.941 (56+) sinds 01/11/2024. Bij ziekte of ongeval wordt het afwezige deel verlaagd tot 30 % van het bruto-uur-SMIC. Deze steun wijzigt noch het bruto noch het netto van de werknemer: ze verlaagt de werkelijke werkgeverskosten.`
  - it: `L'aiuto al posto è un sostegno finanziario statale versato al datore di lavoro dall'Agence de services et de paiement (ASP) per l'impiego di un lavoratore disabile (RQTH) in un'impresa adattata. Importo forfettario annuo per equivalente a tempo pieno, versato in dodicesimi e proporzionato al tempo di lavoro. Importo per fascia d'età: 18.230 € (< 50), 18.465 € (50-55), 18.941 € (56+) dal 01/11/2024. In caso di malattia o infortunio, la quota assente è ridotta al 30 % dello SMIC orario lordo. Questo aiuto non modifica né il lordo né il netto del dipendente: riduce il costo reale a carico del datore di lavoro.`
  - es: `La ayuda al puesto es una ayuda financiera del Estado abonada al empleador por la Agence de services et de paiement (ASP) por emplear a un trabajador con discapacidad (RQTH) en una empresa adaptada. Importe fijo anual por equivalente a tiempo completo, pagado en doceavos y prorrateado al tiempo de trabajo. Importe por tramo de edad: 18.230 € (< 50), 18.465 € (50-55), 18.941 € (56+) desde el 01/11/2024. En caso de baja por enfermedad o accidente, la parte ausente se reduce al 30 % del SMIC horario bruto. Esta ayuda no modifica ni el bruto ni el neto del trabajador: reduce el coste real soportado por el empleador.`

> **Convention pays « cœur » (IT, CH, DE, LU, ES, PT, BE, CA, QC, JP, KR)** : les libellés
> sont traduits intégralement. Les explications, souvent longues (rappels historiques, bases
> légales), sont traduites **fidèlement mais resserrées** dans les 5 langues cibles ; les
> placeholders `{x}` et les références légales/numéros de loi sont conservés tels quels. Le
> texte français complet d'origine reste le repli (`fr`).

### 🇮🇹 Italie

> Lignes IRPEF (`IT_IRPEF`, `IT_BONUS_CUNEO`, `IT_ADD_REG_*`) : voir aussi `it_irpef.rs`.

#### IT_IVS
- **libelle** — fr: `IVS — Invalidità, Vecchiaia, Superstiti` (identique 6 langues : terme propre INPS)
- **explication** (placeholders `{annee} {massimale} {annuel}`) :
  - fr: `L'IVS (Invalidità, Vecchiaia, Superstiti) est la cotisation de retraite obligatoire italienne (INPS), régie par la L. 335/1995. Taux total 33 % = 9,19 % salarié + 23,81 % employeur. Massimale contributivo {annee} : {massimale} €/mois ({annuel} €/an), applicable aux seuls travailleurs sans ancienneté INPS au 31/12/1995 ; les salariés pré-1996 cotisent sur la totalité.`
  - en: `IVS (Invalidità, Vecchiaia, Superstiti) is Italy's mandatory pension contribution (INPS), governed by L. 335/1995. Total rate 33 % = 9.19 % employee + 23.81 % employer. Massimale contributivo {annee}: {massimale} €/month ({annuel} €/yr), applying only to workers with no INPS seniority at 31/12/1995; pre-1996 employees contribute on the full salary.`
  - de: `IVS (Invalidità, Vecchiaia, Superstiti) ist Italiens obligatorischer Rentenbeitrag (INPS), geregelt durch L. 335/1995. Gesamtsatz 33 % = 9,19 % Arbeitnehmer + 23,81 % Arbeitgeber. Massimale contributivo {annee}: {massimale} €/Monat ({annuel} €/Jahr), nur für Arbeitnehmer ohne INPS-Anwartschaft zum 31.12.1995; vor 1996 Beschäftigte zahlen auf das volle Gehalt.`
  - nl: `IVS (Invalidità, Vecchiaia, Superstiti) is de verplichte Italiaanse pensioenbijdrage (INPS), geregeld door L. 335/1995. Totaaltarief 33 % = 9,19 % werknemer + 23,81 % werkgever. Massimale contributivo {annee}: {massimale} €/maand ({annuel} €/jr), alleen voor werknemers zonder INPS-anciënniteit op 31-12-1995; vóór 1996 betalen op het volledige loon.`
  - it: `L'IVS (Invalidità, Vecchiaia, Superstiti) è il contributo pensionistico obbligatorio italiano (INPS), disciplinato dalla L. 335/1995. Aliquota totale 33 % = 9,19 % dipendente + 23,81 % datore di lavoro. Massimale contributivo {annee}: {massimale} €/mese ({annuel} €/anno), applicabile ai soli lavoratori senza anzianità INPS al 31/12/1995; gli iscritti ante 1996 contribuiscono sull'intera retribuzione.`
  - es: `El IVS (Invalidità, Vecchiaia, Superstiti) es la cotización de jubilación obligatoria italiana (INPS), regida por la L. 335/1995. Tipo total 33 % = 9,19 % trabajador + 23,81 % empleador. Massimale contributivo {annee}: {massimale} €/mes ({annuel} €/año), aplicable solo a trabajadores sin antigüedad INPS al 31/12/1995; los anteriores a 1996 cotizan sobre el salario íntegro.`

#### IT_NASPI
- **libelle** — fr: `NASpI — Assicurazione chômage (cotisation ordinaire)` · en: `NASpI — Unemployment insurance (ordinary contribution)` · de: `NASpI — Arbeitslosenversicherung (ordentlicher Beitrag)` · nl: `NASpI — Werkloosheidsverzekering (gewone bijdrage)` · it: `NASpI — Assicurazione disoccupazione (contributo ordinario)` · es: `NASpI — Seguro de desempleo (cotización ordinaria)`
- **explication** (sans placeholder) :
  - fr: `La NASpI (D.Lgs. 22/2015, Jobs Act) indemnise les salariés licenciés, proportionnellement au salaire moyen des 4 dernières années (durée = moitié des semaines cotisées, max 24 mois). La cotisation salarié (0,30 %) est supprimée depuis le 01/01/2013 (L. 228/2012) : ne subsiste que la cotisation ordinaire patronale (1,61 %), sans plafond.`
  - en: `NASpI (D.Lgs. 22/2015, Jobs Act) indemnifies dismissed employees, proportional to the average wage of the last 4 years (duration = half the contributed weeks, max 24 months). The employee contribution (0.30 %) was abolished on 01/01/2013 (L. 228/2012): only the ordinary employer contribution (1.61 %) remains, with no cap.`
  - de: `NASpI (D.Lgs. 22/2015, Jobs Act) entschädigt entlassene Arbeitnehmer, anteilig zum Durchschnittslohn der letzten 4 Jahre (Dauer = halbe Beitragswochen, max. 24 Monate). Der Arbeitnehmerbeitrag (0,30 %) ist seit 01.01.2013 abgeschafft (L. 228/2012): es bleibt nur der ordentliche Arbeitgeberbeitrag (1,61 %), ohne Obergrenze.`
  - nl: `NASpI (D.Lgs. 22/2015, Jobs Act) vergoedt ontslagen werknemers, evenredig aan het gemiddelde loon van de laatste 4 jaar (duur = helft van de bijdrageweken, max. 24 maanden). De werknemersbijdrage (0,30 %) is sinds 01-01-2013 afgeschaft (L. 228/2012): alleen de gewone werkgeversbijdrage (1,61 %) blijft, zonder plafond.`
  - it: `La NASpI (D.Lgs. 22/2015, Jobs Act) indennizza i dipendenti licenziati, in proporzione alla retribuzione media degli ultimi 4 anni (durata = metà delle settimane contribuite, max 24 mesi). Il contributo dipendente (0,30 %) è soppresso dal 01/01/2013 (L. 228/2012): resta solo il contributo ordinario datoriale (1,61 %), senza massimale.`
  - es: `La NASpI (D.Lgs. 22/2015, Jobs Act) indemniza a los trabajadores despedidos, en proporción al salario medio de los últimos 4 años (duración = mitad de las semanas cotizadas, máx. 24 meses). La cotización del trabajador (0,30 %) se suprimió el 01/01/2013 (L. 228/2012): solo queda la cotización ordinaria patronal (1,61 %), sin tope.`

#### IT_NASPI_TERMINE
- **libelle** — fr: `NASpI — Contributo addizionale CDD (+1,40 % pat.)` · en: `NASpI — Additional fixed-term contribution (+1.40 % empr)` · de: `NASpI — Zusatzbeitrag befristet (+1,40 % AG)` · nl: `NASpI — Aanvullende bijdrage tijdelijk (+1,40 % wg)` · it: `NASpI — Contributo addizionale a termine (+1,40 % dat.)` · es: `NASpI — Cotización adicional temporal (+1,40 % empr)`
- **explication** (sans placeholder) :
  - fr: `Majoration patronale de 1,40 % sur les contrats à durée déterminée (L. 92/2012 art. 2 c. 28-29), remboursée si le CDD est transformé en CDI sous 6 mois. Non applicable aux CDD de remplacement, saisonniers, apprentis ni intermittents.`
  - en: `Employer surcharge of 1.40 % on fixed-term contracts (L. 92/2012 art. 2 c. 28-29), refunded if the contract is converted to permanent within 6 months. Not applicable to replacement, seasonal, apprentice or intermittent fixed-term contracts.`
  - de: `Arbeitgeberzuschlag von 1,40 % auf befristete Verträge (L. 92/2012 Art. 2 Abs. 28-29), erstattet bei Umwandlung in unbefristet binnen 6 Monaten. Nicht für Vertretungs-, Saison-, Ausbildungs- oder intermittierende Verträge.`
  - nl: `Werkgeverstoeslag van 1,40 % op tijdelijke contracten (L. 92/2012 art. 2 c. 28-29), terugbetaald bij omzetting naar vast binnen 6 maanden. Niet voor vervangings-, seizoens-, leerling- of oproepcontracten.`
  - it: `Maggiorazione datoriale dell'1,40 % sui contratti a termine (L. 92/2012 art. 2 c. 28-29), restituita in caso di trasformazione a tempo indeterminato entro 6 mesi. Non si applica a sostituzione, stagionali, apprendisti o intermittenti.`
  - es: `Recargo patronal del 1,40 % sobre contratos temporales (L. 92/2012 art. 2 c. 28-29), reembolsado si se transforma en indefinido en 6 meses. No aplicable a sustitución, temporeros, aprendices ni intermitentes.`

#### IT_MALATTIA
- **libelle** — fr: `Malattia — Indemnités journalières (INPS)` · en: `Malattia — Daily sickness allowance (INPS)` · de: `Malattia — Krankengeld (INPS)` · nl: `Malattia — Dagvergoeding ziekte (INPS)` · it: `Malattia — Indennità giornaliere (INPS)` · es: `Malattia — Subsidio diario por enfermedad (INPS)`
- **explication** (sans placeholder) :
  - fr: `Finance les indemnités journalières INPS à partir du 4ᵉ jour d'arrêt (les 3 premiers jours — carenza — sont à charge de l'employeur ou du CCNL). Indemnité : 50 % du salaire journalier du 4ᵉ au 20ᵉ jour, 66,66 % du 21ᵉ au 180ᵉ. Taux 2,22 % indicatif (commercio/industria), variable selon le CCNL.`
  - en: `Funds INPS daily allowances from the 4th day of leave (first 3 days — carenza — borne by employer or CCNL). Allowance: 50 % of daily wage from day 4 to 20, 66.66 % from day 21 to 180. Rate 2.22 % indicative (commercio/industria), varies by CCNL.`
  - de: `Finanziert INPS-Krankengeld ab dem 4. Tag (erste 3 Tage — carenza — vom Arbeitgeber oder CCNL getragen). Leistung: 50 % des Tageslohns vom 4.–20. Tag, 66,66 % vom 21.–180. Satz 2,22 % indikativ (commercio/industria), je nach CCNL.`
  - nl: `Financiert INPS-dagvergoeding vanaf de 4e ziektedag (eerste 3 dagen — carenza — door werkgever of CCNL gedragen). Vergoeding: 50 % van het dagloon dag 4-20, 66,66 % dag 21-180. Tarief 2,22 % indicatief (commercio/industria), varieert per CCNL.`
  - it: `Finanzia le indennità giornaliere INPS dal 4° giorno di assenza (primi 3 giorni — carenza — a carico del datore o del CCNL). Indennità: 50 % della retribuzione giornaliera dal 4° al 20° giorno, 66,66 % dal 21° al 180°. Aliquota 2,22 % indicativa (commercio/industria), variabile per CCNL.`
  - es: `Financia los subsidios diarios del INPS desde el 4º día de baja (los 3 primeros — carenza — a cargo del empleador o del CCNL). Subsidio: 50 % del salario diario del día 4 al 20, 66,66 % del 21 al 180. Tipo 2,22 % indicativo (commercio/industria), variable según CCNL.`

#### IT_MATERNITA
- **libelle** — fr: `Maternità / Paternità — Congés parentaux (INPS)` · en: `Maternità / Paternità — Parental leave (INPS)` · de: `Maternità / Paternità — Elternzeit (INPS)` · nl: `Maternità / Paternità — Ouderschapsverlof (INPS)` · it: `Maternità / Paternità — Congedi parentali (INPS)` · es: `Maternità / Paternità — Permisos parentales (INPS)`
- **explication** (sans placeholder) :
  - fr: `Finance les congés parentaux INPS : maternité 5 mois à 80 %, paternité obligatoire 10 jours à 80 %, congé parental jusqu'à 6 mois/parent (L. 207/2024 : 80 % le 1er mois, 60 % le 2ᵉ). Cotisation 100 % patronale (0,46 %), stable.`
  - en: `Funds INPS parental leave: maternity 5 months at 80 %, mandatory paternity 10 days at 80 %, parental leave up to 6 months/parent (L. 207/2024: 80 % 1st month, 60 % 2nd). 100 % employer contribution (0.46 %), stable.`
  - de: `Finanziert INPS-Elternzeit: Mutterschaft 5 Monate zu 80 %, Pflicht-Vaterschaft 10 Tage zu 80 %, Elternzeit bis 6 Monate/Elternteil (L. 207/2024: 80 % 1. Monat, 60 % 2.). 100 % Arbeitgeberbeitrag (0,46 %), stabil.`
  - nl: `Financiert INPS-ouderschapsverlof: moederschap 5 maanden tegen 80 %, verplicht vaderschap 10 dagen tegen 80 %, ouderschapsverlof tot 6 maanden/ouder (L. 207/2024: 80 % 1e maand, 60 % 2e). 100 % werkgeversbijdrage (0,46 %), stabiel.`
  - it: `Finanzia i congedi parentali INPS: maternità 5 mesi all'80 %, paternità obbligatoria 10 giorni all'80 %, congedo parentale fino a 6 mesi/genitore (L. 207/2024: 80 % 1° mese, 60 % 2°). Contributo 100 % datoriale (0,46 %), stabile.`
  - es: `Financia los permisos parentales del INPS: maternidad 5 meses al 80 %, paternidad obligatoria 10 días al 80 %, permiso parental hasta 6 meses/progenitor (L. 207/2024: 80 % 1er mes, 60 % 2º). Cotización 100 % patronal (0,46 %), estable.`

#### IT_FONDO_GARANZIA
- **libelle** — fr: `Fondo di Garanzia TFR — INPS (L. 297/1982)` (identique 6 langues : terme propre, ref légale)
- **explication** (sans placeholder) :
  - fr: `Le Fondo di Garanzia (INPS) garantit le paiement du TFR si l'employeur est insolvable (L. 297/1982 art. 2). Cotisation 0,20 % patronale, versée via F24. Distinct du versement direct au Fondo Tesoreria INPS (obligatoire > 50 salariés depuis 2007).`
  - en: `The Fondo di Garanzia (INPS) guarantees TFR payment if the employer is insolvent (L. 297/1982 art. 2). Employer contribution 0.20 %, paid via F24. Distinct from the direct transfer to the Fondo Tesoreria INPS (mandatory > 50 employees since 2007).`
  - de: `Der Fondo di Garanzia (INPS) sichert die TFR-Zahlung bei Insolvenz des Arbeitgebers (L. 297/1982 Art. 2). Arbeitgeberbeitrag 0,20 %, gezahlt über F24. Unterscheidet sich von der direkten Zahlung an den Fondo Tesoreria INPS (Pflicht > 50 Beschäftigte seit 2007).`
  - nl: `Het Fondo di Garanzia (INPS) waarborgt de TFR-betaling bij insolventie van de werkgever (L. 297/1982 art. 2). Werkgeversbijdrage 0,20 %, betaald via F24. Te onderscheiden van de directe storting aan het Fondo Tesoreria INPS (verplicht > 50 werknemers sinds 2007).`
  - it: `Il Fondo di Garanzia (INPS) garantisce il pagamento del TFR in caso di insolvenza del datore (L. 297/1982 art. 2). Contributo datoriale 0,20 %, versato tramite F24. Distinto dal versamento diretto al Fondo Tesoreria INPS (obbligatorio > 50 dipendenti dal 2007).`
  - es: `El Fondo di Garanzia (INPS) garantiza el pago del TFR si el empleador es insolvente (L. 297/1982 art. 2). Cotización patronal 0,20 %, abonada vía F24. Distinto del ingreso directo al Fondo Tesoreria INPS (obligatorio > 50 trabajadores desde 2007).`

#### IT_INAIL
- **libelle** — fr: `INAIL — Assicurazione Infortuni e Malattie Professionali` (identique 6 langues : terme propre)
- **explication** (placeholders `{}` = taux ; conserver `{:.2}`) :
  - fr: `Assurance accidents du travail et maladies professionnelles (INAIL), obligatoire et 100 % patronale (DPR 1124/1965). Taux {} % indicatif (bureau/terziario) ; le taux réel dépend de la voce di tariffa ATECO, de la sinistralité (±28 %) et des mesures de prévention. Auto-liquidazione au 16 février.`
  - en: `Work accident and occupational disease insurance (INAIL), mandatory and 100 % employer (DPR 1124/1965). Rate {} % indicative (office/terziario); the actual rate depends on the ATECO voce di tariffa, claims history (±28 %) and prevention measures. Auto-liquidazione on 16 February.`
  - de: `Arbeitsunfall- und Berufskrankheitenversicherung (INAIL), Pflicht und 100 % Arbeitgeber (DPR 1124/1965). Satz {} % indikativ (Büro/terziario); der tatsächliche Satz hängt von der ATECO voce di tariffa, der Schadenshistorie (±28 %) und Präventionsmaßnahmen ab. Auto-liquidazione am 16. Februar.`
  - nl: `Verzekering arbeidsongevallen en beroepsziekten (INAIL), verplicht en 100 % werkgever (DPR 1124/1965). Tarief {} % indicatief (kantoor/terziario); het werkelijke tarief hangt af van de ATECO voce di tariffa, schadeverleden (±28 %) en preventiemaatregelen. Auto-liquidazione op 16 februari.`
  - it: `Assicurazione infortuni sul lavoro e malattie professionali (INAIL), obbligatoria e 100 % datoriale (DPR 1124/1965). Aliquota {} % indicativa (ufficio/terziario); l'aliquota reale dipende dalla voce di tariffa ATECO, dalla sinistrosità (±28 %) e dalle misure di prevenzione. Autoliquidazione al 16 febbraio.`
  - es: `Seguro de accidentes de trabajo y enfermedades profesionales (INAIL), obligatorio y 100 % patronal (DPR 1124/1965). Tipo {} % indicativo (oficina/terziario); el tipo real depende de la voce di tariffa ATECO, la siniestralidad (±28 %) y las medidas de prevención. Auto-liquidazione el 16 de febrero.`

#### IT_TFR
- **libelle** — fr: `TFR — Trattamento Fine Rapporto (accrual mensuel)` · en: `TFR — Trattamento Fine Rapporto (monthly accrual)` · de: `TFR — Trattamento Fine Rapporto (monatliche Rückstellung)` · nl: `TFR — Trattamento Fine Rapporto (maandelijkse opbouw)` · it: `TFR — Trattamento Fine Rapporto (accantonamento mensile)` · es: `TFR — Trattamento Fine Rapporto (provisión mensual)`
- **explication** (placeholders `{montant}`) :
  - fr: `Le TFR (L. 297/1982) est une rémunération différée : provision mensuelle de {montant} € (6,91 % = 1/13,5 du brut annuel), payée à la fin du contrat. Destination selon la taille (≤ 50 : chez l'employeur ; > 50 : Fondo Tesoreria INPS ou fonds de pension). Revalorisation 75 % ISTAT + 1,5 %.`
  - en: `TFR (L. 297/1982) is deferred pay: monthly provision of {montant} € (6.91 % = 1/13.5 of annual gross), paid at contract end. Destination by size (≤ 50: with employer; > 50: Fondo Tesoreria INPS or pension fund). Revaluation 75 % ISTAT + 1.5 %.`
  - de: `TFR (L. 297/1982) ist aufgeschobenes Entgelt: monatliche Rückstellung von {montant} € (6,91 % = 1/13,5 des Jahresbruttos), bei Vertragsende ausgezahlt. Ziel je nach Größe (≤ 50: beim Arbeitgeber; > 50: Fondo Tesoreria INPS oder Pensionsfonds). Aufwertung 75 % ISTAT + 1,5 %.`
  - nl: `TFR (L. 297/1982) is uitgesteld loon: maandelijkse voorziening van {montant} € (6,91 % = 1/13,5 van het jaarbruto), uitbetaald bij einde contract. Bestemming naar grootte (≤ 50: bij werkgever; > 50: Fondo Tesoreria INPS of pensioenfonds). Herwaardering 75 % ISTAT + 1,5 %.`
  - it: `Il TFR (L. 297/1982) è retribuzione differita: accantonamento mensile di {montant} € (6,91 % = 1/13,5 del lordo annuo), pagato a fine rapporto. Destinazione per dimensione (≤ 50: presso il datore; > 50: Fondo Tesoreria INPS o fondo pensione). Rivalutazione 75 % ISTAT + 1,5 %.`
  - es: `El TFR (L. 297/1982) es retribución diferida: provisión mensual de {montant} € (6,91 % = 1/13,5 del bruto anual), pagada al final del contrato. Destino según tamaño (≤ 50: en el empleador; > 50: Fondo Tesoreria INPS o fondo de pensiones). Revalorización 75 % ISTAT + 1,5 %.`

#### IT_ESONERO_2022 / IT_ESONERO_2023 / IT_ESONERO_2024 (allègements temporaires, montant négatif)
- `IT_ESONERO_2022` — libelle fr `Esonero contributivo H2 2022 (−0,80 % IVS)` · en `Contribution relief H2 2022 (−0.80 % IVS)` · de `Beitragsentlastung H2 2022 (−0,80 % IVS)` · nl `Bijdragevermindering H2 2022 (−0,80 % IVS)` · it `Esonero contributivo H2 2022 (−0,80 % IVS)` · es `Exoneración de cotización H2 2022 (−0,80 % IVS)`
  - expl — fr: `Réduction temporaire de 0,80 pt de la cotisation IVS salarié, de juillet à décembre 2022 si reddito ≤ 35 000 € (DL 115/2022, conv. L. 142/2022). Le montant négatif augmente le net.` · en: `Temporary 0.80 pt reduction of the employee IVS contribution, July–December 2022 if reddito ≤ 35,000 € (DL 115/2022, conv. L. 142/2022). The negative amount increases net pay.` · de: `Vorübergehende Senkung um 0,80 Pkt. des AN-IVS-Beitrags, Juli–Dezember 2022 bei reddito ≤ 35.000 € (DL 115/2022, umgew. L. 142/2022). Der negative Betrag erhöht das Netto.` · nl: `Tijdelijke verlaging van 0,80 pt van de werknemers-IVS-bijdrage, juli–december 2022 bij reddito ≤ 35.000 € (DL 115/2022, omgez. L. 142/2022). Het negatieve bedrag verhoogt het netto.` · it: `Riduzione temporanea di 0,80 pt del contributo IVS dipendente, luglio–dicembre 2022 se reddito ≤ 35.000 € (DL 115/2022, conv. L. 142/2022). L'importo negativo aumenta il netto.` · es: `Reducción temporal de 0,80 pt de la cotización IVS del trabajador, julio–diciembre 2022 si reddito ≤ 35.000 € (DL 115/2022, conv. L. 142/2022). El importe negativo aumenta el neto.`
- `IT_ESONERO_2023` — libelle (placeholder `{taux_pp}`) fr `Esonero contributivo 2023 (−{taux_pp} % IVS)` · en `Contribution relief 2023 (−{taux_pp} % IVS)` · de `Beitragsentlastung 2023 (−{taux_pp} % IVS)` · nl `Bijdragevermindering 2023 (−{taux_pp} % IVS)` · it `Esonero contributivo 2023 (−{taux_pp} % IVS)` · es `Exoneración de cotización 2023 (−{taux_pp} % IVS)`
  - expl (placeholders `{taux_pp} {reddito} {taux_pp}`) — fr: `Réduction de {taux_pp} pts sur la cotisation IVS salarié (2023) : −3 pts si reddito ≤ 25 000 €, −2 pts si 25 001–35 000 €. Reddito estimé : {reddito} €/an → {taux_pp} pts. L. 197/2022 art. 1 c. 281-286.` · en: `Reduction of {taux_pp} pts on employee IVS contribution (2023): −3 pts if reddito ≤ 25,000 €, −2 pts if 25,001–35,000 €. Estimated reddito: {reddito} €/yr → {taux_pp} pts. L. 197/2022 art. 1 c. 281-286.` · de: `Senkung um {taux_pp} Pkt. beim AN-IVS-Beitrag (2023): −3 bei reddito ≤ 25.000 €, −2 bei 25.001–35.000 €. Geschätzter reddito: {reddito} €/Jahr → {taux_pp} Pkt. L. 197/2022 Art. 1 Abs. 281-286.` · nl: `Verlaging van {taux_pp} pt op de werknemers-IVS-bijdrage (2023): −3 bij reddito ≤ 25.000 €, −2 bij 25.001–35.000 €. Geschatte reddito: {reddito} €/jr → {taux_pp} pt. L. 197/2022 art. 1 c. 281-286.` · it: `Riduzione di {taux_pp} pt sul contributo IVS dipendente (2023): −3 se reddito ≤ 25.000 €, −2 se 25.001–35.000 €. Reddito stimato: {reddito} €/anno → {taux_pp} pt. L. 197/2022 art. 1 c. 281-286.` · es: `Reducción de {taux_pp} pt en la cotización IVS del trabajador (2023): −3 si reddito ≤ 25.000 €, −2 si 25.001–35.000 €. Reddito estimado: {reddito} €/año → {taux_pp} pt. L. 197/2022 art. 1 c. 281-286.`
- `IT_ESONERO_2024` — libelle (placeholder `{taux_pp}`) : même schéma que 2023, année 2024.
  - expl (placeholders `{taux_pp} {reddito} {taux_pp}`) — fr: `Réduction de {taux_pp} pts sur la cotisation IVS salarié (2024) : −7 pts si reddito ≤ 25 000 €, −6 pts si 25 001–35 000 €. Reddito estimé : {reddito} €/an → {taux_pp} pts. L. 213/2023 art. 1 cc. 15-17.` · en: `Reduction of {taux_pp} pts on employee IVS (2024): −7 if reddito ≤ 25,000 €, −6 if 25,001–35,000 €. Estimated reddito: {reddito} €/yr → {taux_pp} pts. L. 213/2023 art. 1 cc. 15-17.` · de: `Senkung um {taux_pp} Pkt. (2024): −7 bei reddito ≤ 25.000 €, −6 bei 25.001–35.000 €. Geschätzter reddito: {reddito} €/Jahr → {taux_pp} Pkt. L. 213/2023 Art. 1 cc. 15-17.` · nl: `Verlaging van {taux_pp} pt (2024): −7 bij reddito ≤ 25.000 €, −6 bij 25.001–35.000 €. Geschatte reddito: {reddito} €/jr → {taux_pp} pt. L. 213/2023 art. 1 cc. 15-17.` · it: `Riduzione di {taux_pp} pt (2024): −7 se reddito ≤ 25.000 €, −6 se 25.001–35.000 €. Reddito stimato: {reddito} €/anno → {taux_pp} pt. L. 213/2023 art. 1 cc. 15-17.` · es: `Reducción de {taux_pp} pt (2024): −7 si reddito ≤ 25.000 €, −6 si 25.001–35.000 €. Reddito estimado: {reddito} €/año → {taux_pp} pt. L. 213/2023 art. 1 cc. 15-17.`

#### IT_IRPEF / IT_BONUS_CUNEO / IT_ADD_REG_* (résumé — détail dans `it_irpef.rs`)
- `IT_IRPEF` — libelle (placeholder `{annee}`) fr `IRPEF — Retenue à la source {annee}` · en `IRPEF — Withholding {annee}` · de `IRPEF — Quellensteuer {annee}` · nl `IRPEF — Inhouding {annee}` · it `IRPEF — Ritenuta alla fonte {annee}` · es `IRPEF — Retención en origen {annee}` *(explication longue : à traduire au câblage, placeholders {annee} {nb_tranches} {reddito} {irpef_b} {det} {irpef_n} {irpef_m} {teff})*
- `IT_BONUS_CUNEO` — libelle (placeholder `{annee}`) fr `Bonus cuneo fiscale {annee} (trattamento integrativo)` (terme propre, conserver ; gloser « tax wedge bonus / Steuerkeil-Bonus / ... » selon langue) *(explication : placeholders {annee} {desc} {reddito} {bonus_a} {bonus_m})*
- `IT_ADD_REG_{regione}` — libelle (placeholder `{libelle_region}`) fr `Addizionale regionale IRPEF — {libelle_region}` · en `Regional IRPEF surcharge — {libelle_region}` · de `Regionaler IRPEF-Zuschlag — {libelle_region}` · nl `Regionale IRPEF-toeslag — {libelle_region}` · it `Addizionale regionale IRPEF — {libelle_region}` · es `Recargo regional del IRPEF — {libelle_region}` *(noms de régions = noms propres, non traduits)*

### 🇱🇺 Luxembourg

> Toutes les explications LU partagent les placeholders `{plafond} {annee}` (plafond 5 × SSM).

#### LU_AP
- **libelle** — fr: `AP — Assurance pension` · en: `AP — Pension insurance` · de: `AP — Rentenversicherung` · nl: `AP — Pensioenverzekering` · it: `AP — Assicurazione pensione` · es: `AP — Seguro de pensión`
- **explication** (placeholders `{plafond} {annee}`) :
  - fr: `Assurance pension obligatoire (CNAP, CSS LU Livre II), par répartition. Taux 16 % total (8 % salarié, 8 % employeur) ; l'État ajoute un tiers. Assiette plafonnée à 5 × SSM (≈ {plafond} €/mois en {annee}). Pension complète après 40 ans ; retraite à 65 ans (ou 57 ans anticipée).`
  - en: `Mandatory pension insurance (CNAP, CSS LU Book II), pay-as-you-go. Rate 16 % total (8 % employee, 8 % employer); the State adds a third. Base capped at 5 × SSM (≈ {plafond} €/month in {annee}). Full pension after 40 years; retirement at 65 (or 57 early).`
  - de: `Obligatorische Rentenversicherung (CNAP, CSS LU Buch II), Umlageverfahren. Satz 16 % gesamt (8 % AN, 8 % AG); der Staat ergänzt ein Drittel. Bemessung gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in {annee}). Volle Rente nach 40 Jahren; Rente mit 65 (oder 57 vorgezogen).`
  - nl: `Verplichte pensioenverzekering (CNAP, CSS LU Boek II), omslagstelsel. Tarief 16 % totaal (8 % wn, 8 % wg); de Staat voegt een derde toe. Grondslag begrensd op 5 × SSM (≈ {plafond} €/maand in {annee}). Volledig pensioen na 40 jaar; pensioen op 65 (of 57 vervroegd).`
  - it: `Assicurazione pensione obbligatoria (CNAP, CSS LU Libro II), a ripartizione. Aliquota 16 % totale (8 % dipendente, 8 % datore); lo Stato aggiunge un terzo. Base limitata a 5 × SSM (≈ {plafond} €/mese in {annee}). Pensione piena dopo 40 anni; pensione a 65 (o 57 anticipata).`
  - es: `Seguro de pensión obligatorio (CNAP, CSS LU Libro II), por reparto. Tipo 16 % total (8 % trabajador, 8 % empleador); el Estado añade un tercio. Base limitada a 5 × SSM (≈ {plafond} €/mes en {annee}). Pensión completa tras 40 años; jubilación a los 65 (o 57 anticipada).`

#### LU_AM
- **libelle** — fr: `AM — Assurance maladie-maternité (CNS)` · en: `AM — Health-maternity insurance (CNS)` · de: `AM — Kranken-Mutterschaftsversicherung (CNS)` · nl: `AM — Ziekte-moederschapsverzekering (CNS)` · it: `AM — Assicurazione malattia-maternità (CNS)` · es: `AM — Seguro de enfermedad-maternidad (CNS)`
- **explication** (placeholders `{plafond} {annee}`) :
  - fr: `Assurance maladie-maternité (CNS) : soins de santé + indemnités pécuniaires (100 % du salaire 52 semaines, puis 80 % jusqu'à 78). Cotisation 3,05 % (soins 2,80 % + indemnités 0,25 %). Assiette plafonnée à 5 × SSM (≈ {plafond} €/mois en {annee}). Tiers payant généralisé depuis 2010.`
  - en: `Health-maternity insurance (CNS): healthcare + cash benefits (100 % of pay for 52 weeks, then 80 % up to 78). Contribution 3.05 % (care 2.80 % + cash 0.25 %). Base capped at 5 × SSM (≈ {plafond} €/month in {annee}). Generalised third-party payment since 2010.`
  - de: `Kranken-Mutterschaftsversicherung (CNS): Gesundheitsversorgung + Geldleistungen (100 % des Lohns 52 Wochen, dann 80 % bis 78). Beitrag 3,05 % (Pflege 2,80 % + Geld 0,25 %). Bemessung gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in {annee}). Sachleistungsprinzip seit 2010.`
  - nl: `Ziekte-moederschapsverzekering (CNS): zorg + uitkeringen (100 % van het loon 52 weken, daarna 80 % tot 78). Bijdrage 3,05 % (zorg 2,80 % + uitkering 0,25 %). Grondslag begrensd op 5 × SSM (≈ {plafond} €/maand in {annee}). Algemeen derdebetalerssysteem sinds 2010.`
  - it: `Assicurazione malattia-maternità (CNS): assistenza sanitaria + indennità (100 % della retribuzione 52 settimane, poi 80 % fino a 78). Contributo 3,05 % (cure 2,80 % + indennità 0,25 %). Base limitata a 5 × SSM (≈ {plafond} €/mese in {annee}). Terzo pagante generalizzato dal 2010.`
  - es: `Seguro de enfermedad-maternidad (CNS): asistencia sanitaria + prestaciones (100 % del salario 52 semanas, luego 80 % hasta 78). Cotización 3,05 % (atención 2,80 % + prestaciones 0,25 %). Base limitada a 5 × SSM (≈ {plafond} €/mes en {annee}). Tercero pagador generalizado desde 2010.`

#### LU_AD
- **libelle** — fr: `AD — Assurance dépendance` · en: `AD — Long-term care insurance` · de: `AD — Pflegeversicherung` · nl: `AD — Langdurigezorgverzekering` · it: `AD — Assicurazione dipendenza` · es: `AD — Seguro de dependencia`
- **explication** (placeholders `{plafond} {annee}`) :
  - fr: `Assurance dépendance (loi du 19/06/1998) : prestations pour les personnes non autonomes. Originalité : cotisation 100 % salariale (1,40 %), sans part patronale. Plafonnée à 5 × SSM (≈ {plafond} €/mois en {annee}). Gestion CNS.`
  - en: `Long-term care insurance (law of 19/06/1998): benefits for dependent persons. Notably 100 % employee contribution (1.40 %), no employer share. Capped at 5 × SSM (≈ {plafond} €/month in {annee}). Managed by CNS.`
  - de: `Pflegeversicherung (Gesetz vom 19.06.1998): Leistungen für pflegebedürftige Personen. Besonderheit: 100 % Arbeitnehmerbeitrag (1,40 %), kein Arbeitgeberanteil. Gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in {annee}). Verwaltung CNS.`
  - nl: `Langdurigezorgverzekering (wet 19-06-1998): uitkeringen voor zorgafhankelijke personen. Bijzonderheid: 100 % werknemersbijdrage (1,40 %), geen werkgeversdeel. Begrensd op 5 × SSM (≈ {plafond} €/maand in {annee}). Beheer CNS.`
  - it: `Assicurazione dipendenza (legge 19/06/1998): prestazioni per persone non autonome. Particolarità: contributo 100 % dipendente (1,40 %), senza quota datoriale. Limitata a 5 × SSM (≈ {plafond} €/mese in {annee}). Gestione CNS.`
  - es: `Seguro de dependencia (ley 19/06/1998): prestaciones para personas dependientes. Particularidad: cotización 100 % del trabajador (1,40 %), sin parte patronal. Limitada a 5 × SSM (≈ {plafond} €/mes en {annee}). Gestión CNS.`

#### LU_AA
- **libelle** — fr: `AA — Assurance accidents (AAA)` · en: `AA — Accident insurance (AAA)` · de: `AA — Unfallversicherung (AAA)` · nl: `AA — Ongevallenverzekering (AAA)` · it: `AA — Assicurazione infortuni (AAA)` · es: `AA — Seguro de accidentes (AAA)`
- **explication** (placeholders `{} {plafond} {annee}` — premier `{}` = taux) :
  - fr: `Assurance accidents obligatoire (AAA), accidents du travail et maladies professionnelles, 100 % patronale. Taux {} % indicatif (tertiaire) ; 3 à 10× plus élevé dans les secteurs à risque. Plafonnée à 5 × SSM (≈ {plafond} €/mois en {annee}). CSS LU Livre III.`
  - en: `Mandatory accident insurance (AAA), work accidents and occupational diseases, 100 % employer. Rate {} % indicative (services); 3–10× higher in high-risk sectors. Capped at 5 × SSM (≈ {plafond} €/month in {annee}). CSS LU Book III.`
  - de: `Obligatorische Unfallversicherung (AAA), Arbeitsunfälle und Berufskrankheiten, 100 % Arbeitgeber. Satz {} % indikativ (Dienstleistung); in Risikobranchen 3–10× höher. Gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in {annee}). CSS LU Buch III.`
  - nl: `Verplichte ongevallenverzekering (AAA), arbeidsongevallen en beroepsziekten, 100 % werkgever. Tarief {} % indicatief (diensten); 3–10× hoger in risicosectoren. Begrensd op 5 × SSM (≈ {plafond} €/maand in {annee}). CSS LU Boek III.`
  - it: `Assicurazione infortuni obbligatoria (AAA), infortuni sul lavoro e malattie professionali, 100 % datoriale. Aliquota {} % indicativa (terziario); 3–10× più alta nei settori a rischio. Limitata a 5 × SSM (≈ {plafond} €/mese in {annee}). CSS LU Libro III.`
  - es: `Seguro de accidentes obligatorio (AAA), accidentes laborales y enfermedades profesionales, 100 % patronal. Tipo {} % indicativo (servicios); 3–10× más alto en sectores de riesgo. Limitado a 5 × SSM (≈ {plafond} €/mes en {annee}). CSS LU Libro III.`

#### LU_ME
- **libelle** — fr: `ME — Mutualité des employeurs` · en: `ME — Employers' mutual fund` · de: `ME — Arbeitgeber-Ausgleichskasse` · nl: `ME — Werkgeversmutualiteit` · it: `ME — Mutua dei datori di lavoro` · es: `ME — Mutualidad de empleadores`
- **explication** (placeholders `{} {plafond} {annee}` — premier `{}` = taux) :
  - fr: `Mutualité des employeurs (CCSS) : mécanisme de solidarité remboursant aux employeurs le salaire maintenu (jours 1 à 77 de maladie), la CNS prenant le relais au 78ᵉ. Taux {} % indicatif (tertiaire). Plafonnée à 5 × SSM (≈ {plafond} €/mois en {annee}).`
  - en: `Employers' mutual fund (CCSS): solidarity scheme reimbursing employers for continued pay (sick days 1–77), CNS taking over from day 78. Rate {} % indicative (services). Capped at 5 × SSM (≈ {plafond} €/month in {annee}).`
  - de: `Arbeitgeber-Ausgleichskasse (CCSS): Solidarsystem, das Arbeitgebern die Lohnfortzahlung erstattet (Krankheitstage 1–77), CNS übernimmt ab Tag 78. Satz {} % indikativ (Dienstleistung). Gedeckelt auf 5 × SSM (≈ {plafond} €/Monat in {annee}).`
  - nl: `Werkgeversmutualiteit (CCSS): solidariteitsmechanisme dat werkgevers het doorbetaalde loon vergoedt (ziektedagen 1–77), CNS neemt over vanaf dag 78. Tarief {} % indicatief (diensten). Begrensd op 5 × SSM (≈ {plafond} €/maand in {annee}).`
  - it: `Mutua dei datori di lavoro (CCSS): meccanismo di solidarietà che rimborsa ai datori la retribuzione mantenuta (giorni di malattia 1–77), la CNS subentra dal 78°. Aliquota {} % indicativa (terziario). Limitata a 5 × SSM (≈ {plafond} €/mese in {annee}).`
  - es: `Mutualidad de empleadores (CCSS): mecanismo de solidaridad que reembolsa a los empleadores el salario mantenido (días de baja 1–77), la CNS toma el relevo desde el 78. Tipo {} % indicativo (servicios). Limitada a 5 × SSM (≈ {plafond} €/mes en {annee}).`

### 🇪🇸 Espagne

> Placeholders communs : `{base_min} {base_max} {annee} {ts_pct} {tp_pct} {ms} {mp} {total} {tot} {base}`.

#### ES_CC
- **libelle** — fr: `Contingencias Comunes — maladie, maternité, retraite` · en: `Contingencias Comunes — sickness, maternity, pension` · de: `Contingencias Comunes — Kranken, Mutterschaft, Rente` · nl: `Contingencias Comunes — ziekte, moederschap, pensioen` · it: `Contingencias Comunes — malattia, maternità, pensione` · es: `Contingencias Comunes — enfermedad, maternidad, jubilación`
- **explication** (placeholders `{base_min} {base_max} {annee} {base} {ts_pct} {ms} {tp_pct} {mp} {total} {tot}`) :
  - fr: `Cotisation principale du régime général espagnol (maladie, maternité, incapacité, retraite, décès/survie). Assiette bornée entre {base_min} € et {base_max} € en {annee} ; retenue : {base} €.\nSalarié : {ts_pct} % = {ms} € | Employeur : {tp_pct} % = {mp} € | Total : {total} % = {tot} €.\nLGSS art. 143-144. Taux stables depuis 2015 (4,70 + 23,60 = 28,30 %).`
  - en: `Main contribution of the Spanish general scheme (sickness, maternity, disability, pension, death/survivors). Base bounded between {base_min} € and {base_max} € in {annee}; applied: {base} €.\nEmployee: {ts_pct} % = {ms} € | Employer: {tp_pct} % = {mp} € | Total: {total} % = {tot} €.\nLGSS art. 143-144. Rates stable since 2015 (4.70 + 23.60 = 28.30 %).`
  - de: `Hauptbeitrag des spanischen Allgemeinsystems (Kranken, Mutterschaft, Invalidität, Rente, Tod/Hinterbliebene). Bemessung zwischen {base_min} € und {base_max} € in {annee}; angewandt: {base} €.\nArbeitnehmer: {ts_pct} % = {ms} € | Arbeitgeber: {tp_pct} % = {mp} € | Gesamt: {total} % = {tot} €.\nLGSS Art. 143-144. Sätze seit 2015 stabil (4,70 + 23,60 = 28,30 %).`
  - nl: `Hoofdbijdrage van het Spaanse algemene stelsel (ziekte, moederschap, invaliditeit, pensioen, overlijden/nabestaanden). Grondslag tussen {base_min} € en {base_max} € in {annee}; toegepast: {base} €.\nWerknemer: {ts_pct} % = {ms} € | Werkgever: {tp_pct} % = {mp} € | Totaal: {total} % = {tot} €.\nLGSS art. 143-144. Tarieven stabiel sinds 2015 (4,70 + 23,60 = 28,30 %).`
  - it: `Contributo principale del regime generale spagnolo (malattia, maternità, invalidità, pensione, morte/superstiti). Base compresa tra {base_min} € e {base_max} € nel {annee}; applicata: {base} €.\nDipendente: {ts_pct} % = {ms} € | Datore: {tp_pct} % = {mp} € | Totale: {total} % = {tot} €.\nLGSS art. 143-144. Aliquote stabili dal 2015 (4,70 + 23,60 = 28,30 %).`
  - es: `Cotización principal del régimen general español (enfermedad, maternidad, incapacidad, jubilación, muerte/supervivencia). Base acotada entre {base_min} € y {base_max} € en {annee}; aplicada: {base} €.\nTrabajador: {ts_pct} % = {ms} € | Empleador: {tp_pct} % = {mp} € | Total: {total} % = {tot} €.\nLGSS art. 143-144. Tipos estables desde 2015 (4,70 + 23,60 = 28,30 %).`

#### ES_DESEMPLEO
- **libelle** — fr: `Desempleo — assurance chômage (contrato indefinido)` · en: `Desempleo — unemployment insurance (permanent contract)` · de: `Desempleo — Arbeitslosenversicherung (unbefristet)` · nl: `Desempleo — werkloosheidsverzekering (vast contract)` · it: `Desempleo — assicurazione disoccupazione (contratto indeterminato)` · es: `Desempleo — seguro de desempleo (contrato indefinido)`
- **explication** (placeholders `{ts_pct} {base} {ms} {tp_pct} {mp} {total} {tot}`) :
  - fr: `Cotisation chômage pour contrat indéterminé (SEPE). Salarié : {ts_pct} % × {base} € = {ms} € | Employeur : {tp_pct} % = {mp} € | Total : {total} % = {tot} €.\nLGSS art. 270.`
  - en: `Unemployment contribution for permanent contracts (SEPE). Employee: {ts_pct} % × {base} € = {ms} € | Employer: {tp_pct} % = {mp} € | Total: {total} % = {tot} €.\nLGSS art. 270.`
  - de: `Arbeitslosenbeitrag für unbefristete Verträge (SEPE). Arbeitnehmer: {ts_pct} % × {base} € = {ms} € | Arbeitgeber: {tp_pct} % = {mp} € | Gesamt: {total} % = {tot} €.\nLGSS Art. 270.`
  - nl: `Werkloosheidsbijdrage voor vaste contracten (SEPE). Werknemer: {ts_pct} % × {base} € = {ms} € | Werkgever: {tp_pct} % = {mp} € | Totaal: {total} % = {tot} €.\nLGSS art. 270.`
  - it: `Contributo disoccupazione per contratti a tempo indeterminato (SEPE). Dipendente: {ts_pct} % × {base} € = {ms} € | Datore: {tp_pct} % = {mp} € | Totale: {total} % = {tot} €.\nLGSS art. 270.`
  - es: `Cotización por desempleo para contratos indefinidos (SEPE). Trabajador: {ts_pct} % × {base} € = {ms} € | Empleador: {tp_pct} % = {mp} € | Total: {total} % = {tot} €.\nLGSS art. 270.`

#### ES_FOGASA
- **libelle** — fr: `FOGASA — Fondo de Garantía Salarial` (identique 6 langues : terme propre)
- **explication** (placeholders `{tp_pct} {mp}`) :
  - fr: `Fonds de garantie des salaires impayés en cas d'insolvabilité de l'employeur. 100 % patronal : {tp_pct} % = {mp} €. Stable à 0,20 %. ET (RDL 2/2015) art. 33.`
  - en: `Wage guarantee fund for unpaid salaries upon employer insolvency. 100 % employer: {tp_pct} % = {mp} €. Stable at 0.20 %. ET (RDL 2/2015) art. 33.`
  - de: `Lohngarantiefonds für unbezahlte Löhne bei Arbeitgeberinsolvenz. 100 % Arbeitgeber: {tp_pct} % = {mp} €. Stabil bei 0,20 %. ET (RDL 2/2015) Art. 33.`
  - nl: `Loongarantiefonds voor onbetaalde lonen bij insolventie werkgever. 100 % werkgever: {tp_pct} % = {mp} €. Stabiel op 0,20 %. ET (RDL 2/2015) art. 33.`
  - it: `Fondo di garanzia salariale per retribuzioni non pagate in caso di insolvenza del datore. 100 % datoriale: {tp_pct} % = {mp} €. Stabile allo 0,20 %. ET (RDL 2/2015) art. 33.`
  - es: `Fondo de garantía salarial para salarios impagados por insolvencia del empleador. 100 % patronal: {tp_pct} % = {mp} €. Estable en 0,20 %. ET (RDL 2/2015) art. 33.`

#### ES_FP
- **libelle** — fr: `Formación Profesional — formation professionnelle continue` · en: `Formación Profesional — continuing vocational training` · de: `Formación Profesional — berufliche Weiterbildung` · nl: `Formación Profesional — voortgezette beroepsopleiding` · it: `Formación Profesional — formazione professionale continua` · es: `Formación Profesional — formación profesional continua`
- **explication** (placeholders `{ts_pct} {tp_pct} {total} {ms} {mp}`) :
  - fr: `Finance la formation professionnelle continue (FUNDAE). Salarié : {ts_pct} % — Employeur : {tp_pct} % — Total : {total} %.\nSalarié : {ms} € — Employeur : {mp} €. LGSS art. 7 et DA 19a.`
  - en: `Funds continuing vocational training (FUNDAE). Employee: {ts_pct} % — Employer: {tp_pct} % — Total: {total} %.\nEmployee: {ms} € — Employer: {mp} €. LGSS art. 7 and DA 19a.`
  - de: `Finanziert berufliche Weiterbildung (FUNDAE). Arbeitnehmer: {ts_pct} % — Arbeitgeber: {tp_pct} % — Gesamt: {total} %.\nArbeitnehmer: {ms} € — Arbeitgeber: {mp} €. LGSS Art. 7 und DA 19a.`
  - nl: `Financiert voortgezette beroepsopleiding (FUNDAE). Werknemer: {ts_pct} % — Werkgever: {tp_pct} % — Totaal: {total} %.\nWerknemer: {ms} € — Werkgever: {mp} €. LGSS art. 7 en DA 19a.`
  - it: `Finanzia la formazione professionale continua (FUNDAE). Dipendente: {ts_pct} % — Datore: {tp_pct} % — Totale: {total} %.\nDipendente: {ms} € — Datore: {mp} €. LGSS art. 7 e DA 19a.`
  - es: `Financia la formación profesional continua (FUNDAE). Trabajador: {ts_pct} % — Empleador: {tp_pct} % — Total: {total} %.\nTrabajador: {ms} € — Empleador: {mp} €. LGSS art. 7 y DA 19a.`

#### ES_MEI
- **libelle** — fr: `MEI — Mecanismo de Equidad Intergeneracional {annee}` (identique 6 langues : terme propre + `{annee}`)
- **explication** (placeholders `{annee} {ts_pct} {tp_pct} {total} {ms} {mp}`) :
  - fr: `Cotisation additionnelle (Ley 21/2021) alimentant le Fonds de réserve des retraites ; taux croissant jusqu'en 2032. {annee} : salarié {ts_pct} % + employeur {tp_pct} % = {total} %.\nSalarié : {ms} € — Employeur : {mp} €. En vigueur depuis le 01/01/2023.`
  - en: `Additional contribution (Ley 21/2021) feeding the pension Reserve Fund; rate rising until 2032. {annee}: employee {ts_pct} % + employer {tp_pct} % = {total} %.\nEmployee: {ms} € — Employer: {mp} €. In force since 01/01/2023.`
  - de: `Zusatzbeitrag (Ley 21/2021) zur Speisung des Renten-Reservefonds; steigender Satz bis 2032. {annee}: Arbeitnehmer {ts_pct} % + Arbeitgeber {tp_pct} % = {total} %.\nArbeitnehmer: {ms} € — Arbeitgeber: {mp} €. In Kraft seit 01.01.2023.`
  - nl: `Aanvullende bijdrage (Ley 21/2021) voor het pensioenreservefonds; stijgend tarief tot 2032. {annee}: werknemer {ts_pct} % + werkgever {tp_pct} % = {total} %.\nWerknemer: {ms} € — Werkgever: {mp} €. Van kracht sinds 01-01-2023.`
  - it: `Contributo aggiuntivo (Ley 21/2021) che alimenta il Fondo di riserva delle pensioni; aliquota crescente fino al 2032. {annee}: dipendente {ts_pct} % + datore {tp_pct} % = {total} %.\nDipendente: {ms} € — Datore: {mp} €. In vigore dal 01/01/2023.`
  - es: `Cotización adicional (Ley 21/2021) que alimenta el Fondo de Reserva de las pensiones; tipo creciente hasta 2032. {annee}: trabajador {ts_pct} % + empleador {tp_pct} % = {total} %.\nTrabajador: {ms} € — Empleador: {mp} €. En vigor desde el 01/01/2023.`

### 🇵🇹 Portugal

#### PT_SS
- **libelle** — fr: `Segurança Social — Taxa Social Única (TSU)` (identique 6 langues : terme propre)
- **explication** (placeholders `{ts_pct} {brut} {ms} {tp_pct} {mp} {total} {tot}`) :
  - fr: `Cotisation principale du régime général (TSU) : maladie/maternité, invalidité, retraite, survie, chômage. Assiette = brut intégral, sans plafond.\nSalarié : {ts_pct} % × {brut} € = {ms} € | Employeur : {tp_pct} % = {mp} € | Total : {total} % = {tot} €.\nStable depuis 2013. Lei 110/2009 art. 53-54.`
  - en: `Main general-scheme contribution (TSU): sickness/maternity, disability, pension, survivors, unemployment. Base = full gross, no cap.\nEmployee: {ts_pct} % × {brut} € = {ms} € | Employer: {tp_pct} % = {mp} € | Total: {total} % = {tot} €.\nStable since 2013. Lei 110/2009 art. 53-54.`
  - de: `Hauptbeitrag des Allgemeinsystems (TSU): Kranken/Mutterschaft, Invalidität, Rente, Hinterbliebene, Arbeitslosigkeit. Bemessung = volles Brutto, ohne Obergrenze.\nArbeitnehmer: {ts_pct} % × {brut} € = {ms} € | Arbeitgeber: {tp_pct} % = {mp} € | Gesamt: {total} % = {tot} €.\nStabil seit 2013. Lei 110/2009 art. 53-54.`
  - nl: `Hoofdbijdrage van het algemene stelsel (TSU): ziekte/moederschap, invaliditeit, pensioen, nabestaanden, werkloosheid. Grondslag = volledig bruto, geen plafond.\nWerknemer: {ts_pct} % × {brut} € = {ms} € | Werkgever: {tp_pct} % = {mp} € | Totaal: {total} % = {tot} €.\nStabiel sinds 2013. Lei 110/2009 art. 53-54.`
  - it: `Contributo principale del regime generale (TSU): malattia/maternità, invalidità, pensione, superstiti, disoccupazione. Base = lordo intero, senza massimale.\nDipendente: {ts_pct} % × {brut} € = {ms} € | Datore: {tp_pct} % = {mp} € | Totale: {total} % = {tot} €.\nStabile dal 2013. Lei 110/2009 art. 53-54.`
  - es: `Cotización principal del régimen general (TSU): enfermedad/maternidad, invalidez, jubilación, supervivencia, desempleo. Base = bruto íntegro, sin tope.\nTrabajador: {ts_pct} % × {brut} € = {ms} € | Empleador: {tp_pct} % = {mp} € | Total: {total} % = {tot} €.\nEstable desde 2013. Lei 110/2009 art. 53-54.`

#### PT_AT_SEG
- **libelle** — fr: `Acidentes de Trabalho — assurance accidents du travail` · en: `Acidentes de Trabalho — work accident insurance` · de: `Acidentes de Trabalho — Arbeitsunfallversicherung` · nl: `Acidentes de Trabalho — arbeidsongevallenverzekering` · it: `Acidentes de Trabalho — assicurazione infortuni sul lavoro` · es: `Acidentes de Trabalho — seguro de accidentes de trabajo`
- **explication** (placeholders `{tp_pct} {mp}`) :
  - fr: `Assurance obligatoire des accidents du travail et maladies professionnelles, 100 % patronale. Taux {tp_pct} % indicatif (tertiaire) ; de 0,5 % (bureau) à 10 %+ (BTP). Employeur : {mp} €. Lei 98/2009 art. 79.`
  - en: `Mandatory work-accident and occupational-disease insurance, 100 % employer. Rate {tp_pct} % indicative (services); from 0.5 % (office) to 10 %+ (construction). Employer: {mp} €. Lei 98/2009 art. 79.`
  - de: `Pflicht-Arbeitsunfall- und Berufskrankheitenversicherung, 100 % Arbeitgeber. Satz {tp_pct} % indikativ (Dienstleistung); von 0,5 % (Büro) bis 10 %+ (Bau). Arbeitgeber: {mp} €. Lei 98/2009 art. 79.`
  - nl: `Verplichte arbeidsongevallen- en beroepsziekteverzekering, 100 % werkgever. Tarief {tp_pct} % indicatief (diensten); van 0,5 % (kantoor) tot 10 %+ (bouw). Werkgever: {mp} €. Lei 98/2009 art. 79.`
  - it: `Assicurazione obbligatoria infortuni sul lavoro e malattie professionali, 100 % datoriale. Aliquota {tp_pct} % indicativa (terziario); da 0,5 % (ufficio) a 10 %+ (edilizia). Datore: {mp} €. Lei 98/2009 art. 79.`
  - es: `Seguro obligatorio de accidentes de trabajo y enfermedades profesionales, 100 % patronal. Tipo {tp_pct} % indicativo (servicios); de 0,5 % (oficina) a 10 %+ (construcción). Empleador: {mp} €. Lei 98/2009 art. 79.`

#### PT_FCT
- **libelle** — fr: `FCT — Fundo de Compensação do Trabalho` (identique 6 langues : terme propre)
- **explication** (placeholders `{tp_pct} {mp}`) :
  - fr: `Fonds couvrant 50 % des indemnités de licenciement en cas d'insolvabilité (CDI postérieurs au 01/10/2013). 100 % patronal : {tp_pct} %. Employeur : {mp} €. DL 210/2015 art. 4.`
  - en: `Fund covering 50 % of severance pay on insolvency (permanent contracts after 01/10/2013). 100 % employer: {tp_pct} %. Employer: {mp} €. DL 210/2015 art. 4.`
  - de: `Fonds, der 50 % der Abfindung bei Insolvenz abdeckt (unbefristete Verträge nach dem 01.10.2013). 100 % Arbeitgeber: {tp_pct} %. Arbeitgeber: {mp} €. DL 210/2015 art. 4.`
  - nl: `Fonds dat 50 % van de ontslagvergoeding dekt bij insolventie (vaste contracten na 01-10-2013). 100 % werkgever: {tp_pct} %. Werkgever: {mp} €. DL 210/2015 art. 4.`
  - it: `Fondo che copre il 50 % dell'indennità di licenziamento in caso di insolvenza (contratti a tempo indeterminato dopo il 01/10/2013). 100 % datoriale: {tp_pct} %. Datore: {mp} €. DL 210/2015 art. 4.`
  - es: `Fondo que cubre el 50 % de la indemnización por despido en caso de insolvencia (contratos indefinidos posteriores al 01/10/2013). 100 % patronal: {tp_pct} %. Empleador: {mp} €. DL 210/2015 art. 4.`

#### PT_FGCT
- **libelle** — fr: `FGCT — Fundo de Garantia de Compensação do Trabalho` (identique 6 langues : terme propre)
- **explication** (placeholders `{tp_pct} {mp}`) :
  - fr: `Fonds garantissant les 50 % restants des indemnités de licenciement non couverts par le FCT. 100 % patronal : {tp_pct} %. Employeur : {mp} €. DL 210/2015 art. 5.`
  - en: `Fund guaranteeing the remaining 50 % of severance not covered by the FCT. 100 % employer: {tp_pct} %. Employer: {mp} €. DL 210/2015 art. 5.`
  - de: `Fonds, der die restlichen 50 % der vom FCT nicht gedeckten Abfindung garantiert. 100 % Arbeitgeber: {tp_pct} %. Arbeitgeber: {mp} €. DL 210/2015 art. 5.`
  - nl: `Fonds dat de resterende 50 % van de ontslagvergoeding garandeert die niet door het FCT wordt gedekt. 100 % werkgever: {tp_pct} %. Werkgever: {mp} €. DL 210/2015 art. 5.`
  - it: `Fondo che garantisce il restante 50 % dell'indennità non coperta dal FCT. 100 % datoriale: {tp_pct} %. Datore: {mp} €. DL 210/2015 art. 5.`
  - es: `Fondo que garantiza el 50 % restante de la indemnización no cubierta por el FCT. 100 % patronal: {tp_pct} %. Empleador: {mp} €. DL 210/2015 art. 5.`

> **Portugal — IRS** (`pt_irs.rs`) : barème de retenue à la source, libellé + explication à
> extraire au câblage (placeholders à préserver). À traduire selon la même convention.

### 🇨🇭 Suisse

#### CH_AVS
- **libelle** — fr: `AVS — Assurance-vieillesse et survivants` · en: `AVS — Old-age and survivors insurance` · de: `AVS — Alters- und Hinterlassenenversicherung` · nl: `AVS — Ouderdoms- en nabestaandenverzekering` · it: `AVS — Assicurazione vecchiaia e superstiti` · es: `AVS — Seguro de vejez y supervivencia`
- **explication** (sans placeholder) :
  - fr: `L'AVS (1er pilier, LAVS 1948) est financée par répartition et verse une rente dès 65 ans (réforme AVS 21, 01/01/2024). Taux 8,70 % depuis 2020 (financement RFFA) — 4,35 % chacun. Assiette = brut total, sans plafond. Rente max 2025 : CHF 2 590/mois ; min : CHF 1 225.`
  - en: `AVS (1st pillar, LAVS 1948) is pay-as-you-go and pays a pension from age 65 (AVS 21 reform, 01/01/2024). Rate 8.70 % since 2020 (RFFA funding) — 4.35 % each. Base = full gross, no cap. Max pension 2025: CHF 2,590/month; min: CHF 1,225.`
  - de: `Die AVS (1. Säule, LAVS 1948) ist umlagefinanziert und zahlt ab 65 eine Rente (Reform AHV 21, 01.01.2024). Satz 8,70 % seit 2020 (STAF-Finanzierung) — je 4,35 %. Bemessung = volles Brutto, ohne Obergrenze. Max. Rente 2025: CHF 2'590/Monat; min.: CHF 1'225.`
  - nl: `De AVS (1e pijler, LAVS 1948) is omslaggefinancierd en betaalt pensioen vanaf 65 (hervorming AHV 21, 01-01-2024). Tarief 8,70 % sinds 2020 (RFFA-financiering) — elk 4,35 %. Grondslag = volledig bruto, geen plafond. Max. pensioen 2025: CHF 2.590/maand; min.: CHF 1.225.`
  - it: `L'AVS (1º pilastro, LAVS 1948) è a ripartizione e versa una rendita dai 65 anni (riforma AVS 21, 01/01/2024). Aliquota 8,70 % dal 2020 (finanziamento RFFA) — 4,35 % ciascuno. Base = lordo totale, senza massimale. Rendita max 2025: CHF 2'590/mese; min: CHF 1'225.`
  - es: `El AVS (1er pilar, LAVS 1948) es de reparto y paga una pensión desde los 65 años (reforma AVS 21, 01/01/2024). Tipo 8,70 % desde 2020 (financiación RFFA) — 4,35 % cada uno. Base = bruto total, sin tope. Pensión máx 2025: CHF 2.590/mes; mín: CHF 1.225.`

#### CH_AI
- **libelle** — fr: `AI — Assurance invalidité` · en: `AI — Disability insurance` · de: `AI — Invalidenversicherung` · nl: `AI — Invaliditeitsverzekering` · it: `AI — Assicurazione invalidità` · es: `AI — Seguro de invalidez`
- **explication** (sans placeholder) :
  - fr: `L'AI (LAI 1959) complète le 1er pilier : rentes et mesures de réadaptation pour capacité de gain durablement réduite. Taux stable 1,40 % (0,70 % chacun), brut total sans plafond. La révision AI (2022) priorise la réinsertion ('la réadaptation prime sur la rente').`
  - en: `AI (LAI 1959) supplements the 1st pillar: pensions and rehabilitation measures for lastingly reduced earning capacity. Stable rate 1.40 % (0.70 % each), full gross with no cap. The 2022 AI revision prioritises reintegration ('rehabilitation before pension').`
  - de: `Die IV (IVG 1959) ergänzt die 1. Säule: Renten und Eingliederungsmassnahmen bei dauerhaft verminderter Erwerbsfähigkeit. Stabiler Satz 1,40 % (je 0,70 %), volles Brutto ohne Obergrenze. Die IV-Revision 2022 priorisiert die Eingliederung ('Eingliederung vor Rente').`
  - nl: `De AI (LAI 1959) vult de 1e pijler aan: uitkeringen en re-integratiemaatregelen bij duurzaam verminderd verdienvermogen. Stabiel tarief 1,40 % (elk 0,70 %), volledig bruto zonder plafond. De AI-herziening 2022 geeft prioriteit aan re-integratie ('re-integratie vóór rente').`
  - it: `L'AI (LAI 1959) integra il 1º pilastro: rendite e misure di reinserimento per capacità di guadagno durevolmente ridotta. Aliquota stabile 1,40 % (0,70 % ciascuno), lordo totale senza massimale. La revisione AI 2022 dà priorità al reinserimento ('riabilitazione prima della rendita').`
  - es: `El AI (LAI 1959) complementa el 1er pilar: rentas y medidas de reinserción por capacidad de ganancia duraderamente reducida. Tipo estable 1,40 % (0,70 % cada uno), bruto total sin tope. La revisión AI 2022 prioriza la reinserción ('la rehabilitación antes que la renta').`

#### CH_APG
- **libelle** — fr: `APG — Allocations pour perte de gain` · en: `APG — Loss-of-earnings allowances` · de: `APG — Erwerbsersatzordnung` · nl: `APG — Inkomstenvervangende uitkeringen` · it: `APG — Indennità per perdita di guadagno` · es: `APG — Subsidios por pérdida de ganancia`
- **explication** (sans placeholder) :
  - fr: `Les APG (LAPG) compensent la perte de gain durant service militaire/civil/protection civile, maternité et, depuis 2021, congé paternité (2 semaines). Taux 0,50 % (0,25 % chacun), brut total sans plafond. Souvent groupées AVS/AI/APG = 10,60 % (5,30 % chacun).`
  - en: `APG (LAPG) compensate loss of earnings during military/civil/civil-protection service, maternity and, since 2021, paternity leave (2 weeks). Rate 0.50 % (0.25 % each), full gross with no cap. Often grouped AVS/AI/APG = 10.60 % (5.30 % each).`
  - de: `Die EO (EOG) ersetzt den Erwerbsausfall bei Militär-/Zivil-/Zivilschutzdienst, Mutterschaft und seit 2021 Vaterschaftsurlaub (2 Wochen). Satz 0,50 % (je 0,25 %), volles Brutto ohne Obergrenze. Oft AHV/IV/EO = 10,60 % (je 5,30 %).`
  - nl: `De APG (LAPG) compenseert inkomstenverlies tijdens militaire/civiele/civiele-beschermingsdienst, moederschap en sinds 2021 vaderschapsverlof (2 weken). Tarief 0,50 % (elk 0,25 %), volledig bruto zonder plafond. Vaak AVS/AI/APG = 10,60 % (elk 5,30 %).`
  - it: `Le APG (LAPG) compensano la perdita di guadagno durante servizio militare/civile/protezione civile, maternità e, dal 2021, congedo di paternità (2 settimane). Aliquota 0,50 % (0,25 % ciascuno), lordo totale senza massimale. Spesso AVS/AI/APG = 10,60 % (5,30 % ciascuno).`
  - es: `Los APG (LAPG) compensan la pérdida de ganancia durante el servicio militar/civil/protección civil, maternidad y, desde 2021, permiso de paternidad (2 semanas). Tipo 0,50 % (0,25 % cada uno), bruto total sin tope. A menudo AVS/AI/APG = 10,60 % (5,30 % cada uno).`

#### CH_AC
- **libelle** — fr: `AC — Assurance-chômage` · en: `AC — Unemployment insurance` · de: `AC — Arbeitslosenversicherung` · nl: `AC — Werkloosheidsverzekering` · it: `AC — Assicurazione contro la disoccupazione` · es: `AC — Seguro de desempleo`
- **explication** (placeholders `{}` = plafond mensuel) :
  - fr: `L'AC (LACI 1982) verse 70 à 80 % du gain assuré. Taux 2,20 % (1,10 % chacun), plafonné à CHF 148 200/an (CHF {} /mois) ; au-delà, pas de cotisation. Administrée par le SECO, les cantons et les caisses.`
  - en: `AC (LACI 1982) pays 70–80 % of insured earnings. Rate 2.20 % (1.10 % each), capped at CHF 148,200/yr (CHF {} /month); above that, no contribution. Administered by SECO, cantons and funds.`
  - de: `Die ALV (AVIG 1982) zahlt 70–80 % des versicherten Verdienstes. Satz 2,20 % (je 1,10 %), gedeckelt auf CHF 148'200/Jahr (CHF {} /Monat); darüber kein Beitrag. Verwaltet von SECO, Kantonen und Kassen.`
  - nl: `De AC (LACI 1982) betaalt 70–80 % van het verzekerde loon. Tarief 2,20 % (elk 1,10 %), begrensd op CHF 148.200/jr (CHF {} /maand); daarboven geen bijdrage. Beheerd door SECO, kantons en kassen.`
  - it: `L'AC (LADI 1982) versa il 70–80 % del guadagno assicurato. Aliquota 2,20 % (1,10 % ciascuno), limitata a CHF 148'200/anno (CHF {} /mese); oltre, nessun contributo. Gestita da SECO, cantoni e casse.`
  - es: `El AC (LACI 1982) paga el 70–80 % de la ganancia asegurada. Tipo 2,20 % (1,10 % cada uno), limitado a CHF 148.200/año (CHF {} /mes); por encima, sin cotización. Administrado por SECO, cantones y cajas.`

#### CH_AANP
- **libelle** — fr: `AANP — Assurance accidents non professionnels` · en: `AANP — Non-occupational accident insurance` · de: `NBU — Nichtberufsunfallversicherung` · nl: `AANP — Verzekering niet-beroepsongevallen` · it: `AINP — Assicurazione infortuni non professionali` · es: `AANP — Seguro de accidentes no profesionales`
- **explication** (placeholders `{}` = plafond, `{}` = taux) :
  - fr: `La LAA distingue accidents professionnels (AAP, employeur) et non professionnels (AANP, salarié). L'AANP couvre les accidents hors travail. À la charge du salarié. Assiette plafonnée à CHF 148 200/an (CHF {} /mois). Taux {} % fixé par l'assureur (SUVA/privé) — indicatif bureau.`
  - en: `LAA distinguishes occupational (AAP, employer) and non-occupational (AANP, employee) accidents. AANP covers off-work accidents. Employee-borne. Base capped at CHF 148,200/yr (CHF {} /month). Rate {} % set by the insurer (SUVA/private) — office indicative.`
  - de: `Das UVG unterscheidet Berufsunfälle (BU, Arbeitgeber) und Nichtberufsunfälle (NBU, Arbeitnehmer). Die NBU deckt Unfälle ausserhalb der Arbeit. Vom Arbeitnehmer getragen. Bemessung gedeckelt auf CHF 148'200/Jahr (CHF {} /Monat). Satz {} % vom Versicherer (SUVA/privat) — Büro indikativ.`
  - nl: `De LAA onderscheidt beroepsongevallen (AAP, werkgever) en niet-beroepsongevallen (AANP, werknemer). AANP dekt ongevallen buiten het werk. Door werknemer gedragen. Grondslag begrensd op CHF 148.200/jr (CHF {} /maand). Tarief {} % bepaald door verzekeraar (SUVA/privé) — kantoor indicatief.`
  - it: `La LAINF distingue infortuni professionali (AAP, datore) e non professionali (AINP, dipendente). L'AINP copre gli infortuni fuori dal lavoro. A carico del dipendente. Base limitata a CHF 148'200/anno (CHF {} /mese). Aliquota {} % fissata dall'assicuratore (SUVA/privato) — ufficio indicativo.`
  - es: `La LAA distingue accidentes profesionales (AAP, empleador) y no profesionales (AANP, trabajador). El AANP cubre accidentes fuera del trabajo. A cargo del trabajador. Base limitada a CHF 148.200/año (CHF {} /mes). Tipo {} % fijado por el asegurador (SUVA/privado) — oficina indicativo.`

#### CH_AAP
- **libelle** — fr: `AAP — Assurance accidents professionnels` · en: `AAP — Occupational accident insurance` · de: `BU — Berufsunfallversicherung` · nl: `AAP — Verzekering beroepsongevallen` · it: `AAP — Assicurazione infortuni professionali` · es: `AAP — Seguro de accidentes profesionales`
- **explication** (placeholders `{}` = plafond, `{}` = taux) :
  - fr: `L'AAP couvre les accidents et maladies professionnels survenus dans le cadre du travail. 100 % employeur. Assiette plafonnée à CHF 148 200/an (CHF {} /mois). Taux {} % fixé par la SUVA/assureur selon la classe de risque (code NOGA). Indicatif tertiaire.`
  - en: `AAP covers occupational accidents and diseases arising at work. 100 % employer. Base capped at CHF 148,200/yr (CHF {} /month). Rate {} % set by SUVA/insurer per risk class (NOGA code). Services indicative.`
  - de: `Die BU deckt Berufsunfälle und -krankheiten am Arbeitsplatz. 100 % Arbeitgeber. Bemessung gedeckelt auf CHF 148'200/Jahr (CHF {} /Monat). Satz {} % von SUVA/Versicherer nach Risikoklasse (NOGA-Code). Dienstleistung indikativ.`
  - nl: `AAP dekt beroepsongevallen en -ziekten op het werk. 100 % werkgever. Grondslag begrensd op CHF 148.200/jr (CHF {} /maand). Tarief {} % bepaald door SUVA/verzekeraar per risicoklasse (NOGA-code). Diensten indicatief.`
  - it: `L'AAP copre infortuni e malattie professionali sul lavoro. 100 % datore. Base limitata a CHF 148'200/anno (CHF {} /mese). Aliquota {} % fissata da SUVA/assicuratore per classe di rischio (codice NOGA). Terziario indicativo.`
  - es: `El AAP cubre accidentes y enfermedades profesionales en el trabajo. 100 % empleador. Base limitada a CHF 148.200/año (CHF {} /mes). Tipo {} % fijado por SUVA/asegurador según clase de riesgo (código NOGA). Servicios indicativo.`

#### CH_IJM
- **libelle** — fr: `IJM — Indemnités journalières maladie (plan collectif)` · en: `IJM — Daily sickness benefits (collective plan)` · de: `KTG — Krankentaggeld (Kollektivvertrag)` · nl: `IJM — Dagvergoeding ziekte (collectief plan)` · it: `IJM — Indennità giornaliera malattia (piano collettivo)` · es: `IJM — Subsidio diario de enfermedad (plan colectivo)`
- **explication** (sans placeholder) :
  - fr: `La Suisse n'a pas d'IJM maladie obligatoire (seule la LAMal de base l'est). Les employeurs souscrivent un plan collectif (LCA ou LAMal art. 67-77) couvrant ~80 % du salaire 720-730 jours. Financement conventionnel ; taux indicatif 1,50 % (0,75 % chacun), variable.`
  - en: `Switzerland has no mandatory daily sickness benefit (only basic LAMal is). Employers take out a collective plan (LCA or LAMal art. 67-77) covering ~80 % of pay for 720-730 days. Contractual funding; indicative rate 1.50 % (0.75 % each), variable.`
  - de: `Die Schweiz kennt kein obligatorisches Krankentaggeld (nur die KVG-Grundversicherung). Arbeitgeber schliessen einen Kollektivvertrag (VVG oder KVG Art. 67-77) ab, der ~80 % des Lohns für 720-730 Tage deckt. Vertragliche Finanzierung; Richtsatz 1,50 % (je 0,75 %), variabel.`
  - nl: `Zwitserland kent geen verplichte ziektedaguitkering (alleen de basis-LAMal). Werkgevers sluiten een collectief plan (LCA of LAMal art. 67-77) dat ~80 % van het loon 720-730 dagen dekt. Contractuele financiering; indicatief tarief 1,50 % (elk 0,75 %), variabel.`
  - it: `La Svizzera non ha un'indennità giornaliera malattia obbligatoria (solo la LAMal di base). I datori stipulano un piano collettivo (LCA o LAMal art. 67-77) che copre ~80 % della retribuzione per 720-730 giorni. Finanziamento convenzionale; aliquota indicativa 1,50 % (0,75 % ciascuno), variabile.`
  - es: `Suiza no tiene un subsidio diario de enfermedad obligatorio (solo la LAMal básica). Los empleadores contratan un plan colectivo (LCA o LAMal art. 67-77) que cubre ~80 % del salario durante 720-730 días. Financiación convencional; tipo indicativo 1,50 % (0,75 % cada uno), variable.`

#### CH_LPP
- **libelle** — fr: `LPP — Prévoyance professionnelle (2ème pilier)` · en: `LPP — Occupational pension (2nd pillar)` · de: `BVG — Berufliche Vorsorge (2. Säule)` · nl: `LPP — Beroepspensioen (2e pijler)` · it: `LPP — Previdenza professionale (2º pilastro)` · es: `LPP — Previsión profesional (2º pilar)`
- **explication** (placeholders `{annee} {coord_min} {coord_ded} {brut} {coord} {coord_max}`) :
  - fr: `La LPP (RS 831.40, 1985) est le 2ème pilier ; obligatoire au-delà de CHF 22 680/an (seuil 2025).\n\n[ Salaire coordonné {annee} ]\nSalaire coordonné = max(CHF {coord_min}, brut − déduction de coordination)\n  = max(CHF {coord_min}, {brut} − CHF {coord_ded}) = CHF {coord}\nPlafonné à CHF {coord_max}/mois.\n\nTaux minimum légal par âge (art. 16) : 25-34 → 7 % ; 35-44 → 10 % ; 45-54 → 15 % ; 55-65 → 18 % (moitié chacun). Cotisations déductibles ; gestion par capitalisation.`
  - en: `LPP (RS 831.40, 1985) is the 2nd pillar; mandatory above CHF 22,680/yr (2025 threshold).\n\n[ Coordinated salary {annee} ]\nCoordinated salary = max(CHF {coord_min}, gross − coordination deduction)\n  = max(CHF {coord_min}, {brut} − CHF {coord_ded}) = CHF {coord}\nCapped at CHF {coord_max}/month.\n\nLegal minimum rate by age (art. 16): 25-34 → 7 %; 35-44 → 10 %; 45-54 → 15 %; 55-65 → 18 % (half each). Deductible contributions; funded (capitalisation).`
  - de: `Das BVG (SR 831.40, 1985) ist die 2. Säule; obligatorisch ab CHF 22'680/Jahr (Schwelle 2025).\n\n[ Koordinierter Lohn {annee} ]\nKoordinierter Lohn = max(CHF {coord_min}, brutto − Koordinationsabzug)\n  = max(CHF {coord_min}, {brut} − CHF {coord_ded}) = CHF {coord}\nGedeckelt auf CHF {coord_max}/Monat.\n\nGesetzlicher Mindestsatz nach Alter (Art. 16): 25-34 → 7 %; 35-44 → 10 %; 45-54 → 15 %; 55-65 → 18 % (je hälftig). Abzugsfähige Beiträge; Kapitaldeckungsverfahren.`
  - nl: `De LPP (SR 831.40, 1985) is de 2e pijler; verplicht boven CHF 22.680/jr (drempel 2025).\n\n[ Gecoördineerd loon {annee} ]\nGecoördineerd loon = max(CHF {coord_min}, bruto − coördinatieaftrek)\n  = max(CHF {coord_min}, {brut} − CHF {coord_ded}) = CHF {coord}\nBegrensd op CHF {coord_max}/maand.\n\nWettelijk minimumtarief naar leeftijd (art. 16): 25-34 → 7 %; 35-44 → 10 %; 45-54 → 15 %; 55-65 → 18 % (elk de helft). Aftrekbare bijdragen; kapitaaldekking.`
  - it: `La LPP (RS 831.40, 1985) è il 2º pilastro; obbligatoria oltre CHF 22'680/anno (soglia 2025).\n\n[ Salario coordinato {annee} ]\nSalario coordinato = max(CHF {coord_min}, lordo − deduzione di coordinamento)\n  = max(CHF {coord_min}, {brut} − CHF {coord_ded}) = CHF {coord}\nLimitato a CHF {coord_max}/mese.\n\nAliquota minima legale per età (art. 16): 25-34 → 7 %; 35-44 → 10 %; 45-54 → 15 %; 55-65 → 18 % (metà ciascuno). Contributi deducibili; gestione a capitalizzazione.`
  - es: `La LPP (RS 831.40, 1985) es el 2º pilar; obligatoria por encima de CHF 22.680/año (umbral 2025).\n\n[ Salario coordinado {annee} ]\nSalario coordinado = máx(CHF {coord_min}, bruto − deducción de coordinación)\n  = máx(CHF {coord_min}, {brut} − CHF {coord_ded}) = CHF {coord}\nLimitado a CHF {coord_max}/mes.\n\nTipo mínimo legal por edad (art. 16): 25-34 → 7 %; 35-44 → 10 %; 45-54 → 15 %; 55-65 → 18 % (mitad cada uno). Cotizaciones deducibles; gestión por capitalización.`

> **Suisse — Impôt à la source** (`ch_is.rs`) : barème cantonal, libellé + explication à
> extraire au câblage. À traduire selon la même convention (placeholders préservés).

<!-- MARKER_SUITE -->







