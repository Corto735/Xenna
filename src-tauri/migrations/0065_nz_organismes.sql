-- 0065 — Nouvelle-Zélande : organismes et textes de loi
-- Périmètre : résident fiscal, secteur privé. Année fiscale 2025-26 (1 avr. 2025 → 31 mars 2026).
-- Données : 2026 (pilote machine). Devise NZD.
--
-- Modèle néo-zélandais : pas de sécurité sociale par cotisations. Le salarié supporte :
--   • l'impôt sur le revenu (PAYE, barème progressif, pas de tranche exonérée) ;
--   • l'ACC earner's levy (couverture accidents, plafonnée).
-- KiwiSaver (retraite par capitalisation) : optionnel ; cotisation employeur par défaut 3 %.

INSERT INTO organisme (code, libelle, url) VALUES
  ('NZ_IRD', 'Inland Revenue Department (Te Tari Taake) — impôt sur le revenu (PAYE)', 'https://www.ird.govt.nz'),
  ('NZ_ACC', 'Accident Compensation Corporation — ACC earner''s levy',                'https://www.acc.co.nz');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('NZ_ITA_2007', 'LOI', 'Income Tax Act 2007', 'No. 97 of 2007', '2007-11-01', '2008-04-01',
   'https://www.legislation.govt.nz/act/public/2007/0097/latest/DLM1512301.html',
   'Barème PAYE des résidents (pas de tranche exonérée). Année 2025-26 : 10,5 % jusqu''à 15 600 $, 17,5 % de 15 601 à 53 500 $, 30 % de 53 501 à 78 100 $, 33 % de 78 101 à 180 000 $, 39 % au-delà.'),

  ('NZ_ACC_ACT_2001', 'LOI', 'Accident Compensation Act 2001', 'No. 49 of 2001', '2001-09-13', '2002-04-01',
   'https://www.legislation.govt.nz/act/public/2001/0049/latest/DLM99494.html',
   'ACC earner''s levy : 1,67 % du salaire brut (année 2025-26), plafonné à 152 790 $/an (levy max 2 551,59 $). Couvre les accidents non professionnels.'),

  ('NZ_KIWISAVER_2006', 'LOI', 'KiwiSaver Act 2006', 'No. 40 of 2006', '2006-09-19', '2007-07-01',
   'https://www.legislation.govt.nz/act/public/2006/0040/latest/DLM378372.html',
   'KiwiSaver : épargne-retraite optionnelle. Cotisation employeur par défaut 3 % du salaire brut (en sus). Cotisation salariale par défaut 3 % (non prélevée ici si non adhérent).');
