-- 0082 — Danemark : backfill AM-bidrag 2024 (taux stable 8 %)
-- L'AM-bidrag est 8 % depuis 2011 (inchangé). On étend la couverture à 2024.
-- Les paramètres d'impôt 2024 (personfradrag 49 700, topskat 588 900, kommune moyen 25,067 %)
-- sont gérés par année dans dk_bulletin.rs. Source : Skattestyrelsen 2024.

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'DK_AM'), '2024-01-01', '2025-01-01', '0.08', '0',
   (SELECT id FROM texte_loi WHERE code = 'DK_AMBIDRAG_LOV'), 'AM-bidrag 2024 : 8 % salarié.');
