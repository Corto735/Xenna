-- ============================================================
-- SMIC : revalorisation de 2 % au 1er NOVEMBRE 2024 (et non au 1er décembre)
--
-- Décret n° 2024-951 du 23 octobre 2024 : SMIC horaire 11,88 € et mensuel
-- 1 801,80 € (151,67 h) à compter du 1er novembre 2024. Le seed 0003 datait
-- cette période du 1er décembre : une paie de novembre 2024 retenait encore
-- 11,65 € / 1 766,92 €.
-- En 2024 la revalorisation n'était pas neutralisée pour la réduction
-- générale (le SMIC « gelé au 1er janvier » ne vaut qu'à partir de 2025) :
-- le SMIC de novembre 2024 vaut aussi pour son calcul (Éditions Tissot,
-- « Réduction générale : comment prendre en compte la hausse du SMIC au
-- 1er novembre 2024 »).
-- Le trigger anti-chevauchement ne surveille que les INSERT : on raccourcit
-- d'abord la période précédente, puis on avance la suivante.
-- ============================================================
UPDATE plafond_reference SET date_fin = '2024-11-01'
 WHERE code IN ('SMIC_HORAIRE', 'SMIC_MENSUEL') AND date_debut = '2024-01-01' AND date_fin = '2024-12-01';
UPDATE plafond_reference SET date_debut = '2024-11-01'
 WHERE code IN ('SMIC_HORAIRE', 'SMIC_MENSUEL') AND date_debut = '2024-12-01';
