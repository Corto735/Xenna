-- ============================================================
-- Dates de fin : une seule convention, EXCLUSIVE
--
-- Le moteur (db/context.rs) retient une période si date_debut ≤ date de paie
-- < date_fin. Or une partie des migrations a écrit des dates de fin
-- INCLUSIVES (« 2017-12-31 » suivi d'une période au « 2018-01-01 ») : une
-- paie datée du dernier jour de la période ne trouvait alors AUCUN taux, et
-- la cotisation disparaissait du bulletin (ex. maladie Japon au 28/02/2026,
-- 87 taux et 105 plafonds concernés).
-- Règle : une date de fin au dernier jour d'un mois est inclusive → reportée
-- au premier jour du mois suivant. Toute migration future écrit des dates de
-- fin exclusives.
-- ============================================================
UPDATE cotisation_taux SET date_fin = date(date_fin, '+1 day')
 WHERE date_fin IS NOT NULL AND strftime('%d', date(date_fin, '+1 day')) = '01';
UPDATE plafond_reference SET date_fin = date(date_fin, '+1 day')
 WHERE date_fin IS NOT NULL AND strftime('%d', date(date_fin, '+1 day')) = '01';
UPDATE allegement_param SET date_fin = date(date_fin, '+1 day')
 WHERE date_fin IS NOT NULL AND strftime('%d', date(date_fin, '+1 day')) = '01';
UPDATE irpef_tranche SET date_fin = date(date_fin, '+1 day')
 WHERE date_fin IS NOT NULL AND strftime('%d', date(date_fin, '+1 day')) = '01';
UPDATE irpef_detrazione_lavdip SET date_fin = date(date_fin, '+1 day')
 WHERE date_fin IS NOT NULL AND strftime('%d', date(date_fin, '+1 day')) = '01';
