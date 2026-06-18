-- Posts « Human input » publiés sur la page À propos depuis le module Méliinda.
-- contenu = texte final du paragraphe ; events = JSON Vec<KeyEvent> pour le replay.
CREATE TABLE IF NOT EXISTS apropos_posts (
    id         TEXT PRIMARY KEY,   -- uuid
    contenu    TEXT NOT NULL,      -- texte final du paragraphe
    events     TEXT NOT NULL,      -- JSON Vec<KeyEvent> pour le replay
    created_at TEXT NOT NULL       -- RFC3339
);
