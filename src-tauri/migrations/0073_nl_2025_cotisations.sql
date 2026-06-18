-- 0073 — Pays-Bas : cotisations patronales 2025 (backfill historique)
-- Mêmes cotisations que 2026, taux 2025 (date_fin = 2026-01-01 pour ne pas chevaucher).
-- Source : Belastingdienst/UWV — premiepercentages 2025. Maximumpremieloon 2025 : 75 864 €/an
-- (le plafond annuel est appliqué dans nl_loonheffing.rs par année).

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'NL_ZVW'),       '2025-01-01', '2026-01-01', '0', '0.0651',
   (SELECT id FROM texte_loi WHERE code = 'NL_ZVW'),  'Werkgeversheffing Zvw 2025 : 6,51 %.'),
  ((SELECT id FROM cotisation WHERE code = 'NL_AWF'),       '2025-01-01', '2026-01-01', '0', '0.0274',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'AWf-laag 2025 : 2,74 % (CDI écrit).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_AOF'),       '2025-01-01', '2026-01-01', '0', '0.0628',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Aof-laag 2025 : 6,28 % (petit employeur).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_WHK'),       '2025-01-01', '2026-01-01', '0', '0.0145',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Whk 2025 : moyenne ~1,45 % (différenciée par employeur).'),
  ((SELECT id FROM cotisation WHERE code = 'NL_OPSLAG_KO'), '2025-01-01', '2026-01-01', '0', '0.0050',
   (SELECT id FROM texte_loi WHERE code = 'NL_WFSV'), 'Opslag kinderopvang 2025 : 0,50 %.');
