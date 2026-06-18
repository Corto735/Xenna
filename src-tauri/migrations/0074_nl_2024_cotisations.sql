-- 0074 — Pays-Bas : cotisations patronales 2024 (backfill historique)
-- Taux 2024 (date_fin = 2025-01-01). Source : Belastingdienst/UWV — premiepercentages 2024.
-- Maximumpremieloon 2024 : 71 628 €/an (appliqué dans nl_loonheffing.rs).

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'NL_ZVW'),       '2024-01-01', '2025-01-01', '0', '0.0657',
   (SELECT id FROM texte_loi WHERE code = 'NL_ZVW'),  'Werkgeversheffing Zvw 2024 : 6,57 %.'),
  ((SELECT id FROM cotisation WHERE code = 'NL_AWF'),       '2024-01-01', '2025-01-01', '0', '0.0264',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'AWf-laag 2024 : 2,64 % (CDI écrit).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_AOF'),       '2024-01-01', '2025-01-01', '0', '0.0618',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Aof-laag 2024 : 6,18 % (petit employeur).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_WHK'),       '2024-01-01', '2025-01-01', '0', '0.0122',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Whk 2024 : moyenne ~1,22 % (différenciée par employeur).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_OPSLAG_KO'), '2024-01-01', '2025-01-01', '0', '0.0050',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Opslag kinderopvang 2024 : 0,50 %.');
