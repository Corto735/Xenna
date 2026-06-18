-- 0093 — Slovénie : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise EUR. Données : 2025.
--
-- Prispevki 22,1 % salarié / 16,1 % employeur. Dohodnina (barème progressif) et
-- abattement général calculés en Rust (si_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('SI_FURS', 'Finančna uprava (FURS) — administration fiscale', 'https://www.fu.gov.si'),
  ('SI_ZPIZ', 'ZPIZ — institut des retraites et de l''invalidité', 'https://www.zpiz.si'),
  ('SI_ZZZS', 'ZZZS — institut d''assurance maladie',              'https://www.zzzs.si');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('SI_SOC', 'LOI', 'Zakon o pokojninskem in invalidskem zavarovanju (ZPIZ-2)', '96/2012', '2012-12-14', '2013-01-01',
   'https://www.uradni-list.si',
   'Cotisations 2025 : salarié 22,1 % (retraite/invalidité 15,5 % + maladie 6,36 % + chômage 0,14 % + parental 0,10 %), employeur 16,1 %.'),
  ('SI_DOH', 'LOI', 'Zakon o dohodnini (ZDoh-2)', '117/2006', '2006-11-16', '2007-01-01',
   'https://www.uradni-list.si',
   'Dohodnina 2025 : barème progressif 16 % / 26 % / 33 % / 39 % / 50 % (seuils 9 210 / 27 089 / 54 179 / 78 016 €). Abattement général ≈ 5 000 €/an (majoré et dégressif pour bas revenus).'),
  ('SI_HISTOIRE', 'LOI', 'Slovénie — histoire fiscale et sociale', '—', '1992-01-01', '1992-01-01',
   'https://www.fu.gov.si',
   'État social le plus généreux des nouveaux membres de l''UE, hérité du modèle yougoslave autogestionnaire et consolidé après l''indépendance (1991). Cotisations salariales élevées (22,1 %) et impôt fortement progressif (jusqu''à 50 %). Politiquement : forte tradition de concertation sociale et de redistribution.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('SI_PRISPEVKI', 'Prispevki — Cotisations sociales',
   (SELECT id FROM organisme WHERE code = 'SI_ZPIZ'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 22,1 % salarié / 16,1 % employeur.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'SI_PRISPEVKI'), '2025-01-01', NULL, '0.221', '0.161',
   (SELECT id FROM texte_loi WHERE code = 'SI_SOC'), 'Prispevki 2025 : 22,1 % sal / 16,1 % pat.');
