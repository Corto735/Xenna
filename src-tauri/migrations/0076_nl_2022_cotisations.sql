-- 0076 — Pays-Bas : cotisations patronales 2022 (backfill historique)
-- Taux 2022 (date_fin = 2023-01-01). Source : Belastingdienst/UWV — premiepercentages 2022.
-- Maximumpremieloon 2022 : 59 706 €/an (appliqué dans nl_loonheffing.rs).

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'NL_ZVW'),       '2022-01-01', '2023-01-01', '0', '0.0675',
   (SELECT id FROM texte_loi WHERE code = 'NL_ZVW'),  'Werkgeversheffing Zvw 2022 : 6,75 %.'),
  ((SELECT id FROM cotisation WHERE code = 'NL_AWF'),       '2022-01-01', '2023-01-01', '0', '0.0270',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'AWf-laag 2022 : 2,70 % (CDI écrit).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_AOF'),       '2022-01-01', '2023-01-01', '0', '0.0549',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Aof-laag 2022 : 5,49 % (petit employeur).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_WHK'),       '2022-01-01', '2023-01-01', '0', '0.0152',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Whk 2022 : moyenne ~1,52 % (différenciée par employeur).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_OPSLAG_KO'), '2022-01-01', '2023-01-01', '0', '0.0050',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Opslag kinderopvang 2022 : 0,50 %.');
