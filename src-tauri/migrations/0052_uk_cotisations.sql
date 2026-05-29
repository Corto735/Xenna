-- 0052 — Royaume-Uni : cotisations National Insurance 2024/25
--
-- UK_NI_SAL : NI Class 1 — part salariale (8 % tranche PT→UEL, 2 % au-delà)
-- UK_NI_PAT : NI Class 1 — part patronale (13,8 % au-dessus du ST)
-- Note : les taux de l'Income Tax PAYE (20/40/45 %) sont hardcodés dans uk_cotisations.rs
--        car ce sont des tranches progressives, pas un taux unique sur assiette simple.
-- Source : Finance Act 2024 ; HMRC NI rates 2024/25.

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('UK_NI_SAL', 'National Insurance Class 1 — part salariale',
   (SELECT id FROM organisme WHERE code = 'HMRC'), 'SECURITE_SOCIALE', 1, 1, 'SPECIFIQUE',
   '8 % sur la tranche [PT – UEL], 2 % au-dessus de l''UEL. PT = £12 570/an ; UEL = £50 270/an (2024/25). Taux réduit de 12 % → 10 % (janv. 2024) → 8 % (avr. 2024). NIA 2014 + Finance Act 2024.'),

  ('UK_NI_PAT', 'National Insurance Class 1 — part patronale',
   (SELECT id FROM organisme WHERE code = 'HMRC'), 'SECURITE_SOCIALE', 1, 1, 'SPECIFIQUE',
   '13,8 % sur le salaire brut excédant le Secondary Threshold (ST = £9 100/an). Pas de plafond côté employeur. NIA 2014 + Finance Act 2024.'),

  ('UK_INCOME_TAX', 'Income Tax PAYE — retenue mensuelle à la source',
   (SELECT id FROM organisme WHERE code = 'HMRC'), 'CSG_CRDS', 1, 1, 'SPECIFIQUE',
   'Impôt progressif sur le revenu retenu à la source (PAYE). Personal Allowance £12 570. 20 % (Basic), 40 % (Higher), 45 % (Additional). Calcul Rust uk_cotisations.rs — aucun taux en DB. ITA 2007 + Finance Act 2024.');

-- NI salarié : taux 8 % (taux de base sur tranche principale)
-- Note : le calcul Rust gère la double tranche (8 % / 2 %) et les seuils.
--        Le taux ici est à titre documentaire ; le code lit ctx.taux_sal("UK_NI_SAL").
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'UK_NI_SAL'), '2024-04-06', NULL, '0.0800', '0.0000',
   'Taux tranche principale [PT–UEL] pour 2024/25. Taux secondaire 2 % au-delà UEL géré directement en Rust. Source : Finance Act 2024.');

-- NI patronal : taux 13,8 %
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'UK_NI_PAT'), '2024-04-06', NULL, '0.0000', '0.1380',
   '13,8 % du salaire excédant le ST (£9 100/an). Source : Finance Act 2024.');
