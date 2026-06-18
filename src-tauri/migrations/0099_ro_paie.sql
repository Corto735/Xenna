-- 0099 — Roumanie : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise RON. Données : 2025.
--
-- CAS (pension) 25 % sal / CASS (santé) 10 % sal / CAM (contribution de travail)
-- 2,25 % pat. Impôt 10 % proportionnel calculé en Rust (ro_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('RO_ANAF', 'ANAF — administration fiscale',                 'https://www.anaf.ro'),
  ('RO_CNPP', 'CNPP / CNAS — pensions et assurance santé',     'https://www.cnpp.ro');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('RO_CF', 'LOI', 'Codul fiscal (Legea 227/2015)', '227/2015', '2015-09-08', '2016-01-01',
   'https://static.anaf.ro/static/10/Anaf/legislatie/Cod_fiscal_norme_2025.htm',
   'Codul fiscal 2025 : CAS (pension) 25 % salarié, CASS (santé) 10 % salarié, CAM (contribution de travail) 2,25 % employeur. Impozit pe venit 10 % proportionnel sur le revenu après CAS et CASS.'),
  ('RO_HISTOIRE', 'LOI', 'Roumanie — histoire fiscale et sociale', '—', '2018-01-01', '2018-01-01',
   'https://www.anaf.ro',
   'Réforme de 2018 (« revoluția fiscală ») : transfert quasi total des cotisations sociales de l''employeur vers le salarié (CAS 25 % + CASS 10 %), l''employeur ne conservant que la CAM 2,25 %. Flat tax à 10 % sur le revenu depuis 2018 (auparavant 16 %). Politiquement : choix d''un modèle à fiscalité basse et lisible, au prix d''un brut affiché élevé mais d''un net fortement amputé côté salarié.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('RO_CAS', 'CAS — Pension',
   (SELECT id FROM organisme WHERE code = 'RO_CNPP'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 25 % salarié.'),
  ('RO_CASS', 'CASS — Assurance santé',
   (SELECT id FROM organisme WHERE code = 'RO_CNPP'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 10 % salarié.'),
  ('RO_CAM', 'CAM — Contribution de travail (employeur)',
   (SELECT id FROM organisme WHERE code = 'RO_ANAF'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 2,25 % employeur.');

-- Taux figés depuis la « révolution fiscale » de 2018 (OUG 79/2017) : CAS 25 %,
-- CASS 10 %, CAM 2,25 %, impôt 10 %. Une seule plage 2018→NULL couvre 2018-2025.
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'RO_CAS'),  '2018-01-01', NULL, '0.25',   '0',      (SELECT id FROM texte_loi WHERE code = 'RO_CF'), 'CAS : 25 % sal depuis 2018.'),
  ((SELECT id FROM cotisation WHERE code = 'RO_CASS'), '2018-01-01', NULL, '0.10',   '0',      (SELECT id FROM texte_loi WHERE code = 'RO_CF'), 'CASS : 10 % sal depuis 2018.'),
  ((SELECT id FROM cotisation WHERE code = 'RO_CAM'),  '2018-01-01', NULL, '0',      '0.0225', (SELECT id FROM texte_loi WHERE code = 'RO_CF'), 'CAM : 2,25 % pat depuis 2018.');
