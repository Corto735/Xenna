-- ============================================================
-- QUIZZ COMMUNITY — Leaderboard & Suggestions de questions
-- ============================================================

CREATE TABLE quizz_scores (
    id         INTEGER PRIMARY KEY,
    pseudo     TEXT    NOT NULL,
    pays       TEXT    NOT NULL CHECK (pays IN ('fr','fpt','ch','lu','it','ca','qc')),
    score      INTEGER NOT NULL CHECK (score >= 0),
    total      INTEGER NOT NULL CHECK (total >= 5),
    created_at TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE quizz_suggestions (
    id          INTEGER PRIMARY KEY,
    pays        TEXT NOT NULL CHECK (pays IN ('fr','fpt','ch','lu','it','ca','qc')),
    question    TEXT NOT NULL,
    reponse     TEXT,
    source      TEXT,
    pseudo      TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_quizz_scores_pays ON quizz_scores(pays, score DESC);
CREATE INDEX idx_quizz_sugg_pays   ON quizz_suggestions(pays, created_at DESC);
