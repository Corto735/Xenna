-- ============================================================
-- GARANTIE DES SALAIRES — AGS : historique corrigé 2015-2026
-- ============================================================
-- Les migrations 0004 et 0007 portaient un historique erroné (0,25 % en
-- 2020-2022, 0,20 % en 2023, 0,15 % depuis 2024). Taux fixés par le conseil
-- d'administration de l'AGS, relevés dans ses communiqués et dans la presse
-- sociale (LégiSocial, Revue fiduciaire, Service-Public Entreprendre) :
--   2015            0,30 %  (taux gelé depuis avril 2011)
--   2016            0,25 %  (CA du 06/01/2016)
--   01/2017-06/2017 0,20 %
--   07/2017-12/2023 0,15 %
--   01/2024-06/2024 0,20 %
--   depuis 07/2024  0,25 %  (maintenu au 01/01/2025, 01/01/2026 — CA du
--                            16/12/2025 — et au 01/07/2026)
-- Entièrement patronal, assiette plafonnée à 4 PMSS (inchangé).
DELETE FROM cotisation_taux WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'AGS');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'AGS'), '2015-01-01', '2015-12-31', '0.0000', '0.0030',
   'AGS 0,30 % — taux gelé d''avril 2011 à décembre 2015'),
  ((SELECT id FROM cotisation WHERE code = 'AGS'), '2016-01-01', '2016-12-31', '0.0000', '0.0025',
   'AGS 0,25 % au 01/01/2016 — CA de l''AGS du 06/01/2016'),
  ((SELECT id FROM cotisation WHERE code = 'AGS'), '2017-01-01', '2017-06-30', '0.0000', '0.0020',
   'AGS 0,20 % au 01/01/2017'),
  ((SELECT id FROM cotisation WHERE code = 'AGS'), '2017-07-01', '2023-12-31', '0.0000', '0.0015',
   'AGS 0,15 % du 01/07/2017 au 31/12/2023'),
  ((SELECT id FROM cotisation WHERE code = 'AGS'), '2024-01-01', '2024-06-30', '0.0000', '0.0020',
   'AGS 0,20 % au 01/01/2024'),
  ((SELECT id FROM cotisation WHERE code = 'AGS'), '2024-07-01', NULL, '0.0000', '0.0025',
   'AGS 0,25 % au 01/07/2024 — maintenu au 01/01/2026 (CA du 16/12/2025) et au 01/07/2026');
