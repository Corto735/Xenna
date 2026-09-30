-- ============================================================
-- IDCC 0016 (transports routiers) — INDEMNITÉS DE REPAS DES OUVRIERS
-- Protocole du 30 avril 1974 relatif aux frais de déplacement des ouvriers
-- (annexe I), revalorisé par avenants. Montants du transport routier de
-- marchandises, activités auxiliaires, déménagement, fonds et valeurs,
-- logistique :
--   repas unique (art. 4)      : zone de camionnage autour de Paris ;
--   repas unique de nuit (art. 12) : service d'au moins 4 h entre 22 h et 7 h ;
--   indemnité spéciale (art. 7)    : amplitude couvrant 11 h-14 h 30 ou
--                                    18 h 30-22 h sans coupure d'au moins 1 h ;
--   casse-croûte (art. 5)          : prise de service avant 5 h en déplacement.
-- Sources :
--   av. n° 75 du 10/11/2022 (01/12/2022) — juristique.org, CNT-SO ;
--   av. n° 77 du 11/10/2023 (01/12/2023, étendu le 19/12/2023) — soluciaspj.fr ;
--   av. n° 79 du 06/02/2025 (01/03/2025, +1,5 %) — unostra.fr ;
--   av. n° 81 du 02/12/2025 (01/01/2026, +1 %, étendu le 03/02/2026) — juristique.org.
-- Avant le 01/12/2022 : historique non sourcé de bout en bout (pas d'entrée).
-- date_fin exclusive (ContextPaie : date_fin > date_paie). Montants unitaires
-- par indemnité ; « MENSUEL » ne sert qu'à satisfaire le CHECK du schéma.
-- ============================================================
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('IDCC16_REPAS_UNIQUE',       '2022-12-01', '2023-12-01', '9.35',  'MENSUEL'),
    ('IDCC16_REPAS_UNIQUE',       '2023-12-01', '2025-03-01', '9.82',  'MENSUEL'),
    ('IDCC16_REPAS_UNIQUE',       '2025-03-01', '2026-01-01', '9.97',  'MENSUEL'),
    ('IDCC16_REPAS_UNIQUE',       '2026-01-01', NULL,         '10.07', 'MENSUEL'),
    ('IDCC16_REPAS_UNIQUE_NUIT',  '2022-12-01', '2023-12-01', '9.11',  'MENSUEL'),
    ('IDCC16_REPAS_UNIQUE_NUIT',  '2023-12-01', '2025-03-01', '9.57',  'MENSUEL'),
    ('IDCC16_REPAS_UNIQUE_NUIT',  '2025-03-01', '2026-01-01', '9.71',  'MENSUEL'),
    ('IDCC16_REPAS_UNIQUE_NUIT',  '2026-01-01', NULL,         '9.81',  'MENSUEL'),
    ('IDCC16_INDEMNITE_SPECIALE', '2022-12-01', '2023-12-01', '4.11',  'MENSUEL'),
    ('IDCC16_INDEMNITE_SPECIALE', '2023-12-01', '2025-03-01', '4.32',  'MENSUEL'),
    ('IDCC16_INDEMNITE_SPECIALE', '2025-03-01', '2026-01-01', '4.38',  'MENSUEL'),
    ('IDCC16_INDEMNITE_SPECIALE', '2026-01-01', NULL,         '4.42',  'MENSUEL'),
    ('IDCC16_CASSE_CROUTE',       '2022-12-01', '2023-12-01', '8.24',  'MENSUEL'),
    ('IDCC16_CASSE_CROUTE',       '2023-12-01', '2025-03-01', '8.65',  'MENSUEL'),
    ('IDCC16_CASSE_CROUTE',       '2025-03-01', '2026-01-01', '8.78',  'MENSUEL'),
    ('IDCC16_CASSE_CROUTE',       '2026-01-01', NULL,         '8.87',  'MENSUEL');
