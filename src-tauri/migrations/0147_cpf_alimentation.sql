-- ============================================================
-- COMPTE PERSONNEL DE FORMATION — alimentation annuelle (calculs/droits.rs)
--
-- En euros depuis le 1er janvier 2019 (loi n° 2018-771 du 5 septembre 2018,
-- décret n° 2018-1329 du 28 décembre 2018) :
--   • salarié ayant travaillé au moins la moitié de la durée légale ou
--     conventionnelle sur l'année : 500 € par an, plafond 5 000 € (art. R6323-1
--     du code du travail) ; en deçà, au prorata du temps de travail ; montants
--     arrondis au centime supérieur ;
--   • salarié sans qualification de niveau 3 (CAP, BEP), au moins à mi-temps :
--     800 € par an, plafond 8 000 € (art. L6323-11-1, R6323-3-1) ;
--   • bénéficiaire de l'obligation d'emploi (travailleur handicapé), au moins à
--     mi-temps : majoration de 300 €, soit 800 € par an, plafond 8 000 €
--     (art. L6323-11, D6323-3-3, décret n° 2019-566 du 7 juin 2019) ;
--   • personne accueillie en ESAT : 800 € par année d'admission, à temps plein
--     ou partiel, plafond 8 000 € (art. L6323-33 et s., R6323-27 et s.).
-- Droits de l'année N calculés par la Caisse des dépôts sur la DSN et crédités
-- au plus tard le 15 juin N+1. Montants inchangés en 2026 : fiche
-- service-public.gouv.fr F10705, vérifiée le 27 juin 2026.
-- ============================================================
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('CPF_ALIMENTATION',         '2019-01-01', NULL, '500',  'ANNUEL'),
    ('CPF_PLAFOND',              '2019-01-01', NULL, '5000', 'ANNUEL'),
    ('CPF_ALIMENTATION_MAJOREE', '2019-01-01', NULL, '800',  'ANNUEL'),
    ('CPF_PLAFOND_MAJORE',       '2019-01-01', NULL, '8000', 'ANNUEL');
