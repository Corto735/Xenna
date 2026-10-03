-- ============================================================
-- SLOVÉNIE — deux retenues absentes
--   Prispevek za dolgotrajno oskrbo (assurance dépendance) : 1 % salarié et
--   1 % employeur depuis le 01/07/2025 (Zakon o dolgotrajni oskrbi).
--   Obvezni zdravstveni prispevek (OZP), forfait mensuel retenu sur le net :
--   35 € (2024), 37,17 € dès le 01/03/2025, 39,36 € dès le 01/03/2026
--   (revalorisé chaque 1er mars selon le salaire moyen ; ZZVZZ).
-- Sources : RTV SLO, n1info (2025-2026) ; ZZZS.
-- ============================================================
INSERT INTO cotisation (code, libelle, categorie, type_assiette, plafond_coeff_min, plafond_coeff_max, notes) VALUES
    ('SI_DOLGOTRAJNA', 'Prispevek za dolgotrajno oskrbo', 'SECURITE_SOCIALE', 'BRUT_TOTAL', '0', NULL, 'Depuis le 01/07/2025');
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
    ((SELECT id FROM cotisation WHERE code = 'SI_DOLGOTRAJNA'), '2025-07-01', NULL, '0.01', '0.01', '1 % salarié + 1 % employeur');
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('SI_OZP', '2024-01-01', '2025-03-01', '35.00', 'MENSUEL'),
    ('SI_OZP', '2025-03-01', '2026-03-01', '37.17', 'MENSUEL'),
    ('SI_OZP', '2026-03-01', NULL,         '39.36', 'MENSUEL');
