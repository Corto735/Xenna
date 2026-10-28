-- ============================================================
-- CITATIONS CORRIGÉES DANS LES FICHES CONVENTIONNELLES (Chakrram)
--
-- 1. Frais professionnels : l'arrêté du 20 décembre 2002 est abrogé depuis le
--    7 septembre 2025, remplacé par l'arrêté du 4 septembre 2025 (régimes
--    général et agricole). Les barèmes cités valent sous l'un puis l'autre.
--
-- 2. Mise à la retraite par l'employeur (art. L. 137-12 CSS) : la contribution
--    patronale est de 40 % depuis le 1er janvier 2026 (LFSS 2026), 30 % du
--    1er septembre 2023 au 31 décembre 2025 (LFRSS 2023), 50 % auparavant. La
--    fiche annonçait encore 50 %.
-- ============================================================

UPDATE ccn_reglementations
SET corps  = replace(corps,  'arrêté du 20 décembre 2002',
                     'arrêté du 4 septembre 2025 (arrêté du 20 décembre 2002 jusqu''au 6 septembre 2025)'),
    source = replace(source, 'arrêté du 20 décembre 2002',
                     'arrêté du 4 septembre 2025 (auparavant arrêté du 20 décembre 2002)')
WHERE corps LIKE '%arrêté du 20 décembre 2002%' OR source LIKE '%arrêté du 20 décembre 2002%';

UPDATE ccn_indemnites
SET corps  = replace(corps,  'arrêté du 20 décembre 2002',
                     'arrêté du 4 septembre 2025 (arrêté du 20 décembre 2002 jusqu''au 6 septembre 2025)'),
    source = replace(source, 'arrêté du 20 décembre 2002',
                     'arrêté du 4 septembre 2025 (auparavant arrêté du 20 décembre 2002)')
WHERE corps LIKE '%arrêté du 20 décembre 2002%' OR source LIKE '%arrêté du 20 décembre 2002%';

UPDATE ccn_reglementations
SET corps  = replace(corps,
                     'l''employeur acquitte une contribution spécifique de 50 % sur le montant total (art. L. 137-12 CSS).',
                     'l''employeur acquitte une contribution spécifique de 40 % sur le montant total depuis le 1er janvier 2026 (30 % du 1er septembre 2023 au 31 décembre 2025, 50 % auparavant ; art. L. 137-12 CSS).'),
    valeur = 'Contribution 40 %'
WHERE idcc = '0016' AND titre = 'Départ et mise à la retraite' AND valeur = 'Contribution 50 %';
