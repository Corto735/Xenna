-- 0077 — Andorre : backfill CASS 2015-2024 (taux stables)
-- La CASS (salarié 6,5 % / employeur 15,5 %) et le barème IRPF (0/5/10) sont restés
-- stables depuis la création de l'IRPF en 2015 (Llei 5/2014). On étend donc la couverture
-- à 2015-2024 avec les mêmes taux (la ligne 2025→NULL existe déjà en 0069).

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'AD_CASS'), '2015-01-01', '2025-01-01', '0.065', '0.155',
   (SELECT id FROM texte_loi WHERE code = 'AD_LLEI_CASS'), 'CASS 2015-2024 : 6,5 % sal / 15,5 % pat (taux stable sur la période).');
