-- ============================================================
-- SUISSE — Cotisations sociales fédérales au 01/01/2026
-- Données applicables dès le 01/01/2026 (pas d'historique antérieur).
-- Taux AVS/AI/APG/AC : fédéraux, confirmés inchangés.
-- Taux AANP/AAP : indicatifs SUVA bureau (varient par assureur/classe risque).
-- Taux IJM : indicatif plan collectif standard (varie par assureur).
-- Taux LPP  : minimum légal tranche d'âge 35-44 ans (art. 16 LPP).
-- Plafonds LPP : valeurs 2025 (OFAS, entrée en vigueur 01/01/2025, valables 2026).
-- ============================================================

-- ── Organismes collecteurs ───────────────────────────────────
INSERT INTO organisme (code, libelle, url) VALUES
  ('AVS_CH',    'Caisse de compensation AVS / OFAS',          'https://www.bsv.admin.ch'),
  ('SUVA',      'SUVA — Assurance-accidents (LAA)',           'https://www.suva.ch'),
  ('SECO',      'SECO — Assurance-chômage (LACI)',            'https://www.seco.admin.ch'),
  ('CAISSE_LPP','Institution de prévoyance LPP (2ème pilier)','https://www.bsv.admin.ch');

-- ── Textes de loi suisses ────────────────────────────────────
INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES
  ('LAVS', 'LOI',
   'Loi fédérale sur l''assurance-vieillesse et survivants',
   'RS 831.10', '1946-12-20', '1948-01-01',
   '1er pilier. Taux 2020+ : 8,70 % total (4,35 % chacun). Pas de plafond.'),
  ('LAI', 'LOI',
   'Loi fédérale sur l''assurance-invalidité',
   'RS 831.20', '1959-06-19', '1960-01-01',
   '1er pilier AI. Taux : 1,40 % total (0,70 % chacun).'),
  ('LAPG', 'LOI',
   'Loi fédérale sur les allocations pour perte de gain',
   'RS 834.1', '1952-09-25', '1953-01-01',
   '1er pilier APG. Taux : 0,50 % total (0,25 % chacun).'),
  ('LACI', 'LOI',
   'Loi fédérale sur l''assurance-chômage obligatoire et l''indemnité en cas d''insolvabilité',
   'RS 837.0', '1982-06-25', '1984-01-01',
   'AC obligatoire. Taux : 2,20 % total (1,10 % chacun) plafonné à CHF 148 200/an.'),
  ('LAA', 'LOI',
   'Loi fédérale sur l''assurance-accidents',
   'RS 832.20', '1981-03-20', '1984-01-01',
   'AAP (employeur seul) et AANP (employé seul). Plafond CHF 148 200/an. Taux fixés par assureur.'),
  ('LPP', 'LOI',
   'Loi fédérale sur la prévoyance professionnelle vieillesse, survivants et invalidité',
   'RS 831.40', '1982-06-25', '1985-01-01',
   '2ème pilier obligatoire. Taux selon âge (7/10/15/18 %). Salaire coordonné = brut − déduction coordination.');

-- ── Définition des cotisations ───────────────────────────────
INSERT INTO cotisation (code, libelle, organisme_id, categorie,
  applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES

  ('CH_AVS', 'AVS — Assurance-vieillesse et survivants (1er pilier)',
    (SELECT id FROM organisme WHERE code='AVS_CH'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
    'Taux stable depuis 2020 : 4,35 % chacun. Pas de plafond.'),

  ('CH_AI', 'AI — Assurance invalidité (1er pilier)',
    (SELECT id FROM organisme WHERE code='AVS_CH'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
    'Taux stable : 0,70 % chacun. Pas de plafond.'),

  ('CH_APG', 'APG — Allocations pour perte de gain (1er pilier)',
    (SELECT id FROM organisme WHERE code='AVS_CH'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
    'Taux stable : 0,25 % chacun. Pas de plafond.'),

  ('CH_AC', 'AC — Assurance-chômage',
    (SELECT id FROM organisme WHERE code='SECO'),
    'CHOMAGE', 1, 1, 'BRUT_PLAFONNÉ',
    'Taux : 1,10 % chacun. Plafond CHF 148 200/an (CHF 12 350/mois). Inchangé depuis 2014.'),

  ('CH_AANP', 'AANP — Assurance accidents non professionnels (LAA)',
    (SELECT id FROM organisme WHERE code='SUVA'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
    'Salarié uniquement. Taux indicatif (1 %) : varie selon assureur et classe de risque SUVA.'),

  ('CH_AAP', 'AAP — Assurance accidents professionnels (LAA)',
    (SELECT id FROM organisme WHERE code='SUVA'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
    'Employeur uniquement. Taux indicatif (1 %) : varie selon classe de risque SUVA.'),

  ('CH_IJM', 'IJM — Indemnités journalières maladie (plan collectif)',
    (SELECT id FROM organisme WHERE code='AVS_CH'),
    'PREVOYANCE', 1, 1, 'BRUT_TOTAL',
    'Plan collectif facultatif (LCA/LAMAL). Taux indicatif (1,50 % total) : varie par assureur.'),

  ('CH_LPP', 'LPP — Prévoyance professionnelle (2ème pilier, taux 35-44 ans)',
    (SELECT id FROM organisme WHERE code='CAISSE_LPP'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'SPECIFIQUE',
    'Taux légal minimum art. 16 LPP pour tranche 35-44 ans : 10 % total (5 % chacun). Base = salaire coordonné.');

-- ── Taux en vigueur au 01/01/2026 ───────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES

  ((SELECT id FROM cotisation WHERE code='CH_AVS'),
   '2026-01-01', NULL, '0.0435', '0.0435',
   'Stable depuis 2020. Réforme AVS 21 n''a pas modifié ce taux.'),

  ((SELECT id FROM cotisation WHERE code='CH_AI'),
   '2026-01-01', NULL, '0.0070', '0.0070',
   'Stable depuis 2015.'),

  ((SELECT id FROM cotisation WHERE code='CH_APG'),
   '2026-01-01', NULL, '0.0025', '0.0025',
   'Stable depuis 2011.'),

  ((SELECT id FROM cotisation WHERE code='CH_AC'),
   '2026-01-01', NULL, '0.0110', '0.0110',
   'Inchangé depuis la révision LACI 2014. Plafond CHF 148 200/an.'),

  ((SELECT id FROM cotisation WHERE code='CH_AANP'),
   '2026-01-01', NULL, '0.0100', '0.0000',
   'Taux indicatif SUVA employés de bureau. Salarié uniquement.'),

  ((SELECT id FROM cotisation WHERE code='CH_AAP'),
   '2026-01-01', NULL, '0.0000', '0.0100',
   'Taux indicatif SUVA. Employeur uniquement.'),

  ((SELECT id FROM cotisation WHERE code='CH_IJM'),
   '2026-01-01', NULL, '0.0075', '0.0075',
   'Plan collectif type. Varie selon contrat assureur.'),

  ((SELECT id FROM cotisation WHERE code='CH_LPP'),
   '2026-01-01', NULL, '0.0500', '0.0500',
   'Minimum légal 35-44 ans. Base = salaire coordonné (plafonds OPP 2 revalorisés 01/01/2025).');
