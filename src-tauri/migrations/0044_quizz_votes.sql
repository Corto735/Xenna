-- Colonnes ajoutées à quizz_suggestions : réponses alternatives, mauvaises réponses, compteur votes
ALTER TABLE quizz_suggestions ADD COLUMN reps_alt     TEXT;
ALTER TABLE quizz_suggestions ADD COLUMN mauvaises_rep TEXT;
ALTER TABLE quizz_suggestions ADD COLUMN votes INTEGER DEFAULT 0;

-- Table de déduplication des votes (voter_id = UUID aléatoire stocké côté client)
CREATE TABLE quizz_suggestion_votes (
    suggestion_id INTEGER NOT NULL,
    voter_id      TEXT    NOT NULL,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (suggestion_id, voter_id)
);
