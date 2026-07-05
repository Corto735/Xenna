-- 0110 — Inde : EPF + ESI. Devise INR. Données : exercice fiscal 2025-26.
--
-- Taux sociaux (EPF 12/12, ESI 0,75/3,25) en base. Professional Tax (forfait
-- Karnataka) et impôt sur le revenu (ancien/nouveau régime) calculés en Rust
-- (in_bulletin.rs / in_impot.rs) — la catégorie « impôt » n'existe pas au schéma.

INSERT INTO organisme (code, libelle, url) VALUES
  ('IN_EPFO', 'Employees'' Provident Fund Organisation', 'https://www.epfindia.gov.in'),
  ('IN_ESIC', 'Employees'' State Insurance Corporation', 'https://www.esic.gov.in');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('IN_EPF52', 'LOI', 'Employees'' Provident Funds & Misc. Provisions Act 1952', 'Act 19/1952',
   '1952-03-04', '1952-11-14', 'https://www.epfindia.gov.in',
   'EPF : 12 % salarié + 12 % employeur ; assiette légale minimale 15 000 INR/mois (EPS 8,33 % / EPF 3,67 %).'),
  ('IN_ESI48', 'LOI', 'Employees'' State Insurance Act 1948', 'Act 34/1948',
   '1948-04-19', '1948-04-19', 'https://www.esic.gov.in',
   'ESI : 0,75 % salarié + 3,25 % employeur, dû si brut mensuel ≤ 21 000 INR.'),
  ('IN_ITA61', 'LOI', 'Income-tax Act 1961 (Finance Act 2025)', 'Act 43/1961',
   '1961-09-13', '1962-04-01', 'https://incometaxindia.gov.in',
   'Impôt sur le revenu : ancien et nouveau régime (sec. 115BAC, défaut) ; déduction standard, rebate 87A, cess santé & éducation 4 %.'),
  ('IN_HISTOIRE', 'LOI', 'Inde — histoire sociale et fiscale', '—', '1948-04-19', '1948-04-19',
   'https://www.epfindia.gov.in',
   'ESI Act (1948) et EPF Act (1952) fondent la sécurité sociale du secteur formel. Income-tax Act 1961. Nouveau régime d''imposition simplifié introduit en 2020 (sec. 115BAC), rendu régime par défaut en 2023.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('IN_EPF', 'EPF — Fonds de prévoyance',
   (SELECT id FROM organisme WHERE code = 'IN_EPFO'), 'RETRAITE_COMPLEMENTAIRE', 1, 1, 'SPECIFIQUE',
   '12 % sal + 12 % pat, assiette min. 15 000 INR.'),
  ('IN_ESI', 'ESI — Assurance maladie',
   (SELECT id FROM organisme WHERE code = 'IN_ESIC'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '0,75 % sal + 3,25 % pat si brut ≤ 21 000 INR.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'IN_EPF'), '2025-01-01', NULL, '0.12',   '0.12',
   (SELECT id FROM texte_loi WHERE code = 'IN_EPF52'), 'EPF 2025-26.'),
  ((SELECT id FROM cotisation WHERE code = 'IN_ESI'), '2025-01-01', NULL, '0.0075', '0.0325',
   (SELECT id FROM texte_loi WHERE code = 'IN_ESI48'), 'ESI 2025-26.');
