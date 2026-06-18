-- 0095 — Chypre : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise EUR. Données : 2025.
--
-- Assurance sociale 8,8 % sal / 8,8 % pat (plafond 5 551 €/mois) + GESY 2,65 % sal /
-- 2,90 % pat. Impôt progressif (0-35 %) calculé en Rust (cy_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('CY_SID', 'Υπηρεσίες Κοινωνικών Ασφαλίσεων — assurances sociales', 'https://www.mlsi.gov.cy'),
  ('CY_TAX', 'Τμήμα Φορολογίας — administration fiscale',             'https://www.mof.gov.cy');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('CY_SI_L', 'LOI', 'Περί Κοινωνικών Ασφαλίσεων Νόμος', '59(I)/2010', '2010-07-30', '2011-01-01',
   'https://www.mlsi.gov.cy',
   'Assurance sociale 2025 : 8,8 % salarié / 8,8 % employeur (plafond 5 551 €/mois). GESY (santé) : 2,65 % salarié / 2,90 % employeur.'),
  ('CY_TAX_L', 'LOI', 'Περί Φορολογίας του Εισοδήματος Νόμος', '118(I)/2002', '2002-07-15', '2003-01-01',
   'https://www.mof.gov.cy',
   'Impôt sur le revenu 2025 : 0 % jusqu''à 19 500 €, 20 % / 25 % / 30 % / 35 % (seuils 28 000 / 36 300 / 60 000 €). Cotisations déductibles.'),
  ('CY_HISTOIRE', 'LOI', 'Chypre — histoire fiscale et sociale', '—', '2019-01-01', '2019-01-01',
   'https://www.mof.gov.cy',
   'Fiscalité attractive (taux d''IS bas) ayant fait de l''île un centre financier. Tranche d''impôt sur le revenu à 0 % généreuse (19 500 €). Création du système national de santé GESY en 2019, financé par des contributions partagées — réforme sociale majeure. Politiquement : équilibre entre compétitivité fiscale et universalisation de la santé.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('CY_SI', 'Κοινωνικές Ασφαλίσεις — Assurance sociale',
   (SELECT id FROM organisme WHERE code = 'CY_SID'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025 : 8,8 % salarié / 8,8 % employeur. Plafond 5 551 €/mois.'),
  ('CY_GESY', 'ΓΕΣΥ — Système national de santé',
   (SELECT id FROM organisme WHERE code = 'CY_SID'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 2,65 % salarié / 2,90 % employeur.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'CY_SI'),   '2025-01-01', NULL, '0.088',  '0.088',
   (SELECT id FROM texte_loi WHERE code = 'CY_SI_L'), 'Assurance sociale 2025 : 8,8 % sal / 8,8 % pat.'),
  ((SELECT id FROM cotisation WHERE code = 'CY_GESY'), '2025-01-01', NULL, '0.0265', '0.029',
   (SELECT id FROM texte_loi WHERE code = 'CY_SI_L'), 'GESY 2025 : 2,65 % sal / 2,90 % pat.');
