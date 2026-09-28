-- ============================================================
-- ÉTATS-UNIS : California SDI 1,3 % en 2026
-- ============================================================
-- EDD (« Contribution Rates, Withholding Schedules ») : taux de retenue SDI
-- porté de 1,2 % à 1,3 % au 01/01/2026, toujours sans plafond de salaire.
-- Barème fédéral 2026 (Rev. Proc. 2025-32) et plafond Social Security 2026
-- (184 500 $, SSA) : codés dans us_impot.rs et us_cotisations.rs.
UPDATE cotisation_taux SET date_fin = '2025-12-31'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'US_CA_SDI')
  AND date_debut = '2025-01-01';

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'US_CA_SDI'), '2026-01-01', NULL, '0.013', '0',
   'California SDI 2026 : 1,3 % salarié, sans plafond — EDD.');
