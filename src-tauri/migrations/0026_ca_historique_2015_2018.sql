-- ============================================================
-- CANADA & QUÉBEC — Taux et plafonds historiques 2015-2018
--
-- Toutes les cotisations CA/QC démarraient au 01/01/2019 (0021).
-- Cette migration couvre 2015-2018 pour les simulations historiques.
--
-- TAUX PRÉ-BONIFICATION :
--   CA_RPC / QC_RRQ : taux fixes jusqu'au démarrage de la bonification
--   en 2019 (L.C. 2018 ch. 12 / L.Q. 2018 ch. 2).
--   CA_AE, QC_AE, QC_RQAP : varient chaque année (publication annuelle ARC / RQ).
--
-- Sources : ARC T4001, Revenu Québec TP-1015.F, Retraite Québec.
-- ============================================================

-- ── CA_RPC : stable 4,95 % (2015-2018) ──────────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='CA_RPC'),
   '2015-01-01', '2019-01-01', '0.0495', '0.0495',
   'Taux fixe pré-bonification. Inchangé depuis 2003 (L.R.C. 1985 ch. C-8). Bonification progressive dès 2019 (L.C. 2018 ch. 12).');

-- ── QC_RRQ : stable 5,25 % (2015-2018) ──────────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_RRQ'),
   '2015-01-01', '2019-01-01', '0.0525', '0.0525',
   'Taux RRQ > RPC depuis 2012 (relèvement progressif pour équilibre actuariel). Bonification progressive dès 2019 (L.Q. 2018 ch. 2).');

-- ── CA_AE : taux annuels 2015-2018 (hors Québec) ────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='CA_AE'),
   '2015-01-01', '2016-01-01', '0.0188', '0.02632', 'Taux 2015. Pat = sal × 1,4.'),
  ((SELECT id FROM cotisation WHERE code='CA_AE'),
   '2016-01-01', '2017-01-01', '0.0188', '0.02632', 'Taux 2016. Stable.'),
  ((SELECT id FROM cotisation WHERE code='CA_AE'),
   '2017-01-01', '2018-01-01', '0.0163', '0.02282', 'Taux 2017. Réduction suite à révision actuarielle.'),
  ((SELECT id FROM cotisation WHERE code='CA_AE'),
   '2018-01-01', '2019-01-01', '0.0166', '0.02324', 'Taux 2018.');

-- ── QC_AE : taux annuels 2015-2018 (taux réduit Québec) ─────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_AE'),
   '2015-01-01', '2016-01-01', '0.0154', '0.02156', 'Taux réduit QC 2015. Pat = sal × 1,4.'),
  ((SELECT id FROM cotisation WHERE code='QC_AE'),
   '2016-01-01', '2017-01-01', '0.0152', '0.02128', 'Taux réduit QC 2016.'),
  ((SELECT id FROM cotisation WHERE code='QC_AE'),
   '2017-01-01', '2018-01-01', '0.0127', '0.01778', 'Taux réduit QC 2017. Réduction parallèle au régime général.'),
  ((SELECT id FROM cotisation WHERE code='QC_AE'),
   '2018-01-01', '2019-01-01', '0.0130', '0.01820', 'Taux réduit QC 2018.');

-- ── QC_RQAP : taux annuels 2015-2018 ────────────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_RQAP'),
   '2015-01-01', '2016-01-01', '0.00559', '0.00782', 'Taux RQAP 2015. Règlement RQAP, Décret annuel.'),
  ((SELECT id FROM cotisation WHERE code='QC_RQAP'),
   '2016-01-01', '2017-01-01', '0.00548', '0.00767', 'Taux RQAP 2016.'),
  ((SELECT id FROM cotisation WHERE code='QC_RQAP'),
   '2017-01-01', '2018-01-01', '0.00537', '0.00752', 'Taux RQAP 2017.'),
  ((SELECT id FROM cotisation WHERE code='QC_RQAP'),
   '2018-01-01', '2019-01-01', '0.00534', '0.00747', 'Taux RQAP 2018.');

-- ── QC_FSS : indicatif 2015-2018 (taux inchangé) ────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_FSS'),
   '2015-01-01', '2019-01-01', '0.0000', '0.0205',
   'Taux indicatif secteur services, masse salariale intermédiaire. Inchangé depuis 2015.');

-- ── QC_CNT : stable 2015-2018 ────────────────────────────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='QC_CNT'),
   '2015-01-01', '2019-01-01', '0.0000', '0.0006',
   'Stable. Plafond = MAGA RQAP. Finance la CNESST (ex-CNT).');

-- ============================================================
-- Plafonds contributifs annuels 2015-2018
-- (valeurs en CAD/an — periodicite = ANNUEL)
-- Les entrées 2019+ existent déjà dans la migration 0020.
-- ============================================================

-- CA_MGA (Maximum des gains annuels ouvrant droit à pension — RPC/RRQ)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CA_MGA', '2015-01-01', '2016-01-01', '53600.00', 'ANNUEL'),
  ('CA_MGA', '2016-01-01', '2017-01-01', '54900.00', 'ANNUEL'),
  ('CA_MGA', '2017-01-01', '2018-01-01', '55300.00', 'ANNUEL'),
  ('CA_MGA', '2018-01-01', '2019-01-01', '55900.00', 'ANNUEL');

-- CA_MAGA (Maximum des gains annuels assurables AE)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CA_MAGA', '2015-01-01', '2016-01-01', '49500.00', 'ANNUEL'),
  ('CA_MAGA', '2016-01-01', '2017-01-01', '50800.00', 'ANNUEL'),
  ('CA_MAGA', '2017-01-01', '2018-01-01', '51300.00', 'ANNUEL'),
  ('CA_MAGA', '2018-01-01', '2019-01-01', '51700.00', 'ANNUEL');

-- QC_MAGA_RQAP (Maximum des gains assurables RQAP — Québec)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('QC_MAGA_RQAP', '2015-01-01', '2016-01-01', '70375.00', 'ANNUEL'),
  ('QC_MAGA_RQAP', '2016-01-01', '2017-01-01', '71500.00', 'ANNUEL'),
  ('QC_MAGA_RQAP', '2017-01-01', '2018-01-01', '72500.00', 'ANNUEL'),
  ('QC_MAGA_RQAP', '2018-01-01', '2019-01-01', '74000.00', 'ANNUEL');
