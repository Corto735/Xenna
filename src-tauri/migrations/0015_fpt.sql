-- ============================================================
-- MIGRATION 0015 — FONCTION PUBLIQUE TERRITORIALE (FPT)
-- Cotisations pour les agents titulaires de la FPT depuis le 01/01/2016
--
-- Différences clés vs secteur privé :
--   • CNRACL remplace SS vieillesse + AGIRC-ARRCO
--   • Pas d'assurance chômage pour les titulaires
--   • Pas de réduction Fillon pour les employeurs publics
--   • Maladie, famille, AT/MP, CSG/CRDS : mêmes règles qu'en France
-- ============================================================

-- ── Organisme collecteur ─────────────────────────────────────
INSERT INTO organisme (code, libelle, url) VALUES
  ('CNRACL', 'Caisse Nationale de Retraite des Agents des Collectivités Locales',
   'https://www.cnracl.fr');

-- ── Texte de loi de référence ────────────────────────────────
INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES
  ('DECRET_2011_291',
   'DECRET',
   'Décret relatif à la convergence des taux de cotisation CNRACL vers le régime général',
   'n°2011-291', '2011-03-15', '2012-01-01',
   'Plan de "montée en charge" du taux salarial CNRACL de 8,39 % (2012) vers 11,10 % (2019), '
   || 'dans le cadre de la convergence public-privé initiée par la réforme des retraites 2010. '
   || 'Le taux employeur (collectivités) est maintenu à 30,65 %.');

-- ── Cotisation CNRACL ────────────────────────────────────────
-- Régime de retraite principal des fonctionnaires FPT titulaires.
-- Remplace à la fois SS_VIEILLESSE et AGIRC-ARRCO pour ces agents.
INSERT INTO cotisation (code, libelle, organisme_id, categorie,
                        applicable_cadre, applicable_non_cadre,
                        type_assiette, plafond_coeff_min, plafond_coeff_max)
VALUES (
  'FPT_CNRACL',
  'CNRACL — Retraite principale (FPT titulaires)',
  (SELECT id FROM organisme WHERE code = 'CNRACL'),
  'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL', '0', NULL
);

-- Taux historiques : montée en charge 2016-2018, taux cible 2019+
-- Sources : décrets annuels CNRACL, rapport CNRACL 2020
INSERT INTO cotisation_taux
  (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes)
VALUES
  ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'),
   '2016-01-01', '2016-12-31', '0.1029', '0.3065',
   (SELECT id FROM texte_loi WHERE code = 'DECRET_2011_291'),
   'Montée en charge. Agent : 10,29 %. Collectivité : 30,65 %.'),

  ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'),
   '2017-01-01', '2017-12-31', '0.1056', '0.3065',
   (SELECT id FROM texte_loi WHERE code = 'DECRET_2011_291'),
   'Agent : 10,56 %. Collectivité : 30,65 %.'),

  ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'),
   '2018-01-01', '2018-12-31', '0.1083', '0.3065',
   (SELECT id FROM texte_loi WHERE code = 'DECRET_2011_291'),
   'Agent : 10,83 %. Collectivité : 30,65 %.'),

  ((SELECT id FROM cotisation WHERE code = 'FPT_CNRACL'),
   '2019-01-01', NULL, '0.1110', '0.3065',
   (SELECT id FROM texte_loi WHERE code = 'DECRET_2011_291'),
   'Taux cible atteint. Agent : 11,10 %. Collectivité : 30,65 %.');
