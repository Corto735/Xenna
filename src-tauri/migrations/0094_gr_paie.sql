-- 0094 — Grèce : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise EUR. Données : 2025.
--
-- EFKA 13,87 % sal / 22,29 % pat (assiette plafonnée 7 572,62 €/mois). Impôt
-- progressif (9-44 %) et réduction salarié calculés en Rust (gr_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('GR_EFKA', 'e-EFKA — organisme unifié de sécurité sociale', 'https://www.efka.gov.gr'),
  ('GR_AADE', 'ΑΑΔΕ — administration fiscale indépendante',    'https://www.aade.gr');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('GR_EFKA_L', 'LOI', 'Νόμος 4387/2016 — réforme de la sécurité sociale (EFKA)', '4387/2016', '2016-05-12', '2017-01-01',
   'https://www.efka.gov.gr',
   'EFKA 2025 : salarié 13,87 % / employeur 22,29 % (retraite, maladie, complémentaire). Assiette plafonnée à 7 572,62 €/mois.'),
  ('GR_KFE', 'LOI', 'Κώδικας Φορολογίας Εισοδήματος (Ν. 4172/2013)', '4172/2013', '2013-07-23', '2014-01-01',
   'https://www.aade.gr',
   'Impôt sur le revenu 2025 : 9 % jusqu''à 10 000 €, 22 % / 28 % / 36 % / 44 % (seuils 20 000 / 30 000 / 40 000 €). Réduction d''impôt salarié 777 € (sans enfant).'),
  ('GR_HISTOIRE', 'LOI', 'Grèce — histoire fiscale et sociale', '—', '2016-01-01', '2016-01-01',
   'https://www.efka.gov.gr',
   'Système bismarckien longtemps fragmenté en multiples caisses, unifié dans l''EFKA en 2017 sous la pression des mémorandums post-crise (2010-2018). Cotisations et fiscalité profondément remaniées durant l''ajustement. Politiquement : réformes imposées par la crise de la dette, puis stabilisation.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('GR_EFKA', 'EFKA — Cotisations sociales',
   (SELECT id FROM organisme WHERE code = 'GR_EFKA'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025 : 13,87 % salarié / 22,29 % employeur. Assiette plafonnée à 7 572,62 €/mois.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'GR_EFKA'), '2025-01-01', NULL, '0.1387', '0.2229',
   (SELECT id FROM texte_loi WHERE code = 'GR_EFKA_L'), 'EFKA 2025 : 13,87 % sal / 22,29 % pat.');
