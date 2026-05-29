-- 0056 — Chine : organismes collecteurs et textes de loi
-- Périmètre : salarié secteur privé, base Pékin 2024
-- Régime : 五险一金 (cinq assurances + fonds logement) + 个人所得税

INSERT INTO organisme (code, libelle, url) VALUES
  ('CN_SS_BEIJING',  'Bureau de sécurité sociale de Pékin / 北京市社会保险基金管理中心', 'https://rsj.beijing.gov.cn'),
  ('CN_CPF_BEIJING', 'Centre de gestion du fonds de logement de Pékin / 北京住房公积金管理中心', 'https://gjj.beijing.gov.cn'),
  ('CN_SAT',         'Administration fiscale de l''État / 国家税务总局 (IIT)',              'https://www.chinatax.gov.cn');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('CN_SHEHUI_BAOXIAN_FA', 'LOI', '社会保险法 / Loi sur les assurances sociales', 'Loi du 28/10/2010', '2010-10-28', '2011-07-01',
   'http://www.npc.gov.cn/npc/c30834/201007/a34b0b26e6d14c5c89bef576f7c3af5a.shtml',
   'Loi cadre des cinq assurances sociales : retraite (养老), maladie (医疗), chômage (失业), accidents du travail (工伤), maternité (生育). Taux fixés par décret local annuel. Pékin 2024 : voir 0058_cn_cotisations.sql.'),

  ('CN_GONGJIJIN_TIAOLI', 'DECRET', '住房公积金管理条例 / Règlement sur la gestion du fonds de logement', 'Décret n°262 (1999, révisé 2019)', '1999-04-03', '1999-04-03',
   'https://www.gov.cn/gongbao/content/1999/content_60197.htm',
   'Instaure le fonds de logement obligatoire (住房公积金). Employeur et salarié cotisent chacun (5–12 % selon ville). Pékin 2024 : 12 % chacun. Base : min/max fixés annuellement.'),

  ('CN_IIT_FA', 'LOI', '个人所得税法 / Loi sur l''impôt sur le revenu des personnes physiques (réforme 2018)', 'Loi du 31/08/2018', '2018-08-31', '2019-01-01',
   'https://www.chinatax.gov.cn/chinatax/n364/n375/c5218367/content.html',
   'Grande réforme IIT 2018 : déduction forfaitaire mensuelle portée à ¥5 000 ; tranches annuelles (3/10/20/25/30/35/45 %) ; six déductions spéciales optionnelles (enfants, logement, formation…). Retenue mensuelle sur base cumulative annualisée.');
