-- ============================================================
-- CANADA & QUÉBEC — Plafonds contributifs annuels
--
-- MGA   = Maximum des gains annuels ouvrant droit à pension (RPC/RRQ)
-- MGAP2 = Maximum des gains annuels supplémentaires (RPC2/RRQ2, dès 2024)
-- MAGA  = Maximum des gains assurables AE (annuel)
-- MAGA_RQAP = Maximum des gains assurables RQAP (annuel, Québec)
--
-- Valeurs en CAD — periodicite = ANNUEL.
-- Sources : ARC T4001, Revenu Québec TP-1015.F, Service Canada.
-- ============================================================

-- MGA (Year's Maximum Pensionable Earnings) — plafond RPC/RRQ
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CA_MGA', '2019-01-01', '2019-12-31', '57400.00',  'ANNUEL'),
  ('CA_MGA', '2020-01-01', '2020-12-31', '58700.00',  'ANNUEL'),
  ('CA_MGA', '2021-01-01', '2021-12-31', '61600.00',  'ANNUEL'),
  ('CA_MGA', '2022-01-01', '2022-12-31', '64900.00',  'ANNUEL'),
  ('CA_MGA', '2023-01-01', '2023-12-31', '66600.00',  'ANNUEL'),
  ('CA_MGA', '2024-01-01', '2024-12-31', '68500.00',  'ANNUEL'),
  ('CA_MGA', '2025-01-01', NULL,          '71300.00',  'ANNUEL');

-- MGAP2 (Year's Additional Maximum Pensionable Earnings) — plafond RPC2/RRQ2
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CA_MGAP2', '2024-01-01', '2024-12-31', '73200.00',  'ANNUEL'),
  ('CA_MGAP2', '2025-01-01', NULL,          '81900.00',  'ANNUEL');

-- MAGA (Maximum des gains annuels assurables AE)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CA_MAGA', '2019-01-01', '2019-12-31', '53100.00',  'ANNUEL'),
  ('CA_MAGA', '2020-01-01', '2020-12-31', '54200.00',  'ANNUEL'),
  ('CA_MAGA', '2021-01-01', '2021-12-31', '56300.00',  'ANNUEL'),
  ('CA_MAGA', '2022-01-01', '2022-12-31', '60300.00',  'ANNUEL'),
  ('CA_MAGA', '2023-01-01', '2023-12-31', '61500.00',  'ANNUEL'),
  ('CA_MAGA', '2024-01-01', '2024-12-31', '63200.00',  'ANNUEL'),
  ('CA_MAGA', '2025-01-01', NULL,          '65700.00',  'ANNUEL');

-- MAGA_RQAP (Maximum des gains assurables RQAP — Québec)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('QC_MAGA_RQAP', '2019-01-01', '2019-12-31', '76500.00',  'ANNUEL'),
  ('QC_MAGA_RQAP', '2020-01-01', '2020-12-31', '78500.00',  'ANNUEL'),
  ('QC_MAGA_RQAP', '2021-01-01', '2021-12-31', '83500.00',  'ANNUEL'),
  ('QC_MAGA_RQAP', '2022-01-01', '2022-12-31', '88000.00',  'ANNUEL'),
  ('QC_MAGA_RQAP', '2023-01-01', '2023-12-31', '91000.00',  'ANNUEL'),
  ('QC_MAGA_RQAP', '2024-01-01', '2024-12-31', '94000.00',  'ANNUEL'),
  ('QC_MAGA_RQAP', '2025-01-01', NULL,          '97500.00',  'ANNUEL');
