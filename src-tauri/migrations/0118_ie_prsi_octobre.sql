-- ============================================================
-- IRLANDE : PRSI Class A, hausses du 1er octobre 2025 et 2026
-- ============================================================
-- Guide SW14 « PRSI Contribution Rates and User Guide », janvier 2026
-- (Department of Social Protection) : taux salarié / employeur plein
--   jusqu'au 30/09/2025 : 4,1 % / 11,15 %
--   01/10/2025 - 30/09/2026 : 4,2 % / 11,25 %
--   dès le 01/10/2026 : 4,35 % / 11,40 %
-- (La migration 0103 plaçait la hausse à 4,2 % au 01/01/2026 et laissait
-- l'employeur à 11,15 %.) Taux employeur réduit et crédit PRSI : ie_bulletin.rs.
DELETE FROM cotisation_taux WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'IE_PRSI');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'IE_PRSI'), '2025-01-01', '2025-09-30', '0.041', '0.1115',
   (SELECT id FROM texte_loi WHERE code = 'IE_PRSI_L'), 'PRSI Class A : 4,1 % sal / 11,15 % pat (8,90 % jusqu''à 527 €/semaine).'),
  ((SELECT id FROM cotisation WHERE code = 'IE_PRSI'), '2025-10-01', '2026-09-30', '0.042', '0.1125',
   (SELECT id FROM texte_loi WHERE code = 'IE_PRSI_L'), 'PRSI Class A : 4,2 % sal / 11,25 % pat (9,00 % sous le seuil) — SW14 janv. 2026.'),
  ((SELECT id FROM cotisation WHERE code = 'IE_PRSI'), '2026-10-01', NULL, '0.0435', '0.1140',
   (SELECT id FROM texte_loi WHERE code = 'IE_PRSI_L'), 'PRSI Class A : 4,35 % sal / 11,40 % pat (9,15 % sous le seuil) — SW14 janv. 2026.');
