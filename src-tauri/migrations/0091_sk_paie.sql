-- 0091 — Slovaquie : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise EUR. Données : 2025.
--
-- Zdravotné 4 % (non plafonné) + sociálne 9,4 % (plafond 15 730 €/mois) salarié.
-- Daň 19 % / 25 % et part non imposable calculées en Rust (sk_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('SK_SP',  'Sociálna poisťovňa — sécurité sociale',                 'https://www.socpoist.sk'),
  ('SK_UDZS','Úrad pre dohľad nad zdravotnou starostlivosťou — santé', 'https://www.udzs-sk.sk'),
  ('SK_FS',  'Finančná správa — administration fiscale',              'https://www.financnasprava.sk');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('SK_SOC', 'LOI', 'Zákon o sociálnom poistení', '461/2003', '2003-10-30', '2004-01-01',
   'https://www.slov-lex.sk',
   'Cotisations 2025 : zdravotné (santé) 4 % salarié / 11 % employeur (sans plafond) ; sociálne 9,4 % salarié / 25,2 % employeur, assiette plafonnée à 15 730 €/mois.'),
  ('SK_DAN', 'LOI', 'Zákon o dani z príjmov', '595/2003', '2003-12-04', '2004-01-01',
   'https://www.slov-lex.sk',
   'Daň z príjmov 2025 : 19 % jusqu''à 48 441,43 €/an (4 036,79 €/mois), 25 % au-delà. Nezdaniteľná časť (part non imposable) 479,48 €/mois, dégressive pour hauts revenus.'),
  ('SK_HISTOIRE', 'LOI', 'Slovaquie — histoire fiscale et sociale', '—', '2004-01-01', '2004-01-01',
   'https://www.financnasprava.sk',
   'Symbole des réformes libérales d''Europe centrale : « flat tax » à 19 % introduite en 2004 (gouvernement Dzurinda), saluée internationalement. Retour à un léger barème progressif (19 % / 25 %) à partir de 2013. Sécurité sociale bismarckienne (Sociálna poisťovňa). Politiquement : oscillation entre libéralisme fiscal et redistribution.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('SK_ZDRAVOTNE', 'Zdravotné poistenie — Assurance maladie',
   (SELECT id FROM organisme WHERE code = 'SK_UDZS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 4 % salarié / 11 % employeur (sans plafond).'),
  ('SK_SOCIALNE', 'Sociálne poistenie — Sécurité sociale',
   (SELECT id FROM organisme WHERE code = 'SK_SP'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025 : 9,4 % salarié / 25,2 % employeur. Assiette plafonnée à 15 730 €/mois.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'SK_ZDRAVOTNE'), '2025-01-01', NULL, '0.04',  '0.11',
   (SELECT id FROM texte_loi WHERE code = 'SK_SOC'), 'Zdravotné 2025 : 4 % sal / 11 % pat.'),
  ((SELECT id FROM cotisation WHERE code = 'SK_SOCIALNE'),  '2025-01-01', NULL, '0.094', '0.252',
   (SELECT id FROM texte_loi WHERE code = 'SK_SOC'), 'Sociálne 2025 : 9,4 % sal / 25,2 % pat (plafond 15 730 €).');
