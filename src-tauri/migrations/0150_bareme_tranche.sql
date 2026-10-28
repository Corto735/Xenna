-- ============================================================
-- BARÈMES PAR TRANCHES EN BASE — table générique, pilote : le Japon
--
-- Les taux vivent en base depuis le début ; les barèmes (paliers, tranches
-- d'impôt, déductions par tranche) étaient écrits en dur dans le Rust, au fil
-- de `match annee`. Cette table les date comme les taux : une ligne par
-- tranche, `date_fin` exclusive (cf. 0136), lue par ContextPaie à la date de
-- paie. Trois lectures (db/context.rs) :
--   • bareme_palier     : valeur `fixe` de la première tranche où x < borne_sup ;
--   • bareme_lineaire   : x × taux + fixe de la première tranche où x ≤ borne_sup ;
--   • bareme_progressif : Σ des parts de x dans chaque tranche × taux.
-- borne_sup NULL = sans limite. Valeurs en TEXT, comme les taux.
--
-- Japon : valeurs reprises À L'IDENTIQUE du code qu'elles remplacent
-- (jp_cotisations.rs, jp_impot.rs), sources citées ligne à ligne. Dates :
--   • 標準報酬月額 : 32 paliers de pension depuis septembre 2020 (palier
--     650 000 ¥ ajouté) ; 50 paliers maladie depuis avril 2016 (1 390 000 ¥) ;
--   • 給与所得控除 et 基礎控除 : réforme applicable aux revenus 2020, puis
--     réformes 2025 et 2026 ;
--   • barème 所得税 : 7 tranches depuis 2015 (tranche à 45 %).
-- Avant ces dates, rien en base : le bulletin japonais se déclare non couvert
-- au lieu de prolonger un barème qui n'était pas le bon.
-- ============================================================

CREATE TABLE bareme_tranche (
    id         INTEGER PRIMARY KEY,
    code       TEXT    NOT NULL,
    date_debut TEXT    NOT NULL,
    date_fin   TEXT,
    rang       INTEGER NOT NULL,
    borne_sup  TEXT,
    taux       TEXT    NOT NULL DEFAULT '0',
    fixe       TEXT    NOT NULL DEFAULT '0',
    source     TEXT    NOT NULL,
    UNIQUE (code, date_debut, rang)
);
CREATE INDEX idx_bareme_tranche ON bareme_tranche (code, date_debut);

INSERT INTO bareme_tranche (code, date_debut, date_fin, rang, borne_sup, taux, fixe, source) VALUES
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 1, '93000', '0', '88000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 2, '101000', '0', '98000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 3, '107000', '0', '104000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 4, '114000', '0', '110000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 5, '122000', '0', '118000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 6, '130000', '0', '126000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 7, '138000', '0', '134000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 8, '146000', '0', '142000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 9, '155000', '0', '150000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 10, '165000', '0', '160000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 11, '175000', '0', '170000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 12, '185000', '0', '180000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 13, '195000', '0', '190000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 14, '210000', '0', '200000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 15, '230000', '0', '220000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 16, '250000', '0', '240000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 17, '270000', '0', '260000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 18, '290000', '0', '280000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 19, '310000', '0', '300000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 20, '330000', '0', '320000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 21, '350000', '0', '340000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 22, '370000', '0', '360000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 23, '395000', '0', '380000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 24, '425000', '0', '410000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 25, '455000', '0', '440000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 26, '485000', '0', '470000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 27, '515000', '0', '500000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 28, '545000', '0', '530000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 29, '575000', '0', '560000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 30, '605000', '0', '590000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 31, '635000', '0', '620000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KOSEI', '2020-09-01', NULL, 32, NULL, '0', '650000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 1, '93000', '0', '88000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 2, '101000', '0', '98000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 3, '107000', '0', '104000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 4, '114000', '0', '110000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 5, '122000', '0', '118000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 6, '130000', '0', '126000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 7, '138000', '0', '134000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 8, '146000', '0', '142000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 9, '155000', '0', '150000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 10, '165000', '0', '160000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 11, '175000', '0', '170000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 12, '185000', '0', '180000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 13, '195000', '0', '190000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 14, '210000', '0', '200000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 15, '230000', '0', '220000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 16, '250000', '0', '240000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 17, '270000', '0', '260000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 18, '290000', '0', '280000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 19, '310000', '0', '300000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 20, '330000', '0', '320000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 21, '350000', '0', '340000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 22, '370000', '0', '360000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 23, '395000', '0', '380000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 24, '425000', '0', '410000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 25, '455000', '0', '440000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 26, '485000', '0', '470000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 27, '515000', '0', '500000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 28, '545000', '0', '530000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 29, '575000', '0', '560000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 30, '605000', '0', '590000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 31, '635000', '0', '620000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 32, '665000', '0', '650000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 33, '695000', '0', '680000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 34, '730000', '0', '710000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 35, '770000', '0', '750000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 36, '810000', '0', '790000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 37, '855000', '0', '830000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 38, '905000', '0', '880000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 39, '955000', '0', '930000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 40, '1005000', '0', '980000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 41, '1055000', '0', '1030000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 42, '1115000', '0', '1090000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 43, '1175000', '0', '1150000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 44, '1235000', '0', '1210000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 45, '1295000', '0', '1270000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 46, '1355000', '0', '1330000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_HYOJUN_KENPO', '2016-04-01', NULL, 47, NULL, '0', '1390000', '健康保険法 art. 40 ; 厚生年金保険法 art. 20 (標準報酬月額)'),
  ('JP_KYUYO_KOJO', '2020-01-01', NULL, 1, '1800000', '0.40', '-100000', '所得税法 art. 28 (給与所得控除, réforme 2020)'),
  ('JP_KYUYO_KOJO', '2020-01-01', NULL, 2, '3600000', '0.30', '80000', '所得税法 art. 28 (給与所得控除, réforme 2020)'),
  ('JP_KYUYO_KOJO', '2020-01-01', NULL, 3, '6600000', '0.20', '440000', '所得税法 art. 28 (給与所得控除, réforme 2020)'),
  ('JP_KYUYO_KOJO', '2020-01-01', NULL, 4, '8500000', '0.10', '1100000', '所得税法 art. 28 (給与所得控除, réforme 2020)'),
  ('JP_KYUYO_KOJO', '2020-01-01', NULL, 5, NULL, '0', '1950000', '所得税法 art. 28 (給与所得控除, réforme 2020)'),
  ('JP_KISO_KOJO', '2020-01-01', '2025-01-01', 1, '23500000', '0', '480000', '所得税法 art. 86 (基礎控除) — 2020-2024'),
  ('JP_KISO_KOJO', '2020-01-01', '2025-01-01', 2, '24000000', '0', '480000', '所得税法 art. 86 (基礎控除) — 2020-2024'),
  ('JP_KISO_KOJO', '2020-01-01', '2025-01-01', 3, '24500000', '0', '320000', '所得税法 art. 86 (基礎控除) — 2020-2024'),
  ('JP_KISO_KOJO', '2020-01-01', '2025-01-01', 4, '25000000', '0', '160000', '所得税法 art. 86 (基礎控除) — 2020-2024'),
  ('JP_KISO_KOJO', '2020-01-01', '2025-01-01', 5, NULL, '0', '0', '所得税法 art. 86 (基礎控除) — 2020-2024'),
  ('JP_KISO_KOJO', '2025-01-01', '2026-01-01', 1, '1320000', '0', '950000', '所得税法 art. 86 (基礎控除) — réforme 2025'),
  ('JP_KISO_KOJO', '2025-01-01', '2026-01-01', 2, '3360000', '0', '880000', '所得税法 art. 86 (基礎控除) — réforme 2025'),
  ('JP_KISO_KOJO', '2025-01-01', '2026-01-01', 3, '4890000', '0', '680000', '所得税法 art. 86 (基礎控除) — réforme 2025'),
  ('JP_KISO_KOJO', '2025-01-01', '2026-01-01', 4, '6550000', '0', '630000', '所得税法 art. 86 (基礎控除) — réforme 2025'),
  ('JP_KISO_KOJO', '2025-01-01', '2026-01-01', 5, '23500000', '0', '580000', '所得税法 art. 86 (基礎控除) — réforme 2025'),
  ('JP_KISO_KOJO', '2025-01-01', '2026-01-01', 6, '24000000', '0', '480000', '所得税法 art. 86 (基礎控除) — réforme 2025'),
  ('JP_KISO_KOJO', '2025-01-01', '2026-01-01', 7, '24500000', '0', '320000', '所得税法 art. 86 (基礎控除) — réforme 2025'),
  ('JP_KISO_KOJO', '2025-01-01', '2026-01-01', 8, '25000000', '0', '160000', '所得税法 art. 86 (基礎控除) — réforme 2025'),
  ('JP_KISO_KOJO', '2025-01-01', '2026-01-01', 9, NULL, '0', '0', '所得税法 art. 86 (基礎控除) — réforme 2025'),
  ('JP_KISO_KOJO', '2026-01-01', NULL, 1, '4890000', '0', '1040000', '所得税法 art. 86 (基礎控除) — réforme 2026'),
  ('JP_KISO_KOJO', '2026-01-01', NULL, 2, '6550000', '0', '670000', '所得税法 art. 86 (基礎控除) — réforme 2026'),
  ('JP_KISO_KOJO', '2026-01-01', NULL, 3, '23500000', '0', '620000', '所得税法 art. 86 (基礎控除) — réforme 2026'),
  ('JP_KISO_KOJO', '2026-01-01', NULL, 4, '24000000', '0', '480000', '所得税法 art. 86 (基礎控除) — réforme 2026'),
  ('JP_KISO_KOJO', '2026-01-01', NULL, 5, '24500000', '0', '320000', '所得税法 art. 86 (基礎控除) — réforme 2026'),
  ('JP_KISO_KOJO', '2026-01-01', NULL, 6, '25000000', '0', '160000', '所得税法 art. 86 (基礎控除) — réforme 2026'),
  ('JP_KISO_KOJO', '2026-01-01', NULL, 7, NULL, '0', '0', '所得税法 art. 86 (基礎控除) — réforme 2026'),
  ('JP_SHOTOKU', '2015-01-01', NULL, 1, '1950000', '0.05', '0', '所得税法 art. 89 (barème, 7 tranches depuis 2015)'),
  ('JP_SHOTOKU', '2015-01-01', NULL, 2, '3300000', '0.10', '0', '所得税法 art. 89 (barème, 7 tranches depuis 2015)'),
  ('JP_SHOTOKU', '2015-01-01', NULL, 3, '6950000', '0.20', '0', '所得税法 art. 89 (barème, 7 tranches depuis 2015)'),
  ('JP_SHOTOKU', '2015-01-01', NULL, 4, '9000000', '0.23', '0', '所得税法 art. 89 (barème, 7 tranches depuis 2015)'),
  ('JP_SHOTOKU', '2015-01-01', NULL, 5, '18000000', '0.33', '0', '所得税法 art. 89 (barème, 7 tranches depuis 2015)'),
  ('JP_SHOTOKU', '2015-01-01', NULL, 6, '40000000', '0.40', '0', '所得税法 art. 89 (barème, 7 tranches depuis 2015)'),
  ('JP_SHOTOKU', '2015-01-01', NULL, 7, NULL, '0.45', '0', '所得税法 art. 89 (barème, 7 tranches depuis 2015)');

-- Paramètres scalaires du Japon, aux côtés des PMSS et SMIC.
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  -- Seuil d'affiliation du temps partiel (被用者保険の適用要件, 月額 8,8 万円).
  ('JP_SEUIL_AFFILIATION', '2016-10-01', NULL,         '88000',   'MENSUEL'),
  -- Minimum de la déduction d'emploi (所得税法 art. 28).
  ('JP_KYUYO_MIN',         '2020-01-01', '2025-01-01', '550000',  'ANNUEL'),
  ('JP_KYUYO_MIN',         '2025-01-01', '2026-01-01', '650000',  'ANNUEL'),
  ('JP_KYUYO_MIN',         '2026-01-01', NULL,         '740000',  'ANNUEL'),
  -- Même minimum pour la taxe locale, sans l'exception de 50 000 ¥ de 2026.
  ('JP_KYUYO_MIN_JUMIN',   '2020-01-01', '2025-01-01', '550000',  'ANNUEL'),
  ('JP_KYUYO_MIN_JUMIN',   '2025-01-01', '2026-01-01', '650000',  'ANNUEL'),
  ('JP_KYUYO_MIN_JUMIN',   '2026-01-01', NULL,         '690000',  'ANNUEL'),
  -- Déduction de base de la taxe locale (地方税法 art. 34 et 314-2).
  ('JP_JUMIN_KISO',        '2020-01-01', NULL,         '430000',  'ANNUEL'),
  -- Taxe locale : 6 % municipal + 4 % préfectoral en droit, 10 % au total.
  ('JP_JUMIN_TAUX',        '2020-01-01', NULL,         '0.10',    'ANNUEL'),
  -- Surtaxe de reconstruction (復興特別所得税), 2013 à 2037.
  ('JP_FUKKO_TAUX',        '2013-01-01', '2038-01-01', '0.021',   'ANNUEL');
