-- 0058 — Chine : cotisations 五险一金 + IIT (Pékin 2024)
--
-- 五险一金 = 五险 (cinq assurances) + 一金 (fonds logement)
--   养老保险 : retraite          sal 8 %  / pat 16 %
--   医疗保险 : maladie           sal 2 %  / pat  8 %
--   失业保险 : chômage           sal 0,5 %/ pat  0,5 %
--   工伤保险 : accidents travail sal 0 %  / pat  0,4 % (Pékin général)
--   生育保险 : maternité         sal 0 %  / pat  0,8 % (Pékin)
--   住房公积金: fonds logement   sal 12 % / pat 12 %
--
-- Source : 北京市社会保险 + 住房公积金 taux 2024.

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('CN_YANGLAO', '养老保险 — Assurance retraite',
   (SELECT id FROM organisme WHERE code = 'CN_SS_BEIJING'), 'SECURITE_SOCIALE', 1, 1, 'SPECIFIQUE',
   'Sal 8 % + pat 16 % sur base clampée [¥6 891 – ¥35 283/mois]. 社会保险法 art. 12.'),

  ('CN_YILIAO', '医疗保险 — Assurance maladie',
   (SELECT id FROM organisme WHERE code = 'CN_SS_BEIJING'), 'SECURITE_SOCIALE', 1, 1, 'SPECIFIQUE',
   'Sal 2 % + pat 8 % sur base clampée. 社会保险法 art. 23.'),

  ('CN_SHIYE', '失业保险 — Assurance chômage',
   (SELECT id FROM organisme WHERE code = 'CN_SS_BEIJING'), 'CHOMAGE', 1, 1, 'SPECIFIQUE',
   'Sal 0,5 % + pat 0,5 % sur base clampée. 社会保险法 art. 44.'),

  ('CN_GONGSHANG', '工伤保险 — Assurance accidents du travail',
   (SELECT id FROM organisme WHERE code = 'CN_SS_BEIJING'), 'SECURITE_SOCIALE', 1, 1, 'SPECIFIQUE',
   '100 % patronale : 0,4 % (Pékin, secteur général). Taux variable par secteur. 社会保险法 art. 33.'),

  ('CN_SHENGYU', '生育保险 — Assurance maternité',
   (SELECT id FROM organisme WHERE code = 'CN_SS_BEIJING'), 'SECURITE_SOCIALE', 1, 1, 'SPECIFIQUE',
   '100 % patronale : 0,8 % (Pékin 2024). 社会保险法 art. 53.'),

  ('CN_GONGJIJIN', '住房公积金 — Fonds de logement obligatoire',
   (SELECT id FROM organisme WHERE code = 'CN_CPF_BEIJING'), 'AUTRES', 1, 1, 'SPECIFIQUE',
   'Sal 12 % + pat 12 % sur base clampée. Épargne logement mandatory. 住房公积金管理条例 (1999, rév. 2019).');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'CN_YANGLAO'), '2024-01-01', NULL, '0.0800', '0.1600',
   'Pékin 2024 : sal 8 %, pat 16 %. Base [¥6 891 – ¥35 283]. 社会保险法 + 北京公告 2024.'),

  ((SELECT id FROM cotisation WHERE code = 'CN_YILIAO'), '2024-01-01', NULL, '0.0200', '0.0800',
   'Pékin 2024 : sal 2 %, pat 8 %. 社会保险法 art. 23.'),

  ((SELECT id FROM cotisation WHERE code = 'CN_SHIYE'), '2024-01-01', NULL, '0.0050', '0.0050',
   'Pékin 2024 : sal 0,5 %, pat 0,5 %.'),

  ((SELECT id FROM cotisation WHERE code = 'CN_GONGSHANG'), '2024-01-01', NULL, '0.0000', '0.0040',
   'Pékin général 2024 : 0,4 % patronal.'),

  ((SELECT id FROM cotisation WHERE code = 'CN_SHENGYU'), '2024-01-01', NULL, '0.0000', '0.0080',
   'Pékin 2024 : 0,8 % patronal.'),

  ((SELECT id FROM cotisation WHERE code = 'CN_GONGJIJIN'), '2024-01-01', NULL, '0.1200', '0.1200',
   'Pékin 2024 : 12 % sal + 12 % pat sur base [¥6 891 – ¥35 283].');
