-- 0050 — Royaume-Uni : organismes collecteurs et textes de loi
-- Périmètre : salarié secteur privé anglais, année fiscale 2024/25
-- Régime : National Insurance Class 1 + Income Tax PAYE

INSERT INTO organisme (code, libelle, url) VALUES
  ('HMRC', 'HM Revenue & Customs', 'https://www.gov.uk/government/organisations/hm-revenue-customs');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('UK_NIA_2014', 'LOI', 'National Insurance Contributions Act 2014', 'c. 7', '2014-05-14', '2015-04-06',
   'https://www.legislation.gov.uk/ukpga/2014/7/contents',
   'Encadre les cotisations National Insurance (Class 1, 1A, 1B, 2, 3, 4). Class 1 : prélevées sur salariés et employeurs au-dessus des seuils définis annuellement par statutory instrument.'),

  ('UK_ITA_2007', 'LOI', 'Income Tax Act 2007', 'c. 3', '2007-03-20', '2008-04-06',
   'https://www.legislation.gov.uk/ukpga/2007/3/contents',
   'Fixe le barème de l''impôt sur le revenu (Income Tax) : Personal Allowance, Basic Rate (20 %), Higher Rate (40 %), Additional Rate (45 %). Seuils révisés chaque année fiscale par Finance Act.'),

  ('UK_FINANCE_ACT_2024', 'LOI', 'Finance Act 2024 — Année fiscale 2024/25', 'c. 3', '2024-04-22', '2024-04-06',
   'https://www.legislation.gov.uk/ukpga/2024/3/contents',
   'Fixe les seuils et taux NI et Income Tax pour 2024/25 : Personal Allowance £12 570, PT £12 570, UEL £50 270, ST £9 100. NI salarié : 8 %/2 %. NI employeur : 13,8 %. Income Tax : 20/40/45 %.');
