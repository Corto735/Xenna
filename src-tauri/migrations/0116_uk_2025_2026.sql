-- ============================================================
-- ROYAUME-UNI : NI employeur 15 % dès l'exercice 2025/26
-- ============================================================
-- National Insurance Contributions (Secondary Class 1 Contributions) Act 2025 :
-- taux secondaire 13,8 % → 15 % et Secondary Threshold £9 100 → £5 000 au
-- 06/04/2025 (le seuil est codé dans uk_cotisations.rs). Inchangés en 2026/27
-- (GOV.UK, « Rates and thresholds for employers 2026 to 2027 »).
-- Part salariale inchangée : 8 % [PT–UEL], 2 % au-delà.
UPDATE cotisation_taux SET date_fin = '2025-04-05'
WHERE cotisation_id = (SELECT id FROM cotisation WHERE code = 'UK_NI_PAT')
  AND date_debut = '2024-04-06';

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'UK_NI_PAT'), '2025-04-06', NULL, '0.0000', '0.1500',
   '15 % du salaire excédant le ST (£5 000/an) dès 2025/26, reconduit en 2026/27. Source : NIC (Secondary Class 1 Contributions) Act 2025 ; GOV.UK rates and thresholds 2026-27.');
