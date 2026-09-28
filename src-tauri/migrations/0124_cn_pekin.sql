-- ============================================================
-- CHINE (Pékin) : bases 2025-2026, fonds des grosses dépenses médicales
-- ============================================================
-- Assiette (养老、失业、工伤、医疗含生育) : 7 162-35 811 ¥ de juillet 2025 à juin
-- 2026, 7 270-36 348 ¥ dès juillet 2026 (avis de Pékin du 18/09/2025 et du
-- 21/08/2026) — bornes codées dans cn_cotisations.rs, reportées ici.
-- Assurance maladie employeur : 8,8 % médical + maternité, plus 1 % de fonds
-- mutuel des grosses dépenses (大额医疗互助资金), soit 9,8 % ; la maternité (0,8 %)
-- ayant sa propre ligne, CN_YILIAO porte 9 % (au lieu de 8 %).
UPDATE plafond_reference SET date_fin = '2025-06-30' WHERE code = 'CN_BASE_MIN' AND date_debut = '2024-01-01';
UPDATE plafond_reference SET date_fin = '2025-06-30' WHERE code = 'CN_BASE_MAX' AND date_debut = '2024-01-01';
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CN_BASE_MIN', '2025-07-01', '2026-06-30', '7162',  'MENSUEL'),
  ('CN_BASE_MAX', '2025-07-01', '2026-06-30', '35811', 'MENSUEL'),
  ('CN_BASE_MIN', '2026-07-01', NULL,         '7270',  'MENSUEL'),
  ('CN_BASE_MAX', '2026-07-01', NULL,         '36348', 'MENSUEL');

UPDATE cotisation_taux SET taux_patronal = '0.0900',
  notes = 'Pékin : sal 2 % ; pat 8 % médical + 1 % fonds des grosses dépenses (maternité 0,8 % à part).'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'CN_YILIAO') AND date_debut = '2024-01-01';
