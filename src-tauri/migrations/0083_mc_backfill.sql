-- 0083 — Monaco : backfill 2020-2024 (part salariale stable)
-- CAR (6,85 % sal) et chômage (2,4 % sal) sont stables depuis ~2019 ; Monaco ne prélève pas
-- d'IR → le net est constant sur la période. On étend la couverture (lignes 2025→NULL en 0070).
-- La part CCSS patronale (13,40 %) est reconduite (n'affecte pas le net ; valeur récente).
-- Source : Caisses Sociales de Monaco / UNEDIC.

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'MC_CAR'),  '2020-01-01', '2025-01-01', '0.0685', '0.0850',
   (SELECT id FROM texte_loi WHERE code = 'MC_CAR_LOI'),  'CAR 2020-2024 : 6,85 % sal / 8,50 % pat (stable).'),
  ((SELECT id FROM cotisation WHERE code = 'MC_CCSS'), '2020-01-01', '2025-01-01', '0',      '0.1340',
   (SELECT id FROM texte_loi WHERE code = 'MC_CCSS_LOI'), 'CCSS 2020-2024 : ≈ 13,40 % pat (valeur récente reconduite ; n''affecte pas le net).'),
  ((SELECT id FROM cotisation WHERE code = 'MC_CHOM'), '2020-01-01', '2025-01-01', '0.024',  '0.040',
   (SELECT id FROM texte_loi WHERE code = 'MC_CHOM_LOI'), 'Chômage 2020-2024 : 2,4 % sal / 4,0 % pat (stable).');
