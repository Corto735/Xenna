-- 0085 — Suède : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise SEK. Données : 2025.
--
-- Pas de cotisation sociale salariale nette (l'allmän pensionsavgift 7 % est
-- compensée par une réduction d'impôt). L'employeur paie les arbetsgivaravgifter
-- 31,42 %. Impôt (communal moyen 32,41 % + État 20 % au-delà de 625 800 SEK/an)
-- calculé en Rust (se_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('SE_SKV', 'Skatteverket — administration fiscale',                 'https://www.skatteverket.se'),
  ('SE_FK',  'Försäkringskassan — assurance sociale',                 'https://www.forsakringskassan.se');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('SE_SAL', 'LOI', 'Socialavgiftslagen — cotisations sociales', '2000:980', '2000-11-23', '2001-01-01',
   'https://www.riksdagen.se',
   'Arbetsgivaravgifter 2025 : 31,42 % à la charge de l''employeur (ålderspension 10,21 %, sjukförsäkring, föräldraförsäkring, arbetsmarknad, allmän löneavgift, etc.).'),
  ('SE_IL', 'LOI', 'Inkomstskattelagen — impôt sur le revenu', '1999:1229', '1999-12-16', '2000-01-01',
   'https://www.riksdagen.se',
   'Impôt communal moyen 2025 ≈ 32,41 %. Impôt d''État (statlig inkomstskatt) 20 % au-delà du skiktgräns 625 800 SEK/an (2025). Allmän pensionsavgift 7 % compensée par réduction d''impôt.'),
  ('SE_HISTOIRE', 'LOI', 'Suède — histoire du modèle social', '—', '1960-01-01', '1960-01-01',
   'https://www.skatteverket.se',
   'État-providence nordique « folkhemmet » bâti par la social-démocratie (SAP). Système de retraite refondé en 1999 (inkomstpension par répartition à comptes notionnels + premiepension par capitalisation). Financement très majoritairement par l''impôt et les cotisations patronales ; quasi-absence de cotisations salariales nettes. Politiquement : universalisme et forte imposition consenties par consensus.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('SE_ARBETSGIVARAVGIFT', 'Arbetsgivaravgifter — cotisations patronales',
   (SELECT id FROM organisme WHERE code = 'SE_FK'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 31,42 % patronal (100 %). Aucune part salariale nette.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'SE_ARBETSGIVARAVGIFT'), '2025-01-01', NULL, '0', '0.3142',
   (SELECT id FROM texte_loi WHERE code = 'SE_SAL'), 'Arbetsgivaravgifter 2025 : 31,42 % patronal.');
