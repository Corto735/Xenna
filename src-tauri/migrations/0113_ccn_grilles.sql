-- ============================================================
-- LE CHAKRRAM — refonte : grilles de salaires conventionnels
-- et maintien de salaire maladie, IDCC 0016
--
-- La page précédente affichait de la prose rédigée de mémoire
-- métier, en 'a_verifier' du début à la fin. On repart de zéro
-- sur un principe simple : n'afficher que ce qui a été lu dans
-- le texte, avec la source, la date d'effet et la date de
-- consultation en face de chaque tableau.
--
-- PROVENANCE DE CHAQUE GRILLE (consultation du 12 août 2026) :
--
--  • Marchandises — Accord du 11 octobre 2023, applicable au
--    1er décembre 2023, étendu par arrêté du 19 décembre 2023.
--    Tableaux lus sur Légifrance (KALITEXT000049067154).
--    C'est le dernier accord de revalorisation étendu pour la
--    branche marchandises : la grille n'a pas bougé depuis, et
--    le SMIC est passé devant plusieurs coefficients d'entrée.
--
--  • Voyageurs — Avenants n° 120 (ouvriers), 102 (employés),
--    100 (TAM) et 93 (ingénieurs et cadres) du 27 novembre
--    2025, applicables au 1er janvier 2026, étendus par arrêté
--    du 7 avril 2026 (JO du 10 avril 2026, NOR TRST2607009A).
--    Tableaux lus sur les avenants signés eux-mêmes.
--
--  • Déménagement — Avenant n° 24 du 21 janvier 2026 à l'accord
--    du 1er février 2003, applicable au plus tard le 1er juin
--    2026. Texte signé, tableaux annexés lus intégralement.
--
--  • Prestations logistiques — Avenant n° 16 du 9 avril 2025,
--    applicable au 1er mai 2025. Lu sur Légifrance
--    (KALITEXT000051927426).
--
--  • Transport sanitaire — Avenant n° 8 du 6 mai 2025 à
--    l'accord du 16 février 2004, applicable au 1er juin 2025,
--    étendu par arrêté du 22 juillet 2025.
--
--  • Maintien de salaire maladie — annexes I à IV de la
--    convention : art. 10 ter (ouvriers), 17 bis (employés),
--    21 bis (TAM), 21 bis (ingénieurs et cadres).
--
-- Ces tables sont ÉDITORIALES : aucun calcul de bulletin ne les
-- lit. Le moteur garde ses barèmes dans ContextPaie et
-- calculs/absence.rs.
-- ============================================================

-- ── Branches (sous-champs conventionnels) ────────────────────
CREATE TABLE ccn_branches (
    id       INTEGER PRIMARY KEY,
    idcc     TEXT    NOT NULL REFERENCES ccn_conventions(idcc) ON DELETE CASCADE,
    code     TEXT    NOT NULL,
    libelle  TEXT    NOT NULL,
    detail   TEXT,
    ordre    INTEGER NOT NULL DEFAULT 0,
    UNIQUE (idcc, code)
);

-- ── Grilles de minima : une par (branche, catégorie) ─────────
CREATE TABLE ccn_grilles (
    id          INTEGER PRIMARY KEY,
    idcc        TEXT    NOT NULL REFERENCES ccn_conventions(idcc) ON DELETE CASCADE,
    branche     TEXT    NOT NULL,
    categorie   TEXT    NOT NULL CHECK (categorie IN ('ouvriers','employes','tam','cadres')),

    intitule    TEXT    NOT NULL,   -- titre affiché
    corps       TEXT    NOT NULL,   -- lecture en paie, sauts de ligne réels
    tableaux    TEXT    NOT NULL,   -- JSON [{titre,colonnes,lignes,note}]

    source      TEXT    NOT NULL,   -- texte conventionnel exact
    source_url  TEXT,
    extension   TEXT,               -- arrêté d'extension
    date_effet  TEXT    NOT NULL,   -- AAAA-MM-JJ : début de validité
    consulte_le TEXT    NOT NULL,   -- date de lecture de la source
    ordre       INTEGER NOT NULL DEFAULT 0,
    UNIQUE (idcc, branche, categorie)
);

-- ── Maintien de salaire maladie : une entrée par catégorie ───
CREATE TABLE ccn_maintien (
    id          INTEGER PRIMARY KEY,
    idcc        TEXT    NOT NULL REFERENCES ccn_conventions(idcc) ON DELETE CASCADE,
    categorie   TEXT    NOT NULL CHECK (categorie IN ('ouvriers','employes','tam','cadres')),
    intitule    TEXT    NOT NULL,
    article     TEXT    NOT NULL,
    corps       TEXT    NOT NULL,
    tableaux    TEXT    NOT NULL,
    source      TEXT    NOT NULL,
    source_url  TEXT,
    consulte_le TEXT    NOT NULL,
    ordre       INTEGER NOT NULL DEFAULT 0,
    UNIQUE (idcc, categorie)
);

-- ── Branches de l'IDCC 16 ────────────────────────────────────
INSERT INTO ccn_branches (idcc, code, libelle, detail, ordre) VALUES
('0016', 'marchandises',  'Transport routier de marchandises',
 'Marchandises et activités auxiliaires du transport (CCNA 1 à 4)', 1),
('0016', 'voyageurs',     'Transport routier de voyageurs',
 'Lignes régulières, scolaire, tourisme (CCNA 1 à 4)', 2),
('0016', 'demenagement',  'Transport de déménagement',
 'Accord du 1er février 2003 sur les rémunérations conventionnelles', 3),
('0016', 'logistique',    'Prestations logistiques',
 'Entrepôts et prestations logistiques (coefficients « L »)', 4),
('0016', 'sanitaire',     'Transport sanitaire',
 'Personnels ambulanciers — accord du 16 février 2004', 5);

-- ============================================================
-- MARCHANDISES — accord du 11 octobre 2023
-- ============================================================

INSERT INTO ccn_grilles
  (idcc, branche, categorie, intitule, corps, tableaux,
   source, source_url, extension, date_effet, consulte_le, ordre)
VALUES

('0016', 'marchandises', 'ouvriers',
 'Personnel ouvrier — transport routier de marchandises',
 'Trois chiffres cohabitent dans cette grille et ne servent pas à la même chose.

Le taux horaire conventionnel est le plancher de la ligne « salaire de base » et la base de toute retenue pour absence. Les colonnes d''ancienneté ne sont pas une prime distincte : elles majorent le taux du coefficient de 2 % à 2 ans, 4 % à 5 ans, 6 % à 10 ans et 8 % à 15 ans.

La garantie annuelle de rémunération (GAR) est le vrai contrôle. Elle se vérifie sur l''année civile complète, rémunération brute effectivement versée, et se régularise sur la paie de décembre. Trois barèmes coexistent selon le service : 151,67 h pour un sédentaire, 169 h en courte distance, 200 h en longue distance. Diviser le brut d''un grand routier par 151,67 produit un taux horaire faux.

Ce qui n''entre ni dans la comparaison au minimum ni dans la GAR : les remboursements de frais de déplacement (repas, casse-croûte, découcher), qui sont des frais professionnels et non du salaire.

Point de vigilance à la date de consultation : la branche marchandises n''a pas revalorisé sa grille depuis le 1er décembre 2023. Le SMIC horaire est passé à 12,31 € au 1er juin 2026, soit au-dessus des taux d''embauche 110M à 138M. C''est alors le SMIC qui s''applique — payer le taux conventionnel serait payer sous le minimum légal.',
 '[{"titre":"Taux horaires conventionnels (en €)","colonnes":["Coefficient","À l''embauche","Après 2 ans","Après 5 ans","Après 10 ans","Après 15 ans"],"lignes":[["110M à 120M","12,09","12,3318","12,5736","12,8154","13,0572"],["128M","12,12","12,3624","12,6048","12,8472","13,0896"],["138M","12,14","12,3828","12,6256","12,8684","13,1112"],["150M","12,43","12,6786","12,9272","13,1758","13,4244"]],"note":"Majorations d''ancienneté : + 2 % à 2 ans, + 4 % à 5 ans, + 6 % à 10 ans, + 8 % à 15 ans, appliquées au taux du coefficient à l''embauche."},{"titre":"Garantie annuelle de rémunération — 151,67 h/mois (en €)","colonnes":["Coefficient","À l''embauche","Après 2 ans","Après 5 ans","Après 10 ans","Après 15 ans"],"lignes":[["110M à 120M","22 664,41","23 117,70","23 570,99","24 024,28","24 477,57"],["128M","22 720,65","23 175,06","23 629,48","24 083,89","24 538,30"],["138M","22 758,14","23 213,31","23 668,47","24 123,63","24 578,80"],["150M","23 301,79","23 767,83","24 233,86","24 699,90","25 165,93"]],"note":"Personnel sédentaire et roulant à l''horaire légal."},{"titre":"Garantie annuelle de rémunération — 169 h/mois, courte distance (en €)","colonnes":["Coefficient","À l''embauche","Après 2 ans","Après 5 ans","Après 10 ans","Après 15 ans"],"lignes":[["115M à 120M","25 901,12","26 419,14","26 937,16","27 455,18","27 973,21"],["128M","25 965,39","26 484,70","27 004,00","27 523,31","28 042,62"],["138M","26 008,24","26 528,40","27 048,57","27 568,73","28 088,89"],["150M","26 629,52","27 162,11","27 694,70","28 227,29","28 759,88"]],"note":"Durée d''équivalence des personnels roulants « courte distance »."},{"titre":"Garantie annuelle de rémunération — 200 h/mois, longue distance (en €)","colonnes":["Coefficient","À l''embauche","Après 2 ans","Après 5 ans","Après 10 ans","Après 15 ans"],"lignes":[["115M à 120M","32 202,68","32 846,74","33 490,79","34 134,84","34 778,90"],["128M","32 282,59","32 928,24","33 573,89","34 219,54","34 865,20"],["138M","32 335,86","32 982,58","33 629,30","34 276,01","34 922,73"],["150M","33 108,30","33 770,47","34 432,63","35 094,80","35 756,96"]],"note":"Durée d''équivalence des personnels roulants « longue distance » (grands routiers)."},{"titre":"Indemnités pour travail du dimanche et des jours fériés (en €)","colonnes":["Durée du travail","Montant"],"lignes":[["Moins de 3 heures","12,45"],["3 heures et plus","28,94"]],"note":"Montants fixés par le même accord, applicables au 1er décembre 2023."}]',
 'Accord du 11 octobre 2023 relatif à la revalorisation des rémunérations (annexe CCNA 1)',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000049067154/?idConteneur=KALICONT000005635624',
 'Étendu par arrêté du 19 décembre 2023',
 '2023-12-01', '2026-08-12', 10),

('0016', 'marchandises', 'employes',
 'Personnel employé — transport routier de marchandises',
 'Grille des employés administratifs et d''exploitation (CCNA 2). La progression d''ancienneté est ici triennale et plus généreuse que celle des ouvriers : + 3 % tous les trois ans, jusqu''à + 15 % à quinze ans, contre + 8 % au plafond côté ouvriers.

Les cinq lignes couvrent l''ensemble des coefficients de la classification : les coefficients 105 à 120 partagent le même taux d''entrée.

Comme pour les ouvriers, l''accord fixe également une garantie annuelle de rémunération par coefficient, non reproduite ici faute d''avoir pu la lire ligne à ligne dans le texte. Le taux horaire reste le contrôle mensuel ; la GAR, le contrôle annuel.

À la date de consultation, le SMIC (12,31 €/h) dépasse les taux d''embauche des coefficients 105 à 140 : c''est le SMIC qui s''applique.',
 '[{"titre":"Taux horaires conventionnels (en €)","colonnes":["Coefficient","À l''embauche","Après 3 ans","Après 6 ans","Après 9 ans","Après 12 ans","Après 15 ans"],"lignes":[["105 à 120","12,09","12,4527","12,8154","13,1781","13,5408","13,9035"],["125","12,10","12,4630","12,8260","13,1890","13,5520","13,9150"],["132,5","12,12","12,4836","12,8472","13,2108","13,5744","13,9380"],["140","12,15","12,5145","12,8790","13,2435","13,6080","13,9725"],["148,5","12,43","12,8029","13,1758","13,5487","13,9216","14,2945"]],"note":"Majorations d''ancienneté : + 3 % à 3 ans, + 6 % à 6 ans, + 9 % à 9 ans, + 12 % à 12 ans, + 15 % à 15 ans, appliquées au taux du coefficient à l''embauche (art. 3 CCNA 2)."}]',
 'Accord du 11 octobre 2023 relatif à la revalorisation des rémunérations (annexe CCNA 2)',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000049067154/?idConteneur=KALICONT000005635624',
 'Étendu par arrêté du 19 décembre 2023',
 '2023-12-01', '2026-08-12', 11),

('0016', 'marchandises', 'tam',
 'Techniciens et agents de maîtrise — transport routier de marchandises',
 'Huit coefficients, de 150 (technicien d''exploitation débutant) à 225 (agent de maîtrise de haut niveau). Progression triennale de + 3 % par palier, plafonnée à + 15 % après quinze ans, appliquée au taux d''embauche du coefficient (art. 4 CCNA 3).

Les taux d''ancienneté publiés au texte sont exactement le taux d''embauche majoré du pourcentage du palier : ils sont reproduits ici avec la même règle, à quatre décimales, parce que la troisième décimale d''un taux horaire finit par se voir sur un cumul annuel.

L''accord fixe aussi une garantie annuelle de rémunération par coefficient, du même ordre de grandeur que le taux horaire × 151,67 × 12, non reproduite ici.

Seul le coefficient 150 est proche du SMIC (12,60 € contre 12,31 €) ; les autres restent nettement au-dessus.',
 '[{"titre":"Taux horaires conventionnels (en €)","colonnes":["Coefficient","À l''embauche","Après 3 ans","Après 6 ans","Après 9 ans","Après 12 ans","Après 15 ans"],"lignes":[["150","12,60","12,9780","13,3560","13,7340","14,1120","14,4900"],["157,5","12,73","13,1119","13,4938","13,8757","14,2576","14,6395"],["165","13,34","13,7402","14,1404","14,5406","14,9408","15,3410"],["175","14,17","14,5951","15,0202","15,4453","15,8704","16,2955"],["185","14,94","15,3882","15,8364","16,2846","16,7328","17,1810"],["200","16,17","16,6551","17,1402","17,6253","18,1104","18,5955"],["215","17,37","17,8911","18,4122","18,9333","19,4544","19,9755"],["225","18,21","18,7563","19,3026","19,8489","20,3952","20,9415"]],"note":"Majorations d''ancienneté : + 3 %, + 6 %, + 9 %, + 12 %, + 15 % appliquées au taux d''embauche du coefficient (art. 4 CCNA 3)."}]',
 'Accord du 11 octobre 2023 relatif à la revalorisation des rémunérations (annexe CCNA 3)',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000049067154/?idConteneur=KALICONT000005635624',
 'Étendu par arrêté du 19 décembre 2023',
 '2023-12-01', '2026-08-12', 12),

('0016', 'marchandises', 'cadres',
 'Ingénieurs et cadres — transport routier de marchandises',
 'Les cadres ne relèvent pas d''un taux horaire mais d''une rémunération annuelle garantie (RAG), doublée d''un paiement mensuel minimum. La RAG se contrôle sur l''année civile et intègre les primes de toute nature versées en contrepartie du travail ; le paiement mensuel minimum, lui, est un plancher de trésorerie mois par mois.

L''ancienneté est décomptée dans le groupe, pas dans l''entreprise (art. 5 al. 4 CCNA 4) : un cadre promu repart à la première tranche de son nouveau coefficient.

Les rémunérations minimales professionnelles garanties sont majorées de 10 % en région parisienne (art. 5 al. 2 CCNA 4). Le groupe 7 (cadres supérieurs) relève d''un régime propre, sans grille chiffrée.',
 '[{"titre":"Rémunérations annuelles garanties (en €)","colonnes":["Coefficient","Ancienneté dans le groupe","Rémunération annuelle garantie","Paiement mensuel minimum"],"lignes":[["100","Jusqu''à 5 ans","34 957,27","2 621,80"],["100","5 à 10 ans","36 705,13","2 752,88"],["100","10 à 15 ans","38 453,00","2 883,98"],["100","Après 15 ans","40 200,86","3 015,06"],["106,5","Jusqu''à 5 ans","37 226,07","2 791,96"],["106,5","5 à 10 ans","39 087,37","2 931,55"],["106,5","10 à 15 ans","40 948,68","3 071,15"],["106,5","Après 15 ans","42 809,98","3 210,75"],["113","Jusqu''à 5 ans","39 495,36","2 962,15"],["113","5 à 10 ans","41 470,13","3 110,26"],["113","10 à 15 ans","43 444,89","3 258,37"],["113","Après 15 ans","45 419,66","3 406,47"],["119","Jusqu''à 5 ans","41 588,71","3 119,15"],["119","5 à 10 ans","43 668,15","3 275,11"],["119","10 à 15 ans","45 747,58","3 431,07"],["119","Après 15 ans","47 827,02","3 587,03"],["132","Jusqu''à 5 ans","46 126,60","3 459,50"],["132","5 à 10 ans","48 432,93","3 632,47"],["132","10 à 15 ans","50 739,26","3 805,44"],["132","Après 15 ans","53 045,60","3 978,42"],["145","Jusqu''à 5 ans","50 664,72","3 799,85"],["145","5 à 10 ans","53 197,95","3 989,85"],["145","10 à 15 ans","55 731,18","4 179,84"],["145","Après 15 ans","58 264,42","4 369,83"]],"note":"Base 151,67 h/mois. Majoration de 10 % en région parisienne. Groupe 7 (cadres supérieurs) : voir art. 6-3 CCNA 4, sans montant chiffré."}]',
 'Accord du 11 octobre 2023 relatif à la revalorisation des rémunérations (annexe CCNA 4)',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000049067154/?idConteneur=KALICONT000005635624',
 'Étendu par arrêté du 19 décembre 2023',
 '2023-12-01', '2026-08-12', 13),

-- ============================================================
-- VOYAGEURS — avenants du 27 novembre 2025, effet 1er janvier 2026
-- ============================================================

('0016', 'voyageurs', 'ouvriers',
 'Personnel ouvrier — transport routier de voyageurs',
 'Contrairement à la branche marchandises, la grille voyageurs continue d''être négociée chaque année : + 1,3 % sur toute la grille au 1er janvier 2026. Les minima conventionnels restent donc au-dessus du SMIC, y compris au premier coefficient (1 884,80 € contre 1 867,02 € au SMIC mensuel) — c''est bien la convention qui gouverne, pas le plancher légal.

La grille donne deux chiffres par ligne : le taux horaire, à quatre décimales, et le salaire mensuel professionnel garanti (SMPG) pour 151,67 heures. Les colonnes d''ancienneté majorent l''un comme l''autre (art. 13 CCNA 1 et art. 26 al. 7 de l''accord du 18 avril 2002).

À ajouter le cas échéant :
• 3 % pour la qualification de mécanicien ou d''encaisseur (art. 13 b et c CCNA 1) ;
• 48,85 € par jour férié travaillé autre que le 1er mai, quel que soit le nombre d''heures ;
• 48,85 € par dimanche travaillé, quel que soit le nombre d''heures.

Ces deux dernières indemnités sont forfaitaires : elles ne se proratisent pas au temps de présence.',
 '[{"titre":"Taux horaires et salaires mensuels garantis pour 151,67 h (en €)","colonnes":["Groupe","Coefficient","Taux horaire","À l''embauche","Après 1 an","Après 5 ans","Après 10 ans","Après 15 ans","Après 20 ans","Après 25 ans","Après 30 ans"],"lignes":[["2","110 V","12,4270","1 884,80","1 922,50","1 997,89","2 035,58","2 073,28","2 148,67","2 205,22","2 261,76"],["3","115 V","12,4270","1 884,80","1 922,50","1 997,89","2 035,58","2 073,28","2 148,67","2 205,22","2 261,76"],["4","120 V","12,4270","1 884,80","1 922,50","1 997,89","2 035,58","2 073,28","2 148,67","2 205,22","2 261,76"],["5","123 V","12,4270","1 884,80","1 922,50","1 997,89","2 035,58","2 073,28","2 148,67","2 205,22","2 261,76"],["6","128 V","12,4270","1 884,80","1 922,50","1 997,89","2 035,58","2 073,28","2 148,67","2 205,22","2 261,76"],["7","131 V","12,6159","1 913,45","1 951,72","2 028,26","2 066,53","2 104,80","2 181,33","2 238,74","2 296,14"],["7","136 V","12,7241","1 929,86","1 968,46","2 045,65","2 084,25","2 122,85","2 200,04","2 257,94","2 315,83"],["7 bis","137 V","12,7647","1 936,02","1 974,74","2 052,18","2 090,90","2 129,62","2 207,06","2 265,14","2 323,22"],["8","138 V","13,1197","1 989,86","2 029,66","2 109,25","2 149,05","2 188,85","2 268,44","2 328,14","2 387,83"],["9","140 V","13,2156","2 004,41","2 044,50","2 124,67","2 164,76","2 204,85","2 285,03","2 345,16","2 405,29"],["9","142 V","13,3478","2 024,46","2 064,95","2 145,93","2 186,42","2 226,91","2 307,88","2 368,62","2 429,35"],["9 bis","145 V","13,4890","2 045,88","2 086,80","2 168,63","2 209,55","2 250,47","2 332,30","2 393,68","2 455,06"],["10","150 V","13,8167","2 095,58","2 137,49","2 221,31","2 263,23","2 305,14","2 388,96","2 451,83","2 514,70"],["10","155 V","14,5091","2 200,60","2 244,61","2 332,64","2 376,65","2 420,66","2 508,68","2 574,70","2 640,72"]],"note":"Les colonnes d''ancienneté majorent le taux horaire comme le SMPG conventionnels à l''embauche."},{"titre":"Majorations à ajouter le cas échéant (en €)","colonnes":["Situation","Montant"],"lignes":[["Qualification de mécanicien ou d''encaisseur","+ 3 %"],["Jour férié travaillé (hors 1er mai)","48,85"],["Dimanche travaillé","48,85"]],"note":"Forfaits par journée, quel que soit le nombre d''heures effectuées (art. 2.1 et 2.2 de l''avenant n° 114 du 19 mars 2021)."}]',
 'Avenant n° 120 du 27 novembre 2025 à la CCNA 1 (personnel ouvrier des entreprises de transport routier de voyageurs)',
 'https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000053788378',
 'Étendu par arrêté du 7 avril 2026 (JO du 10 avril 2026)',
 '2026-01-01', '2026-08-12', 20),

('0016', 'voyageurs', 'employes',
 'Personnel employé — transport routier de voyageurs',
 'Neuf coefficients, de 105 à 148,5, revalorisés de 1,3 % au 1er janvier 2026. Progression triennale : les colonnes d''ancienneté majorent le taux horaire et le SMPG conventionnels à l''embauche (art. 3 CCNA 2).

Trois indemnités mensuelles s''ajoutent pour des fonctions particulières : sténodactylographe et sténotypiste 45,69 €, traducteur 182,75 €, traducteur et rédacteur 274,13 €.

Comme pour les ouvriers de la branche, le travail d''un jour férié (autre que le 1er mai) ou d''un dimanche ouvre droit à 48,85 € forfaitaires, quel que soit le nombre d''heures effectuées.',
 '[{"titre":"Taux horaires et salaires mensuels garantis pour 151,67 h (en €)","colonnes":["Groupe","Coefficient","Taux horaire","À l''embauche","Après 3 ans","Après 6 ans","Après 9 ans","Après 12 ans","Après 15 ans","Après 20 ans","Après 25 ans","Après 30 ans"],"lignes":[["2","105","12,6622","1 920,48","1 978,09","2 035,71","2 093,32","2 150,94","2 208,55","2 246,96","2 275,77","2 304,58"],["3","110","12,6622","1 920,48","1 978,09","2 035,71","2 093,32","2 150,94","2 208,55","2 246,96","2 275,77","2 304,58"],["4","115","12,6658","1 921,02","1 978,65","2 036,28","2 093,91","2 151,54","2 209,17","2 247,59","2 276,41","2 305,22"],["5","120","12,6676","1 921,29","1 978,93","2 036,57","2 094,21","2 151,84","2 209,48","2 247,91","2 276,73","2 305,55"],["6","125","12,6689","1 921,49","1 979,13","2 036,78","2 094,42","2 152,07","2 209,71","2 248,14","2 276,97","2 305,79"],["7","132,5","12,7903","1 939,90","1 998,10","2 056,29","2 114,49","2 172,69","2 230,89","2 269,68","2 298,78","2 327,88"],["8","140","12,9055","1 957,38","2 016,10","2 074,82","2 133,54","2 192,27","2 250,99","2 290,13","2 319,50","2 348,86"],["9","148,5","13,6923","2 076,71","2 139,01","2 201,31","2 263,61","2 325,92","2 388,22","2 429,75","2 460,90","2 492,05"]],"note":"Les majorations pour ancienneté s''appliquent sur les taux horaires et les SMPG conventionnels à l''embauche (art. 3 CCNA 2)."},{"titre":"Indemnités mensuelles de fonction (en €)","colonnes":["Fonction","Montant"],"lignes":[["Sténodactylographe et sténotypiste","45,69"],["Traducteur","182,75"],["Traducteur et rédacteur","274,13"],["Jour férié travaillé (hors 1er mai)","48,85"],["Dimanche travaillé","48,85"]],"note":"Les deux dernières lignes sont des forfaits par journée (art. 3.1 et 3.2 de l''avenant n° 96 du 19 mars 2021)."}]',
 'Avenant n° 102 du 27 novembre 2025 à la CCNA 2 (personnel employé des entreprises de transport routier de voyageurs)',
 'https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000053788378',
 'Étendu par arrêté du 7 avril 2026 (JO du 10 avril 2026)',
 '2026-01-01', '2026-08-12', 21),

('0016', 'voyageurs', 'tam',
 'Techniciens et agents de maîtrise — transport routier de voyageurs',
 'Huit groupes, du coefficient 150 au coefficient 225, revalorisés de 1,3 % au 1er janvier 2026. Progression triennale jusqu''à trente ans d''ancienneté — c''est la particularité des grilles voyageurs, qui continuent de progresser bien après le plafond de quinze ans retenu en marchandises.

Les majorations d''ancienneté s''appliquent aux taux horaires comme aux SMPG conventionnels à l''embauche (art. 4 CCNA 3).

Indemnités mensuelles de fonction : traducteur 184,64 €, traducteur et rédacteur 276,96 €. Jour férié (hors 1er mai) ou dimanche travaillé : 48,85 € forfaitaires.',
 '[{"titre":"Taux horaires et salaires mensuels garantis pour 151,67 h (en €)","colonnes":["Groupe","Coefficient","Taux horaire","À l''embauche","Après 3 ans","Après 6 ans","Après 9 ans","Après 12 ans","Après 15 ans","Après 20 ans","Après 25 ans","Après 30 ans"],"lignes":[["1","150","13,8338","2 098,17","2 161,12","2 224,06","2 287,01","2 349,95","2 412,90","2 454,86","2 486,33","2 517,80"],["2","157,5","14,5219","2 202,54","2 268,62","2 334,69","2 400,77","2 466,84","2 532,92","2 576,97","2 610,01","2 643,05"],["3","165","15,2186","2 308,21","2 377,46","2 446,70","2 515,95","2 585,20","2 654,44","2 700,61","2 735,23","2 769,85"],["4","175","16,1469","2 449,00","2 522,47","2 595,94","2 669,41","2 742,88","2 816,35","2 865,33","2 902,07","2 938,80"],["5","185","17,0550","2 586,73","2 664,33","2 741,93","2 819,54","2 897,14","2 974,74","3 026,47","3 065,28","3 104,08"],["6","200","18,4367","2 796,29","2 880,18","2 964,07","3 047,96","3 131,84","3 215,73","3 271,66","3 313,60","3 355,55"],["7","215","19,8183","3 005,84","3 096,02","3 186,19","3 276,37","3 366,54","3 456,72","3 516,83","3 561,92","3 607,01"],["8","225","20,7463","3 146,59","3 240,99","3 335,39","3 429,78","3 524,18","3 618,58","3 681,51","3 728,71","3 775,91"]],"note":"Les majorations pour ancienneté s''appliquent sur les taux horaires et les SMPG conventionnels à l''embauche (art. 4 CCNA 3)."},{"titre":"Indemnités (en €)","colonnes":["Fonction ou situation","Montant"],"lignes":[["Traducteur","184,64"],["Traducteur et rédacteur","276,96"],["Jour férié travaillé (hors 1er mai)","48,85"],["Dimanche travaillé","48,85"]],"note":"Forfaits par journée pour les deux dernières lignes (art. 3.1 et 3.2 de l''avenant n° 94 du 19 mars 2021)."}]',
 'Avenant n° 100 du 27 novembre 2025 à la CCNA 3 (techniciens et agents de maîtrise des entreprises de transport routier de voyageurs)',
 'https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000053788378',
 'Étendu par arrêté du 7 avril 2026 (JO du 10 avril 2026)',
 '2026-01-01', '2026-08-12', 22),

('0016', 'voyageurs', 'cadres',
 'Ingénieurs et cadres — transport routier de voyageurs',
 'Six groupes chiffrés, du coefficient 100 au coefficient 145, plus un groupe 7 (cadres supérieurs) renvoyé à l''article 6-3 de l''annexe 4, sans montant.

Deux montants par ligne : la rémunération annuelle garantie (art. 5 al. 4 CCNA 4), contrôle annuel, et le paiement mensuel minimum (art. 6-4 al. 5), plancher mois par mois. La base est de 151,67 heures et inclut les éventuelles indemnités différentielles instituées par les lois de réduction du temps de travail.

L''ancienneté s''entend dans le groupe, non dans l''entreprise : sept tranches, jusqu''à trente ans.

Majoration de 10 % en région parisienne (art. 5 al. 2 CCNA 4). Jour férié travaillé (hors 1er mai) ou dimanche : 48,85 € forfaitaires.',
 '[{"titre":"Rémunérations annuelles garanties et paiements mensuels minimaux (en €)","colonnes":["Groupe","Coefficient","Ancienneté dans le groupe","Rémunération annuelle garantie","Paiement mensuel minimal"],"lignes":[["1","100","Jusqu''à 5 ans","38 884,61","2 916,35"],["1","100","5 à 10 ans","40 828,84","3 062,16"],["1","100","10 à 15 ans","42 773,07","3 207,98"],["1","100","15 à 20 ans","44 717,30","3 353,80"],["1","100","20 à 25 ans","45 494,99","3 412,12"],["1","100","25 à 30 ans","46 078,26","3 455,87"],["1","100","Après 30 ans","46 661,53","3 499,61"],["2","106,5","Jusqu''à 5 ans","41 412,53","3 105,94"],["2","106,5","5 à 10 ans","43 483,16","3 261,24"],["2","106,5","10 à 15 ans","45 553,78","3 416,53"],["2","106,5","15 à 20 ans","47 624,41","3 571,83"],["2","106,5","20 à 25 ans","48 452,66","3 633,95"],["2","106,5","25 à 30 ans","49 073,85","3 680,54"],["2","106,5","Après 30 ans","49 695,04","3 727,13"],["3","113","Jusqu''à 5 ans","43 939,44","3 295,46"],["3","113","5 à 10 ans","46 136,41","3 460,23"],["3","113","10 à 15 ans","48 333,38","3 625,00"],["3","113","15 à 20 ans","50 530,36","3 789,78"],["3","113","20 à 25 ans","51 409,14","3 855,69"],["3","113","25 à 30 ans","52 068,24","3 905,12"],["3","113","Après 30 ans","52 727,33","3 954,55"],["4","119","Jusqu''à 5 ans","46 271,81","3 470,39"],["4","119","5 à 10 ans","48 585,40","3 643,91"],["4","119","10 à 15 ans","50 898,99","3 817,42"],["4","119","15 à 20 ans","53 212,58","3 990,94"],["4","119","20 à 25 ans","54 138,02","4 060,35"],["4","119","25 à 30 ans","54 832,09","4 112,41"],["4","119","Après 30 ans","55 526,17","4 164,46"],["5","132","Jusqu''à 5 ans","51 327,09","3 849,53"],["5","132","5 à 10 ans","53 893,44","4 042,01"],["5","132","10 à 15 ans","56 459,80","4 234,49"],["5","132","15 à 20 ans","59 026,15","4 426,96"],["5","132","20 à 25 ans","60 052,70","4 503,95"],["5","132","25 à 30 ans","60 822,60","4 561,70"],["5","132","Après 30 ans","61 592,51","4 619,44"],["6","145","Jusqu''à 5 ans","56 382,10","4 228,66"],["6","145","5 à 10 ans","59 201,21","4 440,09"],["6","145","10 à 15 ans","62 020,31","4 651,52"],["6","145","15 à 20 ans","64 839,42","4 862,96"],["6","145","20 à 25 ans","65 967,06","4 947,53"],["6","145","25 à 30 ans","66 812,79","5 010,96"],["6","145","Après 30 ans","67 658,52","5 074,39"]],"note":"Base 151,67 h, indemnités différentielles RTT incluses. Le groupe 7 (cadres supérieurs) relève de l''article 6-3 de l''annexe 4, sans montant chiffré. Majoration de 10 % en région parisienne. Jour férié (hors 1er mai) ou dimanche travaillé : 48,85 €."}]',
 'Avenant n° 93 du 27 novembre 2025 à la CCNA 4 (ingénieurs et cadres des entreprises de transport routier de voyageurs)',
 'https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000053788378',
 'Étendu par arrêté du 7 avril 2026 (JO du 10 avril 2026)',
 '2026-01-01', '2026-08-12', 23),

-- ============================================================
-- DÉMÉNAGEMENT — avenant n° 24 du 21 janvier 2026
-- ============================================================

('0016', 'demenagement', 'ouvriers',
 'Personnel ouvrier — entreprises de déménagement',
 'Quatre coefficients (1 A à 1 D DEM), et surtout trois grilles superposées : DC 0, DC 1 et DC 2 majorent le taux de base selon la conduite effectuée. Ces majorations ne se cumulent pas — on retient celle qui correspond à la situation, pas leur somme.

Deux indemnités propres au déménagement figurent au même texte, et sont du salaire, non des frais : l''heure de dépassement d''amplitude et l''heure de temps de liaison, toutes deux à 7,65 € (accord du 22 septembre 2005).

Le travail d''un jour férié ou d''un dimanche est majoré de 13,13 € ou 30,54 € selon le cas (art. 7 et 7 quater CCNA 1).

Entrée en vigueur : le premier jour du mois suivant la parution de l''arrêté d''extension au Journal officiel, et au plus tard le 1er juin 2026. À la date de consultation, cette échéance est dépassée : la grille s''applique.',
 '[{"titre":"Taux horaires — base (en €)","colonnes":["Coefficient","À l''embauche","Après 2 ans","Après 5 ans","Après 10 ans","Après 15 ans"],"lignes":[["1 A DEM","12,03","12,27","12,51","12,75","12,99"],["1 B DEM","12,21","12,45","12,70","12,94","13,19"],["1 C DEM","12,68","12,93","13,19","13,44","13,69"],["1 D DEM","13,59","13,86","14,13","14,41","14,68"]],"note":"Le SMIC horaire est de 12,31 € au 1er juin 2026 : les coefficients 1 A et 1 B sont rattrapés à l''embauche, c''est alors le SMIC qui s''applique."},{"titre":"Taux horaires majorés « DC 0 » (en €)","colonnes":["Coefficient","À l''embauche","Après 2 ans","Après 5 ans","Après 10 ans","Après 15 ans"],"lignes":[["1 A DEM DC0","12,15","12,39","12,64","12,88","13,12"],["1 B DEM DC0","12,33","12,58","12,82","13,07","13,32"],["1 C DEM DC0","12,81","13,07","13,32","13,58","13,83"],["1 D DEM DC0","13,73","14,00","14,28","14,55","14,83"]],"note":"Les majorations DC 0, DC 1 et DC 2 ne se cumulent pas."},{"titre":"Taux horaires majorés « DC 1 » (en €)","colonnes":["Coefficient","À l''embauche","Après 2 ans","Après 5 ans","Après 10 ans","Après 15 ans"],"lignes":[["1 A DEM DC1","12,27","12,52","12,76","13,01","13,25"],["1 B DEM DC1","12,45","12,70","12,95","13,20","13,45"],["1 C DEM DC1","12,93","13,19","13,45","13,71","13,96"],["1 D DEM DC1","13,86","14,14","14,41","14,69","14,97"]],"note":null},{"titre":"Taux horaires majorés « DC 2 » (en €)","colonnes":["Coefficient","À l''embauche","Après 2 ans","Après 5 ans","Après 10 ans","Après 15 ans"],"lignes":[["1 A DEM DC2","12,39","12,64","12,89","13,13","13,38"],["1 B DEM DC2","12,58","12,83","13,08","13,33","13,59"],["1 C DEM DC2","13,06","13,32","13,58","13,84","14,10"],["1 D DEM DC2","14,00","14,28","14,56","14,84","15,12"]],"note":null},{"titre":"Éléments annexes (en €)","colonnes":["Élément","Montant"],"lignes":[["Heure de dépassement d''amplitude","7,65"],["Heure de temps de liaison","7,65"],["Jour férié ou dimanche travaillé (art. 7 / 7 quater CCNA 1)","13,13 ou 30,54"]],"note":"Les heures de dépassement d''amplitude et de temps de liaison sont du salaire : elles entrent dans l''assiette des cotisations et des congés payés."}]',
 'Avenant n° 24 du 21 janvier 2026 à l''accord du 1er février 2003 sur les rémunérations conventionnelles dans les entreprises de transport de déménagement',
 NULL,
 'Extension demandée ; application au plus tard le 1er juin 2026',
 '2026-06-01', '2026-08-12', 30),

('0016', 'demenagement', 'employes',
 'Personnel employé — entreprises de déménagement',
 'Quatre coefficients (2 A à 2 D DEM) et une progression triennale classique : + 3 % par palier jusqu''à quinze ans.

Le texte remplace intégralement les tableaux précédents de l''avenant n° 23 du 27 février 2025 : il n''y a pas de superposition à faire, la grille ci-dessous est la seule applicable.

Rappel utile du même avenant : les salariés sous contrat journalier ou saisonnier — courants dans le déménagement — relèvent des mêmes grilles de classification et du taux horaire qui en découle. Un contrat court ne justifie pas un positionnement plus bas.',
 '[{"titre":"Taux horaires (en €)","colonnes":["Coefficient","À l''embauche","Après 3 ans","Après 6 ans","Après 9 ans","Après 12 ans","Après 15 ans"],"lignes":[["2 A DEM","12,03","12,39","12,75","13,11","13,47","13,83"],["2 B DEM","12,11","12,47","12,84","13,20","13,56","13,93"],["2 C DEM","12,44","12,81","13,19","13,56","13,93","14,31"],["2 D DEM","12,85","13,24","13,62","14,01","14,39","14,78"]],"note":"Le SMIC horaire est de 12,31 € au 1er juin 2026 : les coefficients 2 A et 2 B sont rattrapés à l''embauche."}]',
 'Avenant n° 24 du 21 janvier 2026 à l''accord du 1er février 2003 sur les rémunérations conventionnelles dans les entreprises de transport de déménagement',
 NULL,
 'Extension demandée ; application au plus tard le 1er juin 2026',
 '2026-06-01', '2026-08-12', 31),

('0016', 'demenagement', 'tam',
 'Techniciens et agents de maîtrise — entreprises de déménagement',
 'Quatre coefficients (3 A à 3 D DEM), progression triennale de + 3 % par palier jusqu''à quinze ans.

L''écart entre le premier et le dernier coefficient est ici considérable — de 13,44 € à 18,29 € à l''embauche, soit 36 % — ce qui fait du positionnement au coefficient le vrai enjeu de paie de cette catégorie, bien avant l''ancienneté.',
 '[{"titre":"Taux horaires (en €)","colonnes":["Coefficient","À l''embauche","Après 3 ans","Après 6 ans","Après 9 ans","Après 12 ans","Après 15 ans"],"lignes":[["3 A DEM","13,44","13,84","14,25","14,65","15,05","15,46"],["3 B DEM","14,19","14,62","15,04","15,47","15,89","16,32"],["3 C DEM","16,23","16,72","17,20","17,69","18,18","18,66"],["3 D DEM","18,29","18,84","19,39","19,94","20,48","21,03"]],"note":null}]',
 'Avenant n° 24 du 21 janvier 2026 à l''accord du 1er février 2003 sur les rémunérations conventionnelles dans les entreprises de transport de déménagement',
 NULL,
 'Extension demandée ; application au plus tard le 1er juin 2026',
 '2026-06-01', '2026-08-12', 32),

('0016', 'demenagement', 'cadres',
 'Ingénieurs et cadres — entreprises de déménagement',
 'Trois coefficients (4 A à 4 C DEM), quatre tranches d''ancienneté dans le groupe (art. 5 al. 4 CCNA 4).

Particularité de la branche : la rémunération annuelle garantie est établie pour 169 heures, et non pour 151,67 heures comme ailleurs dans la convention. Comparer ces montants à ceux du transport de marchandises sans corriger la base fausse la comparaison de plus de 11 %.',
 '[{"titre":"Rémunérations annuelles garanties pour 169 h (en €)","colonnes":["Coefficient","Ancienneté dans le groupe","Rémunération annuelle garantie","Paiement mensuel minimum"],"lignes":[["4 A DEM","Jusqu''à 5 ans","41 578,56","3 118,39"],["4 A DEM","Après 5 ans","43 657,49","3 274,31"],["4 A DEM","Après 10 ans","45 736,42","3 430,23"],["4 A DEM","Après 15 ans","47 815,34","3 586,15"],["4 B DEM","Jusqu''à 5 ans","46 458,07","3 484,36"],["4 B DEM","Après 5 ans","48 780,97","3 658,57"],["4 B DEM","Après 10 ans","51 103,88","3 832,79"],["4 B DEM","Après 15 ans","53 426,78","4 007,01"],["4 C DEM","Jusqu''à 5 ans","56 608,91","4 245,67"],["4 C DEM","Après 5 ans","59 439,36","4 457,95"],["4 C DEM","Après 10 ans","62 269,80","4 670,24"],["4 C DEM","Après 15 ans","65 100,25","4 882,52"]],"note":"Base 169 h/mois, propre au déménagement. L''ancienneté s''apprécie dans le groupe (art. 5 al. 4 CCNA 4)."}]',
 'Avenant n° 24 du 21 janvier 2026 à l''accord du 1er février 2003 sur les rémunérations conventionnelles dans les entreprises de transport de déménagement',
 NULL,
 'Extension demandée ; application au plus tard le 1er juin 2026',
 '2026-06-01', '2026-08-12', 33),

-- ============================================================
-- PRESTATIONS LOGISTIQUES — avenant n° 16 du 9 avril 2025
-- ============================================================

('0016', 'logistique', 'ouvriers',
 'Personnel ouvrier — prestations logistiques',
 'La grille logistique est la seule de la convention à nommer les emplois plutôt qu''à s''en tenir aux coefficients : préparateur de commandes, cariste, opérateur de ligne. C''est le libellé de l''emploi qui commande le coefficient, pas l''intitulé du contrat.

Elle comporte un palier propre à la branche, « après 6 mois », qui n''existe nulle part ailleurs dans l''IDCC 16, puis une progression à 2, 5, 10 et 15 ans.

Deux barèmes : taux horaires et garanties annuelles de rémunération pour 151,67 heures mensuelles. La GAR se contrôle sur l''année civile.

À la date de consultation, tous les taux d''embauche de cette grille sont inférieurs au SMIC (12,31 €/h) : c''est le SMIC qui s''applique à l''entrée, la grille ne reprend la main qu''à partir du palier des 2 ans.',
 '[{"titre":"Taux horaires (en €)","colonnes":["Coefficient","Emploi","À l''embauche","Après 6 mois","Après 2 ans","Après 5 ans","Après 10 ans","Après 15 ans"],"lignes":[["110 L","Opérateur / emballeur","11,91","11,96","12,1992","12,4384","12,6776","12,9168"],["110 L","Manutentionnaire logistique","11,91","11,96","12,1992","12,4384","12,6776","12,9168"],["115 L","Préparateur de commandes","11,91","12,05","12,2910","12,5320","12,7730","13,0140"],["115 L","Agent logistique","11,91","12,05","12,2910","12,5320","12,7730","13,0140"],["120 L","Contrôleur / flasheur","11,92","12,12","12,3624","12,6048","12,8472","13,0896"],["120 L","Agent de maintenance d''entrepôt logistique","11,92","12,12","12,3624","12,6048","12,8472","13,0896"],["125 L","Cariste en prestation logistique","11,95","12,18","12,4236","12,6672","12,9108","13,1544"],["138 L","Opérateur de ligne","11,98","12,26","12,5052","12,7504","12,9956","13,2408"]],"note":"Taux inférieurs au SMIC à l''embauche et après 6 mois : le SMIC (12,31 €/h au 1er juin 2026) prime."},{"titre":"Garanties annuelles de rémunération — 151,67 h/mois (en €)","colonnes":["Coefficient","Emploi","À l''embauche","Après 2 ans","Après 5 ans","Après 10 ans","Après 15 ans"],"lignes":[["110 L","Opérateur / emballeur","22 916,93","23 375,27","23 833,61","24 291,95","24 750,28"],["110 L","Manutentionnaire logistique","22 916,93","23 375,27","23 833,61","24 291,95","24 750,28"],["115 L","Préparateur de commandes","23 112,45","23 574,70","24 036,95","24 499,20","24 961,45"],["115 L","Agent logistique","23 112,45","23 574,70","24 036,95","24 499,20","24 961,45"],["120 L","Contrôleur / flasheur","23 210,86","23 675,08","24 139,29","24 603,51","25 067,73"],["120 L","Agent de maintenance d''entrepôt logistique","23 210,86","23 675,08","24 139,29","24 603,51","25 067,73"],["125 L","Cariste en prestation logistique","23 367,84","23 835,20","24 302,55","24 769,91","25 237,27"],["138 L","Opérateur de ligne","23 463,13","23 932,39","24 401,66","24 870,92","25 340,18"]],"note":"Contrôle annuel, sur l''année civile complète."}]',
 'Avenant n° 16 du 9 avril 2025 relatif à la revalorisation des minima conventionnels (prestations logistiques, annexe 1)',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000051927426/?idConteneur=KALICONT000005635624',
 NULL,
 '2025-05-01', '2026-08-12', 40),

('0016', 'logistique', 'employes',
 'Personnel employé — prestations logistiques',
 'Deux coefficients seulement (110 L et 120 L), trois emplois nommés. Le palier « après 6 mois » précède ici une progression triennale de + 3 % par palier, jusqu''à quinze ans.

La grille est courte parce que la classification employée de la logistique l''est : l''essentiel des effectifs administratifs d''un site logistique relève en pratique de la catégorie technicien ou agent de maîtrise dès qu''une responsabilité d''encadrement apparaît.',
 '[{"titre":"Taux horaires (en €)","colonnes":["Coefficient","Emploi","À l''embauche","Après 6 mois","Après 3 ans","Après 6 ans","Après 9 ans","Après 12 ans","Après 15 ans"],"lignes":[["110 L","Assistant inventaire","11,98","12,18","12,5454","12,9108","13,2762","13,6416","14,0070"],["120 L","Employé d''ordonnancement","12,03","12,26","12,6278","12,9956","13,3634","13,7312","14,0990"],["120 L","Agent administratif logistique","12,03","12,26","12,6278","12,9956","13,3634","13,7312","14,0990"]],"note":"Taux d''embauche inférieurs au SMIC (12,31 €/h au 1er juin 2026) : le SMIC prime jusqu''au palier des 3 ans."},{"titre":"Garanties annuelles de rémunération — 151,67 h/mois (en €)","colonnes":["Coefficient","Emploi","À l''embauche","Après 3 ans","Après 6 ans","Après 9 ans","Après 12 ans","Après 15 ans"],"lignes":[["110 L","Assistant inventaire","23 326,58","24 026,38","24 726,17","25 425,97","26 125,77","26 825,57"],["120 L","Employé d''ordonnancement","23 463,48","24 167,38","24 871,29","25 575,19","26 279,10","26 983,00"],["120 L","Agent administratif logistique","23 463,48","24 167,38","24 871,29","25 575,19","26 279,10","26 983,00"]],"note":null}]',
 'Avenant n° 16 du 9 avril 2025 relatif à la revalorisation des minima conventionnels (prestations logistiques, annexe 2)',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000051927426/?idConteneur=KALICONT000005635624',
 NULL,
 '2025-05-01', '2026-08-12', 41),

('0016', 'logistique', 'tam',
 'Techniciens et agents de maîtrise — prestations logistiques',
 'Quatre coefficients (150 L à 200 L), dix emplois nommés. Progression triennale de + 3 % par palier jusqu''à quinze ans.

C''est la catégorie où le nom de l''emploi pèse le plus lourd : chef d''équipe, gestionnaire de stocks et superviseur de lignes partagent le même coefficient 157,5 L, tandis que le chef de quai passe à 165 L et le chef d''exploitation à 200 L. Un intitulé de poste imprécis dans le contrat, et c''est un écart de 3,12 €/h qui se discute devant le conseil de prud''hommes.',
 '[{"titre":"Taux horaires (en €)","colonnes":["Coefficient","Emploi","À l''embauche","Après 3 ans","Après 6 ans","Après 9 ans","Après 12 ans","Après 15 ans"],"lignes":[["150 L","Technicien de maintenance d''entrepôt logistique","13,54","13,9462","14,3524","14,7586","15,1648","15,5710"],["157,5 L","Chef d''équipe logistique","13,64","14,0492","14,4584","14,8676","15,2768","15,6860"],["157,5 L","Gestionnaire de stocks","13,64","14,0492","14,4584","14,8676","15,2768","15,6860"],["157,5 L","Correspondant du responsable management de la qualité","13,64","14,0492","14,4584","14,8676","15,2768","15,6860"],["157,5 L","Responsable ou superviseur de lignes","13,64","14,0492","14,4584","14,8676","15,2768","15,6860"],["165 L","Chef de quai logistique","14,15","14,5745","14,9990","15,4235","15,8480","16,2725"],["200 L","Chef d''exploitation logistique","16,76","17,2628","17,7656","18,2684","18,7712","19,2740"],["200 L","Responsable maintenance d''entrepôt logistique","16,76","17,2628","17,7656","18,2684","18,7712","19,2740"],["200 L","Responsable service client logistique","16,76","17,2628","17,7656","18,2684","18,7712","19,2740"],["200 L","Responsable conditionnement à façon","16,76","17,2628","17,7656","18,2684","18,7712","19,2740"]],"note":null},{"titre":"Garanties annuelles de rémunération — 151,67 h/mois (en €)","colonnes":["Coefficient","Emploi","À l''embauche","Après 3 ans","Après 6 ans","Après 9 ans","Après 12 ans","Après 15 ans"],"lignes":[["150 L","Technicien de maintenance d''entrepôt logistique","26 227,38","27 014,20","27 801,02","28 587,84","29 374,67","30 161,49"],["157,5 L","Chef d''équipe logistique","26 391,83","27 183,58","27 975,34","28 767,09","29 558,85","30 350,60"],["157,5 L","Gestionnaire de stocks","26 391,83","27 183,58","27 975,34","28 767,09","29 558,85","30 350,60"],["157,5 L","Correspondant du responsable management de la qualité","26 391,83","27 183,58","27 975,34","28 767,09","29 558,85","30 350,60"],["157,5 L","Responsable ou superviseur de lignes","26 391,83","27 183,58","27 975,34","28 767,09","29 558,85","30 350,60"],["165 L","Chef de quai logistique","27 405,83","28 228,00","29 050,18","29 872,35","30 694,53","31 516,70"],["200 L","Chef d''exploitation logistique","32 316,30","33 285,79","34 255,28","35 224,77","36 194,26","37 163,75"],["200 L","Responsable maintenance d''entrepôt logistique","32 316,30","33 285,79","34 255,28","35 224,77","36 194,26","37 163,75"],["200 L","Responsable service client logistique","32 316,30","33 285,79","34 255,28","35 224,77","36 194,26","37 163,75"],["200 L","Responsable conditionnement à façon","32 316,30","33 285,79","34 255,28","35 224,77","36 194,26","37 163,75"]],"note":null}]',
 'Avenant n° 16 du 9 avril 2025 relatif à la revalorisation des minima conventionnels (prestations logistiques, annexe 3)',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000051927426/?idConteneur=KALICONT000005635624',
 NULL,
 '2025-05-01', '2026-08-12', 42),

('0016', 'logistique', 'cadres',
 'Ingénieurs et cadres — prestations logistiques',
 'Six coefficients (100 L à 132 L), huit emplois nommés, quatre tranches d''ancienneté. Rémunération annuelle garantie pour 151,67 heures mensuelles, assortie d''un paiement mensuel minimum.

La progression d''ancienneté est de 5 % par tranche de cinq ans, appliquée à la rémunération d''entrée du coefficient. Le paiement mensuel minimum n''est pas le douzième de la RAG : il vaut la RAG divisée par 13,33, ce qui laisse la place à une part variable ou à un treizième mois — le contrôle mensuel est donc plus souple que le contrôle annuel.',
 '[{"titre":"Rémunérations annuelles garanties — 151,67 h/mois (en €)","colonnes":["Coefficient","Emploi","Ancienneté","Rémunération annuelle garantie","Paiement mensuel minimum"],"lignes":[["100 L","Responsable management de la qualité","À l''embauche","40 547,22","3 041,04"],["100 L","Responsable management de la qualité","Après 5 ans","42 574,58","3 193,09"],["100 L","Responsable management de la qualité","Après 10 ans","44 601,94","3 345,15"],["100 L","Responsable management de la qualité","Après 15 ans","46 629,30","3 497,20"],["106,5 L","Chef de projet","À l''embauche","43 192,88","3 239,47"],["106,5 L","Chef de projet","Après 5 ans","45 352,52","3 401,44"],["106,5 L","Chef de projet","Après 10 ans","47 512,17","3 563,41"],["106,5 L","Chef de projet","Après 15 ans","49 671,81","3 725,39"],["106,5 L","Responsable sécurité","À l''embauche","43 192,88","3 239,47"],["106,5 L","Responsable sécurité","Après 15 ans","49 671,81","3 725,39"],["113 L","Directeur d''exploitation logistique","À l''embauche","45 818,94","3 436,42"],["113 L","Directeur d''exploitation logistique","Après 5 ans","48 109,89","3 608,24"],["113 L","Directeur d''exploitation logistique","Après 10 ans","50 400,83","3 780,06"],["113 L","Directeur d''exploitation logistique","Après 15 ans","52 691,78","3 951,88"],["113 L","Directeur méthode logistique","À l''embauche","45 818,94","3 436,42"],["113 L","Directeur méthode logistique","Après 15 ans","52 691,78","3 951,88"],["119 L","Directeur conditionnement à façon","À l''embauche","47 915,88","3 593,69"],["119 L","Directeur conditionnement à façon","Après 5 ans","50 311,67","3 773,38"],["119 L","Directeur conditionnement à façon","Après 10 ans","52 707,47","3 953,06"],["119 L","Directeur conditionnement à façon","Après 15 ans","55 103,26","4 132,74"],["119 L","Directeur de site logistique","À l''embauche","47 915,88","3 593,69"],["119 L","Directeur de site logistique","Après 15 ans","55 103,26","4 132,74"],["132 L","Directeur de sites logistiques","À l''embauche","53 543,94","4 015,80"],["132 L","Directeur de sites logistiques","Après 5 ans","56 221,14","4 216,59"],["132 L","Directeur de sites logistiques","Après 10 ans","58 898,33","4 417,37"],["132 L","Directeur de sites logistiques","Après 15 ans","61 575,53","4 618,16"]],"note":"Les emplois partageant un coefficient partagent la grille complète : seules les bornes sont reproduites pour les doublons (responsable sécurité, directeur méthode, directeur de site)."}]',
 'Avenant n° 16 du 9 avril 2025 relatif à la revalorisation des minima conventionnels (prestations logistiques, annexe 4)',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000051927426/?idConteneur=KALICONT000005635624',
 NULL,
 '2025-05-01', '2026-08-12', 43),

-- ============================================================
-- TRANSPORT SANITAIRE — avenant n° 8 du 6 mai 2025
-- ============================================================

('0016', 'sanitaire', 'ouvriers',
 'Personnels ambulanciers — entreprises de transport sanitaire',
 'La branche sanitaire ne raisonne pas en coefficients mais en niveaux d''emploi, et l''avenant de salaires ne fixe qu''une chose : le taux horaire garanti à l''embauche de chacun des trois niveaux. La progression d''ancienneté relève de l''accord-cadre du 4 mai 2000 modifié, non reproduit ici.

Le point qui compte en paie : à la date de consultation, les niveaux 1 et 2 sont sous le SMIC (11,89 € et 11,90 € contre 12,31 €). Le taux conventionnel d''embauche est donc inapplicable tel quel — c''est le SMIC qui fixe le plancher, et l''écart de 0,42 €/h se retrouve sur toute la paie.

L''indemnité pour travail du dimanche et des jours fériés (art. 12-6 de l''accord-cadre) est portée à 23,90 €. Elle est du salaire, pas un remboursement de frais : elle entre dans l''assiette des cotisations et dans celle des congés payés.

Le temps de travail des ambulanciers relève par ailleurs d''un régime d''équivalence propre, qui ne figure pas dans cet avenant.',
 '[{"titre":"Taux horaires garantis à l''embauche (en €)","colonnes":["Emploi","Taux horaire"],"lignes":[["Ambulancier niveau 1","11,89"],["Ambulancier niveau 2","11,90"],["Ambulancier niveau 3","12,79"]],"note":"SMIC horaire au 1er juin 2026 : 12,31 €. Les niveaux 1 et 2 sont donc rattrapés par le SMIC."},{"titre":"Indemnités (en €)","colonnes":["Situation","Montant"],"lignes":[["Travail un dimanche ou un jour férié","23,90"]],"note":"Art. 12-6 de l''accord-cadre du 4 mai 2000 modifié. Élément de salaire soumis à cotisations."}]',
 'Avenant n° 8 du 6 mai 2025 à l''accord du 16 février 2004 relatif aux rémunérations conventionnelles des personnels ambulanciers',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000052051824/?idConteneur=KALICONT000005635624',
 'Étendu par arrêté du 22 juillet 2025',
 '2025-06-01', '2026-08-12', 50);

-- ============================================================
-- MAINTIEN DE SALAIRE EN CAS D'ABSENCE MALADIE
-- Annexes I à IV — texte de base de la convention
-- ============================================================

INSERT INTO ccn_maintien
  (idcc, categorie, intitule, article, corps, tableaux, source, source_url, consulte_le, ordre)
VALUES

('0016', 'ouvriers',
 'Maintien de salaire — personnel ouvrier',
 'Annexe I, article 10 ter',
 'Le droit conventionnel s''ouvre à trois ans d''ancienneté en maladie, à un an en accident du travail. En dessous, c''est la mensualisation légale qui s''applique, pas cette grille.

Délai de franchise en maladie : 5 jours. L''indemnisation démarre donc au 6e jour d''absence. Il est ramené à 3 jours en cas d''hospitalisation dans le transport routier de marchandises.

En accident du travail et maladie professionnelle : aucune franchise, l''indemnisation part du premier jour. Le droit à un an d''ancienneté suppose toutefois une hospitalisation d''au moins trois jours ou une incapacité d''au moins vingt-huit jours.

Assiette : la rémunération que le salarié aurait perçue en continuant de travailler, part variable comprise dès lors qu''elle est la contrepartie du travail. En sont exclus les remboursements de frais professionnels — pour un roulant, cela retire les indemnités de repas et de découcher de l''assiette du maintien.

Déduction : les indemnités journalières de sécurité sociale viennent en déduction de ce que verse l''employeur, ainsi que les prestations de prévoyance pour leur seule part financée par l''employeur. Le salarié doit déclarer les IJSS perçues.

Plafond : le total perçu, toutes provenances confondues, ne peut dépasser la rémunération nette que le salarié aurait perçue en travaillant.

Comptage : la durée totale d''indemnisation sur douze mois consécutifs ne peut excéder les durées ci-dessous. Une reprise effective du travail est nécessaire pour rouvrir un droit après une incapacité de longue durée. En cas d''hospitalisation, les périodes à 75 % sont prolongées de 30 jours.',
 '[{"titre":"Absence pour maladie — franchise de 5 jours","colonnes":["Ancienneté","À 100 %","À 75 %"],"lignes":[["3 ans","Du 6e au 40e jour","Du 41e au 70e jour"],["5 ans","Du 6e au 70e jour","Du 71e au 130e jour"],["10 ans","Du 6e au 100e jour","Du 101e au 190e jour"]],"note":"Franchise ramenée à 3 jours en cas d''hospitalisation dans le transport routier de marchandises. Hospitalisation : + 30 jours sur la période à 75 %."},{"titre":"Absence pour accident du travail — sans franchise","colonnes":["Ancienneté","À 100 %","À 75 %"],"lignes":[["1 an","Du 1er au 30e jour","Du 31e au 90e jour"],["3 ans","Du 1er au 30e jour","Du 31e au 90e jour"],["5 ans","Du 1er au 60e jour","Du 61e au 150e jour"],["10 ans","Du 1er au 90e jour","Du 91e au 210e jour"]],"note":"Le droit à 1 an d''ancienneté suppose une hospitalisation d''au moins 3 jours ou une incapacité d''au moins 28 jours."}]',
 'Convention collective nationale des transports routiers, annexe I (accord du 16 juin 1961 relatif aux ouvriers), article 10 ter — version en vigueur étendue',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALISCTA000005723162',
 '2026-08-12', 1),

('0016', 'employes',
 'Maintien de salaire — personnel employé',
 'Annexe II, article 17 bis',
 'Le régime des employés est calqué sur celui des ouvriers : mêmes conditions d''ancienneté, même franchise de 5 jours en maladie, mêmes durées d''indemnisation.

L''ancienneté s''apprécie au premier jour de l''absence. Le droit s''ouvre à trois ans en maladie, à un an en accident du travail sous condition de gravité (hospitalisation d''au moins trois jours ou incapacité d''au moins vingt-huit jours).

Les indemnités versées par l''employeur sont réduites des indemnités journalières de sécurité sociale et des prestations de prévoyance pour leur part patronale. Le total ne peut dépasser la rémunération nette qui aurait été perçue en travaillant.

La durée totale d''indemnisation par période de douze mois consécutifs ne peut excéder les durées du tableau. En cas d''hospitalisation, la période à 75 % est prolongée de 30 jours.',
 '[{"titre":"Absence pour maladie — franchise de 5 jours","colonnes":["Ancienneté","À 100 %","À 75 %"],"lignes":[["3 ans","Du 6e au 40e jour","Du 41e au 70e jour"],["5 ans","Du 6e au 70e jour","Du 71e au 130e jour"],["10 ans","Du 6e au 100e jour","Du 101e au 190e jour"]],"note":"Hospitalisation : + 30 jours sur la période à 75 %."},{"titre":"Absence pour accident du travail — sans franchise","colonnes":["Ancienneté","À 100 %","À 75 %"],"lignes":[["1 an","Du 1er au 30e jour","Du 31e au 90e jour"],["5 ans","Du 1er au 60e jour","Du 61e au 150e jour"],["10 ans","Du 1er au 90e jour","Du 91e au 210e jour"]],"note":"Le droit à 1 an d''ancienneté suppose une hospitalisation d''au moins 3 jours ou une incapacité d''au moins 28 jours."}]',
 'Convention collective nationale des transports routiers, annexe II (accord du 27 février 1951 relatif aux employés), article 17 bis — version en vigueur étendue',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALIARTI000005849520/?idConteneur=KALICONT000005635624',
 '2026-08-12', 2),

('0016', 'tam',
 'Maintien de salaire — techniciens et agents de maîtrise',
 'Annexe III, article 21 bis',
 'Rupture nette avec les ouvriers et les employés : aucun délai de franchise. L''indemnisation démarre au premier jour d''absence, y compris en maladie simple. Sur un arrêt court — le plus fréquent — c''est tout l''écart entre les deux régimes.

Second particularisme : la grille se dédouble selon le groupe de classification. Les groupes 6 à 8, qui rassemblent les agents de maîtrise de haut niveau, bénéficient de durées doublées par rapport aux groupes 1 à 5 à ancienneté égale.

Comme pour les autres catégories, les indemnités versées par l''employeur sont réduites des indemnités journalières de sécurité sociale, et le total perçu ne peut excéder la rémunération nette qui aurait été perçue en travaillant. En cas d''hospitalisation, la période à taux réduit est prolongée de 30 jours.

L''article traite également l''accident du travail : le droit s''ouvre dès un an d''ancienneté, sans franchise, sous condition d''hospitalisation d''au moins trois jours ou d''incapacité d''au moins vingt-huit jours, sur des durées supérieures à celles de la maladie. Ces durées ne sont pas reproduites ici : elles n''ont pas été relevées ligne à ligne dans le texte, et un tableau à moitié sûr ne vaut pas mieux que pas de tableau.',
 '[{"titre":"Absence pour maladie — groupes 1 à 5, sans franchise","colonnes":["Ancienneté","À 100 %","À 75 %"],"lignes":[["3 ans","Du 1er au 30e jour","Du 31e au 60e jour"],["5 ans","Du 1er au 60e jour","Du 61e au 120e jour"],["10 ans","Du 1er au 90e jour","Du 91e au 180e jour"]],"note":"Hospitalisation : + 30 jours sur la période à 75 %."},{"titre":"Absence pour maladie — groupes 6 à 8, sans franchise","colonnes":["Ancienneté","À 100 %","À 75 %"],"lignes":[["3 ans","Du 1er au 60e jour","Du 61e au 120e jour"],["5 ans","Du 1er au 90e jour","Du 91e au 180e jour"],["10 ans","Du 1er au 120e jour","Du 121e au 240e jour"]],"note":"Agents de maîtrise et techniciens de haut niveau : durées doublées par rapport aux groupes 1 à 5."}]',
 'Convention collective nationale des transports routiers, annexe III (accord du 30 mars 1951 relatif aux techniciens et agents de maîtrise), article 21 bis — version en vigueur étendue',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALISCTA000005723112?idConteneur=KALICONT000005635624',
 '2026-08-12', 3),

('0016', 'cadres',
 'Maintien de salaire — ingénieurs et cadres',
 'Annexe IV, article 21 bis',
 'Le régime le plus protecteur de la convention : aucune franchise, et des durées qui atteignent quatre mois à plein tarif après dix ans d''ancienneté.

À trois ans, un cadre est maintenu à 100 % pendant deux mois là où un ouvrier l''est pendant 35 jours après une franchise de 5 jours. L''écart de coût pour l''employeur, sur un arrêt d''un mois, est intégral : il paie tout, dès le premier jour.

Les indemnités versées par l''employeur sont réduites des indemnités journalières de sécurité sociale et des prestations de prévoyance pour leur part patronale ; le total ne peut excéder la rémunération nette qui aurait été perçue en travaillant. La durée totale par période de douze mois consécutifs ne peut excéder les durées du tableau. En cas d''hospitalisation, la période à 75 % est prolongée de 30 jours.',
 '[{"titre":"Absence pour maladie — sans franchise","colonnes":["Ancienneté","À 100 %","À 75 %"],"lignes":[["3 ans","Du 1er au 60e jour","Du 61e au 120e jour"],["5 ans","Du 1er au 90e jour","Du 91e au 180e jour"],["10 ans","Du 1er au 120e jour","Du 121e au 240e jour"]],"note":"Hospitalisation : + 30 jours sur la période à 75 %."},{"titre":"Absence pour accident du travail — sans franchise","colonnes":["Ancienneté","À 100 %","À 75 %"],"lignes":[["1 an","Du 1er au 60e jour","Du 61e au 150e jour"],["3 ans","Du 1er au 60e jour","Du 61e au 150e jour"],["5 ans","Du 1er au 90e jour","Du 91e au 210e jour"],["10 ans","Du 1er au 120e jour","Du 121e au 270e jour"]],"note":"Le droit à 1 an d''ancienneté suppose une hospitalisation d''au moins 3 jours ou une incapacité d''au moins 28 jours."}]',
 'Convention collective nationale des transports routiers, annexe IV (accord du 30 octobre 1951 relatif aux ingénieurs et cadres), article 21 bis — version en vigueur étendue',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALIARTI000005849581/?idConteneur=KALICONT000005635624',
 '2026-08-12', 4);
