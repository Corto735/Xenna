-- ============================================================
-- ITALIE — Esonero contributivo 2024 (taglio cuneo)
--
-- L.213/2023 art. 1 cc. 15-17 (Legge di Bilancio 2024) proroge
-- et amplifie le taglio cuneo de 2023 :
--   • Reddito annuel estimé ≤ 25 000 € → −7 pp IVS salarié
--   • Reddito annuel estimé 25 001–35 000 € → −6 pp IVS salarié
--
-- Même mécanisme que 2023 : taux maximum stocké en BDD (−7 pp),
-- le code Rust ajuste à −6 pp selon l'estimation de revenu.
--
-- À partir de 2025 : L.207/2024 remplace le mécanisme par un
-- bonus IRPEF (non cotisatoire) — aucun IT_ESONERO_2025 à créer.
-- ============================================================

INSERT INTO cotisation (code, libelle, organisme_id, categorie,
  applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('IT_ESONERO_2024',
   'Esonero contributivo 2024 — Taglio cuneo salarie (-6/-7 %)',
   (SELECT id FROM organisme WHERE code='INPS'),
   'AUTRES', 1, 1, 'BRUT_TOTAL',
   'Réduction de la cotisation IVS salarié. -7 pp si reddito annuel estimé <= 25 000 EUR ; -6 pp si <= 35 000 EUR. L.213/2023 art. 1 cc. 15-17 (Bilancio 2024). Même mécanisme que 2023 avec taux plus élevés.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_ESONERO_2024'),
   '2024-01-01', '2024-12-31', '-0.0700', '0.0000',
   'Taux maximum stocké (-7 pp pour reddito <= 25 000 EUR). Le Rust applique -6 pp pour reddito 25 001-35 000 EUR. L.213/2023.');
