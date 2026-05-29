-- 0053 — Japon : organismes collecteurs et textes de loi
-- Périmètre : salarié secteur privé, régime général (協会けんぽ Tokyo), 2024
-- Hypothèse : salarié ≥ 40 ans (介護保険 applicable)

INSERT INTO organisme (code, libelle, url) VALUES
  ('JP_KENPO',      '全国健康保険協会 / Kyokai Kenpo (Assurance Santé Nationale)',      'https://www.kyoukaikenpo.or.jp'),
  ('JP_MHLW',       '厚生労働省 / Ministère de la Santé, du Travail et des Affaires sociales (retraite)', 'https://www.mhlw.go.jp'),
  ('JP_HELLOWORK',  '公共職業安定所 / Hello Work — assurance emploi (雇用保険)',         'https://www.mhlw.go.jp/stf/seisakunitsuite/bunya/koyou_roudou/koyou/hellowork.html');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('JP_KENPO_HO', 'LOI', '健康保険法 / Health Insurance Act', 'Loi n°70 de 1922', '1922-04-22', '1926-07-01',
   'https://elaws.e-gov.go.jp/document?lawid=211AC0000000070',
   'Loi fondatrice de l''assurance maladie des salariés. Taux fixés par décret annuel. Tokyo 2024 : 9,98 % total (4,99 % chacun). Soins longue durée (介護) : 1,60 % total (0,80 % chacun, pour salariés ≥ 40 ans).'),

  ('JP_KOUNEN_HO', 'LOI', '厚生年金保険法 / Employees'' Pension Insurance Act', 'Loi n°115 de 1954', '1954-05-19', '1954-10-01',
   'https://elaws.e-gov.go.jp/document?lawid=329AC0000000115',
   'Régime de retraite des salariés (厚生年金). Taux unique national : 18,30 % (9,15 % chacun depuis oct. 2017). Plafond 標準報酬月額 : ¥650 000/mois (grade 32).'),

  ('JP_KOYO_HO', 'LOI', '雇用保険法 / Employment Insurance Act', 'Loi n°116 de 1974', '1974-12-28', '1975-04-01',
   'https://elaws.e-gov.go.jp/document?lawid=349AC0000000116',
   'Assurance chômage (雇用保険). Taux 2024 (一般の事業) : 1,55 % total — salarié 0,60 %, employeur 0,95 %. Assiette : salaire brut sans plafond.'),

  ('JP_ROUSAI_HO', 'LOI', '労働者災害補償保険法 / Workers'' Accident Compensation Insurance Act', 'Loi n°50 de 1947', '1947-04-07', '1947-09-01',
   'https://elaws.e-gov.go.jp/document?lawid=322AC0000000050',
   'Assurance accidents du travail (労災保険). 100 % patronale. Taux variable par secteur : bureaux/services généraux 0,30 %. Assiette : salaire brut total.'),

  ('JP_SHOTOKU_HO', 'LOI', '所得税法 / Income Tax Act', 'Loi n°33 de 1965', '1965-03-31', '1965-04-01',
   'https://elaws.e-gov.go.jp/document?lawid=340AC0000000033',
   'Impôt sur le revenu des personnes physiques (所得税). 7 tranches : 5/10/20/23/33/40/45 %. Retenue à la source mensuelle (源泉徴収). Surtaxe reconstruction : 2,1 % de l''impôt (復興特別所得税). Déduction emploi (給与所得控除) progressive.');
