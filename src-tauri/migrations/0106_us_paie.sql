-- 0106 — États-Unis : organismes, textes de loi, cotisations FICA 2025
-- Périmètre : salarié secteur privé. Devise USD. Données : 2025 (2026 reconduit).
--
-- Fédéral : Social Security 6,2 %+6,2 % (plafonné), Medicare 1,45 %+1,45 %,
-- Additional Medicare 0,9 % salarié (> 200 000 $/an), FUTA 0,6 % employeur.
-- California SDI 1,2 % salarié (Californie uniquement).
-- Plafonds (wage base) et barèmes d'impôt fédéral/État calculés en Rust
-- (us_cotisations.rs / us_impot.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('US_SSA', 'Social Security Administration', 'https://www.ssa.gov'),
  ('US_IRS', 'Internal Revenue Service',       'https://www.irs.gov'),
  ('US_CA_EDD', 'California Employment Development Department', 'https://edd.ca.gov');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('US_FICA', 'LOI', 'Federal Insurance Contributions Act', '26 U.S.C. ch. 21', '1935-08-14', '1937-01-01',
   'https://www.law.cornell.edu/uscode/text/26/subtitle-C/chapter-21',
   'FICA 2025 : Social Security (OASDI) 6,2 %+6,2 % plafonné à 176 100 $/an ; Medicare 1,45 %+1,45 % sans plafond ; Additional Medicare 0,9 % salarié au-delà de 200 000 $/an.'),
  ('US_FUTA', 'LOI', 'Federal Unemployment Tax Act', '26 U.S.C. ch. 23', '1939-02-10', '1939-01-01',
   'https://www.law.cornell.edu/uscode/text/26/subtitle-C/chapter-23',
   'FUTA 2025 : chômage fédéral, 6,0 % nominal ramené à 0,6 % effectif (crédit d''État 5,4 %), 100 % employeur, sur les 7 000 premiers $/an.'),
  ('US_CASDI', 'LOI', 'California State Disability Insurance', 'Cal. Unemp. Ins. Code §984', '2022-09-30', '2024-01-01',
   'https://leginfo.legislature.ca.gov',
   'SDI Californie : 1,2 % salarié en 2025, sans plafond de salaire depuis SB 951 (01/01/2024).'),
  ('US_HISTOIRE', 'LOI', 'États-Unis — histoire fiscale et sociale', '—', '1935-08-14', '1935-08-14',
   'https://www.ssa.gov/history',
   'Social Security Act de 1935 (New Deal, F. D. Roosevelt) : socle du système social américain. Medicare créé en 1965 (L. B. Johnson). Impôt fédéral sur le revenu : 16e amendement (1913). Fédéralisme fiscal : chaque État fixe (ou non) son impôt sur le revenu — 9 États sans impôt (TX, FL, WA…), d''autres forfaitaires ou progressifs.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('US_SS', 'Social Security (OASDI)',
   (SELECT id FROM organisme WHERE code = 'US_SSA'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025 : 6,2 % salarié + 6,2 % employeur, plafond 176 100 $/an.'),
  ('US_MEDICARE', 'Medicare',
   (SELECT id FROM organisme WHERE code = 'US_SSA'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 1,45 % salarié + 1,45 % employeur, sans plafond.'),
  ('US_ADD_MEDICARE', 'Additional Medicare',
   (SELECT id FROM organisme WHERE code = 'US_IRS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 0,9 % salarié au-delà de 200 000 $/an.'),
  ('US_FUTA', 'FUTA — Chômage fédéral',
   (SELECT id FROM organisme WHERE code = 'US_IRS'), 'CHOMAGE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025 : 0,6 % effectif employeur sur 7 000 $/an.'),
  ('US_CA_SDI', 'California SDI',
   (SELECT id FROM organisme WHERE code = 'US_CA_EDD'), 'PREVOYANCE', 1, 1, 'BRUT_TOTAL',
   '2025 : 1,2 % salarié (Californie), sans plafond.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'US_SS'),           '2025-01-01', NULL, '0.062',  '0.062',
   (SELECT id FROM texte_loi WHERE code = 'US_FICA'), 'Social Security 2025.'),
  ((SELECT id FROM cotisation WHERE code = 'US_MEDICARE'),     '2025-01-01', NULL, '0.0145', '0.0145',
   (SELECT id FROM texte_loi WHERE code = 'US_FICA'), 'Medicare 2025.'),
  ((SELECT id FROM cotisation WHERE code = 'US_ADD_MEDICARE'), '2025-01-01', NULL, '0.009',  '0',
   (SELECT id FROM texte_loi WHERE code = 'US_FICA'), 'Additional Medicare 2025.'),
  ((SELECT id FROM cotisation WHERE code = 'US_FUTA'),         '2025-01-01', NULL, '0',      '0.006',
   (SELECT id FROM texte_loi WHERE code = 'US_FUTA'), 'FUTA 2025.'),
  ((SELECT id FROM cotisation WHERE code = 'US_CA_SDI'),       '2025-01-01', NULL, '0.012',  '0',
   (SELECT id FROM texte_loi WHERE code = 'US_CASDI'), 'California SDI 2025.');
