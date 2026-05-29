-- 0054 — Japon : plafonds de référence (標準報酬月額上限)
--
-- JP_PLAFOND_KENPO  : plafond santé + soins (健康保険 / 介護保険) — grade 50
-- JP_PLAFOND_KOSEI  : plafond retraite (厚生年金) — grade 32
--
-- Valeurs mensuelles en JPY.
-- Source : MHLW, 厚生労働省告示 annuels.

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  -- Plafond assurance santé + soins longue durée (健康保険 / 介護保険)
  ('JP_PLAFOND_KENPO', '2024-04-01', NULL, '1390000', 'MENSUEL'),

  -- Plafond assurance retraite salariés (厚生年金保険)
  ('JP_PLAFOND_KOSEI', '2024-04-01', NULL, '650000', 'MENSUEL');
