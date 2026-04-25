-- ============================================================
-- MIGRATION 0007 — HISTORIQUE COMPLET DES TAUX 2015-2026
-- Complète les taux manquants et corrige les périodes inexactes
-- ============================================================

-- ============================================================
-- NOUVEAUX TEXTES DE LOI DE RÉFÉRENCE
-- ============================================================
INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES

    ('DECRET_2014_FAMILLE',
     'DECRET',
     'Décret relatif au taux réduit de cotisation famille pour les bas salaires',
     'n°2014-1531', '2014-12-18', '2015-01-01',
     'Instaure un taux réduit allocations familiales (3,45% au lieu de 5,25%) pour les salaires ≤ 1,6 SMIC dès le 1er janvier 2015. Étendu à ≤ 3,5 SMIC par décret n°2015-390 du 3 avril 2015.'),

    ('ANI_2011_ARRCO',
     'ANI',
     'Accord national interprofessionnel ARRCO — paramètres 2012-2015',
     NULL, '2011-03-30', '2012-01-01',
     'Fixe les taux d''appel ARRCO à 125% du taux contractuel pour la période 2012-2015. Tranche 1 : taux d''appel 7,75% (sal 3,10% + pat 4,65%). Signé CFDT, CGT, FO, CFE-CGC, CFTC, MEDEF, CGPME, UPA.'),

    ('ANI_2015_RETRAITE',
     'ANI',
     'Accord national interprofessionnel AGIRC-ARRCO — paramètres 2016-2018',
     NULL, '2015-10-30', '2016-01-01',
     'Fixe les taux pour la période 2016-2018 en prévision de la fusion des deux régimes. Maintien des taux d''appel à 125%. Première étape vers la création du régime unique au 1er janvier 2019.'),

    ('CONVENTION_UNEDIC_2017',
     'ANI',
     'Convention d''assurance chômage (UNEDIC)',
     NULL, '2017-04-14', '2017-10-01',
     'Relève le taux patronal chômage de 4,00% à 4,05% à compter du 1er octobre 2017. Signé CFDT, CFTC, CFE-CGC, MEDEF, CPME, U2P. CGT et FO n''ont pas signé.'),

    ('DECRET_2018_CHOMAGE_2',
     'DECRET',
     'Suppression définitive de la cotisation salariale chômage — 2ème étape',
     NULL, '2018-07-01', '2018-10-01',
     'Deuxième étape de la réforme Macron : cotisation salariale chômage réduite à 0% (0,95% → 0%). La 1ère étape avait réduit de 2,40% à 0,95% au 01/01/2018 (LFSS 2018). Gain net pour le salarié : +0,95 point de salaire net.');

-- ============================================================
-- ALLOCATIONS FAMILIALES — FAMILLE
-- ============================================================
-- Avant le décret n°2015-390 du 3 avril 2015, le taux réduit (3,45%) n'était
-- applicable qu'aux salaires ≤ 1,6 SMIC (décret n°2014-1531). Au-delà : 5,25%.
-- Pour un simulateur à taux unique, on utilise 5,25% (taux standard > 1,6 SMIC).
-- Le décret 2015-390 (01/04/2015) étend le taux réduit à tous les salaires ≤ 3,5 SMIC.
INSERT INTO cotisation_taux
    (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes)
VALUES (
    (SELECT id FROM cotisation WHERE code = 'FAMILLE'),
    '2015-01-01', '2015-03-31', '0.0000', '0.0525',
    (SELECT id FROM texte_loi WHERE code = 'DECRET_2014_FAMILLE'),
    'Taux standard (> 1,6 SMIC). Le taux réduit 3,45% s''appliquait déjà aux salaires ≤ 1,6 SMIC depuis le 01/01/2015.'
);

-- ============================================================
-- ACCIDENTS DU TRAVAIL / MALADIES PROFESSIONNELLES — AT/MP
-- ============================================================
-- Taux fixé individuellement par la CARSAT selon le code risque de l'entreprise.
-- Les valeurs ci-dessous correspondent au taux moyen national toutes activités.
-- Source : publications URSSAF / rapports CNAM AT-MP annuels.
INSERT INTO cotisation_taux
    (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes)
VALUES
    ((SELECT id FROM cotisation WHERE code = 'AT_MP'),
     '2015-01-01', '2018-12-31', '0.0000', '0.0220',
     'Taux moyen indicatif toutes activités — varie selon le code risque CARSAT de l''entreprise (0,7% à 24%). Source : rapports CNAM AT-MP.'),

    ((SELECT id FROM cotisation WHERE code = 'AT_MP'),
     '2019-01-01', '2022-12-31', '0.0000', '0.0222',
     'Taux moyen indicatif toutes activités — varie selon le code risque CARSAT de l''entreprise.'),

    ((SELECT id FROM cotisation WHERE code = 'AT_MP'),
     '2023-01-01', '2023-12-31', '0.0000', '0.0235',
     'Taux moyen indicatif toutes activités — varie selon le code risque CARSAT de l''entreprise.');

-- ============================================================
-- ASSURANCE CHÔMAGE — CHOMAGE
-- ============================================================
-- Les entrées précédentes (2015-2019-09-30 et 2019-10-01) étaient inexactes :
--   • Le taux patronal était 4,00% jusqu'au 30/09/2017 (convention UNEDIC 2014),
--     puis 4,05% à partir du 01/10/2017 (convention du 14/04/2017).
--   • La réforme Macron s'est faite en 2 étapes :
--     1. LFSS 2018 (01/01/2018) : sal 2,40% → 0,95% (compensation hausse CSG)
--     2. 2ème étape (01/10/2018) : sal 0,95% → 0% (suppression définitive)
DELETE FROM cotisation_taux
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'CHOMAGE');

INSERT INTO cotisation_taux
    (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes)
VALUES
    -- Convention UNEDIC 2014 en vigueur jusqu'au 30/09/2017
    ((SELECT id FROM cotisation WHERE code = 'CHOMAGE'),
     '2015-01-01', '2017-09-30', '0.0240', '0.0400',
     NULL,
     'Convention UNEDIC du 14/05/2014 — taux patronal 4,00%'),

    -- Convention UNEDIC du 14/04/2017, applicable au 01/10/2017
    ((SELECT id FROM cotisation WHERE code = 'CHOMAGE'),
     '2017-10-01', '2017-12-31', '0.0240', '0.0405',
     (SELECT id FROM texte_loi WHERE code = 'CONVENTION_UNEDIC_2017'),
     'Convention UNEDIC du 14/04/2017 — hausse patronal 4,00% → 4,05%'),

    -- LFSS 2018 — 1ère étape : suppression partielle cotisation salariale chômage
    -- Compensation : hausse CSG déductible 5,10% → 6,80% (+1,70 pt)
    ((SELECT id FROM cotisation WHERE code = 'CHOMAGE'),
     '2018-01-01', '2018-09-30', '0.0095', '0.0405',
     (SELECT id FROM texte_loi WHERE code = 'LFSS_2018'),
     'LFSS 2018 — 1ère étape : sal 2,40% → 0,95% (compensation hausse CSG +1,70 pt)'),

    -- 2ème étape : suppression totale de la cotisation salariale chômage
    ((SELECT id FROM cotisation WHERE code = 'CHOMAGE'),
     '2018-10-01', NULL, '0.0000', '0.0405',
     (SELECT id FROM texte_loi WHERE code = 'DECRET_2018_CHOMAGE_2'),
     'Suppression définitive cotisation salariale chômage — gain net salarié +0,95 point');

-- ============================================================
-- GARANTIE DES SALAIRES — AGS
-- ============================================================
-- Taux fixé annuellement par UNEDIC/AGS selon l'équilibre financier du fonds.
-- Entièrement patronal, assiette plafonnée à 4 PMSS.
-- Sources : règlements AGS / circulaires FNGS annuelles.
INSERT INTO cotisation_taux
    (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes)
VALUES
    ((SELECT id FROM cotisation WHERE code = 'AGS'),
     '2015-01-01', '2016-12-31', '0.0000', '0.0025',
     'Taux AGS 2015-2016 — source : règlement AGS, circulaire FNGS'),

    ((SELECT id FROM cotisation WHERE code = 'AGS'),
     '2017-01-01', '2017-12-31', '0.0000', '0.0020',
     'Taux AGS 2017 — réduction grâce à l''excédent du fonds de garantie'),

    ((SELECT id FROM cotisation WHERE code = 'AGS'),
     '2018-01-01', '2019-12-31', '0.0000', '0.0015',
     'Taux AGS 2018-2019 — niveau plancher, fonds excédentaire'),

    ((SELECT id FROM cotisation WHERE code = 'AGS'),
     '2020-01-01', '2022-12-31', '0.0000', '0.0025',
     'Taux AGS relevé en 2020 — hausse des procédures collectives post-COVID'),

    ((SELECT id FROM cotisation WHERE code = 'AGS'),
     '2023-01-01', '2023-12-31', '0.0000', '0.0020',
     'Taux AGS 2023 — retour progressif à la normale après la vague de faillites post-COVID');

-- ============================================================
-- RETRAITE COMPLÉMENTAIRE AGIRC-ARRCO — AVANT FUSION (2015-2018)
-- ============================================================
-- Avant le 1er janvier 2019, deux régimes distincts coexistaient :
--   ARRCO : tous les salariés (Tranche 1 : 0-3 PMSS)
--   AGIRC : cadres uniquement (Tranche B : 1-4 PMSS, Tranche C : 4-8 PMSS)
--
-- Sources : ANI du 30/03/2011, ANI du 30/10/2015
--
-- ARRCO Tranche 1 (taux d'appel 125% du taux contractuel 6,20%) :
--   Taux d'appel = 7,75% → sal 3,10% (40%) + pat 4,65% (60%)
--
-- AGIRC Tranche B, approximation 0-8 PMSS cadres + non-cadres (taux d'appel ~125%) :
--   Taux d'appel ≈ 20,55% → sal 7,80% (38%) + pat 12,75% (62%)
--   NB : en réalité applicable cadres 1-4 PMSS (AGIRC) ; pour non-cadres l'ARRCO T2
--   démarrait à 3 PMSS avec des taux différents. La fusion 2019 a unifié à 1 PMSS.
--
-- CEG (Contribution Équilibre Général) et CET (Contribution Équilibre Technique)
-- n'existaient pas avant la fusion : non insérés (la fonction Rust retourne 0 si absent).

INSERT INTO cotisation_taux
    (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes)
VALUES
    -- ARRCO Tranche 1 (0-1 PMSS, tous salariés)
    ((SELECT id FROM cotisation WHERE code = 'AGIRC_ARRCO_T1'),
     '2015-01-01', '2018-12-31', '0.0310', '0.0465',
     (SELECT id FROM texte_loi WHERE code = 'ANI_2015_RETRAITE'),
     'ARRCO Tranche 1 avant fusion — taux d''appel 7,75% (125% × 6,20% contractuel) — sal 40% + pat 60%'),

    -- AGIRC Tranche B / ARRCO T2 — approximation pour la tranche > 1 PMSS
    ((SELECT id FROM cotisation WHERE code = 'AGIRC_ARRCO_T2'),
     '2015-01-01', '2018-12-31', '0.0780', '0.1275',
     (SELECT id FROM texte_loi WHERE code = 'ANI_2015_RETRAITE'),
     'AGIRC Tranche B avant fusion — taux d''appel ≈ 20,55% — sal 38% + pat 62% — approximation ; cadres 1-4 PMSS (AGIRC), non-cadres 3-8 PMSS (ARRCO T2) unifiés ici par simplification');

-- ============================================================
-- APEC — Cotisation cadres (2015-2018)
-- ============================================================
-- Taux stable depuis de nombreuses années — assiette plafonnée à 4 PMSS.
-- Applicable uniquement aux salariés ayant le statut cadre.
INSERT INTO cotisation_taux
    (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes)
VALUES (
    (SELECT id FROM cotisation WHERE code = 'APEC'),
    '2015-01-01', '2018-12-31', '0.0024', '0.0036',
    'Taux APEC identique aux périodes suivantes — stable depuis 2008 environ'
);

-- ============================================================
-- CONTRIBUTION FORMATION PROFESSIONNELLE (2015-2021)
-- ============================================================
-- Loi n°2014-288 du 5 mars 2014 (applicable 2015) :
--   < 11 salariés : 0,55% | ≥ 11 salariés : 1,00%
-- Loi Avenir professionnel n°2018-771 (applicable 2020) :
--   < 50 salariés : 0,55% | ≥ 50 salariés : 1,00%
-- La réforme 2022 (CUFPA) a restructuré les assiettes sans changer le taux de référence.
-- Taux ci-dessous : entreprises ≥ 11 salariés (taux de droit commun).
INSERT INTO cotisation_taux
    (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes)
VALUES (
    (SELECT id FROM cotisation WHERE code = 'FORMATION_PROF'),
    '2015-01-01', '2021-12-31', '0.0000', '0.0100',
    'CFP ≥ 11 salariés — loi n°2014-288 du 05/03/2014 puis loi Avenir professionnel 2018 — taux identique avant la réforme CUFPA 2022'
);

-- ============================================================
-- TAXE D'APPRENTISSAGE (2015-2021)
-- ============================================================
-- Avant la réforme loi n°2018-771 (Avenir professionnel), applicable 2022 :
--   Taux unique 0,68% de la masse salariale brute
-- Depuis 2022 (CUFPA) : 0,59% base légale + 0,09% contribution supplémentaire
--   (notre seed a 0,59% depuis 2022, ce qui représente la base légale)
INSERT INTO cotisation_taux
    (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes)
VALUES (
    (SELECT id FROM cotisation WHERE code = 'TAXE_APPRENTISSAGE'),
    '2015-01-01', '2021-12-31', '0.0000', '0.0068',
    'Taxe d''apprentissage avant réforme loi n°2018-771 (Avenir professionnel) — taux 0,68% masse salariale brute'
);

-- ============================================================
-- NOTE : PLAFONDS MANQUANTS (à compléter manuellement)
-- ============================================================
-- PMSS 2026 : non inclus (valeur officielle non disponible à la date de rédaction de
--   cette migration). La requête utilisera le PMSS 2025 (3 925,00 €) comme fallback.
--   À ajouter dans migration 0008 ou via UPDATE sur l'entrée 2025 une fois l'arrêté publié.
--
-- PASS 2026 : idem (= 12 × PMSS mensuel 2026).
--
-- SS MALADIE taux réduit 2023+ : LFSS 2023 instaure un taux patronal réduit à 7,00%
--   pour les salaires ≤ 2,5 SMIC. L'architecture actuelle (un seul taux par période)
--   ne supporte pas encore les taux variables selon assiette — à implémenter ultérieurement.
