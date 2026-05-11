-- 0038 — Portugal : textes de référence SMN et barème IRS annuel
-- PT_SS/AT_SEG/FCT/FGCT : stables depuis 2015 (une seule entrée NULL dans 0037).
-- Le barème IRS est en Rust (pt_irs.rs), ce fichier documente les sources légales SMN.
-- Barème IRS annuel documenté en commentaires ci-dessous (calcul Rust, pas de taux DB).

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES
  ('PT_SMN_2015', 'DECRET', 'SMN 2015 — 505,00 €/mois', 'DL 144/2015', '2015-08-08', '2015-01-01',
   'SMN 2015 : 505 €/mois. Revalorisation de +3,3 % par rapport à 2014.'),

  ('PT_SMN_2016', 'DECRET', 'SMN 2016 — 530,00 €/mois (+5 %)', 'DL 254-A/2015', '2015-12-31', '2016-01-01',
   'SMN 2016 : +5 %. Première grande hausse dans le programme de convergence vers 600 €.'),

  ('PT_SMN_2017', 'DECRET', 'SMN 2017 — 557,00 €/mois (+5,1 %)', 'DL 86-B/2016', '2016-12-29', '2017-01-01',
   'SMN 2017 : +5,1 %.'),

  ('PT_SMN_2018', 'DECRET', 'SMN 2018 — 580,00 €/mois (+4,1 %)', 'DL 156/2017', '2017-12-29', '2018-01-01',
   'SMN 2018 : +4,1 %.'),

  ('PT_SMN_2019', 'DECRET', 'SMN 2019 — 600,00 €/mois (+3,4 %)', 'DL 619/2018', '2018-09-11', '2019-01-01',
   'SMN 2019 : objectif 600 € du gouvernement Costa I atteint.'),

  ('PT_SMN_2020', 'DECRET', 'SMN 2020 — 635,00 €/mois (+5,8 %)', 'DL 107/2019', '2019-08-09', '2020-01-01',
   'SMN 2020 : +5,8 %. Début du programme vers 1 020 € à horizon 2023.'),

  ('PT_SMN_2021', 'DECRET', 'SMN 2021 — 665,00 €/mois (+4,7 %)', 'DL 109-G/2021', '2021-08-26', '2021-01-01',
   'SMN 2021 : +4,7 % malgré la crise COVID.'),

  ('PT_SMN_2022', 'DECRET', 'SMN 2022 — 705,00 €/mois (+6 %)', 'DL 109-A/2021', '2021-12-31', '2022-01-01',
   'SMN 2022 : +6 %. Accélération post-COVID.'),

  ('PT_SMN_2023', 'DECRET', 'SMN 2023 — 760,00 €/mois (+7,8 %)', 'DL 119/2022', '2022-12-30', '2023-01-01',
   'SMN 2023 : +7,8 %. Hausse en réponse à l''inflation.'),

  ('PT_SMN_2024', 'DECRET', 'SMN 2024 — 820,00 €/mois (+7,8 %)', 'DL 107/2023', '2023-11-28', '2024-01-01',
   'SMN 2024 : +7,8 %.'),

  ('PT_SMN_2025', 'DECRET', 'SMN 2025 — 870,00 €/mois (+6,1 %)', 'DL 125/2024', '2024-12-30', '2025-01-01',
   'SMN 2025 : +6,1 %. Engagement du gouvernement Montenegro.');

-- Barème IRS annuel (CIRS art. 68) — implémenté dans pt_irs.rs :
--
-- OE 2015 (Lei 82-B/2014) — 5 tranches :
--   ≤7 000 : 14,50% | 7 000-20 000 : 28,50% | 20 000-40 000 : 37%
--   40 000-80 000 : 45% | >80 000 : 48%
--
-- OE 2016 (Lei 7-A/2016) — 5 tranches (seuils légèrement ajustés)
--
-- OE 2017 (Lei 42/2016) — 5 tranches :
--   ≤7 091 : 14,50% | 7 091-20 261 : 28,50% | 20 261-40 522 : 37%
--   40 522-80 640 : 45% | >80 640 : 48%
--
-- OE 2018-2019 (Lei 114/2017 + Lei 71/2018) — 7 tranches :
--   ≤7 091 : 14,50% | 7 091-10 700 : 23% | 10 700-20 261 : 28,50%
--   20 261-25 000 : 35% | 25 000-36 856 : 37% | 36 856-80 640 : 45% | >80 640 : 48%
--
-- OE 2020-2022 (Lei 2/2020, 75-B/2020, 12/2022) — 7 tranches :
--   ≤7 112 : 14,50% | 7 112-10 732 : 23% | 10 732-20 322 : 28,50%
--   20 322-25 075 : 35% | 25 075-36 967 : 37% | 36 967-80 882 : 45% | >80 882 : 48%
--
-- OE 2023 (Lei 24-D/2022) — 9 tranches, taux T1 réduit à 13,25% :
--   ≤7 479 : 13,25% | 7 479-11 284 : 18% | 11 284-15 992 : 23%
--   15 992-20 700 : 26% | 20 700-26 355 : 32,75% | 26 355-38 632 : 37%
--   38 632-50 483 : 43,5% | 50 483-78 834 : 45% | >78 834 : 48%
--
-- OE 2024 (Lei 24/2023) — 8 tranches :
--   ≤7 703 : 13,25% | 7 703-11 623 : 18% | 11 623-16 472 : 23%
--   16 472-22 000 : 26% | 22 000-28 000 : 32,75% | 28 000-40 000 : 37%
--   40 000-80 000 : 43,5% | >80 000 : 48%
--
-- OE 2025 (Lei 24-D/2024) — 9 tranches :
--   ≤8 059 : 13% | 8 059-12 160 : 16,5% | 12 160-17 233 : 22%
--   17 233-22 306 : 25% | 22 306-28 400 : 32% | 28 400-41 629 : 35,5%
--   41 629-44 987 : 43,5% | 44 987-83 696 : 45% | >83 696 : 48%
--
-- Dedução específica (CIRS art. 25) : 4 104 € (2015-2022), 4 208 € (2023),
--   4 462 € (2024), 4 718 € (2025).
SELECT 1;
