-- 0039 — Belgique : organismes collecteurs et textes de loi
-- Périmètre : régime général secteur privé, 2015-01/2026
-- Régions : Wallonie, Flandres, Bruxelles-Capitale

INSERT INTO organisme (code, libelle, url) VALUES
  ('ONSS',          'Office National de Sécurité Sociale / Rijksdienst voor Sociale Zekerheid', 'https://www.socialsecurity.be'),
  ('SPF_FINANCES_BE','SPF Finances — Précompte professionnel / Bedrijfsvoorheffing',             'https://finances.belgium.be'),
  ('SEPP_BE',        'Service fédéral des Pensions (référence)',                                 'https://www.sfpd.fgov.be');

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('LSSL_BE', 'LOI', 'Loi révisant l''arrêté-loi du 28/12/1944 concernant la sécurité sociale des travailleurs', 'Loi du 27/06/1969', '1969-06-27', '1969-07-01',
   'https://www.ejustice.just.fgov.be/eli/loi/1969/06/27/1969062701/justel',
   'Texte fondateur de l''ONSS. Fixe le taux salarial de 13,07 % et les cotisations patronales. Modifié par AR annuels.'),

  ('AR_ONSS_ANNUEL', 'ARRETE', 'AR fixant les cotisations ONSS — série annuelle 2015-2025', 'AR annuels', '2015-01-01', '2015-01-01',
   'https://www.socialsecurity.be/site_fr/employer/applics/tarifs/index.htm',
   'Arrêtés royaux annuels fixant les taux et les réductions (structurelle, bonus emploi). Taux salarial stable à 13,07 % depuis 2003.'),

  ('CIRCULAIRE_PP_BE', 'CIRCULAIRE', 'Circulaires SPF Finances — Barèmes PP/BV par région', 'Circulaires annuelles', '2015-01-01', '2015-01-01',
   'https://finances.belgium.be/fr/particuliers/bases/baremes_et_indices',
   'Barèmes annuels du précompte professionnel publiés par le SPF Finances. Depuis 2015, trois jeux de tables distincts : Flandres (réduction flamande), Wallonie (additionnels régionaux), Bruxelles.'),

  ('LOI_BONUS_EMPLOI_BE', 'LOI', 'Bonus emploi — réduction des cotisations personnelles de sécurité sociale', 'Loi du 20/12/1999', '1999-12-20', '2000-01-01',
   'https://www.ejustice.just.fgov.be',
   'Instaure la réduction des cotisations personnelles ONSS pour les travailleurs à bas salaires (bonus emploi). Montant et seuils révisés annuellement par AR.'),

  ('AR_REDUCTION_STRUCT_BE', 'ARRETE', 'AR du 16/05/2003 sur la réduction structurelle des cotisations patronales', 'AR 16/05/2003', '2003-05-16', '2003-07-01',
   'https://www.socialsecurity.be',
   'Instaure la réduction structurelle patronale. Formule et montants révisés annuellement. Réduit la charge patronale ONSS pour les bas et moyens salaires.'),

  ('CCT_43_BE', 'LOI', 'CCT n°43 — Revenu Minimum Mensuel Moyen Garanti (RMMMG)', 'CCT n°43 du 02/05/1988 (révisions)', '1988-05-02', '1988-05-02',
   'https://www.cnt-nar.be/CCT-COORD/cct-043.pdf',
   'Fixe le RMMMG (salaire minimum interprofessionnel belge). Révisé périodiquement par accord au sein du CNT. Série de révisions 2015-2025.');
