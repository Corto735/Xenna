-- ============================================================
-- CANADA & QUÉBEC — Cotisations sociales et retenues
-- Sources : ARC T4001, Revenu Québec, Service Canada.
-- ============================================================

-- ── Définitions ─────────────────────────────────────────────
INSERT INTO cotisation (code, libelle, organisme_id, categorie,
  applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES

  -- RPC (Canada hors Québec)
  ('CA_RPC', 'RPC — Régime de pensions du Canada (phase 1)',
    (SELECT id FROM organisme WHERE code='CRA'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ',
    'Pension de retraite fédérale obligatoire. Taux sal. = taux pat. Exonération de base : 3 500 CAD/an. Plafond = MGA (68 500 CAD en 2024). Bonification progressive 2019-2023 : 4,95 % -> 5,95 %.'),

  ('CA_RPC2', 'RPC2 — Bonification supplémentaire (phase 2, dès 2024)',
    (SELECT id FROM organisme WHERE code='CRA'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ',
    'Assiette = gains entre MGA et MGAP2 (73 200 CAD en 2024). Taux : 4 % sal. = 4 % pat. Sans exonération de base. Introduit par L.C. 2018 ch. 12.'),

  -- AE Canada (hors Québec)
  ('CA_AE', 'AE — Assurance-emploi (taux général, hors Québec)',
    (SELECT id FROM organisme WHERE code='EDSC'),
    'CHOMAGE', 1, 1, 'BRUT_PLAFONNÉ',
    'Plafond = MAGA (63 200 CAD en 2024). Employeur = salarié x 1,4 (art. 68 LAE). Taux 2024 : 1,66 % sal. + 2,324 % pat.'),

  -- RRQ (Québec — remplace le RPC)
  ('QC_RRQ', 'RRQ — Régime de rentes du Québec (phase 1)',
    (SELECT id FROM organisme WHERE code='RETRAITE_QC'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ',
    'Équivalent québécois du RPC. Taux supérieur : 6,40 % sal. + 6,40 % pat. en 2024. Même MGA que le RPC. Exonération de base identique (3 500 CAD/an).'),

  ('QC_RRQ2', 'RRQ2 — Bonification supplémentaire (phase 2, dès 2024)',
    (SELECT id FROM organisme WHERE code='RETRAITE_QC'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ',
    'Assiette = gains entre MGA et MGAP2. Taux : 4 % sal. = 4 % pat. Identique au RPC2 sauf que géré par Retraite Québec.'),

  -- AE Québec (taux réduit car RQAP couvre les prestations parentales)
  ('QC_AE', 'AE — Assurance-emploi (taux réduit Québec)',
    (SELECT id FROM organisme WHERE code='EDSC'),
    'CHOMAGE', 1, 1, 'BRUT_PLAFONNÉ',
    'Taux réduit car RQAP couvre les prestations parentales (art. 69 LAE). Taux 2024 : 1,31 % sal. + 1,834 % pat. Plafond MAGA identique au régime général.'),

  -- RQAP (Québec — assurance parentale)
  ('QC_RQAP', 'RQAP — Régime québécois d''assurance parentale',
    (SELECT id FROM organisme WHERE code='RQAP'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
    'Assurance parentale unique au Québec. Taux 2024 : 0,494 % sal. + 0,692 % pat. Plafond MAGA-RQAP : 94 000 CAD en 2024. Couvre : congé maternité (18 sem.), paternité (5 sem.), parental (40 ou 25 sem.), adoption.'),

  -- FSS (Fonds des services de santé — Québec, employeur uniquement)
  ('QC_FSS', 'FSS — Fonds des services de santé (Québec)',
    (SELECT id FROM organisme WHERE code='CRA'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
    'Contribution patronale finançant le régime public de santé québécois. Taux variable selon masse salariale totale (1,65 % à 4,26 %). Taux indicatif : 2,05 % (secteur services, masse salariale intermediaire). Pas de plafond par salarié.'),

  -- CNT / CNESST contribution normes du travail
  ('QC_CNT', 'CNT — Contribution aux normes du travail (CNESST)',
    (SELECT id FROM organisme WHERE code='CNESST'),
    'AUTRES', 1, 1, 'BRUT_PLAFONNÉ',
    'Cotisation patronale : 0,06 % sur les salaires jusqu''au plafond RQAP (94 000 CAD). Finance les activités de la CNESST (inspection, aide aux travailleurs lésés).'),

  -- Impôt fédéral (retenue à la source)
  ('CA_IMPOT_FED', 'Impôt fédéral sur le revenu — retenue à la source',
    (SELECT id FROM organisme WHERE code='CRA'),
    'AUTRES', 1, 1, 'SPECIFIQUE',
    'Retenue mensuelle estimative. Barème 2024 : 15/20,5/26/29/33 %. MPB 2024 : 15 705 CAD (crédit = 2 355,75 CAD). Régularisation décembre ou déclaration T1.'),

  -- Impôt provincial Ontario (retenue à la source)
  ('ON_IMPOT_PROV', 'Impôt provincial Ontario — retenue à la source',
    (SELECT id FROM organisme WHERE code='CRA'),
    'AUTRES', 1, 1, 'SPECIFIQUE',
    'Ontario — province de référence hors Québec. Barème 2024 : 5,05/9,15/11,16/12,16/13,16 %. MPB Ontario : 11 865 CAD (crédit = 599,18 CAD). Retenu conjointement avec le fédéral (formulaire TD1ON).'),

  -- Impôt provincial Québec
  ('QC_IMPOT_PROV', 'Impôt provincial Québec — retenue à la source',
    (SELECT id FROM organisme WHERE code='RQ'),
    'AUTRES', 1, 1, 'SPECIFIQUE',
    'Québec perçoit son propre impôt (formulaire TP-1015.3). Barème 2024 : 14/19/24/25,75 %. MPB Québec 2024 : 17 183 CAD (crédit = 2 405,62 CAD). Régularisation via TP-1.');

-- ── Taux RPC (Canada hors Québec) ───────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='CA_RPC'), '2019-01-01','2019-12-31','0.0510','0.0510','Bonification phase 1 : 4,95 % -> 5,10 %'),
  ((SELECT id FROM cotisation WHERE code='CA_RPC'), '2020-01-01','2020-12-31','0.0525','0.0525','5,25 %'),
  ((SELECT id FROM cotisation WHERE code='CA_RPC'), '2021-01-01','2021-12-31','0.0545','0.0545','5,45 %'),
  ((SELECT id FROM cotisation WHERE code='CA_RPC'), '2022-01-01','2022-12-31','0.0570','0.0570','5,70 %'),
  ((SELECT id FROM cotisation WHERE code='CA_RPC'), '2023-01-01','2023-12-31','0.0595','0.0595','5,95 % — taux cible atteint'),
  ((SELECT id FROM cotisation WHERE code='CA_RPC'), '2024-01-01', NULL,        '0.0595','0.0595','Stable depuis 2023');

-- ── Taux RPC2 (dès 2024) ────────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='CA_RPC2'), '2024-01-01', NULL, '0.0400','0.0400','Phase 2. Assiette : gains entre MGA et MGAP2. Sans exonération de base.');

-- ── Taux AE Canada ───────────────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='CA_AE'), '2019-01-01','2019-12-31','0.0162','0.02268','Emp. x1,4'),
  ((SELECT id FROM cotisation WHERE code='CA_AE'), '2020-01-01','2021-12-31','0.0158','0.02212','Taux stable 2020-2021'),
  ((SELECT id FROM cotisation WHERE code='CA_AE'), '2022-01-01','2022-12-31','0.0158','0.02212','Taux stable'),
  ((SELECT id FROM cotisation WHERE code='CA_AE'), '2023-01-01','2023-12-31','0.0163','0.02282','Légère hausse'),
  ((SELECT id FROM cotisation WHERE code='CA_AE'), '2024-01-01','2024-12-31','0.0166','0.02324','Taux 2024'),
  ((SELECT id FROM cotisation WHERE code='CA_AE'), '2025-01-01', NULL,        '0.0164','0.02296','Taux 2025');

-- ── Taux RRQ (Québec) ────────────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_RRQ'), '2019-01-01','2019-12-31','0.0555','0.0555','Taux RRQ > RPC dès la bonification'),
  ((SELECT id FROM cotisation WHERE code='QC_RRQ'), '2020-01-01','2020-12-31','0.0570','0.0570',''),
  ((SELECT id FROM cotisation WHERE code='QC_RRQ'), '2021-01-01','2021-12-31','0.0590','0.0590',''),
  ((SELECT id FROM cotisation WHERE code='QC_RRQ'), '2022-01-01','2022-12-31','0.0615','0.0615',''),
  ((SELECT id FROM cotisation WHERE code='QC_RRQ'), '2023-01-01','2023-12-31','0.0640','0.0640','Taux cible atteint'),
  ((SELECT id FROM cotisation WHERE code='QC_RRQ'), '2024-01-01', NULL,        '0.0640','0.0640','Stable');

-- ── Taux RRQ2 (Québec, dès 2024) ────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_RRQ2'), '2024-01-01', NULL, '0.0400','0.0400','Identique RPC2. Géré par Retraite Québec.');

-- ── Taux AE Québec (réduit) ──────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_AE'), '2019-01-01','2019-12-31','0.0125','0.01750','Taux réduit Québec'),
  ((SELECT id FROM cotisation WHERE code='QC_AE'), '2020-01-01','2020-12-31','0.0120','0.01680',''),
  ((SELECT id FROM cotisation WHERE code='QC_AE'), '2021-01-01','2021-12-31','0.0118','0.01652',''),
  ((SELECT id FROM cotisation WHERE code='QC_AE'), '2022-01-01','2022-12-31','0.0120','0.01680',''),
  ((SELECT id FROM cotisation WHERE code='QC_AE'), '2023-01-01','2023-12-31','0.0127','0.01778',''),
  ((SELECT id FROM cotisation WHERE code='QC_AE'), '2024-01-01','2024-12-31','0.0131','0.01834','Taux 2024'),
  ((SELECT id FROM cotisation WHERE code='QC_AE'), '2025-01-01', NULL,        '0.0131','0.01834','Estimation stable');

-- ── Taux RQAP ────────────────────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_RQAP'), '2019-01-01','2020-12-31','0.00526','0.00736','Taux 2019-2020'),
  ((SELECT id FROM cotisation WHERE code='QC_RQAP'), '2021-01-01', NULL,        '0.00494','0.00692','Taux stable dès 2021. Révision annuelle possible.');

-- ── Taux FSS ─────────────────────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_FSS'), '2019-01-01', NULL, '0.0000','0.0205',
   'Taux indicatif secteur services, masse salariale intermediaire. Varie de 1,65 % à 4,26 %.');

-- ── Taux CNT ─────────────────────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_CNT'), '2019-01-01', NULL, '0.0000','0.0006',
   'Stable. Plafond = MAGA RQAP. Finance la CNESST.');
