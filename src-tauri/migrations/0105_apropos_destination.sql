-- ============================================================
-- Destination des posts « human input »
--
-- Un post publié par l'admin (contenu + frappes enregistrées) peut
-- désormais viser deux surfaces distinctes du site :
--   'apropos' : en haut de la page « À propos » (comportement historique)
--   'carnet'  : la nouvelle page « Carnet de bord »
--
-- Les posts existants sont réputés destinés à « À propos » (DEFAULT).
-- ============================================================

ALTER TABLE apropos_posts ADD COLUMN destination TEXT NOT NULL DEFAULT 'apropos';
