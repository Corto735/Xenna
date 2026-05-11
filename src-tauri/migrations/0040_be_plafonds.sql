-- 0040 — Belgique : plafonds de référence
--
-- BE_SMW   : Revenu Minimum Mensuel Moyen Garanti (RMMMG / GMM) — CCT n°43
-- BE_SEUIL_BONUS_BAS  : seuil annuel inférieur du bonus emploi (ONSS sal)
-- BE_SEUIL_BONUS_HAUT : seuil annuel supérieur (au-delà : pas de bonus emploi)

-- Salaire minimum mensuel (RMMMG)
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('BE_SMW', '2015-01-01', '2017-04-01', '1501.82', 'MENSUEL'),  -- CCT n°43 révision 2015
  ('BE_SMW', '2017-04-01', '2018-06-01', '1562.59', 'MENSUEL'),  -- CCT n°43 révision 04/2017
  ('BE_SMW', '2018-06-01', '2020-04-01', '1593.81', 'MENSUEL'),  -- CCT n°43 révision 06/2018
  ('BE_SMW', '2020-04-01', '2022-04-01', '1625.72', 'MENSUEL'),  -- CCT n°43 révision 04/2020
  ('BE_SMW', '2022-04-01', '2024-04-01', '1806.16', 'MENSUEL'),  -- CCT n°43 relèvement 04/2022 (programme 2022-2024)
  ('BE_SMW', '2024-04-01', '2025-01-01', '1994.00', 'MENSUEL'),  -- CCT n°43 troisième étape 04/2024
  ('BE_SMW', '2025-01-01', NULL,          '2070.48', 'MENSUEL'); -- CCT n°43 indexation 2025

-- Seuil bas bonus emploi (annuel) — sous ce seuil : bonus emploi maximum
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('BE_SEUIL_BONUS_BAS', '2015-01-01', '2019-01-01', '17800',  'ANNUEL'),  -- AR 2015-2018
  ('BE_SEUIL_BONUS_BAS', '2019-01-01', '2024-01-01', '20832',  'ANNUEL'),  -- AR 2019-2023
  ('BE_SEUIL_BONUS_BAS', '2024-01-01', NULL,          '21060',  'ANNUEL'); -- AR 2024+

-- Seuil haut bonus emploi (annuel) — au-delà : pas de bonus emploi
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('BE_SEUIL_BONUS_HAUT', '2015-01-01', '2019-01-01', '27082',  'ANNUEL'),
  ('BE_SEUIL_BONUS_HAUT', '2019-01-01', '2024-01-01', '29736',  'ANNUEL'),
  ('BE_SEUIL_BONUS_HAUT', '2024-01-01', NULL,          '30120',  'ANNUEL');
