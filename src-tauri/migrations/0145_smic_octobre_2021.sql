-- ============================================================
-- SMIC : revalorisation automatique de 2,2 % au 1er OCTOBRE 2021
-- (et non au 1er août)
--
-- Arrêté du 27 septembre 2021 relatif au relèvement du salaire minimum de
-- croissance : SMIC horaire 10,48 € et mensuel 1 589,47 € (151,67 h) à
-- compter du 1er octobre 2021 (hausse de l'indice des prix du premier
-- quintile supérieure à 2 %, art. L3231-5 C. trav.). Le seed 0003 datait
-- cette période du 1er août : les paies d'août et de septembre 2021
-- retenaient 10,48 € au lieu de 10,25 €.
-- Comme en 2024 (migration 0143), la revalorisation en cours d'année n'était
-- pas neutralisée pour la réduction générale : la valeur du mois s'applique.
-- Le trigger anti-chevauchement ne surveille que les INSERT : on raccourcit
-- d'abord la période précédente, puis on recule le début de la suivante.
-- ============================================================
UPDATE plafond_reference SET date_fin = '2021-10-01'
 WHERE code IN ('SMIC_HORAIRE', 'SMIC_MENSUEL') AND date_debut = '2021-01-01' AND date_fin = '2021-08-01';
UPDATE plafond_reference SET date_debut = '2021-10-01'
 WHERE code IN ('SMIC_HORAIRE', 'SMIC_MENSUEL') AND date_debut = '2021-08-01';
