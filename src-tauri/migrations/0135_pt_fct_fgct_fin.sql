-- ============================================================
-- PORTUGAL — fin des contributions FCT et FGCT au 01/05/2023
-- L'Agenda do Trabalho Digno (Lei 13/2023, DL 115/2023) a mis fin à
-- l'obligation de contribuer au Fundo de Compensação do Trabalho (0,925 %)
-- et suspendu les contributions au Fundo de Garantia de Compensação do
-- Trabalho (0,075 %) à compter du 1er mai 2023. Elles étaient encore
-- facturées en 2026.
-- Sources : PLMJ, « FCT e FGCT » (déc. 2023) ; Andersen, note du 19/12/2023.
-- ============================================================
UPDATE cotisation_taux SET date_fin = '2023-05-01',
       notes = 'Contributions arrêtées au 01/05/2023 (Agenda do Trabalho Digno)'
 WHERE cotisation_id IN (SELECT id FROM cotisation WHERE code IN ('PT_FCT', 'PT_FGCT'))
   AND date_fin IS NULL;
