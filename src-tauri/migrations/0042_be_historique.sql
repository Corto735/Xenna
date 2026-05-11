-- 0042 — Belgique : textes de référence historiques RMMMG et PP
-- Les taux BE_ONSS_SAL (13,07 %) et BE_ONSS_PAT (25,92 %) sont stables
-- (une seule entrée date_fin NULL dans 0041 couvre toute la période).
-- Le barème PP et les paramètres bonus emploi / réduction structurelle
-- sont implémentés en Rust (be_pp.rs, be_cotisations.rs).

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES
  ('BE_CCT43_2017', 'LOI', 'CCT n°43 — RMMMG 1 562,59 €/mois (+4,0 %)', 'Révision 04/2017', '2017-03-24', '2017-04-01',
   'RMMMG porté à 1 562,59 €/mois. Source : CNT séance 24/03/2017.'),

  ('BE_CCT43_2018', 'LOI', 'CCT n°43 — RMMMG 1 593,81 €/mois (+2,0 %)', 'Révision 06/2018', '2018-05-25', '2018-06-01',
   'RMMMG porté à 1 593,81 €/mois. Source : CNT 25/05/2018.'),

  ('BE_CCT43_2020', 'LOI', 'CCT n°43 — RMMMG 1 625,72 €/mois (+2,0 %)', 'Révision 04/2020', '2020-03-27', '2020-04-01',
   'RMMMG porté à 1 625,72 €/mois. Source : CNT 27/03/2020.'),

  ('BE_CCT43_2022', 'LOI', 'CCT n°43 — RMMMG 1 806,16 €/mois (+11,1 %)', 'Programme relèvement 04/2022', '2022-03-18', '2022-04-01',
   'Début du programme de relèvement vers 1 994 € (2022-2024). CNT accord interprofessionnel 18/03/2022.'),

  ('BE_CCT43_2024', 'LOI', 'CCT n°43 — RMMMG 1 994,00 €/mois (+10,4 %)', 'Programme relèvement 04/2024', '2024-01-31', '2024-04-01',
   'Troisième étape du programme relèvement 2022-2024. Objectif atteint : 1 994 €. Source : CNT 31/01/2024.'),

  ('BE_CCT43_2025', 'LOI', 'CCT n°43 — RMMMG 2 070,48 €/mois (+3,8 %)', 'Indexation 2025', '2025-01-01', '2025-01-01',
   'Indexation RMMMG 2025. Source : SPF Emploi.');

-- Barème PP belge par année (implémenté dans be_pp.rs) :
--
-- Barème fédéral IPP (4 tranches depuis reform 2018, approximation 2015-2017) :
--   0-exo : 0 % (exonération de base)
--   exo-15 820 : 25 %
--   15 820-27 920 : 40 %
--   27 920-48 320 : 45 %
--   >48 320 : 50 %
--
-- Exonération de base (quotient exempté) :
--   2015 : 7 090 € | 2016 : 7 130 € | 2017 : 7 130 € | 2018 : 7 270 €
--   2019 : 8 860 € | 2020 : 8 990 € | 2021 : 9 050 € | 2022 : 9 270 €
--   2023 : 10 160 € | 2024 : 10 570 € | 2025 : 10 910 €
--
-- Forfait frais professionnels (30 % du brut, plafonné) :
--   2015-2019 : max 4 720 € | 2020 : max 5 100 € | 2021 : max 5 500 €
--   2022 : max 5 750 € | 2023-2024 : max 5 940 € | 2025 : max 6 010 €
--
-- Coefficients régionaux (appliqués sur PP fédéral) :
--   Flandres : × 0,94 (réduction flamande Vlaamse korting)
--   Wallonie : × 1,09 (additionnels régionaux wallons)
--   Bruxelles : × 1,00 (pas d''additionnel BXL)
--
-- Sources : SPF Finances, circulaires annuelles PP/BV ; Moniteur belge.
SELECT 1;
