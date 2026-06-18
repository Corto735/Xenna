-- 0063 — Australie : organismes et textes de loi
-- Périmètre : résident fiscal, secteur privé. Exercice fiscal 2025-26 (1 juil. 2025 → 30 juin 2026).
-- Données : 2026 (pilote machine). Devise AUD.
--
-- Modèle australien : le salarié supporte l'impôt sur le revenu (PAYG withholding) + Medicare levy 2 %.
-- Pas de cotisation sociale salariale. L'employeur verse la Superannuation Guarantee (12 % en 2025-26),
-- en sus du salaire (n'entre pas dans le net, mais dans le coût employeur).

INSERT INTO organisme (code, libelle, url) VALUES
  ('AU_ATO',   'Australian Taxation Office — impôt sur le revenu (PAYG) + Medicare levy', 'https://www.ato.gov.au'),
  ('AU_SUPER', 'Superannuation (fonds de retraite, Super Guarantee)',                     'https://www.ato.gov.au/individuals-and-families/super-for-individuals-and-families/super');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('AU_ITAA_1997', 'LOI', 'Income Tax Assessment Act 1997', 'No. 38 of 1997', '1997-04-17', '1997-07-01',
   'https://www.legislation.gov.au/C2004A05138',
   'Barème de l''impôt sur le revenu des résidents. 2025-26 (Stage 3) : 0 % jusqu''à 18 200 $, 16 % de 18 201 à 45 000 $, 30 % de 45 001 à 135 000 $, 37 % de 135 001 à 190 000 $, 45 % au-delà.'),

  ('AU_MEDICARE_ACT', 'LOI', 'Medicare Levy Act 1986', 'No. 110 of 1986', '1986-12-04', '1986-12-04',
   'https://www.legislation.gov.au/C2004A03340',
   'Medicare levy : 2 % du revenu imposable (financement du système de santé). Réductions/exemptions pour bas revenus non modélisées.'),

  ('AU_SGAA_1992', 'LOI', 'Superannuation Guarantee (Administration) Act 1992', 'No. 111 of 1992', '1992-06-30', '1992-07-01',
   'https://www.legislation.gov.au/C2004A04455',
   'Superannuation Guarantee : cotisation retraite 100 % patronale. Taux 2025-26 : 12,0 % des ordinary time earnings. Maximum contribution base 2025-26 : 62 500 $/trimestre (≈ 250 000 $/an).');
