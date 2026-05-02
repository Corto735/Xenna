-- ============================================================
-- SUISSE — Taux historiques 2015-2025 et plafonds LPP OPP 2
--
-- CONTEXTE : la migration 0012 ne contenait que des taux 2026+.
-- Toute simulation antérieure à 2026 retournait 0 % (aucun taux trouvé).
-- Cette migration couvre 2015-2025 et permet les simulations historiques.
--
-- RFFA (Réforme fiscale et financement de l''AVS, RO 2019 2395) :
--   → entrée en vigueur 01/01/2020 : AVS 4,20 % → 4,35 % chacun.
-- Tous les autres taux sont stables sur la période 2015-2026.
-- ============================================================

-- ── AVS : changement de taux au 01/01/2020 (RFFA) ──────────

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='CH_AVS'),
   '2015-01-01', '2020-01-01', '0.0420', '0.0420',
   'Pré-RFFA : total AVS 8,40 %. Taux en vigueur depuis la réforme de 2006.'),
  ((SELECT id FROM cotisation WHERE code='CH_AVS'),
   '2020-01-01', '2026-01-01', '0.0435', '0.0435',
   'Post-RFFA (RO 2019 2395, art. 5 LAVS) : total AVS 8,70 %. +0,15 pp chacun dès 01/01/2020.');

-- ── AI, APG, AC : stables sur toute la période ──────────────

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='CH_AI'),
   '2015-01-01', '2026-01-01', '0.0070', '0.0070',
   'Stable depuis 2006 (réforme 5e révision AI). Art. 3 LAI.'),
  ((SELECT id FROM cotisation WHERE code='CH_APG'),
   '2015-01-01', '2026-01-01', '0.0025', '0.0025',
   'Stable depuis 2011. Congé paternité inclus dès 01/01/2021 (L. 25/09/2020).'),
  ((SELECT id FROM cotisation WHERE code='CH_AC'),
   '2015-01-01', '2026-01-01', '0.0110', '0.0110',
   'Stable depuis la révision LACI 2014. Plafond CHF 148 200/an inchangé.');

-- ── AANP / AAP / IJM / LPP : taux indicatifs stables ────────

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='CH_AANP'),
   '2015-01-01', '2026-01-01', '0.0100', '0.0000',
   'Taux indicatif SUVA bureau. Varie selon assureur et classe de risque. Salarié uniquement.'),
  ((SELECT id FROM cotisation WHERE code='CH_AAP'),
   '2015-01-01', '2026-01-01', '0.0000', '0.0100',
   'Taux indicatif SUVA bureau. Varie selon assureur et classe de risque. Employeur uniquement.'),
  ((SELECT id FROM cotisation WHERE code='CH_IJM'),
   '2015-01-01', '2026-01-01', '0.0075', '0.0075',
   'Plan collectif type. Varie selon assureur, sinistralité et garanties.'),
  ((SELECT id FROM cotisation WHERE code='CH_LPP'),
   '2015-01-01', '2026-01-01', '0.0500', '0.0500',
   'Taux minimum légal art. 16 LPP pour tranche 35-44 ans (10 % total). Inchangé depuis 2005. Les plafonds OPP 2 (seuil entrée, déduction coord.) évoluent tous les 2 ans — voir ci-dessous.');

-- ============================================================
-- LPP — Plafonds OPP 2 (seuil d'entrée et déduction de coordination)
-- Revalorisés tous les 2 ans par le Conseil fédéral.
-- Source : OFAS, RS 831.441.1 (OPP 2), art. 2 et 8.
-- Valeurs en CHF/an — periodicite = ANNUEL.
-- La conversion mensuelle est faite dans le code Rust.
-- ============================================================

-- Seuil d'entrée LPP (salaire annuel minimum pour être assuré)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CH_LPP_SEUIL',  '2015-01-01', '2019-01-01', '21150.00', 'ANNUEL'),
  ('CH_LPP_SEUIL',  '2019-01-01', '2021-01-01', '21330.00', 'ANNUEL'),
  ('CH_LPP_SEUIL',  '2021-01-01', '2023-01-01', '21510.00', 'ANNUEL'),
  ('CH_LPP_SEUIL',  '2023-01-01', '2025-01-01', '22050.00', 'ANNUEL'),
  ('CH_LPP_SEUIL',  '2025-01-01', NULL,          '22680.00', 'ANNUEL');

-- Déduction de coordination (montant soustrait du salaire brut pour obtenir le salaire coordonné)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CH_LPP_COORD_DED', '2015-01-01', '2019-01-01', '24570.00', 'ANNUEL'),
  ('CH_LPP_COORD_DED', '2019-01-01', '2021-01-01', '24885.00', 'ANNUEL'),
  ('CH_LPP_COORD_DED', '2021-01-01', '2023-01-01', '25095.00', 'ANNUEL'),
  ('CH_LPP_COORD_DED', '2023-01-01', '2025-01-01', '25725.00', 'ANNUEL'),
  ('CH_LPP_COORD_DED', '2025-01-01', NULL,          '27225.00', 'ANNUEL');

-- Salaire coordonné minimum (plancher du salaire assuré)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CH_LPP_COORD_MIN', '2015-01-01', '2019-01-01', '3525.00',  'ANNUEL'),
  ('CH_LPP_COORD_MIN', '2019-01-01', '2021-01-01', '3555.00',  'ANNUEL'),
  ('CH_LPP_COORD_MIN', '2021-01-01', '2023-01-01', '3585.00',  'ANNUEL'),
  ('CH_LPP_COORD_MIN', '2023-01-01', '2025-01-01', '3675.00',  'ANNUEL'),
  ('CH_LPP_COORD_MIN', '2025-01-01', NULL,          '3780.00',  'ANNUEL');

-- Salaire coordonné maximum (plafond du salaire assuré)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CH_LPP_COORD_MAX', '2015-01-01', '2019-01-01', '59535.00', 'ANNUEL'),
  ('CH_LPP_COORD_MAX', '2019-01-01', '2021-01-01', '59985.00', 'ANNUEL'),
  ('CH_LPP_COORD_MAX', '2021-01-01', '2023-01-01', '60435.00', 'ANNUEL'),
  ('CH_LPP_COORD_MAX', '2023-01-01', '2025-01-01', '61950.00', 'ANNUEL'),
  ('CH_LPP_COORD_MAX', '2025-01-01', NULL,          '64260.00', 'ANNUEL');
