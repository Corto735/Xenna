-- 0067 — Pologne : organismes, textes de loi et cotisations ZUS 2025
-- Périmètre : salarié secteur privé (umowa o pracę). Devise PLN. Données : 2025.
--
-- Côté salarié : ZUS social (emerytalne 9,76 % + rentowe 1,5 % + chorobowe 2,45 %)
--   + składka zdrowotna 9 % (assiette = brut − ZUS social) + PIT (12 % / 32 %).
-- Emerytalne + rentowe plafonnées au « 30-krotność » (260 190 PLN/an en 2025).
-- Santé et PIT calculés en Rust (pl_bulletin.rs) ; taux ZUS lus en base.

INSERT INTO organisme (code, libelle, url) VALUES
  ('PL_ZUS', 'Zakład Ubezpieczeń Społecznych (ZUS) — assurances sociales',          'https://www.zus.pl'),
  ('PL_NFZ', 'Narodowy Fundusz Zdrowia (NFZ) — assurance maladie (składka zdrowotna)', 'https://www.nfz.gov.pl'),
  ('PL_MF',  'Ministerstwo Finansów — impôt sur le revenu (PIT)',                     'https://www.gov.pl/web/finanse');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('PL_USTAWA_ZUS', 'LOI', 'Ustawa o systemie ubezpieczeń społecznych — Loi sur le système d''assurances sociales', 'Dz.U. 1998 nr 137 poz. 887', '1998-10-13', '1999-01-01',
   'https://isap.sejm.gov.pl/isap.nsf/DocDetails.xsp?id=WDU19981370887',
   'Cotisations ZUS. Salarié 2025 : emerytalne 9,76 %, rentowe 1,5 %, chorobowe 2,45 %. Emerytalne + rentowe plafonnées au 30-krotność (260 190 PLN/an en 2025).'),

  ('PL_USTAWA_ZDROW', 'LOI', 'Ustawa o świadczeniach opieki zdrowotnej — Loi sur les prestations de santé', 'Dz.U. 2004 nr 210 poz. 2135', '2004-08-27', '2004-10-01',
   'https://isap.sejm.gov.pl/isap.nsf/DocDetails.xsp?id=WDU20042102135',
   'Składka zdrowotna : 9 % de l''assiette (brut − cotisations ZUS sociales salariales). Non déductible du PIT depuis 2022 (Polski Ład).'),

  ('PL_USTAWA_PIT', 'LOI', 'Ustawa o podatku dochodowym od osób fizycznych (PIT)', 'Dz.U. 1991 nr 80 poz. 350', '1991-07-26', '1992-01-01',
   'https://isap.sejm.gov.pl/isap.nsf/DocDetails.xsp?id=WDU19910800350',
   'Barème PIT 2025 : 12 % jusqu''à 120 000 PLN de revenu imposable, 32 % au-delà. Montant réducteur d''impôt 3 600 PLN/an (kwota wolna 30 000 PLN). KUP standard 3 000 PLN/an.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('PL_EMERYTALNE', 'Emerytalne — Assurance vieillesse',
   (SELECT id FROM organisme WHERE code = 'PL_ZUS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025 : 9,76 % sal + 9,76 % pat. Plafonnée au 30-krotność (260 190 PLN/an).'),
  ('PL_RENTOWE', 'Rentowe — Assurance invalidité/décès',
   (SELECT id FROM organisme WHERE code = 'PL_ZUS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025 : 1,5 % sal + 6,5 % pat. Plafonnée au 30-krotność (260 190 PLN/an).'),
  ('PL_CHOROBOWE', 'Chorobowe — Assurance maladie (indemnités)',
   (SELECT id FROM organisme WHERE code = 'PL_ZUS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 2,45 % sal. Non plafonnée.'),
  ('PL_WYPADKOWE', 'Wypadkowe — Accidents du travail',
   (SELECT id FROM organisme WHERE code = 'PL_ZUS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 1,67 % pat (taux standard, variable par employeur). Non plafonnée.'),
  ('PL_FP', 'Fundusz Pracy — Fonds du travail',
   (SELECT id FROM organisme WHERE code = 'PL_ZUS'), 'CHOMAGE', 1, 1, 'BRUT_TOTAL',
   '2025 : 2,45 % pat. Non plafonné.'),
  ('PL_FGSP', 'FGŚP — Fonds garantie des prestations',
   (SELECT id FROM organisme WHERE code = 'PL_ZUS'), 'CHOMAGE', 1, 1, 'BRUT_TOTAL',
   '2025 : 0,10 % pat. Non plafonné.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'PL_EMERYTALNE'), '2025-01-01', NULL, '0.0976', '0.0976',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'Emerytalne 2025 : 9,76 % chacun.'),
  ((SELECT id FROM cotisation WHERE code = 'PL_RENTOWE'),    '2025-01-01', NULL, '0.0150', '0.0650',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'Rentowe 2025 : 1,5 % sal / 6,5 % pat.'),
  ((SELECT id FROM cotisation WHERE code = 'PL_CHOROBOWE'),  '2025-01-01', NULL, '0.0245', '0',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'Chorobowe 2025 : 2,45 % sal.'),
  ((SELECT id FROM cotisation WHERE code = 'PL_WYPADKOWE'),  '2025-01-01', NULL, '0', '0.0167',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'Wypadkowe 2025 : 1,67 % pat (standard).'),
  ((SELECT id FROM cotisation WHERE code = 'PL_FP'),         '2025-01-01', NULL, '0', '0.0245',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'Fundusz Pracy 2025 : 2,45 % pat.'),
  ((SELECT id FROM cotisation WHERE code = 'PL_FGSP'),       '2025-01-01', NULL, '0', '0.0010',
   (SELECT id FROM texte_loi WHERE code = 'PL_USTAWA_ZUS'), 'FGŚP 2025 : 0,10 % pat.');
