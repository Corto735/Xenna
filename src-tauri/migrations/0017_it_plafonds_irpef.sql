-- ============================================================
-- ITALIE — Plafonds contributifs et tables IRPEF
--
-- MASSIMALE CONTRIBUTIVO IVS
-- S'applique UNIQUEMENT aux salariés sans ancienneté INPS au 31/12/1995
-- (regime contributivo puro — L. 335/1995).
-- Pour les salariés pré-1996 : aucun plafond IVS (retribuzione totale).
-- Sources : Circolari INPS annuali (publiées en janvier).
-- Valeurs en € — periodicite = ANNUEL.
-- ============================================================

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('IT_MASSIMALE_IVS', '2015-01-01', '2015-12-31', '100324.00', 'ANNUEL'),  -- circ. INPS 13/2015
  ('IT_MASSIMALE_IVS', '2016-01-01', '2016-12-31', '100324.00', 'ANNUEL'),  -- circ. INPS 16/2016
  ('IT_MASSIMALE_IVS', '2017-01-01', '2017-12-31', '100324.00', 'ANNUEL'),  -- circ. INPS 21/2017 (approx.)
  ('IT_MASSIMALE_IVS', '2018-01-01', '2018-12-31', '101427.00', 'ANNUEL'),  -- circ. INPS 17/2018 (approx.)
  ('IT_MASSIMALE_IVS', '2019-01-01', '2019-12-31', '102543.00', 'ANNUEL'),  -- circ. INPS 12/2019
  ('IT_MASSIMALE_IVS', '2020-01-01', '2020-12-31', '103055.00', 'ANNUEL'),  -- circ. INPS 12/2020
  ('IT_MASSIMALE_IVS', '2021-01-01', '2021-12-31', '103055.00', 'ANNUEL'),  -- gelé — circ. INPS 50/2021
  ('IT_MASSIMALE_IVS', '2022-01-01', '2022-12-31', '103055.00', 'ANNUEL'),  -- gelé maintenu
  ('IT_MASSIMALE_IVS', '2023-01-01', '2023-12-31', '105014.00', 'ANNUEL'),  -- circ. INPS 7/2023
  ('IT_MASSIMALE_IVS', '2024-01-01', '2024-12-31', '119650.00', 'ANNUEL'),  -- circ. INPS 3/2024 (+13,9 %)
  ('IT_MASSIMALE_IVS', '2025-01-01', NULL,          '123379.00', 'ANNUEL'); -- estimation ISTAT ~3,1 %

-- ============================================================
-- IRPEF — Barème progressif par année
-- ============================================================
CREATE TABLE IF NOT EXISTS irpef_tranche (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    date_debut  DATE    NOT NULL,
    date_fin    DATE,
    tranche     INTEGER NOT NULL,
    seuil_bas   TEXT    NOT NULL DEFAULT '0',
    seuil_haut  TEXT,              -- NULL = illimité (dernière tranche)
    taux        TEXT    NOT NULL,
    loi_ref     TEXT,
    notes       TEXT
);

-- 2015–2021 : 5 tranches (DPR 917/1986)
INSERT INTO irpef_tranche (date_debut, date_fin, tranche, seuil_bas, seuil_haut, taux, loi_ref, notes) VALUES
  ('2015-01-01','2021-12-31', 1, '0',     '15000',  '0.23', 'DPR 917/1986 art. 11', NULL),
  ('2015-01-01','2021-12-31', 2, '15000', '28000',  '0.27', 'DPR 917/1986 art. 11', NULL),
  ('2015-01-01','2021-12-31', 3, '28000', '55000',  '0.38', 'DPR 917/1986 art. 11', NULL),
  ('2015-01-01','2021-12-31', 4, '55000', '75000',  '0.41', 'DPR 917/1986 art. 11', NULL),
  ('2015-01-01','2021-12-31', 5, '75000', NULL,     '0.43', 'DPR 917/1986 art. 11', NULL);

-- 2022–2023 : 4 tranches (L. 234/2021 — Bilancio 2022)
INSERT INTO irpef_tranche (date_debut, date_fin, tranche, seuil_bas, seuil_haut, taux, loi_ref, notes) VALUES
  ('2022-01-01','2023-12-31', 1, '0',     '15000',  '0.23', 'L. 234/2021 art. 1 c. 2', 'Taux inchangé'),
  ('2022-01-01','2023-12-31', 2, '15000', '28000',  '0.25', 'L. 234/2021 art. 1 c. 2', '27 % → 25 %'),
  ('2022-01-01','2023-12-31', 3, '28000', '50000',  '0.35', 'L. 234/2021 art. 1 c. 2', '38 % → 35 %, seuil 55k → 50k'),
  ('2022-01-01','2023-12-31', 4, '50000', NULL,     '0.43', 'L. 234/2021 art. 1 c. 2', 'Seuil 75k → 50k');

-- 2024+ : 3 tranches (L. 213/2023 — Bilancio 2024)
INSERT INTO irpef_tranche (date_debut, date_fin, tranche, seuil_bas, seuil_haut, taux, loi_ref, notes) VALUES
  ('2024-01-01', NULL, 1, '0',     '28000', '0.23', 'L. 213/2023 art. 1 c. 2', 'Fusion tranches 1+2 à 23 %'),
  ('2024-01-01', NULL, 2, '28000', '50000', '0.35', 'L. 213/2023 art. 1 c. 2', NULL),
  ('2024-01-01', NULL, 3, '50000', NULL,    '0.43', 'L. 213/2023 art. 1 c. 2', NULL);

-- ============================================================
-- IRPEF — Détractions travail salarié (art. 13 TUIR)
-- ============================================================
CREATE TABLE IF NOT EXISTS irpef_detrazione_lavdip (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    date_debut      DATE    NOT NULL,
    date_fin        DATE,
    seuil_bas       TEXT    NOT NULL DEFAULT '0',
    seuil_haut      TEXT,
    detrazione_fixe TEXT,
    formule_type    TEXT    NOT NULL,  -- 'FIXE', 'MIXTE', 'LINEAIRE', 'ZERO'
    loi_ref         TEXT,
    notes           TEXT
);

-- 2015–2021 (art. 13 TUIR original)
INSERT INTO irpef_detrazione_lavdip
  (date_debut, date_fin, seuil_bas, seuil_haut, detrazione_fixe, formule_type, loi_ref, notes) VALUES
  ('2015-01-01','2021-12-31','0',    '8000',  '1880','FIXE',     'DPR 917/1986 art. 13','Min 690 € si emploi partiel'),
  ('2015-01-01','2021-12-31','8000', '28000', '902', 'MIXTE',    'DPR 917/1986 art. 13','902 + 978×(28000−reddito)/20000'),
  ('2015-01-01','2021-12-31','28000','55000', '978', 'LINEAIRE', 'DPR 917/1986 art. 13','978×(55000−reddito)/27000'),
  ('2015-01-01','2021-12-31','55000', NULL,   '0',   'ZERO',     'DPR 917/1986 art. 13', NULL);

-- 2022–2023 (L. 234/2021 — relèvement seuils)
INSERT INTO irpef_detrazione_lavdip
  (date_debut, date_fin, seuil_bas, seuil_haut, detrazione_fixe, formule_type, loi_ref, notes) VALUES
  ('2022-01-01','2023-12-31','0',    '15000', '1880','FIXE',     'L. 234/2021','Min 690 €'),
  ('2022-01-01','2023-12-31','15000','28000', '1910','FIXE',     'L. 234/2021','€1 910 plat : allègement classe moyenne'),
  ('2022-01-01','2023-12-31','28000','50000', '1910','LINEAIRE', 'L. 234/2021','1910×(50000−reddito)/22000'),
  ('2022-01-01','2023-12-31','50000', NULL,   '0',   'ZERO',     'L. 234/2021', NULL);

-- 2024+ (L. 213/2023 — relèvement 1ère tranche à 1955 €)
INSERT INTO irpef_detrazione_lavdip
  (date_debut, date_fin, seuil_bas, seuil_haut, detrazione_fixe, formule_type, loi_ref, notes) VALUES
  ('2024-01-01', NULL, '0',    '15000', '1955','FIXE',     'L. 213/2023','Min 690 € si emploi partiel'),
  ('2024-01-01', NULL, '15000','28000', '1910','FIXE',     'L. 213/2023','€1 910 plat'),
  ('2024-01-01', NULL, '28000','50000', '1910','LINEAIRE', 'L. 213/2023','1910×(50000−reddito)/22000'),
  ('2024-01-01', NULL, '50000', NULL,   '0',   'ZERO',     'L. 213/2023', NULL);

-- ============================================================
-- IRPEF — Addizionale regionale (taux de base 2024)
-- Délibérations régionales — à vérifier chaque année.
-- Certaines régions ont des tranches supplémentaires pour hauts revenus
-- non reproduites ici (voir notes).
-- ============================================================
CREATE TABLE IF NOT EXISTS irpef_addizionale_regionale (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    regione_code    TEXT    NOT NULL,
    regione_libelle TEXT    NOT NULL,
    annee           INTEGER NOT NULL,
    taux_base       TEXT    NOT NULL,
    notes           TEXT
);

INSERT INTO irpef_addizionale_regionale (regione_code, regione_libelle, annee, taux_base, notes) VALUES
  ('AB', 'Abruzzo',                2024, '0.0173', NULL),
  ('BS', 'Basilicata',             2024, '0.0090', '+0,30 pp au-delà de 75 000 € de revenu'),
  ('BZ', 'Bolzano / Südtirol',     2024, '0.0090', 'Provincia autonoma'),
  ('CL', 'Calabria',               2024, '0.0203', 'Plan de redressement fiscal régional'),
  ('CP', 'Campania',               2024, '0.0203', NULL),
  ('ER', 'Emilia-Romagna',         2024, '0.0090', 'Tranches supplémentaires progressives jusqu''à 2,19 %'),
  ('FV', 'Friuli-Venezia Giulia',  2024, '0.0070', NULL),
  ('LA', 'Lazio',                  2024, '0.0173', NULL),
  ('LG', 'Liguria',                2024, '0.0090', NULL),
  ('LO', 'Lombardia',              2024, '0.0123', NULL),
  ('MR', 'Marche',                 2024, '0.0153', NULL),
  ('ML', 'Molise',                 2024, '0.0203', NULL),
  ('PM', 'Piemonte',               2024, '0.0095', 'Tranches supplémentaires jusqu''à 2,33 %'),
  ('PG', 'Puglia',                 2024, '0.0197', NULL),
  ('SR', 'Sardegna',               2024, '0.0049', 'Statut régional spécial'),
  ('SC', 'Sicilia',                2024, '0.0100', 'Statut régional spécial'),
  ('TS', 'Toscana',                2024, '0.0142', NULL),
  ('TN', 'Trento',                 2024, '0.0060', 'Provincia autonoma'),
  ('UM', 'Umbria',                 2024, '0.0135', 'Tranche supplémentaire 1,62 % au-delà de 28 000 €'),
  ('VA', 'Valle d''Aosta',         2024, '0.0090', 'Statut régional spécial'),
  ('VE', 'Veneto',                 2024, '0.0090', 'Tranches supplémentaires jusqu''à 1,21 %');
