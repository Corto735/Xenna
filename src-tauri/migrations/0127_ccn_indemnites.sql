-- ============================================================
-- LE CHAKRRAM — INDEMNITÉS DE DÉPLACEMENT DES OUVRIERS (IDCC 0016)
-- Troisième bloc de la page de consultation, à côté des grilles et du
-- maintien. Contenu éditorial : les montants qu'utilise le bulletin
-- vivent dans plafond_reference (migration 0126).
-- Sources : protocole du 30/04/1974 (annexe I) ; av. n° 75, 77, 79, 81.
-- ============================================================
CREATE TABLE ccn_indemnites (
    id          INTEGER PRIMARY KEY,
    idcc        TEXT    NOT NULL REFERENCES ccn_conventions(idcc) ON DELETE CASCADE,
    categorie   TEXT    NOT NULL CHECK (categorie IN ('ouvriers','employes','tam','cadres')),
    -- Branches couvertes, séparées par des virgules (codes de ccn_branches).
    branches    TEXT    NOT NULL,
    intitule    TEXT    NOT NULL,
    article     TEXT    NOT NULL,
    corps       TEXT    NOT NULL,
    tableaux    TEXT    NOT NULL,
    source      TEXT    NOT NULL,
    source_url  TEXT,
    consulte_le TEXT    NOT NULL,
    ordre       INTEGER NOT NULL DEFAULT 0
);

INSERT INTO ccn_indemnites
  (idcc, categorie, branches, intitule, article, corps, tableaux, source, source_url, consulte_le, ordre)
VALUES
('0016', 'ouvriers', 'marchandises,demenagement,logistique',
 'Indemnités de déplacement — personnel ouvrier',
 'Annexe I, protocole du 30 avril 1974 relatif aux frais de déplacement des ouvriers',
 'Ces indemnités remboursent forfaitairement les frais d''un ouvrier contraint, par un déplacement de service, de manger ou de dormir hors de chez lui. Elles sont dues lorsque les frais ne sont pas remboursés sur justificatifs par l''employeur.

Ce ne sont pas des salaires. Le protocole les répute sans caractère de rémunération : elles n''entrent ni dans l''indemnité de congés payés, ni dans l''indemnité de licenciement, ni dans l''assiette du maintien de salaire. Remboursements de frais professionnels, elles sont exclues des cotisations et de la CSG dans la limite des forfaits de l''arrêté du 20 décembre 2002, et exonérées d''impôt sur le revenu (CGI art. 81, 1°) : elles se versent en net, en bas de bulletin.

Les montants conventionnels restent sous les forfaits URSSAF de leur situation (2026 : 7,50 € pour un repas sur le lieu de travail, 10,40 € hors des locaux, 21,40 € au restaurant en déplacement).

Dans le simulateur : avec la convention IDCC 0016 choisie dans le menu pays/région, le « + » d''un bulletin français propose le repas unique, le repas unique de nuit, l''indemnité spéciale et le casse-croûte.',
 '[{"titre": "Montants en vigueur au 1er janvier 2026 — avenant n° 81 du 2 décembre 2025", "colonnes": ["Indemnité", "Article", "Condition d''attribution", "Montant"], "lignes": [["Repas", "art. 3", "Repas pris hors du lieu de travail du fait d''un déplacement, amplitude couvrant 11 h 45-14 h 15 ou 18 h 45-21 h 15", "16,36 €"], ["Repas unique", "art. 4", "Déplacements dans la zone de camionnage autour de Paris", "10,07 €"], ["Repas unique de nuit", "art. 12", "Service comportant au moins 4 h de travail effectif entre 22 h et 7 h, sans autre indemnité de repas", "9,81 €"], ["Indemnité spéciale", "art. 7", "Amplitude couvrant entièrement 11 h-14 h 30 ou 18 h 30-22 h sans coupure d''au moins 1 h", "4,42 €"], ["Casse-croûte", "art. 5", "Prise de service avant 5 h en raison d''un déplacement", "8,87 €"], ["Grand déplacement — 1 repas + 1 découcher", "art. 6", "Impossibilité de regagner le domicile pour le repos journalier", "52,31 €"], ["Grand déplacement — 2 repas + 1 découcher", "art. 6", "Impossibilité de regagner le domicile pour le repos journalier", "68,67 €"]], "note": "Revalorisation de 1 % au 1er janvier 2026, avenant étendu par arrêté du 3 février 2026. Le repas unique de nuit ne se cumule pas avec les autres indemnités de repas ; repas et casse-croûte se cumulent entre eux."}, {"titre": "Historique des montants", "colonnes": ["Indemnité", "01/12/2022 (av. n° 75)", "01/12/2023 (av. n° 77)", "01/03/2025 (av. n° 79)", "01/01/2026 (av. n° 81)"], "lignes": [["Repas", "15,20 €", "15,96 €", "16,20 €", "16,36 €"], ["Repas unique", "9,35 €", "9,82 €", "9,97 €", "10,07 €"], ["Repas unique de nuit", "9,11 €", "9,57 €", "9,71 €", "9,81 €"], ["Indemnité spéciale", "4,11 €", "4,32 €", "4,38 €", "4,42 €"], ["Casse-croûte", "8,24 €", "8,65 €", "8,78 €", "8,87 €"], ["Grand déplacement — 1 repas + 1 découcher", "48,59 €", "51,02 €", "51,79 €", "52,31 €"], ["Grand déplacement — 2 repas + 1 découcher", "63,79 €", "66,98 €", "67,99 €", "68,67 €"]], "note": "L''avenant n° 77 (11 octobre 2023) a été étendu par arrêté du 19 décembre 2023 : pour une entreprise non adhérente à une organisation signataire, il s''impose à compter de l''extension. Avant le 1er décembre 2022, l''historique n''a pas été relu texte par texte et n''est pas reproduit."}]',
 'Protocole du 30 avril 1974 (annexe I), revalorisé par l''avenant n° 81 du 2 décembre 2025 (transport routier de marchandises, activités auxiliaires, déménagement, fonds et valeurs, prestations logistiques)',
 'https://www.legifrance.gouv.fr/conv_coll/id/KALITEXT000005678899/?idConteneur=KALICONT000005635624',
 '2026-09-30', 1);
