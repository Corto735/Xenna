-- ============================================================
-- ALLEMAGNE — Plafonds de cotisation (Beitragsbemessungsgrenzen)
-- Historique 2015 → 2026
--
-- DE_BBG_KV : plafond KV + PV (identique)
-- DE_BBG_RV : plafond RV + AV (identique)
--
-- Sources :
--   SGB V / SGB VI / SGB XI / SGB III
--   Deutsche Rentenversicherung — Rechenwerte de chaque année
--   Bundesregierung — communiqué BBG 2026
--
-- ⚠️ Avant 2025, la BBG RV distinguait Alte Länder (Ouest) et
--    Neue Länder (Est). Ce simulateur utilise exclusivement la
--    valeur Alte Länder (Ouest), la plus courante.
--    À partir du 01/01/2025, la BBG est unifiée (loi RV-Stabilitäts-G).
-- ============================================================

-- ── DE_BBG_KV (= plafond PV) — mensuel ──────────────────────

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('DE_BBG_KV', '2015-01-01', '2015-12-31', '4125.00',  'MENSUEL'),
    ('DE_BBG_KV', '2016-01-01', '2016-12-31', '4237.50',  'MENSUEL'),
    ('DE_BBG_KV', '2017-01-01', '2017-12-31', '4350.00',  'MENSUEL'),
    ('DE_BBG_KV', '2018-01-01', '2018-12-31', '4425.00',  'MENSUEL'),
    ('DE_BBG_KV', '2019-01-01', '2019-12-31', '4537.50',  'MENSUEL'),
    ('DE_BBG_KV', '2020-01-01', '2020-12-31', '4687.50',  'MENSUEL'),
    ('DE_BBG_KV', '2021-01-01', '2022-12-31', '4837.50',  'MENSUEL'), -- gel 2022
    ('DE_BBG_KV', '2023-01-01', '2023-12-31', '4987.50',  'MENSUEL'),
    ('DE_BBG_KV', '2024-01-01', '2024-12-31', '5175.00',  'MENSUEL'),
    ('DE_BBG_KV', '2025-01-01', '2025-12-31', '5512.50',  'MENSUEL'),
    ('DE_BBG_KV', '2026-01-01', NULL,          '5812.50',  'MENSUEL');

-- ── DE_BBG_RV (= plafond AV) — mensuel ──────────────────────

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('DE_BBG_RV', '2015-01-01', '2015-12-31', '6050.00',  'MENSUEL'),
    ('DE_BBG_RV', '2016-01-01', '2016-12-31', '6200.00',  'MENSUEL'),
    ('DE_BBG_RV', '2017-01-01', '2017-12-31', '6350.00',  'MENSUEL'),
    ('DE_BBG_RV', '2018-01-01', '2018-12-31', '6500.00',  'MENSUEL'),
    ('DE_BBG_RV', '2019-01-01', '2019-12-31', '6700.00',  'MENSUEL'),
    ('DE_BBG_RV', '2020-01-01', '2020-12-31', '6900.00',  'MENSUEL'),
    ('DE_BBG_RV', '2021-01-01', '2021-12-31', '7100.00',  'MENSUEL'),
    ('DE_BBG_RV', '2022-01-01', '2022-12-31', '7050.00',  'MENSUEL'), -- baisse exceptionnelle
    ('DE_BBG_RV', '2023-01-01', '2023-12-31', '7300.00',  'MENSUEL'),
    ('DE_BBG_RV', '2024-01-01', '2024-12-31', '7550.00',  'MENSUEL'),
    ('DE_BBG_RV', '2025-01-01', '2025-12-31', '8050.00',  'MENSUEL'), -- unification Est/Ouest
    ('DE_BBG_RV', '2026-01-01', NULL,          '8450.00',  'MENSUEL');
