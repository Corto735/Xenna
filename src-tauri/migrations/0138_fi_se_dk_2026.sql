-- ============================================================
-- FINLANDE — chômage employeur 2026 : 0,31 % (masse salariale ≤ 2 509 500 €),
-- et non 0,20 % (Veronmaksajat, työttömyysvakuutusmaksu 2026).
-- ============================================================
UPDATE cotisation_taux SET taux_patronal = '0.0031',
       notes = 'Chômage 2026 : 0,89 % sal. ; employeur 0,31 % (masse ≤ 2 509 500 €), 1,23 % au-delà'
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'FI_TYOTTOMYYS') AND date_debut = '2026-01-01';

-- TyEL 2026 : 24,4 % au total, dont 7,30 % salarié → employeur 17,10 % en
-- moyenne (Työeläkevakuuttajat TELA, taux 2026), et non 17,55 %.
UPDATE cotisation_taux SET taux_patronal = '0.1710',
       notes = 'TyEL 2026 : 24,4 % au total ; 7,30 % sal. (17-52 et 63-67 ans), employeur 17,10 % en moyenne'
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'FI_TYEL') AND date_debut = '2026-01-01';
