-- ============================================================
-- ORGANISMES COLLECTEURS
-- ============================================================
INSERT INTO organisme (code, libelle, url) VALUES
    ('URSSAF',        'Union de Recouvrement des cotisations de Sécurité Sociale et d''Allocations Familiales', 'https://www.urssaf.fr'),
    ('AGIRC_ARRCO',   'Association Générale des Institutions de Retraite Complémentaire', 'https://www.agirc-arrco.fr'),
    ('FRANCE_TRAVAIL', 'France Travail (ex-Pôle Emploi / UNEDIC)', 'https://www.francetravail.fr'),
    ('APEC',          'Association Pour l''Emploi des Cadres', 'https://www.apec.fr'),
    ('CADES',         'Caisse d''Amortissement de la Dette Sociale', 'https://www.cades.fr');

-- ============================================================
-- PARTENAIRES SOCIAUX
-- ============================================================
INSERT INTO partenaire_social (code, libelle, type, actif) VALUES
    -- Syndicats salariés
    ('CFDT',    'Confédération Française Démocratique du Travail',           'SYNDICAT_SALARIE',  1),
    ('CGT',     'Confédération Générale du Travail',                         'SYNDICAT_SALARIE',  1),
    ('FO',      'Force Ouvrière',                                            'SYNDICAT_SALARIE',  1),
    ('CFE_CGC', 'Confédération Française de l''Encadrement - CGC',           'SYNDICAT_SALARIE',  1),
    ('CFTC',    'Confédération Française des Travailleurs Chrétiens',        'SYNDICAT_SALARIE',  1),
    ('UNSA',    'Union Nationale des Syndicats Autonomes',                   'SYNDICAT_SALARIE',  1),
    ('SOLIDAIRES', 'Union Syndicale Solidaires',                             'SYNDICAT_SALARIE',  1),
    -- Syndicats patronaux
    ('MEDEF',   'Mouvement des Entreprises de France',                       'SYNDICAT_PATRONAL', 1),
    ('CPME',    'Confédération des Petites et Moyennes Entreprises',         'SYNDICAT_PATRONAL', 1),
    ('U2P',     'Union des Entreprises de Proximité',                        'SYNDICAT_PATRONAL', 1),
    -- Anciens noms (historique)
    ('CNPF',    'Conseil National du Patronat Français (devenu MEDEF 1998)', 'SYNDICAT_PATRONAL', 0),
    ('CGPME',   'Confédération Générale des PME (devenue CPME 2017)',        'SYNDICAT_PATRONAL', 0);

-- ============================================================
-- TEXTES DE LOI FONDATEURS
-- ============================================================
INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, resume) VALUES
    ('ORDONNANCE_1945',
     'ORDONNANCE',
     'Ordonnance portant organisation de la Sécurité Sociale',
     'n°45-2250',
     '1945-10-04',
     '1945-10-04',
     'Texte fondateur de la Sécurité Sociale française, signé par le Gouvernement Provisoire de la République Française. Crée les 4 branches : maladie, accidents du travail, vieillesse, famille.'),

    ('ANI_1947_CADRES',
     'ANI',
     'Convention Collective Nationale de Retraite et de Prévoyance des Cadres',
     NULL,
     '1947-03-14',
     '1947-03-14',
     'Crée le régime de retraite complémentaire obligatoire pour les cadres (AGIRC) et impose la cotisation patronale minimale de 1,5% pour la prévoyance décès.'),

    ('LOI_1898_AT',
     'LOI',
     'Loi sur les accidents du travail',
     NULL,
     '1898-04-09',
     '1898-04-09',
     'Première loi reconnaissant la responsabilité de l''employeur sans faute prouvée. Principe fondateur de la branche AT/MP : l''employeur assume le risque professionnel.'),

    ('LOI_1990_CSG',
     'LOI',
     'Loi de finances pour 1991 — Création de la CSG',
     'n°90-1168',
     '1990-12-29',
     '1991-01-01',
     'Crée la Contribution Sociale Généralisée sous le gouvernement Michel Rocard. Élargit le financement de la Sécurité Sociale aux revenus du capital et de remplacement, pas seulement aux salaires.'),

    ('ORDONNANCE_1996_CRDS',
     'ORDONNANCE',
     'Ordonnance portant création de la CRDS',
     'n°96-50',
     '1996-01-24',
     '1996-02-01',
     'Plan Juppé : crée la CRDS (0,5%) et la CADES pour rembourser la dette de la Sécurité Sociale. Prévu pour durer 13 ans, la CRDS existe toujours en 2025.'),

    ('ANI_2017_AGIRC_ARRCO',
     'ANI',
     'Accord national interprofessionnel sur la réforme des retraites complémentaires',
     NULL,
     '2017-11-17',
     '2019-01-01',
     'Fusion de l''AGIRC (cadres, 1947) et de l''ARRCO (non-cadres, 1961) en un régime unique AGIRC-ARRCO au 1er janvier 2019. Simplifie le système mais supprime la distinction de statut cadre/non-cadre pour la retraite complémentaire.'),

    ('LFSS_2018',
     'LOI',
     'Loi de Financement de la Sécurité Sociale pour 2018',
     'n°2017-1836',
     '2017-12-30',
     '2018-01-01',
     'Supprime les cotisations salariales maladie et chômage. En contrepartie, hausse de 1,7 point de CSG. Objectif : augmenter le salaire net sans toucher au coût employeur. Gouvernement Philippe / Macron.'),

    ('DECRET_2015_FILLON_FAM',
     'DECRET',
     'Décret relatif aux taux de cotisation famille réduit',
     'n°2015-390',
     '2015-04-03',
     '2015-04-01',
     'Instaure un taux réduit de cotisation allocations familiales (3,45% au lieu de 5,25%) pour les salaires inférieurs à 3,5 SMIC.');

-- Signataires de l'ANI AGIRC-ARRCO 2017
INSERT INTO texte_loi_signataire (texte_loi_id, partenaire_id, a_signe)
SELECT t.id, p.id, 1
FROM texte_loi t, partenaire_social p
WHERE t.code = 'ANI_2017_AGIRC_ARRCO'
  AND p.code IN ('CFDT', 'CFE_CGC', 'CFTC', 'MEDEF', 'CPME', 'U2P');

-- CGT et FO n'ont pas signé l'ANI 2017
INSERT INTO texte_loi_signataire (texte_loi_id, partenaire_id, a_signe)
SELECT t.id, p.id, 0
FROM texte_loi t, partenaire_social p
WHERE t.code = 'ANI_2017_AGIRC_ARRCO'
  AND p.code IN ('CGT', 'FO');
