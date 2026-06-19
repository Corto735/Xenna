-- ============================================================
-- HEURES SUPPLÉMENTAIRES & COMPLÉMENTAIRES — exonérations
-- Trois dispositifs distincts, tous postérieurs à la loi du 24/12/2018 :
--   1. Réduction de cotisations salariales (CSS art. L241-17) — plafond 11,31 %
--      stocké côté Rust (constante, pas de périodicité « taux » en base).
--   2. Déduction forfaitaire patronale par heure supp (CSS art. L241-18).
--   3. Exonération d'impôt sur le revenu (CGI art. 81 quater), plafond annuel.
-- Montants/plafonds stockés en TEXT pour précision exacte.
-- Sources : urssaf.fr (réduction salariale, DFP), boss.gouv.fr (exonérations HS),
-- service-public.fr (majorations Code du travail).
-- ============================================================

-- Références (réceptacle texte_loi pour traçabilité ; non encore surfacé en UI).
INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('HS_MAJORATION', 'LOI', 'Majoration des heures supplémentaires et complémentaires', 'CT L3121-36 / L3123-8',
   '2008-08-20', '2008-08-20', 'https://www.service-public.fr/particuliers/vosdroits/F2280',
   'Heures supplémentaires (au-delà de 35 h) : majoration légale de 25 % pour les 8 premières, 50 % au-delà (un accord peut prévoir un taux différent, plancher 10 %). Heures complémentaires (temps partiel) : 10 % dans la limite du dixième des heures contractuelles, 25 % au-delà, dans la limite du tiers.'),

  ('HS_REDUC_SAL', 'LOI', 'Réduction de cotisations salariales sur les heures supplémentaires', 'CSS L241-17',
   '2018-12-24', '2019-01-01', 'https://www.urssaf.fr/accueil/employeur/beneficier-exonerations/exonerations-heures/reduction-cotisations-salariales.html',
   'Loi du 24/12/2018 (mesures d''urgence économiques et sociales) : réduction des cotisations salariales d''assurance vieillesse (de base et complémentaire) sur la rémunération des heures supplémentaires et complémentaires, dans la limite de 11,31 %.'),

  ('HS_DFP', 'LOI', 'Déduction forfaitaire patronale sur les heures supplémentaires', 'CSS L241-18',
   '2007-08-21', '2019-01-01', 'https://www.urssaf.fr/accueil/employeur/beneficier-exonerations/exonerations-heures/deduction-forfaitaire-patronale.html',
   'Déduction forfaitaire de cotisations patronales par heure supplémentaire : 1,50 € (entreprises de moins de 20 salariés), 0,50 € (20 salariés et plus, depuis le 01/10/2022), étendue aux entreprises de 250 salariés et plus à compter du 01/01/2026.'),

  ('HS_EXO_FISCALE', 'LOI', 'Exonération d''impôt sur le revenu des heures supplémentaires', 'CGI 81 quater',
   '2018-12-24', '2019-01-01', 'https://www.economie.gouv.fr/particuliers/heures-supplementaires-exonerees-impot',
   'Exonération d''impôt sur le revenu de la rémunération des heures supplémentaires et complémentaires, dans la limite de 5 000 € nets imposables par an (2019-2021), relevée à 7 500 € à compter de 2022.');

-- Déduction forfaitaire patronale par heure supplémentaire (€/heure).
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite, texte_loi_id) VALUES
  ('DFP_HS_MOINS20', '2019-01-01', NULL, '1.50', 'HORAIRE', (SELECT id FROM texte_loi WHERE code='HS_DFP')),  -- < 20 salariés
  ('DFP_HS_20_249',  '2022-10-01', NULL, '0.50', 'HORAIRE', (SELECT id FROM texte_loi WHERE code='HS_DFP')),  -- 20 à 249 salariés
  ('DFP_HS_250P',    '2026-01-01', NULL, '0.50', 'HORAIRE', (SELECT id FROM texte_loi WHERE code='HS_DFP'));  -- 250 salariés et + (depuis 2026)

-- Exonération d'impôt sur le revenu — plafond annuel de net imposable exonéré.
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite, texte_loi_id) VALUES
  ('EXO_FISCALE_HS_ANNUEL', '2019-01-01', '2021-12-31', '5000.00', 'ANNUEL', (SELECT id FROM texte_loi WHERE code='HS_EXO_FISCALE')),
  ('EXO_FISCALE_HS_ANNUEL', '2022-01-01', NULL,         '7500.00', 'ANNUEL', (SELECT id FROM texte_loi WHERE code='HS_EXO_FISCALE'));
