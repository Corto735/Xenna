-- ============================================================
-- DROITS OUVERTS PAR LE MOIS — Sécurité sociale, chômage, congés payés
-- (calculs/droits.rs, bloc « Ce que ce mois vous ouvre comme droits »)
--
-- SÉCURITÉ SOCIALE — indemnités journalières maladie (arrêt ≤ 6 mois) :
--   • ouverture : 150 heures de travail sur les 3 mois civils ou 90 jours
--     précédant l'arrêt, OU cotisations sur une rémunération au moins égale à
--     1 015 fois le SMIC horaire sur les 6 mois civils précédents (art. R313-3
--     CSS, rédaction du décret n° 2015-86 du 30 janvier 2015) ;
--   • montant : 50 % du salaire journalier de base (salaires bruts des 3 mois
--     ÷ 91,25, art. R323-4 CSS), salaire retenu dans la limite de 1,8 SMIC
--     mensuel, 1,4 SMIC pour les arrêts débutant depuis le 1er avril 2025
--     (décret n° 2025-160 du 20 février 2025). Même règle qu'absence.rs.
--
-- ASSURANCE CHÔMAGE — ARE (règlement annexé à la convention du 15 nov. 2024,
-- « Paramètres utiles » Unédic, juillet 2026, relevé le 18 octobre 2026) :
--   • affiliation : 130 jours travaillés ou 910 heures sur les 24 derniers mois
--     (36 mois à partir de 55 ans), depuis le 1er décembre 2021 ;
--   • durée : jours calendaires de la période d'emploi × 0,75 pour les fins de
--     contrat depuis le 1er février 2023 (décret n° 2023-33 du 26 janvier 2023) ;
--   • montant journalier : le plus favorable de 40,4 % du SJR + partie fixe ou
--     57 % du SJR, au moins l'allocation minimale, dans la limite de 75 % du SJR
--     (depuis le 1er octobre 2021) ;
--   • partie fixe 13,18 €, allocation minimale 32,13 €, depuis le 1er juillet
--     2025, NON revalorisées au 1er juillet 2026 (CA Unédic du 30 juin 2026) ;
--     seuil de dégressivité 92,57 € (coefficient 0,7 au 7e mois, moins de 55 ans).
--   Valeurs antérieures au 1er juillet 2025 non relevées : le montant n'est pas
--   affiché avant (lacune déclarée, pas de valeur supposée).
--
-- CONGÉS PAYÉS — 2,5 jours ouvrables par mois de travail effectif, 30 au plus
-- (art. L3141-3 C. trav.) ; 2 jours par mois d'arrêt maladie non professionnel
-- (art. L3141-5-1, loi n° 2024-364 du 22 avril 2024, en vigueur le 24 avril 2024).
-- ============================================================
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('IJSS_HEURES_3MOIS',          '2015-02-01', NULL,         '150',   'ANNUEL'),
    ('IJSS_SMIC_6MOIS',            '2015-02-01', NULL,         '1015',  'ANNUEL'),
    ('IJSS_TAUX',                  '2015-02-01', NULL,         '0.5',   'ANNUEL'),
    ('IJSS_PLAFOND_SMIC',          '2015-02-01', '2025-04-01', '1.8',   'ANNUEL'),
    ('IJSS_PLAFOND_SMIC',          '2025-04-01', NULL,         '1.4',   'ANNUEL'),
    ('CHOMAGE_HEURES_AFFILIATION', '2021-12-01', NULL,         '910',   'ANNUEL'),
    ('CHOMAGE_JOURS_AFFILIATION',  '2021-12-01', NULL,         '130',   'ANNUEL'),
    ('CHOMAGE_COEFF_DUREE',        '2023-02-01', NULL,         '0.75',  'ANNUEL'),
    ('ARE_TAUX_PROPORTIONNEL',     '2021-10-01', NULL,         '0.404', 'ANNUEL'),
    ('ARE_TAUX_UNIQUE',            '2021-10-01', NULL,         '0.57',  'ANNUEL'),
    ('ARE_PLAFOND_SJR',            '2021-10-01', NULL,         '0.75',  'ANNUEL'),
    ('ARE_PARTIE_FIXE',            '2025-07-01', NULL,         '13.18', 'ANNUEL'),
    ('ARE_MINIMUM',                '2025-07-01', NULL,         '32.13', 'ANNUEL'),
    ('ARE_SEUIL_DEGRESSIVITE',     '2025-07-01', NULL,         '92.57', 'ANNUEL'),
    ('CP_JOURS_MOIS',              '1982-04-01', NULL,         '2.5',   'MENSUEL'),
    ('CP_JOURS_MOIS_MALADIE',      '2024-04-24', NULL,         '2',     'MENSUEL');
