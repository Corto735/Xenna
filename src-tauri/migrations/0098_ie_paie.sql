-- 0098 — Irlande : organismes, textes de loi, cotisations 2025
-- Périmètre : salarié secteur privé (PRSI Class A). Devise EUR. Données : 2025.
--
-- PRSI 4,1 % sal / 11,15 % pat. USC (bandes) et Income Tax (20/40 % avec crédits)
-- calculés en Rust (ie_bulletin.rs).

INSERT INTO organisme (code, libelle, url) VALUES
  ('IE_DSP',     'Department of Social Protection — PRSI', 'https://www.gov.ie/dsp'),
  ('IE_REVENUE', 'Revenue — administration fiscale',       'https://www.revenue.ie');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('IE_PRSI_L', 'LOI', 'Social Welfare Consolidation Act 2005 (PRSI)', '2005', '2005-12-27', '2006-01-01',
   'https://www.irishstatutebook.ie',
   'PRSI Class A 2025 : salarié 4,1 % / employeur 11,15 % (8,9 % sous le seuil hebdomadaire). USC : 0,5 % / 2 % / 3 % / 8 % (seuils 12 012 / 27 382 / 70 044 €).'),
  ('IE_TAX', 'LOI', 'Taxes Consolidation Act 1997 (Income Tax / PAYE)', '1997', '1997-11-30', '1998-01-01',
   'https://www.irishstatutebook.ie',
   'Income Tax 2025 : 20 % jusqu''à 44 000 €/an (célibataire), 40 % au-delà. Crédits d''impôt : personnel 2 000 € + PAYE 2 000 €.'),
  ('IE_HISTOIRE', 'LOI', 'Irlande — histoire fiscale et sociale', '—', '2011-01-01', '2011-01-01',
   'https://www.revenue.ie',
   'Modèle libéral anglo-saxon : protection sociale financée par l''impôt et un PRSI modéré, fiscalité des entreprises très basse (12,5 %) attirant les multinationales. L''USC, créé en 2011 en pleine crise (troïka), a durablement complété l''impôt sur le revenu. Politiquement : équilibre entre attractivité fiscale et redistribution par crédits d''impôt.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('IE_PRSI', 'PRSI (Class A) — Cotisation sociale',
   (SELECT id FROM organisme WHERE code = 'IE_DSP'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : 4,1 % salarié / 11,15 % employeur.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'IE_PRSI'), '2025-01-01', NULL, '0.041', '0.1115',
   (SELECT id FROM texte_loi WHERE code = 'IE_PRSI_L'), 'PRSI 2025 : 4,1 % sal / 11,15 % pat.');
