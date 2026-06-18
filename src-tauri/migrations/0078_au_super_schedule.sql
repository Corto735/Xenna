-- 0078 — Australie : échéancier Superannuation Guarantee 2014-2025 (backfill)
-- Le taux SG augmente chaque 1er juillet : 9,5 % → 10 % → 10,5 % → 11 % → 11,5 % → 12 %.
-- La ligne 2025-07-01 → NULL (12 %) existe déjà (0064). On ajoute l'historique antérieur.
-- Source : ATO — Super guarantee percentage (SGAA 1992).

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'AU_SUPER'), '2014-07-01', '2021-07-01', '0', '0.095',
   (SELECT id FROM texte_loi WHERE code = 'AU_SGAA_1992'), 'SG 9,5 % (1er juil. 2014 → 30 juin 2021).'),
  ((SELECT id FROM cotisation WHERE code = 'AU_SUPER'), '2021-07-01', '2022-07-01', '0', '0.100',
   (SELECT id FROM texte_loi WHERE code = 'AU_SGAA_1992'), 'SG 10,0 % (exercice 2021-22).'),
  ((SELECT id FROM cotisation WHERE code = 'AU_SUPER'), '2022-07-01', '2023-07-01', '0', '0.105',
   (SELECT id FROM texte_loi WHERE code = 'AU_SGAA_1992'), 'SG 10,5 % (exercice 2022-23).'),
  ((SELECT id FROM cotisation WHERE code = 'AU_SUPER'), '2023-07-01', '2024-07-01', '0', '0.110',
   (SELECT id FROM texte_loi WHERE code = 'AU_SGAA_1992'), 'SG 11,0 % (exercice 2023-24).'),
  ((SELECT id FROM cotisation WHERE code = 'AU_SUPER'), '2024-07-01', '2025-07-01', '0', '0.115',
   (SELECT id FROM texte_loi WHERE code = 'AU_SGAA_1992'), 'SG 11,5 % (exercice 2024-25).');
