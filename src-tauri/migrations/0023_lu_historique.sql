-- ============================================================
-- LUXEMBOURG — Taux historiques 2015-2025 et plafonds SSM
--
-- CONTEXTE : la migration 0013 ne contenait que des taux 2026+.
-- Cette migration couvre 2015-2025 pour les simulations historiques.
--
-- Les taux AP/AM/AD/AA/ME sont stables sur la période 2015-2026.
-- Le plafond cotisable (5 × SSM non-qualifié) varie chaque année
-- via les mécanismes d'indexation et revalorisation luxembourgeois.
--
-- Indexation automatique (échelle mobile) : déclenchée quand
-- l'indice des prix dépasse ±2,5 % (tranche). Des ajustements
-- discrétionnaires peuvent s'y ajouter (accords tripartites).
-- ============================================================

-- ── AP / AM / AD / AA / ME : stables sur toute la période ───

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES

  ((SELECT id FROM cotisation WHERE code='LU_AP'),
   '2015-01-01', '2026-01-01', '0.0800', '0.0800',
   'Taux stable. CSS LU Livre II. L''État verse en sus 1/3 additionnel du budget.'),

  ((SELECT id FROM cotisation WHERE code='LU_AM'),
   '2015-01-01', '2026-01-01', '0.0305', '0.0305',
   'Soins de santé 2,80 % + indemnités pécuniaires 0,25 % = 3,05 % chacun. Stable.'),

  ((SELECT id FROM cotisation WHERE code='LU_AD'),
   '2015-01-01', '2026-01-01', '0.0140', '0.0000',
   'Salarié uniquement. Loi du 19/06/1998. Stable.'),

  ((SELECT id FROM cotisation WHERE code='LU_AA'),
   '2015-01-01', '2026-01-01', '0.0000', '0.0075',
   'Taux indicatif AAA secteur tertiaire. Varie par classification des risques.'),

  ((SELECT id FROM cotisation WHERE code='LU_ME'),
   '2015-01-01', '2026-01-01', '0.0000', '0.0140',
   'Taux indicatif moyen national CCSS. Varie selon sinistralité de l''entreprise.');

-- ============================================================
-- Plafond cotisable mensuel luxembourgeois (5 × SSM non-qualifié)
-- Source : CCSS, SSM publié au Journal officiel luxembourgeois.
-- Valeurs approximatives — à vérifier contre les publications CCSS.
-- Le SSM est indexé automatiquement et peut être revalorisé.
-- ============================================================

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('LU_PLAFOND_5SSM', '2015-01-01', '2017-01-01', '9615.00',  'MENSUEL'),  -- ~5 x 1 923 EUR
  ('LU_PLAFOND_5SSM', '2017-01-01', '2019-01-01', '9995.00',  'MENSUEL'),  -- ~5 x 1 999 EUR (indexation jan. 2017)
  ('LU_PLAFOND_5SSM', '2019-01-01', '2020-01-01', '10245.00', 'MENSUEL'),  -- ~5 x 2 049 EUR (indexation jan. 2019)
  ('LU_PLAFOND_5SSM', '2020-01-01', '2021-01-01', '10710.00', 'MENSUEL'),  -- ~5 x 2 142 EUR
  ('LU_PLAFOND_5SSM', '2021-01-01', '2022-01-01', '11010.00', 'MENSUEL'),  -- ~5 x 2 202 EUR
  ('LU_PLAFOND_5SSM', '2022-01-01', '2023-01-01', '11985.00', 'MENSUEL'),  -- ~5 x 2 397 EUR
  ('LU_PLAFOND_5SSM', '2023-01-01', '2024-01-01', '12855.00', 'MENSUEL'),  -- ~5 x 2 571 EUR
  ('LU_PLAFOND_5SSM', '2024-01-01', '2025-01-01', '13185.00', 'MENSUEL'),  -- ~5 x 2 637 EUR
  ('LU_PLAFOND_5SSM', '2025-01-01', NULL,          '13518.80', 'MENSUEL'); -- 5 x 2 703,76 EUR (valeur CCSS 2025)
