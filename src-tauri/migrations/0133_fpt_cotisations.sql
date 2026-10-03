-- ============================================================
-- FONCTION PUBLIQUE TERRITORIALE — titulaires CNRACL
--
-- 1. Part agent CNRACL : la montée en charge était décalée d'un an
--    (11,10 % dès 2019). Taux officiels : 9,54 % (2015), 9,94 % (2016),
--    10,29 % (2017), 10,56 % (2018), 10,83 % (2019), 11,10 % (2020+).
--    Collectivité : 30,65 % jusqu'en 2024 puis décret n°2025-86.
--    Dates de fin rendues exclusives (une paie du 31/12 tombait hors période).
-- 2. Lignes propres aux titulaires, jusqu'ici remplacées par celles du privé :
--    maladie 11,50 % (≤ 2017) puis 9,88 % au lieu de 13 % ; ATIACL 0,40 % au
--    lieu de l'AT/MP du régime général ; FNAL 0,10 % (≤ PMSS, moins de
--    50 agents) ou 0,50 % ; contribution solidarité autonomie 0,30 % ;
--    CNFPT 1 % (2015), 0,9 % (2016-2021), 0,95 % (2022) puis 1 % (2023+), cotisation
--    apprentissage comprise (loi n°2021-1900, art. 122).
-- Sources : CDG 44, « Cotisations 2026 — régime spécial CNRACL » ; IPP,
-- barèmes CNRACL ; CDG 33, récapitulatifs des cotisations 2024-2026.
-- ============================================================
DELETE FROM cotisation_taux
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'FPT_CNRACL');
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2015-01-01', '2016-01-01', '0.0954', '0.3065', 'Agent 9,54 %'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2016-01-01', '2017-01-01', '0.0994', '0.3065', 'Agent 9,94 %'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2017-01-01', '2018-01-01', '0.1029', '0.3065', 'Agent 10,29 %'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2018-01-01', '2019-01-01', '0.1056', '0.3065', 'Agent 10,56 %'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2019-01-01', '2020-01-01', '0.1083', '0.3065', 'Agent 10,83 %'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2020-01-01', '2025-01-01', '0.1110', '0.3065', 'Taux cible agent 11,10 % atteint en 2020'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2025-01-01', '2026-01-01', '0.1110', '0.3465', 'Décret n°2025-86 : collectivité 34,65 %'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2026-01-01', '2027-01-01', '0.1110', '0.3765', 'Décret n°2025-86 : collectivité 37,65 %'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2027-01-01', '2028-01-01', '0.1110', '0.4065', 'Décret n°2025-86 : collectivité 40,65 %'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2028-01-01', NULL,         '0.1110', '0.4365', 'Décret n°2025-86 : collectivité 43,65 %');

INSERT INTO cotisation (code, libelle, categorie, type_assiette, plafond_coeff_min, plafond_coeff_max, notes) VALUES
    ('FPT_MALADIE', 'Maladie, maternité, invalidité, décès (fonctionnaires CNRACL)', 'SECURITE_SOCIALE', 'BRUT_TOTAL', '0', NULL, 'Régime spécial, employeur seul'),
    ('FPT_ATIACL',  'Allocation temporaire d''invalidité (ATIACL)',                 'SECURITE_SOCIALE', 'SPECIFIQUE', '0', NULL, 'Traitement indiciaire hors NBI'),
    ('FPT_FNAL',    'FNAL — collectivités de moins de 50 agents',                   'SECURITE_SOCIALE', 'BRUT_PLAFONNÉ', '0', '1', NULL),
    ('FPT_FNAL_50', 'FNAL — collectivités de 50 agents et plus',                    'SECURITE_SOCIALE', 'BRUT_TOTAL', '0', NULL, NULL),
    ('FPT_CSA',     'Contribution solidarité autonomie',                            'AUTRES', 'BRUT_TOTAL', '0', NULL, NULL),
    ('FPT_CNFPT',   'Cotisation CNFPT',                                             'FORMATION', 'BRUT_TOTAL', '0', NULL, NULL);

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
    ((SELECT id FROM cotisation WHERE code = 'FPT_MALADIE'), '2015-01-01', '2018-01-01', '0', '0.1150', NULL),
    ((SELECT id FROM cotisation WHERE code = 'FPT_MALADIE'), '2018-01-01', NULL,         '0', '0.0988', 'Baisse compensant la hausse de la CSG'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_ATIACL'),  '2015-01-01', NULL,         '0', '0.0040', NULL),
    ((SELECT id FROM cotisation WHERE code = 'FPT_FNAL'),    '2015-01-01', NULL,         '0', '0.0010', NULL),
    ((SELECT id FROM cotisation WHERE code = 'FPT_FNAL_50'), '2015-01-01', NULL,         '0', '0.0050', NULL),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CSA'),     '2015-01-01', NULL,         '0', '0.0030', NULL),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNFPT'),   '2015-01-01', '2016-01-01', '0', '0.0100', NULL),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNFPT'),   '2016-01-01', '2022-01-01', '0', '0.0090', NULL),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNFPT'),   '2022-01-01', '2023-01-01', '0', '0.0095', 'Dont 0,05 % pour la formation des apprentis'),
    ((SELECT id FROM cotisation WHERE code = 'FPT_CNFPT'),   '2023-01-01', NULL,         '0', '0.0100', 'Dont 0,10 % pour la formation des apprentis');
