-- 0035 — Portugal : organismes collecteurs et textes de loi
-- Périmètre : régime geral de segurança social (secteur privé), 2015-01/2026

INSERT INTO organisme (code, libelle, url) VALUES
  ('IGFSS',  'Instituto de Gestão Financeira da Segurança Social',                 'https://www.igfss.gov.pt'),
  ('ACT_PT', 'Autoridade para as Condições do Trabalho — Acidentes de Trabalho',  'https://www.act.gov.pt'),
  ('FCT_PT', 'Fundo de Compensação do Trabalho / FGCT',                           'https://www.fct.gov.pt'),
  ('AT_PT',  'Autoridade Tributária e Aduaneira (IRS)',                            'https://www.portaldasfinancas.gov.pt');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('CODIGO_CONTRIBUTIVO_PT', 'LOI', 'Código dos Regimes Contributivos do Sistema Previdencial de Segurança Social', 'Lei 110/2009', '2009-09-16', '2010-01-01',
   'https://www.seg-social.pt/legislacao?journal=536929',
   'Texte fondateur du régime contributif SS portugaise. TSU : 11 % sal + 23,75 % pat (régime général). Art. 53-54. Modifié par DL 25/2017 et OE annuelles.'),

  ('FCT_DL_PT', 'DECRET', 'Regime FCT et FGCT', 'DL 210/2015', '2015-09-25', '2015-10-01',
   'https://www.dre.pt/dre/detalhe/decreto-lei/210-2015-70326',
   'Instaure FCT (0,925 % pat, CDI) et FGCT (0,075 % pat). Garantie paiement indemnités licenciement. Portaria 1458/2009 (régime pilote antérieur).'),

  ('CIRS_PT', 'LOI', 'Código do IRS — barème et retenção na fonte', 'DL 442-A/88', '1988-11-30', '1989-01-01',
   'https://www.portaldasfinancas.gov.pt/at/html/index.html',
   'CIRS art. 68 : barème progressif IRS. Art. 99 : retenção na fonte (retenue à la source par l''employeur). Tables AT publiées annuellement.'),

  ('PT_OE_ANNUAL', 'LOI', 'Lei do Orçamento do Estado — série annuelle 2015-2025', 'Lei OE annuelle', '2015-01-01', '2015-01-01',
   'https://www.dre.pt',
   'OE fixent taux IRS, tranches et tables de retenue. Séries : Lei 82-B/2014, 7-A/2016, 42/2016, 114/2017, 71/2018, 2/2020, 75-B/2020, 12/2022, 24-D/2022, 24/2023, 24-D/2024.'),

  ('PT_SMN_ANNUAL', 'DECRET', 'Salário Mínimo Nacional — série annuelle 2015-2025', 'DL annuel', '2015-01-01', '2015-01-01',
   'https://www.dre.pt',
   'Décrets annuels SMN : DL 144/2015, 254-A/2015, 86-B/2016, 156/2017, 619/2018, 107/2019, 109-G/2021, 109-A/2021, 119/2022, 107/2023, 125/2024.');
