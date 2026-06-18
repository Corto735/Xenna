-- 0096 — Malte : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé (Klassi 1). Devise EUR. Données : 2025.
--
-- SSC 10 % sal / 10 % pat (assiette plafonnée ≈ 2 306,58 €/mois). Impôt sur le revenu
-- (barème single 0-35 %) calculé en Rust (mt_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('MT_DSS', 'Department of Social Security',           'https://socialsecurity.gov.mt'),
  ('MT_CFR', 'Commissioner for Revenue — fiscalité',    'https://cfr.gov.mt');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('MT_SSA', 'LOI', 'Social Security Act (Cap. 318)', 'Cap. 318', '1987-01-01', '1987-01-01',
   'https://legislation.mt',
   'Social Security Contributions 2025 (Klassi 1) : 10 % salarié / 10 % employeur, assiette plafonnée (base maximale ≈ 27 679 €/an pour les personnes nées après 1962).'),
  ('MT_ITA', 'LOI', 'Income Tax Act (Cap. 123)', 'Cap. 123', '1949-01-01', '1949-01-01',
   'https://legislation.mt',
   'Impôt sur le revenu 2025 (barème single) : 0 % jusqu''à 12 000 €, 15 % / 25 % / 35 % (abattements 1 800 / 3 400 / 9 400 €). Élargissement de la tranche à 0 % au budget 2025.'),
  ('MT_HISTOIRE', 'LOI', 'Malte — histoire fiscale et sociale', '—', '1979-01-01', '1979-01-01',
   'https://cfr.gov.mt',
   'Héritage britannique : système de sécurité sociale de type Beveridge (financement par contributions forfaitaires/proportionnelles) et impôt sur le revenu à barèmes différenciés (single, married, parent). Système d''imputation intégrale pour l''IS, attractif pour les holdings. Politiquement : petit État insulaire combinant protection sociale et compétitivité fiscale.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('MT_SSC', 'Social Security Contributions (Klassi 1)',
   (SELECT id FROM organisme WHERE code = 'MT_DSS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025 : 10 % salarié / 10 % employeur. Assiette plafonnée à ≈ 2 306,58 €/mois.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'MT_SSC'), '2025-01-01', NULL, '0.10', '0.10',
   (SELECT id FROM texte_loi WHERE code = 'MT_SSA'), 'SSC 2025 : 10 % sal / 10 % pat (plafonnée).');
