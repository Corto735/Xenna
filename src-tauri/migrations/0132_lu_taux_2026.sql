-- ============================================================
-- LUXEMBOURG — taux 2026
--   Pension : 8,00 % → 8,50 % salarié et employeur au 01/01/2026 (loi du
--     18/12/2025 portant réforme des pensions).
--   Accidents : taux unique 0,65 % pour 2026 (avant facteur bonus-malus).
--   Mutualité des employeurs : 1,40 % ne correspond à aucune classe. Classes
--     2026 : 0,23 / 0,95 / 1,56 / 2,66 % selon l'absentéisme financier ;
--     défaut retenu : classe 2 (0,95 %).
-- Source : FEDIL, paramètres sociaux au 01/01/2026 et au 01/06/2026.
-- ============================================================
UPDATE cotisation_taux SET taux_salarial = '0.0850', taux_patronal = '0.0850',
       notes = 'Réforme des pensions (loi du 18/12/2025) : 8,50 % par partie'
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'LU_AP') AND date_debut = '2026-01-01';
UPDATE cotisation_taux SET taux_patronal = '0.0065',
       notes = 'Taux unique 2026 avant facteur bonus-malus (0,85 à 1,5)'
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'LU_AA') AND date_debut = '2026-01-01';
UPDATE cotisation_taux SET taux_patronal = '0.0095',
       notes = 'Classe 2 (absentéisme financier 0,65 % à 1,60 %) ; classes 0,23 / 0,95 / 1,56 / 2,66 %'
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'LU_ME') AND date_debut = '2026-01-01';
