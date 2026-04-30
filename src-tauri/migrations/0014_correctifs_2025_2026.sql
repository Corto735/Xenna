-- ============================================================
-- MIGRATION 0014 — Correctifs 2025-2026
-- 1. PMSS 2026 et PASS 2026 (valeurs manquantes en migration 0007)
-- 2. Correction du calcul Fillon :
--    – 2019-2025 : formule linéaire, seuil 1,6 × SMIC (inchangé depuis 2003)
--    – 2026      : nouvelle formule puissance, seuil 3 × SMIC
-- ============================================================

-- ── 1. PMSS 2026 ────────────────────────────────────────────────────────────
-- La migration 0003 a la période 2025-01-01 → NULL.
-- On la ferme puis on ajoute 2026.
-- Valeur 2026 : arrêté du xx/12/2025 (à vérifier ; estimation +2,0% sur 3 925 €)
UPDATE plafond_reference
SET date_fin = '2025-12-31'
WHERE code = 'PMSS' AND date_debut = '2025-01-01' AND date_fin IS NULL;

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('PMSS', '2026-01-01', NULL, '4004.00', 'MENSUEL');  -- à confirmer contre arrêté officiel

-- ── 2. PASS 2026 ────────────────────────────────────────────────────────────
UPDATE plafond_reference
SET date_fin = '2025-12-31'
WHERE code = 'PASS' AND date_debut = '2025-01-01' AND date_fin IS NULL;

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('PASS', '2026-01-01', NULL, '48048.00', 'ANNUEL');  -- = 12 × 4 004 €

-- ── 3. Correction formule Fillon 2019-2025 ──────────────────────────────────
-- Migration 0005 avait appliqué la formule puissance (seuil=3, tmin, puissance)
-- à la période 2019-01-01 → NULL, ce qui est INCORRECT pour 2019-2025.
--
-- Jusqu'au 31/12/2025, la réduction générale (ex-Fillon) utilise la formule
-- LINÉAIRE avec seuil = 1,6 × SMIC (inchangé depuis la loi Fillon du 17/01/2003) :
--
--   C = (T / 0,6) × (1,6 × SMIC_annualisé / Rémunération_annualisée − 1)
--   T = 0,3981 (depuis la réforme de 2019 — intégration AGIRC-ARRCO)
--   Borné [0 ; T] — s'annule à 1,6 × SMIC
--
-- Correctif : on remet la ligne 2019 à la formule linéaire (tmin=NULL, puissance=NULL)
-- et on limite sa période à 2019-2025.
UPDATE allegement_param
SET date_fin        = '2025-12-31',
    seuil_smic_coeff = '1.6',
    tmin             = NULL,
    puissance        = NULL,
    coeff_max_fillon = '0.3981',
    formule_texte    = 'C = (T / 0,6) × (1,6 × SMIC_annualisé / Rémun._annualisée − 1) ; T=0,3981 ; s''annule à 1,6 × SMIC'
WHERE allegement_type_id = (SELECT id FROM allegement_type WHERE code = 'FILLON')
  AND date_debut = '2019-01-01';

-- ── 4. Nouvelle formule Fillon 2026 ─────────────────────────────────────────
-- À partir du 01/01/2026, la formule de la réduction générale évolue :
-- introduction d'un plancher (Tmin = 2 %) et extension du seuil à 3 × SMIC.
--
--   C = Tmin + (Tdelta × D^P)
--   D = (1/2) × (3 × SMIC_annualisé / Rémunération_annualisée − 1)
--   Tmin = 0,0200  Tdelta = 0,3781  Tmax = 0,3981  P = 1,75
--
--   D = 0 → C = Tmin à 3 × SMIC ; D = 1 → C = Tmax au SMIC.
INSERT INTO allegement_param
    (allegement_type_id, date_debut, date_fin,
     coeff_max_fillon, seuil_smic_coeff, tmin, puissance,
     formule_texte, texte_loi_id)
VALUES (
    (SELECT id FROM allegement_type WHERE code = 'FILLON'),
    '2026-01-01', NULL,
    '0.3981', '3.0', '0.0200', '1.75',
    'C = Tmin + (Tdelta × D^P) ; D=(1/2)×(3×SMIC/Rémun.−1) ; Tmin=0,0200 ; Tdelta=0,3781 ; Tmax=0,3981 ; P=1,75 ; s''annule au-delà de 3 × SMIC',
    NULL
);
