-- 0097 — Croatie : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise EUR (depuis 2023). Données : 2025.
--
-- Retraite 20 % salarié (1er pilier 15 % + 2ᵉ pilier 5 %) ; santé 16,5 % employeur.
-- Porez na dohodak (20 % / 30 %) et abattement personnel calculés en Rust (hr_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('HR_HZMO', 'Hrvatski zavod za mirovinsko osiguranje — retraites', 'https://www.mirovinsko.hr'),
  ('HR_HZZO', 'Hrvatski zavod za zdravstveno osiguranje — santé',    'https://www.hzzo.hr'),
  ('HR_PU',   'Porezna uprava — administration fiscale',             'https://www.porezna-uprava.hr');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('HR_MIR', 'LOI', 'Zakon o doprinosima — cotisations sociales', '84/2008', '2008-07-15', '2009-01-01',
   'https://www.zakon.hr',
   'Cotisations 2025 : retraite 20 % salarié (1er pilier 15 % + 2ᵉ pilier 5 %) ; santé 16,5 % employeur.'),
  ('HR_POR', 'LOI', 'Zakon o porezu na dohodak — impôt sur le revenu', '115/2016', '2016-12-09', '2017-01-01',
   'https://www.zakon.hr',
   'Porez na dohodak 2025 (réforme 2024 supprimant la prirez communale) : taux fixés par commune, représentatifs 20 % jusqu''à 5 000 €/mois, 30 % au-delà. Abattement personnel (osobni odbitak) 600 €/mois.'),
  ('HR_HISTOIRE', 'LOI', 'Croatie — histoire fiscale et sociale', '—', '2023-01-01', '2023-01-01',
   'https://www.porezna-uprava.hr',
   'Système hérité du modèle yougoslave puis aligné sur l''UE (adhésion 2013, euro et Schengen au 01/01/2023). Retraite à deux piliers (répartition + capitalisation) depuis 2002. Réforme fiscale 2024 : suppression de la surtaxe communale (prirez), intégrée dans des taux d''imposition fixés localement. Politiquement : convergence européenne et modernisation post-transition.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('HR_MIROVINSKO', 'Mirovinsko osiguranje — Retraite',
   (SELECT id FROM organisme WHERE code = 'HR_HZMO'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 20 % salarié (1er pilier 15 % + 2ᵉ pilier 5 %).'),
  ('HR_ZDRAVSTVENO', 'Zdravstveno osiguranje — Santé (employeur)',
   (SELECT id FROM organisme WHERE code = 'HR_HZZO'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 16,5 % employeur.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'HR_MIROVINSKO'),  '2025-01-01', NULL, '0.20', '0',
   (SELECT id FROM texte_loi WHERE code = 'HR_MIR'), 'Retraite 2025 : 20 % salarié.'),
  ((SELECT id FROM cotisation WHERE code = 'HR_ZDRAVSTVENO'), '2025-01-01', NULL, '0', '0.165',
   (SELECT id FROM texte_loi WHERE code = 'HR_MIR'), 'Santé 2025 : 16,5 % employeur.');
