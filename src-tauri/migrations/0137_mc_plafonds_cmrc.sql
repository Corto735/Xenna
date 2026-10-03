-- ============================================================
-- MONACO — plafonds, CAR employeur et retraite complémentaire CMRC
-- Année sociale d'octobre à septembre (Caisses sociales de Monaco, lettres
-- d'information aux employeurs d'octobre 2025 et d'octobre 2026) :
--                          oct. 2025 – sept. 2026   dès oct. 2026
--   CAR (plafond)          6 112 €                  6 276 €
--   CAR employeur          7,45 % + 0,88 % variable 7,45 % + 0,94 %
--   CCSS (plafond)         9 800 €                  10 000 €
--   Chômage (plafond)      15 700 €                 16 020 €
--   CMRC tranche A         3 971 €                  4 067 €
--   CMRC TA 4,008 / 6,012 % (dont CEG 2,15 %), TB 9,716 / 14,574 %
--   (dont 2,70 %), jusqu'à 8 fois la tranche A.
-- Avant octobre 2025 : taux antérieurs non revérifiés, inchangés.
-- ============================================================
UPDATE cotisation_taux SET date_fin = '2025-10-01'
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'MC_CAR') AND date_debut = '2025-01-01';
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
    ((SELECT id FROM cotisation WHERE code = 'MC_CAR'), '2025-10-01', '2026-10-01', '0.0685', '0.0833', 'Base 7,45 % + variable 0,88 % (total 15,18 %)'),
    ((SELECT id FROM cotisation WHERE code = 'MC_CAR'), '2026-10-01', NULL,         '0.0685', '0.0839', 'Base 7,45 % + variable 0,94 % (total 15,24 %)');

INSERT INTO cotisation (code, libelle, categorie, type_assiette, plafond_coeff_min, plafond_coeff_max, notes) VALUES
    ('MC_CMRC_TA', 'CMRC — retraite complémentaire, tranche A', 'RETRAITE_COMPLEMENTAIRE', 'SPECIFIQUE', '0', NULL, 'Jusqu''au plafond de la tranche A'),
    ('MC_CMRC_TB', 'CMRC — retraite complémentaire, tranche B', 'RETRAITE_COMPLEMENTAIRE', 'SPECIFIQUE', '0', NULL, 'De 1 à 8 tranches A');
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
    ((SELECT id FROM cotisation WHERE code = 'MC_CMRC_TA'), '2025-10-01', NULL, '0.04008', '0.06012', '10,02 % dont 2,15 % non générateur de droits'),
    ((SELECT id FROM cotisation WHERE code = 'MC_CMRC_TB'), '2025-10-01', NULL, '0.09716', '0.14574', '24,29 % dont 2,70 % non générateur de droits');

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('MC_PLAF_CAR',  '2025-10-01', '2026-10-01', '6112',  'MENSUEL'),
    ('MC_PLAF_CAR',  '2026-10-01', NULL,         '6276',  'MENSUEL'),
    ('MC_PLAF_CCSS', '2025-10-01', '2026-10-01', '9800',  'MENSUEL'),
    ('MC_PLAF_CCSS', '2026-10-01', NULL,         '10000', 'MENSUEL'),
    ('MC_PLAF_CHOM', '2025-10-01', '2026-10-01', '15700', 'MENSUEL'),
    ('MC_PLAF_CHOM', '2026-10-01', NULL,         '16020', 'MENSUEL'),
    ('MC_PLAF_TA',   '2025-10-01', '2026-10-01', '3971',  'MENSUEL'),
    ('MC_PLAF_TA',   '2026-10-01', NULL,         '4067',  'MENSUEL');
