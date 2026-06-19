-- ============================================================
-- ENTREPRISE ADAPTÉE (EA) — AIDE AU POSTE (aide socle)
-- Forfait annuel par ETP versé à l'employeur par l'ASP au titre de
-- l'emploi d'un travailleur handicapé (RQTH).
-- Montants métropole, arrêté du 16/01/2025 (en vigueur depuis le 01/11/2024,
-- aligné sur la revalorisation du SMIC).
-- Source : CT art. L5213-19 / R5213-76 ; arrêté du 16/01/2025 (JORF).
-- Stockés comme plafonds de référence (valeur TEXT pour précision exacte).
-- ============================================================
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('AIDE_POSTE_EA_M50',   '2024-11-01', NULL, '18230.00', 'ANNUEL'),  -- < 50 ans
    ('AIDE_POSTE_EA_50_55', '2024-11-01', NULL, '18465.00', 'ANNUEL'),  -- 50 à 55 ans
    ('AIDE_POSTE_EA_56P',   '2024-11-01', NULL, '18941.00', 'ANNUEL');  -- 56 ans et +
