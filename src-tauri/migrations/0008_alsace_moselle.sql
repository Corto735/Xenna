-- Régime local d'Alsace-Moselle — cotisation maladie complémentaire (droit local)
-- Applicable aux salariés des dép. Bas-Rhin (67), Haut-Rhin (68) et Moselle (57).
-- Purement salariale (aucune part patronale), assiette = salaire brut total.

INSERT INTO cotisation (code, libelle, organisme_id, categorie,
    applicable_cadre, applicable_non_cadre, type_assiette,
    plafond_coeff_min, plafond_coeff_max, code_dsn_sal, code_dsn_pat)
VALUES (
    'ALSACE_MOSELLE_MALADIE',
    'Maladie complémentaire Alsace-Moselle (régime local)',
    (SELECT id FROM organisme WHERE code = 'URSSAF'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL', '0', NULL, '107', NULL
);

-- 2015-01-01 → 2018-06-30 : taux salarial 1,50 %
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal)
VALUES (
    (SELECT id FROM cotisation WHERE code = 'ALSACE_MOSELLE_MALADIE'),
    '2015-01-01', '2018-06-30', '0.0150', '0'
);

-- 2018-07-01 → … : taux salarial 1,30 % (abaissement LFSS 2018 / loi 2018-1203)
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal)
VALUES (
    (SELECT id FROM cotisation WHERE code = 'ALSACE_MOSELLE_MALADIE'),
    '2018-07-01', NULL, '0.0130', '0'
);
