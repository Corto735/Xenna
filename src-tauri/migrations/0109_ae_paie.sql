-- 0109 — Émirats arabes unis : GPSSA (nationaux). Devise AED. Données : 2025.
--
-- Aucun impôt sur le revenu. Expatrié (défaut) : net = brut, aucune ligne DB.
-- National émirati : GPSSA 5 % salarié + 12,5 % employeur, assiette plafonnée
-- 50 000 AED (plafond appliqué en Rust, ae_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('AE_GPSSA', 'General Pension and Social Security Authority', 'https://gpssa.gov.ae');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('AE_FDL57', 'LOI', 'Federal Decree-Law No. 57 of 2023 — Pensions & Social Securities', 'FDL 57/2023',
   '2023-10-01', '2023-10-31', 'https://gpssa.gov.ae',
   'GPSSA : national émirati 5 % salarié + 12,5 % employeur (+ État 2,5 %). Aucun impôt sur le revenu des personnes.'),
  ('AE_HISTOIRE', 'LOI', 'Émirats — fiscalité et protection sociale', '—', '1971-12-02', '1971-12-02',
   'https://gpssa.gov.ae',
   'Fédération créée en 1971. Aucun impôt sur le revenu des personnes physiques. Régime de pension GPSSA (2000) réservé aux nationaux ; Federal Decree-Law 57/2023 harmonise et relève les taux. Impôt sur les sociétés introduit en 2023 (hors paie).');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('AE_GPSSA', 'GPSSA — Retraite (national)',
   (SELECT id FROM organisme WHERE code = 'AE_GPSSA'), 'RETRAITE_COMPLEMENTAIRE', 1, 1, 'SPECIFIQUE',
   '2025 : 5 % sal + 12,5 % pat, assiette plafonnée 50 000 AED. Nationaux uniquement.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'AE_GPSSA'), '2025-01-01', NULL, '0.05', '0.125',
   (SELECT id FROM texte_loi WHERE code = 'AE_FDL57'), 'GPSSA 2025.');
