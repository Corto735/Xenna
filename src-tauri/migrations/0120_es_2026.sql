-- ============================================================
-- ESPAGNE : bases 2026, MEI corrigé 2023-2026
-- ============================================================
-- Orden PJC/297/2026 (BOE-A-2026-7296) : base maximale 5 101,20 €/mois,
-- base minimale des groupes 4-7 1 424,40 € ; MEI 0,90 % (0,75 % employeur,
-- 0,15 % salarié). Les taux MEI des migrations 0033 étaient faux :
-- 2023 0,60 % (0,50/0,10), 2024 0,70 % (0,58/0,12), 2025 0,80 % (0,67/0,13).
-- Les bases minimales en base portaient le SMI, non la base des groupes 4-7
-- (SMI majoré d'un sixième) : corrigé dans es_cotizaciones.rs, qui fait foi.
UPDATE plafond_reference SET date_fin = '2026-01-01'
WHERE code = 'ES_BASE_MAX' AND date_debut = '2025-01-01';
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('ES_BASE_MAX', '2026-01-01', NULL, '5101.20', 'MENSUEL');

UPDATE plafond_reference SET date_fin = '2026-01-01'
WHERE code = 'ES_BASE_MIN' AND date_debut = '2025-01-01';
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('ES_BASE_MIN', '2026-01-01', NULL, '1424.40', 'MENSUEL');

DELETE FROM cotisation_taux WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'ES_MEI');
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'ES_MEI'), '2023-01-01', '2024-01-01', '0.0010', '0.0050', 'MEI 2023 : 0,60 %.'),
  ((SELECT id FROM cotisation WHERE code = 'ES_MEI'), '2024-01-01', '2025-01-01', '0.0012', '0.0058', 'MEI 2024 : 0,70 %.'),
  ((SELECT id FROM cotisation WHERE code = 'ES_MEI'), '2025-01-01', '2026-01-01', '0.0013', '0.0067', 'MEI 2025 : 0,80 %.'),
  ((SELECT id FROM cotisation WHERE code = 'ES_MEI'), '2026-01-01', NULL,         '0.0015', '0.0075', 'MEI 2026 : 0,90 % — Orden PJC/297/2026.');
