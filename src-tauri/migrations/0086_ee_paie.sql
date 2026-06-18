-- 0086 — Estonie : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise EUR. Données : 2025.
--
-- Salarié : chômage 1,6 % + 2ᵉ pilier 2 % + impôt 22 % (abattement de base dégressif,
-- calculé en Rust ee_bulletin.rs). Employeur : sotsiaalmaks 33 % + chômage 0,8 %.

INSERT INTO organisme (code, libelle, url) VALUES
  ('EE_EMTA', 'Maksu- ja Tolliamet — administration fiscale et douanière', 'https://www.emta.ee'),
  ('EE_TK',   'Eesti Töötukassa — caisse chômage',                          'https://www.tootukassa.ee'),
  ('EE_SK',   'Sotsiaalkindlustusamet — assurance sociale',                 'https://www.sotsiaalkindlustusamet.ee');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('EE_TMS', 'LOI', 'Tulumaksuseadus — Loi sur l''impôt sur le revenu', '—', '1999-12-15', '2000-01-01',
   'https://www.riigiteataja.ee',
   'Impôt sur le revenu 2025 : taux unique 22 % (relevé de 20 % à 22 % au 01/01/2025). Abattement de base (maksuvaba tulu) dégressif : 7 848 €/an si revenu ≤ 14 400 €, nul si ≥ 25 200 €.'),
  ('EE_SMS', 'LOI', 'Sotsiaalmaksuseadus — Loi sur la charge sociale', '—', '2000-12-13', '2001-01-01',
   'https://www.riigiteataja.ee',
   'Sotsiaalmaks 2025 : 33 % patronal (20 % retraite + 13 % maladie). Chômage : 1,6 % salarié / 0,8 % employeur. 2ᵉ pilier (kogumispension) : 2 % salarié (défaut).'),
  ('EE_HISTOIRE', 'LOI', 'Estonie — histoire fiscale et sociale', '—', '1994-01-01', '1994-01-01',
   'https://www.emta.ee',
   'Pionnière de la « flat tax » : impôt proportionnel introduit en 1994 (26 %), emblème du libéralisme post-soviétique balte. Baisse progressive jusqu''à 20 %, puis remontée à 22 % en 2025 pour financer la défense et le social. Charge sociale concentrée sur l''employeur (sotsiaalmaks 33 %). 2ᵉ pilier par capitalisation rendu plus flexible (réforme 2021). Politiquement : modèle numérique et fiscalement simple.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('EE_TOOTUS', 'Töötuskindlustusmakse — Chômage',
   (SELECT id FROM organisme WHERE code = 'EE_TK'), 'CHOMAGE', 1, 1, 'BRUT_TOTAL',
   '2025 : 1,6 % salarié / 0,8 % employeur.'),
  ('EE_KOGUMISPENSION', 'Kogumispension — Retraite 2ᵉ pilier',
   (SELECT id FROM organisme WHERE code = 'EE_SK'), 'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_TOTAL',
   '2025 : 2 % salarié (taux par défaut ; 4 % ou 6 % sur option).'),
  ('EE_SOTSIAALMAKS', 'Sotsiaalmaks — Charge sociale (employeur)',
   (SELECT id FROM organisme WHERE code = 'EE_SK'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 33 % patronal (20 % retraite + 13 % maladie).');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'EE_TOOTUS'),         '2025-01-01', NULL, '0.016', '0.008',
   (SELECT id FROM texte_loi WHERE code = 'EE_SMS'), 'Chômage 2025 : 1,6 % sal / 0,8 % pat.'),
  ((SELECT id FROM cotisation WHERE code = 'EE_KOGUMISPENSION'), '2025-01-01', NULL, '0.02', '0',
   (SELECT id FROM texte_loi WHERE code = 'EE_SMS'), '2ᵉ pilier 2025 : 2 % sal (défaut).'),
  ((SELECT id FROM cotisation WHERE code = 'EE_SOTSIAALMAKS'),   '2025-01-01', NULL, '0', '0.33',
   (SELECT id FROM texte_loi WHERE code = 'EE_SMS'), 'Sotsiaalmaks 2025 : 33 % pat.');
