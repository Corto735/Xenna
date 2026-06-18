-- 0089 — Autriche : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé (Angestellte). Devise EUR. Données : 2025.
--
-- Sozialversicherung 18,07 % sal / ≈ 21,03 % pat, assiette plafonnée à 6 450 €/mois.
-- Lohnsteuer (barème progressif) calculé en Rust (at_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('AT_OEGK', 'Österreichische Gesundheitskasse — assurance sociale', 'https://www.gesundheitskasse.at'),
  ('AT_BMF',  'Bundesministerium für Finanzen — administration fiscale', 'https://www.bmf.gv.at');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('AT_ASVG', 'LOI', 'Allgemeines Sozialversicherungsgesetz (ASVG)', '189/1955', '1955-09-09', '1956-01-01',
   'https://www.ris.bka.gv.at',
   'Sozialversicherung 2025 : salarié 18,07 % (PV 10,25 + KV 3,87 + ALV 2,95 + AK 0,50 + WBF 0,50), employeur ≈ 21,03 %. Höchstbeitragsgrundlage 6 450 €/mois.'),
  ('AT_ESTG', 'LOI', 'Einkommensteuergesetz (EStG 1988)', '400/1988', '1988-07-07', '1989-01-01',
   'https://www.ris.bka.gv.at',
   'Lohnsteuer 2025 : 0 % jusqu''à 13 308 €, 20 % / 30 % / 40 % / 48 % / 50 % (seuils 21 617 / 35 836 / 69 166 / 103 072 €), 55 % au-delà de 1 000 000 €. 13ᵉ/14ᵉ mois imposés à 6 %.'),
  ('AT_HISTOIRE', 'LOI', 'Autriche — histoire du modèle social', '—', '1955-01-01', '1955-01-01',
   'https://www.gesundheitskasse.at',
   'Modèle bismarckien corporatiste : assurance sociale unifiée par l''ASVG (1955). Partenariat social (Sozialpartnerschaft) très structurant. 13ᵉ et 14ᵉ mois quasi-généralisés et fiscalement privilégiés. Politiquement : consensus social-démocrate/conservateur et fédéralisme.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('AT_SV', 'Sozialversicherung — Cotisations sociales',
   (SELECT id FROM organisme WHERE code = 'AT_OEGK'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
   '2025 : 18,07 % salarié / ≈ 21,03 % employeur. Assiette plafonnée à 6 450 €/mois.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'AT_SV'), '2025-01-01', NULL, '0.1807', '0.2103',
   (SELECT id FROM texte_loi WHERE code = 'AT_ASVG'), 'SV 2025 : 18,07 % sal / 21,03 % pat (plafond 6 450 €).');
