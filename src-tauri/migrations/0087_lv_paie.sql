-- 0087 — Lettonie : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé (régime général). Devise EUR. Données : 2025.
--
-- VSAOI : 10,50 % salarié / 23,59 % employeur. IIN (impôt) 25,5 % / 33 % et minimum
-- non imposable 510 €/mois calculés en Rust (lv_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('LV_VID', 'Valsts ieņēmumu dienests — administration fiscale',        'https://www.vid.gov.lv'),
  ('LV_VSAA', 'Valsts sociālās apdrošināšanas aģentūra — assurance sociale', 'https://www.vsaa.gov.lv');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('LV_SOC', 'LOI', 'Par valsts sociālo apdrošināšanu — assurance sociale', '—', '1997-10-01', '1998-01-01',
   'https://likumi.lv',
   'VSAOI 2025 (régime général) : 10,50 % salarié / 23,59 % employeur (retraite, maladie, chômage, maternité, accidents).'),
  ('LV_IIN', 'LOI', 'Par iedzīvotāju ienākuma nodokli — impôt sur le revenu', '—', '1993-05-11', '1994-01-01',
   'https://likumi.lv',
   'IIN 2025 : barème simplifié 25,5 % jusqu''à 105 300 €/an (8 775 €/mois), 33 % au-delà. Minimum non imposable fixe 510 €/mois (fin du dispositif différencié).'),
  ('LV_HISTOIRE', 'LOI', 'Lettonie — histoire fiscale et sociale', '—', '1995-01-01', '1995-01-01',
   'https://www.vid.gov.lv',
   'Système reconstruit après l''indépendance (1991) sur le modèle balte : assurance sociale unifiée (VSAOI, 1997-98) et impôt proportionnel. Longtemps « flat tax » (23 %), passage à un barème progressif en 2018, puis simplification en deux taux (25,5 % / 33 %) avec minimum non imposable fixe en 2025. Politiquement : convergence européenne et stabilisation post-crise 2008.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('LV_VSAOI', 'VSAOI — Cotisations sociales obligatoires',
   (SELECT id FROM organisme WHERE code = 'LV_VSAA'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 10,50 % salarié / 23,59 % employeur (régime général).');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'LV_VSAOI'), '2025-01-01', NULL, '0.105', '0.2359',
   (SELECT id FROM texte_loi WHERE code = 'LV_SOC'), 'VSAOI 2025 : 10,50 % sal / 23,59 % pat.');
