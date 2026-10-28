-- ============================================================
-- BARÈMES EN BASE — lot 1 : Royaume-Uni, Allemagne, Italie, Espagne
-- (cf. 0150 pour la table et ses lectures). Valeurs reprises À L'IDENTIQUE du
-- code qu'elles remplacent ; instantané avant/après sans écart.
-- ============================================================

-- ── Royaume-Uni ───────────────────────────────────────────────────────────────
-- Exercice du 6 avril au 5 avril. Les taux NI ne sont en base que depuis le
-- 06/04/2024 (exercice 2024/25) : les seuils aussi.
-- Seuils NI mensuels publiés par HMRC (arrondis à la livre) : PT £1 048 et UEL
-- £4 189 (£12 570 et £50 270 par an) ; ST £758 (£9 100) jusqu'au 05/04/2025,
-- £417 (£5 000) dès 2025/26 (National Insurance Contributions (Secondary Class 1
-- Contributions) Act 2025), inchangé en 2026/27 (GOV.UK, « Rates and thresholds
-- for employers 2026 to 2027 »).
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('UK_NI_PT',        '2024-04-06', NULL,         '1048', 'MENSUEL'),
  ('UK_NI_UEL',       '2024-04-06', NULL,         '4189', 'MENSUEL'),
  ('UK_NI_ST',        '2024-04-06', '2025-04-06', '758',  'MENSUEL'),
  ('UK_NI_ST',        '2025-04-06', NULL,         '417',  'MENSUEL'),
  ('UK_NI_TAUX_HAUT', '2024-04-06', NULL,         '0.02', 'MENSUEL');

-- Income Tax (England) : Personal Allowance £12 570, Basic Rate 20 % jusqu'à
-- £50 270, Higher Rate 40 % jusqu'à £125 140, Additional Rate 45 % au-delà
-- (Income Tax Act 2007 ; seuils gelés). Le retrait de la Personal Allowance
-- au-delà de £100 000 n'est pas modélisé.
INSERT INTO bareme_tranche (code, date_debut, date_fin, rang, borne_sup, taux, fixe, source) VALUES
  ('UK_IT', '2024-04-06', NULL, 1, '12570',  '0',    '0', 'Income Tax Act 2007 — Personal Allowance'),
  ('UK_IT', '2024-04-06', NULL, 2, '50270',  '0.20', '0', 'Income Tax Act 2007 — Basic Rate'),
  ('UK_IT', '2024-04-06', NULL, 3, '125140', '0.40', '0', 'Income Tax Act 2007 — Higher Rate'),
  ('UK_IT', '2024-04-06', NULL, 4, NULL,     '0.45', '0', 'Income Tax Act 2007 — Additional Rate');

-- ── Allemagne — Lohnsteuer (de_lohnsteuer.rs) ─────────────────────────────────
-- Paramètres annuels du tarif et des forfaits ; la formule du §32a (zones
-- quadratiques) reste dans le code. Valeurs reprises à l'identique, sauf UNE
-- correction : la part retraite de la Vorsorgepauschale était de 68 % pour
-- toutes les années jusqu'en 2017 ; l'échéancier légal (§39b Abs. 4 EStG :
-- +4 points par an jusqu'à 88 % en 2022, 100 % dès 2023) donne 60 % en 2015 et
-- 64 % en 2016.
-- DE_ESTG_Z1 / Z2 : fin des zones 1 et 2. Coefficients DE_ESTG_A1, A2, C2,
-- K42, K45 du tarif exact, publiés pour 2023 à 2026 ; avant 2023, faute de
-- coefficients relevés, de_lohnsteuer.rs garde son approximation (bornes
-- datées, coefficients 2023) — déclarée comme telle dans l'explication.
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('DE_GRUNDFREIBETRAG', '2015-01-01', '2016-01-01', '8472', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2016-01-01', '2017-01-01', '8652', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2017-01-01', '2018-01-01', '8820', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2018-01-01', '2019-01-01', '9000', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2019-01-01', '2020-01-01', '9168', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2020-01-01', '2021-01-01', '9408', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2021-01-01', '2022-01-01', '9744', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2022-01-01', '2023-01-01', '10347', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2023-01-01', '2024-01-01', '10908', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2024-01-01', '2025-01-01', '11784', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2025-01-01', '2026-01-01', '12096', 'ANNUEL'),  -- §32a EStG
  ('DE_GRUNDFREIBETRAG', '2026-01-01', NULL, '12348', 'ANNUEL'),  -- §32a EStG
  ('DE_ENTLASTUNG', '2015-01-01', '2020-01-01', '1908', 'ANNUEL'),  -- §24b EStG
  ('DE_ENTLASTUNG', '2020-01-01', '2023-01-01', '4008', 'ANNUEL'),  -- §24b EStG
  ('DE_ENTLASTUNG', '2023-01-01', NULL, '4260', 'ANNUEL'),  -- §24b EStG
  ('DE_AN_PAUSCH', '2015-01-01', '2022-01-01', '1000', 'ANNUEL'),  -- §9a EStG
  ('DE_AN_PAUSCH', '2022-01-01', '2023-01-01', '1200', 'ANNUEL'),  -- §9a EStG
  ('DE_AN_PAUSCH', '2023-01-01', NULL, '1230', 'ANNUEL'),  -- §9a EStG
  ('DE_VORSORGE_RV', '2015-01-01', '2016-01-01', '0.60', 'ANNUEL'),  -- §39b Abs. 4 EStG
  ('DE_VORSORGE_RV', '2016-01-01', '2017-01-01', '0.64', 'ANNUEL'),  -- §39b Abs. 4 EStG
  ('DE_VORSORGE_RV', '2017-01-01', '2018-01-01', '0.68', 'ANNUEL'),  -- §39b Abs. 4 EStG
  ('DE_VORSORGE_RV', '2018-01-01', '2019-01-01', '0.72', 'ANNUEL'),  -- §39b Abs. 4 EStG
  ('DE_VORSORGE_RV', '2019-01-01', '2020-01-01', '0.76', 'ANNUEL'),  -- §39b Abs. 4 EStG
  ('DE_VORSORGE_RV', '2020-01-01', '2021-01-01', '0.80', 'ANNUEL'),  -- §39b Abs. 4 EStG
  ('DE_VORSORGE_RV', '2021-01-01', '2022-01-01', '0.84', 'ANNUEL'),  -- §39b Abs. 4 EStG
  ('DE_VORSORGE_RV', '2022-01-01', '2023-01-01', '0.88', 'ANNUEL'),  -- §39b Abs. 4 EStG
  ('DE_VORSORGE_RV', '2023-01-01', NULL, '1', 'ANNUEL'),  -- §39b Abs. 4 EStG
  ('DE_SOLI_FREIGRENZE', '2015-01-01', '2021-01-01', '972', 'ANNUEL'),  -- §3 SolZG
  ('DE_SOLI_FREIGRENZE', '2021-01-01', '2023-01-01', '16956', 'ANNUEL'),  -- §3 SolZG
  ('DE_SOLI_FREIGRENZE', '2023-01-01', '2024-01-01', '17543', 'ANNUEL'),  -- §3 SolZG
  ('DE_SOLI_FREIGRENZE', '2024-01-01', '2025-01-01', '18130', 'ANNUEL'),  -- §3 SolZG
  ('DE_SOLI_FREIGRENZE', '2025-01-01', '2026-01-01', '19950', 'ANNUEL'),  -- §3 SolZG
  ('DE_SOLI_FREIGRENZE', '2026-01-01', NULL, '20350', 'ANNUEL'),  -- §3 SolZG
  ('DE_ESTG_Z1', '2015-01-01', '2016-01-01', '13469', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2016-01-01', '2017-01-01', '13669', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2017-01-01', '2018-01-01', '13769', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2018-01-01', '2019-01-01', '13996', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2019-01-01', '2020-01-01', '14254', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2020-01-01', '2021-01-01', '14532', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2021-01-01', '2022-01-01', '14754', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2022-01-01', '2023-01-01', '14927', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2023-01-01', '2024-01-01', '15999', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2024-01-01', '2025-01-01', '17005', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2025-01-01', '2026-01-01', '17443', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z1', '2026-01-01', NULL, '17799', 'ANNUEL'),  -- §32a EStG — fin de la zone 1
  ('DE_ESTG_Z2', '2015-01-01', '2016-01-01', '52881', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2016-01-01', '2017-01-01', '53665', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2017-01-01', '2018-01-01', '54057', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2018-01-01', '2019-01-01', '54949', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2019-01-01', '2020-01-01', '55960', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2020-01-01', '2021-01-01', '57051', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2021-01-01', '2022-01-01', '57918', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2022-01-01', '2023-01-01', '58597', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2023-01-01', '2024-01-01', '62809', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2024-01-01', '2025-01-01', '66760', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2025-01-01', '2026-01-01', '68480', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_Z2', '2026-01-01', NULL, '69878', 'ANNUEL'),  -- §32a EStG — fin de la zone 2
  ('DE_ESTG_A1', '2023-01-01', '2024-01-01', '979.18', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_A1', '2024-01-01', '2025-01-01', '954.80', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_A1', '2025-01-01', '2026-01-01', '932.30', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_A1', '2026-01-01', NULL, '914.51', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_A2', '2023-01-01', '2024-01-01', '192.59', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_A2', '2024-01-01', '2025-01-01', '181.19', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_A2', '2025-01-01', '2026-01-01', '176.64', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_A2', '2026-01-01', NULL, '173.10', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_C2', '2023-01-01', '2024-01-01', '966.53', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_C2', '2024-01-01', '2025-01-01', '991.21', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_C2', '2025-01-01', '2026-01-01', '1015.13', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_C2', '2026-01-01', NULL, '1034.87', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_K42', '2023-01-01', '2024-01-01', '9972.98', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_K42', '2024-01-01', '2025-01-01', '10636.31', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_K42', '2025-01-01', '2026-01-01', '10911.92', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_K42', '2026-01-01', NULL, '11135.63', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_K45', '2023-01-01', '2024-01-01', '18307.73', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_K45', '2024-01-01', '2025-01-01', '18971.06', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_K45', '2025-01-01', '2026-01-01', '19246.67', 'ANNUEL'),  -- §32a Abs. 1 EStG — tarif exact
  ('DE_ESTG_K45', '2026-01-01', NULL, '19470.38', 'ANNUEL');  -- §32a Abs. 1 EStG — tarif exact

-- ── Italie (it_irpef.rs, it_cotisations.rs) ───────────────────────────────────
-- IT_IRPEF (progressif) : scaglioni de l'art. 11 TUIR, quatre régimes. Les
-- montants cumulés que le code écrivait (3 450, 6 960, 17 220…) en découlent.
-- IT_BONUS_CUNEO (linéaire, dès 2025) : 7,1 % / 5,3 % / 4,8 % du revenu, puis
-- 1 000 € jusqu'à 32 000 €, puis 1 000 × (40 000 − r) / 8 000 = 5 000 − r/8 ;
-- au-delà de 40 000 €, rien (aucune tranche).
-- IT_ESONERO (linéaire à taux nul : `fixe` = point d'exonération) : réduction
-- de la part salariale IVS selon le brut mensuel, 2022 à 2024.
-- La détraction pour travail salarié (art. 13 TUIR) reste une formule dans le
-- code : ses fractions (÷ 27 000, ÷ 13 000, ÷ 22 000) ne s'écrivent pas en taux
-- sans perte de précision.
INSERT INTO bareme_tranche (code, date_debut, date_fin, rang, borne_sup, taux, fixe, source) VALUES
  ('IT_IRPEF', '2015-01-01', '2022-01-01', 1, '15000', '0.23', '0', 'DPR 917/1986 art. 11 (TUIR) — 5 scaglioni'),
  ('IT_IRPEF', '2015-01-01', '2022-01-01', 2, '28000', '0.27', '0', 'DPR 917/1986 art. 11 (TUIR) — 5 scaglioni'),
  ('IT_IRPEF', '2015-01-01', '2022-01-01', 3, '55000', '0.38', '0', 'DPR 917/1986 art. 11 (TUIR) — 5 scaglioni'),
  ('IT_IRPEF', '2015-01-01', '2022-01-01', 4, '75000', '0.41', '0', 'DPR 917/1986 art. 11 (TUIR) — 5 scaglioni'),
  ('IT_IRPEF', '2015-01-01', '2022-01-01', 5, NULL, '0.43', '0', 'DPR 917/1986 art. 11 (TUIR) — 5 scaglioni'),
  ('IT_IRPEF', '2022-01-01', '2024-01-01', 1, '15000', '0.23', '0', 'TUIR art. 11 — L. 234/2021 (4 scaglioni)'),
  ('IT_IRPEF', '2022-01-01', '2024-01-01', 2, '28000', '0.25', '0', 'TUIR art. 11 — L. 234/2021 (4 scaglioni)'),
  ('IT_IRPEF', '2022-01-01', '2024-01-01', 3, '50000', '0.35', '0', 'TUIR art. 11 — L. 234/2021 (4 scaglioni)'),
  ('IT_IRPEF', '2022-01-01', '2024-01-01', 4, NULL, '0.43', '0', 'TUIR art. 11 — L. 234/2021 (4 scaglioni)'),
  ('IT_IRPEF', '2024-01-01', '2026-01-01', 1, '28000', '0.23', '0', 'TUIR art. 11 — D.Lgs. 216/2023, L. 207/2024 (3 scaglioni)'),
  ('IT_IRPEF', '2024-01-01', '2026-01-01', 2, '50000', '0.35', '0', 'TUIR art. 11 — D.Lgs. 216/2023, L. 207/2024 (3 scaglioni)'),
  ('IT_IRPEF', '2024-01-01', '2026-01-01', 3, NULL, '0.43', '0', 'TUIR art. 11 — D.Lgs. 216/2023, L. 207/2024 (3 scaglioni)'),
  ('IT_IRPEF', '2026-01-01', NULL, 1, '28000', '0.23', '0', 'TUIR art. 11 — L. 199/2025 (Bilancio 2026)'),
  ('IT_IRPEF', '2026-01-01', NULL, 2, '50000', '0.33', '0', 'TUIR art. 11 — L. 199/2025 (Bilancio 2026)'),
  ('IT_IRPEF', '2026-01-01', NULL, 3, NULL, '0.43', '0', 'TUIR art. 11 — L. 199/2025 (Bilancio 2026)'),
  ('IT_BONUS_CUNEO', '2025-01-01', NULL, 1, '8500', '0.071', '0', 'L. 207/2024 art. 1 c. 4-9'),
  ('IT_BONUS_CUNEO', '2025-01-01', NULL, 2, '15000', '0.053', '0', 'L. 207/2024 art. 1 c. 4-9'),
  ('IT_BONUS_CUNEO', '2025-01-01', NULL, 3, '20000', '0.048', '0', 'L. 207/2024 art. 1 c. 4-9'),
  ('IT_BONUS_CUNEO', '2025-01-01', NULL, 4, '32000', '0', '1000', 'L. 207/2024 art. 1 c. 4-9'),
  ('IT_BONUS_CUNEO', '2025-01-01', NULL, 5, '40000', '-0.125', '5000', 'L. 207/2024 art. 1 c. 4-9'),
  ('IT_ESONERO', '2022-01-01', '2022-07-01', 1, '2692', '0', '0.008', 'L. 234/2021 art. 1 c. 121'),
  ('IT_ESONERO', '2022-07-01', '2023-01-01', 1, '2692', '0', '0.02', 'L. 234/2021 art. 1 c. 121 — DL 115/2022 art. 20'),
  ('IT_ESONERO', '2023-01-01', '2023-07-01', 1, '1923', '0', '0.03', 'L. 197/2022 art. 1 c. 281'),
  ('IT_ESONERO', '2023-01-01', '2023-07-01', 2, '2692', '0', '0.02', 'L. 197/2022 art. 1 c. 281'),
  ('IT_ESONERO', '2023-07-01', '2024-01-01', 1, '1923', '0', '0.07', 'L. 197/2022 art. 1 c. 281 — DL 48/2023 art. 39'),
  ('IT_ESONERO', '2023-07-01', '2024-01-01', 2, '2692', '0', '0.06', 'L. 197/2022 art. 1 c. 281 — DL 48/2023 art. 39'),
  ('IT_ESONERO', '2024-01-01', '2025-01-01', 1, '1923', '0', '0.07', 'L. 213/2023 art. 1 c. 15'),
  ('IT_ESONERO', '2024-01-01', '2025-01-01', 2, '2692', '0', '0.06', 'L. 213/2023 art. 1 c. 15');

-- ── Espagne (es_cotizaciones.rs, es_irpf.rs) ──────────────────────────────────
-- ES_BASE_MAX existait déjà (0032, 0120), aux valeurs du code : inchangée.
-- ES_BASE_MIN existait aussi mais FAUSSE et jamais lue : la 0032 y avait mis le
-- SMI mensuel (648,60 € en 2015…), alors que la base minimale des groupes 4 à 7
-- est le SMI majoré d'un sixième (prorata des deux paies extraordinaires :
-- 756,60 € en 2015, 1 381,20 € en 2025), valeur que le code appliquait ; seule
-- 2026 (1 424,40 €, 0120) l'était déjà. Remplacée par la série du code
-- (Órdenes de cotización annuelles ; changement au 1er septembre 2021).
DELETE FROM plafond_reference WHERE code = 'ES_BASE_MIN';
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('ES_BASE_MIN', '2015-01-01', '2016-01-01', '756.60', 'MENSUEL'),
  ('ES_BASE_MIN', '2016-01-01', '2017-01-01', '764.40', 'MENSUEL'),
  ('ES_BASE_MIN', '2017-01-01', '2018-01-01', '825.60', 'MENSUEL'),
  ('ES_BASE_MIN', '2018-01-01', '2019-01-01', '858.60', 'MENSUEL'),
  ('ES_BASE_MIN', '2019-01-01', '2020-01-01', '1050.00', 'MENSUEL'),
  ('ES_BASE_MIN', '2020-01-01', '2021-09-01', '1108.33', 'MENSUEL'),
  ('ES_BASE_MIN', '2021-09-01', '2022-01-01', '1125.83', 'MENSUEL'),
  ('ES_BASE_MIN', '2022-01-01', '2023-01-01', '1166.70', 'MENSUEL'),
  ('ES_BASE_MIN', '2023-01-01', '2024-01-01', '1260.00', 'MENSUEL'),
  ('ES_BASE_MIN', '2024-01-01', '2025-01-01', '1323.00', 'MENSUEL'),
  ('ES_BASE_MIN', '2025-01-01', '2026-01-01', '1381.20', 'MENSUEL'),
  ('ES_BASE_MIN', '2026-01-01', NULL, '1424.40', 'MENSUEL');

-- ES_SOLIDARIDAD_Tn_SAL/PAT : taux de la cotisation de solidarité (LGSS art.
-- 19 bis, RDL 2/2023) par tranche d'excédent sur la base maximale — jusqu'à
-- 10 %, de 10 à 50 %, au-delà (bornes fixées par la loi, restées dans le code).
-- Retenue IRPF (algorithme de l'AEAT, 2025 et 2026) : barème de retenue
-- ES_IRPF_ESCALA (progressif) ; réduction pour revenus du travail
-- ES_IRPF_REDUCCION (linéaire : 7 302 € puis 7 302 − 1,75 × (r − 14 852) =
-- 33 293 − 1,75 r, puis 2 364,34 − 1,14 × (r − 17 673,52) = 22 512,1528 − 1,14 r) ;
-- autres frais, minimum personnel, limite d'exonération et plafond de 43 %.
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('ES_SOLIDARIDAD_T1_SAL', '2025-01-01', '2026-01-01', '0.0015', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T1_PAT', '2025-01-01', '2026-01-01', '0.0077', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T2_SAL', '2025-01-01', '2026-01-01', '0.0017', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T2_PAT', '2025-01-01', '2026-01-01', '0.0083', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T3_SAL', '2025-01-01', '2026-01-01', '0.0019', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T3_PAT', '2025-01-01', '2026-01-01', '0.0098', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T1_SAL', '2026-01-01', NULL, '0.0019', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T1_PAT', '2026-01-01', NULL, '0.0096', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T2_SAL', '2026-01-01', NULL, '0.0021', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T2_PAT', '2026-01-01', NULL, '0.0104', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T3_SAL', '2026-01-01', NULL, '0.0024', 'MENSUEL'),
  ('ES_SOLIDARIDAD_T3_PAT', '2026-01-01', NULL, '0.0122', 'MENSUEL'),
  ('ES_IRPF_OTROS', '2025-01-01', NULL, '2000', 'ANNUEL'),
  ('ES_IRPF_MINIMO', '2025-01-01', NULL, '5550', 'ANNUEL'),
  ('ES_IRPF_LIMITE', '2025-01-01', NULL, '15876', 'ANNUEL'),
  ('ES_IRPF_PLAFOND_SEUIL', '2025-01-01', NULL, '35200', 'ANNUEL'),
  ('ES_IRPF_PLAFOND_TAUX', '2025-01-01', NULL, '0.43', 'ANNUEL');

INSERT INTO bareme_tranche (code, date_debut, date_fin, rang, borne_sup, taux, fixe, source) VALUES
  ('ES_IRPF_ESCALA', '2025-01-01', NULL, 1, '12450', '0.19', '0', 'Algoritmo AEAT — tabla 2 (art. 101 LIRPF)'),
  ('ES_IRPF_ESCALA', '2025-01-01', NULL, 2, '20200', '0.24', '0', 'Algoritmo AEAT — tabla 2 (art. 101 LIRPF)'),
  ('ES_IRPF_ESCALA', '2025-01-01', NULL, 3, '35200', '0.30', '0', 'Algoritmo AEAT — tabla 2 (art. 101 LIRPF)'),
  ('ES_IRPF_ESCALA', '2025-01-01', NULL, 4, '60000', '0.37', '0', 'Algoritmo AEAT — tabla 2 (art. 101 LIRPF)'),
  ('ES_IRPF_ESCALA', '2025-01-01', NULL, 5, '300000', '0.45', '0', 'Algoritmo AEAT — tabla 2 (art. 101 LIRPF)'),
  ('ES_IRPF_ESCALA', '2025-01-01', NULL, 6, NULL, '0.47', '0', 'Algoritmo AEAT — tabla 2 (art. 101 LIRPF)'),
  ('ES_IRPF_REDUCCION', '2025-01-01', NULL, 1, '14852', '0', '7302', 'Art. 20 LIRPF (RDL 4/2024)'),
  ('ES_IRPF_REDUCCION', '2025-01-01', NULL, 2, '17673.52', '-1.75', '33293', 'Art. 20 LIRPF (RDL 4/2024)'),
  ('ES_IRPF_REDUCCION', '2025-01-01', NULL, 3, '19747.50', '-1.14', '22512.1528', 'Art. 20 LIRPF (RDL 4/2024)'),
  ('ES_IRPF_REDUCCION', '2025-01-01', NULL, 4, NULL, '0', '0', 'Art. 20 LIRPF (RDL 4/2024)');
