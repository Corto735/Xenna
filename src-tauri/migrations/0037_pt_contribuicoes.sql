-- 0037 — Portugal : cotisations et taux 2015-2026
-- Périmètre : régime geral, secteur privé, CDI.
-- PT_IRS : barème calculé en Rust (pt_irs.rs), pas de cotisation_taux DB (même pattern que IT_IRPEF).

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('PT_SS', 'Segurança Social — Taxa Social Única (TSU)',
   (SELECT id FROM organisme WHERE code = 'IGFSS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   'TSU : 11,00 % sal + 23,75 % pat = 34,75 % total. Sans plafond (régime général). Couvre maladie, maternité, invalidité, retraite, survivants, chômage. Stable depuis 2013. Lei 110/2009 art. 53-54.'),

  ('PT_AT_SEG', 'Seguro de Acidentes de Trabalho e Doenças Profissionais',
   (SELECT id FROM organisme WHERE code = 'ACT_PT'), 'AUTRES', 1, 1, 'BRUT_TOTAL',
   'Assurance AT/MP obligatoire, exclusivement patronale. Taux moyen 1,75 % (tertiaire, risque moyen). Varie de 0,5 % (bureaux) à 10 %+ (BTP). Lei 98/2009 art. 79.'),

  ('PT_FCT', 'FCT — Fundo de Compensação do Trabalho',
   (SELECT id FROM organisme WHERE code = 'FCT_PT'), 'AUTRES', 1, 1, 'BRUT_TOTAL',
   'Garantit 50 % des indemnités licenciement (CDI post-01/10/2013). Exclusivement patronal : 0,925 %. DL 210/2015 art. 4.'),

  ('PT_FGCT', 'FGCT — Fundo de Garantia de Compensação do Trabalho',
   (SELECT id FROM organisme WHERE code = 'FCT_PT'), 'AUTRES', 1, 1, 'BRUT_TOTAL',
   'Garantit les 50 % restants des indemnités. Exclusivement patronal : 0,075 %. DL 210/2015 art. 5.'),

  ('PT_IRS', 'IRS — Retenção na Fonte (retenue à la source)',
   (SELECT id FROM organisme WHERE code = 'AT_PT'), 'CSG_CRDS', 1, 1, 'SPECIFIQUE',
   'Avance mensuelle IRS retenue par l''employeur (CIRS art. 99). Barème progressif art. 68, annualisé puis /12. Dedução específica art. 25 déduite. Calcul en Rust pt_irs.rs — aucun taux DB chargé (même pattern que IT_IRPEF).');

-- PT_SS : stable depuis 2015
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'PT_SS'), '2015-01-01', NULL, '0.1100', '0.2375',
   '11,00 % sal + 23,75 % pat. Stable depuis 2013. Source : Lei 110/2009 art. 53.');

-- PT_AT_SEG : stable
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'PT_AT_SEG'), '2015-01-01', NULL, '0.0000', '0.0175',
   'Taux moyen 1,75 % (tertiaire). Varie selon le contrat d''assurance. Lei 98/2009 art. 79.');

-- PT_FCT : stable depuis DL 210/2015
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'PT_FCT'), '2015-01-01', NULL, '0.0000', '0.00925',
   '0,925 % pat. Stable depuis DL 210/2015. Applicable CDI post-01/10/2013.');

-- PT_FGCT : stable
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'PT_FGCT'), '2015-01-01', NULL, '0.0000', '0.00075',
   '0,075 % pat. Stable depuis DL 210/2015.');
