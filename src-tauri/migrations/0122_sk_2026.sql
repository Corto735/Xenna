-- ============================================================
-- SLOVAQUIE : santé salariale 5 % en 2026-2027
-- ============================================================
-- Consolidation budgétaire : taux salarié de l'assurance maladie porté de 4 à
-- 5 % du 01/01/2026, à titre temporaire jusqu'en 2027 ; employeur inchangé à
-- 11 % (VšZP, « Zmeny od 01. 01. 2026 »). Plafond social 2026 (16 764 €) :
-- sk_bulletin.rs.
UPDATE cotisation_taux SET date_fin = '2025-12-31'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'SK_ZDRAVOTNE')
  AND date_debut = '2025-01-01';
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'SK_ZDRAVOTNE'), '2026-01-01', NULL, '0.05', '0.11',
   'Santé 2026 : 5 % sal (hausse temporaire 2026-2027) / 11 % pat — VšZP.');
