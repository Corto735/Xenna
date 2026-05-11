-- 0033 — Espagne : cotisations et taux historiques 2015-2026
-- Périmètre : contrato indefinido, régime général, secteur privé.
-- Assiette : salaire réel borné entre ES_BASE_MIN et ES_BASE_MAX.
-- ES_CC/DESEMPLEO/FOGASA/FP : stables depuis 2015, une seule entrée date_fin NULL.
-- ES_MEI : en vigueur depuis le 01/01/2023, taux progressif annuel.

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('ES_CC', 'Contingencias Comunes — enfermedad, maternidad, jubilación, invalidez, muerte',
   (SELECT id FROM organisme WHERE code = 'TGSS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   'Cotisation principale SS espagnole. Couvre maladie commune, maternité, retraite, invalidité, décès. Assiette : salaire borné ES_BASE_MIN/MAX. LGSS (RDL 8/2015) art. 143-144. Taux : 4,70 % sal + 23,60 % pat. Stable 2015-2026.'),

  ('ES_DESEMPLEO', 'Desempleo — assurance chômage (contrato indefinido)',
   (SELECT id FROM organisme WHERE code = 'SEPE'), 'CHOMAGE', 1, 1, 'BRUT_PLAFONNÉ',
   'Assurance chômage régime général, contrato indefinido. LGSS art. 270. 1,55 % sal + 5,50 % pat = 7,05 % total. Stable 2015-2026.'),

  ('ES_FOGASA', 'FOGASA — Fondo de Garantía Salarial',
   (SELECT id FROM organisme WHERE code = 'FOGASA_ES'), 'AUTRES', 1, 1, 'BRUT_PLAFONNÉ',
   'Garantie des salaires impayés en cas d''insolvabilité. Exclusivement patronal. Art. 33 ET (RDL 2/2015) + LGSS. 0,20 % pat. Stable 2015-2026.'),

  ('ES_FP', 'Formación Profesional — formation professionnelle continue',
   (SELECT id FROM organisme WHERE code = 'FUNDAE'), 'FORMATION', 1, 1, 'BRUT_PLAFONNÉ',
   'Finance la formation continue (créditos FUNDAE). LGSS art. 7 + DA 19a. 0,10 % sal + 0,60 % pat = 0,70 % total. Stable 2015-2026.'),

  ('ES_MEI', 'MEI — Mecanismo de Equidad Intergeneracional',
   (SELECT id FROM organisme WHERE code = 'TGSS'), 'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ',
   'Cotisation additionnelle alimentant le Fondo de Reserva de la SS (Ley 21/2021 art. 2). Taux progressif jusqu''en 2032. En vigueur depuis le 01/01/2023. Même assiette que CC.');

-- Taux CC : stable depuis 2015
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'ES_CC'), '2015-01-01', NULL, '0.0470', '0.2360',
   '4,70 % sal + 23,60 % pat = 28,30 % total. Source : Ordenes ESS/86/2015 … TRM/42/2025. LGSS art. 143.');

-- Taux Desempleo : stable depuis 2015
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'ES_DESEMPLEO'), '2015-01-01', NULL, '0.0155', '0.0550',
   'Contrato indefinido. 1,55 % sal + 5,50 % pat. Source : LGSS art. 270 + Ordenes annuelles.');

-- Taux FOGASA : stable depuis 2015
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'ES_FOGASA'), '2015-01-01', NULL, '0.0000', '0.0020',
   '0 % sal + 0,20 % pat. Stable depuis 2015. Source : LGSS art. 33 ET.');

-- Taux FP : stable depuis 2015
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'ES_FP'), '2015-01-01', NULL, '0.0010', '0.0060',
   '0,10 % sal + 0,60 % pat = 0,70 % total. Source : LGSS art. 7.');

-- Taux MEI : 3 périodes distinctes (entrée en vigueur 2023)
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'ES_MEI'), '2023-01-01', '2024-01-01', '0.0010', '0.0040',
   'MEI 2023 : 0,50 % total (0,10 sal + 0,40 pat). Source : Ley 21/2021 + Orden CEM/74/2023.'),
  ((SELECT id FROM cotisation WHERE code = 'ES_MEI'), '2024-01-01', '2025-01-01', '0.0012', '0.0046',
   'MEI 2024 : 0,58 % total (0,12 sal + 0,46 pat). Source : Orden TRM/43/2024.'),
  ((SELECT id FROM cotisation WHERE code = 'ES_MEI'), '2025-01-01', NULL, '0.0013', '0.0053',
   'MEI 2025 : 0,66 % total (0,13 sal + 0,53 pat). Source : Orden TRM/42/2025. Progression prévue jusqu''en 2032.');
