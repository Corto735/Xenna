-- ============================================================
-- LUXEMBOURG — Cotisations sociales au 01/01/2026
-- Organisme collecteur : CCSS (Centre commun de la sécurité sociale)
-- Référence : Code de la Sécurité Sociale (CSS LU)
--
-- Taux AP/AM/AD : fédéraux, confirmés inchangés pour 2026.
-- Taux AA (accidents) : indicatif AAA secteur tertiaire.
-- Taux ME (mutualité) : indicatif taux moyen national CCSS.
--
-- SSM non-qualifié au 01/01/2026 : env. €2 703,76/mois
-- (estimation après indexation janvier 2025 ; vérifier publication CCSS)
-- Plafond cotisable : 5 × SSM = €13 518,80/mois
-- ============================================================

-- ── Organismes collecteurs ───────────────────────────────────
INSERT INTO organisme (code, libelle, url) VALUES
  ('CNAP',  'CNAP — Caisse nationale d''assurance pension',         'https://www.cnap.lu'),
  ('CNS_LU','CNS — Caisse nationale de santé',                      'https://www.cns.lu'),
  ('AAA_LU','AAA — Association d''assurance accident',              'https://www.aaa.lu'),
  ('CCSS',  'CCSS — Centre commun de la sécurité sociale',          'https://www.ccss.lu');

-- ── Textes de loi luxembourgeois ─────────────────────────────
INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES
  ('CSS_AP_LU', 'LOI',
   'Code de la Sécurité Sociale — Assurance pension',
   'Livre II CSS LU', '1984-12-27', '1988-01-01',
   'Régime obligatoire d''assurance pension géré par la CNAP. Taux : 16 % total (8 % chacun). Plafonné à 5 × SSM.'),
  ('CSS_AM_LU', 'LOI',
   'Code de la Sécurité Sociale — Assurance maladie-maternité',
   'Livre II CSS LU (art. 10 et s.)', '1984-12-27', '1975-01-01',
   'Assurance maladie-maternité gérée par la CNS. Soins de santé (2,80 %) + indemnités pécuniaires (0,25 %) = 3,05 % chacun. Plafonné à 5 × SSM.'),
  ('CSS_AD_LU', 'LOI',
   'Loi du 19 juin 1998 portant introduction d''une assurance dépendance',
   '1998-06-19', '1999-01-01', '1999-01-01',
   'Assurance dépendance (Pflegeversicherung) : 1,40 % salarié uniquement. Financée uniquement par la retenue salariale.'),
  ('CSS_AA_LU', 'LOI',
   'Code de la Sécurité Sociale — Assurance accidents',
   'Livre III CSS LU', '1984-12-27', '1976-01-01',
   'Assurance accidents du travail et maladies professionnelles gérée par l''AAA. Taux patronal variable selon classe de risque.'),
  ('CSS_ME_LU', 'LOI',
   'Code de la Sécurité Sociale — Mutualité des employeurs',
   'Livre II CSS LU (art. 3 et s.)', '1984-12-27', '1999-01-01',
   'Mutualité des employeurs : rembourse aux employeurs les salaires maintenus pendant les congés maladie (jours 1-77). Taux patronal variable selon sinistralité et secteur.');

-- ── Définition des cotisations ───────────────────────────────
INSERT INTO cotisation (code, libelle, organisme_id, categorie,
  applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES

  ('LU_AP', 'AP — Assurance pension (CNAP)',
    (SELECT id FROM organisme WHERE code='CNAP'),
    'RETRAITE_COMPLEMENTAIRE', 1, 1, 'BRUT_PLAFONNÉ',
    'Taux : 8 % chacun. Plafonné à 5 × SSM (≈ €13 518,80/mois en 2026).'),

  ('LU_AM', 'AM — Assurance maladie-maternité (CNS)',
    (SELECT id FROM organisme WHERE code='CNS_LU'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
    'Soins de santé 2,80 % + indemnités pécuniaires 0,25 % = 3,05 % chacun. Plafonné à 5 × SSM.'),

  ('LU_AD', 'AD — Assurance dépendance',
    (SELECT id FROM organisme WHERE code='CNS_LU'),
    'PREVOYANCE', 1, 1, 'BRUT_PLAFONNÉ',
    'Salarié uniquement : 1,40 %. Plafonné à 5 × SSM.'),

  ('LU_AA', 'AA — Assurance accidents (AAA)',
    (SELECT id FROM organisme WHERE code='AAA_LU'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
    'Employeur uniquement. Taux indicatif secteur tertiaire : 0,75 %. Varie selon classe de risque AAA.'),

  ('LU_ME', 'ME — Mutualité des employeurs (CCSS)',
    (SELECT id FROM organisme WHERE code='CCSS'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
    'Employeur uniquement. Taux indicatif moyen national : 1,40 %. Varie selon sinistralité de l''entreprise.');

-- ── Taux en vigueur au 01/01/2026 ───────────────────────────
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES

  ((SELECT id FROM cotisation WHERE code='LU_AP'),
   '2026-01-01', NULL, '0.0800', '0.0800',
   'Taux stable. CSS LU Livre II.'),

  ((SELECT id FROM cotisation WHERE code='LU_AM'),
   '2026-01-01', NULL, '0.0305', '0.0305',
   'Soins de santé 2,80 % + indemnités pécuniaires 0,25 % = 3,05 %. CSS LU art. 10.'),

  ((SELECT id FROM cotisation WHERE code='LU_AD'),
   '2026-01-01', NULL, '0.0140', '0.0000',
   'Loi du 19/06/1998. Salarié uniquement.'),

  ((SELECT id FROM cotisation WHERE code='LU_AA'),
   '2026-01-01', NULL, '0.0000', '0.0075',
   'Taux indicatif AAA secteur tertiaire. Varie par classification des risques.'),

  ((SELECT id FROM cotisation WHERE code='LU_ME'),
   '2026-01-01', NULL, '0.0000', '0.0140',
   'Taux indicatif moyen national CCSS. Varie selon historique maladie et secteur.');
