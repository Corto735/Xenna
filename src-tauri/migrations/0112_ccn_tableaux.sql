-- ============================================================
-- CONVENTIONS COLLECTIVES — Barèmes chiffrés
--
-- La 0111 ne savait afficher que de la prose. Or l'essentiel de
-- ce qu'un gestionnaire vient chercher dans une convention est
-- tabulaire : une grille de minima, un barème d'indemnités, une
-- progression d'ancienneté. On ajoute donc une colonne `tableaux`
-- portant un JSON, et on y verse les grilles des six branches.
--
-- Forme du JSON : liste de tableaux, chacun
--   { "titre": "...", "colonnes": [...], "lignes": [[...], ...],
--     "note": "..." }
-- La première colonne est rendue à gauche, les suivantes à droite
-- (ce sont des montants dans tous les cas rencontrés).
--
-- SOURCES ET FRAÎCHEUR — à lire avant de faire confiance :
--  • Prestations logistiques : avenant n° 16 du 9 avril 2025,
--    texte intégral consulté sur Légifrance. Fiable.
--  • Déménagement : avenant n° 24 du 21 janvier 2026, effet
--    1er juin 2026.
--  • Transport sanitaire : avenant n° 8 du 6 mai 2025, effet
--    1er juin 2025.
--  • Voyageurs : avenant n° 120 du 27 novembre 2025, étendu par
--    arrêté du 7 avril 2026 (extension vérifiée sur Légifrance).
--  • Marchandises : accord du 11 octobre 2023, étendu le
--    19 décembre 2023. ATTENTION : les sources divergent sur une
--    revalorisation au 1er janvier 2026 — voir la règle dédiée.
--  • Frais de déplacement : avenant n° 81 du 2 décembre 2025,
--    étendu par arrêté du 3 février 2026.
--
-- Tout reste en statut 'a_verifier' : ces grilles ont été
-- reconstituées à partir de sources secondaires pour la plupart,
-- et une grille de salaires se périme en un an.
-- ============================================================

ALTER TABLE ccn_reglementations ADD COLUMN tableaux TEXT;

-- ── Ancienneté : le barème existant mérite son tableau ───────
UPDATE ccn_reglementations
   SET tableaux = '[{"titre":"Majoration du salaire minimum du coefficient","colonnes":["Ancienneté dans l''entreprise","Majoration"],"lignes":[["2 ans","2 %"],["5 ans","4 %"],["10 ans","6 %"],["15 ans","8 %"]],"note":"Assiette : le taux horaire conventionnel garanti du coefficient, pas le salaire réellement pratiqué s''il est supérieur."}]',
       maj_le = datetime('now')
 WHERE idcc = '0016' AND theme = 'anciennete' AND titre LIKE 'Prime d''ancienneté — ouvriers%';

UPDATE ccn_reglementations
   SET tableaux = '[{"titre":"Majoration du salaire minimum du coefficient","colonnes":["Ancienneté dans l''entreprise","Majoration"],"lignes":[["3 ans","3 %"],["6 ans","6 %"],["9 ans","9 %"],["12 ans","12 %"],["15 ans","15 %"]],"note":"Paliers triennaux, contre des paliers plus espacés et plafonnés à 8 % pour les ouvriers."}]',
       maj_le = datetime('now')
 WHERE idcc = '0016' AND theme = 'anciennete' AND titre LIKE 'Prime d''ancienneté — employés%';

-- ── Frais de déplacement marchandises : le barème ────────────
UPDATE ccn_reglementations
   SET valeur   = 'Repas 16,36 €',
       source   = 'Protocole du 30 avril 1974 ; avenant n° 81 du 2 décembre 2025, étendu par arrêté du 3 février 2026 ; arrêté du 20 décembre 2002 (frais professionnels)',
       date_effet = '2025-12-02',
       tableaux = '[{"titre":"Barème des indemnités — personnel roulant marchandises","colonnes":["Indemnité","Montant","Situation"],"lignes":[["Casse-croûte","8,87 €","Prise de service avant 5 h ou fin après 22 h"],["Repas","16,36 €","Repas hors du lieu de travail habituel"],["Repas unique","10,07 €","Un seul repas hors du domicile"],["Repas de nuit","9,81 €","Service de nuit"],["Grand déplacement (1 découcher)","52,31 €","Logement + repas du matin"],["Grand déplacement (2 repas + 1 découcher)","68,67 €","Journée complète hors du domicile"]],"note":"Frais professionnels : hors du brut, hors assiette des congés payés, hors comparaison au SMIC, hors assiette de la réduction générale. Exonérés dans les limites de l''arrêté du 20 décembre 2002 — l''excédent est réintégré à défaut de justificatif."}]',
       maj_le   = datetime('now')
 WHERE idcc = '0016' AND activite = 'marchandises' AND theme = 'frais';

-- ============================================================
-- GRILLES DE MINIMA — une par branche
-- ============================================================

INSERT INTO ccn_reglementations
  (idcc, activite, theme, titre, resume, corps, valeur, source, source_url,
   date_effet, impact, regime_social, ordre, tableaux)
VALUES

-- ── SMIC et articulation (transverse) ───────────────────────
('0016', 'transverse', 'minima',
 'Articulation SMIC / minimum conventionnel',
 'Plusieurs coefficients de la branche sont passés sous le SMIC : c''est alors le SMIC qui s''applique.',
 'Le minimum conventionnel est un plancher de branche ; le SMIC est un plancher d''ordre public. On retient le plus élevé des deux, coefficient par coefficient et mois par mois.

Le point sensible de l''IDCC 16 : les grilles ouvrières du transport de marchandises n''ont pas suivi le rythme des revalorisations du SMIC. Plusieurs coefficients d''entrée sont désormais rattrapés, voire dépassés, par le salaire minimum légal. Verser le taux conventionnel d''un 110M ou d''un 128M revient alors à payer sous le SMIC.

Méthode de contrôle en paie :
• Comparer le taux horaire effectif au SMIC horaire en vigueur au mois considéré.
• Comparer séparément la rémunération au minimum conventionnel du coefficient.
• Appliquer le plus favorable. Un rappel de salaire sur le fondement du SMIC se prescrit par trois ans et se cumule avec les congés payés afférents.

Attention : la comparaison au SMIC exclut les remboursements de frais professionnels (indemnités de repas, de découcher), les primes d''ancienneté et les majorations pour heures supplémentaires. Une paie de roulant peut donc afficher un brut confortable tout en étant sous le SMIC sur sa partie comparable.',
 '12,31 €/h',
 'Art. L. 3231-1 et suivants du code du travail ; décret annuel de revalorisation du SMIC',
 NULL, '2026-06-01', 'Brut', 'Soumis à cotisations', 15,
 '[{"titre":"SMIC en vigueur","colonnes":["Base","Montant brut"],"lignes":[["Horaire","12,31 €"],["Mensuel 151,67 h","1 867,02 €"]],"note":"Valeur au 1er juin 2026. À revalider à chaque revalorisation — c''est la donnée qui se périme le plus vite de toute cette page."}]'),

-- ── Marchandises ────────────────────────────────────────────
('0016', 'marchandises', 'minima',
 'Grille des minima — ouvriers marchandises',
 'Taux horaires par coefficient, majorations d''ancienneté intégrées, et garanties annuelles par durée de service.',
 'Grille des personnels ouvriers du transport routier de marchandises. Les taux ci-dessous intègrent déjà la majoration d''ancienneté : la colonne « 2 ans » vaut le taux d''embauche majoré de 2 %, et ainsi de suite.

Trois lectures obligatoires de cette grille :

• Le taux horaire sert au calcul du salaire de base et de toute retenue pour absence.
• La garantie annuelle de rémunération est le vrai contrôle : elle se vérifie sur l''année civile complète, rémunération brute effective cumulée, hors frais de déplacement. Un rappel se constate sur la paie de décembre.
• La base de décompte diffère selon le service : 151,67 h pour un sédentaire, 169 h en courte distance, 200 h en longue distance. Diviser un brut de grand routier par 151,67 produit un taux horaire faux.

Réserve importante sur la fraîcheur : la grille reproduite est celle de l''accord du 11 octobre 2023, étendu le 19 décembre 2023. Les sources consultées divergent sur l''existence d''une revalorisation au 1er janvier 2026 — certaines publient ces mêmes montants comme étant ceux de 2026, d''autres annoncent une hausse de l''ordre de 2,5 à 3 % applicable au 1er janvier 2026, ce qui porterait par exemple le 150M de 1 885,26 € à environ 1 941,82 € mensuels. À trancher sur le texte de l''accord avant tout paramétrage.',
 '12,09 à 12,43 €/h',
 'Accord du 11 octobre 2023, étendu par arrêté du 19 décembre 2023 (revalorisation 2026 à confirmer)',
 NULL, '2023-10-11', 'Brut', 'Soumis à cotisations', 27,
 '[{"titre":"Taux horaires bruts, ancienneté incluse","colonnes":["Coefficient","Embauche","2 ans","5 ans","10 ans","15 ans"],"lignes":[["110M à 120M","12,09 €","12,33 €","12,57 €","12,82 €","13,06 €"],["128M","12,12 €","12,36 €","12,60 €","12,85 €","13,09 €"],["138M","12,14 €","12,38 €","12,63 €","12,87 €","13,11 €"],["150M","12,43 €","12,68 €","12,93 €","13,18 €","13,42 €"]],"note":"Les coefficients d''entrée sont rattrapés par le SMIC (12,31 €/h au 1er juin 2026) : c''est alors le SMIC qui s''applique."},{"titre":"Garanties annuelles de rémunération — longue distance (200 h/mois)","colonnes":["Coefficient","Embauche","2 ans","5 ans","10 ans","15 ans"],"lignes":[["110M à 120M","32 203 €","32 847 €","33 491 €","34 135 €","34 779 €"],["128M","32 283 €","32 928 €","33 574 €","34 220 €","34 865 €"],["138M","32 336 €","32 983 €","33 629 €","34 276 €","34 923 €"],["150M","33 108 €","33 770 €","34 433 €","35 095 €","35 757 €"]],"note":"Autres bases de service : environ 22 664 € à 151,67 h et 25 901 € à 169 h (courte distance) pour les coefficients d''entrée."}]'),

-- ── Voyageurs ───────────────────────────────────────────────
('0016', 'voyageurs', 'minima',
 'Grille des minima — ouvriers voyageurs',
 'Salaires mensuels garantis par coefficient, revalorisés au 1er janvier 2026.',
 'Grille des personnels ouvriers du transport routier de voyageurs, exprimée en salaires mensuels garantis pour 151,67 heures.

Différence de fond avec le transport de marchandises : la grille voyageurs a continué d''être négociée, et ses montants restent au-dessus du SMIC. Le minimum conventionnel prime donc réellement, alors qu''en marchandises c''est le SMIC qui gouverne les bas coefficients.

En paie :
• Les conducteurs en période scolaire relèvent des mêmes coefficients, avec proratisation au temps de travail contractuel.
• La comparaison au minimum exclut les indemnités de repas (frais professionnels), mais inclut les indemnités d''amplitude et de coupure, qui sont du salaire.
• Les avenants du 27 novembre 2025 couvrent les quatre catégories : n° 120 pour les ouvriers, n° 102 pour les employés, n° 100 pour les techniciens et agents de maîtrise, n° 93 pour les ingénieurs et cadres. Tous ont été étendus par le même arrêté du 7 avril 2026.',
 '1 884,80 € à 2 200,60 €',
 'Avenant n° 120 du 27 novembre 2025, étendu par arrêté du 7 avril 2026',
 'https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000053788378',
 '2026-01-01', 'Brut', 'Soumis à cotisations', 35,
 '[{"titre":"Salaires mensuels garantis (151,67 h)","colonnes":["Coefficient","Embauche","Après 5 ans","Après 15 ans"],"lignes":[["110V à 128V","1 884,80 €","1 997,89 €","2 073,28 €"],["131V","1 913,45 €","2 028,26 €","2 104,80 €"],["138V","1 989,86 €","2 109,25 €","2 188,85 €"],["155V","2 200,60 €","2 332,64 €","2 420,66 €"]],"note":"Revalorisation de 1,3 % par rapport à la grille précédente."}]'),

-- ── Déménagement ────────────────────────────────────────────
('0016', 'demenagement', 'minima',
 'Grille des minima — déménagement',
 'Quatre grilles distinctes, coefficients suffixés « DEM », en vigueur au 1er juin 2026.',
 'Le déménagement dispose de sa propre classification et de ses propres grilles, sans rapport avec les coefficients « M » du transport de marchandises. Les coefficients se lisent 1 A DEM à 1 D DEM pour les ouvriers, 2 A à 2 D pour les employés, 3 A à 3 D pour les techniciens et agents de maîtrise, 4 A à 4 C pour les cadres.

Deux barèmes d''ancienneté cohabitent dans la même branche :
• Ouvriers : paliers à 2, 5, 10 et 15 ans.
• Employés, techniciens et agents de maîtrise : paliers triennaux, de 3 à 15 ans.

En paie :
• Le coefficient d''entrée ouvrier (1 A DEM) est sous le SMIC à l''embauche et à deux ans d''ancienneté : c''est le SMIC qui s''applique à ces deux positions.
• Les cadres relèvent d''une rémunération annuelle garantie, contrôlée sur l''année civile, et non d''un taux horaire.
• Les primes de chantier et de rendement, fréquentes dans la branche, entrent dans la comparaison au minimum conventionnel.',
 '12,03 à 21,03 €/h',
 'Avenant n° 24 du 21 janvier 2026 relatif aux salaires minimums (revalorisation moyenne de 1,18 %)',
 NULL, '2026-06-01', 'Brut', 'Soumis à cotisations', 44,
 '[{"titre":"Ouvriers — taux horaires bruts","colonnes":["Coefficient","Embauche","2 ans","5 ans","10 ans","15 ans"],"lignes":[["1 A DEM","12,03 €","12,27 €","12,51 €","12,75 €","12,99 €"],["1 B DEM","12,21 €","12,45 €","12,70 €","12,94 €","13,19 €"],["1 C DEM","12,68 €","12,93 €","13,19 €","13,44 €","13,69 €"],["1 D DEM","13,59 €","13,86 €","14,13 €","14,41 €","14,68 €"]],"note":"Le 1 A DEM est sous le SMIC (12,31 €/h) à l''embauche et à 2 ans : appliquer le SMIC."},{"titre":"Employés — taux horaires bruts","colonnes":["Coefficient","Embauche","3 ans","6 ans","9 ans","12 ans","15 ans"],"lignes":[["2 A DEM","12,03 €","12,39 €","12,75 €","13,11 €","13,47 €","13,83 €"],["2 B DEM","12,11 €","12,47 €","12,84 €","13,20 €","13,56 €","13,93 €"],["2 C DEM","12,44 €","12,81 €","13,19 €","13,56 €","13,93 €","14,31 €"],["2 D DEM","12,85 €","13,24 €","13,62 €","14,01 €","14,39 €","14,78 €"]]},{"titre":"Techniciens et agents de maîtrise — taux horaires bruts","colonnes":["Coefficient","Embauche","3 ans","6 ans","9 ans","12 ans","15 ans"],"lignes":[["3 A DEM","13,44 €","13,84 €","14,25 €","14,65 €","15,05 €","15,46 €"],["3 B DEM","14,19 €","14,62 €","15,04 €","15,47 €","15,89 €","16,32 €"],["3 C DEM","16,23 €","16,72 €","17,20 €","17,69 €","18,18 €","18,66 €"],["3 D DEM","18,29 €","18,84 €","19,39 €","19,94 €","20,48 €","21,03 €"]]},{"titre":"Ingénieurs et cadres — rémunération annuelle garantie","colonnes":["Coefficient","Ancienneté","Annuel","Mensuel"],"lignes":[["4 A DEM","Jusqu''à 5 ans","41 578,56 €","3 118,39 €"],["4 A DEM","Après 5 ans","43 657,49 €","3 274,31 €"],["4 A DEM","Après 10 ans","45 736,42 €","3 430,23 €"],["4 A DEM","Après 15 ans","47 815,34 €","3 586,15 €"],["4 B DEM","Jusqu''à 5 ans","46 458,07 €","3 484,36 €"],["4 B DEM","Après 5 ans","48 780,97 €","3 658,57 €"],["4 B DEM","Après 10 ans","51 103,88 €","3 832,79 €"],["4 B DEM","Après 15 ans","53 426,78 €","4 007,01 €"],["4 C DEM","Jusqu''à 5 ans","56 608,91 €","4 245,67 €"],["4 C DEM","Après 5 ans","59 439,36 €","4 457,95 €"],["4 C DEM","Après 10 ans","62 269,80 €","4 670,24 €"],["4 C DEM","Après 15 ans","65 100,25 €","4 882,52 €"]],"note":"Contrôle sur l''année civile, proratisé au temps de présence."}]'),

-- ── Transport sanitaire ─────────────────────────────────────
('0016', 'sanitaire', 'minima',
 'Grille des minima — transport sanitaire',
 'Trois niveaux d''emploi seulement, dont deux sous le SMIC.',
 'La grille du transport sanitaire est courte : trois niveaux de taux horaire d''embauche, sans progression d''ancienneté intégrée au barème principal.

Deux constats qui pèsent lourd en paie :
• Les niveaux 1 et 2 sont inférieurs au SMIC horaire. C''est donc le SMIC qui s''applique, et la grille conventionnelle ne joue qu''à partir du niveau 3.
• Le vrai levier de rémunération de la branche n''est pas la grille, mais l''articulation entre le décompte du temps de travail (amplitude affectée d''un coefficient d''équivalence) et les indemnités de permanence, de dimanche et de jour férié.

Une négociation de fond est en cours sur les classifications : un accord de méthode du 4 novembre 2025 relatif aux classifications des emplois du transport sanitaire a été étendu par arrêté du 12 février 2026. La grille ci-dessous a donc vocation à être remplacée.',
 '11,89 à 12,79 €/h',
 'Avenant n° 8 du 6 mai 2025 relatif aux rémunérations conventionnelles des personnels ambulanciers',
 NULL, '2025-06-01', 'Brut', 'Soumis à cotisations', 54,
 '[{"titre":"Taux horaires bruts à l''embauche","colonnes":["Niveau d''emploi","Taux horaire"],"lignes":[["Ambulancier niveau 1","11,89 €"],["Ambulancier niveau 2","11,90 €"],["Ambulancier niveau 3","12,79 €"]],"note":"Niveaux 1 et 2 sous le SMIC (12,31 €/h au 1er juin 2026) : appliquer le SMIC."},{"titre":"Indemnités complémentaires","colonnes":["Indemnité","Montant"],"lignes":[["Travail du dimanche ou d''un jour férié","23,90 €"]],"note":"Salaire, non frais professionnel : cotisée, imposable, dans l''assiette des congés payés."}]'),

-- ── Auxiliaires et logistique ───────────────────────────────
('0016', 'auxiliaires', 'minima',
 'Grille des minima — prestations logistiques',
 'Quatre grilles, coefficients suffixés « L », issues du texte publié au Journal officiel.',
 'Les entreprises de prestations logistiques disposent de leur propre grille, avec des coefficients suffixés « L ». Elle est la mieux documentée de la convention : le texte intégral de l''avenant est consultable sur Légifrance, grilles annexées comprises.

Particularités utiles en paie :
• Un palier à six mois d''ancienneté existe chez les ouvriers et les employés, en plus des paliers classiques. Il est souvent oublié au paramétrage.
• Les ouvriers suivent des paliers à 2, 5, 10 et 15 ans ; les employés et les techniciens des paliers triennaux jusqu''à 15 ans.
• Chaque coefficient est assorti d''une garantie annuelle de rémunération, contrôlée sur l''année civile.
• Les cadres relèvent d''une rémunération annuelle garantie, déclinée en équivalent mensuel dans le texte lui-même.
• Aucun régime d''équivalence ici : la durée du travail est celle du droit commun, 35 heures et 151,67 heures mensuelles.',
 '11,91 à 16,76 €/h',
 'Avenant n° 16 du 9 avril 2025 relatif à la revalorisation des minima conventionnels (prestations logistiques)',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000051927426/?idConteneur=KALICONT000005635624',
 '2025-05-01', 'Brut', 'Soumis à cotisations', 63,
 '[{"titre":"Ouvriers — taux horaires bruts","colonnes":["Coefficient","Emploi","Embauche","6 mois","2 ans","5 ans","10 ans","15 ans"],"lignes":[["110 L","Opérateur emballeur, manutentionnaire","11,91 €","11,96 €","12,20 €","12,44 €","12,68 €","12,92 €"],["115 L","Préparateur de commandes, agent logistique","11,91 €","12,05 €","12,29 €","12,53 €","12,77 €","13,01 €"],["120 L","Contrôleur flasheur, agent de maintenance","11,92 €","12,12 €","12,36 €","12,60 €","12,85 €","13,09 €"],["125 L","Cariste","11,95 €","12,18 €","12,42 €","12,67 €","12,91 €","13,15 €"],["138 L","Opérateur de ligne","11,98 €","12,26 €","12,51 €","12,75 €","13,00 €","13,24 €"]],"note":"Toute la grille d''entrée est sous le SMIC (12,31 €/h) : c''est le SMIC qui s''applique jusqu''au palier où la grille le dépasse."},{"titre":"Ouvriers — garanties annuelles de rémunération (151,67 h)","colonnes":["Coefficient","Embauche","2 ans","5 ans","10 ans","15 ans"],"lignes":[["110 L","22 916,93 €","23 375,27 €","23 833,61 €","24 291,95 €","24 750,28 €"],["115 L","23 112,45 €","23 574,70 €","24 036,95 €","24 499,20 €","24 961,45 €"],["120 L","23 210,86 €","23 675,08 €","24 139,29 €","24 603,51 €","25 067,73 €"],["125 L","23 367,84 €","23 835,20 €","24 302,55 €","24 769,91 €","25 237,27 €"],["138 L","23 463,13 €","23 932,39 €","24 401,66 €","24 870,92 €","25 340,18 €"]]},{"titre":"Employés — taux horaires bruts","colonnes":["Coefficient","Emploi","Embauche","6 mois","3 ans","6 ans","9 ans","12 ans","15 ans"],"lignes":[["110 L","Assistant inventaire","11,98 €","12,18 €","12,55 €","12,91 €","13,28 €","13,64 €","14,01 €"],["120 L","Employé d''ordonnancement, agent administratif","12,03 €","12,26 €","12,63 €","13,00 €","13,36 €","13,73 €","14,10 €"]]},{"titre":"Techniciens et agents de maîtrise — taux horaires bruts","colonnes":["Coefficient","Emploi","Embauche","3 ans","6 ans","9 ans","12 ans","15 ans"],"lignes":[["150 L","Technicien de maintenance","13,54 €","13,95 €","14,35 €","14,76 €","15,16 €","15,57 €"],["157,5 L","Chef d''équipe, gestionnaire de stocks, superviseur","13,64 €","14,05 €","14,46 €","14,87 €","15,28 €","15,69 €"],["165 L","Chef de quai","14,15 €","14,57 €","15,00 €","15,42 €","15,85 €","16,27 €"],["200 L","Chef d''exploitation, responsable de service","16,76 €","17,26 €","17,77 €","18,27 €","18,77 €","19,27 €"]]},{"titre":"Ingénieurs et cadres — rémunération annuelle garantie","colonnes":["Coefficient","Emploi","Embauche","5 ans","10 ans","15 ans"],"lignes":[["100 L","Responsable management qualité","40 547,22 €","42 574,58 €","44 601,94 €","46 629,30 €"],["106,5 L","Chef de projet, responsable sécurité","43 192,88 €","45 352,52 €","47 512,17 €","49 671,81 €"],["113 L","Directeur d''exploitation, directeur méthode","45 818,94 €","48 109,89 €","50 400,83 €","52 691,78 €"],["119 L","Directeur de site, directeur conditionnement","47 915,88 €","50 311,67 €","52 707,47 €","55 103,26 €"],["132 L","Directeur de sites logistiques","53 543,94 €","56 221,14 €","58 898,33 €","61 575,53 €"]],"note":"Équivalents mensuels dans le texte : de 3 041,04 € à 4 618,16 € selon le coefficient et l''ancienneté."}]');
