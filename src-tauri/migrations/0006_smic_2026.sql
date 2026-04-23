-- ============================================================
-- SMIC 2026 — en vigueur au 1er janvier 2026
-- Source : décret n° 2025-… (JO du …)
-- Mensuel brut : 1 823,03 € (base 35h/semaine, 151,67 h/mois)
-- Horaire      :    12,02 € (= 1 823,03 / 151,67)
-- ============================================================

-- Fermeture de la période 2025 (date_fin IS NULL → limité au 31/12/2025)
UPDATE plafond_reference
SET date_fin = '2025-12-31'
WHERE code = 'SMIC_MENSUEL' AND date_debut = '2025-01-01' AND date_fin IS NULL;

UPDATE plafond_reference
SET date_fin = '2025-12-31'
WHERE code = 'SMIC_HORAIRE' AND date_debut = '2025-01-01' AND date_fin IS NULL;

-- Nouvelles valeurs 2026 (date_fin IS NULL = en vigueur indéfiniment)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('SMIC_MENSUEL', '2026-01-01', NULL, '1823.03', 'MENSUEL'),
    ('SMIC_HORAIRE', '2026-01-01', NULL, '12.02',   'HORAIRE');
