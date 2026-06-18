-- 0092 — Hongrie : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise HUF. Données : 2025.
--
-- TB (cotisation sociale) 18,5 % salarié ; szocho 13 % employeur. SZJA 15 % (flat)
-- calculé en Rust (hu_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('HU_NAV', 'Nemzeti Adó- és Vámhivatal — administration fiscale et douanière', 'https://www.nav.gov.hu'),
  ('HU_MAK', 'Magyar Államkincstár — trésor / prestations sociales',             'https://www.allamkincstar.gov.hu');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('HU_TBL', 'LOI', 'Társadalombiztosítási járulék — loi sur la cotisation sociale', '2019. évi CXXII.', '2019-12-23', '2020-07-01',
   'https://njt.hu',
   'TB (cotisation sociale unifiée) 2025 : 18,5 % salarié (retraite 10 % + maladie 7 % + chômage 1,5 %). Szociális hozzájárulási adó (szocho) employeur 13 %.'),
  ('HU_SZJAL', 'LOI', 'SZJA — loi sur l''impôt sur le revenu', '1995. évi CXVII.', '1995-12-22', '1996-01-01',
   'https://njt.hu',
   'Impôt sur le revenu 2025 : taux proportionnel unique 15 %. Abattements familiaux (családi kedvezmény), exonération des moins de 25 ans et des mères de moins de 30 ans.'),
  ('HU_HISTOIRE', 'LOI', 'Hongrie — histoire fiscale et sociale', '—', '2011-01-01', '2011-01-01',
   'https://www.nav.gov.hu',
   'Bascule vers la « flat tax » en 2011 (gouvernement Orbán) : impôt proportionnel unique (16 %, puis 15 % en 2016), parmi les plus bas d''Europe. Cotisations sociales unifiées en TB (2020). Politique nataliste marquée : forts abattements familiaux et exonérations ciblées (jeunes, mères). Politiquement : modèle « illibéral » combinant impôt bas et redistribution familiale.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('HU_TB', 'Társadalombiztosítás — Cotisation sociale (salarié)',
   (SELECT id FROM organisme WHERE code = 'HU_MAK'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 18,5 % salarié (retraite 10 % + maladie 7 % + chômage 1,5 %).'),
  ('HU_SZOCHO', 'Szociális hozzájárulási adó (employeur)',
   (SELECT id FROM organisme WHERE code = 'HU_NAV'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 13 % employeur.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'HU_TB'),     '2025-01-01', NULL, '0.185', '0',
   (SELECT id FROM texte_loi WHERE code = 'HU_TBL'), 'TB 2025 : 18,5 % salarié.'),
  ((SELECT id FROM cotisation WHERE code = 'HU_SZOCHO'), '2025-01-01', NULL, '0', '0.13',
   (SELECT id FROM texte_loi WHERE code = 'HU_TBL'), 'Szocho 2025 : 13 % employeur.');
