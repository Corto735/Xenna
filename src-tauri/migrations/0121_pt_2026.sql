-- ============================================================
-- PORTUGAL : salaire minimum 2026
-- ============================================================
-- Retribuição mínima mensal garantida : 920 € au 01/01/2026 (Decreto-Lei
-- n.º 139/2025, du 29 décembre). Barème IRS 2026 (Lei 73-A/2025) et 2025
-- rétroactif (Lei 55-A/2025) : codés dans pt_irs.rs.
UPDATE plafond_reference SET date_fin = '2026-01-01'
WHERE code = 'PT_SMN' AND date_debut = '2025-01-01';
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('PT_SMN', '2026-01-01', NULL, '920.00', 'MENSUEL');
