-- 0090 — Tchéquie : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise CZK. Données : 2025.
--
-- Sociální 7,1 % + zdravotní 4,5 % salarié. Daň 15 % / 23 % et sleva 2 570 CZK/mois
-- calculées en Rust (cz_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('CZ_CSSZ', 'Česká správa sociálního zabezpečení — sécurité sociale', 'https://www.cssz.cz'),
  ('CZ_VZP',  'Všeobecná zdravotní pojišťovna — assurance maladie',     'https://www.vzp.cz'),
  ('CZ_FS',   'Finanční správa — administration fiscale',               'https://www.financnisprava.cz');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('CZ_SOC', 'LOI', 'Zákon o pojistném na sociální zabezpečení', '589/1992', '1992-12-20', '1993-01-01',
   'https://www.zakonyprolidi.cz',
   'Sociální pojištění 2025 : salarié 7,1 % (retraite 6,5 % + maladie 0,6 %), employeur 24,8 %. Zdravotní : 4,5 % salarié / 9 % employeur.'),
  ('CZ_DAN', 'LOI', 'Zákon o daních z příjmů', '586/1992', '1992-12-20', '1993-01-01',
   'https://www.zakonyprolidi.cz',
   'Daň z příjmů 2025 : 15 % jusqu''à 36× le salaire moyen (≈ 139 671 CZK/mois), 23 % au-delà. Sleva na poplatníka 2 570 CZK/mois. Super-hrubá mzda supprimée en 2021.'),
  ('CZ_HISTOIRE', 'LOI', 'Tchéquie — histoire fiscale et sociale', '—', '1993-01-01', '1993-01-01',
   'https://www.financnisprava.cz',
   'Système refondé après la partition de la Tchécoslovaquie (1993) et la transition de marché. Impôt longtemps proportionnel (« super-brut » 2008-2020), revenu à un barème à deux taux (15 % / 23 %) après suppression de la super-hrubá mzda en 2021. Politiquement : libéralisme fiscal tempéré par une protection sociale d''héritage bismarckien.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('CZ_SOCIAL', 'Sociální pojištění — Sécurité sociale',
   (SELECT id FROM organisme WHERE code = 'CZ_CSSZ'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 7,1 % salarié / 24,8 % employeur.'),
  ('CZ_ZDRAVOTNI', 'Zdravotní pojištění — Assurance maladie',
   (SELECT id FROM organisme WHERE code = 'CZ_VZP'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 4,5 % salarié / 9 % employeur.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'CZ_SOCIAL'),    '2025-01-01', NULL, '0.071', '0.248',
   (SELECT id FROM texte_loi WHERE code = 'CZ_SOC'), 'Sociální 2025 : 7,1 % sal / 24,8 % pat.'),
  ((SELECT id FROM cotisation WHERE code = 'CZ_ZDRAVOTNI'), '2025-01-01', NULL, '0.045', '0.09',
   (SELECT id FROM texte_loi WHERE code = 'CZ_SOC'), 'Zdravotní 2025 : 4,5 % sal / 9 % pat.');
