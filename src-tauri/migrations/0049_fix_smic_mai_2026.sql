-- ============================================================
-- CORRECTIF migration 0047 — SMIC_MENSUEL et SMIC_HORAIRE
--
-- Cause : la migration 0047 a fermé la période jan-mai 2026
-- avec date_fin = '2026-05-31' (convention inclusive), alors que
-- la requête dans context.rs utilise :
--
--   AND (date_fin IS NULL OR date_fin > ?)   ← borne EXCLUSIVE
--
-- Conséquence : le 31/05/2026 tombait dans un trou — aucun taux
-- trouvé → "SMIC_MENSUEL introuvable pour cette date".
--
-- Correction : date_fin passe à '2026-06-01' (premier jour où le
-- taux n'est plus valide), ce qui rend le 31/05 couvert.
-- ============================================================

UPDATE plafond_reference
SET date_fin = '2026-06-01'
WHERE code = 'SMIC_MENSUEL'
  AND date_debut = '2026-01-01'
  AND date_fin   = '2026-05-31';

UPDATE plafond_reference
SET date_fin = '2026-06-01'
WHERE code = 'SMIC_HORAIRE'
  AND date_debut = '2026-01-01'
  AND date_fin   = '2026-05-31';
