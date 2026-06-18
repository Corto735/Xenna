-- 0069 — Andorre : organismes, textes de loi et CASS 2025
-- Périmètre : salarié secteur privé. Devise EUR. Données : 2025.
-- Côté salarié : CASS 6,5 % (branche générale + retraite) + IRPF (0 / 5 / 10 %).
-- IRPF calculé en Rust (ad_bulletin.rs) ; taux CASS lu en base.

INSERT INTO organisme (code, libelle, url) VALUES
  ('AD_CASS', 'Caixa Andorrana de Seguretat Social (CASS) — sécurité sociale', 'https://www.cass.ad'),
  ('AD_TRIB', 'Govern d''Andorra — Departament de Tributs i Fronteres (IRPF)',  'https://www.impostos.ad');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('AD_LLEI_CASS', 'LOI', 'Llei 17/2008 de la seguretat social', 'Llei 17/2008', '2008-10-03', '2009-01-01',
   'https://www.bopa.ad',
   'CASS : salarié 6,5 % (branche générale + retraite), employeur 15,5 %, total 22 %.'),
  ('AD_LLEI_IRPF', 'LOI', 'Llei 5/2014 de l''impost sobre la renda de les persones físiques (IRPF)', 'Llei 5/2014', '2014-04-24', '2015-01-01',
   'https://www.bopa.ad',
   'IRPF (depuis 2015) : 0 % jusqu''à 24 000 €, 5 % de 24 001 à 40 000 €, 10 % au-delà de 40 000 €.');

INSERT INTO cotisation (code, libelle, organisme_id, categorie, applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES
  ('AD_CASS', 'CASS — Sécurité sociale (branche générale + retraite)',
   (SELECT id FROM organisme WHERE code = 'AD_CASS'), 'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
   '2025 : salarié 6,5 % (≈ 3 % général + 3,5 % retraite), employeur 15,5 %.');

INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin, taux_salarial, taux_patronal, texte_loi_id, notes) VALUES
  ((SELECT id FROM cotisation WHERE code = 'AD_CASS'), '2025-01-01', NULL, '0.065', '0.155',
   (SELECT id FROM texte_loi WHERE code = 'AD_LLEI_CASS'), 'CASS 2025 : 6,5 % sal / 15,5 % pat.');
