-- 0072 — Finlande : organismes, textes de loi et cotisations salariales 2026
-- Périmètre : salarié secteur privé (17-68 ans). Devise EUR. Données : 2026.
--
-- Côté salarié : TyEL (retraite 7,30 %) + chômage 0,89 % + assurance maladie
--   (päivärahamaksu 0,88 % si revenu ≥ 17 255 € + sairaanhoitomaksu 1,10 %)
--   + impôt d'État (barème progressif) + impôt communal (moyen 7,50 %).
-- Impôts calculés en Rust (fi_bulletin.rs) ; taux de cotisation lus en base.

INSERT INTO organisme (code, libelle, url) VALUES
  ('FI_VERO', 'Verohallinto — administration fiscale (impôt d''État + communal)', 'https://www.vero.fi'),
  ('FI_ETK',  'Eläketurvakeskus / TyEL — retraite liée aux revenus',              'https://www.etk.fi'),
  ('FI_TVR',  'Työllisyysrahasto — assurance chômage (työttömyysvakuutus)',       'https://www.tyollisyysrahasto.fi'),
  ('FI_KELA', 'Kela — assurance maladie (sairausvakuutus)',                       'https://www.kela.fi');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('FI_TVL', 'LOI', 'Tuloverolaki — Loi sur l''impôt sur le revenu', '1535/1992', '1992-12-30', '1993-01-01',
   'https://www.finlex.fi/fi/laki/ajantasa/1992/19921535',
   'Barème d''État 2026 (valtionvero) : 12,64 % jusqu''à 21 200 €, 19 % de 21 200 à 32 600 €, 30,25 % de 32 600 à 40 100 €, 33,25 % de 40 100 à 52 100 €, 37,5 % au-delà. Impôt communal moyen 2026 : 7,50 %.'),
  ('FI_TYEL', 'LOI', 'Työntekijän eläkelaki (TyEL) — Loi sur les retraites des salariés', '395/2006', '2006-05-19', '2007-01-01',
   'https://www.finlex.fi/fi/laki/ajantasa/2006/20060395',
   'TyEL 2026 : part salariale 7,30 % (17-68 ans ; 8,80 % pour 53-62 ans). Part employeur moyenne ≈ 17,55 %.'),
  ('FI_SVL', 'LOI', 'Sairausvakuutuslaki — Loi sur l''assurance maladie', '1224/2004', '2004-12-21', '2005-01-01',
   'https://www.finlex.fi/fi/laki/ajantasa/2004/20041224',
   'Assurance maladie salariale 2026 : sairaanhoitomaksu 1,10 % + päivärahamaksu 0,88 % (si revenu ≥ 17 255 €/an). Employeur : 1,91 %.'),
  ('FI_TVA', 'LOI', 'Työttömyysetuuksien rahoituslaki — Financement chômage', '555/1998', '1998-07-24', '1999-01-01',
   'https://www.finlex.fi/fi/laki/ajantasa/1998/19980555',
   'Assurance chômage salariale 2026 : 0,89 %. Part employeur graduée (≈ 0,20 % bas / plus pour la masse salariale élevée).');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('FI_TYEL', 'TyEL — Retraite liée aux revenus',
   (SELECT id FROM organisme WHERE code = 'FI_ETK'), 'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_TOTAL',
   '2026 : 7,30 % sal (17-68 ans) / ≈ 17,55 % pat (moyen). Déductible du revenu imposable.'),
  ('FI_TYOTTOMYYS', 'Työttömyysvakuutus — Chômage',
   (SELECT id FROM organisme WHERE code = 'FI_TVR'), 'CHOMAGE', 1, 1, 'BRUT_TOTAL',
   '2026 : 0,89 % sal / ≈ 0,20 % pat (bas, gradué). Déductible.'),
  ('FI_PAIVARAHA', 'Päivärahamaksu — Indemnités journalières (maladie)',
   (SELECT id FROM organisme WHERE code = 'FI_KELA'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2026 : 0,88 % sal, uniquement si revenu ≥ 17 255 €/an. Déductible.'),
  ('FI_SAIRAANHOITO', 'Sairaanhoitomaksu — Soins de santé',
   (SELECT id FROM organisme WHERE code = 'FI_KELA'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2026 : 1,10 % sal. Non déductible du revenu imposable.'),
  ('FI_TYONANTAJA_SV', 'Työnantajan sairausvakuutusmaksu — Assurance maladie (employeur)',
   (SELECT id FROM organisme WHERE code = 'FI_KELA'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2026 : 1,91 % pat.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'FI_TYEL'),         '2026-01-01', NULL, '0.073',  '0.1755',
   (SELECT id FROM texte_loi WHERE code = 'FI_TYEL'), 'TyEL 2026 : 7,30 % sal / 17,55 % pat (moyen).'),
  ((SELECT id FROM cotisation WHERE code = 'FI_TYOTTOMYYS'),   '2026-01-01', NULL, '0.0089', '0.0020',
   (SELECT id FROM texte_loi WHERE code = 'FI_TVA'),  'Chômage 2026 : 0,89 % sal / 0,20 % pat (bas).'),
  ((SELECT id FROM cotisation WHERE code = 'FI_PAIVARAHA'),    '2026-01-01', NULL, '0.0088', '0',
   (SELECT id FROM texte_loi WHERE code = 'FI_SVL'),  'Päivärahamaksu 2026 : 0,88 % (si revenu ≥ 17 255 €).'),
  ((SELECT id FROM cotisation WHERE code = 'FI_SAIRAANHOITO'), '2026-01-01', NULL, '0.0110', '0',
   (SELECT id FROM texte_loi WHERE code = 'FI_SVL'),  'Sairaanhoitomaksu 2026 : 1,10 %.'),
  ((SELECT id FROM cotisation WHERE code = 'FI_TYONANTAJA_SV'),'2026-01-01', NULL, '0',      '0.0191',
   (SELECT id FROM texte_loi WHERE code = 'FI_SVL'),  'Sairausvakuutus employeur 2026 : 1,91 %.');
