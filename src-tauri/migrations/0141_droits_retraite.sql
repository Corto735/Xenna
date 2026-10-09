-- ============================================================
-- DROITS À LA RETRAITE OUVERTS PAR LE MOIS — paramètres
-- (calculs/droits.rs : encart « ce que ce mois vous ouvre comme droits »)
--
-- 1. Agirc-Arrco (régime unifié depuis le 01/01/2019)
--    Points acquis = assiette × taux de calcul des points ÷ prix d'achat du
--    point (« salaire de référence »). Les cotisations sont APPELÉES à 127 %
--    de ce taux (7,87 % = 6,20 % × 127 % en tranche 1 ; 21,59 % = 17 % × 127 %
--    en tranche 2) : les 27 % appelés en plus n'ouvrent aucun point.
--    Source des valeurs : Agirc-Arrco, « Valeurs de service du point et
--    salaires de référence », compilation au 1er novembre 2025
--    https://www.agirc-arrco.fr/storage/2024/10/Compilation_valeurs_de_point_novembre_2025.pdf
--    (2026 : valeurs inchangées, circulaire Agirc-Arrco 2025-15-DT du 23/10/2025).
--    Valeur de service : revalorisée au 1er novembre ; prix d'achat : au
--    1er janvier. La valeur de service n'est affichée qu'à titre indicatif
--    (ce que ces points rapporteraient par an, à la valeur d'aujourd'hui).
--
-- 2. Validation des trimestres (régime général)
--    Un trimestre par tranche de salaire soumis à cotisation vieillesse égale
--    à 150 heures au SMIC horaire en vigueur au 1er janvier de l'année, dans
--    la limite de 4 par an (art. R351-9 du code de la sécurité sociale, dans
--    sa rédaction issue du décret n° 2014-349 du 19 mars 2014). Le SMIC
--    horaire est déjà en base (SMIC_HORAIRE).
-- ============================================================
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('AA_PRIX_ACHAT_POINT', '2019-01-01', '2020-01-01', '17.0571', 'ANNUEL'),
    ('AA_PRIX_ACHAT_POINT', '2020-01-01', '2022-01-01', '17.3982', 'ANNUEL'),
    ('AA_PRIX_ACHAT_POINT', '2022-01-01', '2023-01-01', '17.4316', 'ANNUEL'),
    ('AA_PRIX_ACHAT_POINT', '2023-01-01', '2024-01-01', '18.7669', 'ANNUEL'),
    ('AA_PRIX_ACHAT_POINT', '2024-01-01', '2025-01-01', '19.6321', 'ANNUEL'),
    ('AA_PRIX_ACHAT_POINT', '2025-01-01', NULL,         '20.1877', 'ANNUEL'),

    ('AA_VALEUR_POINT', '2019-01-01', '2019-11-01', '1.2588', 'ANNUEL'),
    ('AA_VALEUR_POINT', '2019-11-01', '2021-11-01', '1.2714', 'ANNUEL'),
    ('AA_VALEUR_POINT', '2021-11-01', '2022-11-01', '1.2841', 'ANNUEL'),
    ('AA_VALEUR_POINT', '2022-11-01', '2023-11-01', '1.3498', 'ANNUEL'),
    ('AA_VALEUR_POINT', '2023-11-01', '2024-11-01', '1.4159', 'ANNUEL'),
    ('AA_VALEUR_POINT', '2024-11-01', NULL,         '1.4386', 'ANNUEL'),

    ('AA_TAUX_POINTS_T1', '2019-01-01', NULL, '0.062', 'MENSUEL'),
    ('AA_TAUX_POINTS_T2', '2019-01-01', NULL, '0.17',  'MENSUEL'),

    ('RETRAITE_HEURES_TRIMESTRE', '2014-01-01', NULL, '150', 'HORAIRE');
