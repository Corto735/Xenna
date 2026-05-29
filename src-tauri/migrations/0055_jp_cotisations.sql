-- 0055 — Japon : cotisations sociales 2024
-- Régime : 協会けんぽ Tokyo, secteur privé général
-- Source : Kyokai Kenpo Tokyo 2024, MHLW 2024, Hello Work 2024

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('JP_KENPO', '健康保険 — Assurance maladie (Tokyo Kyokai Kenpo)',
   (SELECT id FROM organisme WHERE code = 'JP_KENPO'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   'Tokyo 2024 : 9,98 % total (4,99 % sal + 4,99 % pat). Assiette : min(salaire brut, ¥1 390 000/mois). 健康保険法.'),

  ('JP_KAIGO', '介護保険 — Assurance soins longue durée (≥ 40 ans)',
   (SELECT id FROM organisme WHERE code = 'JP_KENPO'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   'Taux national 2024 : 1,60 % total (0,80 % sal + 0,80 % pat). Applicable aux salariés âgés de 40 à 64 ans. Même plafond que 健康保険 (¥1 390 000/mois). 介護保険法.'),

  ('JP_KOSEI', '厚生年金保険 — Assurance retraite salariés',
   (SELECT id FROM organisme WHERE code = 'JP_MHLW'), 'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ',
   'Taux unique national depuis oct. 2017 : 18,30 % (9,15 % sal + 9,15 % pat). Assiette : min(salaire brut, ¥650 000/mois). 厚生年金保険法.'),

  ('JP_KOYO', '雇用保険 — Assurance emploi (chômage)',
   (SELECT id FROM organisme WHERE code = 'JP_HELLOWORK'), 'CHOMAGE', 1, 1, 'BRUT_TOTAL',
   'Taux 2024 (一般の事業) : sal 0,60 % + pat 0,95 % = 1,55 % total. Assiette : salaire brut sans plafond. 雇用保険法.'),

  ('JP_ROUSAI', '労災保険 — Assurance accidents du travail',
   (SELECT id FROM organisme WHERE code = 'JP_HELLOWORK'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '100 % patronale. Taux bureau/services généraux 2024 : 0,30 %. Assiette : salaire brut total. 労働者災害補償保険法.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'JP_KENPO'), '2024-03-01', NULL, '0.0499', '0.0499',
   'Tokyo Kyokai Kenpo FY2024 : 4,99 % sal + 4,99 % pat = 9,98 % total.'),

  ((SELECT id FROM cotisation WHERE code = 'JP_KAIGO'), '2024-03-01', NULL, '0.0080', '0.0080',
   'Taux national 介護保険 FY2024 : 1,60 % total, soit 0,80 % chacun.'),

  ((SELECT id FROM cotisation WHERE code = 'JP_KOSEI'), '2017-09-01', NULL, '0.0915', '0.0915',
   'Taux unifié depuis oct. 2017 : 18,30 % total (9,15 % chacun). Stable depuis.'),

  ((SELECT id FROM cotisation WHERE code = 'JP_KOYO'), '2024-04-01', NULL, '0.0060', '0.0095',
   '一般の事業 FY2024 : sal 0,60 %, pat 0,95 %.'),

  ((SELECT id FROM cotisation WHERE code = 'JP_ROUSAI'), '2024-04-01', NULL, '0.0000', '0.0030',
   'Bureaux/services généraux : 0,30 % patronal (料率表 2024).');
