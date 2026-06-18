-- 0071 — Danemark : organismes, textes de loi et AM-bidrag 2025
-- Périmètre : salarié secteur privé. Devise DKK. Données : 2025.
--
-- Côté salarié : AM-bidrag 8 % (prélevé en premier) + ATP (forfait) + impôt
--   (bundskat 12,01 % + kommuneskat moyen 25,1 % + topskat 15 % au-delà du seuil),
--   après personfradrag (51 600 DKK/an). ATP et impôt calculés en Rust (dk_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('DK_SKAT', 'Skattestyrelsen — administration fiscale (A-skat, AM-bidrag)', 'https://www.skat.dk'),
  ('DK_ATP',  'ATP — Arbejdsmarkedets Tillægspension (retraite complémentaire)', 'https://www.atp.dk');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('DK_AMBIDRAG_LOV', 'LOI', 'Arbejdsmarkedsbidragsloven — Loi sur la contribution au marché du travail', 'LBK nr 121', '2017-01-31', '2011-01-01',
   'https://www.retsinformation.dk',
   'AM-bidrag : 8 % du salaire brut, prélevé avant l''impôt sur le revenu.'),
  ('DK_PERSONSKAT_LOV', 'LOI', 'Personskatteloven — Loi sur l''impôt des personnes physiques', 'LBK nr 1284', '2024-11-14', '2025-01-01',
   'https://www.retsinformation.dk',
   '2025 : bundskat 12,01 %, kommuneskat moyen 25,1 %, topskat 15 % au-delà de 611 800 DKK (revenu après AM-bidrag). Personfradrag 51 600 DKK/an.'),
  ('DK_ATP_LOV', 'LOI', 'ATP-loven — Loi sur la pension complémentaire du marché du travail', 'LBK nr 1110', '2018-08-24', '1964-04-01',
   'https://www.retsinformation.dk',
   'ATP : cotisation forfaitaire. Temps plein 2025 : ≈ 94,65 DKK/mois salarié (2/3 employeur).');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('DK_AM', 'AM-bidrag — Contribution marché du travail',
   (SELECT id FROM organisme WHERE code = 'DK_SKAT'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 8 % du salaire brut, prélevé avant l''impôt.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'DK_AM'), '2025-01-01', NULL, '0.08', '0',
   (SELECT id FROM texte_loi WHERE code = 'DK_AMBIDRAG_LOV'), 'AM-bidrag 2025 : 8 % salarié.');
