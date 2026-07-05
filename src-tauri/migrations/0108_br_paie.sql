-- 0108 — Brésil : organismes, textes de loi, cotisations patronales 2025.
-- Périmètre : salarié secteur privé (CLT). Devise BRL. Données : 2025.
--
-- Seuls les taux lus depuis la base figurent ici (INSS patronal 20 %, FGTS 8 %).
-- INSS salarié (progressif plafonné) et IRRF (barème mensuel) sont calculés en
-- Rust (br_bulletin.rs), comme l'ISR mexicain — la catégorie « impôt » n'existe
-- pas dans le schéma des cotisations.

INSERT INTO organisme (code, libelle, url) VALUES
  ('BR_INSS', 'Instituto Nacional do Seguro Social', 'https://www.gov.br/inss'),
  ('BR_CAIXA', 'Caixa Econômica Federal (FGTS)', 'https://www.caixa.gov.br/fgts');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('BR_L8212', 'LOI', 'Lei 8.212/1991 — Custeio da Seguridade Social', 'Lei 8.212', '1991-07-24', '1991-07-25',
   'https://www.planalto.gov.br/ccivil_03/leis/l8212cons.htm',
   'INSS salarial progressif (7,5 / 9 / 12 / 14 %) plafonné au teto ; INSS patronal 20 % (art. 22).'),
  ('BR_L7713', 'LOI', 'Lei 7.713/1988 — Imposto de Renda', 'Lei 7.713', '1988-12-22', '1989-01-01',
   'https://www.planalto.gov.br/ccivil_03/leis/l7713.htm',
   'IRRF retido na fonte : barème mensuel progressif (tabela 2025) ; desconto simplificado 564,80 R$.'),
  ('BR_L8036', 'LOI', 'Lei 8.036/1990 — FGTS', 'Lei 8.036', '1990-05-11', '1990-05-14',
   'https://www.planalto.gov.br/ccivil_03/leis/l8036consol.htm',
   'FGTS : 8 % patronal déposés sur le compte lié du salarié (mobilisable au licenciement).'),
  ('BR_HISTOIRE', 'LOI', 'Brésil — histoire sociale', '—', '1923-01-24', '1923-01-24',
   'https://www.gov.br/inss',
   'Lei Eloy Chaves (1923) fonde la prévoyance ferroviaire. CLT (1943, Vargas). FGTS créé en 1966. Constitution 1988 : seguridade social universelle. Réforme des retraites 2019 (EC 103, âge minimum).');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('BR_INSS_PAT', 'INSS patronal',
   (SELECT id FROM organisme WHERE code = 'BR_INSS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 20 % employeur (RAT/terceiros non détaillés).'),
  ('BR_FGTS', 'FGTS — Fonds de garantie',
   (SELECT id FROM organisme WHERE code = 'BR_CAIXA'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 8 % employeur.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'BR_INSS_PAT'), '2025-01-01', NULL, '0', '0.20',
   (SELECT id FROM texte_loi WHERE code = 'BR_L8212'), 'INSS patronal 2025.'),
  ((SELECT id FROM cotisation WHERE code = 'BR_FGTS'),     '2025-01-01', NULL, '0', '0.08',
   (SELECT id FROM texte_loi WHERE code = 'BR_L8036'), 'FGTS 2025.');
