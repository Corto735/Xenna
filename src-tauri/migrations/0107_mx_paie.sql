-- 0107 — Mexique : organismes, textes de loi, cotisations IMSS/INFONAVIT 2025
-- Périmètre : salarié secteur privé. Devise MXN. Données : 2025 (2026 reconduit).
--
-- Salarié : IMSS obrero ~2,375 % + excédent 0,40 % (> 3 UMA). Employeur :
-- INFONAVIT 5 % + retiro SAR 2 %. Barème ISR, UMA et subsidio en Rust (mx_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('MX_IMSS', 'Instituto Mexicano del Seguro Social', 'https://www.imss.gob.mx'),
  ('MX_SAT',  'Servicio de Administración Tributaria', 'https://www.sat.gob.mx'),
  ('MX_INFONAVIT', 'Instituto del Fondo Nacional de la Vivienda para los Trabajadores', 'https://portalmx.infonavit.org.mx');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('MX_LSS', 'LOI', 'Ley del Seguro Social', 'DOF 21-12-1995', '1995-12-21', '1997-07-01',
   'https://www.diputados.gob.mx/LeyesBiblio/pdf/LSS.pdf',
   'Cuotas obrero 2025 : enfermedad/maternidad, invalidez y vida, cesantía y vejez (~2,375 %) + excédent 0,40 % au-delà de 3 UMA.'),
  ('MX_LISR', 'LOI', 'Ley del Impuesto sobre la Renta', 'DOF 11-12-2013', '2013-12-11', '2014-01-01',
   'https://www.diputados.gob.mx/LeyesBiblio/pdf/LISR.pdf',
   'ISR retención mensual (art. 96) : barème progressif cuota fija + % excédent ; subsidio al empleo (DOF 01/05/2024, jusqu''à 406,83 $ pour revenu ≤ 9 081 $).'),
  ('MX_LINF', 'LOI', 'Ley del INFONAVIT', 'DOF 24-04-1972', '1972-04-24', '1972-05-01',
   'https://www.diputados.gob.mx/LeyesBiblio/pdf/LinfonavitAdmin.pdf',
   'INFONAVIT : 5 % employeur pour le logement des travailleurs (art. 29).'),
  ('MX_HISTOIRE', 'LOI', 'Mexique — histoire fiscale et sociale', '—', '1943-01-01', '1943-01-01',
   'https://www.imss.gob.mx',
   'IMSS fondé en 1943 (sécurité sociale tripartite). INFONAVIT créé en 1972 (logement). Réforme des retraites 1997 (comptes individuels Afore, capitalisation). Montée en charge cesantía y vejez jusqu''en 2030 (réforme 2020). UMA (Unidad de Medida y Actualización) remplace le salaire minimum comme référence depuis 2016.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('MX_IMSS', 'IMSS — Cuotas obrero',
   (SELECT id FROM organisme WHERE code = 'MX_IMSS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : ~2,375 % salarié (agrégé).'),
  ('MX_IMSS_EXC', 'IMSS — Excédente (> 3 UMA)',
   (SELECT id FROM organisme WHERE code = 'MX_IMSS'), 'SECURITE_SOCIALE', 1, 1, 'SPECIFIQUE',
   '2025 : 0,40 % salarié sur l''excédent au-delà de 3 UMA.'),
  ('MX_INFONAVIT', 'INFONAVIT — Logement',
   (SELECT id FROM organisme WHERE code = 'MX_INFONAVIT'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 5 % employeur.'),
  ('MX_RETIRO', 'Retiro (SAR)',
   (SELECT id FROM organisme WHERE code = 'MX_IMSS'), 'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_TOTAL',
   '2025 : 2 % employeur (Afore).');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'MX_IMSS'),      '2025-01-01', NULL, '0.02375', '0',
   (SELECT id FROM texte_loi WHERE code = 'MX_LSS'), 'IMSS obrero 2025.'),
  ((SELECT id FROM cotisation WHERE code = 'MX_IMSS_EXC'),  '2025-01-01', NULL, '0.004',   '0',
   (SELECT id FROM texte_loi WHERE code = 'MX_LSS'), 'Excédente 2025.'),
  ((SELECT id FROM cotisation WHERE code = 'MX_INFONAVIT'), '2025-01-01', NULL, '0',       '0.05',
   (SELECT id FROM texte_loi WHERE code = 'MX_LINF'), 'INFONAVIT 2025.'),
  ((SELECT id FROM cotisation WHERE code = 'MX_RETIRO'),    '2025-01-01', NULL, '0',       '0.02',
   (SELECT id FROM texte_loi WHERE code = 'MX_LSS'), 'Retiro SAR 2025.');
