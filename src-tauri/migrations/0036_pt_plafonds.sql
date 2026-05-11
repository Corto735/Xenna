-- ═══════════════════════════════════════════════════════════════════════════════
-- 0036 — Portugal : plafonds de référence
--
-- PT_SMN : Salário Mínimo Nacional mensuel (référence, non utilisé pour
--          plafonner l'assiette SS qui est le salaire réel sans plafond
--          pour le régime général).
--          Source : DL annuels.
-- ═══════════════════════════════════════════════════════════════════════════════

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('PT_SMN', '2015-01-01', '2016-01-01', '505.00',  'MENSUEL'),  -- DL 144/2015
  ('PT_SMN', '2016-01-01', '2017-01-01', '530.00',  'MENSUEL'),  -- DL 254-A/2015
  ('PT_SMN', '2017-01-01', '2018-01-01', '557.00',  'MENSUEL'),  -- DL 86-B/2016
  ('PT_SMN', '2018-01-01', '2019-01-01', '580.00',  'MENSUEL'),  -- DL 156/2017
  ('PT_SMN', '2019-01-01', '2020-01-01', '600.00',  'MENSUEL'),  -- DL 619/2018
  ('PT_SMN', '2020-01-01', '2021-01-01', '635.00',  'MENSUEL'),  -- DL 107/2019
  ('PT_SMN', '2021-01-01', '2022-01-01', '665.00',  'MENSUEL'),  -- DL 109-G/2021
  ('PT_SMN', '2022-01-01', '2023-01-01', '705.00',  'MENSUEL'),  -- DL 109-A/2021
  ('PT_SMN', '2023-01-01', '2024-01-01', '760.00',  'MENSUEL'),  -- DL 119/2022
  ('PT_SMN', '2024-01-01', '2025-01-01', '820.00',  'MENSUEL'),  -- DL 107/2023
  ('PT_SMN', '2025-01-01', NULL,          '870.00',  'MENSUEL'); -- DL 125/2024
