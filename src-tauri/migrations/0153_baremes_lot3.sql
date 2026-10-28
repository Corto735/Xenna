
-- ============================================================
-- BARÈMES EN BASE — lot 3 : Australie, Nouvelle-Zélande, Pologne, Danemark,
-- Pays-Bas (cf. 0150). Valeurs reprises À L'IDENTIQUE ; instantané avant/après.
-- ============================================================

-- ── Australie (au_bulletin.rs) ────────────────────────────────────────────────
-- Barème des résidents par exercice (1er juillet → 30 juin). Le dernier, 2026-27,
-- s'arrête au 30/06/2027 au lieu de se prolonger : le taux de 14 % prévu pour
-- 2027-28 n'est pas intégré, prolonger 15 % serait faux. Medicare levy 2 %.
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('AU_MEDICARE_TAUX', '2014-07-01', NULL, '0.02', 'ANNUEL'),
  -- Plafond annuel de la Superannuation Guarantee (maximum contribution base) :
  -- 250 000 $ en 2025-26 (62 500 $ par trimestre), APPLIQUÉ AUSSI aux exercices
  -- antérieurs faute de relevé (approximation, reprise du code) ; 270 830 $ en
  -- 2026-27 (Payday Super, base devenue annuelle).
  ('AU_SUPER_PLAFOND', '2014-07-01', '2026-07-01', '250000', 'ANNUEL'),
  ('AU_SUPER_PLAFOND', '2026-07-01', NULL, '270830', 'ANNUEL');

INSERT INTO bareme_tranche (code, date_debut, date_fin, rang, borne_sup, taux, fixe, source) VALUES
  ('AU_IT', '2014-07-01', '2016-07-01', 1, '18200', '0', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2014-15 et 2015-16'),
  ('AU_IT', '2014-07-01', '2016-07-01', 2, '37000', '0.19', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2014-15 et 2015-16'),
  ('AU_IT', '2014-07-01', '2016-07-01', 3, '80000', '0.325', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2014-15 et 2015-16'),
  ('AU_IT', '2014-07-01', '2016-07-01', 4, '180000', '0.37', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2014-15 et 2015-16'),
  ('AU_IT', '2014-07-01', '2016-07-01', 5, NULL, '0.45', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2014-15 et 2015-16'),
  ('AU_IT', '2016-07-01', '2018-07-01', 1, '18200', '0', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2016-17 et 2017-18'),
  ('AU_IT', '2016-07-01', '2018-07-01', 2, '37000', '0.19', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2016-17 et 2017-18'),
  ('AU_IT', '2016-07-01', '2018-07-01', 3, '87000', '0.325', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2016-17 et 2017-18'),
  ('AU_IT', '2016-07-01', '2018-07-01', 4, '180000', '0.37', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2016-17 et 2017-18'),
  ('AU_IT', '2016-07-01', '2018-07-01', 5, NULL, '0.45', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2016-17 et 2017-18'),
  ('AU_IT', '2018-07-01', '2020-07-01', 1, '18200', '0', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2018-19 et 2019-20'),
  ('AU_IT', '2018-07-01', '2020-07-01', 2, '37000', '0.19', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2018-19 et 2019-20'),
  ('AU_IT', '2018-07-01', '2020-07-01', 3, '90000', '0.325', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2018-19 et 2019-20'),
  ('AU_IT', '2018-07-01', '2020-07-01', 4, '180000', '0.37', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2018-19 et 2019-20'),
  ('AU_IT', '2018-07-01', '2020-07-01', 5, NULL, '0.45', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2018-19 et 2019-20'),
  ('AU_IT', '2020-07-01', '2024-07-01', 1, '18200', '0', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2020-21 à 2023-24 (Stage 2)'),
  ('AU_IT', '2020-07-01', '2024-07-01', 2, '45000', '0.19', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2020-21 à 2023-24 (Stage 2)'),
  ('AU_IT', '2020-07-01', '2024-07-01', 3, '120000', '0.325', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2020-21 à 2023-24 (Stage 2)'),
  ('AU_IT', '2020-07-01', '2024-07-01', 4, '180000', '0.37', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2020-21 à 2023-24 (Stage 2)'),
  ('AU_IT', '2020-07-01', '2024-07-01', 5, NULL, '0.45', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2020-21 à 2023-24 (Stage 2)'),
  ('AU_IT', '2024-07-01', '2026-07-01', 1, '18200', '0', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2024-25 et 2025-26 (Stage 3)'),
  ('AU_IT', '2024-07-01', '2026-07-01', 2, '45000', '0.16', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2024-25 et 2025-26 (Stage 3)'),
  ('AU_IT', '2024-07-01', '2026-07-01', 3, '135000', '0.30', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2024-25 et 2025-26 (Stage 3)'),
  ('AU_IT', '2024-07-01', '2026-07-01', 4, '190000', '0.37', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2024-25 et 2025-26 (Stage 3)'),
  ('AU_IT', '2024-07-01', '2026-07-01', 5, NULL, '0.45', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2024-25 et 2025-26 (Stage 3)'),
  ('AU_IT', '2026-07-01', '2027-07-01', 1, '18200', '0', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2026-27 (More Cost of Living Relief Act 2025)'),
  ('AU_IT', '2026-07-01', '2027-07-01', 2, '45000', '0.15', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2026-27 (More Cost of Living Relief Act 2025)'),
  ('AU_IT', '2026-07-01', '2027-07-01', 3, '135000', '0.30', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2026-27 (More Cost of Living Relief Act 2025)'),
  ('AU_IT', '2026-07-01', '2027-07-01', 4, '190000', '0.37', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2026-27 (More Cost of Living Relief Act 2025)'),
  ('AU_IT', '2026-07-01', '2027-07-01', 5, NULL, '0.45', '0', 'Income Tax Rates Act 1986, Sch. 7 — 2026-27 (More Cost of Living Relief Act 2025)');

-- ── Nouvelle-Zélande (nz_bulletin.rs) ─────────────────────────────────────────
-- Barème PAYE par exercice (1er avril → 31 mars). Le dernier, 2026-27, s'arrête
-- au 31/03/2027 comme le code (seuils 2027-28 non relevés). ACC earner's levy :
-- taux et plafond annuel des gains par exercice.
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('NZ_ACC_TAUX', '2014-04-01', '2015-04-01', '0.0145', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2014-04-01', '2015-04-01', '118191', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2015-04-01', '2016-04-01', '0.0145', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2015-04-01', '2016-04-01', '120070', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2016-04-01', '2017-04-01', '0.0139', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2016-04-01', '2017-04-01', '122063', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2017-04-01', '2018-04-01', '0.0139', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2017-04-01', '2018-04-01', '124053', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2018-04-01', '2019-04-01', '0.0139', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2018-04-01', '2019-04-01', '126286', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2019-04-01', '2020-04-01', '0.0139', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2019-04-01', '2020-04-01', '128470', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2020-04-01', '2021-04-01', '0.0139', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2020-04-01', '2021-04-01', '130911', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2021-04-01', '2022-04-01', '0.0139', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2021-04-01', '2022-04-01', '130911', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2022-04-01', '2023-04-01', '0.0146', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2022-04-01', '2023-04-01', '136544', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2023-04-01', '2024-04-01', '0.0153', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2023-04-01', '2024-04-01', '139384', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2024-04-01', '2025-04-01', '0.0160', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2024-04-01', '2025-04-01', '142283', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2025-04-01', '2026-04-01', '0.0167', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2025-04-01', '2026-04-01', '152790', 'ANNUEL'),
  ('NZ_ACC_TAUX', '2026-04-01', NULL, '0.0175', 'ANNUEL'),
  ('NZ_ACC_PLAFOND', '2026-04-01', NULL, '156641', 'ANNUEL');

INSERT INTO bareme_tranche (code, date_debut, date_fin, rang, borne_sup, taux, fixe, source) VALUES
  ('NZ_PAYE', '2014-04-01', '2021-04-01', 1, '14000', '0.105', '0', 'Income Tax Act 2007, Sch. 1 — 2014-15 à 2020-21'),
  ('NZ_PAYE', '2014-04-01', '2021-04-01', 2, '48000', '0.175', '0', 'Income Tax Act 2007, Sch. 1 — 2014-15 à 2020-21'),
  ('NZ_PAYE', '2014-04-01', '2021-04-01', 3, '70000', '0.30', '0', 'Income Tax Act 2007, Sch. 1 — 2014-15 à 2020-21'),
  ('NZ_PAYE', '2014-04-01', '2021-04-01', 4, NULL, '0.33', '0', 'Income Tax Act 2007, Sch. 1 — 2014-15 à 2020-21'),
  ('NZ_PAYE', '2021-04-01', '2024-04-01', 1, '14000', '0.105', '0', 'Income Tax Act 2007, Sch. 1 — 2021-22 à 2023-24 (taux de 39 %)'),
  ('NZ_PAYE', '2021-04-01', '2024-04-01', 2, '48000', '0.175', '0', 'Income Tax Act 2007, Sch. 1 — 2021-22 à 2023-24 (taux de 39 %)'),
  ('NZ_PAYE', '2021-04-01', '2024-04-01', 3, '70000', '0.30', '0', 'Income Tax Act 2007, Sch. 1 — 2021-22 à 2023-24 (taux de 39 %)'),
  ('NZ_PAYE', '2021-04-01', '2024-04-01', 4, '180000', '0.33', '0', 'Income Tax Act 2007, Sch. 1 — 2021-22 à 2023-24 (taux de 39 %)'),
  ('NZ_PAYE', '2021-04-01', '2024-04-01', 5, NULL, '0.39', '0', 'Income Tax Act 2007, Sch. 1 — 2021-22 à 2023-24 (taux de 39 %)'),
  ('NZ_PAYE', '2024-04-01', '2027-04-01', 1, '15600', '0.105', '0', 'Income Tax Act 2007, Sch. 1 — 2024-25 à 2026-27 (seuils relevés en 2024)'),
  ('NZ_PAYE', '2024-04-01', '2027-04-01', 2, '53500', '0.175', '0', 'Income Tax Act 2007, Sch. 1 — 2024-25 à 2026-27 (seuils relevés en 2024)'),
  ('NZ_PAYE', '2024-04-01', '2027-04-01', 3, '78100', '0.30', '0', 'Income Tax Act 2007, Sch. 1 — 2024-25 à 2026-27 (seuils relevés en 2024)'),
  ('NZ_PAYE', '2024-04-01', '2027-04-01', 4, '180000', '0.33', '0', 'Income Tax Act 2007, Sch. 1 — 2024-25 à 2026-27 (seuils relevés en 2024)'),
  ('NZ_PAYE', '2024-04-01', '2027-04-01', 5, NULL, '0.39', '0', 'Income Tax Act 2007, Sch. 1 — 2024-25 à 2026-27 (seuils relevés en 2024)');

-- ── Pologne (pl_bulletin.rs) ──────────────────────────────────────────────────
-- Plafond annuel des cotisations retraite et invalidité (« 30-krotność »),
-- 2022 à 2026 — 2027 non relevé, d'où la date de fin ; structure du PIT stable
-- depuis la réforme Polski Ład 2.0 (2022) : 12 % / 32 % au-delà de 120 000 PLN,
-- montant réducteur 3 600 PLN, frais forfaitaires 3 000 PLN, santé 9 %.
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('PL_PLAFOND_30X', '2022-01-01', '2023-01-01', '177660', 'ANNUEL'),
  ('PL_PLAFOND_30X', '2023-01-01', '2024-01-01', '208050', 'ANNUEL'),
  ('PL_PLAFOND_30X', '2024-01-01', '2025-01-01', '234720', 'ANNUEL'),
  ('PL_PLAFOND_30X', '2025-01-01', '2026-01-01', '260190', 'ANNUEL'),
  ('PL_PLAFOND_30X', '2026-01-01', '2027-01-01', '282600', 'ANNUEL'),
  ('PL_KUP', '2022-01-01', NULL, '3000', 'ANNUEL'),
  ('PL_KWOTA_RELIEF', '2022-01-01', NULL, '3600', 'ANNUEL'),
  ('PL_PIT_SEUIL', '2022-01-01', NULL, '120000', 'ANNUEL'),
  ('PL_PIT_T1', '2022-01-01', NULL, '0.12', 'ANNUEL'),
  ('PL_PIT_T2', '2022-01-01', NULL, '0.32', 'ANNUEL'),
  ('PL_SANTE_TAUX', '2022-01-01', NULL, '0.09', 'ANNUEL');

-- ── Danemark (dk_bulletin.rs) ─────────────────────────────────────────────────
-- 2024 à 2026, d'où la date de fin au 31/12/2026 comme dans le code. Impôt
-- d'État DK_ETAT sur le revenu après AM-bidrag, en taux CUMULÉS : 0 % jusqu'au
-- seuil, puis 15 % (topskat, ≤ 2025), ou 7,5 / 15 / 20 % (mellem-, top- et
-- toptopskat, 2026). Bundskat, kommuneskat moyen, personfradrag, ATP et
-- déductions liées à l'emploi (beskæftigelsesfradrag, jobfradrag dès 2025).
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('DK_PERSONFRADRAG', '2024-01-01', '2025-01-01', '49700', 'ANNUEL'),
  ('DK_KOMMUNESKAT', '2024-01-01', '2025-01-01', '0.25067', 'ANNUEL'),
  ('DK_PERSONFRADRAG', '2025-01-01', '2026-01-01', '51600', 'ANNUEL'),
  ('DK_KOMMUNESKAT', '2025-01-01', '2026-01-01', '0.251', 'ANNUEL'),
  ('DK_PERSONFRADRAG', '2026-01-01', '2027-01-01', '54100', 'ANNUEL'),
  ('DK_KOMMUNESKAT', '2026-01-01', '2027-01-01', '0.25049', 'ANNUEL'),
  ('DK_BUNDSKAT', '2024-01-01', '2027-01-01', '0.1201', 'ANNUEL'),
  ('DK_ATP', '2024-01-01', '2026-01-01', '94.65', 'MENSUEL'),
  ('DK_ATP', '2026-01-01', '2027-01-01', '99.00', 'MENSUEL'),
  ('DK_BESK_TAUX', '2024-01-01', '2025-01-01', '0.1065', 'ANNUEL'),
  ('DK_BESK_MAX', '2024-01-01', '2025-01-01', '45100', 'ANNUEL'),
  ('DK_BESK_TAUX', '2025-01-01', '2026-01-01', '0.1230', 'ANNUEL'),
  ('DK_BESK_MAX', '2025-01-01', '2026-01-01', '55600', 'ANNUEL'),
  ('DK_BESK_TAUX', '2026-01-01', '2027-01-01', '0.1275', 'ANNUEL'),
  ('DK_BESK_MAX', '2026-01-01', '2027-01-01', '63300', 'ANNUEL'),
  ('DK_JOB_SEUIL', '2025-01-01', '2026-01-01', '224500', 'ANNUEL'),
  ('DK_JOB_MAX', '2025-01-01', '2026-01-01', '2900', 'ANNUEL'),
  ('DK_JOB_TAUX', '2025-01-01', '2026-01-01', '0.045', 'ANNUEL'),
  ('DK_JOB_SEUIL', '2026-01-01', '2027-01-01', '235200', 'ANNUEL'),
  ('DK_JOB_MAX', '2026-01-01', '2027-01-01', '3100', 'ANNUEL'),
  ('DK_JOB_TAUX', '2026-01-01', '2027-01-01', '0.045', 'ANNUEL');

INSERT INTO bareme_tranche (code, date_debut, date_fin, rang, borne_sup, taux, fixe, source) VALUES
  ('DK_ETAT', '2024-01-01', '2025-01-01', 1, '588900', '0', '0', 'Personskatteloven § 7 — topskat 2024'),
  ('DK_ETAT', '2024-01-01', '2025-01-01', 2, NULL, '0.15', '0', 'Personskatteloven § 7 — topskat 2024'),
  ('DK_ETAT', '2025-01-01', '2026-01-01', 1, '611800', '0', '0', 'Personskatteloven § 7 — topskat 2025'),
  ('DK_ETAT', '2025-01-01', '2026-01-01', 2, NULL, '0.15', '0', 'Personskatteloven § 7 — topskat 2025'),
  ('DK_ETAT', '2026-01-01', '2027-01-01', 1, '641200', '0', '0', 'Personskatteloven §§ 6a, 7, 7a — mellem-, top- et toptopskat 2026'),
  ('DK_ETAT', '2026-01-01', '2027-01-01', 2, '845543', '0.075', '0', 'Personskatteloven §§ 6a, 7, 7a — mellem-, top- et toptopskat 2026'),
  ('DK_ETAT', '2026-01-01', '2027-01-01', 3, '2592700', '0.15', '0', 'Personskatteloven §§ 6a, 7, 7a — mellem-, top- et toptopskat 2026'),
  ('DK_ETAT', '2026-01-01', '2027-01-01', 4, NULL, '0.20', '0', 'Personskatteloven §§ 6a, 7, 7a — mellem-, top- et toptopskat 2026');

-- ── Pays-Bas (nl_loonheffing.rs) ──────────────────────────────────────────────
-- Les 21 paramètres de la loonheffing (box 1, algemene heffingskorting,
-- arbeidskorting, premies volksverzekeringen, maximumpremieloon), un code
-- NL_<CHAMP> par champ de NlParams, 2022 à 2026 (tables officielles de la
-- Belastingdienst, relevées par année ; 2015-2021 et 2027 non sourcés, d'où les
-- dates de fin). Formules inchangées dans le code.
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('NL_B1_PLAFOND', '2022-01-01', '2023-01-01', '69398', 'ANNUEL'),
  ('NL_B1_PLAFOND', '2023-01-01', '2024-01-01', '73031', 'ANNUEL'),
  ('NL_B1_PLAFOND', '2024-01-01', '2025-01-01', '38098', 'ANNUEL'),
  ('NL_B1_PLAFOND', '2025-01-01', '2026-01-01', '38441', 'ANNUEL'),
  ('NL_B1_PLAFOND', '2026-01-01', '2027-01-01', '38883', 'ANNUEL'),
  ('NL_B1_TAUX', '2022-01-01', '2023-01-01', '0.3707', 'ANNUEL'),
  ('NL_B1_TAUX', '2023-01-01', '2024-01-01', '0.3693', 'ANNUEL'),
  ('NL_B1_TAUX', '2024-01-01', '2025-01-01', '0.3697', 'ANNUEL'),
  ('NL_B1_TAUX', '2025-01-01', '2026-01-01', '0.3582', 'ANNUEL'),
  ('NL_B1_TAUX', '2026-01-01', '2027-01-01', '0.3575', 'ANNUEL'),
  ('NL_B2_PLAFOND', '2022-01-01', '2023-01-01', '69398', 'ANNUEL'),
  ('NL_B2_PLAFOND', '2023-01-01', '2024-01-01', '73031', 'ANNUEL'),
  ('NL_B2_PLAFOND', '2024-01-01', '2025-01-01', '75518', 'ANNUEL'),
  ('NL_B2_PLAFOND', '2025-01-01', '2026-01-01', '76817', 'ANNUEL'),
  ('NL_B2_PLAFOND', '2026-01-01', '2027-01-01', '78426', 'ANNUEL'),
  ('NL_B2_TAUX', '2022-01-01', '2023-01-01', '0.3707', 'ANNUEL'),
  ('NL_B2_TAUX', '2023-01-01', '2024-01-01', '0.3693', 'ANNUEL'),
  ('NL_B2_TAUX', '2024-01-01', '2025-01-01', '0.3697', 'ANNUEL'),
  ('NL_B2_TAUX', '2025-01-01', '2026-01-01', '0.3748', 'ANNUEL'),
  ('NL_B2_TAUX', '2026-01-01', '2027-01-01', '0.3756', 'ANNUEL'),
  ('NL_B3_TAUX', '2022-01-01', '2023-01-01', '0.4950', 'ANNUEL'),
  ('NL_B3_TAUX', '2023-01-01', '2024-01-01', '0.4950', 'ANNUEL'),
  ('NL_B3_TAUX', '2024-01-01', '2025-01-01', '0.4950', 'ANNUEL'),
  ('NL_B3_TAUX', '2025-01-01', '2026-01-01', '0.4950', 'ANNUEL'),
  ('NL_B3_TAUX', '2026-01-01', '2027-01-01', '0.4950', 'ANNUEL'),
  ('NL_PREMIE_VV', '2022-01-01', '2023-01-01', '0.2765', 'ANNUEL'),
  ('NL_PREMIE_VV', '2023-01-01', '2024-01-01', '0.2765', 'ANNUEL'),
  ('NL_PREMIE_VV', '2024-01-01', '2025-01-01', '0.2765', 'ANNUEL'),
  ('NL_PREMIE_VV', '2025-01-01', '2026-01-01', '0.2765', 'ANNUEL'),
  ('NL_PREMIE_VV', '2026-01-01', '2027-01-01', '0.2765', 'ANNUEL'),
  ('NL_AHK_MAX', '2022-01-01', '2023-01-01', '2888', 'ANNUEL'),
  ('NL_AHK_MAX', '2023-01-01', '2024-01-01', '3070', 'ANNUEL'),
  ('NL_AHK_MAX', '2024-01-01', '2025-01-01', '3362', 'ANNUEL'),
  ('NL_AHK_MAX', '2025-01-01', '2026-01-01', '3068', 'ANNUEL'),
  ('NL_AHK_MAX', '2026-01-01', '2027-01-01', '3115', 'ANNUEL'),
  ('NL_AHK_SEUIL', '2022-01-01', '2023-01-01', '21317', 'ANNUEL'),
  ('NL_AHK_SEUIL', '2023-01-01', '2024-01-01', '22660', 'ANNUEL'),
  ('NL_AHK_SEUIL', '2024-01-01', '2025-01-01', '24812', 'ANNUEL'),
  ('NL_AHK_SEUIL', '2025-01-01', '2026-01-01', '28406', 'ANNUEL'),
  ('NL_AHK_SEUIL', '2026-01-01', '2027-01-01', '29736', 'ANNUEL'),
  ('NL_AHK_AFBOUW', '2022-01-01', '2023-01-01', '0.06007', 'ANNUEL'),
  ('NL_AHK_AFBOUW', '2023-01-01', '2024-01-01', '0.06095', 'ANNUEL'),
  ('NL_AHK_AFBOUW', '2024-01-01', '2025-01-01', '0.0663', 'ANNUEL'),
  ('NL_AHK_AFBOUW', '2025-01-01', '2026-01-01', '0.06337', 'ANNUEL'),
  ('NL_AHK_AFBOUW', '2026-01-01', '2027-01-01', '0.06398', 'ANNUEL'),
  ('NL_AK_S1_PLAFOND', '2022-01-01', '2023-01-01', '10350', 'ANNUEL'),
  ('NL_AK_S1_PLAFOND', '2023-01-01', '2024-01-01', '10741', 'ANNUEL'),
  ('NL_AK_S1_PLAFOND', '2024-01-01', '2025-01-01', '11490', 'ANNUEL'),
  ('NL_AK_S1_PLAFOND', '2025-01-01', '2026-01-01', '12169', 'ANNUEL'),
  ('NL_AK_S1_PLAFOND', '2026-01-01', '2027-01-01', '11965', 'ANNUEL'),
  ('NL_AK_S1_TAUX', '2022-01-01', '2023-01-01', '0.04541', 'ANNUEL'),
  ('NL_AK_S1_TAUX', '2023-01-01', '2024-01-01', '0.08231', 'ANNUEL'),
  ('NL_AK_S1_TAUX', '2024-01-01', '2025-01-01', '0.08425', 'ANNUEL'),
  ('NL_AK_S1_TAUX', '2025-01-01', '2026-01-01', '0.08053', 'ANNUEL'),
  ('NL_AK_S1_TAUX', '2026-01-01', '2027-01-01', '0.08324', 'ANNUEL'),
  ('NL_AK_S2_PLAFOND', '2022-01-01', '2023-01-01', '22356', 'ANNUEL'),
  ('NL_AK_S2_PLAFOND', '2023-01-01', '2024-01-01', '23201', 'ANNUEL'),
  ('NL_AK_S2_PLAFOND', '2024-01-01', '2025-01-01', '24820', 'ANNUEL'),
  ('NL_AK_S2_PLAFOND', '2025-01-01', '2026-01-01', '26288', 'ANNUEL'),
  ('NL_AK_S2_PLAFOND', '2026-01-01', '2027-01-01', '25845', 'ANNUEL'),
  ('NL_AK_S2_BASE', '2022-01-01', '2023-01-01', '470', 'ANNUEL'),
  ('NL_AK_S2_BASE', '2023-01-01', '2024-01-01', '884', 'ANNUEL'),
  ('NL_AK_S2_BASE', '2024-01-01', '2025-01-01', '968', 'ANNUEL'),
  ('NL_AK_S2_BASE', '2025-01-01', '2026-01-01', '980', 'ANNUEL'),
  ('NL_AK_S2_BASE', '2026-01-01', '2027-01-01', '996', 'ANNUEL'),
  ('NL_AK_S2_TAUX', '2022-01-01', '2023-01-01', '0.28461', 'ANNUEL'),
  ('NL_AK_S2_TAUX', '2023-01-01', '2024-01-01', '0.29861', 'ANNUEL'),
  ('NL_AK_S2_TAUX', '2024-01-01', '2025-01-01', '0.31433', 'ANNUEL'),
  ('NL_AK_S2_TAUX', '2025-01-01', '2026-01-01', '0.30030', 'ANNUEL'),
  ('NL_AK_S2_TAUX', '2026-01-01', '2027-01-01', '0.31009', 'ANNUEL'),
  ('NL_AK_S3_PLAFOND', '2022-01-01', '2023-01-01', '36649', 'ANNUEL'),
  ('NL_AK_S3_PLAFOND', '2023-01-01', '2024-01-01', '37691', 'ANNUEL'),
  ('NL_AK_S3_PLAFOND', '2024-01-01', '2025-01-01', '39957', 'ANNUEL'),
  ('NL_AK_S3_PLAFOND', '2025-01-01', '2026-01-01', '43071', 'ANNUEL'),
  ('NL_AK_S3_PLAFOND', '2026-01-01', '2027-01-01', '45592', 'ANNUEL'),
  ('NL_AK_S3_BASE', '2022-01-01', '2023-01-01', '3887', 'ANNUEL'),
  ('NL_AK_S3_BASE', '2023-01-01', '2024-01-01', '4605', 'ANNUEL'),
  ('NL_AK_S3_BASE', '2024-01-01', '2025-01-01', '5158', 'ANNUEL'),
  ('NL_AK_S3_BASE', '2025-01-01', '2026-01-01', '5220', 'ANNUEL'),
  ('NL_AK_S3_BASE', '2026-01-01', '2027-01-01', '5300', 'ANNUEL'),
  ('NL_AK_S3_TAUX', '2022-01-01', '2023-01-01', '0.02610', 'ANNUEL'),
  ('NL_AK_S3_TAUX', '2023-01-01', '2024-01-01', '0.03085', 'ANNUEL'),
  ('NL_AK_S3_TAUX', '2024-01-01', '2025-01-01', '0.02471', 'ANNUEL'),
  ('NL_AK_S3_TAUX', '2025-01-01', '2026-01-01', '0.02258', 'ANNUEL'),
  ('NL_AK_S3_TAUX', '2026-01-01', '2027-01-01', '0.01950', 'ANNUEL'),
  ('NL_AK_MAX', '2022-01-01', '2023-01-01', '4260', 'ANNUEL'),
  ('NL_AK_MAX', '2023-01-01', '2024-01-01', '5052', 'ANNUEL'),
  ('NL_AK_MAX', '2024-01-01', '2025-01-01', '5532', 'ANNUEL'),
  ('NL_AK_MAX', '2025-01-01', '2026-01-01', '5599', 'ANNUEL'),
  ('NL_AK_MAX', '2026-01-01', '2027-01-01', '5685', 'ANNUEL'),
  ('NL_AK_AFBOUW_SEUIL', '2022-01-01', '2023-01-01', '36649', 'ANNUEL'),
  ('NL_AK_AFBOUW_SEUIL', '2023-01-01', '2024-01-01', '37691', 'ANNUEL'),
  ('NL_AK_AFBOUW_SEUIL', '2024-01-01', '2025-01-01', '39957', 'ANNUEL'),
  ('NL_AK_AFBOUW_SEUIL', '2025-01-01', '2026-01-01', '43071', 'ANNUEL'),
  ('NL_AK_AFBOUW_SEUIL', '2026-01-01', '2027-01-01', '45592', 'ANNUEL'),
  ('NL_AK_AFBOUW', '2022-01-01', '2023-01-01', '0.0586', 'ANNUEL'),
  ('NL_AK_AFBOUW', '2023-01-01', '2024-01-01', '0.06510', 'ANNUEL'),
  ('NL_AK_AFBOUW', '2024-01-01', '2025-01-01', '0.06510', 'ANNUEL'),
  ('NL_AK_AFBOUW', '2025-01-01', '2026-01-01', '0.06510', 'ANNUEL'),
  ('NL_AK_AFBOUW', '2026-01-01', '2027-01-01', '0.06510', 'ANNUEL'),
  ('NL_MAX_PREMIELOON', '2022-01-01', '2023-01-01', '59706', 'ANNUEL'),
  ('NL_MAX_PREMIELOON', '2023-01-01', '2024-01-01', '66956', 'ANNUEL'),
  ('NL_MAX_PREMIELOON', '2024-01-01', '2025-01-01', '71628', 'ANNUEL'),
  ('NL_MAX_PREMIELOON', '2025-01-01', '2026-01-01', '75864', 'ANNUEL'),
  ('NL_MAX_PREMIELOON', '2026-01-01', '2027-01-01', '79409', 'ANNUEL');
