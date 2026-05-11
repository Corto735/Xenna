-- ═══════════════════════════════════════════════════════════════════════════════
-- 0032 — Espagne : bases de cotisation (topes mínimo et máximo)
--
-- ES_BASE_MAX : base máxima de cotización mensuelle
--   Source : Ordenes de cotización annuelles (LGSS art. 147)
--
-- ES_BASE_MIN : base mínima de cotización (≈ SMI mensuel brut)
--   Source : RD annuels fixant le Salario Mínimo Interprofesional (LGSS art. 162)
--   Note   : le SMI est rémunéré en 14 mensualités (12 + 2 gratifications).
--             La base mínima est techniquement (SMI_jour × 30) mais est ici
--             exprimée en équivalent mensuel 12 mois pour simplification.
-- ═══════════════════════════════════════════════════════════════════════════════

-- Base maximale ----------------------------------------------------------------
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('ES_BASE_MAX', '2015-01-01', '2016-01-01', '3606.00',  'MENSUEL'),  -- Orden ESS/86/2015
  ('ES_BASE_MAX', '2016-01-01', '2017-01-01', '3642.00',  'MENSUEL'),  -- Orden ESS/106/2016
  ('ES_BASE_MAX', '2017-01-01', '2018-01-01', '3751.20',  'MENSUEL'),  -- Orden ESS/106/2017
  ('ES_BASE_MAX', '2018-01-01', '2019-01-01', '3803.70',  'MENSUEL'),  -- Orden ESS/55/2018
  ('ES_BASE_MAX', '2019-01-01', '2022-01-01', '4070.10',  'MENSUEL'),  -- RD 1462/2018 ; PGE prorrogada 2020-2021
  ('ES_BASE_MAX', '2022-01-01', '2023-01-01', '4139.40',  'MENSUEL'),  -- Ley 22/2021 PGE ; Orden ICT/182/2022
  ('ES_BASE_MAX', '2023-01-01', '2024-01-01', '4495.50',  'MENSUEL'),  -- Ley 31/2022 PGE ; Orden CEM/74/2023
  ('ES_BASE_MAX', '2024-01-01', '2025-01-01', '4720.50',  'MENSUEL'),  -- Ley 12/2023 PGE ; Orden TRM/43/2024
  ('ES_BASE_MAX', '2025-01-01', NULL,          '4909.50',  'MENSUEL'); -- PGE 2025 ; Orden TRM/42/2025

-- Base minimale ----------------------------------------------------------------
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('ES_BASE_MIN', '2015-01-01', '2016-01-01', '648.60',   'MENSUEL'),  -- RD 1106/2014 (SMI 2015 : 21,62 €/jour)
  ('ES_BASE_MIN', '2016-01-01', '2017-01-01', '655.20',   'MENSUEL'),  -- RD 1171/2015 (SMI 2016 : 21,84 €/jour)
  ('ES_BASE_MIN', '2017-01-01', '2018-01-01', '707.70',   'MENSUEL'),  -- RD 742/2016  (SMI 2017 : 23,59 €/jour, +8%)
  ('ES_BASE_MIN', '2018-01-01', '2019-01-01', '735.90',   'MENSUEL'),  -- RD 1077/2017 (SMI 2018 : 24,53 €/jour, +4%)
  ('ES_BASE_MIN', '2019-01-01', '2020-01-01', '900.00',   'MENSUEL'),  -- RD 1462/2018 (SMI 2019 : +22,3 %, hausse historique)
  ('ES_BASE_MIN', '2020-01-01', '2021-09-01', '950.00',   'MENSUEL'),  -- RD 231/2020  (SMI 2020 : +5,5 %)
  ('ES_BASE_MIN', '2021-09-01', '2022-01-01', '965.00',   'MENSUEL'),  -- RD 817/2021  (SMI 01/09/2021, revalorisation tardive)
  ('ES_BASE_MIN', '2022-01-01', '2023-01-01', '1000.00',  'MENSUEL'),  -- RD 152/2022  (SMI 2022 : +3,6 %)
  ('ES_BASE_MIN', '2023-01-01', '2024-01-01', '1080.00',  'MENSUEL'),  -- RD 99/2023   (SMI 2023 : +8 %)
  ('ES_BASE_MIN', '2024-01-01', '2025-01-01', '1134.00',  'MENSUEL'),  -- RD 145/2024  (SMI 2024 : +5 %)
  ('ES_BASE_MIN', '2025-01-01', NULL,          '1184.00',  'MENSUEL'); -- RD 8/2025    (SMI 2025 : +4,4 %)
