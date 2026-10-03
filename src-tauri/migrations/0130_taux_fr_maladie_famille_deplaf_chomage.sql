-- ============================================================
-- FRANCE PRIVÉ — correction des taux maladie, vieillesse déplafonnée,
-- allocations familiales et chômage (2015-2026)
--
-- Constat (comparaison avec un bulletin réel de septembre 2026) :
--   - SS_MALADIE patronal à 13,13 % depuis 2018 : taux inexistant. Le taux
--     plein est 13 % depuis le 01/01/2018 (12,80 % en 2015, 12,84 % en 2016,
--     12,89 % en 2017).
--   - SS_VIEILLESSE_DEPLAF figé à 0,40 / 1,90 % depuis 2018 : le patronal est
--     passé à 2,02 % au 01/01/2025 et à 2,11 % au 01/01/2026 (LFSS 2025 et
--     2026). Avant 2017 : 0,30 / 1,80 % (2015), 0,35 / 1,85 % (2016).
--   - FAMILLE à 3,45 % pour tous les salaires depuis 2015 : 3,45 % n'est que
--     le taux RÉDUIT, sous un seuil de rémunération. Taux plein 5,25 %.
--   - CHOMAGE 4,05 % : ramené à 4,00 % au 01/05/2025 (fin de la contribution
--     exceptionnelle de 0,05 % créée au 01/10/2017 ; circulaire Unédic du
--     01/05/2025).
--
-- Taux réduits (employeurs éligibles à la réduction générale, CSS art.
-- L241-2-1 et L241-6-1) — portés par plafond_reference, appliqués en code :
--   Maladie 7 % (au lieu de 13 %) si rémunération ≤ seuil × SMIC :
--     2,5 SMIC du 01/01/2019 au 31/12/2024 (2024 : SMIC gelé au 31/12/2023,
--     1 747,20 €) ; 2,25 SMIC en 2025 (SMIC au 01/01/2025, décret 2025-318).
--   Famille 3,45 % (au lieu de 5,25 %) si rémunération ≤ seuil × SMIC :
--     1,6 SMIC du 01/01/2015 au 31/03/2016 ; 3,5 SMIC du 01/04/2016 au
--     31/12/2024 ; 3,3 SMIC en 2025.
--   Les deux taux réduits sont supprimés au 01/01/2026 (LFSS 2026) : taux
--   uniques 13 % et 5,25 %, l'allègement passe par la RGDU.
--
-- Sources : Urssaf (taux secteur privé, « Ce qu'il faut savoir au 1er janvier
-- 2026 ») ; LégiSocial, taux Urssaf 2015 à 2026 ; décret 2025-318 du
-- 04/04/2025 ; circulaire Unédic du 01/05/2025.
-- date_fin exclusive (convention des requêtes : date_fin > date de paie).
-- ============================================================

-- ── Maladie ─────────────────────────────────────────────────
DELETE FROM cotisation_taux
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'SS_MALADIE');
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
    ((SELECT id FROM cotisation WHERE code = 'SS_MALADIE'), '2015-01-01', '2016-01-01', '0.0075', '0.1280', 'Maladie 0,75 % / 12,80 % en 2015'),
    ((SELECT id FROM cotisation WHERE code = 'SS_MALADIE'), '2016-01-01', '2017-01-01', '0.0075', '0.1284', 'Maladie 0,75 % / 12,84 % en 2016'),
    ((SELECT id FROM cotisation WHERE code = 'SS_MALADIE'), '2017-01-01', '2018-01-01', '0.0075', '0.1289', 'Maladie 0,75 % / 12,89 % en 2017'),
    ((SELECT id FROM cotisation WHERE code = 'SS_MALADIE'), '2018-01-01', NULL,         '0.0000', '0.1300', 'Taux plein 13 % depuis 2018 (part salariale supprimée, LFSS 2018). Taux réduit 7 % de 2019 à 2025 : voir TX_REDUIT_MALADIE_*');

-- ── Vieillesse déplafonnée ──────────────────────────────────
DELETE FROM cotisation_taux
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'SS_VIEILLESSE_DEPLAF');
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
    ((SELECT id FROM cotisation WHERE code = 'SS_VIEILLESSE_DEPLAF'), '2015-01-01', '2016-01-01', '0.0030', '0.0180', 'Déplafonnée 0,30 % / 1,80 % en 2015'),
    ((SELECT id FROM cotisation WHERE code = 'SS_VIEILLESSE_DEPLAF'), '2016-01-01', '2017-01-01', '0.0035', '0.0185', 'Déplafonnée 0,35 % / 1,85 % en 2016'),
    ((SELECT id FROM cotisation WHERE code = 'SS_VIEILLESSE_DEPLAF'), '2017-01-01', '2025-01-01', '0.0040', '0.0190', 'Déplafonnée 0,40 % / 1,90 % de 2017 à 2024'),
    ((SELECT id FROM cotisation WHERE code = 'SS_VIEILLESSE_DEPLAF'), '2025-01-01', '2026-01-01', '0.0040', '0.0202', 'Patronal 2,02 % au 01/01/2025 (LFSS 2025)'),
    ((SELECT id FROM cotisation WHERE code = 'SS_VIEILLESSE_DEPLAF'), '2026-01-01', NULL,         '0.0040', '0.0211', 'Patronal 2,11 % au 01/01/2026 (LFSS 2026)');

-- ── Allocations familiales ──────────────────────────────────
DELETE FROM cotisation_taux
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'FAMILLE');
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
    ((SELECT id FROM cotisation WHERE code = 'FAMILLE'), '2015-01-01', NULL, '0.0000', '0.0525', 'Taux plein 5,25 %. Taux réduit 3,45 % de 2015 à 2025 : voir TX_REDUIT_FAMILLE_*');

-- ── Assurance chômage ───────────────────────────────────────
UPDATE cotisation_taux SET date_fin = '2025-05-01'
 WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'CHOMAGE')
   AND date_debut = '2018-10-01';
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
    ((SELECT id FROM cotisation WHERE code = 'CHOMAGE'), '2025-05-01', NULL, '0.0000', '0.0400', 'Fin de la contribution exceptionnelle de 0,05 % : 4,05 % → 4,00 % au 01/05/2025 (circulaire Unédic du 01/05/2025)');

-- ── Taux réduits maladie / famille (seuils en nombre de SMIC) ──
-- « MENSUEL » ne sert qu'à satisfaire le CHECK du schéma.
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('TX_REDUIT_MALADIE_TAUX',  '2019-01-01', '2026-01-01', '0.07',    'MENSUEL'),
    ('TX_REDUIT_MALADIE_SEUIL', '2019-01-01', '2025-01-01', '2.5',     'MENSUEL'),
    ('TX_REDUIT_MALADIE_SEUIL', '2025-01-01', '2026-01-01', '2.25',    'MENSUEL'),
    ('TX_REDUIT_FAMILLE_TAUX',  '2015-01-01', '2026-01-01', '0.0345',  'MENSUEL'),
    ('TX_REDUIT_FAMILLE_SEUIL', '2015-01-01', '2016-04-01', '1.6',     'MENSUEL'),
    ('TX_REDUIT_FAMILLE_SEUIL', '2016-04-01', '2025-01-01', '3.5',     'MENSUEL'),
    ('TX_REDUIT_FAMILLE_SEUIL', '2025-01-01', '2026-01-01', '3.3',     'MENSUEL'),
    -- 2024 : seuils appréciés sur le SMIC en vigueur au 31/12/2023 (gel).
    -- Autres années : SMIC en vigueur à la date de paie (2025 : celui du
    -- 01/01/2025, inchangé toute l'année).
    ('TX_REDUIT_SMIC_REF',      '2024-01-01', '2025-01-01', '1747.20', 'MENSUEL');
