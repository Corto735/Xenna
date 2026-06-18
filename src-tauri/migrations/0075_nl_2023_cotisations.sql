-- 0075 — Pays-Bas : cotisations patronales 2023 (backfill historique)
-- Taux 2023 (date_fin = 2024-01-01). Source : Belastingdienst/UWV — premiepercentages 2023.
-- Maximumpremieloon 2023 : 66 956 €/an (appliqué dans nl_loonheffing.rs).

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'NL_ZVW'),       '2023-01-01', '2024-01-01', '0', '0.0668',
   (SELECT id FROM texte_loi WHERE code = 'NL_ZVW'),  'Werkgeversheffing Zvw 2023 : 6,68 %.'),
  ((SELECT id FROM cotisation WHERE code = 'NL_AWF'),       '2023-01-01', '2024-01-01', '0', '0.0264',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'AWf-laag 2023 : 2,64 % (CDI écrit).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_AOF'),       '2023-01-01', '2024-01-01', '0', '0.0582',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Aof-laag 2023 : 5,82 % (petit employeur).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_WHK'),       '2023-01-01', '2024-01-01', '0', '0.0118',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Whk 2023 : moyenne ~1,18 % (différenciée par employeur).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_OPSLAG_KO'), '2023-01-01', '2024-01-01', '0', '0.0050',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Opslag kinderopvang 2023 : 0,50 %.');
