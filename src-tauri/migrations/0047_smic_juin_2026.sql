-- ============================================================
-- SMIC JUIN 2026 — revalorisation au 1er juin 2026
-- Mensuel brut : 1 867,02 € (base 35h/semaine, 151,67 h/mois)
-- Horaire      :    12,31 € (= 1 867,02 / 151,67 ≈ 12,309…)
-- ============================================================
-- Impact automatique sur la réduction Fillon :
-- ContextPaie lit SMIC_MENSUEL par date_paie — pas de modification
-- d'allegement_param nécessaire (Tmin=0,0200, Tdelta=0,3781,
-- P=1,75, Seuil=3×SMIC restent valides depuis le 01/01/2026).
-- Le nouveau seuil d'extinction Fillon sera 3 × 1 867,02 = 5 601,06 €.
-- ============================================================

-- Fermeture de la période janvier-mai 2026
UPDATE plafond_reference
SET date_fin = '2026-05-31'
WHERE code = 'SMIC_MENSUEL' AND date_debut = '2026-01-01' AND date_fin IS NULL;

UPDATE plafond_reference
SET date_fin = '2026-05-31'
WHERE code = 'SMIC_HORAIRE' AND date_debut = '2026-01-01' AND date_fin IS NULL;

-- Nouvelles valeurs à compter du 1er juin 2026 (date_fin IS NULL = en vigueur indéfiniment)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('SMIC_MENSUEL', '2026-06-01', NULL, '1867.02', 'MENSUEL'),
    ('SMIC_HORAIRE', '2026-06-01', NULL, '12.31',   'HORAIRE');
