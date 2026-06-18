-- 0100 — Bulgarie : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé. Devise BGN. Données : 2025.
--
-- Cotisations sociales salarié 13,78 % / employeur 18,92 %, sur assiette plafonnée
-- (3 750 BGN/mois, plafond codé dans bg_bulletin.rs). Impôt 10 % proportionnel
-- calculé en Rust (bg_bulletin.rs). L'euro est adopté au 01/01/2026 ; 2025 reste en BGN.

INSERT INTO organisme (code, libelle, url) VALUES
  ('BG_NAP', 'НАП (NRA) — Agence nationale des recettes', 'https://nra.bg'),
  ('BG_NOI', 'НОИ (NSSI) — Institut national de sécurité sociale', 'https://www.nssi.bg');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('BG_KSO', 'LOI', 'Кодекс за социално осигуряване (Code de la sécurité sociale)', '—', '1999-12-29', '2000-01-01',
   'https://nra.bg',
   'Cotisations sociales 2025 : salarié ≈ 13,78 % (retraite DOO, maladie NZOK 3,2 %, 2ᵉ pilier UPF 2,2 %) / employeur ≈ 18,92 %. Assiette plafonnée au revenu maximal assurable de 3 750 BGN/mois.'),
  ('BG_ZDDFL', 'LOI', 'Закон за данъците върху доходите на физическите лица (impôt sur le revenu)', '—', '2006-11-24', '2008-01-01',
   'https://nra.bg',
   'Impôt sur le revenu des personnes physiques 2025 : 10 % proportionnel (flat tax), sur le revenu après cotisations salariales.'),
  ('BG_HISTOIRE', 'LOI', 'Bulgarie — histoire fiscale et sociale', '—', '2008-01-01', '2008-01-01',
   'https://nra.bg',
   'Flat tax à 10 % instaurée en 2008, l''une des plus basses de l''UE, dans une stratégie d''attractivité et de simplification post-transition. Système social par répartition (НОИ) complété d''un 2ᵉ pilier obligatoire (УПФ). Adoption de l''euro prévue au 01/01/2026. Politiquement : modèle low-tax assumé, débat récurrent sur le sous-financement de la protection sociale.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('BG_OSIG', 'Осигуровки — Cotisations sociales',
   (SELECT id FROM organisme WHERE code = 'BG_NOI'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 13,78 % salarié / 18,92 % employeur, assiette plafonnée à 3 750 BGN/mois.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'BG_OSIG'), '2025-01-01', NULL, '0.1378', '0.1892',
   (SELECT id FROM texte_loi WHERE code = 'BG_KSO'), 'OSIG 2025 : 13,78 % sal / 18,92 % pat (plafond 3 750 BGN).');
