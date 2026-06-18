-- 0080 — Pologne : backfill cotisations ZUS 2015-2024 (taux salariaux stables)
-- Les taux ZUS salariaux (emerytalne 9,76 %, rentowe 1,5 %, chorobowe 2,45 %) et la
-- składka zdrowotna 9 % sont stables depuis 2012. On étend la couverture en amont avec
-- les mêmes taux (les lignes 2025→NULL existent déjà en 0067). Le PIT par année est géré
-- dans pl_bulletin.rs (structure stable depuis 2023). Source : Ustawa o systemie ub. społ.

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'PL_EMERYTALNE'), '2015-01-01', '2025-01-01', '0.0976', '0.0976',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'Emerytalne 2015-2024 : 9,76 % chacun (stable).'),
  ((SELECT id FROM cotisation WHERE code = 'PL_RENTOWE'),    '2015-01-01', '2025-01-01', '0.0150', '0.0650',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'Rentowe 2015-2024 : 1,5 % sal / 6,5 % pat (stable).'),
  ((SELECT id FROM cotisation WHERE code = 'PL_CHOROBOWE'),  '2015-01-01', '2025-01-01', '0.0245', '0',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'Chorobowe 2015-2024 : 2,45 % sal (stable).'),
  ((SELECT id FROM cotisation WHERE code = 'PL_WYPADKOWE'),  '2015-01-01', '2025-01-01', '0', '0.0167',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'Wypadkowe 2015-2024 : 1,67 % pat (standard ; valeur historique moyenne).'),
  ((SELECT id FROM cotisation WHERE code = 'PL_FP'),         '2015-01-01', '2025-01-01', '0', '0.0245',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'Fundusz Pracy 2015-2024 : 2,45 % pat (stable).'),
  ((SELECT id FROM cotisation WHERE code = 'PL_FGSP'),       '2015-01-01', '2025-01-01', '0', '0.0010',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'FGŚP 2015-2024 : 0,10 % pat (stable).');
