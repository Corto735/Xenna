-- ============================================================
-- CONVENTIONS COLLECTIVES — Consultation des réglementations
-- ayant un impact en paie
--
-- Premier remplissage : IDCC 0016 — Transports routiers et
-- activités auxiliaires du transport, toutes activités
-- (marchandises, voyageurs, déménagement, transport sanitaire,
-- auxiliaires/logistique, transport de fonds).
--
-- Ces tables sont EDITORIALES : elles ne sont lues par aucun
-- calcul. Le moteur de paie continue de tirer ses barèmes de
-- ContextPaie / calculs/absence.rs. On documente ici ce que le
-- gestionnaire doit savoir, pas ce que la machine applique.
--
-- Chaque règle porte un statut de vérification. Le seed est
-- posé en 'a_verifier' : il a été rédigé de mémoire métier, pas
-- recopié du texte conventionnel. Un praticien valide, l'admin
-- bascule en 'verifie'. Mieux vaut un doute affiché qu'un
-- chiffre faux présenté comme une certitude.
-- ============================================================

-- ── Conventions ──────────────────────────────────────────────
CREATE TABLE ccn_conventions (
    idcc            TEXT    PRIMARY KEY,          -- '0016'
    libelle         TEXT    NOT NULL,             -- intitulé officiel complet
    libelle_court   TEXT    NOT NULL,             -- pour les listes / badges
    champ           TEXT    NOT NULL,             -- champ d'application résumé
    brochure_jo     TEXT,                         -- n° de brochure Journal officiel
    legifrance_id   TEXT,                         -- identifiant KALICONT
    date_signature  TEXT,                         -- AAAA-MM-JJ
    publie          INTEGER NOT NULL DEFAULT 1,
    maj_le          TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- ── Sous-champs conventionnels (activités) ───────────────────
-- La CCN 16 n'est pas un bloc homogène : la durée du travail
-- d'un grand routier, d'un ambulancier et d'un transitaire
-- n'ont rien à voir. On segmente.
CREATE TABLE ccn_activites (
    id       INTEGER PRIMARY KEY,
    idcc     TEXT    NOT NULL REFERENCES ccn_conventions(idcc) ON DELETE CASCADE,
    code     TEXT    NOT NULL,   -- 'transverse', 'marchandises', ...
    libelle  TEXT    NOT NULL,
    detail   TEXT,               -- accord ou annexe de rattachement
    ordre    INTEGER NOT NULL DEFAULT 0,
    UNIQUE (idcc, code)
);

-- ── Thèmes (axe transversal de lecture) ──────────────────────
CREATE TABLE ccn_themes (
    code     TEXT    PRIMARY KEY,
    libelle  TEXT    NOT NULL,
    icone    TEXT,
    ordre    INTEGER NOT NULL DEFAULT 0
);

-- ── Règles ───────────────────────────────────────────────────
CREATE TABLE ccn_reglementations (
    id            INTEGER PRIMARY KEY,
    idcc          TEXT    NOT NULL REFERENCES ccn_conventions(idcc) ON DELETE CASCADE,
    activite      TEXT    NOT NULL,   -- code d'activité ('transverse' = toutes)
    theme         TEXT    NOT NULL REFERENCES ccn_themes(code),

    titre         TEXT    NOT NULL,
    resume        TEXT    NOT NULL,   -- une phrase : l'impact paie, sans détour
    corps         TEXT    NOT NULL,   -- texte détaillé, sauts de ligne réels
    valeur        TEXT,               -- chiffre saillant affiché en gros ('2 / 4 / 6 / 8 %')

    source        TEXT    NOT NULL,   -- article, accord, décret
    source_url    TEXT,
    date_effet    TEXT,               -- AAAA-MM-JJ, NULL si non datée

    -- Ce que la règle fait au bulletin : c'est la colonne qui
    -- justifie l'existence de cette page.
    impact        TEXT    NOT NULL CHECK (impact IN (
                      'Brut', 'Cotisations', 'Net imposable',
                      'Cout employeur', 'Temps de travail', 'Hors bulletin')),
    regime_social TEXT,               -- 'Soumis à cotisations', 'Frais professionnels', ...

    statut_verif  TEXT    NOT NULL DEFAULT 'a_verifier'
                          CHECK (statut_verif IN ('brouillon', 'a_verifier', 'verifie')),
    publie        INTEGER NOT NULL DEFAULT 1,
    ordre         INTEGER NOT NULL DEFAULT 0,
    maj_le        TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_ccn_regl_lookup ON ccn_reglementations (idcc, activite, theme);
CREATE INDEX idx_ccn_regl_publie ON ccn_reglementations (publie, ordre);

-- ============================================================
-- SEED — IDCC 0016
-- ============================================================

INSERT INTO ccn_conventions (idcc, libelle, libelle_court, champ, brochure_jo, legifrance_id, date_signature) VALUES
('0016',
 'Convention collective nationale des transports routiers et activités auxiliaires du transport',
 'Transports routiers',
 'Entreprises de transport routier de marchandises et de voyageurs, location de véhicules industriels avec conducteur, déménagement, transport sanitaire, auxiliaires de transport (commissionnaires, transitaires, agents maritimes), prestataires logistiques, transport de fonds et valeurs. Le rattachement se fait par l''activité réelle, pas par le code NAF.',
 '3085',
 'KALICONT000005635624',
 '1950-12-21');

INSERT INTO ccn_activites (idcc, code, libelle, detail, ordre) VALUES
('0016', 'transverse',    'Dispositions communes',      'Corps de la convention + annexes I à IV (ouvriers, employés, TAM, ingénieurs et cadres)', 1),
('0016', 'marchandises',  'Transport de marchandises',  'Annexe I ouvriers roulants « M » + décret n° 83-40 (durée du travail)',                   2),
('0016', 'voyageurs',     'Transport de voyageurs',     'Annexe I ouvriers roulants « V » + décret n° 2003-1242',                                  3),
('0016', 'demenagement',  'Déménagement',               'Accord du 3 juin 1997 et avenants (classifications, temps de service, déplacements)',     4),
('0016', 'sanitaire',     'Transport sanitaire',        'Accord-cadre du 4 mai 2000 et accord du 16 juin 2016 (ambulanciers)',                     5),
('0016', 'auxiliaires',   'Auxiliaires et logistique',  'Annexes II et III — commissionnaires, transitaires, prestataires logistiques',            6),
('0016', 'fonds',         'Transport de fonds',         'Dispositions propres au convoyage de fonds et valeurs',                                   7);

INSERT INTO ccn_themes (code, libelle, icone, ordre) VALUES
('classification', 'Classifications et coefficients',    '#',  1),
('minima',         'Salaires minima conventionnels',     '€',  2),
('anciennete',     'Ancienneté',                         '↗',  3),
('duree',          'Durée du travail et équivalences',   '⏱',  4),
('hsup',           'Heures supplémentaires et repos',    '+',  5),
('nuit',           'Nuit, dimanches et jours fériés',    '☾',  6),
('primes',         'Primes et compléments',              '★',  7),
('frais',          'Frais de déplacement',               '⌂',  8),
('absence',        'Maladie, accident, maternité',       '✚',  9),
('prevoyance',     'Prévoyance, santé, fin d''activité', '⛨', 10),
('conges',         'Congés et absences autorisées',      '▤', 11),
('rupture',        'Préavis et rupture du contrat',      '⇥', 12);

-- ────────────────────────────────────────────────────────────
-- DISPOSITIONS COMMUNES
-- ────────────────────────────────────────────────────────────

INSERT INTO ccn_reglementations (idcc, activite, theme, titre, resume, corps, valeur, source, date_effet, impact, regime_social, ordre) VALUES

('0016', 'transverse', 'classification',
 'Architecture des annexes et des coefficients',
 'Le coefficient détermine le taux horaire minimum : c''est le point de départ de toute paie de la branche.',
 'La convention range les salariés en quatre annexes, chacune avec sa grille de coefficients et son propre barème d''ancienneté.

• Annexe I — Ouvriers. Coefficients suffixés selon l''activité : « M » pour les roulants marchandises (110M à 150M), « V » pour les voyageurs (137V à 150V), « A » pour les ambulanciers, sans suffixe pour les sédentaires (110 à 155).
• Annexe II — Employés. Coefficients 105 à 160.
• Annexe III — Techniciens et agents de maîtrise. Coefficients 150 à 225.
• Annexe IV — Ingénieurs et cadres. Groupes 1 à 9, rémunération annuelle garantie et non taux horaire.

Conséquence en paie : un même salaire brut peut être conforme ou non selon le coefficient porté au contrat. Le coefficient doit figurer sur le bulletin (art. R. 3243-1 du code du travail : position dans la classification).',
 'I à IV',
 'CCN 16, annexes I à IV',
 NULL, 'Brut', NULL, 1),

('0016', 'transverse', 'anciennete',
 'Prime d''ancienneté — ouvriers',
 'Majoration en pourcentage du salaire minimum du coefficient, versée en ligne distincte du bulletin.',
 'Barème par paliers, calculé sur le taux horaire conventionnel garanti du coefficient de l''intéressé, et non sur le salaire réellement pratiqué s''il est supérieur.

• 2 % après 2 ans de présence dans l''entreprise
• 4 % après 5 ans
• 6 % après 10 ans
• 8 % après 15 ans

Points de vigilance :
• L''assiette est le minimum conventionnel du coefficient pour l''horaire de référence, hors heures supplémentaires, hors primes, hors frais de déplacement.
• La prime est un élément de salaire : soumise à cotisations, imposable, intégrée à l''assiette des congés payés et de la réduction générale (Fillon).
• Elle doit apparaître sur une ligne propre. Un brut global « tout compris » ne vaut pas paiement de la prime.',
 '2 / 4 / 6 / 8 %',
 'CCN 16, annexe I (ouvriers)',
 NULL, 'Brut', 'Soumis à cotisations', 2),

('0016', 'transverse', 'anciennete',
 'Prime d''ancienneté — employés, techniciens et agents de maîtrise',
 'Barème plus généreux et plus long que celui des ouvriers, jusqu''à 15 %.',
 'Les annexes II (employés) et III (TAM) retiennent des paliers triennaux :

• 3 % après 3 ans
• 6 % après 6 ans
• 9 % après 9 ans
• 12 % après 12 ans
• 15 % après 15 ans

Même logique d''assiette que pour les ouvriers : pourcentage appliqué au salaire minimum conventionnel du coefficient. Même traitement social : salaire à part entière.

Erreur classique en paie : appliquer le barème ouvrier (2/4/6/8) à un employé sédentaire administratif parce que l''entreprise est « du transport ». C''est l''annexe de rattachement qui commande, pas l''activité de l''employeur.',
 'jusqu''à 15 %',
 'CCN 16, annexes II et III',
 NULL, 'Brut', 'Soumis à cotisations', 3),

('0016', 'transverse', 'minima',
 'Garantie annuelle de rémunération',
 'Contrôle de fin d''année : la rémunération annuelle effective doit atteindre le plancher conventionnel du coefficient.',
 'Au-delà des taux horaires mensuels, la branche garantit un montant annuel par coefficient (« garantie annuelle de rémunération » pour les ouvriers, employés et TAM ; « rémunération annuelle garantie » pour les cadres, où elle remplace le taux horaire).

Mécanique de contrôle :
• On totalise sur l''année civile la rémunération brute soumise à cotisations effectivement versée.
• On exclut de ce total les sommes qui ne sont pas la contrepartie du travail : remboursements de frais de déplacement, participation, intéressement, primes exceptionnelles non prévues au contrat, indemnités de rupture.
• Si le total est inférieur à la garantie du coefficient, un rappel est dû sur la paie de décembre.
• La garantie est proratisée au temps de présence (entrée, sortie, temps partiel, suspension non rémunérée).

C''est le contrôle que les inspections de branche demandent en premier, et celui que les logiciels de paie ne font presque jamais tout seuls.',
 'Contrôle annuel',
 'CCN 16 + accords salariaux de branche',
 NULL, 'Brut', 'Soumis à cotisations', 4),

('0016', 'transverse', 'absence',
 'Garantie de ressources en maladie et accident',
 'Maintien à 100 % puis 75 % dès 3 ans d''ancienneté, nettement au-dessus du régime légal de mensualisation.',
 'Régime conventionnel de maintien de salaire, plus favorable que la mensualisation légale (art. L. 1226-1 et D. 1226-1 du code du travail).

• Ancienneté requise : 3 ans dans l''entreprise (contre 1 an au régime légal).
• Maladie ou accident non professionnel : carence conventionnelle de 5 jours (le régime légal impose 7 jours).
• Accident du travail, accident de trajet et maladie professionnelle : aucune carence, maintien dès le premier jour d''arrêt.
• Taux : 100 % de la rémunération nette pendant la première période, puis 75 %. La durée de chaque période croît avec l''ancienneté.

Traitement en paie :
• Le maintien s''entend déduction faite des indemnités journalières de sécurité sociale — c''est un complément, pas un cumul.
• En cas de subrogation, l''employeur perçoit les IJSS et verse la rémunération maintenue ; il conserve les IJSS pendant toute la durée du maintien, y compris sur les jours de carence sécurité sociale.
• Le complément employeur est soumis à cotisations ; les IJSS reversées au salarié hors subrogation ne le sont pas (CSG/CRDS à 6,20 % / 0,50 % prélevés par la caisse).

Cette règle est celle que le simulateur applique déjà : module Absences, régime « IDCC 0016 — Transport routier ».',
 '100 % puis 75 %',
 'CCN 16, garantie de ressources ; art. L. 1226-1 et D. 1226-1 du code du travail',
 NULL, 'Brut', 'Soumis à cotisations', 5),

('0016', 'transverse', 'prevoyance',
 'Régime de prévoyance conventionnel',
 'Cotisation obligatoire à un régime de branche : incapacité, invalidité, décès, financée employeur et salarié.',
 'La branche impose un régime de prévoyance collectif, historiquement porté par CARCEPT Prévoyance (groupe Klesia), avec recommandation d''organisme et non désignation depuis la censure constitutionnelle de 2013 (décision n° 2013-672 DC).

Ce que cela produit sur le bulletin :
• Une ou plusieurs lignes de cotisation prévoyance, part salariale et part patronale, assises sur la tranche A et la tranche B.
• Pour les cadres, s''y ajoute l''obligation historique de l''article 7 de la convention AGIRC de 1947, reprise par l''ANI du 17 novembre 2017 : 1,50 % de la tranche A à la charge exclusive de l''employeur, affecté en priorité à la couverture décès.
• La part patronale de prévoyance est exclue de l''assiette de cotisations dans les limites de l''article D. 242-1 du code de la sécurité sociale, mais elle est soumise au forfait social à 8 % (entreprises d''au moins 11 salariés) et à la CSG/CRDS.
• Attention à ne pas confondre avec la santé : seule la part patronale finançant les frais de santé est réintégrée dans le net imposable. La prévoyance lourde (incapacité, invalidité, décès) reste hors impôt dans les limites fiscales.',
 'TA + TB',
 'CCN 16, régime de prévoyance de branche ; ANI du 17 novembre 2017 ; art. D. 242-1 CSS',
 NULL, 'Cotisations', 'Part patronale : forfait social 8 % + CSG/CRDS', 6),

('0016', 'transverse', 'prevoyance',
 'Complémentaire santé de branche',
 'Panier de soins obligatoire, part patronale réintégrée dans le net imposable.',
 'Depuis la généralisation issue de l''ANI du 11 janvier 2013 (art. L. 911-7 CSS), tout salarié bénéficie d''une couverture frais de santé, avec un socle conventionnel de branche.

Effets en paie :
• Ligne de cotisation mutuelle en part salariale et part patronale.
• La part patronale est exonérée de cotisations de sécurité sociale dans les limites de l''article D. 242-1 CSS, mais soumise au forfait social 8 % (≥ 11 salariés) et à la CSG/CRDS sur 100 % de son montant.
• Elle est intégralement réintégrée dans le net imposable du salarié depuis la loi de finances pour 2014.
• Les cas de dispense d''adhésion doivent être tracés par écrit ; sans écrit, l''URSSAF requalifie et redresse la totalité de l''exonération.',
 'Socle obligatoire',
 'ANI du 11 janvier 2013 ; art. L. 911-7 et D. 242-1 CSS ; accord santé de branche',
 NULL, 'Net imposable', 'Part patronale imposable, forfait social 8 %', 7),

('0016', 'transverse', 'nuit',
 'Travail de nuit',
 'Majoration conventionnelle des heures de nuit, en plus des contreparties légales en repos.',
 'Est de nuit, dans la branche, le travail accompli entre 21 heures et 6 heures (le code du travail retient 21 h - 6 h par défaut, art. L. 3122-2).

• Majoration conventionnelle des heures effectuées sur cette plage, distincte des majorations pour heures supplémentaires : les deux se cumulent sur la même heure.
• Le travailleur de nuit au sens de l''article L. 3122-5 (deux fois 3 heures par semaine habituellement, ou un seuil annuel) ouvre droit en outre à un repos compensateur.
• La majoration de nuit est du salaire : cotisée, imposable, incluse dans l''assiette des congés payés et de la réduction générale.
• Elle entre dans l''assiette de comparaison avec le minimum conventionnel horaire uniquement pour les heures concernées, pas en lissage sur le mois.',
 '20 %',
 'CCN 16 ; art. L. 3122-2 et L. 3122-5 du code du travail',
 NULL, 'Brut', 'Soumis à cotisations', 8),

('0016', 'transverse', 'hsup',
 'Contingent annuel d''heures supplémentaires',
 'La branche fixe des contingents supérieurs au contingent réglementaire de 220 heures pour les roulants.',
 'Le contingent détermine à partir de quel volume annuel les heures supplémentaires déclenchent la contrepartie obligatoire en repos (art. L. 3121-30 du code du travail).

• Contingent réglementaire de droit commun : 220 heures par an et par salarié (art. D. 3121-24).
• Personnels roulants « grands routiers » : contingent de branche porté à 195 heures dans le régime de décompte trimestriel du décret de 1983 — la comparaison directe avec les 220 heures annuelles est trompeuse, le mode de décompte n''est pas le même.
• Personnels sédentaires : contingent de droit commun.

Ce que cela change en paie : au-delà du contingent, chaque heure supplémentaire ouvre une contrepartie obligatoire en repos (50 % dans les entreprises jusqu''à 20 salariés, 100 % au-delà), qui doit être suivie en compteur et payée si elle n''est pas prise à la rupture.',
 '195 à 220 h',
 'Art. L. 3121-30 et D. 3121-24 du code du travail ; accords de branche',
 NULL, 'Temps de travail', NULL, 9),

('0016', 'transverse', 'conges',
 'Congés supplémentaires d''ancienneté',
 'Jours ouvrables de congés en plus des 30 jours légaux, acquis par paliers d''ancienneté.',
 'La convention accorde des jours de congés supplémentaires au-delà du congé légal de 30 jours ouvrables, par paliers d''ancienneté dans l''entreprise.

Traitement en paie :
• Ces jours sont indemnisés comme du congé payé : règle du dixième ou règle du maintien de salaire, la plus favorable au salarié (art. L. 3141-24).
• Ils s''intègrent au compteur de congés et à l''indemnité compensatrice en cas de rupture.
• Ils n''ouvrent pas de droit distinct en matière de cotisations : c''est du salaire.

Attention à ne pas confondre avec les jours de fractionnement (art. L. 3141-23), qui obéissent à leur propre logique et se cumulent avec les jours d''ancienneté sauf stipulation contraire.',
 'Par paliers',
 'CCN 16 ; art. L. 3141-23 et L. 3141-24 du code du travail',
 NULL, 'Brut', 'Soumis à cotisations', 10),

('0016', 'transverse', 'conges',
 'Congés pour événements familiaux',
 'Autorisations d''absence rémunérées, souvent supérieures au socle légal.',
 'La convention fixe des durées d''absence autorisée payée pour mariage, PACS, naissance, décès d''un proche, annonce d''un handicap chez l''enfant.

En paie :
• Absence rémunérée : le salaire est maintenu, aucune retenue n''est opérée. La ligne peut être neutralisée ou affichée en retenue + rappel selon le paramétrage.
• Ces jours sont assimilés à du temps de travail effectif pour l''acquisition des congés payés.
• Le socle légal (art. L. 3142-1 et suivants) est un plancher : on retient la durée la plus favorable entre la loi et la convention, événement par événement, sans panachage global.',
 'Absence payée',
 'CCN 16 ; art. L. 3142-1 et suivants du code du travail',
 NULL, 'Brut', 'Soumis à cotisations', 11),

('0016', 'transverse', 'rupture',
 'Préavis conventionnel',
 'Durées de préavis par catégorie et ancienneté, supérieures au minimum légal pour les cadres.',
 'Le préavis conventionnel varie selon l''annexe de rattachement et l''ancienneté. Ordre de grandeur usuel : 1 semaine à 1 mois pour les ouvriers selon l''ancienneté, 1 mois pour les employés, 2 mois pour les TAM, 3 mois pour les ingénieurs et cadres.

En paie :
• Préavis non exécuté à l''initiative de l''employeur : indemnité compensatrice de préavis, soumise à cotisations et à l''impôt, intégrée à l''assiette des congés payés.
• Préavis non exécuté à l''initiative du salarié : aucune indemnité due, et l''employeur peut réclamer la contrepartie du préavis non effectué.
• Le préavis légal (art. L. 1234-1) reste un plancher : 1 mois de 6 mois à 2 ans d''ancienneté, 2 mois au-delà. On applique la disposition la plus favorable.',
 '1 semaine à 3 mois',
 'CCN 16, annexes I à IV ; art. L. 1234-1 du code du travail',
 NULL, 'Brut', 'Soumis à cotisations', 12),

('0016', 'transverse', 'rupture',
 'Indemnité de licenciement conventionnelle',
 'Barème conventionnel à comparer, année par année, avec l''indemnité légale.',
 'L''indemnité conventionnelle de licenciement obéit à un barème propre à chaque annexe, fonction de l''ancienneté et parfois de l''âge.

Méthode obligatoire : calculer les deux indemnités, retenir la plus élevée. On ne panache pas — pas de salaire de référence légal appliqué au barème conventionnel.

• Indemnité légale (art. R. 1234-2) : 1/4 de mois par année jusqu''à 10 ans, 1/3 au-delà, salaire de référence = moyenne des 12 ou des 3 derniers mois, la plus favorable.
• Régime social : exonérée de cotisations dans la limite de 2 fois le plafond annuel de sécurité sociale, et dans la limite du montant conventionnel ou légal si celui-ci est supérieur. Assujettie à CSG/CRDS sans abattement au-delà du montant légal ou conventionnel.
• Régime fiscal : exonérée à hauteur du montant conventionnel, ou de 50 % de l''indemnité totale, ou de 2 fois la rémunération annuelle brute de l''année civile précédente, dans la limite de 6 PASS.',
 'Barème vs légal',
 'CCN 16, annexes I à IV ; art. R. 1234-2 du code du travail ; art. 80 duodecies CGI',
 NULL, 'Cout employeur', 'Exonérations plafonnées', 13),

('0016', 'transverse', 'rupture',
 'Départ et mise à la retraite',
 'Indemnité conventionnelle de départ en retraite, au régime social distinct du licenciement.',
 '• Départ volontaire à la retraite : l''indemnité conventionnelle est intégralement soumise à cotisations et à l''impôt sur le revenu. Aucune exonération, hors plan de sauvegarde de l''emploi.
• Mise à la retraite par l''employeur : indemnité au moins égale à l''indemnité légale de licenciement, exonérée dans les mêmes limites que celle-ci, mais l''employeur acquitte une contribution spécifique de 50 % sur le montant total (art. L. 137-12 CSS).

Le barème conventionnel de la branche s''exprime en mois de salaire par tranche d''ancienneté et s''applique dès lors qu''il est plus favorable que le barème légal.',
 'Contribution 50 %',
 'CCN 16 ; art. L. 1237-9 et L. 1237-7 du code du travail ; art. L. 137-12 CSS',
 NULL, 'Cout employeur', 'Départ volontaire : intégralement cotisé', 14);

-- ────────────────────────────────────────────────────────────
-- TRANSPORT DE MARCHANDISES
-- ────────────────────────────────────────────────────────────

INSERT INTO ccn_reglementations (idcc, activite, theme, titre, resume, corps, valeur, source, date_effet, impact, regime_social, ordre) VALUES

('0016', 'marchandises', 'duree',
 'Temps de service et durées équivalentes',
 'Le roulant n''est pas payé sur 35 heures : le décret de 1983 fixe des durées équivalentes plus longues.',
 'Le personnel roulant marchandises relève d''un régime d''équivalence : la durée du travail se décompte en « temps de service » (conduite, travaux annexes, mise à disposition), et une durée supérieure à 35 heures est réputée équivaloir à la durée légale.

• Grands routiers ou longue distance (découchés réguliers) : 43 heures par semaine, soit 559 heures par trimestre, soit 186 heures par mois en équivalent mensuel.
• Courte distance et messagerie : 39 heures par semaine, soit 507 heures par trimestre, soit 169 heures par mois.
• Personnels sédentaires : 35 heures, aucun régime d''équivalence.

Conséquence directe en paie :
• L''horaire mensuel contractuel de référence n''est pas 151,67 heures mais 169 ou 186 heures selon le service.
• Le calcul d''une retenue pour absence, d''un taux horaire, d''un maintien de salaire doit reposer sur cet horaire-là. Diviser un brut de grand routier par 151,67 produit un taux horaire faux et une retenue d''absence surévaluée.
• Le décompte est trimestriel, ce qui décale la constatation des heures supplémentaires par rapport au mois de paie.',
 '186 h / 169 h',
 'Décret n° 83-40 du 26 janvier 1983 ; art. L. 3121-13 du code du travail',
 NULL, 'Temps de travail', NULL, 20),

('0016', 'marchandises', 'hsup',
 'Heures supplémentaires des roulants marchandises',
 'Majorations calculées au-delà de la durée équivalente, sur un décompte trimestriel.',
 'Les heures supplémentaires ne se déclenchent qu''au-delà de la durée équivalente applicable au service, pas au-delà de 35 heures.

• Majoration de 25 % pour les premières heures au-delà du seuil, 50 % ensuite, selon les paliers fixés par le décret et les accords de branche.
• Le décompte trimestriel implique une régularisation : des heures effectuées en janvier peuvent ne devenir supplémentaires qu''à la clôture du trimestre.
• Les heures supplémentaires ouvrent droit à la réduction de cotisations salariales (art. L. 241-17 CSS) et à l''exonération d''impôt sur le revenu dans la limite annuelle en vigueur, ainsi qu''à la déduction forfaitaire patronale pour les entreprises de moins de 250 salariés.
• Elles entrent dans l''assiette de la réduction générale de cotisations patronales, mais la rémunération à retenir pour le calcul du coefficient exclut les majorations elles-mêmes selon les règles de l''article D. 241-7 CSS.',
 '25 % puis 50 %',
 'Décret n° 83-40 ; art. L. 241-17 CSS ; art. 81 quater CGI',
 NULL, 'Brut', 'Réduction salariale + exonération IR plafonnée', 21),

('0016', 'marchandises', 'frais',
 'Indemnités de déplacement — protocole du 30 avril 1974',
 'Le poste le plus lourd du bulletin d''un roulant après le salaire, et le plus contrôlé par l''URSSAF.',
 'Le protocole du 30 avril 1974, régulièrement revalorisé par avenants, fixe les indemnités forfaitaires du personnel roulant marchandises. Les principales :

• Indemnité de repas : service coupé par un repas hors du lieu de travail habituel.
• Indemnité de repas unique : un seul repas pris hors du domicile.
• Indemnité de casse-croûte : prise de service très matinale.
• Indemnité de repas de nuit.
• Indemnité spéciale : petite indemnité pour service ne remplissant pas les conditions du repas.
• Indemnité de grand déplacement ou « découcher » : repos journalier pris hors du domicile, comprenant le logement et le repas du matin.

Régime social — c''est là que tout se joue :
• Ce sont des remboursements de frais professionnels, donc exclus de l''assiette de cotisations, mais seulement dans les limites de l''arrêté du 20 décembre 2002 relatif aux frais professionnels.
• Si l''indemnité conventionnelle dépasse le forfait URSSAF, l''excédent est réintégré dans l''assiette, sauf justificatif de la dépense réelle.
• Ces indemnités n''entrent pas dans l''assiette des congés payés, ni dans la garantie annuelle de rémunération, ni dans la comparaison au SMIC, ni dans l''assiette de la réduction générale.
• Elles doivent figurer sur le bulletin hors du brut, en bas, après le net.',
 'Barème 1974 révisé',
 'Protocole du 30 avril 1974 et avenants ; arrêté du 20 décembre 2002 (frais professionnels)',
 NULL, 'Hors bulletin', 'Frais professionnels — exonéré dans les limites URSSAF', 22),

('0016', 'marchandises', 'primes',
 'Prime annuelle',
 'Treizième mois conventionnel, sous condition d''ancienneté, proratisé.',
 'La branche prévoit une prime annuelle pour le personnel remplissant une condition d''ancienneté, généralement un an de présence.

En paie :
• Montant assis sur le salaire conventionnel du coefficient, proratisé au temps de présence sur la période de référence.
• Salaire à part entière : cotisée, imposable, prise en compte dans la garantie annuelle de rémunération.
• Elle entre dans l''assiette de la réduction générale du mois de versement, ce qui écrase mécaniquement le coefficient Fillon de ce mois — d''où l''intérêt de la régularisation progressive annuelle.
• Sur l''indemnité de congés payés : une prime annuelle couvrant l''ensemble de l''année, périodes de congés comprises, n''entre pas dans l''assiette du dixième. Une prime liée à l''activité, oui.',
 'Après 1 an',
 'CCN 16 ; jurisprudence sur l''assiette du dixième (Cass. soc.)',
 NULL, 'Brut', 'Soumis à cotisations', 23),

('0016', 'marchandises', 'prevoyance',
 'Congé de fin d''activité — FONGECFA-Transport',
 'Cotisation obligatoire spécifique à la branche marchandises, ligne dédiée sur le bulletin.',
 'Le congé de fin d''activité permet aux conducteurs routiers marchandises de cesser leur activité avant l''âge légal de la retraite, sous conditions d''ancienneté de conduite. Il est financé par une cotisation obligatoire recouvrée par le FONGECFA-Transport.

En paie :
• Une ligne de cotisation dédiée, avec une part salariale et une part patronale, assise sur le salaire brut.
• Elle ne se confond ni avec la retraite complémentaire AGIRC-ARRCO, ni avec la prévoyance : c''est un dispositif de branche autonome.
• L''oubli de cette ligne est un classique des reprises de dossier : le contrôle de branche la cherche systématiquement.
• L''équivalent pour le transport de voyageurs est l''AGECFA-Voyageurs, avec ses propres taux.',
 'Cotisation dédiée',
 'Accords de branche relatifs au congé de fin d''activité ; FONGECFA-Transport',
 NULL, 'Cotisations', 'Soumis à cotisations', 24),

('0016', 'marchandises', 'duree',
 'Amplitude, coupures et temps de liaison',
 'Des temps qui ne sont pas de la conduite mais qui se paient tout de même.',
 '• Amplitude : durée entre la prise et la fin de service, repos compris. Elle est plafonnée, avec des dérogations encadrées.
• Coupures et temps de mise à disposition : périodes pendant lesquelles le conducteur n''est ni en conduite ni libre de vaquer. Elles sont en tout ou partie intégrées au temps de service et donc rémunérées.
• Temps de liaison : trajet entre le domicile ou l''établissement et le lieu de prise de service du véhicule, indemnisé selon les règles de branche.

En paie, ces temps grossissent le temps de service du mois, donc le déclenchement des heures supplémentaires, sans toujours apparaître comme de la conduite dans le chronotachygraphe. Le contrôle de cohérence entre disque, feuille de route et bulletin est le point de friction habituel en contentieux prud''homal.',
 'Temps payé',
 'Décret n° 83-40 ; règlement (CE) n° 561/2006 (temps de conduite)',
 NULL, 'Temps de travail', NULL, 25),

('0016', 'marchandises', 'classification',
 'Coefficients des conducteurs marchandises',
 'Du 110M au 150M : le suffixe « M » signale un roulant marchandises.',
 'Échelle usuelle des conducteurs marchandises, du moins qualifié au plus qualifié :

• 110M, 115M, 118M — conducteurs de véhicules légers et poids lourds d''entrée de gamme
• 128M — conducteur poids lourd confirmé
• 138M — conducteur grand routier
• 150M — conducteur hautement qualifié, matières dangereuses, ensembles articulés spécifiques

Le coefficient conditionne trois choses en paie : le taux horaire minimum, l''assiette de la prime d''ancienneté, et la garantie annuelle de rémunération. Un déclassement au contrat est une source de rappel de salaire sur trois ans, prime d''ancienneté recalculée comprise.',
 '110M à 150M',
 'CCN 16, annexe I — ouvriers',
 NULL, 'Brut', NULL, 26);

-- ────────────────────────────────────────────────────────────
-- TRANSPORT DE VOYAGEURS
-- ────────────────────────────────────────────────────────────

INSERT INTO ccn_reglementations (idcc, activite, theme, titre, resume, corps, valeur, source, date_effet, impact, regime_social, ordre) VALUES

('0016', 'voyageurs', 'duree',
 'Amplitude et coupures des conducteurs de voyageurs',
 'Le métier est fait de trous dans la journée : leur indemnisation est le cœur du sujet.',
 'Le service d''un conducteur de car est structurellement discontinu : pointes du matin et du soir, creux au milieu.

• Amplitude maximale de la journée de travail : 12 heures, portée à 13 ou 14 heures dans des cas dérogatoires encadrés par accord.
• Les coupures qui séparent deux vacations ne sont pas du travail effectif, mais elles sont partiellement indemnisées, selon des paliers de durée.
• Au-delà d''un certain seuil d''amplitude, une indemnité spécifique est due.

En paie : ces indemnités d''amplitude et de coupure sont du salaire, cotisées et imposables — à ne pas confondre avec les indemnités de repas, qui sont des frais professionnels. La confusion des deux natures est l''erreur la plus fréquente sur les bulletins du transport de voyageurs, et elle se paie au contrôle URSSAF.',
 'Amplitude 12 h',
 'Décret n° 2003-1242 du 22 décembre 2003 ; CCN 16',
 NULL, 'Brut', 'Indemnités d''amplitude : soumises à cotisations', 30),

('0016', 'voyageurs', 'prevoyance',
 'Congé de fin d''activité — AGECFA-Voyageurs',
 'Pendant voyageurs du FONGECFA : cotisation obligatoire, ligne dédiée.',
 'Dispositif de cessation anticipée d''activité propre au transport routier de voyageurs, géré par l''AGECFA-Voyageurs.

• Cotisation obligatoire répartie entre employeur et salarié, assise sur le brut.
• Conditions d''accès liées à l''âge et au nombre d''années de conduite.
• Ligne de cotisation distincte sur le bulletin, sans lien avec la retraite complémentaire.

Une entreprise mixte marchandises et voyageurs cotise aux deux organismes, chacun sur la population qui le concerne. Le rattachement se fait par l''emploi occupé, pas par l''activité principale de l''entreprise.',
 'Cotisation dédiée',
 'Accords de branche congé de fin d''activité ; AGECFA-Voyageurs',
 NULL, 'Cotisations', 'Soumis à cotisations', 31),

('0016', 'voyageurs', 'nuit',
 'Dimanches et jours fériés travaillés',
 'Indemnisation conventionnelle spécifique du personnel roulant voyageurs.',
 'Le transport de voyageurs fonctionne le dimanche et les jours fériés. La convention prévoit une contrepartie propre pour le personnel roulant amené à travailler ces jours-là.

• Indemnité forfaitaire ou majoration selon la nature du jour et l''activité.
• Le 1er mai obéit à son propre régime légal : paiement double, non compensable par du repos (art. L. 3133-6).
• Ces sommes sont du salaire : cotisées, imposables, incluses dans l''assiette des congés payés.
• Elles n''ouvrent pas droit à l''exonération heures supplémentaires, sauf à correspondre effectivement à des heures au-delà du seuil.',
 'Majoration',
 'CCN 16 ; art. L. 3133-6 du code du travail',
 NULL, 'Brut', 'Soumis à cotisations', 32),

('0016', 'voyageurs', 'duree',
 'Conducteurs en période scolaire',
 'Temps partiel annualisé sur l''année scolaire : le lissage est la principale source d''erreur.',
 'Les conducteurs affectés aux services scolaires travaillent sur une fraction de l''année, avec des périodes non travaillées correspondant aux vacances.

En paie :
• Contrat de travail intermittent (art. L. 3123-33) ou temps partiel modulé, avec lissage de la rémunération sur douze mois.
• Le lissage impose un suivi de compteur : heures dues, heures effectuées, régularisation en fin de période et à la rupture.
• Les périodes non travaillées rémunérées comptent pour l''acquisition des congés payés et pour l''ancienneté.
• Le salarié à temps partiel ouvre droit à la réduction générale de cotisations sur la base du SMIC proratisé à son horaire contractuel — l''erreur de proratisation fausse le Fillon toute l''année.',
 'Lissage 12 mois',
 'CCN 16 ; art. L. 3123-33 et suivants du code du travail',
 NULL, 'Brut', 'Soumis à cotisations', 33),

('0016', 'voyageurs', 'classification',
 'Coefficients des conducteurs de voyageurs',
 'Suffixe « V » : 137V à 150V selon la qualification et le type de service.',
 'Échelle des conducteurs receveurs et conducteurs de voyageurs, du service urbain au grand tourisme.

Le coefficient commande le taux horaire garanti, l''assiette de la prime d''ancienneté et la garantie annuelle de rémunération. Les conducteurs en période scolaire relèvent des mêmes coefficients, avec proratisation de la garantie annuelle à leur temps de travail contractuel.',
 '137V à 150V',
 'CCN 16, annexe I — ouvriers',
 NULL, 'Brut', NULL, 34);

-- ────────────────────────────────────────────────────────────
-- DÉMÉNAGEMENT
-- ────────────────────────────────────────────────────────────

INSERT INTO ccn_reglementations (idcc, activite, theme, titre, resume, corps, valeur, source, date_effet, impact, regime_social, ordre) VALUES

('0016', 'demenagement', 'classification',
 'Classifications du déménagement',
 'Grille propre, distincte de celle du transport de marchandises.',
 'L''accord du 3 juin 1997 et ses avenants dotent le déménagement de sa propre classification, articulée autour des emplois de déménageur, conducteur-déménageur, chef d''équipe et encadrement de chantier.

En paie, il faut résister au réflexe d''appliquer la grille marchandises : les coefficients, les minima et la prime d''ancienneté suivent la grille déménagement. Le rattachement se fait par l''activité réelle de l''établissement et l''emploi occupé.',
 'Grille propre',
 'Accord du 3 juin 1997 (déménagement) et avenants',
 '1997-06-03', 'Brut', NULL, 40),

('0016', 'demenagement', 'duree',
 'Temps de service en déménagement',
 'Régime d''équivalence spécifique, distinct de celui des marchandises.',
 'Le personnel de déménagement relève d''un décompte en temps de service avec régime d''équivalence propre, tenant compte de l''alternance conduite / manutention / attente sur chantier.

Points de paie :
• L''horaire mensuel de référence n''est ni 151,67 heures ni celui des grands routiers. Il faut le lire dans l''accord applicable au service.
• Les temps de trajet vers le chantier et les temps d''attente sur place sont pour tout ou partie du temps de service, donc rémunérés.
• Le déclenchement des heures supplémentaires suit ce seuil-là.',
 'Équivalence propre',
 'Accord du 3 juin 1997 ; décret n° 83-40 pour les dispositions communes',
 NULL, 'Temps de travail', NULL, 41),

('0016', 'demenagement', 'frais',
 'Indemnités de déplacement du déménagement',
 'Barème distinct de celui du protocole marchandises de 1974.',
 'Le déménagement dispose de son propre barème d''indemnités de repas et de grand déplacement, adapté aux chantiers de plusieurs jours loin de l''établissement.

Régime social identique en logique à celui des marchandises :
• Frais professionnels exclus de l''assiette de cotisations dans les limites de l''arrêté du 20 décembre 2002.
• Excédent réintégré, sauf justification de la dépense réelle.
• Hors assiette congés payés, hors comparaison SMIC, hors réduction générale.
• À afficher sous le net, jamais dans le brut.',
 'Barème propre',
 'Accord du 3 juin 1997 ; arrêté du 20 décembre 2002',
 NULL, 'Hors bulletin', 'Frais professionnels — exonéré dans les limites URSSAF', 42),

('0016', 'demenagement', 'primes',
 'Primes de chantier et de rendement',
 'Compléments liés à l''activité, à intégrer dans l''assiette du dixième.',
 'Le déménagement pratique des compléments variables liés au chantier : primes de rendement, primes de chantier, participation aux recettes.

En paie :
• Salaire à part entière : cotisées, imposables.
• Parce qu''elles rémunèrent l''activité et non l''année entière, elles entrent dans l''assiette de l''indemnité de congés payés au dixième — contrairement à une prime annuelle globale.
• Elles entrent dans la comparaison avec le minimum conventionnel et dans l''assiette de la réduction générale.
• Leur caractère variable rend la régularisation annuelle de la réduction générale indispensable : le calcul mois par mois seul produit systématiquement un écart.',
 'Assiette du dixième',
 'Accord du 3 juin 1997 ; art. L. 3141-24 du code du travail',
 NULL, 'Brut', 'Soumis à cotisations', 43);

-- ────────────────────────────────────────────────────────────
-- TRANSPORT SANITAIRE
-- ────────────────────────────────────────────────────────────

INSERT INTO ccn_reglementations (idcc, activite, theme, titre, resume, corps, valeur, source, date_effet, impact, regime_social, ordre) VALUES

('0016', 'sanitaire', 'duree',
 'Décompte du temps de travail des ambulanciers',
 'Le temps payé s''obtient en appliquant un coefficient à l''amplitude : sans lui, aucune paie d''ambulancier n''est juste.',
 'L''activité d''ambulancier alterne interventions et attente. Plutôt que de décompter heure par heure, la branche a retenu un décompte de l''amplitude affectée d''un coefficient minorateur.

Mécanique :
• On mesure l''amplitude de la journée de service.
• On lui applique un coefficient d''équivalence pour obtenir la durée de travail effectif rémunérée.
• Ce coefficient a été relevé par avenants successifs depuis l''accord-cadre du 4 mai 2000, dans le sens d''une réduction de l''écart entre amplitude et temps payé. L''accord du 16 juin 2016 a poursuivi ce relèvement.

Point de vigilance majeur : la valeur du coefficient applicable dépend de la date et du texte étendu en vigueur. C''est la donnée à vérifier en priorité avant tout paramétrage de paie sur cette activité — un coefficient périmé fausse chaque bulletin, chaque mois, pour tout l''effectif roulant.',
 'Amplitude × coefficient',
 'Accord-cadre du 4 mai 2000 ; accord du 16 juin 2016 ; décret n° 2001-679',
 NULL, 'Temps de travail', NULL, 50),

('0016', 'sanitaire', 'primes',
 'Permanences et gardes',
 'Les services de garde ouvrent une indemnisation propre, distincte du salaire des heures travaillées.',
 'Les permanences (nuits, dimanches, jours fériés, gardes préfectorales) donnent lieu à une indemnisation forfaitaire conventionnelle.

En paie :
• Indemnité de permanence : contrepartie de la sujétion, soumise à cotisations et imposable.
• Les interventions réalisées pendant la permanence sont du travail effectif et se rémunèrent en plus.
• Ces indemnités entrent dans l''assiette des congés payés au dixième, dans la comparaison au minimum conventionnel et dans l''assiette de la réduction générale.
• Elles ne relèvent pas des frais professionnels : aucune exonération à ce titre.',
 'Indemnité forfaitaire',
 'Accord-cadre du 4 mai 2000 ; accords transport sanitaire',
 NULL, 'Brut', 'Soumis à cotisations', 51),

('0016', 'sanitaire', 'classification',
 'Coefficients des ambulanciers',
 'Deux degrés selon le diplôme : auxiliaire ambulancier et ambulancier diplômé d''État.',
 'La grille distingue l''ambulancier de degré 1 (auxiliaire ambulancier) de l''ambulancier de degré 2 (titulaire du diplôme d''État), avec des coefficients distincts.

En paie, le passage du degré 1 au degré 2 à l''obtention du diplôme doit être répercuté immédiatement : coefficient, taux horaire garanti, assiette de la prime d''ancienneté et garantie annuelle de rémunération changent tous en même temps. Un décalage produit un rappel sur trois ans.',
 'Degrés 1 et 2',
 'CCN 16, annexe I ; accords transport sanitaire',
 NULL, 'Brut', NULL, 52),

('0016', 'sanitaire', 'frais',
 'Indemnités de repas des ambulanciers',
 'Frais professionnels, exonérés dans les limites URSSAF, hors du brut.',
 'Les personnels ambulanciers en déplacement bénéficient d''indemnités de repas selon un barème conventionnel.

Même grille de lecture que pour les autres activités de la branche :
• Nature de frais professionnels, hors assiette de cotisations dans les limites de l''arrêté du 20 décembre 2002.
• Excédent réintégré à défaut de justificatif.
• Hors assiette congés payés, hors comparaison SMIC, hors réduction générale.
• À ne pas confondre avec l''indemnité de permanence, qui est du salaire.',
 'Frais professionnels',
 'Accords transport sanitaire ; arrêté du 20 décembre 2002',
 NULL, 'Hors bulletin', 'Frais professionnels — exonéré dans les limites URSSAF', 53);

-- ────────────────────────────────────────────────────────────
-- AUXILIAIRES ET LOGISTIQUE
-- ────────────────────────────────────────────────────────────

INSERT INTO ccn_reglementations (idcc, activite, theme, titre, resume, corps, valeur, source, date_effet, impact, regime_social, ordre) VALUES

('0016', 'auxiliaires', 'duree',
 'Durée du travail des sédentaires',
 'Aucun régime d''équivalence : on revient à 35 heures et 151,67 heures mensuelles.',
 'Les personnels des auxiliaires de transport (commissionnaires, transitaires, agents maritimes) et des prestataires logistiques, ainsi que tous les sédentaires de la branche, relèvent de la durée légale de droit commun.

• 35 heures par semaine, 151,67 heures par mois.
• Heures supplémentaires majorées de 25 % de la 36e à la 43e heure, 50 % au-delà (art. L. 3121-36).
• Contingent annuel de droit commun.
• Les conventions de forfait en jours sont possibles pour les cadres autonomes, sous réserve d''un accord collectif le prévoyant et d''un suivi de charge effectif.

L''erreur symétrique de celle des roulants : appliquer 186 heures à un exploitant transport sédentaire parce que l''entreprise est du transport. Le régime d''équivalence est réservé au personnel roulant.',
 '151,67 h',
 'Art. L. 3121-27 et L. 3121-36 du code du travail ; CCN 16 annexes II et III',
 NULL, 'Temps de travail', NULL, 60),

('0016', 'auxiliaires', 'classification',
 'Coefficients employés, techniciens et agents de maîtrise',
 'Annexes II et III : la grille qui couvre l''exploitation, l''administratif et l''encadrement intermédiaire.',
 'Les emplois d''exploitation (agent de quai, agent d''exploitation, déclarant en douane, affréteur), l''administratif et l''encadrement intermédiaire relèvent des annexes II (employés) et III (techniciens et agents de maîtrise).

Ces annexes emportent deux différences majeures avec l''annexe I :
• Barème de prime d''ancienneté par paliers triennaux, jusqu''à 15 %.
• Préavis plus longs.

Le passage employé vers agent de maîtrise change donc simultanément le minimum, l''ancienneté et le préavis. À vérifier lors de toute promotion.',
 'Annexes II et III',
 'CCN 16, annexes II et III',
 NULL, 'Brut', NULL, 61),

('0016', 'auxiliaires', 'minima',
 'Rémunération annuelle garantie des cadres',
 'Les cadres ne relèvent pas d''un taux horaire mais d''un montant annuel par groupe.',
 'L''annexe IV classe les ingénieurs et cadres en groupes, chacun assorti d''une rémunération annuelle garantie.

En paie :
• Le contrôle se fait sur l''année civile, en cumulant la rémunération brute soumise à cotisations, hors frais professionnels et hors sommes non contreparties du travail.
• Proratisation au temps de présence et au taux d''activité.
• Un cadre au forfait jours reste soumis à cette garantie : le forfait affranchit du décompte horaire, pas du minimum conventionnel.
• Le complément éventuel se régularise sur la paie de décembre, avec l''effet mécanique sur le coefficient de la réduction générale de ce mois.',
 'Montant annuel',
 'CCN 16, annexe IV — ingénieurs et cadres',
 NULL, 'Brut', 'Soumis à cotisations', 62);

-- ────────────────────────────────────────────────────────────
-- TRANSPORT DE FONDS
-- ────────────────────────────────────────────────────────────

INSERT INTO ccn_reglementations (idcc, activite, theme, titre, resume, corps, valeur, source, date_effet, impact, regime_social, ordre) VALUES

('0016', 'fonds', 'classification',
 'Emplois du convoyage de fonds',
 'Grille et sujétions propres au transport de fonds et valeurs.',
 'Le transport de fonds relève de dispositions spécifiques au sein de la branche, avec des emplois de convoyeur, garde, chef de bord et opérateur de traitement de valeurs.

Ces emplois s''accompagnent de sujétions particulières (port d''arme, habilitation, formation continue obligatoire) qui se traduisent en paie par des compléments dédiés et des temps de formation rémunérés.',
 'Grille propre',
 'CCN 16, dispositions transport de fonds',
 NULL, 'Brut', NULL, 70),

('0016', 'fonds', 'primes',
 'Primes de sujétion et de risque',
 'Compléments liés au danger et à l''habilitation, intégralement soumis à cotisations.',
 'Le convoyage de fonds ouvre droit à des primes propres : prime de risque, prime de port d''arme, primes liées aux habilitations.

En paie :
• Ce sont des éléments de salaire, sans aucune exonération : cotisées, imposables, dans l''assiette des congés payés au dixième, dans la comparaison au minimum conventionnel et dans l''assiette de la réduction générale.
• Leur caractère parfois variable impose la régularisation annuelle de la réduction générale.
• Ne pas les confondre avec les indemnités d''équipement, qui peuvent relever des frais professionnels si elles couvrent une dépense réelle et justifiée.',
 'Salaire cotisé',
 'CCN 16, dispositions transport de fonds',
 NULL, 'Brut', 'Soumis à cotisations', 71),

('0016', 'fonds', 'duree',
 'Organisation du temps de travail du convoyage',
 'Horaires en équipes, temps d''habillage et de prise d''armes rémunérés.',
 'L''activité s''organise en équipes, avec des temps périphériques qui sont du temps de travail effectif :

• Temps d''habillage et de déshabillage lorsque le port d''une tenue est imposé et que l''habillage se fait sur le lieu de travail (art. L. 3121-3).
• Temps de prise et de remise d''armes, de briefing, de comptage.
• Ces temps entrent dans le décompte hebdomadaire et donc dans le déclenchement des heures supplémentaires.

Leur non-prise en compte est un motif classique de rappel de salaire, majorations comprises, sur trois ans.',
 'Temps effectif',
 'Art. L. 3121-1 et L. 3121-3 du code du travail ; CCN 16',
 NULL, 'Temps de travail', NULL, 72);
