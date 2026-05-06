-- ============================================================
-- ALLEMAGNE — Cotisations sociales (taux au 01/01/2026)
-- Sozialversicherungsbeiträge + Lohnsteuer + Kirchensteuer + Soli
--
-- KV  : 14,6 % allg. + 2,9 % Zusatzbeitrag moyen 2026 = 17,5 % total
--        partagé à 50/50 depuis le 01/01/2019
-- RV  : 18,6 % total — 9,3 % chacun (stable depuis 2018)
-- AV  : 2,6 % total — 1,3 % chacun (stable depuis 2023)
-- PV  : 3,6 % total — 1,8 % chacun (avec enfants) ; + 0,6 % sal sans enfant
-- UV  : ~1,3 % patronal (taux moyen DGUV indicatif — variable par BG)
--
-- Lohnsteuer, Kirchensteuer et Soli sont calculés dynamiquement
-- par le backend (de_lohnsteuer.rs) selon Steuerklasse, Land et année.
-- Ils n'ont pas d'entrée dans cotisation_taux (paramètres complexes).
-- ============================================================

-- ── Définition des cotisations ───────────────────────────────
INSERT INTO cotisation (code, libelle, organisme_id, categorie,
  applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES

  ('DE_KRANKENVERSICHERUNG',
   'KV — Krankenversicherung (assurance maladie)',
   (SELECT id FROM organisme WHERE code='GKV'),
   'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   'Taux 2026 : 14,6 % allg. + 2,9 % Zusatzbeitrag moyen = 17,5 % total (8,75 % chacun). '
   || 'Depuis le 01/01/2019, le Zusatzbeitrag est partagé à parts égales entre salarié et employeur (avant : 100 % salarié). '
   || 'Plafond : Beitragsbemessungsgrenze KV (5 812,50 €/mois en 2026).'),

  ('DE_RENTENVERSICHERUNG',
   'RV — Rentenversicherung (assurance retraite)',
   (SELECT id FROM organisme WHERE code='DRV'),
   'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ',
   'Taux 2026 : 18,6 % total (9,3 % chacun). Stable depuis 2018 (était 18,7 % en 2015-2017). '
   || 'Plafond : Beitragsbemessungsgrenze RV (8 450 €/mois en 2026 — unifié depuis 2025).'),

  ('DE_ARBEITSLOSENVERSICHERUNG',
   'AV — Arbeitslosenversicherung (assurance chômage)',
   (SELECT id FROM organisme WHERE code='BA_DE'),
   'CHOMAGE', 1, 1, 'BRUT_PLAFONNÉ',
   'Taux 2026 : 2,6 % total (1,3 % chacun). '
   || 'Historique : 3,0 % (2015-2018) → 2,6 % (2019) → 2,4 % (2020-2022) → 2,6 % (2023+). '
   || 'Même plafond que RV.'),

  ('DE_PFLEGEVERSICHERUNG',
   'PV — Pflegeversicherung (assurance dépendance — avec enfants)',
   (SELECT id FROM organisme WHERE code='GKV'),
   'PREVOYANCE', 1, 1, 'BRUT_PLAFONNÉ',
   'Taux 2026 : 3,6 % total (1,8 % chacun). S''applique aux parents. '
   || 'Réforme 2023 (PUEG, 01/07/2023) : 3,40 % → réductions supplémentaires pour familles nombreuses non simulées ici. '
   || 'Même plafond que KV.'),

  ('DE_PV_KINDERLOS',
   'PV — Kinderlosenzuschlag (supplément sans enfant)',
   (SELECT id FROM organisme WHERE code='GKV'),
   'PREVOYANCE', 1, 1, 'BRUT_PLAFONNÉ',
   'Supplément exclusivement salarial pour les personnes sans enfant de plus de 23 ans. '
   || 'Taux 2026 : +0,6 % salarié. Historique : +0,25 % (2015-2021) → +0,35 % (2022) → +0,6 % (01/07/2023+). '
   || 'S''ajoute à DE_PFLEGEVERSICHERUNG uniquement si kinderlos = true.'),

  ('DE_UNFALLVERSICHERUNG',
   'UV — Unfallversicherung (assurance accidents — taux moyen)',
   (SELECT id FROM organisme WHERE code='DGUV'),
   'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   'Exclusivement patronale. Taux variable par Berufsgenossenschaft (BG) et classe de risque. '
   || 'Taux moyen national indicatif DGUV : ~1,3 %. '
   || 'Secteurs à risque élevé (BTP, industrie) : jusqu''à 3,5 %. Administration/services : ~0,5-1 %.');

-- ── Taux en vigueur au 01/01/2026 ───────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES

  ((SELECT id FROM cotisation WHERE code='DE_KRANKENVERSICHERUNG'),
   '2026-01-01', NULL, '0.0875', '0.0875',
   'Allg. 7,3 % + Zusatzbeitrag moyen 2026 (2,9 %) / 2 = 1,45 % → 8,75 % chacun. GKV-Spitzenverband.'),

  ((SELECT id FROM cotisation WHERE code='DE_RENTENVERSICHERUNG'),
   '2026-01-01', NULL, '0.0930', '0.0930',
   'Stable depuis 2018. SGB VI.'),

  ((SELECT id FROM cotisation WHERE code='DE_ARBEITSLOSENVERSICHERUNG'),
   '2026-01-01', NULL, '0.0130', '0.0130',
   'Stable depuis 2023. SGB III.'),

  ((SELECT id FROM cotisation WHERE code='DE_PFLEGEVERSICHERUNG'),
   '2026-01-01', NULL, '0.0180', '0.0180',
   'Hausse 2025 : 3,4 % → 3,6 % (1,8 % chacun). SGB XI.'),

  ((SELECT id FROM cotisation WHERE code='DE_PV_KINDERLOS'),
   '2023-07-01', NULL, '0.0060', '0.0000',
   'Réforme PUEG du 01/07/2023 : supplément porté à 0,6 % salarié. SGB XI §55 al. 3.'),

  ((SELECT id FROM cotisation WHERE code='DE_UNFALLVERSICHERUNG'),
   '2015-01-01', NULL, '0.0000', '0.0130',
   'Taux moyen DGUV. Stable et indicatif — varie par BG.');
