-- 0031 — Espagne : organismes collecteurs et textes de loi
-- Périmètre : régime général secteur privé, contrato indefinido, 2015-01/2026

INSERT INTO organisme (code, libelle, url) VALUES
  ('TGSS',      'Tesorería General de la Seguridad Social',         'https://www.seg-social.es'),
  ('SEPE',      'Servicio Público de Empleo Estatal',               'https://www.sepe.es'),
  ('FOGASA_ES', 'Fondo de Garantía Salarial',                       'https://www.mites.gob.es/fogasa'),
  ('FUNDAE',    'Fundación Estatal para la Formación en el Empleo', 'https://www.fundae.es'),
  ('AEAT',      'Agencia Estatal de Administración Tributaria',     'https://www.agenciatributaria.gob.es');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('LGSS_ES', 'LOI', 'Ley General de la Seguridad Social', 'RDL 8/2015', '2015-10-30', '2016-01-02',
   'https://www.boe.es/buscar/act.php?id=BOE-A-2015-11724',
   'Texte fondateur de la SS espagnole. Fixe branches, taux sal/pat, plafonds (topes) et exonérations. En vigueur le 02/01/2016.'),

  ('LEY_21_2021_MEI', 'LOI', 'Mecanismo de Equidad Intergeneracional', 'Ley 21/2021', '2021-12-28', '2023-01-01',
   'https://www.boe.es/diario_boe/txt.php?id=BOE-A-2021-21653',
   'Crée le MEI : cotisation additionnelle alimentant le Fonds de réserve des retraites. Taux progressif 2023-2032. Art. 2 Ley 21/2021.'),

  ('ES_ORDENES_COTIZACION', 'ARRETE', 'Órdenes de cotización SS — série annuelle 2015-2025', 'Ordenes ESS/MITES annuelles', '2015-01-01', '2015-01-01',
   'https://www.seg-social.es',
   'Arrêtés annuels fixant bases min/max et taux. LGSS art. 147-162. Séries : ESS/86/2015, ESS/106/2016, ESS/106/2017, ESS/55/2018, PCM/242/2019, TMS/83/2021, ICT/182/2022, CEM/74/2023, TRM/43/2024, TRM/42/2025.');
