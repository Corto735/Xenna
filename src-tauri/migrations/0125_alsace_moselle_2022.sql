-- ============================================================
-- ALSACE-MOSELLE : date de la baisse du taux corrigée
-- ============================================================
-- 0008 datait le passage de 1,50 % à 1,30 % du 01/07/2018 (LFSS 2018, par
-- confusion avec la suppression de la cotisation maladie du régime général).
-- Le taux est fixé par le conseil d'administration du régime local : 1,50 %
-- du 01/01/2012 au 31/03/2022, 1,30 % depuis le 01/04/2022, maintenu en 2026
-- (CA du 19/12/2025). Les bulletins de juillet 2018 à mars 2022 étaient donc
-- calculés 0,20 point trop bas.
-- Sources : regime-local.fr/cotisation ; Revue Fiduciaire (avril 2022, 2026).
UPDATE cotisation_taux SET date_fin = '2022-03-31'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'ALSACE_MOSELLE_MALADIE')
  AND date_debut = '2015-01-01';

UPDATE cotisation_taux SET date_debut = '2022-04-01'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'ALSACE_MOSELLE_MALADIE')
  AND date_debut = '2018-07-01';
