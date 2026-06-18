-- 0084 — Références historiques et politiques des régimes de paie (9 nouveaux pays)
-- Contexte fondateur + grandes réformes + caractère politique de chaque système.
-- Entrées dédiées dans texte_loi (réceptacle des références) ; faits historiques établis.

INSERT INTO texte_loi (code, type, titre, numero, date_parution, date_vigueur, url_legifrance, resume) VALUES
  ('NL_HISTOIRE', 'LOI', 'Pays-Bas — histoire du modèle social (modèle des polders)', '—', '1957-01-01', '1957-01-01',
   'https://www.belastingdienst.nl',
   'Modèle néerlandais bâti par consensus (« poldermodel »). Retraite de base universelle AOW instaurée en 1957 (gouvernement Drees). Système d''assurance maladie refondé en 2006 (Zorgverzekeringswet) sur une logique de marché régulé. Impôt sur le revenu réorganisé en « boxes » par la Wet IB 2001. Réformes récentes : passage à deux tranches (2020) puis retour à trois tranches (2024). Politiquement : État-providence corporatiste, coalitions de centre.'),

  ('AU_HISTOIRE', 'LOI', 'Australie — histoire du système social', '—', '1984-01-01', '1984-01-01',
   'https://www.ato.gov.au',
   'Pas de cotisations sociales salariales classiques : protection financée par l''impôt et l''épargne obligatoire. Medicare (santé universelle) créé en 1984 (gouvernement Hawke, travailliste). Superannuation Guarantee (retraite par capitalisation obligatoire) instaurée en 1992 par Paul Keating (travailliste), montée programmée de 9 % à 12 %. Réforme « Stage 3 » des barèmes en 2024. Politiquement : protection ciblée sous condition de ressources.'),

  ('NZ_HISTOIRE', 'LOI', 'Nouvelle-Zélande — histoire du système social', '—', '1974-01-01', '1974-01-01',
   'https://www.ird.govt.nz',
   'Protection sociale financée par l''impôt (pas de cotisations). ACC (Accident Compensation Corporation), assurance accidents sans égard à la faute, créée en 1974 — système quasi unique au monde. KiwiSaver (épargne-retraite volontaire) lancé en 2007. Tranche d''imposition à 39 % réintroduite en 2021 (gouvernement travailliste Ardern). Politiquement : pionnier de l''État-providence (1938) puis libéralisations des années 1980-90.'),

  ('PL_HISTOIRE', 'LOI', 'Pologne — histoire du système social', '—', '1999-01-01', '1999-01-01',
   'https://www.zus.pl',
   'ZUS hérité de l''entre-deux-guerres (1934), profondément réformé en 1999 (système à trois piliers, capitalisation partielle) après la transition post-communiste. Grande refonte fiscale « Polski Ład » (2022, gouvernement PiS) : santé non déductible, montant exonéré porté à 30 000 PLN, première tranche ramenée à 12 % en cours d''année. Politiquement : tensions récurrentes entre redistribution et compétitivité.'),

  ('KR_HISTOIRE', 'LOI', 'Corée du Sud — histoire du système social', '—', '1988-01-01', '1988-01-01',
   'https://www.nps.or.kr',
   'État-providence « développemental », construit tardivement et rapidement. Régime national de retraite (국민연금) créé en 1988 ; assurance maladie universelle atteinte en 1989 puis fusionnée en une caisse unique (NHIS) en 2000 ; assurance dépendance (장기요양) en 2008. Réformes des retraites en débat (hausse du taux de 9 % à partir de 2026). Politiquement : extension de la couverture portée par la démocratisation (post-1987).'),

  ('AD_HISTOIRE', 'LOI', 'Andorre — histoire fiscale et sociale', '—', '2015-01-01', '2015-01-01',
   'https://www.impostos.ad',
   'Longtemps sans impôt sur le revenu (« paradis fiscal »). Sous la pression de l''UE et de l''OCDE, l''IRPF est créé par la Llei 5/2014 et entre en vigueur le 1er janvier 2015 — tournant majeur de normalisation fiscale. Sécurité sociale (CASS) organisée par la Llei 17/2008. Politiquement : co-principauté ouvrant son économie et signant des accords d''échange d''informations fiscales.'),

  ('MC_HISTOIRE', 'LOI', 'Monaco — histoire fiscale et sociale', '—', '1963-01-01', '1963-05-18',
   'https://www.caisses-sociales.mc',
   'Pas d''impôt sur le revenu des personnes physiques depuis 1869 — fondement de l''attractivité monégasque. La « crise de 1962 » avec la France aboutit à la convention fiscale du 18 mai 1963 : les ressortissants français résidant à Monaco restent imposés par la France. Protection sociale gérée par les Caisses Sociales de Monaco (CCSS maladie/famille, CAR retraite par points). Politiquement : principauté souveraine liée à la France.'),

  ('DK_HISTOIRE', 'LOI', 'Danemark — histoire du modèle social', '—', '1994-01-01', '1994-01-01',
   'https://www.skat.dk',
   'État-providence universaliste financé massivement par l''impôt (modèle « flexicurité » : marché du travail flexible + protection forte). Contribution au marché du travail (AM-bidrag) de 8 % instaurée en 1994. ATP (retraite complémentaire forfaitaire) créée en 1964. Forte imposition municipale. Réforme fiscale 2024 (nouvelles tranches « mellemskat »/« topskat » à partir de 2026). Politiquement : large consensus social-démocrate.'),

  ('FI_HISTOIRE', 'LOI', 'Finlande — histoire du modèle social', '—', '1962-01-01', '1962-01-01',
   'https://www.vero.fi',
   'Modèle nordique : retraite liée aux revenus (TyEL) gérée par des caisses, assurance maladie via Kela. Imposition combinant impôt d''État progressif et impôt communal. Réforme historique de 2023 : création des « comtés de bien-être » (hyvinvointialueet) transférant le financement de la santé des communes à l''État — d''où la forte baisse de l''impôt communal moyen et la hausse du barème d''État. Politiquement : consensus nordique de l''État-providence.');
