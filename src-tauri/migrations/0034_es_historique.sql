-- 0034 — Espagne : textes de référence annuels SMI 2015-2025
-- Les taux ES_CC/DESEMPLEO/FOGASA/FP sont stables depuis 2015 (une seule entrée NULL dans 0033).
-- Ce fichier enregistre les décrets SMI annuels comme textes de loi.

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES
  ('ES_RD_SMI_2015', 'DECRET', 'SMI 2015 — 21,62 €/j, 648,60 €/mois', 'RD 1106/2014', '2014-12-27', '2015-01-01',
   'SMI 2015 : +0,5 % par rapport à 2014. Source : BOE 2014-12-27.'),

  ('ES_RD_SMI_2016', 'DECRET', 'SMI 2016 — 21,84 €/j, 655,20 €/mois', 'RD 1171/2015', '2015-12-29', '2016-01-01',
   'SMI 2016 : +1 %. Source : BOE 2015-12-29.'),

  ('ES_RD_SMI_2017', 'DECRET', 'SMI 2017 — 23,59 €/j, 707,70 €/mois', 'RD 742/2016', '2016-12-30', '2017-01-01',
   'SMI 2017 : +8 %. Première grande revalorisation de la législature.'),

  ('ES_RD_SMI_2018', 'DECRET', 'SMI 2018 — 24,53 €/j, 735,90 €/mois', 'RD 1077/2017', '2017-12-29', '2018-01-01',
   'SMI 2018 : +4 %.'),

  ('ES_RD_SMI_2019', 'DECRET', 'SMI 2019 — 30,00 €/j, 900,00 €/mois (+22,3 %)', 'RD 1462/2018', '2018-12-21', '2019-01-01',
   'SMI 2019 : +22,3 %. Hausse historique négociée par le gouvernement Sánchez.'),

  ('ES_RD_SMI_2020', 'DECRET', 'SMI 2020 — 31,66 €/j, 950,00 €/mois (+5,5 %)', 'RD 231/2020', '2020-02-04', '2020-01-01',
   'SMI 2020 : +5,5 %. Rétroactif au 01/01/2020.'),

  ('ES_RD_SMI_2021', 'DECRET', 'SMI 2021 — 32,17 €/j, 965,00 €/mois (01/09/2021)', 'RD 817/2021', '2021-09-21', '2021-09-01',
   'SMI 2021 : 950 €/mois jan-août, puis 965 €/mois à partir du 01/09/2021.'),

  ('ES_RD_SMI_2022', 'DECRET', 'SMI 2022 — 33,33 €/j, 1 000,00 €/mois (+3,6 %)', 'RD 152/2022', '2022-03-01', '2022-02-01',
   'SMI 2022 : 1 000 €/mois, premier franchissement du seuil symbolique. Rétroactif au 01/02/2022.'),

  ('ES_RD_SMI_2023', 'DECRET', 'SMI 2023 — 36,00 €/j, 1 080,00 €/mois (+8 %)', 'RD 99/2023', '2023-02-08', '2023-01-01',
   'SMI 2023 : +8 %, rétroactif au 01/01/2023.'),

  ('ES_RD_SMI_2024', 'DECRET', 'SMI 2024 — 37,80 €/j, 1 134,00 €/mois (+5 %)', 'RD 145/2024', '2024-02-06', '2024-01-01',
   'SMI 2024 : +5 %, rétroactif au 01/01/2024.'),

  ('ES_RD_SMI_2025', 'DECRET', 'SMI 2025 — 39,47 €/j, 1 184,00 €/mois (+4,4 %)', 'RD 8/2025', '2025-01-15', '2025-01-01',
   'SMI 2025 : +4,4 %. Source : RD 8/2025 publié le 15/01/2025.');
