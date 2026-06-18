-- 0088 — Lituanie : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé (régime général). Devise EUR. Données : 2025.
--
-- Sodra : 19,50 % salarié / 1,77 % employeur. GPM (impôt) 20 % / 32 % avec NPD
-- (montant non imposable) dégressif, calculé en Rust (lt_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('LT_VMI',   'Valstybinė mokesčių inspekcija — administration fiscale', 'https://www.vmi.lt'),
  ('LT_SODRA', 'Sodra — assurance sociale d''État',                       'https://www.sodra.lt');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('LT_SOC', 'LOI', 'Valstybinio socialinio draudimo įstatymas — assurance sociale', '—', '1991-05-21', '1991-06-01',
   'https://www.e-tar.lt',
   'Sodra 2025 (régime général, réforme 2019) : 19,50 % salarié (retraite 8,72 % + maladie/PSD 6,98 % + maternité…), 1,77 % employeur (hors majoration CDD). Accumulation pension volontaire +3 % possible.'),
  ('LT_GPM', 'LOI', 'Gyventojų pajamų mokesčio įstatymas — impôt sur le revenu', '—', '2002-07-02', '2003-01-01',
   'https://www.e-tar.lt',
   'GPM 2025 : 20 % jusqu''à 60 VDU/an (≈ 10 540 €/mois), 32 % au-delà. NPD (montant non imposable) dégressif : 747 € jusqu''au salaire minimum, décroissant ensuite.'),
  ('LT_HISTOIRE', 'LOI', 'Lituanie — histoire fiscale et sociale', '—', '1991-01-01', '1991-01-01',
   'https://www.vmi.lt',
   'Assurance sociale Sodra héritée de la reconstruction post-soviétique (1991). Réforme majeure de 2019 : fusion et transfert quasi-total des cotisations vers le salarié (avec revalorisation brute compensatoire), abaissant fortement la part employeur. Impôt proportionnel devenu progressif (20 % / 32 %) avec montant non imposable dégressif. Politiquement : modèle balte de fiscalité simple et compétitive.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('LT_SODRA', 'Sodra — Cotisations sociales',
   (SELECT id FROM organisme WHERE code = 'LT_SODRA'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 19,50 % salarié / 1,77 % employeur (régime général).');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'LT_SODRA'), '2025-01-01', NULL, '0.195', '0.0177',
   (SELECT id FROM texte_loi WHERE code = 'LT_SOC'), 'Sodra 2025 : 19,50 % sal / 1,77 % pat.');
