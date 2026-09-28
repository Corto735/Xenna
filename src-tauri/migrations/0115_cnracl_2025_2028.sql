-- ============================================================
-- CNRACL : relèvement du taux employeur 2025-2028
-- ============================================================
-- Décret n° 2025-86 du 30 janvier 2025 (JORFTEXT000051070354) : taux de la
-- contribution employeur porté de 30,65 % à 34,65 % (2025), 37,65 % (2026),
-- 40,65 % (2027) puis 43,65 % (2028). Part agent inchangée : 11,10 %.
UPDATE cotisation_taux SET date_fin = '2024-12-31'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'FPT_CNRACL')
  AND date_debut = '2019-01-01';

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('DECRET_2025_86', 'DECRET',
   'Décret relatif au taux de cotisations vieillesse des employeurs des agents affiliés à la CNRACL',
   'Décret n° 2025-86 du 30/01/2025', '2025-01-31', '2025-01-01',
   'https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000051070354',
   'Taux employeur CNRACL : 34,65 % (2025), 37,65 % (2026), 40,65 % (2027), 43,65 % (2028).');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2025-01-01', '2025-12-31', '0.1110', '0.3465',
   (SELECT id FROM texte_loi WHERE code = 'DECRET_2025_86'), 'Agent : 11,10 %. Collectivité : 34,65 %.'),
  ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2026-01-01', '2026-12-31', '0.1110', '0.3765',
   (SELECT id FROM texte_loi WHERE code = 'DECRET_2025_86'), 'Agent : 11,10 %. Collectivité : 37,65 %.'),
  ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2027-01-01', '2027-12-31', '0.1110', '0.4065',
   (SELECT id FROM texte_loi WHERE code = 'DECRET_2025_86'), 'Agent : 11,10 %. Collectivité : 40,65 %.'),
  ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'), '2028-01-01', NULL, '0.1110', '0.4365',
   (SELECT id FROM texte_loi WHERE code = 'DECRET_2025_86'), 'Agent : 11,10 %. Collectivité : 43,65 % (dernier palier du décret).');
