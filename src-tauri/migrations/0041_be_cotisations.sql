-- 0041 — Belgique : cotisations et taux 2015-2026
-- BE_BONUS_EMPLOI / BE_RED_STRUCT / BE_PP : calcul Rust entièrement (pas de cotisation_taux DB)

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('BE_ONSS_SAL', 'ONSS — cotisation salariale personnelle de sécurité sociale',
   (SELECT id FROM organisme WHERE code = 'ONSS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '13,07 % du salaire brut, sans plafond, stable depuis 2003. Couvre maladie, invalidité, retraite, chômage, accidents. Loi 27/06/1969 ; AR annuels ONSS.'),

  ('BE_ONSS_PAT', 'ONSS — cotisation patronale globale (secteur privé)',
   (SELECT id FROM organisme WHERE code = 'ONSS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   'Taux de base ~25,92 % (hors réduction structurelle). Inclut : pension, maladie, chômage, allocations familiales, accidents. Loi 27/06/1969 ; AR annuels ONSS. Réduction structurelle calculée séparément (BE_RED_STRUCT).'),

  ('BE_BONUS_EMPLOI', 'Bonus emploi — réduction cotisations personnelles (bas salaires)',
   (SELECT id FROM organisme WHERE code = 'ONSS'), 'SECURITE_SOCIALE', 1, 1, 'SPECIFIQUE',
   'Réduction des cotisations ONSS personnelles (13,07 %) pour les travailleurs à bas salaire. Montant mensuel dégressif. Loi 20/12/1999 ; AR annuels. Calcul en Rust be_cotisations.rs — aucun taux DB.'),

  ('BE_RED_STRUCT', 'Réduction structurelle — réduction cotisations patronales',
   (SELECT id FROM organisme WHERE code = 'ONSS'), 'SECURITE_SOCIALE', 1, 1, 'SPECIFIQUE',
   'Réduction forfaitaire mensuelle des cotisations patronales ONSS. Montant fixe selon la rémunération de référence. AR 16/05/2003, mod. annuelles. Calcul en Rust be_cotisations.rs — aucun taux DB.'),

  ('BE_PP', 'Précompte professionnel (PP/BV) — retenue à la source IPP',
   (SELECT id FROM organisme WHERE code = 'SPF_FINANCES_BE'), 'CSG_CRDS', 1, 1, 'SPECIFIQUE',
   'Retenue mensuelle à la source de l''IPP (Impôt des Personnes Physiques). Barème fédéral progressif 4 tranches (25/40/45/50 %) + exonération de base. Ajustement régional : Flandres (-6 %), Wallonie (+9 %), Bruxelles (0 %). Calcul en Rust be_pp.rs — aucun taux DB.');

-- ONSS salarial : stable depuis 2003
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'BE_ONSS_SAL'), '2015-01-01', NULL, '0.1307', '0.0000',
   '13,07 % sal. Stable depuis 2003. Source : Loi 27/06/1969 + AR annuels ONSS.');

-- ONSS patronal : taux de base (avant réduction structurelle)
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'BE_ONSS_PAT'), '2015-01-01', NULL, '0.0000', '0.2592',
   '25,92 % pat (taux global de base secteur privé). Hors réduction structurelle (BE_RED_STRUCT). Source : AR ONSS annuels.');
