// Histoire des cotisations et prélèvements : anecdotes, acteurs, controverses.
//
// L'explication d'une ligne dit CE QUE FAIT la cotisation ; ce module raconte
// d'où elle vient, qui l'a portée et qui l'a combattue. Les deux restent
// séparés : l'explication est technique et chiffrée (placeholders, calcul du
// mois), l'anecdote est un texte fixe, rattaché au CODE de la ligne.
//
// Transport : `generer_bulletin` ajoute l'anecdote à la fin de l'explication,
// derrière le séparateur `SEPARATEUR` (U+0002). Le front l'en détache (comme le
// détail « aide au poste » derrière U+0001) et l'affiche dans un bloc distinct.
// Aucune structure de données n'est touchée : `LigneCotisation` garde sa forme.
//
// Règle de rédaction — la même que pour les chiffres : rien d'inventé. On ne
// retient que des faits établis (lois, dates, acteurs, votations, controverses
// documentées) ; pas de citation incertaine ni de chiffre approximatif. Un code
// sans anecdote sûre n'en reçoit pas : l'absence vaut mieux qu'une légende.
//
// Langues : français seulement pour l'instant. Une autre langue retombe sur
// « pas d'anecdote » (et non sur le français) — l'explication d'un bulletin
// affiché en anglais ne doit pas basculer de langue en cours de lecture.

/// Séparateur entre l'explication et l'anecdote (caractère de contrôle STX).
pub const SEPARATEUR: char = '\u{2}';

/// Anecdote historique d'une cotisation, ou None.
pub fn anecdote(code: &str, lang: &str) -> Option<&'static str> {
    if lang != "fr" {
        return None;
    }
    // Variantes d'une même cotisation : même histoire.
    let code = match code {
        c if c.starts_with("IT_ADD_REG_") => "IT_ADD_REG",   // une par région (IT_ADD_REG_LO…)
        c if c.starts_with("IT_ESONERO_") => "IT_ESONERO",   // une par année
        "IT_NASPI_TERMINE" => "IT_NASPI",
        "DE_PV_KINDERLOS"  => "DE_PFLEGEVERSICHERUNG",
        "PT_FGCT"          => "PT_FCT",
        "QC_RRQ2"          => "CA_RPC2",
        c => c,
    };
    Some(match code {
        // ─────────────────────────────── France ───────────────────────────────
        "SS_MALADIE" =>
            "La Sécurité sociale naît des ordonnances des 4 et 19 octobre 1945, dans le sillage du programme \
            du Conseil national de la Résistance, « Les Jours heureux » (mars 1944). Le haut fonctionnaire \
            Pierre Laroque en dessine l'architecture ; le ministre communiste du Travail Ambroise Croizat \
            en installe les caisses à marche forcée en 1946. Le projet d'une caisse unique se heurte aux \
            médecins libéraux, à la Mutualité, aux régimes déjà existants et aux cadres, qui refusent d'y \
            être fondus : d'où la mosaïque de régimes qui subsiste aujourd'hui. Le vrai premier jalon est \
            antérieur : les lois sur les assurances sociales de 1928 et 1930.",
        "SS_VIEILLESSE_PLAF" =>
            "La première loi sur les retraites ouvrières et paysannes (5 avril 1910) fixait l'âge de départ \
            à 65 ans, quand peu d'ouvriers l'atteignaient : la CGT la combattit comme « la retraite pour les \
            morts ». La répartition s'impose en 1941, sous Vichy, avec l'allocation aux vieux travailleurs \
            salariés, puis 1945 la généralise. L'âge de 60 ans arrive par l'ordonnance du 26 mars 1982. \
            Depuis, chaque réforme a déplacé un curseur : Balladur (1993, calcul sur les 25 meilleures \
            années au lieu de 10), Fillon (2003), Woerth (2010, 62 ans), Touraine (2014, durée de \
            cotisation), Borne (2023, 64 ans, adoptée par le 49.3 après des mois de manifestations).",
        "SS_VIEILLESSE_DEPLAF" =>
            "Toucher aux retraites est resté la réforme la plus inflammable du pays. En novembre-décembre \
            1995, le plan Juppé sur les \
            régimes spéciaux provoque trois semaines de grèves des transports, et le gouvernement retire son \
            volet retraites.",
        "FAMILLE" =>
            "Les allocations familiales sont nées d'initiatives patronales : pendant la Grande Guerre, des \
            industriels versent un « sursalaire familial », puis s'organisent en caisses de compensation pour \
            qu'aucun employeur ne soit pénalisé d'embaucher des pères de famille. La loi Landry du \
            11 mars 1932 rend l'affiliation obligatoire dans l'industrie et le commerce ; le Code de la \
            famille de 1939 durcit l'orientation nataliste. L'universalité, principe de 1945, a été rompue \
            en 2015 par la modulation des allocations selon les revenus, contre l'avis des associations \
            familiales.",
        "AT_MP" =>
            "Avant 1898, l'ouvrier blessé devait prouver la faute de son patron devant le juge : autant dire \
            presque jamais. Il a fallu dix-huit ans de navette parlementaire, depuis la proposition déposée \
            en 1880 par Martin Nadaud, ancien maçon de la Creuse devenu député, pour arracher le compromis \
            de la loi du 9 avril 1898 : réparation automatique, mais forfaitaire. La loi du 25 octobre 1919 \
            l'étend aux premières maladies professionnelles, comme le saturnisme. Le scandale de l'amiante, \
            interdit en France le 1er janvier 1997, a conduit à créer en 2000 un fonds d'indemnisation dédié, \
            le FIVA.",
        "CSG_DEDUCTIBLE" =>
            "La CSG est l'œuvre de Michel Rocard, Premier ministre, dans la loi de finances pour 1991 : \
            1,1 % sur presque tous les revenus, y compris ceux du capital. Le texte passe par le 49.3 et la \
            motion de censure qui suit ne manque la majorité que de cinq voix. Impôt ou cotisation ? Le \
            débat juridique a duré des années ; la Cour de justice européenne a fini par la traiter comme une \
            cotisation sociale. Créée modeste, elle rapporte aujourd'hui davantage que l'impôt sur le \
            revenu.",
        "CSG_NON_DEDUCTIBLE" =>
            "La hausse de 1,7 point de 2018, qui finançait la suppression des cotisations salariales maladie \
            et chômage, profitait aux actifs mais frappait les retraités sans contrepartie. Leur colère a \
            nourri le mouvement des gilets jaunes : dès décembre 2018, le gouvernement crée un taux \
            intermédiaire pour épargner les pensions modestes.",
        "CRDS" =>
            "Créée par l'ordonnance du 24 janvier 1996 (plan Juppé) pour alimenter la CADES, la caisse chargée \
            d'amortir la dette sociale, la CRDS devait s'éteindre en 2009. Chaque nouvelle vague de déficit, \
            et en dernier lieu la dette liée au Covid, a reculé l'échéance, aujourd'hui fixée à 2033 : \
            l'impôt « temporaire » a franchi le cap des trente ans.",
        "CHOMAGE" =>
            "L'Unédic naît d'un accord signé le 31 décembre 1958 entre patronat et syndicats, sous l'impulsion \
            du général de Gaulle, qui préfère un régime paritaire à une gestion d'État. André Bergeron, \
            patron de Force ouvrière, en fera pendant des décennies le symbole du paritarisme. L'équilibre \
            s'est inversé depuis 2018 : la cotisation salariale supprimée, l'État fixe désormais le cadre \
            des négociations par une lettre de cadrage, et a repris la main par décret quand les partenaires \
            sociaux ne s'entendaient pas.",
        "AGIRC_ARRCO_T1" =>
            "Les cadres ont refusé en 1946-1947 d'être fondus dans le régime général : la convention \
            collective du 14 mars 1947 leur donne leur propre caisse par points, l'AGIRC. Les non-cadres \
            obtiennent l'équivalent par l'accord du 8 décembre 1961, qui fonde l'ARRCO. Les deux fusionnent \
            le 1er janvier 2019 (accord du 30 octobre 2015). Le même accord avait créé un « malus » de 10 % \
            pendant trois ans pour qui partait dès l'âge légal ; très impopulaire, il a été supprimé en 2023.",
        "AGIRC_ARRCO_T2" =>
            "Jusqu'en 2018, le statut cadre se lisait directement sur la retraite : cotisations AGIRC, \
            garantie minimale de points, caisse dédiée. La fusion a effacé ces signes distinctifs, et la \
            définition même du cadre a dû être renégociée par un accord interprofessionnel en 2020.",
        "AGIRC_ARRCO_CEG_T1" =>
            "La CEG a pris en 2019 la suite de l'AGFF, elle-même créée en 2001 pour une raison précise : quand \
            la retraite à 60 ans arrive en 1982, les régimes complémentaires ne suivent pas. Il faut une \
            structure dédiée (l'ASF, en 1983, puis l'AGFF) pour financer le versement des pensions \
            complémentaires sans abattement entre 60 et 65 ans.",
        "PREVOYANCE_CADRE_MIN" =>
            "Cette obligation de 1,50 % remonte à l'article 7 de la convention collective des cadres du \
            14 mars 1947 : une assurance décès financée par l'employeur, contrepartie de l'attachement des \
            cadres à leur régime propre. Quand l'AGIRC a disparu en 2019, l'obligation a failli disparaître \
            avec elle ; l'accord interprofessionnel du 17 novembre 2017 l'a maintenue.",
        "REDUCTION_FILLON" =>
            "Les allègements de cotisations sur les bas salaires commencent avec Édouard Balladur en 1993, \
            s'étendent avec la « ristourne » Juppé, puis avec les aides liées aux 35 heures de Martine Aubry. \
            La loi du 17 janvier 2003, portée par François Fillon, ministre des Affaires sociales, les \
            fusionne en un seul dispositif : d'où le surnom qui lui est resté. Son revers est connu sous le \
            nom de « trappe à bas salaires » : chaque euro d'augmentation près du SMIC fait perdre de \
            l'allègement à l'employeur. Le rapport Bozio-Wasmer (2024) l'a chiffré et a inspiré la \
            refonte de 2026.",
        "ALSACE_MOSELLE_MALADIE" =>
            "Quand l'Alsace et la Moselle sont allemandes, de 1871 à 1918, elles reçoivent les assurances \
            sociales de Bismarck : maladie (1883), accidents (1884), vieillesse (1889). Revenus à la France, \
            leurs habitants refusent de perdre des droits plus avancés que ceux de la « France de \
            l'intérieur » ; la loi du 1er juin 1924 maintient le droit local. Le régime maladie complémentaire \
            en est l'héritier direct, comme les deux jours fériés supplémentaires (Vendredi saint et \
            26 décembre).",
        "AIDE_POSTE_EA" =>
            "La loi du 10 juillet 1987 impose aux entreprises de 20 salariés et plus d'employer au moins 6 % de \
            travailleurs handicapés, sous peine de contribution. La loi du 11 février 2005, grande loi sur \
            le handicap, transforme les anciens « ateliers protégés » en entreprises adaptées : des \
            entreprises du milieu ordinaire, soumises au droit du travail, et non plus des structures \
            médico-sociales.",
        "REDUC_SAL_HS" =>
            "« Travailler plus pour gagner plus » : le slogan de campagne de Nicolas Sarkozy devient la loi \
            TEPA d'août 2007, qui exonère les heures supplémentaires d'impôt et de cotisations. François \
            Hollande supprime l'essentiel du dispositif en 2012 ; Emmanuel Macron le rétablit, avancé au \
            1er janvier 2019 sous la pression des gilets jaunes. Trois changements de cap en douze ans pour \
            la même ligne de bulletin.",
        "DFP_HS" =>
            "La déduction forfaitaire patronale est une survivance de la loi TEPA de 2007 : quand la réforme \
            de 2012 a supprimé les exonérations, elle a été conservée pour les entreprises de moins de \
            20 salariés. La loi de finances rectificative de l'été 2022 l'a étendue aux entreprises de 20 à \
            249 salariés, avec un montant plus faible.",
        "FPT_CNRACL" =>
            "La CNRACL a été créée en 1945 et reste gérée par la Caisse des dépôts, depuis Bordeaux. Longtemps \
            excédentaire grâce à une démographie favorable, elle a dû \
            reverser pendant des décennies une partie de ses ressources à d'autres régimes au titre de la \
            « compensation démographique ». Le vieillissement de ses propres effectifs l'a fait basculer \
            dans le déficit, d'où les hausses du taux employeur supportées par les collectivités et les \
            hôpitaux.",

        // ──────────────────────────────── Suisse ───────────────────────────────
        "CH_AVS" =>
            "L'AVS figurait déjà parmi les revendications de la grève générale de novembre 1918. Un article \
            constitutionnel est accepté en 1925, mais la première loi est rejetée en 1931. Il faut attendre \
            la votation du 6 juillet 1947 : environ 80 % de oui, avec une participation record. Plus \
            récemment, le peuple a relevé l'âge de la retraite des femmes à 65 ans (AVS 21, septembre 2022, \
            à une courte majorité) puis accepté en mars 2024 une 13ᵉ rente, première initiative de gauche sur \
            les assurances sociales à passer la barre.",
        "CH_AI" =>
            "Inscrite dans la Constitution dès 1925 avec l'AVS, l'assurance invalidité n'entre en vigueur \
            qu'en 1960. Face à l'envolée du nombre de rentes, la 5ᵉ révision (2008) a érigé en principe que \
            « la réadaptation prime la rente » : l'AI finance d'abord le retour à l'emploi.",
        "CH_APG" =>
            "Les APG sont nées en 1940 pour compenser la perte de salaire des soldats mobilisés. C'est ce \
            régime qui a servi de véhicule à l'assurance maternité, refusée à plusieurs reprises par le \
            peuple (1984, 1987, 1999) avant d'être acceptée en septembre 2004. Le congé paternité de deux \
            semaines a suivi le même chemin, approuvé en votation en septembre 2020.",
        "CH_AC" =>
            "Jusqu'au milieu des années 1970, l'assurance-chômage était facultative en Suisse. Le choc \
            pétrolier et la vague de licenciements qui l'a suivi ont conduit à la rendre obligatoire : \
            article constitutionnel de 1976, loi (LACI) de 1982.",
        "CH_LPP" =>
            "Le système des « trois piliers » a été inscrit dans la Constitution par la votation de décembre \
            1972, préféré à une initiative du Parti du travail qui voulait une retraite populaire unique et \
            généreuse. Il a fallu ensuite plus de dix ans pour que la loi LPP entre en vigueur, en 1985.",
        "CH_AAP" =>
            "La loi sur l'assurance maladie et accidents est acceptée en votation en février 1912. Elle crée la \
            SUVA, établissement public installé à Lucerne, qui assure depuis 1918 les accidents dans \
            l'industrie et les métiers à risque.",
        "CH_AANP" =>
            "Particularité suisse : l'accident survenu en dehors du travail, à ski ou en bricolant, est couvert \
            par l'assurance du salarié. Héritée de la loi de 1911 et de la SUVA, cette couverture a été \
            généralisée à tous les secteurs par la loi sur l'assurance-accidents de 1981.",
        "CH_IS" =>
            "À Genève, l'impôt à la source des frontaliers français est au cœur d'un accord du 29 janvier 1973 : \
            le canton impose les salaires sur place et reverse aux départements de l'Ain et de la Haute-Savoie \
            une compensation calculée sur la masse salariale des frontaliers. Huit autres cantons \
            relèvent d'un accord de 1983, par lequel le frontalier est imposé en France.",

        // ────────────────────────────── Luxembourg ─────────────────────────────
        "LU_AM" =>
            "Le Luxembourg adopte l'assurance maladie obligatoire des ouvriers dès 1901, sur le modèle de \
            Bismarck. Pendant plus d'un siècle, ouvriers et employés privés relèvent de caisses distinctes ; \
            le « statut unique » du 1er janvier 2009 les réunit en une seule Caisse nationale de santé.",
        "LU_ME" =>
            "La Mutualité des employeurs est née avec le statut unique de 2009 : en supprimant la distinction \
            entre ouvriers et employés, la réforme a généralisé le maintien du salaire en cas de maladie. Pour \
            que les petites entreprises n'en supportent pas seules le coût, les employeurs le mutualisent.",
        "LU_AP" =>
            "Comme ses voisins, le Luxembourg a bâti ses retraites sur l'exemple allemand dès le début du \
            XXᵉ siècle. Le pays a longtemps alimenté une réserve considérable, gérée par un fonds de \
            compensation, grâce à la forte proportion de travailleurs frontaliers qui cotisent sans être \
            encore pensionnés.",

        // ────────────────────────────── Allemagne ──────────────────────────────
        "DE_KRANKENVERSICHERUNG" =>
            "La loi de 1883 sur l'assurance maladie des ouvriers est la première des assurances sociales de \
            Bismarck. Le chancelier l'annonce dans le message impérial du 17 novembre 1881 ; le calcul est \
            politique : après avoir interdit les organisations socialistes (loi de 1878), il veut détacher \
            les ouvriers de la social-démocratie en leur offrant une protection venue de l'État.",
        "DE_UNFALLVERSICHERUNG" =>
            "Deuxième pilier bismarckien, l'assurance accidents date de 1884. Les Berufsgenossenschaften, \
            corporations d'employeurs par branche qui la gèrent encore, sont presque aussi anciennes que le \
            régime lui-même.",
        "DE_RENTENVERSICHERUNG" =>
            "L'assurance invalidité et vieillesse de 1889 fixait l'âge de la pension à 70 ans. La grande \
            réforme de 1957, sous Adenauer, indexe les pensions sur les salaires. En 1986, le ministre du \
            Travail Norbert Blüm colle sur les affiches « Die Rente ist sicher » (« la retraite est sûre ») ; \
            la phrase est devenue en Allemagne l'exemple même de la promesse politique ironisée.",
        "DE_ARBEITSLOSENVERSICHERUNG" =>
            "L'assurance chômage allemande naît en 1927, sous la République de Weimar. Trois ans plus tard, un \
            désaccord sur son financement fait tomber la dernière grande coalition de Weimar, en mars 1930 : \
            une querelle sur quelques dixièmes de point de cotisation, au seuil de la crise qui emportera la \
            République.",
        "DE_PFLEGEVERSICHERUNG" =>
            "Cinquième branche de la sécurité sociale, l'assurance dépendance est créée en 1995 par Norbert \
            Blüm. Pour compenser le coût pour les employeurs, un jour férié est supprimé : le Buß- und Bettag. \
            Seule la Saxe l'a conservé, et ses salariés paient en échange une part plus élevée de la \
            cotisation. La surcotisation des personnes sans enfant découle d'une décision de la Cour \
            constitutionnelle de 2001.",
        "DE_LOHNSTEUER" =>
            "La retenue de l'impôt sur les salaires par l'employeur date de la réforme financière de Matthias \
            Erzberger, en 1920, qui centralise l'impôt sur le revenu au niveau du Reich. Erzberger, déjà \
            détesté par la droite nationaliste pour avoir signé l'armistice de 1918, est assassiné en 1921.",
        "DE_SOLI" =>
            "Instauré en 1991 pour financer la réunification sous le chancelier Kohl, le Soli devait être \
            provisoire. Supprimé en 2021 pour environ 90 % des contribuables, il subsiste pour les plus hauts \
            revenus ; la Cour constitutionnelle a jugé en mars 2025 que ce maintien restait conforme à la \
            Loi fondamentale.",
        "DE_KIRCHENSTEUER" =>
            "L'impôt d'église est la contrepartie historique des sécularisations de 1803, quand les biens de \
            l'Église furent transférés aux princes. La Constitution de Weimar (1919) le garantit, et la Loi \
            fondamentale de 1949 en a repris l'article. L'État le collecte avec l'impôt sur le salaire ; on \
            y échappe en quittant officiellement son Église, démarche que des centaines de milliers \
            d'Allemands accomplissent chaque année.",

        // ──────────────────────────────── Autriche ─────────────────────────────
        "AT_SV" =>
            "L'Autriche-Hongrie adopte l'assurance accidents (1887) et l'assurance maladie (1888) dans la \
            foulée de l'Allemagne. Le droit actuel repose sur l'ASVG, la loi générale de sécurité sociale \
            de 1955, l'année même où le pays recouvre sa pleine souveraineté.",
        "AT_LOHNSTEUER" =>
            "Particularité autrichienne : les 13ᵉ et 14ᵉ mois de salaire, versés en été et en fin d'année, \
            bénéficient d'un taux d'imposition réduit forfaitaire. Cet avantage, ancré de longue date dans \
            les conventions collectives, est l'un des acquis les plus défendus du pays.",

        // ───────────────────────────────── Italie ──────────────────────────────
        "IT_IVS" =>
            "L'ancêtre de l'INPS est une caisse nationale de prévoyance créée en 1898, d'adhésion volontaire. \
            La réforme Dini de 1995 fait basculer l'Italie vers un calcul « contributif », fondé sur les \
            cotisations versées. En décembre 2011, en pleine crise des dettes, la ministre Elsa Fornero \
            annonce le relèvement de l'âge et le gel de l'indexation des pensions et fond en larmes en \
            pleine conférence de presse : l'image a fait le tour du pays.",
        "IT_TFR" =>
            "Le TFR a remplacé en 1982 l'ancienne indemnité d'ancienneté. Ce salaire différé, provisionné par \
            l'employeur, a longtemps servi de financement bon marché aux entreprises italiennes. Depuis \
            2007, le salarié qui ne dit rien voit son TFR orienté vers un fonds de pension (« silence vaut \
            accord ») ; beaucoup ont choisi de le garder dans l'entreprise.",
        "IT_IRPEF" =>
            "L'IRPEF naît de la grande réforme fiscale de 1973-1974. À sa création, elle comptait 32 tranches, \
            de 10 % à 72 %. Les réformes successives n'en ont laissé que quelques-unes ; réduire le nombre de \
            tranches, voire instaurer une « flat tax », est devenu un marqueur politique de la droite \
            italienne.",
        "IT_ADD_REG" =>
            "L'addizionale regionale est créée en 1997, avec l'IRAP, par le ministre des Finances Vincenzo \
            Visco, dans le cadre du « fédéralisme fiscal » : les régions financent leur système de santé par \
            un impôt dont elles fixent le taux. Résultat, le même salaire n'est pas imposé de la même façon à \
            Milan et à Naples.",
        "IT_INAIL" =>
            "La loi du 17 mars 1898 rend obligatoire l'assurance des ouvriers de l'industrie contre les \
            accidents, la même année que la loi française. L'INAIL, institut unique, est créé en 1933.",
        "IT_NASPI" =>
            "La NASpI est l'un des volets du Jobs Act de Matteo Renzi (2015), qui a aussi assoupli la \
            protection contre les licenciements prévue par le fameux article 18 du Statut des travailleurs \
            de 1970, au prix d'une rupture durable avec la CGIL.",
        "IT_MATERNITA" =>
            "La loi 1204 de 1971 sur la protection des mères qui travaillent a été l'une des grandes conquêtes \
            des années de mobilisation sociale qui ont suivi l'« automne chaud » de 1969. Le congé de \
            paternité obligatoire n'est apparu qu'en 2012, pour un jour seulement.",
        "IT_BONUS_CUNEO" =>
            "Le « cuneo fiscale », l'écart entre ce que coûte un salarié et ce qu'il touche, est une obsession \
            italienne. Le « bonus 80 euros » de Matteo Renzi (2014) a ouvert une série de dispositifs que \
            chaque gouvernement a renommés et prolongés : Draghi, puis Meloni l'ont élargi et en ont changé \
            la forme.",
        "IT_ESONERO" =>
            "L'exonération de cotisations salariales a été créée par le gouvernement Draghi en 2022 face à \
            l'inflation, puis amplifiée par le gouvernement Meloni. Reconduite d'année en année, elle a été \
            transformée à partir de 2025 en avantage fiscal.",
        "IT_FONDO_GARANZIA" =>
            "Le fonds de garantie du TFR a été créé par la même loi de 1982 que le TFR lui-même : sans lui, un \
            salaire différé pendant des années pouvait disparaître avec la faillite de l'employeur.",

        // ──────────────────────────────── Espagne ──────────────────────────────
        "ES_CC" =>
            "La première loi sociale espagnole est la loi Dato de 1900 sur les accidents du travail. L'Institut \
            national de prévoyance est fondé en 1908 ; mais la Sécurité sociale moderne ne naît qu'avec la \
            loi de bases de 1963, entrée en vigueur en 1967, sous le franquisme. En 1995, le « Pacte de \
            Tolède » engage tous les partis à sortir les retraites de la bataille électorale.",
        "ES_MEI" =>
            "Le mécanisme d'équité intergénérationnelle est l'œuvre du ministre José Luis Escrivá (2021-2023). \
            Il remplace le « facteur de soutenabilité » de la réforme Rajoy de 2013, qui aurait réduit les \
            pensions avec l'allongement de la vie et qui n'a jamais été appliqué.",
        "ES_FOGASA" =>
            "Le FOGASA est créé en 1976, pendant la transition démocratique, pour garantir aux salariés le \
            paiement de leurs salaires en cas d'insolvabilité de l'employeur.",
        "ES_DESEMPLEO" =>
            "La réforme du travail de 2021, négociée par la ministre Yolanda Díaz avec syndicats et patronat, a \
            fait du CDI la règle dans un pays longtemps champion européen des contrats temporaires. Les \
            cotisations chômage plus lourdes sur les contrats temporaires en sont un instrument.",

        // ──────────────────────────────── Portugal ─────────────────────────────
        "PT_SS" =>
            "Sous l'Estado Novo de Salazar, la prévoyance est organisée en caisses corporatistes par \
            profession (loi de 1935). Après la révolution des Œillets du 25 avril 1974, ces caisses sont \
            unifiées dans une sécurité sociale universelle, que consacre la Constitution de 1976.",
        "PT_IRS" =>
            "L'IRS est entré en vigueur le 1er janvier 1989 et a remplacé une mosaïque d'impôts cédulaires. Le \
            Portugal a depuis multiplié les régimes dérogatoires, comme celui des « résidents non habituels » \
            (2009), qui attirait retraités et cadres étrangers avant d'être fermé aux nouveaux arrivants en \
            2024.",
        "PT_FCT" =>
            "Le Fonds de compensation du travail et son fonds de garantie ont été créés en 2013, sous le programme d'assistance de la \
            « troïka » (FMI, BCE, Commission européenne), en contrepartie de la baisse des indemnités de \
            licenciement.",

        // ──────────────────────────────── Belgique ─────────────────────────────
        "BE_ONSS_SAL" =>
            "La sécurité sociale belge naît d'un « pacte social » négocié en secret pendant l'Occupation entre \
            patrons et syndicalistes. L'arrêté-loi du 28 décembre 1944 crée l'ONSS, qui perçoit depuis toutes \
            les cotisations en un seul lieu.",
        "BE_ONSS_PAT" =>
            "La concertation sociale belge repose depuis 1944 sur l'idée que patrons et syndicats gèrent \
            ensemble la sécurité sociale. L'indexation automatique des salaires, rare en Europe, en est un \
            autre pilier : elle est régulièrement contestée par les employeurs au nom de la compétitivité.",
        "BE_PP" =>
            "La réforme fiscale de 1962 institue l'impôt des personnes physiques et le précompte \
            professionnel retenu par l'employeur. La Belgique figure depuis longtemps parmi les pays de \
            l'OCDE où le travail est le plus taxé : chaque gouvernement annonce un « tax shift » pour \
            corriger ce trait.",
        "BE_BONUS_EMPLOI" =>
            "Le bonus à l'emploi, créé en 2005, répond à un problème précis : pour un bas salaire, le gain net \
            à reprendre un travail plutôt qu'à percevoir une allocation était parfois quasi nul. On l'appelle \
            le « piège à l'emploi ».",
        "BE_RED_STRUCT" =>
            "La réduction structurelle est née en 2004 de la fusion de plusieurs allègements de cotisations \
            patronales. Elle est l'équivalent belge de la réduction Fillon, avec la même logique dégressive.",

        // ──────────────────────────────── Royaume-Uni ──────────────────────────
        "UK_NI_SAL" =>
            "La National Insurance naît du National Insurance Act de 1911, porté par David Lloyd George : il \
            vend la réforme par un slogan resté célèbre, « ninepence for fourpence » (neuf pence de \
            prestations pour quatre de cotisation). Le rapport Beveridge de 1942 et la loi de 1946 du \
            gouvernement Attlee en font le socle de l'État-providence britannique.",
        "UK_NI_PAT" =>
            "La hausse de la National Insurance patronale à 15 % à partir d'avril 2025, annoncée dans le \
            premier budget de Rachel Reeves, première femme chancelière de l'Échiquier, a été la mesure la \
            plus contestée par les entreprises britanniques de ce budget.",
        "UK_INCOME_TAX" =>
            "L'impôt sur le revenu britannique a été inventé par William Pitt le Jeune en 1799 pour financer \
            la guerre contre la France révolutionnaire. Aboli en 1816, rétabli en 1842 par Robert Peel, il \
            devait toujours être temporaire. La retenue à la source, le PAYE, est introduite en 1944.",

        // ──────────────────────────────── Irlande ──────────────────────────────
        "IE_USC" =>
            "L'Universal Social Charge a été introduite par le budget 2011, au plus fort de la crise bancaire \
            irlandaise et du plan de sauvetage européen, en remplacement de deux prélèvements antérieurs. \
            Pensée comme une mesure d'urgence, elle est restée.",
        "IE_PRSI" =>
            "Le PRSI actuel date de 1979. La hausse progressive de ses taux, décidée à partir de 2024, finance \
            les retraites face au vieillissement de la population, après l'abandon, sous la pression \
            populaire, du projet de relever l'âge de la pension à 67 ans.",

        // ──────────────────────────────── Pays-Bas ─────────────────────────────
        "NL_LOONHEFFING" =>
            "La pension de base AOW, dont les cotisations sont incluses dans la loonheffing, a été instaurée en \
            1957 par le Premier ministre Willem Drees. Des générations de retraités ont dit « trekken van \
            Drees » (« toucher du Drees ») pour parler de leur pension.",
        "NL_ZVW" =>
            "La loi sur l'assurance santé de 2006, portée par le ministre Hans Hoogervorst, a supprimé la \
            distinction entre caisses publiques et assurance privée : tous les résidents souscrivent une \
            assurance de base auprès d'assureurs privés en concurrence, un modèle unique en Europe.",
        "NL_AOF" =>
            "L'ancienne loi d'invalidité, la WAO de 1967, a été victime de son succès : au début des années \
            1990, près d'un million de Néerlandais en bénéficiaient, et le Premier ministre Ruud Lubbers \
            parlait d'un pays « malade ». La WIA l'a remplacée en 2006 en mettant l'accent sur la capacité de \
            travail restante.",

        // ─────────────────────────────── Scandinavie ───────────────────────────
        "SE_SKATT" =>
            "En 1976, Astrid Lindgren, la créatrice de Fifi Brindacier, découvre que les règles fiscales la \
            conduisent à un taux marginal supérieur à 100 %. Elle publie un conte satirique, « Pomperipossa \
            au pays de Monismanie ». Le débat qu'il déclenche contribue la même année à la défaite des \
            sociaux-démocrates, au pouvoir depuis 44 ans.",
        "SE_ARBETSGIVARAVGIFT" =>
            "La réforme des retraites de 1994-1999, votée par cinq partis, a créé un système par comptes \
            notionnels imité dans plusieurs pays. Chaque année, les Suédois reçoivent une enveloppe orange \
            récapitulant leurs droits : « orange kuvertet » est devenu un symbole national.",
        "DK_AM" =>
            "L'arbejdsmarkedsbidrag (contribution au marché du travail) a été créé en 1994 par la réforme \
            fiscale du gouvernement social-démocrate de Poul Nyrup Rasmussen. Le Danemark finance l'essentiel \
            de sa protection sociale par l'impôt plutôt que par des cotisations, ce qui explique l'un des taux \
            de prélèvement sur le revenu les plus élevés au monde.",
        "DK_ATP" =>
            "L'ATP, pension complémentaire obligatoire, a été instaurée en 1964. Sa cotisation forfaitaire, et \
            non proportionnelle au salaire, en fait une curiosité parmi les régimes de retraite européens.",
        "FI_TYEL" =>
            "La Finlande a mis en place en 1962 la retraite des salariés du privé, gérée par des sociétés \
            d'assurance privées sous mandat public, un modèle original de gestion décentralisée d'un régime \
            obligatoire.",

        // ──────────────────────────────── Pays baltes ──────────────────────────
        "EE_TULUMAKS" =>
            "En 1994, sous le jeune Premier ministre Mart Laar, l'Estonie devient l'un des premiers pays \
            d'Europe à adopter un impôt sur le revenu à taux unique. L'exemple a inspiré toute l'Europe \
            centrale et orientale dans les années 2000.",
        "EE_KOGUMISPENSION" =>
            "Obligatoire pour les jeunes générations depuis 2002, le deuxième pilier est devenu facultatif \
            en 2021 à l'initiative du parti Isamaa. Des dizaines de milliers d'Estoniens en sont sortis pour \
            récupérer leur épargne.",
        "EE_SOTSIAALMAKS" =>
            "La charge sociale estonienne de 33 % est payée intégralement par l'employeur et finance à la fois \
            la retraite et la santé. Le pays a fait le choix d'un prélèvement unique et lisible plutôt que \
            d'une série de cotisations.",
        "LT_SODRA" =>
            "En 2019, la Lituanie a transféré presque toutes les cotisations patronales sur le salarié, tout \
            en relevant les salaires bruts de près de 29 % pour compenser : le salaire net ne changeait pas, \
            mais le bulletin rendait visible le coût réel de la protection sociale.",
        "LV_IIN" =>
            "La Lettonie a abandonné en 2018 son impôt à taux unique au profit d'un barème progressif, à \
            rebours de la tendance qui avait marqué la région depuis les années 1990.",

        // ──────────────────────────── Europe centrale ──────────────────────────
        "PL_EMERYTALNE" =>
            "La réforme de 1999 a créé des fonds de pension privés obligatoires, les OFE. En 2014, le \
            gouvernement de Donald Tusk transfère à l'assurance publique ZUS environ la moitié de leurs \
            actifs, pour alléger la dette publique : l'une des plus spectaculaires marches arrière en \
            matière de retraites par capitalisation en Europe.",
        "PL_ZDROWOTNE" =>
            "Le « Polski Ład » (Nouvel ordre polonais), réforme fiscale de 2022 du gouvernement PiS, a supprimé \
            la déductibilité de la cotisation santé de l'impôt. Son entrée en vigueur chaotique a obligé à \
            corriger dans l'urgence des bulletins de janvier 2022 où certains salaires nets avaient baissé.",
        "CZ_DAN" =>
            "Jusqu'en 2020, l'impôt tchèque était calculé sur un « salaire super-brut », qui ajoutait au brut \
            les cotisations patronales. Cette curiosité, introduite en 2008, a été supprimée en 2021.",
        "SK_DAN" =>
            "En 2004, la Slovaquie de Ivan Mikloš adopte un taux unique de 19 % sur les revenus, les sociétés et \
            la TVA, devenant la vitrine européenne de la « flat tax ». Le gouvernement Fico réintroduit une \
            tranche à 25 % en 2013.",
        "HU_SZJA" =>
            "La Hongrie de Viktor Orbán a instauré en 2011 un impôt sur le revenu à taux unique de 16 %, abaissé \
            à 15 % en 2016. Politique nataliste oblige, les mères de quatre enfants en sont exonérées depuis \
            2020 ; les moins de 25 ans le sont depuis 2022, dans une certaine limite.",
        "HU_SZOCHO" =>
            "La contribution sociale patronale hongroise a été abaissée pas à pas, de 27 % en 2016 à 13 % en \
            2022, dans le cadre d'accords salariaux avec les partenaires sociaux : baisse des charges contre \
            hausse du salaire minimum.",
        "RO_CAS" =>
            "En 2018, la Roumanie a transféré presque toutes les cotisations sociales de l'employeur vers le \
            salarié, en exigeant que les salaires bruts soient relevés d'autant. La mesure, surnommée \
            « révolution fiscale », explique pourquoi le salarié roumain supporte l'essentiel des cotisations \
            sur son bulletin.",
        "RO_IMPOZIT" =>
            "La Roumanie a adopté un impôt à taux unique en 2005 (16 %), puis l'a abaissé à 10 % en 2018, l'un \
            des taux les plus bas de l'Union européenne.",
        "BG_DANAK" =>
            "Avec son impôt à taux unique de 10 %, instauré en 2008, la Bulgarie applique l'un des plus bas \
            impôts sur le revenu de l'Union européenne. Le 1er janvier 2026, elle a adopté l'euro, ce qui a \
            imposé de convertir tous les plafonds de cotisation.",
        "HR_POREZ" =>
            "La réforme fiscale de 2024 a supprimé le « prirez », surtaxe communale sur l'impôt sur le revenu, \
            et laissé aux villes le soin de fixer elles-mêmes leurs taux d'impôt sur le revenu dans une \
            fourchette légale.",
        "GR_EFKA" =>
            "L'EFKA a été créée en 2017 pour réunir une multitude de caisses professionnelles, dont l'IKA des \
            salariés du privé. Pendant la crise des dettes souveraines, les pensions grecques ont été réduites \
            à de nombreuses reprises en application des mémorandums signés avec les créanciers.",
        "CY_GESY" =>
            "Chypre n'a disposé d'un système de santé universel qu'en 2019, avec le lancement du GESY, attendu \
            depuis une loi de 2001. Jusque-là, une grande partie des soins relevait du paiement direct ou de \
            l'assurance privée.",

        // ───────────────────────────── Micro-États ─────────────────────────────
        "AD_IRPF" =>
            "L'Andorre n'a connu aucun impôt sur le revenu des personnes physiques jusqu'en 2015. Son \
            introduction, avec un taux maximal de 10 %, fait partie des engagements pris par la principauté \
            pour sortir des listes de paradis fiscaux et négocier avec l'Union européenne.",
        "MC_CAR" =>
            "Monaco ne perçoit pas d'impôt sur le revenu depuis que le prince Charles III l'a aboli en 1869, \
            grâce aux recettes du casino. Seuls les Français y échappent : après la crise de 1962, pendant \
            laquelle le général de Gaulle fait installer des contrôles douaniers à la frontière, la convention \
            fiscale de 1963 les soumet à l'impôt français.",

        // ──────────────────────────────── Amérique du Nord ─────────────────────
        "US_SS" =>
            "Le Social Security Act est signé par Franklin D. Roosevelt le 14 août 1935, en pleine Grande \
            Dépression. La première pensionnée, Ida May Fuller, institutrice du Vermont, a cotisé moins de \
            25 dollars ; elle a vécu jusqu'à 100 ans et touché près de 23 000 dollars de pensions.",
        "US_MEDICARE" =>
            "Medicare est créé en 1965 par Lyndon B. Johnson dans le cadre de la « Great Society ». Il signe la \
            loi à Independence (Missouri), en présence de l'ancien président Harry Truman, dont le projet \
            d'assurance maladie avait échoué : Truman reçoit la première carte Medicare.",
        "US_ADD_MEDICARE" =>
            "La surtaxe de 0,9 % sur les hauts revenus a été introduite en 2013 par l'Affordable Care Act, \
            l'« Obamacare », pour financer la réforme de l'assurance maladie.",
        "US_FUTA" =>
            "Le chômage fédéral fait partie du Social Security Act de 1935. Son mécanisme de crédit, qui ramène \
            un taux de 6 % à 0,6 % pour les employeurs cotisant à un régime d'État, a été conçu pour pousser \
            chaque État à créer sa propre assurance chômage.",
        "US_IMPOT_FED" =>
            "L'impôt fédéral sur le revenu n'a été rendu possible que par le 16ᵉ amendement de 1913, après que \
            la Cour suprême l'avait jugé inconstitutionnel en 1895. La retenue à la source est instaurée en \
            1943, pour financer la guerre, sur une idée de l'économiste Beardsley Ruml.",
        "US_IMPOT_STATE" =>
            "La surtaxe californienne de 1 % au-delà d'un million de dollars de revenu a été créée par la \
            Proposition 63, adoptée par référendum en 2004 pour financer les services de santé mentale. La \
            Californie pratique ainsi le taux marginal d'impôt d'État le plus élevé des États-Unis.",
        "US_CA_SDI" =>
            "La Californie a été en 2004 le premier État américain à instaurer un congé familial payé, financé \
            par cette cotisation. Les États-Unis restent le seul pays riche sans congé maternité payé au \
            niveau fédéral.",
        "CA_RPC2" =>
            "La bonification du régime de pensions a été convenue en 2016 entre Ottawa et les provinces, \
            première extension majeure depuis sa création ; elle est mise en œuvre progressivement à partir \
            de 2019, et le Québec a bonifié son propre régime selon le même calendrier.",
        "ON_IMPOT_PROV" =>
            "Contrairement au Québec, l'Ontario confie la perception de son impôt sur le revenu à l'Agence du \
            revenu du Canada, en vertu d'un accord de perception fiscale : le salarié ontarien remplit une \
            seule déclaration.",
        "CA_RPC" =>
            "Le Régime de pensions du Canada est créé en 1965 sous le gouvernement de Lester B. Pearson. Le \
            Québec de Jean Lesage refuse d'y adhérer et crée son propre régime ; les réserves québécoises \
            financent la Caisse de dépôt et placement du Québec, devenue l'un des plus grands investisseurs \
            institutionnels du pays.",
        "CA_AE" =>
            "Une première loi fédérale sur l'assurance chômage, adoptée en 1935 par le gouvernement Bennett, \
            est invalidée par les tribunaux au nom du partage des compétences. Il faut modifier la \
            Constitution en 1940 pour que l'assurance chômage fédérale voie le jour.",
        "CA_IMPOT_FED" =>
            "L'impôt fédéral sur le revenu est introduit en 1917 par la Loi de l'impôt de guerre sur le \
            revenu, présentée comme une mesure temporaire pour financer la Première Guerre mondiale.",
        "QC_RRQ" =>
            "Le RRQ est le fruit de la « Révolution tranquille » : en 1964-1965, le gouvernement de Jean \
            Lesage négocie avec Ottawa le droit d'avoir son propre régime. Les cotisations accumulées donnent \
            naissance, en 1965, à la Caisse de dépôt et placement du Québec.",
        "QC_RQAP" =>
            "Le Québec a obtenu de gérer ses propres prestations parentales par une entente avec Ottawa en \
            2005, au terme d'un long bras de fer ; la Cour suprême reconnaissait pourtant la même année la \
            compétence fédérale sur ces prestations. Le RQAP, lancé en 2006, a créé des semaines réservées \
            aux pères, qui ont fortement augmenté leur recours au congé.",
        "QC_IMPOT_PROV" =>
            "Le Québec est la seule province à percevoir elle-même son impôt sur le revenu. Maurice Duplessis \
            crée cet impôt provincial en 1954, pour affirmer l'autonomie fiscale du Québec face à Ottawa.",
        "QC_FSS" =>
            "L'assurance maladie québécoise entre en vigueur en novembre 1970. Elle suscite une grève des \
            médecins spécialistes en octobre 1970, en pleine crise d'Octobre, que l'Assemblée nationale fait \
            cesser par une loi spéciale.",
        "MX_IMSS" =>
            "L'Institut mexicain de sécurité sociale est créé en 1943 sous le président Manuel Ávila Camacho. \
            La moitié environ des travailleurs mexicains, employés dans l'économie informelle, restent \
            aujourd'hui en dehors de sa couverture.",
        "MX_INFONAVIT" =>
            "L'INFONAVIT, fonds de logement des travailleurs, est créé en 1972 sous le président Luis \
            Echeverría. Il est devenu le premier prêteur hypothécaire d'Amérique latine.",
        "MX_RETIRO" =>
            "Le Mexique a créé en 1992 un système d'épargne retraite individuelle, puis en 1997 les Afores, \
            gestionnaires privés des comptes des salariés, sur le modèle chilien.",

        // ────────────────────────────── Amérique du Sud ─────────────────────────
        "BR_INSS" =>
            "La prévoyance sociale brésilienne date de la loi Eloy Chaves de 1923, qui crée une caisse de \
            retraite pour les cheminots. L'INSS, institut unique, est créé en 1990.",
        "BR_FGTS" =>
            "Le FGTS est créé en 1966, sous le régime militaire, pour remplacer la stabilité de l'emploi \
            garantie aux salariés après dix ans d'ancienneté. Les employeurs gagnaient la liberté de \
            licencier ; les salariés, un capital mobilisable pour acheter un logement.",

        // ───────────────────────────────── Asie ────────────────────────────────
        "JP_KENPO" =>
            "La loi japonaise sur l'assurance maladie des salariés est adoptée en 1922, et appliquée à partir \
            de 1927. L'assurance maladie universelle, étendue à toute la population, est réalisée en 1961.",
        "JP_KOSEI" =>
            "L'assurance retraite des salariés naît en 1942 pendant la guerre. En 2007, on découvre qu'environ \
            50 millions de dossiers de cotisation ne peuvent être rattachés à personne : le scandale des \
            « retraites disparues » contribue à la défaite du premier gouvernement de Shinzō Abe aux \
            sénatoriales de 2007.",
        "JP_ROUSAI" =>
            "L'assurance accidents du travail japonaise est créée en 1947, la même année que la loi sur les \
            normes du travail, dans le cadre des réformes de l'après-guerre.",
        "JP_KAIGO" =>
            "L'assurance soins de longue durée, entrée en vigueur en 2000, a été l'une des réponses du pays au \
            vieillissement le plus rapide du monde. On y cotise à partir de 40 ans.",
        "JP_KOYO" =>
            "L'assurance emploi de 1974 a remplacé l'assurance chômage créée en 1947. Elle finance aussi des \
            aides au maintien dans l'emploi, héritage d'une culture de l'emploi à vie.",
        "JP_SHOTOKUZEI" =>
            "Depuis 2013 et jusqu'en 2037, l'impôt sur le revenu est majoré d'une surtaxe spéciale de \
            reconstruction de 2,1 %, qui finance la reconstruction après le séisme et le tsunami du \
            11 mars 2011.",
        "JP_JUMINZEI" =>
            "La taxe résidentielle est calculée sur les revenus de l'année précédente. Les jeunes diplômés ne \
            la paient donc pas pendant leur première année de travail et découvrent en juin de leur \
            deuxième année une baisse de leur salaire net.",
        "CN_GONGJIJIN" =>
            "Le fonds de logement est inspiré du Central Provident Fund de Singapour. Shanghai l'expérimente en \
            1991, dans le cadre de la réforme qui met fin au logement attribué par l'unité de travail.",
        "CN_IIT" =>
            "L'impôt sur le revenu chinois est instauré en 1980 avec un seuil de 800 yuans par mois, qui le \
            réservait en pratique aux étrangers. La réforme de 2018 relève le seuil à 5 000 yuans et introduit \
            des déductions pour l'éducation des enfants, le logement ou les parents âgés.",
        "CN_YANGLAO" =>
            "La réforme des années 1990 combine un fonds commun et des comptes individuels. En 2024, la Chine \
            a engagé le premier relèvement progressif de son âge de départ à la retraite depuis les années \
            1950.",
        "KR_NPS" =>
            "La pension nationale coréenne naît en 1988. Face à l'épuisement prévu de ses réserves, le \
            Parlement a voté en mars 2025 une réforme qui porte le taux de cotisation progressivement de \
            9 % à 13 %, première hausse depuis 1998.",
        "KR_NHI" =>
            "La Corée du Sud a réalisé la couverture maladie universelle en 1989, douze ans seulement après la \
            création de l'assurance maladie obligatoire pour les grandes entreprises, en 1977.",
        "KR_EI" =>
            "L'assurance emploi coréenne est instaurée en 1995. La crise financière asiatique de 1997-1998, \
            qui fait exploser le chômage, conduit à l'étendre en urgence à toutes les entreprises.",
        "KR_SANJAE" =>
            "L'assurance accidents du travail, créée en 1964, est la première assurance sociale de l'histoire \
            de la Corée du Sud.",
        "KR_LTC" =>
            "L'assurance soins de longue durée a été introduite en 2008. La Corée du Sud a connu le \
            vieillissement parmi les plus rapides de l'OCDE, avec le taux de fécondité le plus bas du monde.",
        "IN_EPF" =>
            "Le fonds de prévoyance des salariés est créé en 1952. Il ne couvre que le secteur formel, alors \
            que la grande majorité des travailleurs indiens relèvent de l'économie informelle.",
        "IN_ESI" =>
            "L'Employees' State Insurance Act est adopté en 1948, un an seulement après l'indépendance : l'une \
            des premières lois sociales de l'Inde indépendante.",
        "IN_IMPOT" =>
            "Le budget 2020 a introduit un « nouveau régime » d'impôt, aux taux plus bas mais sans la plupart \
            des déductions. Devenu le régime par défaut en 2023, il laisse au salarié le choix chaque année.",
        "AE_EXPAT" =>
            "Les Émirats ne connaissent pas d'impôt sur le revenu. Ils ont introduit la TVA en 2018 et un impôt \
            sur les sociétés de 9 % en 2023, tout en maintenant l'absence d'impôt sur les salaires.",

        // ──────────────────────────────── Océanie ──────────────────────────────
        "AU_SUPER" =>
            "Le Superannuation Guarantee est introduit en 1992 sous le Premier ministre Paul Keating. La \
            retraite par capitalisation obligatoire a fait des fonds de pension australiens l'un des plus \
            gros réservoirs d'épargne retraite du monde.",
        "AU_MEDICARE" =>
            "L'assurance maladie universelle australienne a connu deux naissances : Medibank, créé en 1975 par \
            le gouvernement Whitlam, puis démantelé par son successeur, et Medicare, rétabli en 1984 par Bob \
            Hawke, financé par cette contribution.",
        "AU_INCOME_TAX" =>
            "En 1942, pendant la guerre, le gouvernement fédéral australien retire aux États le droit de lever \
            l'impôt sur le revenu, en le leur « empruntant » pour la durée du conflit. Il ne le leur a jamais \
            rendu.",
        "NZ_ACC" =>
            "Le rapport du juge Owen Woodhouse (1967) aboutit en 1974 à un système unique au monde : toute \
            victime d'un accident est indemnisée sans faute, mais renonce en échange au droit de poursuivre \
            en justice le responsable.",
        "NZ_KIWISAVER_EMP" =>
            "KiwiSaver est lancé en 2007 par le ministre des Finances travailliste Michael Cullen. \
            L'adhésion est automatique pour les nouveaux salariés, qui peuvent en sortir : un exemple souvent \
            cité d'« incitation douce » en économie comportementale.",

        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn francais_seulement_et_sans_separateur() {
        assert!(anecdote("CSG_DEDUCTIBLE", "fr").is_some());
        assert!(anecdote("CSG_DEDUCTIBLE", "en").is_none());
        assert_eq!(anecdote("IT_ADD_REG_LO", "fr"), anecdote("IT_ADD_REG", "fr"));
        assert!(anecdote("CODE_INCONNU", "fr").is_none());
    }
}
