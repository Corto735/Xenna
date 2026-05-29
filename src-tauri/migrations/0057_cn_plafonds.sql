-- 0057 — Chine : plafonds de base de cotisation (Pékin 2024)
--
-- CN_BASE_MIN : base minimale de cotisation sociale mensuelle (Pékin)
-- CN_BASE_MAX : base maximale de cotisation sociale mensuelle (Pékin = 3× salaire moyen)
--
-- Si le salaire brut < MIN → cotisations calculées sur MIN.
-- Si le salaire brut > MAX → cotisations calculées sur MAX.
-- (Applicable aux cinq assurances ET au fonds logement.)
--
-- Source : Bureau de SS Pékin, 北京市人力资源和社会保障局公告 2024.
-- Salaire moyen Pékin 2023 (base 2024) : ¥11 761/mois → MAX = 3 × ¥11 761 = ¥35 283.
-- MIN = 60 % du salaire moyen = ¥7 056 (arrondi à ¥6 891 selon publication officielle).

INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
  ('CN_BASE_MIN', '2024-01-01', NULL, '6891', 'MENSUEL'),
  ('CN_BASE_MAX', '2024-01-01', NULL, '35283', 'MENSUEL');
