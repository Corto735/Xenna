-- 0064 — Australie : Superannuation Guarantee (100 % patronale) 2025-26
-- Assiette plafonnée à la maximum contribution base (≈ 250 000 $/an en 2025-26).
-- L'impôt sur le revenu + Medicare levy (côté salarié) sont calculés en Rust (au_bulletin.rs).
-- Source : ATO — Super guarantee percentage / maximum super contribution base 2025-26.

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('AU_SUPER', 'Superannuation Guarantee — Retraite (employeur)',
   (SELECT id FROM organisme WHERE code = 'AU_SUPER'), 'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025-26 : 12,0 % patronal, versé en sus du salaire. Assiette plafonnée à la maximum contribution base (≈ 250 000 $/an). SGAA 1992.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'AU_SUPER'), '2025-07-01', NULL, '0', '0.1200',
   (SELECT id FROM texte_loi WHERE code = 'AU_SGAA_1992'), 'Super Guarantee 12,0 % à partir du 01/07/2025 (exercice 2025-26).');
