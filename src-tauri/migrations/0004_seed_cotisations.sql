-- ============================================================
-- DEFINITIONS DES COTISATIONS
-- ============================================================
INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('SS_MALADIE', 'Assurance maladie, maternité, invalidité, décès',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL', '0', NULL, NULL, '100');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('SS_VIEILLESSE_PLAF', 'Assurance vieillesse plafonnée',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ', '0', '1', '101', '101');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('SS_VIEILLESSE_DEPLAF', 'Assurance vieillesse déplafonnée',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL', '0', NULL, '101', '101');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('FAMILLE', 'Allocations familiales',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL', '0', NULL, NULL, '102');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('AT_MP', 'Accidents du travail / Maladies professionnelles',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'SECURITE_SOCIALE', 1, 1, 'SPECIFIQUE', '0', NULL, NULL, '103');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('CSG_DEDUCTIBLE', 'CSG déductible',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'CSG_CRDS', 1, 1, 'CSG', '0', NULL, '090', NULL);

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('CSG_NON_DEDUCTIBLE', 'CSG non déductible',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'CSG_CRDS', 1, 1, 'CSG', '0', NULL, '091', NULL);

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('CRDS', 'Contribution au Remboursement de la Dette Sociale',
    (SELECT id FROM organisme WHERE code = 'CADES'),
    'CSG_CRDS', 1, 1, 'CSG', '0', NULL, '092', NULL);

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('CHOMAGE', 'Assurance chômage',
    (SELECT id FROM organisme WHERE code = 'FRANCE_TRAVAIL'),
    'CHOMAGE', 1, 1, 'BRUT_PLAFONNÉ', '0', '4', NULL, '200');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('AGS', 'Garantie des salaires (AGS/FNGS)',
    (SELECT id FROM organisme WHERE code = 'FRANCE_TRAVAIL'),
    'CHOMAGE', 1, 1, 'BRUT_PLAFONNÉ', '0', '4', NULL, '201');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('AGIRC_ARRCO_T1', 'AGIRC-ARRCO Tranche 1',
    (SELECT id FROM organisme WHERE code = 'AGIRC_ARRCO'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'TRANCHE_A', '0', '1', '300', '300');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('AGIRC_ARRCO_T2', 'AGIRC-ARRCO Tranche 2',
    (SELECT id FROM organisme WHERE code = 'AGIRC_ARRCO'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'TRANCHE_B', '1', '8', '301', '301');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('AGIRC_ARRCO_CEG_T1', 'Contribution Equilibre Général T1',
    (SELECT id FROM organisme WHERE code = 'AGIRC_ARRCO'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'TRANCHE_A', '0', '1', '302', '302');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('AGIRC_ARRCO_CEG_T2', 'Contribution Equilibre Général T2',
    (SELECT id FROM organisme WHERE code = 'AGIRC_ARRCO'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'TRANCHE_B', '1', '8', '303', '303');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('AGIRC_ARRCO_CET', 'Contribution Equilibre Technique',
    (SELECT id FROM organisme WHERE code = 'AGIRC_ARRCO'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ', '0', '8', '304', '304');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('APEC', 'Cotisation APEC (cadres uniquement)',
    (SELECT id FROM organisme WHERE code = 'APEC'),
    'RETRAITE_COMPLEMENTAIRE', 1, 0, 'BRUT_PLAFONNÉ', '0', '4', '400', '400');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('PREVOYANCE_CADRE_MIN', 'Prévoyance cadre minimale (art. 7 CCN 1947)',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'PREVOYANCE', 1, 0, 'TRANCHE_A', '0', '1', NULL, '500');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('FORMATION_PROF', 'Contribution formation professionnelle',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'FORMATION', 1, 1, 'BRUT_TOTAL', '0', NULL, NULL, '600');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('TAXE_APPRENTISSAGE', 'Taxe d''apprentissage (solde)',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'FORMATION', 1, 1, 'BRUT_TOTAL', '0', NULL, NULL, '602');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES ('TRANSPORT', 'Versement mobilité',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'TRANSPORT', 1, 1, 'BRUT_TOTAL', '0', NULL, NULL, '700');

-- ============================================================
-- TAUX ACTUELS (2018 → aujourd'hui)
-- ============================================================
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id) VALUES
    ((SELECT id FROM cotisation WHERE code = 'SS_MALADIE'),           '2018-01-01', NULL,         '0.0000', '0.1313', (SELECT id FROM texte_loi WHERE code = 'LFSS_2018')),
    ((SELECT id FROM cotisation WHERE code = 'SS_VIEILLESSE_PLAF'),   '2018-01-01', NULL,         '0.0690', '0.0855', (SELECT id FROM texte_loi WHERE code = 'LFSS_2018')),
    ((SELECT id FROM cotisation WHERE code = 'SS_VIEILLESSE_DEPLAF'), '2018-01-01', NULL,         '0.0040', '0.0190', (SELECT id FROM texte_loi WHERE code = 'LFSS_2018')),
    ((SELECT id FROM cotisation WHERE code = 'FAMILLE'),              '2015-04-01', NULL,         '0.0000', '0.0345', (SELECT id FROM texte_loi WHERE code = 'DECRET_2015_FILLON_FAM')),
    ((SELECT id FROM cotisation WHERE code = 'AT_MP'),                '2024-01-01', NULL,         '0.0000', '0.0235', NULL),
    ((SELECT id FROM cotisation WHERE code = 'CSG_DEDUCTIBLE'),       '2018-01-01', NULL,         '0.0680', '0.0000', (SELECT id FROM texte_loi WHERE code = 'LFSS_2018')),
    ((SELECT id FROM cotisation WHERE code = 'CSG_NON_DEDUCTIBLE'),   '2018-01-01', NULL,         '0.0240', '0.0000', (SELECT id FROM texte_loi WHERE code = 'LFSS_2018')),
    ((SELECT id FROM cotisation WHERE code = 'CRDS'),                 '1996-02-01', NULL,         '0.0050', '0.0000', (SELECT id FROM texte_loi WHERE code = 'ORDONNANCE_1996_CRDS')),
    ((SELECT id FROM cotisation WHERE code = 'CHOMAGE'),              '2019-10-01', NULL,         '0.0000', '0.0405', NULL),
    ((SELECT id FROM cotisation WHERE code = 'AGS'),                  '2024-01-01', NULL,         '0.0000', '0.0015', NULL),
    ((SELECT id FROM cotisation WHERE code = 'AGIRC_ARRCO_T1'),       '2019-01-01', NULL,         '0.0315', '0.0472', (SELECT id FROM texte_loi WHERE code = 'ANI_2017_AGIRC_ARRCO')),
    ((SELECT id FROM cotisation WHERE code = 'AGIRC_ARRCO_T2'),       '2019-01-01', NULL,         '0.0864', '0.1296', (SELECT id FROM texte_loi WHERE code = 'ANI_2017_AGIRC_ARRCO')),
    ((SELECT id FROM cotisation WHERE code = 'AGIRC_ARRCO_CEG_T1'),   '2019-01-01', NULL,         '0.0086', '0.0129', (SELECT id FROM texte_loi WHERE code = 'ANI_2017_AGIRC_ARRCO')),
    ((SELECT id FROM cotisation WHERE code = 'AGIRC_ARRCO_CEG_T2'),   '2019-01-01', NULL,         '0.0108', '0.0162', (SELECT id FROM texte_loi WHERE code = 'ANI_2017_AGIRC_ARRCO')),
    ((SELECT id FROM cotisation WHERE code = 'AGIRC_ARRCO_CET'),      '2019-01-01', NULL,         '0.0014', '0.0021', (SELECT id FROM texte_loi WHERE code = 'ANI_2017_AGIRC_ARRCO')),
    ((SELECT id FROM cotisation WHERE code = 'APEC'),                 '2019-01-01', NULL,         '0.0024', '0.0036', NULL),
    ((SELECT id FROM cotisation WHERE code = 'PREVOYANCE_CADRE_MIN'), '1947-03-14', NULL,         '0.0000', '0.0150', (SELECT id FROM texte_loi WHERE code = 'ANI_1947_CADRES')),
    ((SELECT id FROM cotisation WHERE code = 'FORMATION_PROF'),       '2022-01-01', NULL,         '0.0000', '0.0100', NULL),
    ((SELECT id FROM cotisation WHERE code = 'TAXE_APPRENTISSAGE'),   '2022-01-01', NULL,         '0.0000', '0.0059', NULL),
    ((SELECT id FROM cotisation WHERE code = 'TRANSPORT'),            '2023-01-01', NULL,         '0.0000', '0.0000', NULL);

-- ============================================================
-- TAUX HISTORIQUES (2015-2017 — avant suppression cotisations salariales)
-- ============================================================
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal) VALUES
    ((SELECT id FROM cotisation WHERE code = 'SS_MALADIE'),           '2015-01-01', '2017-12-31', '0.0075', '0.1275'),
    ((SELECT id FROM cotisation WHERE code = 'SS_VIEILLESSE_PLAF'),   '2015-01-01', '2017-12-31', '0.0685', '0.0845'),
    ((SELECT id FROM cotisation WHERE code = 'SS_VIEILLESSE_DEPLAF'), '2015-01-01', '2017-12-31', '0.0040', '0.0175'),
    ((SELECT id FROM cotisation WHERE code = 'CSG_DEDUCTIBLE'),       '2015-01-01', '2017-12-31', '0.0510', '0.0000'),
    ((SELECT id FROM cotisation WHERE code = 'CSG_NON_DEDUCTIBLE'),   '2015-01-01', '2017-12-31', '0.0240', '0.0000'),
    ((SELECT id FROM cotisation WHERE code = 'CHOMAGE'),              '2015-01-01', '2019-09-30', '0.0240', '0.0405');

-- ============================================================
-- PARAMETRES ALLEGEMENTS
-- ============================================================
INSERT INTO allegement_type (code, libelle, notes) VALUES
    ('FILLON',       'Réduction générale des cotisations patronales (ex-réduction Fillon)',
     'Calcul annualisé sur 12 mois. Instaurée par la loi Fillon de 2003. Formule revue en 2019 (intégration retraite complémentaire et prévoyance).'),
    ('APPRENTISSAGE', 'Exonération cotisations apprentis', NULL);

INSERT INTO allegement_param (allegement_type_id, date_debut, date_fin, coeff_max_fillon, seuil_smic_coeff, formule_texte, texte_loi_id)
VALUES (
    (SELECT id FROM allegement_type WHERE code = 'FILLON'),
    '2019-01-01', NULL,
    '0.3214', '1.6',
    'C = (T / 0.6) × (1.6 × SMIC_annualisé / Rémunération_annuelle − 1) ; borné [0 ; T] ; T = 0,3214 (avec prévoyance)',
    NULL
);

INSERT INTO allegement_param (allegement_type_id, date_debut, date_fin, coeff_max_fillon, seuil_smic_coeff, formule_texte, texte_loi_id)
VALUES (
    (SELECT id FROM allegement_type WHERE code = 'FILLON'),
    '2015-01-01', '2018-12-31',
    '0.2850', '1.6',
    'C = (0.2850 / 0.6) × (1.6 × SMIC_annualisé / Rémunération_annuelle − 1) ; borné [0 ; 0.2850]',
    NULL
);
