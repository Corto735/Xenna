-- ============================================================
-- ALERTES DE COHÉRENCE DU BULLETIN — seuils (calculs/alertes.rs)
--
-- Durée minimale du temps partiel : 24 heures par semaine (art. L3123-27 du
-- code du travail, issu de la loi n° 2013-504 du 14 juin 2013 ; entrée en
-- vigueur générale au 1er juillet 2014). Des dérogations existent (demande
-- écrite du salarié, accord de branche, étudiants…) : l'alerte informe, elle
-- ne conclut pas à une irrégularité.
--
-- Le contrôle « salaire de base ≥ SMIC » réutilise SMIC_HORAIRE, déjà en base.
-- ============================================================
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('TEMPS_PARTIEL_MIN_HEBDO', '2014-07-01', NULL, '24', 'HORAIRE');
