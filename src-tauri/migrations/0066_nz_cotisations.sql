-- 0066 — Nouvelle-Zélande : KiwiSaver employeur (par défaut 3 %) 2025-26
-- Versé en sus du salaire (n'affecte pas le net). L'ACC earner's levy et le PAYE
-- (côté salarié) sont calculés en Rust (nz_bulletin.rs).
-- Source : KiwiSaver Act 2006 — taux employeur minimum 3 %.

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('NZ_KIWISAVER_EMP', 'KiwiSaver — Cotisation retraite employeur',
   (SELECT id FROM organisme WHERE code = 'NZ_IRD'), 'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_TOTAL',
   '2025-26 : 3,0 % patronal (taux minimum par défaut), versé en sus. Optionnel selon adhésion du salarié. KiwiSaver Act 2006.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'NZ_KIWISAVER_EMP'), '2025-04-01', NULL, '0', '0.0300',
   (SELECT id FROM texte_loi WHERE code = 'NZ_KIWISAVER_2006'), 'KiwiSaver employeur 3,0 % (défaut) année 2025-26.');
