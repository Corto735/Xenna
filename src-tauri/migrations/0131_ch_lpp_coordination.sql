-- ============================================================
-- SUISSE — LPP : déduction de coordination et salaire coordonné maximal
-- (miroir des valeurs de ch_cotisations.rs, qui fait foi pour le calcul)
--
-- Les valeurs en base étaient décalées : 24 570 (valeur 2013-2014) au lieu de
-- 24 675 pour 2015-2018, 27 225 au lieu de 26 460 pour 2025-2026, et une
-- série de salaires coordonnés maximaux sans rapport avec la règle.
-- Règle (OPP 2) : rente AVS maximale annuelle R ; déduction = 7/8 R ;
-- maximum coordonné = 3 R − 7/8 R.
--   2015-2018 : R = 28 200 → 24 675 / 59 925
--   2019-2020 : R = 28 440 → 24 885 / 60 435
--   2021-2022 : R = 28 680 → 25 095 / 60 945
--   2023-2024 : R = 29 400 → 25 725 / 62 475
--   2025-2026 : R = 30 240 → 26 460 / 64 260
-- Source : OFAS, montants limites de la prévoyance professionnelle.
-- ============================================================
UPDATE plafond_reference SET valeur = '24675.00' WHERE code = 'CH_LPP_COORD_DED' AND date_debut = '2015-01-01';
UPDATE plafond_reference SET valeur = '26460.00' WHERE code = 'CH_LPP_COORD_DED' AND date_debut = '2025-01-01';
UPDATE plafond_reference SET valeur = '59925.00' WHERE code = 'CH_LPP_COORD_MAX' AND date_debut = '2015-01-01';
UPDATE plafond_reference SET valeur = '60435.00' WHERE code = 'CH_LPP_COORD_MAX' AND date_debut = '2019-01-01';
UPDATE plafond_reference SET valeur = '60945.00' WHERE code = 'CH_LPP_COORD_MAX' AND date_debut = '2021-01-01';
UPDATE plafond_reference SET valeur = '62475.00' WHERE code = 'CH_LPP_COORD_MAX' AND date_debut = '2023-01-01';
