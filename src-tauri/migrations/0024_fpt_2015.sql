-- ============================================================
-- FPT — Taux CNRACL 2015
--
-- La migration 0015 démarre au 01/01/2016.
-- Cette migration couvre 2015 pour les simulations historiques FPT.
--
-- Taux issu du décret n°2011-291 (plan de montée en charge 2012-2019) :
--   2013 : 9,40 %  2014 : 9,67 %  2015 : 9,94 %  2016 : 10,29 %
-- Le taux employeur (collectivités) est stable à 30,65 % sur toute la période.
-- ============================================================

INSERT INTO cotisation_taux
  (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes)
VALUES
  ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'),
   '2015-01-01', '2016-01-01', '0.0994', '0.3065',
   (SELECT id FROM texte_loi WHERE code = 'DECRET_2011_291'),
   'Montée en charge 2015. Agent : 9,94 %. Collectivité : 30,65 %.');
