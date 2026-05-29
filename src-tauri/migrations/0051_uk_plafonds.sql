-- 0051 — Royaume-Uni : plafonds de référence 2024/25
--
-- UK_PT  : Primary Threshold (NI salarié commence au-dessus)
-- UK_UEL : Upper Earnings Limit (NI salarié 2 % au-dessus)
-- UK_ST  : Secondary Threshold (NI employeur commence au-dessus)
-- UK_PA  : Personal Allowance (Income Tax — exonération de base)
--
-- Valeurs annuelles (en GBP). Division par 12 dans le code Rust.
-- Tous les seuils sont gelés depuis 2021/22 (gel fiscal jusqu'en 2028).
-- Source : Finance Act 2024 ; HMRC Tax & NI tables 2024-25.

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  -- Primary Threshold (PT) — seuil NI salarié
  ('UK_PT',  '2024-04-06', NULL, '12570.00', 'ANNUEL'),

  -- Upper Earnings Limit (UEL) — seuil passage au taux 2 %
  ('UK_UEL', '2024-04-06', NULL, '50270.00', 'ANNUEL'),

  -- Secondary Threshold (ST) — seuil NI employeur
  ('UK_ST',  '2024-04-06', NULL,  '9100.00', 'ANNUEL'),

  -- Personal Allowance (PA) — exonération Income Tax
  ('UK_PA',  '2024-04-06', NULL, '12570.00', 'ANNUEL');
