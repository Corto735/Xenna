-- ============================================================
-- ITALIE — Organismes collecteurs et références législatives
-- Sources : INPS, INAIL, Agenzia delle Entrate, TUIR
-- ============================================================

INSERT INTO organisme (code, libelle, url) VALUES
  ('INPS',   'INPS — Istituto Nazionale della Previdenza Sociale',             'https://www.inps.it'),
  ('INAIL',  'INAIL — Istituto Nazionale Assicurazione Infortuni sul Lavoro',  'https://www.inail.it'),
  ('ADE_IT', 'Agenzia delle Entrate (IRPEF / ritenute)',                        'https://www.agenziaentrate.gov.it'),
  ('MEF_IT', 'MEF — Ministero dell''Economia e delle Finanze',                 'https://www.mef.gov.it');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES

  ('L_297_1982', 'LOI',
   'Legge 297/1982 — Disciplina del trattamento di fine rapporto (TFR)',
   'L. 297/1982', '1982-05-29', '1982-06-26',
   'TFR : 6,91 % de la rémunération annuelle brute (retribuzione / 13,5). '
   'Fondo di garanzia INPS (0,20 % patronal) garantit le paiement en cas de faillite employeur.'),

  ('TUIR_DPR_917', 'DECRET',
   'TUIR — Testo Unico delle Imposte sui Redditi',
   'DPR 917/1986', '1986-12-22', '1988-01-01',
   'Code fiscal IRPEF. Art. 11 : barème progressif. Art. 13 : détractions travail salarié. '
   'Art. 23 : retenue à la source obligatoire par le sostituto d''imposta (employeur).'),

  ('L_153_1969', 'LOI',
   'Legge 153/1969 — Base contributiva INPS (imponibile previdenziale)',
   'L. 153/1969', '1969-04-30', '1969-05-15',
   'Définit l''imponibile previdenziale : toute rémunération perçue en raison du travail '
   'est soumise aux cotisations INPS. Base légale des contributions INPS.'),

  ('DPR_1124_1965', 'DECRET',
   'DPR 1124/1965 — Testo Unico INAIL',
   'DPR 1124/1965', '1965-06-30', '1965-12-31',
   'Assurance accidents du travail et maladies professionnelles. Obligatoire, à charge patronale. '
   'Tarification par voce di tariffa ATECO et niveau de risque. Auto-liquidazione annuelle au 16/02.'),

  ('L_335_1995', 'LOI',
   'Legge 335/1995 — Riforma Dini : massimale contributivo',
   'L. 335/1995', '1995-08-08', '1996-01-01',
   'Réforme des retraites. Instaure le massimale contributivo IVS pour les nouveaux entrants '
   '(sans ancienneté INPS au 31/12/1995). Crée le régime contributivo pur (pension proportionnelle aux cotisations).'),

  ('L_92_2012', 'LOI',
   'Legge 92/2012 — Riforma Fornero : ASpI + cotisation CDD',
   'L. 92/2012', '2012-06-27', '2012-07-18',
   'Création de l''ASpI (Assicurazione Sociale per l''Impiego). Taux patronal ordinaire : 1,61 %. '
   'Majoration CDD : +1,40 % patronal. Cotisation chômage salarié supprimée.'),

  ('L_228_2012', 'LOI',
   'Legge 228/2012 — Suppression cotisation chômage salarié',
   'L. 228/2012', '2012-12-24', '2013-01-01',
   'Art. 1 c. 235 : supprime la cotisation chômage à charge du salarié (0,30 % antérieure). '
   'Seule reste la cotisation patronale ordinaire de 1,61 %.'),

  ('DLGS_22_2015', 'DECRET',
   'D.Lgs. 22/2015 — NASpI (Jobs Act)',
   'D.Lgs. 22/2015', '2015-03-04', '2015-05-01',
   'Remplace l''ASpI par la NASpI (Nuova Assicurazione Sociale per l''Impiego). '
   'Durée d''indemnisation = moitié des semaines cotisées dans les 4 dernières années (max 24 mois). '
   'Taux inchangé : 1,61 % exclusivement patronal.'),

  ('L_234_2021', 'LOI',
   'Legge 234/2021 — Bilancio 2022 : IRPEF 4 tranches + esonero',
   'L. 234/2021', '2021-12-30', '2022-01-01',
   'Réforme IRPEF : passage de 5 à 4 tranches (23/25/35/43 %). '
   'Esonero contributivo salarié -0,8 % sur IVS pour reddito ≤ 35 000 € (2ème semestre 2022 seulement).'),

  ('DL_115_2022', 'DECRET',
   'DL 115/2022 — Decreto Aiuti-bis : esonero H2 2022',
   'DL 115/2022 (conv. L. 142/2022)', '2022-08-09', '2022-07-01',
   'Art. 20 : esonero contributivo salarié -0,8 point IVS pour reddito ≤ 35 000 €. '
   'Applicable juillet–décembre 2022. Rétroactif au 01/07/2022 lors de la conversion en loi.'),

  ('L_197_2022', 'LOI',
   'Legge 197/2022 — Bilancio 2023 : taglio cuneo renforcé',
   'L. 197/2022', '2022-12-29', '2023-01-01',
   'Taglio cuneo fiscal côté salarié : -3 pp IVS si reddito ≤ 25 000 € ; '
   '-2 pp IVS si reddito 25 001–35 000 €. Art. 1 c. 281-286.'),

  ('L_213_2023', 'LOI',
   'Legge 213/2023 — Bilancio 2024 : IRPEF 3 tranches + bonus',
   'L. 213/2023', '2023-12-30', '2024-01-01',
   'IRPEF : passage de 4 à 3 tranches (23/35/43 %). '
   'Taglio cuneo converti en bonus IRPEF (trattamento integrativo) : '
   '€ 1 200/an pour reddito ≤ 35 000 €. Art. 1 c. 2-9.'),

  ('L_207_2024', 'LOI',
   'Legge 207/2024 — Bilancio 2025 : refonte du taglio cuneo',
   'L. 207/2024', '2024-12-30', '2025-01-01',
   'Nouveau taglio cuneo 2025 : '
   'bonus = 7,1 % × reddito (plafonné 1 400 €) pour reddito ≤ 20 000 € ; '
   'detrazione fissa 1 000 € pour reddito 20 001–40 000 €. Art. 1 c. 4-9.');
