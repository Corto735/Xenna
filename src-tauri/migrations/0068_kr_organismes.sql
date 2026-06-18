-- 0068 — Corée du Sud : organismes, textes de loi et 4 assurances sociales (4대보험) 2025
-- Périmètre : salarié secteur privé. Devise KRW. Données : 2025.
--
-- Côté salarié : 국민연금 (pension 4,5 %, plafonnée), 건강보험 (santé 3,545 %)
--   + 장기요양 (dépendance = 12,95 % de la prime santé), 고용보험 (emploi 0,9 %),
--   + 소득세 (impôt progressif) + 지방소득세 (impôt local = 10 % de l'impôt).
-- 산재보험 (accidents) : 100 % patronal. Taux santé/pension/emploi lus en base ;
-- 장기요양 et impôt calculés en Rust (kr_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('KR_NPS',  '국민연금공단 / National Pension Service — retraite',                 'https://www.nps.or.kr'),
  ('KR_NHIS', '국민건강보험공단 / National Health Insurance Service — santé + 장기요양', 'https://www.nhis.or.kr'),
  ('KR_MOEL', '고용노동부 / Ministry of Employment and Labor — 고용보험 + 산재보험',   'https://www.moel.go.kr'),
  ('KR_NTS',  '국세청 / National Tax Service — 소득세',                              'https://www.nts.go.kr');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('KR_NPS_LAW',  'LOI', '국민연금법 / National Pension Act', '법률 제3902호', '1986-12-31', '1988-01-01',
   'https://www.law.go.kr/법령/국민연금법',
   '국민연금 : 9 % total (4,5 % sal + 4,5 % pat) en 2025. Plafond mensuel 상한액 : 6 370 000 ₩ (juil. 2025-juin 2026).'),
  ('KR_NHIS_LAW', 'LOI', '국민건강보험법 / National Health Insurance Act', '법률 제5854호', '1999-02-08', '2000-01-01',
   'https://www.law.go.kr/법령/국민건강보험법',
   '건강보험 2025 : 7,09 % total (3,545 % chacun). 장기요양 : 12,95 % de la prime santé (노인장기요양보험법).'),
  ('KR_EI_LAW',   'LOI', '고용보험법 / Employment Insurance Act', '법률 제4644호', '1993-12-27', '1995-07-01',
   'https://www.law.go.kr/법령/고용보험법',
   '고용보험 2025 : salarié 0,9 % (assurance chômage). Employeur 0,9 % + stabilisation/formation (≈ 0,25 % pour les petites entreprises).'),
  ('KR_IT_LAW',   'LOI', '소득세법 / Income Tax Act', '법률 제4661호', '1994-12-22', '1995-01-01',
   'https://www.law.go.kr/법령/소득세법',
   '소득세 : 8 tranches de 6 % à 45 %. Déduction 근로소득공제 + 기본공제 1 500 000 ₩ + crédit 근로소득세액공제. 지방소득세 = 10 % de l''impôt.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('KR_NPS', '국민연금 — Pension nationale',
   (SELECT id FROM organisme WHERE code = 'KR_NPS'), 'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025 : 4,5 % sal + 4,5 % pat. Plafond mensuel 6 370 000 ₩.'),
  ('KR_NHI', '건강보험 — Assurance santé',
   (SELECT id FROM organisme WHERE code = 'KR_NHIS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 3,545 % sal + 3,545 % pat.'),
  ('KR_EI', '고용보험 — Assurance emploi',
   (SELECT id FROM organisme WHERE code = 'KR_MOEL'), 'CHOMAGE', 1, 1, 'BRUT_TOTAL',
   '2025 : salarié 0,9 %, employeur 1,15 % (0,9 % chômage + 0,25 % stabilisation/formation, petites entreprises).'),
  ('KR_SANJAE', '산재보험 — Accidents du travail (employeur)',
   (SELECT id FROM organisme WHERE code = 'KR_MOEL'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 100 % patronal. Taux moyen ≈ 0,7 % (variable par secteur).');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'KR_NPS'),    '2025-01-01', NULL, '0.045',  '0.045',
   (SELECT id FROM texte_loi WHERE code = 'KR_NPS_LAW'),  '국민연금 2025 : 4,5 % chacun.'),
  ((SELECT id FROM cotisation WHERE code = 'KR_NHI'),    '2025-01-01', NULL, '0.03545','0.03545',
   (SELECT id FROM texte_loi WHERE code = 'KR_NHIS_LAW'), '건강보험 2025 : 3,545 % chacun.'),
  ((SELECT id FROM cotisation WHERE code = 'KR_EI'),     '2025-01-01', NULL, '0.009',  '0.0115',
   (SELECT id FROM texte_loi WHERE code = 'KR_EI_LAW'),   '고용보험 2025 : 0,9 % sal / 1,15 % pat.'),
  ((SELECT id FROM cotisation WHERE code = 'KR_SANJAE'), '2025-01-01', NULL, '0',      '0.007',
   (SELECT id FROM texte_loi WHERE code = 'KR_EI_LAW'),   '산재보험 2025 : ≈ 0,7 % pat (moyen).');
