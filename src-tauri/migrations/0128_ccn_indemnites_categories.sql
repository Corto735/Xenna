-- ============================================================
-- LE CHAKRRAM — FRAIS DE DÉPLACEMENT DES AUTRES CATÉGORIES (IDCC 0016)
-- Ouvriers du transport de voyageurs et du transport sanitaire : barème
-- propre du protocole du 30/04/1974 (av. n° 80 du 20/03/2025, n° 82 du
-- 11/02/2026, texte signé relu). Employés, TAM et cadres : pas de forfait,
-- ce que disent (ou ne disent pas) les annexes II, III (art. 16) et IV
-- (art. 14), texte consolidé au 22/11/2016.
-- ============================================================
INSERT INTO ccn_indemnites
  (idcc, categorie, branches, intitule, article, corps, tableaux, source, source_url, consulte_le, ordre)
VALUES
('0016', 'ouvriers', 'voyageurs,sanitaire',
 'Indemnités de déplacement — personnel ouvrier (voyageurs et sanitaire)',
 'Annexe I, protocole du 30 avril 1974, barème des entreprises de transport routier de voyageurs et de transport sanitaire',
 'Le transport de voyageurs et le transport sanitaire appliquent le même protocole du 30 avril 1974 que les marchandises, mais avec leur propre barème et leurs propres articles (art. 8 à 12). Le personnel ambulancier en déplacement ou d''astreinte dans les locaux en relève expressément (chapitre des dispositions particulières au transport sanitaire, 11° « Frais de déplacement »).

Le repas unique est la règle en voyageurs : il est dû au personnel obligé, par un déplacement de service, de prendre un repas hors de son lieu de travail. L''indemnité spéciale remplace l''indemnité de repas quand l''amplitude couvre 11 h-14 h 30 ou 18 h 30-22 h avec une coupure d''au moins 30 minutes mais de moins d''une heure.

Même régime social que pour les marchandises : ce sont des remboursements de frais professionnels, exclus des cotisations dans la limite des forfaits de l''arrêté du 20 décembre 2002 et exonérés d''impôt sur le revenu, versés en net.

Le simulateur ne propose dans son « + » que les indemnités du barème marchandises : celles-ci restent consultables ici.',
 '[{"titre": "Montants en vigueur au 1er mars 2026 — avenant n° 82 du 11 février 2026", "colonnes": ["Indemnité", "Articles du protocole", "Montant"], "lignes": [["Repas", "art. 8-1 al. 2 et 3 ; art. 9-10 al. 1 ; art. 11", "15,70 €"], ["Repas unique", "art. 8-1 al. 1", "9,69 €"], ["Indemnité spéciale", "art. 8-2 al. 2 ; art. 11 bis", "4,38 €"], ["Casse-croûte", "art. 12", "7,76 €"], ["Indemnité spéciale de petit déjeuner", "art. 10 al. 2", "4,38 €"], ["Chambre et indemnité spéciale de petit déjeuner", "art. 10 al. 1", "33,36 €"], ["Repos journalier (chambre et casse-croûte)", "art. 11", "36,74 €"]], "note": "Avenant étendu par arrêté du 6 mai 2026. Applicable quel que soit l''effectif de l''entreprise (art. 3 de l''avenant)."}, {"titre": "Historique des montants", "colonnes": ["Indemnité", "01/04/2025 (av. n° 80)", "01/03/2026 (av. n° 82)"], "lignes": [["Repas", "15,54 €", "15,70 €"], ["Repas unique", "9,59 €", "9,69 €"], ["Indemnité spéciale", "4,34 €", "4,38 €"], ["Casse-croûte", "7,68 €", "7,76 €"], ["Indemnité spéciale de petit déjeuner", "4,34 €", "4,38 €"], ["Chambre et indemnité spéciale de petit déjeuner", "33,03 €", "33,36 €"], ["Repos journalier (chambre et casse-croûte)", "36,37 €", "36,74 €"]], "note": "Les barèmes antérieurs au 1er avril 2025 n''ont pas été relus texte par texte et ne sont pas reproduits."}]',
 'Protocole du 30 avril 1974 (annexe I), revalorisé par l''avenant n° 82 du 11 février 2026 ; avenant n° 80 du 20 mars 2025 pour l''historique',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000005678899/?idConteneur=KALICONT000005635624',
 '2026-09-30', 2),
('0016', 'employes', 'marchandises,voyageurs,demenagement,logistique',
 'Frais de déplacement — personnel employé',
 'Annexe II, accord du 27 février 1951',
 'L''annexe II, qui règle la situation des employés, ne contient ni article sur les frais de déplacement ni barème d''indemnités : le protocole du 30 avril 1974 ne vise que les ouvriers.

À défaut de disposition conventionnelle, un employé envoyé en déplacement est remboursé selon la politique de l''entreprise, sur justificatifs (frais réels) ou par allocations forfaitaires. Les frais réels justifiés sont exclus des cotisations en totalité ; les allocations forfaitaires le sont dans la limite des barèmes de l''arrêté du 20 décembre 2002 (2026 : 7,50 € sur le lieu de travail, 10,40 € hors des locaux, 21,40 € au restaurant en déplacement).',
 '[{"titre": "Ce que dit le texte", "colonnes": ["Catégorie", "Texte", "Frais de déplacement"], "lignes": [["Employés", "Annexe II — accord du 27 février 1951", "Aucune clause de frais de déplacement ni barème forfaitaire"]], "note": "Constat fait sur le texte consolidé à jour au 22 novembre 2016. Les coursiers, rattachés au personnel ouvrier, suivent le protocole des ouvriers au barème des marchandises."}]',
 'Convention collective nationale des transports routiers du 21 décembre 1950, texte consolidé à jour au 22 novembre 2016',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALICONT000005635624',
 '2026-09-30', 3),
('0016', 'tam', 'marchandises,voyageurs,demenagement,logistique',
 'Frais de déplacement — techniciens et agents de maîtrise',
 'Annexe III, accord du 30 mars 1951, article 16',
 'Un technicien ou agent de maîtrise envoyé en déplacement est remboursé aux frais réels : la convention ne fixe aucune indemnité forfaitaire pour lui. Les frais de transport sont comptés au départ et au retour du lieu de travail habituel ; les frais de séjour doivent lui assurer des repas et un logement en rapport avec ses fonctions.

Des remboursements sur justificatifs ne sont pas une rémunération : ils sont exclus des cotisations et de l''impôt sur le revenu pour leur montant réel.',
 '[{"titre": "Remboursement des frais de déplacement — annexe III, article 16", "colonnes": ["Poste", "Règle conventionnelle"], "lignes": [["Transport en commun", "Prix des billets remboursé ; train ou bateau en 2e classe, en classe supérieure ou en couchette si le service le justifie"], ["Véhicule personnel ou de l''entreprise", "Frais à la charge de l''employeur, selon un accord préalable (carburant, huile, entretien, garage, amortissement, assurance)"], ["Séjour", "Repas et logement en rapport avec les fonctions, réglés au retour sur justificatifs ; avances sur demande"], ["Missions de même nature que celles des cadres", "Règles des ingénieurs et cadres (annexe IV, article 14)"]], "note": "Aucun forfait conventionnel : frais réels. Les allocations sur justificatifs sont exclues des cotisations en totalité."}]',
 'Convention collective nationale des transports routiers du 21 décembre 1950, texte consolidé à jour au 22 novembre 2016',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALICONT000005635624',
 '2026-09-30', 4),
('0016', 'cadres', 'marchandises,voyageurs,demenagement,logistique',
 'Frais de déplacement — ingénieurs et cadres',
 'Annexe IV, accord du 30 octobre 1951, article 14',
 'Un ingénieur ou cadre envoyé en déplacement voit ses frais de transport, de séjour et de représentation pris en charge par l''entreprise. Ils sont remboursés au retour, sur justification des dépenses effectuées ; des avances lui sont accordées sur demande.

Remboursements de frais réels justifiés, ils sont exclus des cotisations et de l''impôt sur le revenu pour leur montant exact.',
 '[{"titre": "Remboursement des frais de déplacement — annexe IV, article 14", "colonnes": ["Poste", "Règle conventionnelle"], "lignes": [["Transport en commun", "Prix des billets remboursé ; train ou bateau en 1re classe, en wagon-lit si le service le justifie"], ["Véhicule personnel ou de l''entreprise", "Frais à la charge de l''employeur, selon un accord préalable (carburant, huile, entretien, garage, amortissement, assurance)"], ["Séjour", "Repas et logement en rapport avec les fonctions et les missions"], ["Représentation", "Frais engagés dans l''intérêt de l''entreprise, avec l''accord de l''employeur, réglés intégralement sur notes de frais"]], "note": "Aucun forfait conventionnel : tout est remboursé au retour sur justificatifs, avances sur demande."}]',
 'Convention collective nationale des transports routiers du 21 décembre 1950, texte consolidé à jour au 22 novembre 2016',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALICONT000005635624',
 '2026-09-30', 5);
