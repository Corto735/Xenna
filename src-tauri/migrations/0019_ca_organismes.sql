-- ============================================================
-- CANADA & QUÉBEC — Organismes collecteurs et références législatives
-- Sources : ARC (CRA), Revenu Québec, CNESST, Service Canada
-- ============================================================

INSERT INTO organisme (code, libelle, url) VALUES
  ('CRA',     'ARC — Agence du revenu du Canada / Canada Revenue Agency',   'https://www.canada.ca/fr/agence-revenu.html'),
  ('EDSC',    'Service Canada / EDSC — Assurance-emploi',                   'https://www.canada.ca/fr/emploi-developpement-social.html'),
  ('RQ',      'Revenu Québec',                                              'https://www.revenuquebec.ca'),
  ('CNESST',  'CNESST — Commission des normes, équité, santé et sécurité',  'https://www.cnesst.gouv.qc.ca'),
  ('RQAP',    'RQAP — Conseil de gestion de l''assurance parentale',        'https://www.rqap.gouv.qc.ca'),
  ('RETRAITE_QC', 'Retraite Québec — Régime de rentes du Québec (RRQ)',    'https://www.retraitequebec.gouv.qc.ca');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES

  ('RPC_LRC', 'LOI',
   'Régime de pensions du Canada (RPC) / Canada Pension Plan (CPP)',
   'L.R.C. 1985, ch. C-8', '1965-04-03', '1966-01-01',
   'Régime de retraite obligatoire fédéral. Taux 2024 : 5,95 % salarié + 5,95 % employeur sur gains jusqu''au MGA (68 500 CAD). Exonération de base : 3 500 CAD/an. Bonification RPC2 depuis 2024.'),

  ('AE_LC', 'LOI',
   'Loi sur l''assurance-emploi (LAE) / Employment Insurance Act (EIA)',
   'L.C. 1996, ch. 23', '1996-07-05', '1997-01-01',
   'Assurance-emploi fédérale. Taux 2024 : 1,66 % salarié, 2,324 % employeur (×1,4). Québec taux réduit : 1,31 %/1,834 %. Plafond 2024 : 63 200 CAD.'),

  ('RRQ_LRQ', 'LOI',
   'Régime de rentes du Québec (RRQ) / Quebec Pension Plan (QPP)',
   'RLRQ, ch. R-9', '1965-07-06', '1966-01-01',
   'Remplace le RPC pour les travailleurs québécois. Taux 2024 : 6,40 % salarié + 6,40 % employeur. Même MGA que RPC. Bonification RRQ2 depuis 2024.'),

  ('RQAP_LRQ', 'LOI',
   'Loi sur l''assurance parentale (RQAP)',
   'RLRQ, ch. A-29.011', '2001-05-21', '2006-01-01',
   'Assurance parentale québécoise. Remplace les prestations parentales de l''AE pour les Québécois. Taux 2024 : 0,494 % salarié + 0,692 % employeur. Plafond 2024 : 94 000 CAD. C''est pourquoi le taux AE au Québec est réduit.'),

  ('FSS_LRQ', 'LOI',
   'Loi sur la Régie de l''assurance maladie du Québec — Fonds des services de santé (FSS)',
   'RLRQ, ch. R-5', '1970-12-19', '1971-01-01',
   'FSS (Fonds des services de santé) : cotisation patronale finançant le régime public de santé québécois. Taux 2024 : 1,65 % à 4,26 % selon la masse salariale totale. Taux indicatif : 2,05 % (masse salariale intermédiaire, secteur services).'),

  ('LIR_CA', 'LOI',
   'Loi de l''impôt sur le revenu (LIR) / Income Tax Act (ITA)',
   'L.R.C. 1985, ch. 1 (5e suppl.)', '1985-11-13', '1986-01-01',
   'Impôt fédéral sur le revenu des particuliers. Barème 2024 : 15/20,5/26/29/33 %. Montant personnel de base (MPB) : 15 705 CAD. Retenu à la source par l''employeur (formulaire TD1).'),

  ('LI_QC', 'LOI',
   'Loi sur les impôts du Québec',
   'RLRQ, ch. I-3', '1972-12-21', '1973-01-01',
   'Impôt provincial québécois. Barème 2024 : 14/19/24/25,75 %. Montant personnel de base : 17 183 CAD. Retenu à la source (formulaire TP-1015.3). Unique car Québec perçoit son propre impôt séparément du fédéral.'),

  ('LI_ON', 'LOI',
   'Loi de 2007 sur les impôts (Ontario)',
   'L.O. 2007, ch. 11, ann. A', '2007-05-17', '2007-01-01',
   'Impôt provincial de l''Ontario (province de référence hors Québec). Barème 2024 : 5,05/9,15/11,16/12,16/13,16 %. Montant personnel de base : 11 865 CAD. Retenu à la source (formulaire TD1ON).'),

  ('RPCBON_2019', 'LOI',
   'Loi no 1 d''exécution du budget de 2018 — Bonification du RPC',
   'L.C. 2018, ch. 12', '2018-06-21', '2019-01-01',
   'Bonification progressive du RPC/RRQ sur 2019-2025. Phase 1 (2019-2023) : hausse des taux de 4,95 % à 5,95 %. Phase 2 (2024+) : RPC2/RRQ2 sur les gains entre MGA et MGAP2 (73 200 CAD en 2024) au taux de 4 %.'),

  ('CNT_LNT', 'LOI',
   'Loi sur les normes du travail (CNT) — Contribution employeur',
   'RLRQ, ch. N-1.1', '1979-06-22', '1980-04-16',
   'Contribution de 0,06 % versée à la CNESST (anciennement CNT) sur les salaires jusqu''au plafond RQAP (94 000 CAD en 2024). Employeur uniquement. Très faible, souvent intégrée aux frais généraux.');
