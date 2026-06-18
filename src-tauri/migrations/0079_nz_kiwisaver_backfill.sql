-- 0079 — Nouvelle-Zélande : KiwiSaver employeur backfill 2015-2025 (taux stable 3 %)
-- Le taux minimal employeur KiwiSaver est 3 % depuis le 1er avril 2013 (inchangé).
-- La ligne 2025-04-01 → NULL existe déjà (0066). On étend la couverture en amont.
-- Source : KiwiSaver Act 2006.

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'NZ_KIWISAVER_EMP'), '2015-01-01', '2025-04-01', '0', '0.0300',
   (SELECT id FROM texte_loi WHERE code = 'NZ_KIWISAVER_2006'), 'KiwiSaver employeur 3 % (taux minimal, stable depuis 2013).');
