-- 0070 — Monaco : organismes, textes de loi et cotisations 2025
-- Périmètre : salarié secteur privé. Devise EUR. Données : 2025.
--
-- Particularité : PAS d'impôt sur le revenu pour les résidents monégasques
-- (sauf nationaux français, imposés par la France — convention 1963).
-- Côté salarié : CAR (retraite 6,85 %) + assurance chômage (2,4 %).
-- CCSS (maladie/famille) : 100 % patronale. Net ≈ brut − cotisations salariales.

INSERT INTO organisme (code, libelle, url) VALUES
  ('MC_CCSS', 'Caisses Sociales de Monaco — CCSS (maladie, famille)', 'https://www.caisses-sociales.mc'),
  ('MC_CAR',  'Caisse Autonome des Retraites (CAR) — retraite',        'https://www.caisses-sociales.mc'),
  ('MC_CHOM', 'Assurance chômage des salariés de Monaco (UNEDIC)',     'https://www.caisses-sociales.mc');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('MC_CAR_LOI', 'LOI', 'Régime de retraite de la CAR (Caisse Autonome des Retraites)', 'Loi n°455', '1947-06-27', '1947-06-27',
   'https://www.caisses-sociales.mc',
   'CAR : retraite par points. Part salariale 6,85 %, patronale 8,50 % (taux au 1er oct. 2025). Salaire de base 1 528 €, point 21,92 €.'),
  ('MC_CCSS_LOI', 'LOI', 'Caisse de Compensation des Services Sociaux (CCSS)', 'Ordonnance-loi n°397', '1944-09-27', '1944-09-27',
   'https://www.caisses-sociales.mc',
   'CCSS : maladie, maternité, famille. ≈ 13,40 % patronal (2024-2025), plafonné. 100 % à la charge de l''employeur.'),
  ('MC_CHOM_LOI', 'LOI', 'Assurance chômage des salariés de Monaco', 'Convention UNEDIC', '2017-04-14', '2017-04-14',
   'https://www.unedic.org',
   'Chômage : 6,4 % total dont 2,4 % salarié et 4,0 % employeur.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('MC_CAR',  'CAR — Retraite',         (SELECT id FROM organisme WHERE code = 'MC_CAR'),  'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_TOTAL',
   '2025 : 6,85 % sal / 8,50 % pat. Retraite par points (plafond CAR non modélisé ici).'),
  ('MC_CCSS', 'CCSS — Maladie/famille', (SELECT id FROM organisme WHERE code = 'MC_CCSS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : ≈ 13,40 % patronal. 100 % employeur.'),
  ('MC_CHOM', 'Chômage',                (SELECT id FROM organisme WHERE code = 'MC_CHOM'), 'CHOMAGE', 1, 1, 'BRUT_TOTAL',
   '2025 : 2,4 % sal / 4,0 % pat.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'MC_CAR'),  '2025-01-01', NULL, '0.0685', '0.0850',
   (SELECT id FROM texte_loi WHERE code = 'MC_CAR_LOI'),  'CAR 2025 : 6,85 % sal / 8,50 % pat.'),
  ((SELECT id FROM cotisation WHERE code = 'MC_CCSS'), '2025-01-01', NULL, '0',      '0.1340',
   (SELECT id FROM texte_loi WHERE code = 'MC_CCSS_LOI'), 'CCSS 2025 : 13,40 % pat.'),
  ((SELECT id FROM cotisation WHERE code = 'MC_CHOM'), '2025-01-01', NULL, '0.024',  '0.040',
   (SELECT id FROM texte_loi WHERE code = 'MC_CHOM_LOI'), 'Chômage 2025 : 2,4 % sal / 4,0 % pat.');
