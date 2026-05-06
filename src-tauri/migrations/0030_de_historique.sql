-- ============================================================
-- ALLEMAGNE — Historique des taux de cotisations 2015 → 2025
--
-- KV  : taux_sal > taux_pat avant 2019 (Zusatzbeitrag 100 % salarié)
-- RV  : 9,35 % chacun (2015-2017) → 9,30 % (2018+)
-- AV  : 1,50 % (2015-2018) → 1,30 % (2019) → 1,20 % (2020-2022) → 1,30 % (2023+)
-- PV  : hausse progressive ; coupure intra-annuelle au 01/07/2023
-- UV  : taux moyen stable (entrée déjà créée dans 0029 dès 2015-01-01)
-- ============================================================

-- ── DE_KRANKENVERSICHERUNG ───────────────────────────────────
-- Avant 2019 : Zusatzbeitrag 100 % salarié (asymétrie sal > pat)
-- Depuis 2019 : Zusatzbeitrag partagé à 50/50 (réforme GKV-VEG)

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES

  -- 2015 : Zusatzbeitrag moyen 0,9 % → sal 7,3+0,9=8,2 % / pat 7,3 %
  ((SELECT id FROM cotisation WHERE code='DE_KRANKENVERSICHERUNG'),
   '2015-01-01', '2015-12-31', '0.0820', '0.0730',
   'Zusatzbeitrag moy. 0,9 % intégralement salarié. GKV-Spitzenverband 2015.'),

  -- 2016-2017 : Zusatzbeitrag moyen 1,1 % → sal 8,4 % / pat 7,3 %
  ((SELECT id FROM cotisation WHERE code='DE_KRANKENVERSICHERUNG'),
   '2016-01-01', '2017-12-31', '0.0840', '0.0730',
   'Zusatzbeitrag moy. 1,1 % intégralement salarié. GKV-Spitzenverband 2016-2017.'),

  -- 2018 : Zusatzbeitrag moyen 1,0 % → sal 8,3 % / pat 7,3 %
  ((SELECT id FROM cotisation WHERE code='DE_KRANKENVERSICHERUNG'),
   '2018-01-01', '2018-12-31', '0.0830', '0.0730',
   'Zusatzbeitrag moy. 1,0 % intégralement salarié. GKV-Spitzenverband 2018.'),

  -- 2019 : réforme GKV-VEG → Zusatzbeitrag 50/50 ; moy. 0,9 %
  ((SELECT id FROM cotisation WHERE code='DE_KRANKENVERSICHERUNG'),
   '2019-01-01', '2019-12-31', '0.0775', '0.0775',
   'Réforme 01/01/2019 (GKV-VEG) : Zusatzbeitrag partagé. Moy. 0,9 % → 0,45 % chacun. 7,3+0,45=7,75 %.'),

  -- 2020 : Zusatzbeitrag moy. 1,1 % → 7,3+0,55=7,85 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_KRANKENVERSICHERUNG'),
   '2020-01-01', '2020-12-31', '0.0785', '0.0785',
   'Zusatzbeitrag moy. 1,1 % partagé → 0,55 % chacun. GKV-Spitzenverband 2020.'),

  -- 2021-2022 : Zusatzbeitrag moy. 1,3 % → 7,3+0,65=7,95 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_KRANKENVERSICHERUNG'),
   '2021-01-01', '2022-12-31', '0.0795', '0.0795',
   'Zusatzbeitrag moy. 1,3 % partagé → 0,65 % chacun. GKV-Spitzenverband 2021-2022.'),

  -- 2023 : Zusatzbeitrag moy. 1,6 % → 7,3+0,80=8,10 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_KRANKENVERSICHERUNG'),
   '2023-01-01', '2023-12-31', '0.0810', '0.0810',
   'Zusatzbeitrag moy. 1,6 % partagé → 0,80 % chacun. GKV-Spitzenverband 2023.'),

  -- 2024 : Zusatzbeitrag moy. 1,7 % → 7,3+0,85=8,15 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_KRANKENVERSICHERUNG'),
   '2024-01-01', '2024-12-31', '0.0815', '0.0815',
   'Zusatzbeitrag moy. 1,7 % partagé → 0,85 % chacun. GKV-Spitzenverband 2024.'),

  -- 2025 : Zusatzbeitrag moy. 2,5 % → 7,3+1,25=8,55 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_KRANKENVERSICHERUNG'),
   '2025-01-01', '2025-12-31', '0.0855', '0.0855',
   'Zusatzbeitrag moy. 2,5 % partagé → 1,25 % chacun. GKV-Spitzenverband 2025.');

-- ── DE_RENTENVERSICHERUNG ────────────────────────────────────

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES

  -- 2015-2017 : 18,7 % total → 9,35 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_RENTENVERSICHERUNG'),
   '2015-01-01', '2017-12-31', '0.0935', '0.0935',
   'Beitragssatz 18,7 % (9,35 % chacun). SGB VI. Baisse depuis 18,9 % de 2013.'),

  -- 2018-2025 : 18,6 % total → 9,30 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_RENTENVERSICHERUNG'),
   '2018-01-01', '2025-12-31', '0.0930', '0.0930',
   'Beitragssatz 18,6 % (9,3 % chacun). Stable depuis 2018. SGB VI.');

-- ── DE_ARBEITSLOSENVERSICHERUNG ──────────────────────────────

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES

  -- 2015-2018 : 3,0 % → 1,5 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_ARBEITSLOSENVERSICHERUNG'),
   '2015-01-01', '2018-12-31', '0.0150', '0.0150',
   'Beitragssatz 3,0 % (1,5 % chacun). SGB III.'),

  -- 2019 : réduction → 2,6 % → 1,3 % chacun (Qualifizierungschancengesetz)
  ((SELECT id FROM cotisation WHERE code='DE_ARBEITSLOSENVERSICHERUNG'),
   '2019-01-01', '2019-12-31', '0.0130', '0.0130',
   'Baisse à 2,6 % (1,3 % chacun). Qualifizierungschancengesetz 2019. SGB III.'),

  -- 2020-2022 : réduction temporaire → 2,4 % → 1,2 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_ARBEITSLOSENVERSICHERUNG'),
   '2020-01-01', '2022-12-31', '0.0120', '0.0120',
   'Réduction temporaire COVID → 2,4 % (1,2 % chacun). SGB III.'),

  -- 2023-2025 : retour à 2,6 % → 1,3 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_ARBEITSLOSENVERSICHERUNG'),
   '2023-01-01', '2025-12-31', '0.0130', '0.0130',
   'Retour à 2,6 % (1,3 % chacun). SGB III.');

-- ── DE_PFLEGEVERSICHERUNG ────────────────────────────────────
-- Coupure intra-annuelle au 01/07/2023 (réforme PUEG)

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES

  -- 2015-2016 : 2,35 % → 1,175 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_PFLEGEVERSICHERUNG'),
   '2015-01-01', '2016-12-31', '0.01175', '0.01175',
   'Pflegestärkungsgesetz I (2015). 2,35 % total (1,175 % chacun). SGB XI.'),

  -- 2017-2018 : 2,55 % → 1,275 % chacun (Pflegestärkungsgesetz II)
  ((SELECT id FROM cotisation WHERE code='DE_PFLEGEVERSICHERUNG'),
   '2017-01-01', '2018-12-31', '0.01275', '0.01275',
   'Pflegestärkungsgesetz II (2017). 2,55 % total (1,275 % chacun). SGB XI.'),

  -- 2019-2023 (jusqu'au 30/06) : 3,05 % → 1,525 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_PFLEGEVERSICHERUNG'),
   '2019-01-01', '2023-06-30', '0.01525', '0.01525',
   'Pflegeverbesserungsgesetz 2019. 3,05 % total (1,525 % chacun). SGB XI.'),

  -- 2023-07-01 → 2024 : 3,40 % → 1,70 % chacun (réforme PUEG)
  ((SELECT id FROM cotisation WHERE code='DE_PFLEGEVERSICHERUNG'),
   '2023-07-01', '2024-12-31', '0.01700', '0.01700',
   'Réforme PUEG (Pflegeunterstützungs- und -entlastungsgesetz) au 01/07/2023. 3,4 % → 1,7 % chacun.'),

  -- 2025 : nouvelle hausse → 3,60 % → 1,80 % chacun
  ((SELECT id FROM cotisation WHERE code='DE_PFLEGEVERSICHERUNG'),
   '2025-01-01', '2025-12-31', '0.01800', '0.01800',
   'Hausse 2025. 3,6 % total (1,8 % chacun). SGB XI.');

-- ── DE_PV_KINDERLOS ──────────────────────────────────────────
-- Supplément salarié pour personnes sans enfant >23 ans

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES

  -- 2015-2021 : +0,25 % salarié
  ((SELECT id FROM cotisation WHERE code='DE_PV_KINDERLOS'),
   '2015-01-01', '2021-12-31', '0.0025', '0.0000',
   'Kinderlosenzuschlag 0,25 % salarié uniquement. SGB XI §55 al. 3.'),

  -- 2022 → 30/06/2023 : +0,35 % salarié
  ((SELECT id FROM cotisation WHERE code='DE_PV_KINDERLOS'),
   '2022-01-01', '2023-06-30', '0.0035', '0.0000',
   'Hausse Kinderlosenzuschlag → 0,35 %. SGB XI §55 al. 3.');
  -- Note : l'entrée 2023-07-01 → NULL est déjà créée dans 0029 (0,60 %).
