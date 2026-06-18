-- 0081 — Corée du Sud : backfill cotisations 2024 (taux identiques à 2025)
-- En 2024 : 건강보험 7,09 % (3,545 % chacun, gelé 2024-2025), 장기요양 12,95 % de la prime santé,
-- 국민연금 4,5 %, 고용보험 0,9 % sal — identiques à 2025. Barème 소득세 inchangé depuis 2023.
-- Source : NHIS/NPS/MOEL 2024.

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'KR_NPS'),    '2024-01-01', '2025-01-01', '0.045',  '0.045',
   (SELECT id FROM texte_loi WHERE code = 'KR_NPS_LAW'),  '국민연금 2024 : 4,5 % chacun.'),
  ((SELECT id FROM cotisation WHERE code = 'KR_NHI'),    '2024-01-01', '2025-01-01', '0.03545','0.03545',
   (SELECT id FROM texte_loi WHERE code = 'KR_NHIS_LAW'), '건강보험 2024 : 3,545 % chacun (gelé).'),
  ((SELECT id FROM cotisation WHERE code = 'KR_EI'),     '2024-01-01', '2025-01-01', '0.009',  '0.0115',
   (SELECT id FROM texte_loi WHERE code = 'KR_EI_LAW'),   '고용보험 2024 : 0,9 % sal / 1,15 % pat.'),
  ((SELECT id FROM cotisation WHERE code = 'KR_SANJAE'), '2024-01-01', '2025-01-01', '0',      '0.007',
   (SELECT id FROM texte_loi WHERE code = 'KR_EI_LAW'),   '산재보험 2024 : ≈ 0,7 % pat (moyen).');
