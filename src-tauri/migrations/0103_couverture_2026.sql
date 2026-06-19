-- ============================================================
-- COUVERTURE 2026 — taux de cotisation modifiés au 01/01/2026
-- Pour chaque cotisation dont le TAUX change en 2026, on clôture la période
-- courante au 31/12/2025 puis on ouvre une nouvelle période au 01/01/2026.
-- (Les pays dont seuls les barèmes d'impôt / plafonds changent sont traités
-- côté Rust « match annee » ; aucune ligne ici.)
-- Sources : URSSAF/AADE (GR), Revenue/Budget 2026 (IE), NHIS/NPS (KR).
-- ============================================================

-- ── Grèce : baisse EFKA au 01/01/2026 (13,87→13,37 % sal ; 22,29→21,79 % pat) ──
UPDATE cotisation_taux SET date_fin = '2025-12-31'
  WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'GR_EFKA') AND date_fin IS NULL;
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'GR_EFKA'), '2026-01-01', NULL, '0.1337', '0.2179',
   'EFKA 2026 : 13,37 % sal / 21,79 % pat (baisse des cotisations).');

-- ── Irlande : PRSI Class A salarié 4,1→4,2 % au 01/01/2026 (employeur 11,15 % inchangé) ──
-- (La hausse supplémentaire de +0,15 % au 01/10/2026 n'est pas modélisée — taux annuel 4,2 %.)
UPDATE cotisation_taux SET date_fin = '2025-12-31'
  WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'IE_PRSI') AND date_fin IS NULL;
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'IE_PRSI'), '2026-01-01', NULL, '0.042', '0.1115',
   'PRSI Class A 2026 : 4,2 % sal / 11,15 % pat (Budget 2026).');

-- ── Corée du Sud : pension 9→9,5 % (4,75 % chacun) et santé 7,09→7,19 % (3,595 % chacun) ──
UPDATE cotisation_taux SET date_fin = '2025-12-31'
  WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'KR_NPS') AND date_fin IS NULL;
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'KR_NPS'), '2026-01-01', NULL, '0.0475', '0.0475',
   '국민연금 2026 : 4,75 % chacun (réforme des retraites, +0,5 pt/an jusqu''en 2033).');

UPDATE cotisation_taux SET date_fin = '2025-12-31'
  WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'KR_NHI') AND date_fin IS NULL;
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'KR_NHI'), '2026-01-01', NULL, '0.03595', '0.03595',
   '건강보험 2026 : 7,19 % au total (3,595 % chacun).');
