-- ============================================================
-- PLAFONDS DE COTISATIONS EN BASE — lot 4 (CA, QC, DE, CH, IT, LU)
--
-- Les modules lisaient des plafonds écrits en dur ; la plupart existaient déjà
-- ici (migrations 0017 à 0028) sans être lus, avec quelques écarts. Cette
-- migration aligne la base, que le code lit désormais.
--
-- Canada / Québec : année 2026 ajoutée (MGA 74 600, MGAP2 85 000, MAGA 68 900,
--   MAGA RQAP 103 000) ; MAGA RQAP 2025 = 98 000 $ (la base portait 97 500).
-- Allemagne : les BBG finissaient au 31/12 alors que date_fin est exclusive
--   (même défaut que celui corrigé par 0059) : une paie datée du 31 décembre
--   n'avait pas de plafond.
-- Suisse : gain maximal assuré LAA/LACI 126 000 CHF jusqu'en 2015, 148 200 CHF
--   dès 2016 (OLAA art. 22 al. 1 ; LACI art. 3 al. 2). Le code appliquait
--   148 200 dès 2015.
-- Italie : massimale contributivo (L. 335/1995, art. 2 c. 18) décalé d'un an
--   dans la base et le code depuis 2022, estimations pour 2025-2026.
--   2022 : 105 014 (circ. INPS 15/2022) ; 2023 : 113 520 (circ. 11/2023) ;
--   2024 : 119 650 ; 2025 : 120 607 (circ. 26/2025) ; 2026 : 122 295 (circ. 6/2026).
-- Luxembourg : plafond 5 × SSM au mois près depuis 2025 (valeurs du code :
--   13 188,95 janv.-avr. 2025, 13 518,68 dès mai 2025 à l'indice 968,04,
--   13 856,63 dès juin 2026 à l'indice 992,24). 2015-2024 inchangées,
--   approximatives, à revérifier contre la CCSS.
-- ============================================================

-- ── Canada / Québec ─────────────────────────────────────────
UPDATE plafond_reference SET date_fin = '2026-01-01'
WHERE code IN ('CA_MGA', 'CA_MAGA', 'CA_MGAP2', 'QC_MAGA_RQAP')
  AND date_debut = '2025-01-01' AND date_fin IS NULL;

UPDATE plafond_reference SET valeur = '98000.00'
WHERE code = 'QC_MAGA_RQAP' AND date_debut = '2025-01-01';

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CA_MGA',       '2026-01-01', NULL, '74600.00',  'ANNUEL'),
  ('CA_MGAP2',     '2026-01-01', NULL, '85000.00',  'ANNUEL'),
  ('CA_MAGA',      '2026-01-01', NULL, '68900.00',  'ANNUEL'),
  ('QC_MAGA_RQAP', '2026-01-01', NULL, '103000.00', 'ANNUEL');

-- ── Allemagne : date_fin exclusive ──────────────────────────
UPDATE plafond_reference SET date_fin = date(date_fin, '+1 day')
WHERE code IN ('DE_BBG_KV', 'DE_BBG_RV') AND date_fin LIKE '%-12-31';

-- ── Suisse : gain maximal assuré LAA/LACI (annuel) ──────────
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CH_GAIN_ASSURE_MAX', '2015-01-01', '2016-01-01', '126000.00', 'ANNUEL'),
  ('CH_GAIN_ASSURE_MAX', '2016-01-01', NULL,         '148200.00', 'ANNUEL');

-- ── Italie : massimale contributivo ─────────────────────────
DELETE FROM plafond_reference WHERE code = 'IT_MASSIMALE_IVS' AND date_debut >= '2022-01-01';
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('IT_MASSIMALE_IVS', '2022-01-01', '2023-01-01', '105014.00', 'ANNUEL'),
  ('IT_MASSIMALE_IVS', '2023-01-01', '2024-01-01', '113520.00', 'ANNUEL'),
  ('IT_MASSIMALE_IVS', '2024-01-01', '2025-01-01', '119650.00', 'ANNUEL'),
  ('IT_MASSIMALE_IVS', '2025-01-01', '2026-01-01', '120607.00', 'ANNUEL'),
  ('IT_MASSIMALE_IVS', '2026-01-01', NULL,         '122295.00', 'ANNUEL');

-- ── Luxembourg : plafond 5 × SSM (mensuel) ──────────────────
DELETE FROM plafond_reference WHERE code = 'LU_PLAFOND_5SSM' AND date_debut >= '2025-01-01';
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('LU_PLAFOND_5SSM', '2025-01-01', '2025-05-01', '13188.95', 'MENSUEL'),
  ('LU_PLAFOND_5SSM', '2025-05-01', '2026-06-01', '13518.68', 'MENSUEL'),
  ('LU_PLAFOND_5SSM', '2026-06-01', NULL,         '13856.63', 'MENSUEL');
