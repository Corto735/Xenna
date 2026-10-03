-- ============================================================
-- CANADA / QUÉBEC — taux 2026 et historique corrigé
--
-- 2026 (aucune valeur 2026 n'était en base) :
--   Assurance-emploi hors Québec 1,63 % (employeur × 1,4 = 2,282 %),
--   Québec 1,30 % (1,82 %) ; RRQ 6,30 % (6,40 % en 2025 : cotisation de base
--   ramenée à 5,30 %) ; RQAP 0,430 % / 0,602 %.
-- Historique :
--   RRQ 5,325 % en 2016, 5,40 % en 2017-2018 (5,25 % partout en base) ;
--   RQAP 0,548 % / 0,767 % en 2017-2018, 0,526 % / 0,736 % en 2019,
--   0,494 % / 0,692 % dès 2020 ;
--   AE Québec 1,20 % en 2021 (taux gelés 2021-2022) et 1,32 % en 2024.
-- Dates de fin rendues exclusives.
-- Sources : ARC, T4127 122ᵉ édition (01/01/2026) ; EDSC, taux de cotisation
-- AE 2026 ; Revenu Québec, taux RRQ et RQAP par année.
-- ============================================================
DELETE FROM cotisation_taux WHERE cotisation_id IN
  (SELECT id FROM cotisation WHERE code IN ('CA_AE', 'QC_AE', 'QC_RRQ', 'QC_RQAP'));
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
    ((SELECT id FROM cotisation WHERE code = 'CA_AE'), '2015-01-01', '2017-01-01', '0.0188', '0.02632', NULL),
    ((SELECT id FROM cotisation WHERE code = 'CA_AE'), '2017-01-01', '2018-01-01', '0.0163', '0.02282', NULL),
    ((SELECT id FROM cotisation WHERE code = 'CA_AE'), '2018-01-01', '2019-01-01', '0.0166', '0.02324', NULL),
    ((SELECT id FROM cotisation WHERE code = 'CA_AE'), '2019-01-01', '2020-01-01', '0.0162', '0.02268', NULL),
    ((SELECT id FROM cotisation WHERE code = 'CA_AE'), '2020-01-01', '2023-01-01', '0.0158', '0.02212', 'Gelé en 2021 et 2022'),
    ((SELECT id FROM cotisation WHERE code = 'CA_AE'), '2023-01-01', '2024-01-01', '0.0163', '0.02282', NULL),
    ((SELECT id FROM cotisation WHERE code = 'CA_AE'), '2024-01-01', '2025-01-01', '0.0166', '0.02324', NULL),
    ((SELECT id FROM cotisation WHERE code = 'CA_AE'), '2025-01-01', '2026-01-01', '0.0164', '0.02296', NULL),
    ((SELECT id FROM cotisation WHERE code = 'CA_AE'), '2026-01-01', NULL,         '0.0163', '0.02282', 'EDSC, taux 2026'),
    ((SELECT id FROM cotisation WHERE code = 'QC_AE'), '2015-01-01', '2016-01-01', '0.0154', '0.02156', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_AE'), '2016-01-01', '2017-01-01', '0.0152', '0.02128', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_AE'), '2017-01-01', '2018-01-01', '0.0127', '0.01778', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_AE'), '2018-01-01', '2019-01-01', '0.0130', '0.01820', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_AE'), '2019-01-01', '2020-01-01', '0.0125', '0.01750', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_AE'), '2020-01-01', '2023-01-01', '0.0120', '0.01680', 'Gelé en 2021 et 2022'),
    ((SELECT id FROM cotisation WHERE code = 'QC_AE'), '2023-01-01', '2024-01-01', '0.0127', '0.01778', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_AE'), '2024-01-01', '2025-01-01', '0.0132', '0.01848', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_AE'), '2025-01-01', '2026-01-01', '0.0131', '0.01834', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_AE'), '2026-01-01', NULL,         '0.0130', '0.01820', 'EDSC, taux 2026 (Québec)'),
    ((SELECT id FROM cotisation WHERE code = 'QC_RRQ'), '2015-01-01', '2016-01-01', '0.0525',  '0.0525',  NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_RRQ'), '2016-01-01', '2017-01-01', '0.05325', '0.05325', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_RRQ'), '2017-01-01', '2019-01-01', '0.0540',  '0.0540',  NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_RRQ'), '2019-01-01', '2020-01-01', '0.0555',  '0.0555',  'Bonification : + 0,15'),
    ((SELECT id FROM cotisation WHERE code = 'QC_RRQ'), '2020-01-01', '2021-01-01', '0.0570',  '0.0570',  'Bonification : + 0,30'),
    ((SELECT id FROM cotisation WHERE code = 'QC_RRQ'), '2021-01-01', '2022-01-01', '0.0590',  '0.0590',  'Bonification : + 0,50'),
    ((SELECT id FROM cotisation WHERE code = 'QC_RRQ'), '2022-01-01', '2023-01-01', '0.0615',  '0.0615',  'Bonification : + 0,75'),
    ((SELECT id FROM cotisation WHERE code = 'QC_RRQ'), '2023-01-01', '2026-01-01', '0.0640',  '0.0640',  'Base 5,40 + supplémentaire 1,00'),
    ((SELECT id FROM cotisation WHERE code = 'QC_RRQ'), '2026-01-01', NULL,         '0.0630',  '0.0630',  'Base 5,30 + supplémentaire 1,00'),
    ((SELECT id FROM cotisation WHERE code = 'QC_RQAP'), '2015-01-01', '2016-01-01', '0.00559', '0.00782', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_RQAP'), '2016-01-01', '2019-01-01', '0.00548', '0.00767', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_RQAP'), '2019-01-01', '2020-01-01', '0.00526', '0.00736', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_RQAP'), '2020-01-01', '2026-01-01', '0.00494', '0.00692', NULL),
    ((SELECT id FROM cotisation WHERE code = 'QC_RQAP'), '2026-01-01', NULL,         '0.00430', '0.00602', 'Revenu Québec, taux 2026');
