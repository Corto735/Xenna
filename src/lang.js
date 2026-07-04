// ── Dictionnaires de traduction statiques ─────────────────────────────────────
//
// Utilisés EN PRIORITÉ avant l'API MyMemory (limite 1 000 mots/jour).
// Clés = texte français brut (trimé). Valeurs = [en, de, nl, it, es] —
// le format tableau force structurellement la présence des 5 langues
// (même motif que CAT_DICT / COUNTRY_DICT).
//
// DE = Allemand standard (Hochdeutsch) — couvre Allemagne, Autriche, Suisse-D.
// NL = Néerlandais belge (Vlaams) — terminologie RH belge.
//      Différences clés vs. NL-NL : Bedrijfsvoorheffing (≠ loonheffing),
//      Loonbrief (≠ loonstrook), Bediende/Kader (≠ werknemer/kaderlid).

export const STATIC_DICT = {

  // ── Navigation / vues ────────────────────────────────────────────────────
  'VUE :':                  ['VIEW :', 'ANSICHT :', 'WEERGAVE :', 'VISTA :', 'VISTA :'],
  '⊞ BUREAU':               ['⊞ DESKTOP', '⊞ BÜRO', '⊞ BUREAU', '⊞ DESKTOP', '⊞ ESCRITORIO'],
  '☰ MOBILE':               ['☰ MOBILE', '☰ MOBIL', '☰ MOBIEL', '☰ MOBILE', '☰ MÓVIL'],
  '▦ SIMULATION ANNUELLE':  ['▦ ANNUAL SIMULATION', '▦ JAHRESSIMULATION', '▦ JAARSIMULATIE', '▦ SIMULAZIONE ANNUALE', '▦ SIMULACIÓN ANUAL'],
  'Quizz Paie':             ['Payroll Quiz', 'Gehaltsquiz', 'Loonquiz', 'Quiz Paghe', 'Quiz de Nómina'],
  'La Forge':               ['The Forge', 'Die Schmiede', 'De Smidse', 'La Fucina', 'La Fragua'],
  'À propos':               ['About', 'Über uns', 'Over ons', 'Informazioni', 'Acerca de'],
  'Menu':                   ['Menu', 'Menü', 'Menu', 'Menu', 'Menú'],

  // ── Formulaire salarié ───────────────────────────────────────────────────
  'SAISIE SALARIÉ':         ['EMPLOYEE DETAILS', 'MITARBEITERDATEN', 'WERKNEMERSINVOER', 'DATI DIPENDENTE', 'DATOS DEL EMPLEADO'],
  'SAISIE':                 ['INPUT', 'EINGABE', 'INVOER', 'INSERIMENTO', 'ENTRADA'],
  'PRÉNOM':                 ['FIRST NAME', 'VORNAME', 'VOORNAAM', 'NOME', 'NOMBRE'],
  'NOM':                    ['LAST NAME', 'NACHNAME', 'ACHTERNAAM', 'COGNOME', 'APELLIDO'],
  'SALAIRE BRUT (€)':       ['GROSS SALARY (€)', 'BRUTTOGEHALT (€)', 'BRUTOLOON (€)', 'RETRIBUZIONE LORDA (€)', 'SALARIO BRUTO (€)'],
  'BRUT (€)':               ['GROSS (€)', 'BRUTTO (€)', 'BRUTO (€)', 'LORDO (€)', 'BRUTO (€)'],
  'SALAIRE BRUT (CAD)':     ['GROSS SALARY (CAD)', 'BRUTTOGEHALT (CAD)', 'BRUTOLOON (CAD)', 'RETRIBUZIONE LORDA (CAD)', 'SALARIO BRUTO (CAD)'],
  'BRUT (CAD)':             ['GROSS (CAD)', 'BRUTTO (CAD)', 'BRUTO (CAD)', 'LORDO (CAD)', 'BRUTO (CAD)'],
  'SALAIRE BRUT (CHF)':     ['GROSS SALARY (CHF)', 'BRUTTOGEHALT (CHF)', 'BRUTOLOON (CHF)', 'RETRIBUZIONE LORDA (CHF)', 'SALARIO BRUTO (CHF)'],
  'BRUT (CHF)':             ['GROSS (CHF)', 'BRUTTO (CHF)', 'BRUTO (CHF)', 'LORDO (CHF)', 'BRUTO (CHF)'],
  'STATUT':                 ['STATUS', 'STATUS', 'STATUUT', 'INQUADRAMENTO', 'CATEGORÍA'],
  'Non-cadre':              ['Non-executive', 'Nicht-leitend', 'Bediende', 'Impiegato', 'No ejecutivo'],
  'Cadre':                  ['Executive', 'Leitend', 'Kader', 'Quadro', 'Ejecutivo'],
  'DATE DE SIMULATION':     ['SIMULATION DATE', 'ABRECHNUNGSDATUM', 'BEREKENINGSDATUM', 'DATA DI SIMULAZIONE', 'FECHA DE SIMULACIÓN'],
  'ANNÉE':                  ['YEAR', 'JAHR', 'JAAR', 'ANNO', 'AÑO'],
  'PAYS/RÉGION':            ['COUNTRY/REGION', 'LAND/REGION', 'LAND/REGIO', 'PAESE/REGIONE', 'PAÍS/REGIÓN'],
  'DURÉE DE TRAVAIL':       ['WORKING TIME', 'ARBEITSZEIT', 'ARBEIDSDUUR', 'ORARIO DI LAVORO', 'JORNADA LABORAL'],
  'Heures / semaine':       ['Hours / week', 'Stunden / Woche', 'Uren / week', 'Ore / settimana', 'Horas / semana'],
  'Heures / mois':          ['Hours / month', 'Stunden / Monat', 'Uren / maand', 'Ore / mese', 'Horas / mes'],

  // ── Paramètres avancés ───────────────────────────────────────────────────
  'Assujetti à l\'impôt à la source (IS)': ['Subject to withholding tax (IS)', 'Quellensteuer pflichtig (IS)', 'Onderworpen aan bedrijfsvoorheffing (IS)', 'Soggetto a imposta alla fonte (IS)', 'Sujeto a retención en origen (IS)'],
  'Canton':                 ['Canton', 'Kanton', 'Kanton', 'Cantone', 'Cantón'],
  '— Sélectionner —':       ['— Select —', '— Auswählen —', '— Selecteren —', '— Selezionare —', '— Seleccionar —'],
  'Tarif IS':               ['Withholding tax rate', 'Quellensteuer-Tarif', 'Tarief bedrijfsvoorheffing', 'Tariffa imposta alla fonte', 'Tarifa de retención'],

  // ── Boutons ──────────────────────────────────────────────────────────────
  '[ CALCULER ]':           ['[ CALCULATE ]', '[ BERECHNEN ]', '[ BEREKENEN ]', '[ CALCOLA ]', '[ CALCULAR ]'],
  '[ SIMULER L\'ANNÉE ]':   ['[ SIMULATE THE YEAR ]', '[ JAHR SIMULIEREN ]', '[ JAAR SIMULEREN ]', '[ SIMULA L\'ANNO ]', '[ SIMULAR EL AÑO ]'],

  // ── Messages vides / chargement ──────────────────────────────────────────
  'Saisissez un salaire et cliquez sur [ CALCULER ] pour générer le bulletin': [
    'Enter a salary and click [ CALCULATE ] to generate the payslip',
    'Geben Sie ein Gehalt ein und klicken Sie auf [ BERECHNEN ], um die Gehaltsabrechnung zu erstellen',
    'Voer een loon in en klik op [ BEREKENEN ] om de loonbrief te genereren',
    'Inserisci una retribuzione e clicca su [ CALCOLA ] per generare la busta paga',
    'Introduzca un salario y haga clic en [ CALCULAR ] para generar la nómina',
  ],
  '▶ Cliquez sur une ligne de cotisation pour afficher son explication historique et la référence légale': [
    '▶ Click on a contribution line to display its historical explanation and legal reference',
    '▶ Klicken Sie auf eine Beitragszeile, um die historische Erklärung und den Rechtsgrundlage anzuzeigen',
    '▶ Klik op een bijdragelijn om de historische uitleg en de wettelijke referentie te tonen',
    '▶ Clicca su una riga di contributo per visualizzare la spiegazione storica e il riferimento normativo',
    '▶ Haga clic en una línea de cotización para ver su explicación histórica y la referencia legal',
  ],
  'En attente de saisie…':  ['Waiting for input…', 'Auf Eingabe warten…', 'Wachten op invoer…', 'In attesa di inserimento…', 'Esperando entrada…'],
  'En attente de saisie...': ['Waiting for input…', 'Auf Eingabe warten…', 'Wachten op invoer…', 'In attesa di inserimento…', 'Esperando entrada…'],
  'Saisissez un salaire brut mensuel et une année, puis cliquez sur [ SIMULER L\'ANNÉE ].': [
    'Enter a monthly gross salary and a year, then click [ SIMULATE THE YEAR ].',
    'Geben Sie einen monatlichen Bruttolohn und ein Jahr ein, dann klicken Sie auf [ JAHR SIMULIEREN ].',
    'Voer een maandelijks brutoloon en een jaar in, en klik dan op [ JAAR SIMULEREN ].',
    'Inserisci una retribuzione lorda mensile e un anno, poi clicca su [ SIMULA L\'ANNO ].',
    'Introduzca un salario bruto mensual y un año, luego haga clic en [ SIMULAR EL AÑO ].',
  ],
  'Calcul en cours…':       ['Calculating…', 'Berechnung läuft…', 'Berekening bezig…', 'Calcolo in corso…', 'Calculando…'],

  // ── Barre récap desktop ──────────────────────────────────────────────────
  '▸ SALAIRE BRUT':         ['▸ GROSS SALARY', '▸ BRUTTOGEHALT', '▸ BRUTOLOON', '▸ RETRIBUZIONE LORDA', '▸ SALARIO BRUTO'],
  '▸ RETENUES':             ['▸ DEDUCTIONS', '▸ ABZÜGE', '▸ INHOUDINGEN', '▸ TRATTENUTE', '▸ RETENCIONES'],
  '▸ NET À PAYER':          ['▸ NET PAY', '▸ NETTOLOHN', '▸ NETTOLOON', '▸ NETTO IN BUSTA', '▸ NETO A PAGAR'],
  '▸ CHARGES PAT.':         ['▸ EMPLOYER CONTRIB.', '▸ ARBEITGEB.-BEITR.', '▸ WERKGEVERSBIJDR.', '▸ ONERI DATORE', '▸ CARGAS PATRONALES'],
  '▸ SUPER BRUT':           ['▸ TOTAL COST', '▸ GESAMTKOSTEN', '▸ TOTALE LOONKOST', '▸ COSTO TOTALE', '▸ COSTE TOTAL'],
  'Cot. salariales':        ['Employee contributions', 'Arbeitnehmerabgaben', 'Werknemersbijdragen', 'Contributi dipendente', 'Cotizaciones del trabajador'],
  'Impôt à la source':      ['Withholding tax', 'Quellensteuer', 'Bedrijfsvoorheffing', 'Imposta alla fonte', 'Retención en origen'],
  'IRPEF':                  ['IRPEF', 'IRPEF', 'IRPEF', 'IRPEF', 'IRPEF'],
  'Bonus cuneo fiscale':    ['Tax wedge bonus', 'Steuerkeilbonus', 'Lastenverlagingsbonus', 'Bonus cuneo fiscale', 'Bono cuña fiscal'],
  'PAS':                    ['Withholding tax (PAS)', 'Quellensteuer', 'Bedrijfsvoorheffing', 'Ritenuta alla fonte (PAS)', 'Retención en origen (PAS)'],
  'Total retenues':         ['Total deductions', 'Gesamtabzüge', 'Totale inhoudingen', 'Totale trattenute', 'Total retenciones'],

  // ── Table des cotisations ────────────────────────────────────────────────
  'COTISATION':             ['CONTRIBUTION', 'BEITRAG', 'BIJDRAGE', 'CONTRIBUTO', 'COTIZACIÓN'],
  'BASE':                   ['BASE', 'BEMESSUNGSGRUNDLAGE', 'BEREKENINGSBASIS', 'BASE', 'BASE'],
  'TAUX SAL.':              ['EMPLOYEE RATE', 'ARBEITN.-SATZ', 'WERK.-TARIEF', 'ALIQUOTA DIP.', 'TIPO TRAB.'],
  'PART SALARIÉ':           ['EMPLOYEE SHARE', 'ARBEITNEHMERANTEIL', 'WERKNEMERSAANDEEL', 'QUOTA DIPENDENTE', 'PARTE TRABAJADOR'],
  'TAUX PAT.':              ['EMPLOYER RATE', 'ARBEITGEB.-SATZ', 'WERKGEV.-TARIEF', 'ALIQUOTA DAT.', 'TIPO EMPR.'],
  'PART PATRONALE':         ['EMPLOYER SHARE', 'ARBEITGEBERANTEIL', 'WERKGEVERSAANDEEL', 'QUOTA DATORE', 'PARTE EMPRESA'],
  'TOTAUX':                 ['TOTALS', 'SUMMEN', 'TOTALEN', 'TOTALI', 'TOTALES'],
  'TOTAL ALLÈGEMENTS PATRONAUX': ['TOTAL EMPLOYER RELIEF', 'GESAMTE ARBEITGEBERENTLASTUNGEN', 'TOTALE WERKGEVERSVERMINDERINGEN', 'TOTALE SGRAVI DATORIALI', 'TOTAL REDUCCIONES PATRONALES'],
  '── COTISATIONS ──':      ['── CONTRIBUTIONS ──', '── BEITRÄGE ──', '── BIJDRAGEN ──', '── CONTRIBUTI ──', '── COTIZACIONES ──'],
  '── ALLÈGEMENTS PATRONAUX ──': ['── EMPLOYER RELIEF ──', '── ARBEITGEBERENTLASTUNGEN ──', '── WERKGEVERSVERMINDERINGEN ──', '── SGRAVI DATORIALI ──', '── REDUCCIONES PATRONALES ──'],

  // ── Vue mobile ───────────────────────────────────────────────────────────
  'BULLETIN DE PAYE':       ['PAYSLIP', 'GEHALTSABRECHNUNG', 'LOONBRIEF', 'BUSTA PAGA', 'NÓMINA'],
  'Salaire de base brut':   ['Gross base salary', 'Bruttogehalt', 'Brutoloon', 'Retribuzione base lorda', 'Salario base bruto'],
  'TOTAL cotisations sociales': ['TOTAL social contributions', 'GESAMTE Sozialbeiträge', 'TOTALE sociale bijdragen', 'TOTALE contributi sociali', 'TOTAL cotizaciones sociales'],
  'TOTAL charges patronales':   ['TOTAL employer contributions', 'GESAMTE Arbeitgeberbeiträge', 'TOTALE werkgeversbijdragen', 'TOTALE oneri datoriali', 'TOTAL cargas patronales'],
  'NET IMPOSABLE':          ['TAXABLE NET', 'STEUERPFLICHTIGES NETTOEINKOMMEN', 'BELASTBAAR NETTOLOON', 'NETTO IMPONIBILE', 'NETO IMPONIBLE'],
  'Prélèvement à la source': ['Withholding tax (PAS)', 'Quellensteuer (PAS)', 'Bedrijfsvoorheffing', 'Ritenuta alla fonte (PAS)', 'Retención en origen (PAS)'],
  'NET À PAYER':            ['NET PAY', 'NETTOLOHN', 'NETTOLOON', 'NETTO IN BUSTA', 'NETO A PAGAR'],
  'TOTAL allègements':      ['TOTAL relief', 'GESAMTE Entlastungen', 'TOTALE verminderingen', 'TOTALE sgravi', 'TOTAL reducciones'],
  'SUPER BRUT (coût employeur)': ['TOTAL COST (employer)', 'GESAMTARBEITSKOSTEN (Arbeitgeber)', 'TOTALE LOONKOST (werkgever)', 'COSTO TOTALE (datore)', 'COSTE TOTAL (empresa)'],
  'simulation au':          ['simulation as of', 'Simulation zum', 'simulatie op', 'simulazione al', 'simulación a'],

  // ── Simulation annuelle ──────────────────────────────────────────────────
  'MOIS':        ['MONTH', 'MONAT', 'MAAND', 'MESE', 'MES'],
  'SMIC':        ['MINIMUM WAGE', 'MINDESTLOHN', 'MINIMUMLOON', 'SALARIO MINIMO', 'SALARIO MÍNIMO'],
  'BRUT':        ['GROSS', 'BRUTTO', 'BRUTO', 'LORDO', 'BRUTO'],
  'RETENUES SAL.':  ['EMPLOYEE DEDUCT.', 'ARBEITN.-ABZÜGE', 'WERK.-INHOUDINGEN', 'TRATTENUTE DIP.', 'RETENC. TRAB.'],
  'CHARGES PAT.':   ['EMPLOYER CONTRIB.', 'ARBEITGEB.-BEITR.', 'WERKGEV.-BIJDR.', 'ONERI DATORE', 'CARGAS PATR.'],
  'FILLON':      ['FILLON', 'FILLON', 'FILLON', 'FILLON', 'FILLON'],
  'Δ RÉGUL.':    ['Δ ADJUST.', 'Δ REGULIER.', 'Δ REGULARISATIE', 'Δ REGOLARIZZ.', 'Δ REGULARIZ.'],
  'NET':         ['NET', 'NETTO', 'NETTO', 'NETTO', 'NETO'],
  'COÛT EMPL.':  ['EMPLOYER COST', 'GESAMTKOSTEN', 'TOTALE KOST', 'COSTO DAT.', 'COSTE EMPR.'],
  'TOTAL':       ['TOTAL', 'GESAMT', 'TOTAAL', 'TOTALE', 'TOTAL'],
  'ÉCONOMIE FILLON (annuelle)': ['FILLON SAVINGS (annual)', 'FILLON-ERSPARNIS (jährlich)', 'FILLON-BESPARING (jaarlijks)', 'RISPARMIO FILLON (annuale)', 'AHORRO FILLON (anual)'],
  'TAUX FILLON MOYEN':      ['AVERAGE FILLON RATE', 'DURCHSCHNITTLICHER FILLON-SATZ', 'GEMIDDELD FILLON-TARIEF', 'ALIQUOTA FILLON MEDIA', 'TIPO FILLON MEDIO'],
  'COÛT EMPLOYEUR ANNUEL':  ['ANNUAL EMPLOYER COST', 'JÄHRLICHE ARBEITGEBERKOSTEN', 'JAARLIJKSE LOONKOST WERKGEVER', 'COSTO DATORIALE ANNUO', 'COSTE PATRONAL ANUAL'],
  'SIMULATION AU':          ['SIMULATION AS OF', 'SIMULATION ZUM', 'SIMULATIE OP', 'SIMULAZIONE AL', 'SIMULACIÓN A'],

  // ── Catégories de cotisations ────────────────────────────────────────────
  'Sécurité Sociale':        ['Social security', 'Sozialversicherung', 'Sociale zekerheid', 'Sicurezza sociale', 'Seguridad social'],
  'CSG/CRDS':                ['CSG/CRDS', 'CSG/CRDS', 'CSG/CRDS', 'CSG/CRDS', 'CSG/CRDS'],
  'Retraite complémentaire': ['Supplementary pension', 'Zusatzrente', 'Aanvullend pensioen', 'Pensione complementare', 'Pensión complementaria'],
  'Prévoyance':              ['Occupational benefits', 'Vorsorge', 'Aanvullende verzekering', 'Previdenza', 'Previsión'],
  'Chômage':                 ['Unemployment', 'Arbeitslosigkeit', 'Werkloosheid', 'Disoccupazione', 'Desempleo'],
  'Allègement':              ['Relief', 'Entlastung', 'Vermindering', 'Sgravio', 'Reducción'],
  '1er pilier':              ['First pillar', '1. Säule', '1e pijler', 'Primo pilastro', 'Primer pilar'],
  'Assurance chômage':       ['Unemployment insurance', 'Arbeitslosenversicherung', 'Werkloosheidsverzekering', 'Assicurazione disoccupazione', 'Seguro de desempleo'],
  'Assurance accidents':     ['Accident insurance', 'Unfallversicherung', 'Arbeidsongevallenverzekering', 'Assicurazione infortuni', 'Seguro de accidentes'],
  'Prévoyance maladie':      ['Health provision', 'Krankentaggeld', 'Ziektedagvergoeding', 'Previdenza malattia', 'Previsión de enfermedad'],
  'Prévoyance (LPP)':        ['Occupational pension (LPP)', 'Berufliche Vorsorge (BVG)', 'Beroepspensioen (BVV)', 'Previdenza professionale (LPP)', 'Previsión profesional (LPP)'],
  'Assurance pension':       ['Pension insurance', 'Rentenversicherung', 'Pensioensverzekering', 'Assicurazione pensione', 'Seguro de pensión'],
  'Assurance maladie':       ['Health insurance', 'Krankenversicherung', 'Ziekteverzekering', 'Assicurazione malattia', 'Seguro de enfermedad'],
  'Assurance dépendance':    ['Long-term care insurance', 'Pflegeversicherung', 'Zorgverzekering (afhankelijkheid)', 'Assicurazione dipendenza', 'Seguro de dependencia'],
  'Mutualité des employeurs':["Employers' mutual fund", 'Arbeitgebergegenseitigkeit', 'Werkgeversmutualiteit', 'Mutua datori di lavoro', 'Mutualidad de empleadores'],
  'Previdenza sociale':      ['Social security (IT)', 'Sozialversicherung (IT)', 'Sociale zekerheid (IT)', 'Previdenza sociale', 'Seguridad social (IT)'],
  'Disoccupazione':          ['Unemployment (IT)', 'Arbeitslosigkeit (IT)', 'Werkloosheid (IT)', 'Disoccupazione', 'Desempleo (IT)'],
  'Assicurazione infortuni': ['Accident insurance (IT)', 'Unfallversicherung (IT)', 'Arbeidsongevallenverzekering (IT)', 'Assicurazione infortuni', 'Seguro de accidentes (IT)'],
  'Fine rapporto':           ['End-of-service reserve (TFR)', 'Abfindungsrücklage', 'Eindejaarspremie (TFR)', 'Fine rapporto (TFR)', 'Reserva fin de servicio (TFR)'],
  'Bonus IRPEF':             ['IRPEF bonus', 'IRPEF-Bonus', 'IRPEF-bonus', 'Bonus IRPEF', 'Bono IRPEF'],
  'Imposta':                 ['Income tax', 'Steuer', 'Belasting', 'Imposta', 'Impuesto'],
  'Imposta regionale':       ['Regional tax', 'Regionale Steuer', 'Regionale belasting', 'Imposta regionale', 'Impuesto regional'],
  'Retraite fédérale':       ['Federal pension', 'Bundesrente', 'Federaal pensioen', 'Pensione federale', 'Pensión federal'],
  'Retraite Québec':         ['Quebec pension (QPP)', 'Québec-Rente (RRQ)', 'Québec-pensioen (RRQ)', 'Pensione Québec (RRQ)', 'Pensión de Quebec (RRQ)'],
  'Chômage fédéral':         ['Federal unemployment', 'Bundesarbeitslosenvers.', 'Federale werkloosheidsverzekering', 'Disoccupazione federale', 'Desempleo federal'],
  'Parentalité Québec':      ['Quebec parental insurance (QPIP)', 'Québec-Elternzeit (RQAP)', 'Québec-ouderschapsverlof (RQAP)', 'Assicurazione parentale Québec (RQAP)', 'Seguro parental de Quebec (RQAP)'],
  'Santé Québec':            ['Quebec health fund (FSS)', 'Québec-Gesundheitsfonds (FSS)', 'Québec-gezondheidsfonds (FSS)', 'Fondo sanitario Québec (FSS)', 'Fondo de salud de Quebec (FSS)'],
  'Impôt fédéral':           ['Federal tax', 'Bundessteuer', 'Federale belasting', 'Imposta federale', 'Impuesto federal'],
  'Impôt provincial':        ['Provincial tax', 'Provinzsteuer', 'Gewestelijke belasting', 'Imposta provinciale', 'Impuesto provincial'],
  'Autres':                  ['Other', 'Sonstige', 'Overige', 'Altro', 'Otros'],

  // ── Formules / labels techniques ─────────────────────────────────────────
  'Taux salarial':           ['Employee rate', 'Arbeitnehmersatz', 'Werknemerstarief', 'Aliquota dipendente', 'Tipo del trabajador'],
  'Taux patronal':           ['Employer rate', 'Arbeitgebersatz', 'Werkgeverstarief', 'Aliquota datoriale', 'Tipo patronal'],
  'Montant salarial':        ['Employee amount', 'Arbeitnehmerbetrag', 'Werknemersbedrag', 'Importo dipendente', 'Importe del trabajador'],
  'Montant patronal':        ['Employer amount', 'Arbeitgeberbetrag', 'Werkgeversbedrag', 'Importo datoriale', 'Importe patronal'],
  'Montant allègement':      ['Relief amount', 'Entlastungsbetrag', 'Verminderingsbedrag', 'Importo sgravio', 'Importe de la reducción'],
  'Net imposable':           ['Taxable net', 'Steuerpflichtiges Netto', 'Belastbaar nettoloon', 'Netto imponibile', 'Neto imponible'],
  'taux effectif':           ['effective rate', 'effektiver Satz', 'effectief tarief', 'aliquota effettiva', 'tipo efectivo'],
  'Tranche mensuelle':       ['Monthly bracket', 'Monatliche Steuerklasse', 'Maandelijkse belastingschijf', 'Scaglione mensile', 'Tramo mensual'],
  'Base imposée':            ['Taxed base', 'Steuerpflichtige Basis', 'Belastbare basis', 'Base imponibile', 'Base gravada'],
  'Taux':                    ['Rate', 'Satz', 'Tarief', 'Aliquota', 'Tipo'],
  'Retenue':                 ['Deduction', 'Einbehalt', 'Inhouding', 'Trattenuta', 'Retención'],

  // ── App meta ─────────────────────────────────────────────────────────────
  'simulateur pédagogique · bulletin de paye français · v0.1.8': [
    'educational simulator · French payslip · v0.1.8',
    'pädagogischer Simulator · Französische Gehaltsabrechnung · v0.1.8',
    'educatieve simulator · Franse loonbrief · v0.1.8',
    'simulatore didattico · busta paga francese · v0.1.8',
    'simulador pedagógico · nómina francesa · v0.1.8',
  ],
  'paye ton bulletin':       ['your payslip', 'Deine Gehaltsabrechnung', 'Jouw loonbrief', 'la tua busta paga', 'tu nómina'],

  // ── Accessibilité ────────────────────────────────────────────────────────
  'ACCESSIBILITÉ':           ['ACCESSIBILITY', 'BARRIEREFREIHEIT', 'TOEGANKELIJKHEID', 'ACCESSIBILITÀ', 'ACCESIBILIDAD'],
  'Mode malvoyant':          ['Low-vision mode', 'Sehbehindertenmodus', 'Modus slechtzienden', 'Modalità ipovedenti', 'Modo baja visión'],
  'Zoom ×2':                 ['Zoom ×2', 'Zoom ×2', 'Zoom ×2', 'Zoom ×2', 'Zoom ×2'],
  'Mode noir & blanc':       ['Black & white mode', 'Schwarz-Weiß-Modus', 'Zwart-witmodus', 'Modalità bianco e nero', 'Modo blanco y negro'],
  'Mode dactylo':            ['Typewriter mode', 'Schreibmaschinenmodus', 'Typemachinemodus', 'Modalità macchina da scrivere', 'Modo máquina de escribir'],
  'POLICE D\'ÉCRITURE':      ['FONT', 'SCHRIFTART', 'LETTERTYPE', 'CARATTERE', 'TIPOGRAFÍA'],

  // ── Erreurs ──────────────────────────────────────────────────────────────
  'Salaire brut invalide — saisir un montant positif.': [
    'Invalid gross salary — enter a positive amount.',
    'Ungültiger Bruttolohn — Bitte einen positiven Betrag eingeben.',
    'Ongeldig brutoloon — voer een positief bedrag in.',
    'Retribuzione lorda non valida — inserire un importo positivo.',
    'Salario bruto no válido — introduzca un importe positivo.',
  ],
  '⚠ Salaire brut invalide — saisir un montant positif.': [
    '⚠ Invalid gross salary — enter a positive amount.',
    '⚠ Ungültiger Bruttolohn — Bitte einen positiven Betrag eingeben.',
    '⚠ Ongeldig brutoloon — voer een positief bedrag in.',
    '⚠ Retribuzione lorda non valida — inserire un importo positivo.',
    '⚠ Salario bruto no válido — introduzca un importe positivo.',
  ],
  '⚠ Année invalide.':       ['⚠ Invalid year.', '⚠ Ungültiges Jahr.', '⚠ Ongeldig jaar.', '⚠ Anno non valido.', '⚠ Año no válido.'],
  'ERREUR :':                ['ERROR :', 'FEHLER :', 'FOUT :', 'ERRORE :', 'ERROR :'],
  'Traduction échouée :':    ['Translation failed :', 'Übersetzung fehlgeschlagen :', 'Vertaling mislukt :', 'Traduzione non riuscita :', 'Traducción fallida :'],

  // ── Quizz community ──────────────────────────────────────────────────────
  'Aucune entrée pour ce pays.':        ['No entries for this country.', 'Keine Einträge für dieses Land.', 'Geen gegevens voor dit land.', 'Nessuna voce per questo paese.', 'Sin entradas para este país.'],
  'chargement…':                        ['loading…', 'wird geladen…', 'laden…', 'caricamento…', 'cargando…'],
  'Pseudo requis.':                     ['Nickname required.', 'Nickname erforderlich.', 'Gebruikersnaam vereist.', 'Nickname richiesto.', 'Se requiere un alias.'],
  'Minimum 5 questions requises.':      ['At least 5 questions required.', 'Mindestens 5 Fragen erforderlich.', 'Minimaal 5 vragen vereist.', 'Minimo 5 domande richieste.', 'Se requieren al menos 5 preguntas.'],
  '✓ Score soumis !':                   ['✓ Score submitted!', '✓ Punktzahl eingereicht!', '✓ Score ingediend!', '✓ Punteggio inviato!', '✓ ¡Puntuación enviada!'],
  'SOUMETTRE':                          ['SUBMIT', 'EINREICHEN', 'INDIENEN', 'INVIA', 'ENVIAR'],
  '💡 Proposer une question':           ['💡 Suggest a question', '💡 Frage vorschlagen', '💡 Vraag voorstellen', '💡 Proponi una domanda', '💡 Proponer una pregunta'],
  '// Proposer une question':           ['// Suggest a question', '// Frage vorschlagen', '// Vraag voorstellen', '// Proponi una domanda', '// Proponer una pregunta'],
  'QUESTION *':                         ['QUESTION *', 'FRAGE *', 'VRAAG *', 'DOMANDA *', 'PREGUNTA *'],
  'RÉPONSE':                            ['ANSWER', 'ANTWORT', 'ANTWOORD', 'RISPOSTA', 'RESPUESTA'],
  'SOURCE LÉGALE':                      ['LEGAL SOURCE', 'RECHTSGRUNDLAGE', 'WETTELIJKE BRON', 'FONTE NORMATIVA', 'FUENTE LEGAL'],
  'PSEUDO':                             ['NICKNAME', 'NICKNAME', 'GEBRUIKERSNAAM', 'NICKNAME', 'ALIAS'],
  'ENVOYER':                            ['SEND', 'SENDEN', 'VERZENDEN', 'INVIA', 'ENVIAR'],
  '✓ Merci pour votre contribution !':  ['✓ Thank you for your contribution!', '✓ Danke für Ihren Beitrag!', '✓ Bedankt voor uw bijdrage!', '✓ Grazie per il tuo contributo!', '✓ ¡Gracias por su contribución!'],
  'Question requise.':                  ['Question required.', 'Frage erforderlich.', 'Vraag vereist.', 'Domanda richiesta.', 'Se requiere una pregunta.'],
  'BONNE RÉPONSE':                      ['CORRECT ANSWER', 'RICHTIGE ANTWORT', 'JUIST ANTWOORD', 'RISPOSTA CORRETTA', 'RESPUESTA CORRECTA'],
  'RÉPONSES ALTERNATIVES':              ['ALTERNATIVE ANSWERS', 'ALTERNATIVE ANTWORTEN', 'ALTERNATIEVE ANTWOORDEN', 'RISPOSTE ALTERNATIVE', 'RESPUESTAS ALTERNATIVAS'],
  'MAUVAISES RÉPONSES / DISTRACTEURS':  ['WRONG ANSWERS / DISTRACTORS', 'FALSCHE ANTWORTEN / DISTRAKTOREN', 'FOUT ANTWOORDEN / DISTRACTOREN', 'RISPOSTE ERRATE / DISTRATTORI', 'RESPUESTAS INCORRECTAS / DISTRACTORES'],
  '🗳 Voter pour une question':          ['🗳 Vote for a question', '🗳 Für eine Frage abstimmen', '🗳 Stemmen op een vraag', '🗳 Vota una domanda', '🗳 Votar por una pregunta'],
  '// Questions proposées par la communauté': ['// Community-suggested questions', '// Von der Community vorgeschlagene Fragen', '// Door de community voorgestelde vragen', '// Domande proposte dalla community', '// Preguntas propuestas por la comunidad'],
  'Aucune suggestion pour ce pays.':    ['No suggestions for this country.', 'Keine Vorschläge für dieses Land.', 'Geen suggesties voor dit land.', 'Nessun suggerimento per questo paese.', 'Sin sugerencias para este país.'],
  '✓ voté':                             ['✓ voted', '✓ abgestimmt', '✓ gestemd', '✓ votato', '✓ votado'],
  '👍 voter':                           ['👍 vote', '👍 abstimmen', '👍 stemmen', '👍 vota', '👍 votar'],
};

// ── Index de langue pour les tables [en, de, nl, it, es] ──────────────────────
const _LANG_IDX = { en: 0, de: 1, nl: 2, it: 3, es: 4 };

/// Traduit un texte statique de l'UI ; renvoie undefined si absent du
/// dictionnaire (le repli MyMemory/français est géré par l'appelant).
export function trStatic(raw, lang) {
  const idx = _LANG_IDX[lang];
  if (idx === undefined) return undefined;      // fr ou langue inconnue
  const row = STATIC_DICT[raw];
  return row ? row[idx] : undefined;
}

// ── Catégories de cotisation (champ `categorie`) ─────────────────────────────
// Clé = valeur brute émise par le backend (français canonique + variantes
// historiques en langue locale). Valeurs = [en, de, nl, it, es].
// La valeur brute reste la clé logique côté front (filtres/CAT_CLASS) ; seul
// l'AFFICHAGE est traduit via trCat().
export const CAT_DICT = {
  'Sécurité sociale':        ['Social security', 'Sozialversicherung', 'Sociale zekerheid', 'Sicurezza sociale', 'Seguridad social'],
  'Sécurité Sociale':        ['Social security', 'Sozialversicherung', 'Sociale zekerheid', 'Sicurezza sociale', 'Seguridad social'],
  'Previdenza sociale':      ['Social security', 'Sozialversicherung', 'Sociale zekerheid', 'Previdenza sociale', 'Seguridad social'],
  'Assurance maladie':       ['Health insurance', 'Krankenversicherung', 'Ziektekostenverzekering', 'Assicurazione malattia', 'Seguro de enfermedad'],
  'Assurance pension':       ['Pension insurance', 'Rentenversicherung', 'Pensioenverzekering', 'Assicurazione pensione', 'Seguro de pensión'],
  'Assurance chômage':       ['Unemployment insurance', 'Arbeitslosenversicherung', 'Werkloosheidsverzekering', 'Assicurazione disoccupazione', 'Seguro de desempleo'],
  'Chômage':                 ['Unemployment', 'Arbeitslosigkeit', 'Werkloosheid', 'Disoccupazione', 'Desempleo'],
  'Disoccupazione':          ['Unemployment', 'Arbeitslosigkeit', 'Werkloosheid', 'Disoccupazione', 'Desempleo'],
  'Assurance dépendance':    ['Long-term care insurance', 'Pflegeversicherung', 'Zorgverzekering (langdurig)', 'Assicurazione dipendenza', 'Seguro de dependencia'],
  'Assurance accidents':     ['Accident insurance', 'Unfallversicherung', 'Ongevallenverzekering', 'Assicurazione infortuni', 'Seguro de accidentes'],
  'Assicurazione infortuni': ['Accident insurance', 'Unfallversicherung', 'Ongevallenverzekering', 'Assicurazione infortuni', 'Seguro de accidentes'],
  'Accidents du travail':    ['Work accidents', 'Arbeitsunfälle', 'Arbeidsongevallen', 'Infortuni sul lavoro', 'Accidentes laborales'],
  'Retraite':                ['Pension', 'Rente', 'Pensioen', 'Pensione', 'Jubilación'],
  'Retraite complémentaire': ['Supplementary pension', 'Zusatzrente', 'Aanvullend pensioen', 'Pensione complementare', 'Pensión complementaria'],
  'Retraite (CNRACL)':       ['Pension (CNRACL)', 'Rente (CNRACL)', 'Pensioen (CNRACL)', 'Pensione (CNRACL)', 'Jubilación (CNRACL)'],
  'Prévoyance':              ['Occupational benefits', 'Vorsorge', 'Voorzorg', 'Previdenza', 'Previsión'],
  'Prévoyance (LPP)':        ['Occupational pension (LPP)', 'Berufliche Vorsorge (BVG)', 'Beroepsvoorzorg (LPP)', 'Previdenza professionale (LPP)', 'Previsión profesional (LPP)'],
  'Prévoyance maladie':      ['Health provision', 'Krankenvorsorge', 'Ziektevoorzorg', 'Previdenza malattia', 'Previsión de enfermedad'],
  '1er pilier':              ['First pillar', 'Erste Säule', 'Eerste pijler', 'Primo pilastro', 'Primer pilar'],
  'CSG/CRDS':                ['CSG/CRDS', 'CSG/CRDS', 'CSG/CRDS', 'CSG/CRDS', 'CSG/CRDS'],
  'Impôt sur le revenu':     ['Income tax', 'Einkommensteuer', 'Inkomstenbelasting', 'Imposta sul reddito', 'Impuesto sobre la renta'],
  'Imposta':                 ['Income tax', 'Einkommensteuer', 'Inkomstenbelasting', 'Imposta sul reddito', 'Impuesto sobre la renta'],
  'Imposta regionale':       ['Regional tax', 'Regionalsteuer', 'Regionale belasting', 'Imposta regionale', 'Impuesto regional'],
  'Impôt à la source':       ['Withholding tax', 'Quellensteuer', 'Bronbelasting', 'Imposta alla fonte', 'Retención en origen'],
  'Impôt fédéral':           ['Federal tax', 'Bundessteuer', 'Federale belasting', 'Imposta federale', 'Impuesto federal'],
  'Impôt provincial':        ['Provincial tax', 'Provinzsteuer', 'Provinciale belasting', 'Imposta provinciale', 'Impuesto provincial'],
  'Retraite fédérale':       ['Federal pension', 'Bundesrente', 'Federaal pensioen', 'Pensione federale', 'Pensión federal'],
  'Chômage fédéral':         ['Federal unemployment', 'Bundesarbeitslosigkeit', 'Federale werkloosheid', 'Disoccupazione federale', 'Desempleo federal'],
  'Retraite Québec':         ['Quebec pension', 'Rente Québec', 'Pensioen Québec', 'Pensione Québec', 'Pensión de Quebec'],
  'Taxe locale':             ['Local tax', 'Kommunalsteuer', 'Lokale belasting', 'Imposta locale', 'Impuesto local'],
  'Formation professionnelle':['Vocational training', 'Berufsbildung', 'Beroepsopleiding', 'Formazione professionale', 'Formación profesional'],
  'Garantie salariale':      ['Wage guarantee', 'Lohngarantie', 'Loongarantie', 'Garanzia salariale', 'Garantía salarial'],
  'Garantie emploi':         ['Employment guarantee', 'Beschäftigungsgarantie', 'Werkgelegenheidsgarantie', 'Garanzia occupazione', 'Garantía de empleo'],
  "Aide à l'emploi":         ['Employment support', 'Beschäftigungshilfe', 'Werkgelegenheidssteun', "Sostegno all'occupazione", 'Ayuda al empleo'],
  'Allègement':              ['Relief', 'Entlastung', 'Verlichting', 'Sgravio', 'Reducción'],
  'Allegement':              ['Relief', 'Entlastung', 'Verlichting', 'Sgravio', 'Reducción'],
  'Réduction patronale':     ['Employer relief', 'Arbeitgeberentlastung', 'Werkgeversvermindering', 'Sgravio datore di lavoro', 'Reducción patronal'],
  'Réduction salariale':     ['Employee relief', 'Arbeitnehmerentlastung', 'Werknemersvermindering', 'Sgravio dipendente', 'Reducción salarial'],
  'Mutualité des employeurs':["Employers' mutual fund", 'Arbeitgeber-Ausgleichskasse', 'Werkgeversfonds', 'Mutua datori di lavoro', 'Mutualidad de empleadores'],
  'Cotisations patronales':  ['Employer contributions', 'Arbeitgeberbeiträge', 'Werkgeversbijdragen', 'Contributi datoriali', 'Cotizaciones patronales'],
  'Fine rapporto':           ['End-of-service reserve', 'Abfindungsrücklage', 'Eindedienstreserve', 'Fine rapporto', 'Reserva fin de servicio'],
  'Réserve retraite':        ['Pension reserve', 'Rentenrücklage', 'Pensioenreserve', 'Riserva pensione', 'Reserva de pensión'],
  'Bonus IRPEF':             ['IRPEF bonus', 'IRPEF-Bonus', 'IRPEF-bonus', 'Bonus IRPEF', 'Bono IRPEF'],
  'Parentalité Québec':      ['Quebec parental insurance', 'Elternversicherung Québec', 'Ouderschapsverzekering Québec', 'Assicurazione parentale Québec', 'Seguro parental de Quebec'],
  'Santé Québec':            ['Quebec health', 'Gesundheit Québec', 'Gezondheid Québec', 'Sanità Québec', 'Salud Quebec'],
  'Heures supplémentaires':  ['Overtime', 'Überstunden', 'Overuren', 'Straordinari', 'Horas extra'],
  'Autres':                  ['Other', 'Sonstige', 'Overige', 'Altro', 'Otros'],
  'Information':             ['Information', 'Information', 'Informatie', 'Informazione', 'Información'],
};

/// Traduit l'AFFICHAGE d'une catégorie ; renvoie la valeur brute en repli (fr).
export function trCat(raw, lang) {
  const idx = _LANG_IDX[lang];
  if (idx === undefined) return raw;            // fr ou langue inconnue
  const row = CAT_DICT[raw];
  return row ? row[idx] : raw;
}

// ── Noms de pays et sous-régions (sélecteur + libellés) ──────────────────────
// Clé = nom français. Valeurs = [en, de, nl, it, es]. Pré-injectées dans le
// cache de traduction (translateApp) pour éviter MyMemory sur ces libellés.
export const COUNTRY_DICT = {
  'France': ['France', 'Frankreich', 'Frankrijk', 'Francia', 'Francia'],
  'Fonction publique': ['Civil service', 'Öffentlicher Dienst', 'Overheid', 'Pubblico impiego', 'Función pública'],
  'Entreprise adaptée (AAP)': ['Adapted enterprise (AAP)', 'Inklusionsbetrieb (AAP)', 'Aangepast bedrijf (AAP)', 'Impresa adattata (AAP)', 'Empresa adaptada (AAP)'],
  'Alsace-Moselle': ['Alsace-Moselle', 'Elsass-Mosel', 'Alsace-Moselle', 'Alsazia-Mosella', 'Alsacia-Mosela'],
  'Allemagne': ['Germany', 'Deutschland', 'Duitsland', 'Germania', 'Alemania'],
  'Andorre': ['Andorra', 'Andorra', 'Andorra', 'Andorra', 'Andorra'],
  'Angleterre': ['England', 'England', 'Engeland', 'Inghilterra', 'Inglaterra'],
  'Autriche': ['Austria', 'Österreich', 'Oostenrijk', 'Austria', 'Austria'],
  'Belgique': ['Belgium', 'Belgien', 'België', 'Belgio', 'Bélgica'],
  'Flandres': ['Flanders', 'Flandern', 'Vlaanderen', 'Fiandre', 'Flandes'],
  'Wallonie': ['Wallonia', 'Wallonien', 'Wallonië', 'Vallonia', 'Valonia'],
  'Bruxelles': ['Brussels', 'Brüssel', 'Brussel', 'Bruxelles', 'Bruselas'],
  'Bulgarie': ['Bulgaria', 'Bulgarien', 'Bulgarije', 'Bulgaria', 'Bulgaria'],
  'Chypre': ['Cyprus', 'Zypern', 'Cyprus', 'Cipro', 'Chipre'],
  'Croatie': ['Croatia', 'Kroatien', 'Kroatië', 'Croazia', 'Croacia'],
  'Danemark': ['Denmark', 'Dänemark', 'Denemarken', 'Danimarca', 'Dinamarca'],
  'Espagne': ['Spain', 'Spanien', 'Spanje', 'Spagna', 'España'],
  'Estonie': ['Estonia', 'Estland', 'Estland', 'Estonia', 'Estonia'],
  'Finlande': ['Finland', 'Finnland', 'Finland', 'Finlandia', 'Finlandia'],
  'Grèce': ['Greece', 'Griechenland', 'Griekenland', 'Grecia', 'Grecia'],
  'Hongrie': ['Hungary', 'Ungarn', 'Hongarije', 'Ungheria', 'Hungría'],
  'Irlande': ['Ireland', 'Irland', 'Ierland', 'Irlanda', 'Irlanda'],
  'Italie': ['Italy', 'Italien', 'Italië', 'Italia', 'Italia'],
  'Lettonie': ['Latvia', 'Lettland', 'Letland', 'Lettonia', 'Letonia'],
  'Lituanie': ['Lithuania', 'Litauen', 'Litouwen', 'Lituania', 'Lituania'],
  'Luxembourg': ['Luxembourg', 'Luxemburg', 'Luxemburg', 'Lussemburgo', 'Luxemburgo'],
  'Malte': ['Malta', 'Malta', 'Malta', 'Malta', 'Malta'],
  'Monaco': ['Monaco', 'Monaco', 'Monaco', 'Monaco', 'Mónaco'],
  'Pays-Bas': ['Netherlands', 'Niederlande', 'Nederland', 'Paesi Bassi', 'Países Bajos'],
  'Pologne': ['Poland', 'Polen', 'Polen', 'Polonia', 'Polonia'],
  'Portugal': ['Portugal', 'Portugal', 'Portugal', 'Portogallo', 'Portugal'],
  'Roumanie': ['Romania', 'Rumänien', 'Roemenië', 'Romania', 'Rumanía'],
  'Slovaquie': ['Slovakia', 'Slowakei', 'Slowakije', 'Slovacchia', 'Eslovaquia'],
  'Slovénie': ['Slovenia', 'Slowenien', 'Slovenië', 'Slovenia', 'Eslovenia'],
  'Suède': ['Sweden', 'Schweden', 'Zweden', 'Svezia', 'Suecia'],
  'Suisse': ['Switzerland', 'Schweiz', 'Zwitserland', 'Svizzera', 'Suiza'],
  'Tchéquie': ['Czechia', 'Tschechien', 'Tsjechië', 'Cechia', 'Chequia'],
  'Canada': ['Canada', 'Kanada', 'Canada', 'Canada', 'Canadá'],
  'Québec': ['Quebec', 'Québec', 'Quebec', 'Québec', 'Quebec'],
  'Japon': ['Japan', 'Japan', 'Japan', 'Giappone', 'Japón'],
  'Chine': ['China', 'China', 'China', 'Cina', 'China'],
  'Corée du Sud': ['South Korea', 'Südkorea', 'Zuid-Korea', 'Corea del Sud', 'Corea del Sur'],
  'Australie': ['Australia', 'Australien', 'Australië', 'Australia', 'Australia'],
  'Nouvelle-Zélande': ['New Zealand', 'Neuseeland', 'Nieuw-Zeeland', 'Nuova Zelanda', 'Nueva Zelanda'],
};
