// ── Dictionnaires de traduction statiques ─────────────────────────────────────
//
// Utilisés EN PRIORITÉ avant l'API MyMemory (limite 1 000 mots/jour).
// Clés = texte français brut (trimé). Valeurs = traduction.
//
// DE = Allemand standard (Hochdeutsch) — couvre Allemagne, Autriche, Suisse-D.
// NL = Néerlandais belge (Vlaams) — terminologie RH belge.
//      Différences clés vs. NL-NL : Bedrijfsvoorheffing (≠ loonheffing),
//      Loonbrief (≠ loonstrook), Bediende/Kader (≠ werknemer/kaderlid).

export const STATIC_DICT = {

  // ── Allemand ───────────────────────────────────────────────────────────────
  de: {
    // Navigation / vues
    'VUE :':                  'ANSICHT :',
    '⊞ BUREAU':               '⊞ BÜRO',
    '☰ MOBILE':               '☰ MOBIL',
    '▦ SIMULATION ANNUELLE':  '▦ JAHRESSIMULATION',
    'Quizz Paie':             'Gehaltsquiz',
    'La Forge':               'Die Schmiede',
    'À propos':               'Über uns',
    'Menu':                   'Menü',

    // Formulaire salarié
    'SAISIE SALARIÉ':         'MITARBEITERDATEN',
    'SAISIE':                 'EINGABE',
    'PRÉNOM':                 'VORNAME',
    'NOM':                    'NACHNAME',
    'SALAIRE BRUT (€)':       'BRUTTOGEHALT (€)',
    'BRUT (€)':               'BRUTTO (€)',
    'SALAIRE BRUT (CAD)':     'BRUTTOGEHALT (CAD)',
    'BRUT (CAD)':             'BRUTTO (CAD)',
    'SALAIRE BRUT (CHF)':     'BRUTTOGEHALT (CHF)',
    'BRUT (CHF)':             'BRUTTO (CHF)',
    'STATUT':                 'STATUS',
    'Non-cadre':              'Nicht-leitend',
    'Cadre':                  'Leitend',
    'DATE DE SIMULATION':     'ABRECHNUNGSDATUM',
    'ANNÉE':                  'JAHR',
    'PAYS/RÉGION':            'LAND/REGION',
    'DURÉE DE TRAVAIL':       'ARBEITSZEIT',
    'Heures / semaine':       'Stunden / Woche',
    'Heures / mois':          'Stunden / Monat',

    // Paramètres avancés
    'Assujetti à l\'impôt à la source (IS)': 'Quellensteuer pflichtig (IS)',
    'Canton':                 'Kanton',
    '— Sélectionner —':       '— Auswählen —',
    'Tarif IS':               'Quellensteuer-Tarif',

    // Boutons
    '[ CALCULER ]':           '[ BERECHNEN ]',
    '[ SIMULER L\'ANNÉE ]':   '[ JAHR SIMULIEREN ]',

    // Messages vides / chargement
    'Saisissez un salaire et cliquez sur [ CALCULER ] pour générer le bulletin':
      'Geben Sie ein Gehalt ein und klicken Sie auf [ BERECHNEN ], um die Gehaltsabrechnung zu erstellen',
    '▶ Cliquez sur une ligne de cotisation pour afficher son explication historique et la référence légale':
      '▶ Klicken Sie auf eine Beitragszeile, um die historische Erklärung und den Rechtsgrundlage anzuzeigen',
    'En attente de saisie…':  'Auf Eingabe warten…',
    'En attente de saisie...': 'Auf Eingabe warten…',
    'Saisissez un salaire brut mensuel et une année, puis cliquez sur [ SIMULER L\'ANNÉE ].':
      'Geben Sie einen monatlichen Bruttolohn und ein Jahr ein, dann klicken Sie auf [ JAHR SIMULIEREN ].',
    'Calcul en cours…':       'Berechnung läuft…',

    // Barre récap desktop
    '▸ SALAIRE BRUT':         '▸ BRUTTOGEHALT',
    '▸ RETENUES':             '▸ ABZÜGE',
    '▸ NET À PAYER':          '▸ NETTOLOHN',
    '▸ CHARGES PAT.':         '▸ ARBEITGEB.-BEITR.',
    '▸ SUPER BRUT':           '▸ GESAMTKOSTEN',
    'Cot. salariales':        'Arbeitnehmerabgaben',
    'Impôt à la source':      'Quellensteuer',
    'IRPEF':                  'IRPEF',
    'Bonus cuneo fiscale':    'Steuerkeilbonus',
    'PAS':                    'Quellensteuer',
    'Total retenues':         'Gesamtabzüge',

    // Table des cotisations
    'COTISATION':             'BEITRAG',
    'BASE':                   'BEMESSUNGSGRUNDLAGE',
    'TAUX SAL.':              'ARBEITN.-SATZ',
    'PART SALARIÉ':           'ARBEITNEHMERANTEIL',
    'TAUX PAT.':              'ARBEITGEB.-SATZ',
    'PART PATRONALE':         'ARBEITGEBERANTEIL',
    'TOTAUX':                 'SUMMEN',
    'TOTAL ALLÈGEMENTS PATRONAUX': 'GESAMTE ARBEITGEBERENTLASTUNGEN',
    '── COTISATIONS ──':      '── BEITRÄGE ──',
    '── ALLÈGEMENTS PATRONAUX ──': '── ARBEITGEBERENTLASTUNGEN ──',

    // Vue mobile
    'BULLETIN DE PAYE':       'GEHALTSABRECHNUNG',
    'Salaire de base brut':   'Bruttogehalt',
    'TOTAL cotisations sociales': 'GESAMTE Sozialbeiträge',
    'TOTAL charges patronales':   'GESAMTE Arbeitgeberbeiträge',
    'NET IMPOSABLE':          'STEUERPFLICHTIGES NETTOEINKOMMEN',
    'Prélèvement à la source': 'Quellensteuer (PAS)',
    'NET À PAYER':            'NETTOLOHN',
    'TOTAL allègements':      'GESAMTE Entlastungen',
    'SUPER BRUT (coût employeur)': 'GESAMTARBEITSKOSTEN (Arbeitgeber)',
    'simulation au':          'Simulation zum',

    // Simulation annuelle
    'MOIS':        'MONAT',
    'SMIC':        'MINDESTLOHN',
    'BRUT':        'BRUTTO',
    'RETENUES SAL.':  'ARBEITN.-ABZÜGE',
    'CHARGES PAT.':   'ARBEITGEB.-BEITR.',
    'FILLON':      'FILLON',
    'Δ RÉGUL.':    'Δ REGULIER.',
    'NET':         'NETTO',
    'COÛT EMPL.':  'GESAMTKOSTEN',
    'TOTAL':       'GESAMT',
    'ÉCONOMIE FILLON (annuelle)': 'FILLON-ERSPARNIS (jährlich)',
    'TAUX FILLON MOYEN':      'DURCHSCHNITTLICHER FILLON-SATZ',
    'COÛT EMPLOYEUR ANNUEL':  'JÄHRLICHE ARBEITGEBERKOSTEN',
    'SIMULATION AU':          'SIMULATION ZUM',

    // Catégories de cotisations
    'Sécurité Sociale':        'Sozialversicherung',
    'CSG/CRDS':                'CSG/CRDS',
    'Retraite complémentaire': 'Zusatzrente',
    'Prévoyance':              'Vorsorge',
    'Chômage':                 'Arbeitslosigkeit',
    'Allègement':              'Entlastung',
    '1er pilier':              '1. Säule',
    'Assurance chômage':       'Arbeitslosenversicherung',
    'Assurance accidents':     'Unfallversicherung',
    'Prévoyance maladie':      'Krankentaggeld',
    'Prévoyance (LPP)':        'Berufliche Vorsorge (BVG)',
    'Assurance pension':       'Rentenversicherung',
    'Assurance maladie':       'Krankenversicherung',
    'Assurance dépendance':    'Pflegeversicherung',
    'Mutualité des employeurs':'Arbeitgebergegenseitigkeit',
    'Previdenza sociale':      'Sozialversicherung (IT)',
    'Disoccupazione':          'Arbeitslosigkeit (IT)',
    'Assicurazione infortuni': 'Unfallversicherung (IT)',
    'Fine rapporto':           'Abfindungsrücklage',
    'Bonus IRPEF':             'IRPEF-Bonus',
    'Imposta':                 'Steuer',
    'Imposta regionale':       'Regionale Steuer',
    'Retraite fédérale':       'Bundesrente',
    'Retraite Québec':         'Québec-Rente (RRQ)',
    'Chômage fédéral':         'Bundesarbeitslosenvers.',
    'Parentalité Québec':      'Québec-Elternzeit (RQAP)',
    'Santé Québec':            'Québec-Gesundheitsfonds (FSS)',
    'Impôt fédéral':           'Bundessteuer',
    'Impôt provincial':        'Provinzsteuer',
    'Autres':                  'Sonstige',

    // Formules / labels techniques
    'Taux salarial':           'Arbeitnehmersatz',
    'Taux patronal':           'Arbeitgebersatz',
    'Montant salarial':        'Arbeitnehmerbetrag',
    'Montant patronal':        'Arbeitgeberbetrag',
    'Montant allègement':      'Entlastungsbetrag',
    'Net imposable':           'Steuerpflichtiges Netto',
    'taux effectif':           'effektiver Satz',
    'Tranche mensuelle':       'Monatliche Steuerklasse',
    'Base imposée':            'Steuerpflichtige Basis',
    'Taux':                    'Satz',
    'Retenue':                 'Einbehalt',

    // App meta
    'simulateur pédagogique · bulletin de paye français · v0.1.8':
      'pädagogischer Simulator · Französische Gehaltsabrechnung · v0.1.8',
    'paye ton bulletin':       'Deine Gehaltsabrechnung',

    // Accessibilité
    'ACCESSIBILITÉ':           'BARRIEREFREIHEIT',
    'Mode malvoyant':          'Sehbehindertenmodus',
    'Zoom ×2':                 'Zoom ×2',
    'Mode noir & blanc':       'Schwarz-Weiß-Modus',
    'Mode dactylo':            'Schreibmaschinenmodus',
    'POLICE D\'ÉCRITURE':      'SCHRIFTART',

    // Erreurs
    'Salaire brut invalide — saisir un montant positif.':
      'Ungültiger Bruttolohn — Bitte einen positiven Betrag eingeben.',
    '⚠ Salaire brut invalide — saisir un montant positif.':
      '⚠ Ungültiger Bruttolohn — Bitte einen positiven Betrag eingeben.',
    '⚠ Année invalide.':       '⚠ Ungültiges Jahr.',
    'ERREUR :':                'FEHLER :',
    'Traduction échouée :':    'Übersetzung fehlgeschlagen :',

    // Quizz community
    'Aucune entrée pour ce pays.':        'Keine Einträge für dieses Land.',
    'chargement…':                        'wird geladen…',
    'Pseudo requis.':                     'Nickname erforderlich.',
    'Minimum 5 questions requises.':      'Mindestens 5 Fragen erforderlich.',
    '✓ Score soumis !':                   '✓ Punktzahl eingereicht!',
    'SOUMETTRE':                          'EINREICHEN',
    '💡 Proposer une question':           '💡 Frage vorschlagen',
    '// Proposer une question':           '// Frage vorschlagen',
    'QUESTION *':                         'FRAGE *',
    'RÉPONSE':                            'ANTWORT',
    'SOURCE LÉGALE':                      'RECHTSGRUNDLAGE',
    'PSEUDO':                             'NICKNAME',
    'ENVOYER':                            'SENDEN',
    '✓ Merci pour votre contribution !':  '✓ Danke für Ihren Beitrag!',
    'Question requise.':                  'Frage erforderlich.',
    'BONNE RÉPONSE':                      'RICHTIGE ANTWORT',
    'RÉPONSES ALTERNATIVES':              'ALTERNATIVE ANTWORTEN',
    'MAUVAISES RÉPONSES / DISTRACTEURS':  'FALSCHE ANTWORTEN / DISTRAKTOREN',
    '🗳 Voter pour une question':          '🗳 Für eine Frage abstimmen',
    '// Questions proposées par la communauté': '// Von der Community vorgeschlagene Fragen',
    'Aucune suggestion pour ce pays.':    'Keine Vorschläge für dieses Land.',
    '✓ voté':                             '✓ abgestimmt',
    '👍 voter':                           '👍 abstimmen',
  },

  // ── Néerlandais belge (Vlaams) ─────────────────────────────────────────────
  nl: {
    // Navigation / vues
    'VUE :':                  'WEERGAVE :',
    '⊞ BUREAU':               '⊞ BUREAU',
    '☰ MOBILE':               '☰ MOBIEL',
    '▦ SIMULATION ANNUELLE':  '▦ JAARSIMULATIE',
    'Quizz Paie':             'Loonquiz',
    'La Forge':               'De Smidse',
    'À propos':               'Over ons',
    'Menu':                   'Menu',

    // Formulaire salarié
    'SAISIE SALARIÉ':         'WERKNEMERSINVOER',
    'SAISIE':                 'INVOER',
    'PRÉNOM':                 'VOORNAAM',
    'NOM':                    'ACHTERNAAM',
    'SALAIRE BRUT (€)':       'BRUTOLOON (€)',
    'BRUT (€)':               'BRUTO (€)',
    'SALAIRE BRUT (CAD)':     'BRUTOLOON (CAD)',
    'BRUT (CAD)':             'BRUTO (CAD)',
    'SALAIRE BRUT (CHF)':     'BRUTOLOON (CHF)',
    'BRUT (CHF)':             'BRUTO (CHF)',
    'STATUT':                 'STATUUT',
    'Non-cadre':              'Bediende',
    'Cadre':                  'Kader',
    'DATE DE SIMULATION':     'BEREKENINGSDATUM',
    'ANNÉE':                  'JAAR',
    'PAYS/RÉGION':            'LAND/REGIO',
    'DURÉE DE TRAVAIL':       'ARBEIDSDUUR',
    'Heures / semaine':       'Uren / week',
    'Heures / mois':          'Uren / maand',

    // Paramètres avancés
    'Assujetti à l\'impôt à la source (IS)': 'Onderworpen aan bedrijfsvoorheffing (IS)',
    'Canton':                 'Kanton',
    '— Sélectionner —':       '— Selecteren —',
    'Tarif IS':               'Tarief bedrijfsvoorheffing',

    // Boutons
    '[ CALCULER ]':           '[ BEREKENEN ]',
    '[ SIMULER L\'ANNÉE ]':   '[ JAAR SIMULEREN ]',

    // Messages vides / chargement
    'Saisissez un salaire et cliquez sur [ CALCULER ] pour générer le bulletin':
      'Voer een loon in en klik op [ BEREKENEN ] om de loonbrief te genereren',
    '▶ Cliquez sur une ligne de cotisation pour afficher son explication historique et la référence légale':
      '▶ Klik op een bijdragelijn om de historische uitleg en de wettelijke referentie te tonen',
    'En attente de saisie…':  'Wachten op invoer…',
    'En attente de saisie...': 'Wachten op invoer…',
    'Saisissez un salaire brut mensuel et une année, puis cliquez sur [ SIMULER L\'ANNÉE ].':
      'Voer een maandelijks brutoloon en een jaar in, en klik dan op [ JAAR SIMULEREN ].',
    'Calcul en cours…':       'Berekening bezig…',

    // Barre récap desktop
    '▸ SALAIRE BRUT':         '▸ BRUTOLOON',
    '▸ RETENUES':             '▸ INHOUDINGEN',
    '▸ NET À PAYER':          '▸ NETTOLOON',
    '▸ CHARGES PAT.':         '▸ WERKGEVERSBIJDR.',
    '▸ SUPER BRUT':           '▸ TOTALE LOONKOST',
    'Cot. salariales':        'Werknemersbijdragen',
    'Impôt à la source':      'Bedrijfsvoorheffing',
    'IRPEF':                  'IRPEF',
    'Bonus cuneo fiscale':    'Lastenverlagingsbonus',
    'PAS':                    'Bedrijfsvoorheffing',
    'Total retenues':         'Totale inhoudingen',

    // Table des cotisations
    'COTISATION':             'BIJDRAGE',
    'BASE':                   'BEREKENINGSBASIS',
    'TAUX SAL.':              'WERK.-TARIEF',
    'PART SALARIÉ':           'WERKNEMERSAANDEEL',
    'TAUX PAT.':              'WERKGEV.-TARIEF',
    'PART PATRONALE':         'WERKGEVERSAANDEEL',
    'TOTAUX':                 'TOTALEN',
    'TOTAL ALLÈGEMENTS PATRONAUX': 'TOTALE WERKGEVERSVERMINDERINGEN',
    '── COTISATIONS ──':      '── BIJDRAGEN ──',
    '── ALLÈGEMENTS PATRONAUX ──': '── WERKGEVERSVERMINDERINGEN ──',

    // Vue mobile
    'BULLETIN DE PAYE':       'LOONBRIEF',
    'Salaire de base brut':   'Brutoloon',
    'TOTAL cotisations sociales': 'TOTALE sociale bijdragen',
    'TOTAL charges patronales':   'TOTALE werkgeversbijdragen',
    'NET IMPOSABLE':          'BELASTBAAR NETTOLOON',
    'Prélèvement à la source': 'Bedrijfsvoorheffing',
    'NET À PAYER':            'NETTOLOON',
    'TOTAL allègements':      'TOTALE verminderingen',
    'SUPER BRUT (coût employeur)': 'TOTALE LOONKOST (werkgever)',
    'simulation au':          'simulatie op',

    // Simulation annuelle
    'MOIS':        'MAAND',
    'SMIC':        'MINIMUMLOON',
    'BRUT':        'BRUTO',
    'RETENUES SAL.':  'WERK.-INHOUDINGEN',
    'CHARGES PAT.':   'WERKGEV.-BIJDR.',
    'FILLON':      'FILLON',
    'Δ RÉGUL.':    'Δ REGULARISATIE',
    'NET':         'NETTO',
    'COÛT EMPL.':  'TOTALE KOST',
    'TOTAL':       'TOTAAL',
    'ÉCONOMIE FILLON (annuelle)': 'FILLON-BESPARING (jaarlijks)',
    'TAUX FILLON MOYEN':      'GEMIDDELD FILLON-TARIEF',
    'COÛT EMPLOYEUR ANNUEL':  'JAARLIJKSE LOONKOST WERKGEVER',
    'SIMULATION AU':          'SIMULATIE OP',

    // Catégories de cotisations
    'Sécurité Sociale':        'Sociale zekerheid',
    'CSG/CRDS': 'CSG/CRDS',
    'Retraite complémentaire': 'Aanvullend pensioen',
    'Prévoyance':              'Aanvullende verzekering',
    'Chômage':                 'Werkloosheid',
    'Allègement':              'Vermindering',
    '1er pilier':              '1e pijler',
    'Assurance chômage':       'Werkloosheidsverzekering',
    'Assurance accidents':     'Arbeidsongevallenverzekering',
    'Prévoyance maladie':      'Ziektedagvergoeding',
    'Prévoyance (LPP)':        'Beroepspensioen (BVV)',
    'Assurance pension':       'Pensioensverzekering',
    'Assurance maladie':       'Ziekteverzekering',
    'Assurance dépendance':    'Zorgverzekering (afhankelijkheid)',
    'Mutualité des employeurs':'Werkgeversmutualiteit',
    'Previdenza sociale':      'Sociale zekerheid (IT)',
    'Disoccupazione':          'Werkloosheid (IT)',
    'Assicurazione infortuni': 'Arbeidsongevallenverzekering (IT)',
    'Fine rapporto':           'Eindejaarspremie (TFR)',
    'Bonus IRPEF':             'IRPEF-bonus',
    'Imposta':                 'Belasting',
    'Imposta regionale':       'Regionale belasting',
    'Retraite fédérale':       'Federaal pensioen',
    'Retraite Québec':         'Québec-pensioen (RRQ)',
    'Chômage fédéral':         'Federale werkloosheidsverzekering',
    'Parentalité Québec':      'Québec-ouderschapsverlof (RQAP)',
    'Santé Québec':            'Québec-gezondheidsfonds (FSS)',
    'Impôt fédéral':           'Federale belasting',
    'Impôt provincial':        'Gewestelijke belasting',
    'Autres':                  'Overige',

    // Formules / labels techniques
    'Taux salarial':           'Werknemerstarief',
    'Taux patronal':           'Werkgeverstarief',
    'Montant salarial':        'Werknemersbedrag',
    'Montant patronal':        'Werkgeversbedrag',
    'Montant allègement':      'Verminderingsbedrag',
    'Net imposable':           'Belastbaar nettoloon',
    'taux effectif':           'effectief tarief',
    'Tranche mensuelle':       'Maandelijkse belastingschijf',
    'Base imposée':            'Belastbare basis',
    'Taux':                    'Tarief',
    'Retenue':                 'Inhouding',

    // App meta
    'simulateur pédagogique · bulletin de paye français · v0.1.8':
      'educatieve simulator · Franse loonbrief · v0.1.8',
    'paye ton bulletin':       'Jouw loonbrief',

    // Accessibilité
    'ACCESSIBILITÉ':           'TOEGANKELIJKHEID',
    'Mode malvoyant':          'Modus slechtzienden',
    'Zoom ×2':                 'Zoom ×2',
    'Mode noir & blanc':       'Zwart-witmodus',
    'Mode dactylo':            'Typemachinemodus',
    'POLICE D\'ÉCRITURE':      'LETTERTYPE',

    // Erreurs
    'Salaire brut invalide — saisir un montant positif.':
      'Ongeldig brutoloon — voer een positief bedrag in.',
    '⚠ Salaire brut invalide — saisir un montant positif.':
      '⚠ Ongeldig brutoloon — voer een positief bedrag in.',
    '⚠ Année invalide.':       '⚠ Ongeldig jaar.',
    'ERREUR :':                'FOUT :',
    'Traduction échouée :':    'Vertaling mislukt :',

    // Quizz community
    'Aucune entrée pour ce pays.':        'Geen gegevens voor dit land.',
    'chargement…':                        'laden…',
    'Pseudo requis.':                     'Gebruikersnaam vereist.',
    'Minimum 5 questions requises.':      'Minimaal 5 vragen vereist.',
    '✓ Score soumis !':                   '✓ Score ingediend!',
    'SOUMETTRE':                          'INDIENEN',
    '💡 Proposer une question':           '💡 Vraag voorstellen',
    '// Proposer une question':           '// Vraag voorstellen',
    'QUESTION *':                         'VRAAG *',
    'RÉPONSE':                            'ANTWOORD',
    'SOURCE LÉGALE':                      'WETTELIJKE BRON',
    'PSEUDO':                             'GEBRUIKERSNAAM',
    'ENVOYER':                            'VERZENDEN',
    '✓ Merci pour votre contribution !':  '✓ Bedankt voor uw bijdrage!',
    'Question requise.':                  'Vraag vereist.',
    'BONNE RÉPONSE':                      'JUIST ANTWOORD',
    'RÉPONSES ALTERNATIVES':              'ALTERNATIEVE ANTWOORDEN',
    'MAUVAISES RÉPONSES / DISTRACTEURS':  'FOUT ANTWOORDEN / DISTRACTOREN',
    '🗳 Voter pour une question':          '🗳 Stemmen op een vraag',
    '// Questions proposées par la communauté': '// Door de community voorgestelde vragen',
    'Aucune suggestion pour ce pays.':    'Geen suggesties voor dit land.',
    '✓ voté':                             '✓ gestemd',
    '👍 voter':                           '👍 stemmen',
  },
};

// ── Index de langue pour les tables [en, de, nl, it, es] ──────────────────────
const _LANG_IDX = { en: 0, de: 1, nl: 2, it: 3, es: 4 };

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
