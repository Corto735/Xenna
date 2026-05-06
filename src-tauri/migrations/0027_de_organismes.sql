-- ============================================================
-- ALLEMAGNE — Organismes et textes de loi
-- Sources : SGB V, VI, III, XI, VII — EStG — SolZG — KiStG
-- ============================================================

-- ── Organismes collecteurs ───────────────────────────────────
INSERT INTO organisme (code, libelle, url) VALUES
  ('DRV',    'Deutsche Rentenversicherung',
             'https://www.deutsche-rentenversicherung.de'),
  ('GKV',    'GKV-Spitzenverband — Kranken-/Pflegeversicherung',
             'https://www.gkv-spitzenverband.de'),
  ('BA_DE',  'Bundesagentur für Arbeit',
             'https://www.arbeitsagentur.de'),
  ('DGUV',   'Deutsche Gesetzliche Unfallversicherung',
             'https://www.dguv.de'),
  ('BMF_DE', 'Bundesministerium der Finanzen',
             'https://www.bundesfinanzministerium.de');

-- ── Textes de loi allemands ──────────────────────────────────
INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES

  ('SGB_V', 'LOI',
   'Sozialgesetzbuch V — Krankenversicherung',
   'SGB V', '1988-12-20', '1989-01-01',
   'Assurance maladie légale (GKV). Allgemeiner Beitragssatz : 14,6 % partagé à parts égales depuis le 01/01/2019. Avant 2019, le Zusatzbeitrag (taux additionnel) était à la charge exclusive du salarié. Plafond : Beitragsbemessungsgrenze KV.'),

  ('SGB_VI', 'LOI',
   'Sozialgesetzbuch VI — Rentenversicherung',
   'SGB VI', '1989-12-18', '1992-01-01',
   'Assurance retraite légale. Taux : 18,6 % (9,3 % chacun depuis 2018 ; 18,7 % / 9,35 % en 2015-2017). Plafond : Beitragsbemessungsgrenze RV, distinguant anciens Länder de l''Est et de l''Ouest jusqu''au 31/12/2024 ; unifié depuis le 01/01/2025.'),

  ('SGB_III', 'LOI',
   'Sozialgesetzbuch III — Arbeitslosenversicherung',
   'SGB III', '1997-03-24', '1998-01-01',
   'Assurance chômage. Taux : 3,0 % (2015-2018) → 2,6 % (2019) → 2,4 % (2020-2022) → 2,6 % (2023+). Même plafond que RV.'),

  ('SGB_XI', 'LOI',
   'Sozialgesetzbuch XI — Pflegeversicherung',
   'SGB XI', '1994-05-26', '1995-01-01',
   'Assurance dépendance (long-term care). Taux en hausse progressive : 2,35 % (2015-2016) → 2,55 % (2017-2018) → 3,05 % (2019-2023) → 3,40 % dès juillet 2023 → 3,60 % (2025+). Kinderlosenzuschlag (supplément sans enfant) exclusivement salarial. Même plafond que KV.'),

  ('SGB_VII', 'LOI',
   'Sozialgesetzbuch VII — Unfallversicherung',
   'SGB VII', '1996-08-07', '1997-01-01',
   'Assurance accidents du travail et maladies professionnelles. Exclusivement patronale. Taux variable par Berufsgenossenschaft (BG) et classe de risque. Taux moyen national indicatif DGUV : ~1,3 %.'),

  ('ESTG_DE', 'LOI',
   'Einkommensteuergesetz — Lohnsteuer',
   'EStG §32a, §38 ss.', '1934-10-16', '1934-10-16',
   'Impôt sur le revenu des salariés (Lohnsteuer). Barème progressif : 0 % jusqu''au Grundfreibetrag → 14 % à 42 % (zones de progression) → 42 % (Spitzensteuersatz) → 45 % (Reichensteuersatz). 6 Steuerklassen déterminant abattements et modalités de calcul. Grundfreibetrag revalorisé chaque année par la Legge de Finances (Jahressteuergesetz).'),

  ('SOLZG', 'LOI',
   'Solidaritätszuschlaggesetz',
   'SolZG', '1991-06-24', '1991-07-01',
   'Contribution de solidarité = 5,5 % de la Lohnsteuer (taux plein 2015-2020). Depuis le 01/01/2021, exonération pour ~90 % des contribuables : seuil ~17 543 €/an de Lohnsteuer (zone de transition jusqu''à ~66 915 €/an) ; taux plein au-delà. En pratique quasi-nul pour salaires < ~80 000 €/an.'),

  ('KISTG_DE', 'LOI',
   'Kirchensteuergesetze der Länder (KiStG)',
   'KiStG (16 Länder)', NULL, NULL,
   'Taxe d''église perçue par retenue à la source sur la Lohnsteuer si le salarié est membre d''une église concordataire. Taux : 8 % de la Lohnsteuer en Bayern et Baden-Württemberg ; 9 % dans les 14 autres Länder. Optionnelle : le salarié doit déclarer son appartenance confessionnelle.');
