-- ============================================================
-- SMIC DE RÉFÉRENCE FILLON 2026 — gelé au 1er janvier
--
-- Le législateur a décidé que la réduction générale (Fillon / RGDU)
-- reste calculée sur le SMIC en vigueur au 1er JANVIER de l'année,
-- soit 1 823,03 € (12,02 €/h), pour TOUTE l'année 2026. La revalorisation
-- du SMIC au 1er juin 2026 (1 867,02 €, migration 0047) est neutralisée
-- pour ce seul calcul.
--
-- Base légale : décret n°2026-509 du 12 juin 2026 (JO du 14/06/2026),
-- modifiant l'article D. 241-7 du CSS, pris en application de l'art. L. 241-13.
--
-- ⚠️ CORRECTIF de la note de la migration 0047 : contrairement à ce
-- qu'elle affirmait, la hausse de juin NE se répercute PAS sur Fillon.
-- Le seuil d'extinction Fillon 2026 reste 3 × 1 823,03 = 5 469,09 €
-- (et non 3 × 1 867,02 = 5 601,06 €).
--
-- Ce code SMIC_FILLON est lu par ContextPaie (db/context.rs), avec
-- repli sur SMIC_MENSUEL quand il est absent (années historiques :
-- pas de revalo en cours d'année → SMIC de réf = SMIC courant).
-- Seuls les calculs Fillon (cotisations.rs, annee.rs) l'utilisent ;
-- absence et EA gardent le SMIC réel.
-- ============================================================

-- Valeur figée sur toute l'année 2026 (date_fin IS NULL, pas de
-- rupture en juin, contrairement à SMIC_MENSUEL).
INSERT INTO plafond_reference (code, date_debut, date_fin, valeur, periodicite) VALUES
    ('SMIC_FILLON', '2026-01-01', NULL, '1823.03', 'MENSUEL');
