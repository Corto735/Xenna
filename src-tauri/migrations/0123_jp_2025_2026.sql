-- ============================================================
-- JAPON : taux 2025 et 2026, contribution enfance
-- ============================================================
-- 協会けんぽ Tokyo (cotisations de mars, versées en avril) :
--   健康保険 9,91 % (2025) puis 9,85 % (2026) ; 介護保険 1,59 % puis 1,62 %.
-- 雇用保険, 一般の事業 (avril-mars) : 2025 sal 0,55 % / pat 0,90 % ;
--   2026 sal 0,50 % / pat 0,85 % (MHLW).
-- 子ども・子育て支援金 : 0,23 % (0,115 % chacun) dès les cotisations d'avril 2026.
UPDATE cotisation_taux SET date_fin = '2025-02-28'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'JP_KENPO') AND date_debut = '2024-03-01';
UPDATE cotisation_taux SET date_fin = '2025-02-28'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'JP_KAIGO') AND date_debut = '2024-03-01';
UPDATE cotisation_taux SET date_fin = '2025-03-31'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'JP_KOYO') AND date_debut = '2024-04-01';

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'JP_KENPO'), '2025-03-01', '2026-02-28', '0.04955', '0.04955', '協会けんぽ Tokyo R7 : 9,91 %.'),
  ((SELECT id FROM cotisation WHERE code = 'JP_KENPO'), '2026-03-01', NULL,         '0.04925', '0.04925', '協会けんぽ Tokyo R8 : 9,85 %.'),
  ((SELECT id FROM cotisation WHERE code = 'JP_KAIGO'), '2025-03-01', '2026-02-28', '0.00795', '0.00795', '介護保険 R7 : 1,59 %.'),
  ((SELECT id FROM cotisation WHERE code = 'JP_KAIGO'), '2026-03-01', NULL,         '0.0081',  '0.0081',  '介護保険 R8 : 1,62 %.'),
  ((SELECT id FROM cotisation WHERE code = 'JP_KOYO'),  '2025-04-01', '2026-03-31', '0.0055',  '0.0090',  '雇用保険 R7 一般 : 5,5/1000 + 9/1000.'),
  ((SELECT id FROM cotisation WHERE code = 'JP_KOYO'),  '2026-04-01', NULL,         '0.0050',  '0.0085',  '雇用保険 R8 一般 : 5/1000 + 8,5/1000 — MHLW.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('JP_KODOMO', '子ども・子育て支援金 — Contribution enfance et parentalité',
   (SELECT id FROM organisme WHERE code = 'JP_KENPO'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   'Perçue avec l''assurance maladie dès avril 2026 : 0,23 % (0,115 % chacun), même assiette. 子ども・子育て支援法.');
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'JP_KODOMO'), '2026-04-01', NULL, '0.00115', '0.00115', '子ども・子育て支援金 R8 : 0,23 %.');
