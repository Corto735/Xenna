-- ============================================================
-- AVANTAGES EN NATURE — BARÈMES FORFAITAIRES (régime général)
-- Arrêté du 25/02/2025 (en vigueur le 01/02/2025, remplace l'arrêté du
-- 10/12/2002), montants revalorisés au 1er janvier (URSSAF / BOSS).
--   AN_REPAS            : forfait par repas (journée = 2 repas).
--   AN_LOG_1P_Tn        : logement d'une pièce, tranche n de rémunération.
--   AN_LOG_PP_Tn        : logement de plusieurs pièces, montant PAR pièce
--                         principale, tranche n.
--     Tranches (rémunération brute mensuelle hors avantages, en PSS) :
--     T1 < 0,5 ; T2 < 0,6 ; T3 < 0,7 ; T4 < 0,9 ; T5 < 1,1 ; T6 < 1,3 ;
--     T7 < 1,5 ; T8 ≥ 1,5.
--   AN_VE_PLAF_70       : plafond annuel de l'abattement de 70 % (véhicule
--                         100 % électrique mis à disposition du 01/02/2025
--                         au 31/12/2027, éco-score requis).
--   AN_VE_PLAF_50       : plafond annuel de l'abattement de 50 % (véhicule
--                         électrique mis à disposition avant le 01/02/2025),
--                         dans sa version de l'arrêté du 25/02/2025 (dès le
--                         01/02/2025 ; janvier 2025 non intégré).
-- Sources : Légifrance (arrêté du 25/02/2025) ; LégiSocial, barèmes 2025 et
-- 2026 (repas, logement, véhicule). Avant 2025 : non intégré.
-- « MENSUEL » ne sert qu'à satisfaire le CHECK du schéma ; date_fin exclusive.
-- ============================================================
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('AN_REPAS',     '2025-01-01', '2026-01-01', '5.45',  'MENSUEL'),
    ('AN_REPAS',     '2026-01-01', NULL,         '5.50',  'MENSUEL'),
    ('AN_LOG_1P_T1', '2025-01-01', '2026-01-01', '78.70', 'MENSUEL'),
    ('AN_LOG_1P_T2', '2025-01-01', '2026-01-01', '91.80', 'MENSUEL'),
    ('AN_LOG_1P_T3', '2025-01-01', '2026-01-01', '104.80','MENSUEL'),
    ('AN_LOG_1P_T4', '2025-01-01', '2026-01-01', '117.90','MENSUEL'),
    ('AN_LOG_1P_T5', '2025-01-01', '2026-01-01', '144.50','MENSUEL'),
    ('AN_LOG_1P_T6', '2025-01-01', '2026-01-01', '170.40','MENSUEL'),
    ('AN_LOG_1P_T7', '2025-01-01', '2026-01-01', '196.80','MENSUEL'),
    ('AN_LOG_1P_T8', '2025-01-01', '2026-01-01', '222.70','MENSUEL'),
    ('AN_LOG_PP_T1', '2025-01-01', '2026-01-01', '42.10', 'MENSUEL'),
    ('AN_LOG_PP_T2', '2025-01-01', '2026-01-01', '58.90', 'MENSUEL'),
    ('AN_LOG_PP_T3', '2025-01-01', '2026-01-01', '78.70', 'MENSUEL'),
    ('AN_LOG_PP_T4', '2025-01-01', '2026-01-01', '98.20', 'MENSUEL'),
    ('AN_LOG_PP_T5', '2025-01-01', '2026-01-01', '124.50','MENSUEL'),
    ('AN_LOG_PP_T6', '2025-01-01', '2026-01-01', '150.40','MENSUEL'),
    ('AN_LOG_PP_T7', '2025-01-01', '2026-01-01', '183.30','MENSUEL'),
    ('AN_LOG_PP_T8', '2025-01-01', '2026-01-01', '209.60','MENSUEL'),
    ('AN_LOG_1P_T1', '2026-01-01', NULL,         '79.70', 'MENSUEL'),
    ('AN_LOG_1P_T2', '2026-01-01', NULL,         '93.00', 'MENSUEL'),
    ('AN_LOG_1P_T3', '2026-01-01', NULL,         '106.20','MENSUEL'),
    ('AN_LOG_1P_T4', '2026-01-01', NULL,         '119.40','MENSUEL'),
    ('AN_LOG_1P_T5', '2026-01-01', NULL,         '146.40','MENSUEL'),
    ('AN_LOG_1P_T6', '2026-01-01', NULL,         '172.60','MENSUEL'),
    ('AN_LOG_1P_T7', '2026-01-01', NULL,         '199.40','MENSUEL'),
    ('AN_LOG_1P_T8', '2026-01-01', NULL,         '225.60','MENSUEL'),
    ('AN_LOG_PP_T1', '2026-01-01', NULL,         '42.60', 'MENSUEL'),
    ('AN_LOG_PP_T2', '2026-01-01', NULL,         '59.70', 'MENSUEL'),
    ('AN_LOG_PP_T3', '2026-01-01', NULL,         '79.70', 'MENSUEL'),
    ('AN_LOG_PP_T4', '2026-01-01', NULL,         '99.50', 'MENSUEL'),
    ('AN_LOG_PP_T5', '2026-01-01', NULL,         '126.10','MENSUEL'),
    ('AN_LOG_PP_T6', '2026-01-01', NULL,         '152.40','MENSUEL'),
    ('AN_LOG_PP_T7', '2026-01-01', NULL,         '185.70','MENSUEL'),
    ('AN_LOG_PP_T8', '2026-01-01', NULL,         '212.30','MENSUEL'),
    ('AN_VE_PLAF_70', '2025-02-01', '2026-01-01', '4582.00', 'ANNUEL'),
    ('AN_VE_PLAF_70', '2026-01-01', NULL,         '4641.60', 'ANNUEL'),
    ('AN_VE_PLAF_50', '2025-02-01', '2026-01-01', '2000.30', 'ANNUEL'),
    ('AN_VE_PLAF_50', '2026-01-01', NULL,         '2026.30', 'ANNUEL');
