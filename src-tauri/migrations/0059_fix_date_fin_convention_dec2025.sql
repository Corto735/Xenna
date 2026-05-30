-- ============================================================
-- CORRECTIF — Convention date_fin exclusive (migrations 0006 + 0014)
--
-- Cause : les migrations 0006 et 0014 ont fermé les périodes 2025
-- avec date_fin = '2025-12-31' (borne inclusive), alors que
-- context.rs utilise :
--
--   AND (date_fin IS NULL OR date_fin > ?)   ← borne EXCLUSIVE
--
-- Conséquence : le 31/12/2025 tombait dans un trou pour
-- SMIC_MENSUEL, SMIC_HORAIRE, PMSS, PASS et allegement_param Fillon.
-- Aucun taux trouvé → erreur au calcul pour cette date.
--
-- Correction : date_fin passe à '2026-01-01' (premier jour où le
-- taux n'est plus valide), ce qui couvre le 31/12/2025.
-- ============================================================

-- ── plafond_reference ───────────────────────────────────────────────────────

UPDATE plafond_reference
SET date_fin = '2026-01-01'
WHERE code = 'SMIC_MENSUEL'
  AND date_debut = '2025-01-01'
  AND date_fin   = '2025-12-31';

UPDATE plafond_reference
SET date_fin = '2026-01-01'
WHERE code = 'SMIC_HORAIRE'
  AND date_debut = '2025-01-01'
  AND date_fin   = '2025-12-31';

UPDATE plafond_reference
SET date_fin = '2026-01-01'
WHERE code = 'PMSS'
  AND date_debut = '2025-01-01'
  AND date_fin   = '2025-12-31';

UPDATE plafond_reference
SET date_fin = '2026-01-01'
WHERE code = 'PASS'
  AND date_debut = '2025-01-01'
  AND date_fin   = '2025-12-31';

-- ── allegement_param (Fillon 2019-2025) ─────────────────────────────────────

UPDATE allegement_param
SET date_fin = '2026-01-01'
WHERE allegement_type_id = (SELECT id FROM allegement_type WHERE code = 'FILLON')
  AND date_debut = '2019-01-01'
  AND date_fin   = '2025-12-31';
