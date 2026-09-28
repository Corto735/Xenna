-- ============================================================
-- NOUVELLE-ZÉLANDE : KiwiSaver employeur 3,5 % puis 4 %
-- ============================================================
-- Taux par défaut (salarié et employeur) : 3 % → 3,5 % au 01/04/2026, puis 4 %
-- au 01/04/2028 (IRD, « KiwiSaver changes »). L'ACC earner's levy 2026-27
-- (1,75 %, plafond 156 641 $) est codé dans nz_bulletin.rs.
UPDATE cotisation_taux SET date_fin = '2026-03-31'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'NZ_KIWISAVER_EMP')
  AND date_debut = '2025-04-01';

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'NZ_KIWISAVER_EMP'), '2026-04-01', '2028-03-31', '0', '0.0350',
   (SELECT id FROM texte_loi WHERE code = 'NZ_KIWISAVER_2006'), 'KiwiSaver employeur 3,5 % (défaut) dès le 01/04/2026 — IRD.'),
  ((SELECT id FROM cotisation WHERE code = 'NZ_KIWISAVER_EMP'), '2028-04-01', NULL, '0', '0.0400',
   (SELECT id FROM texte_loi WHERE code = 'NZ_KIWISAVER_2006'), 'KiwiSaver employeur 4 % (défaut) dès le 01/04/2028 — IRD.');
