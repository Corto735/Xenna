-- ── Mot de passe membres ─────────────────────────────────────────────────────
ALTER TABLE users ADD COLUMN password_hash TEXT;

-- ── Forum : sujets ────────────────────────────────────────────────────────────
CREATE TABLE forum_topics (
    id          INTEGER PRIMARY KEY,
    author_id   INTEGER NOT NULL REFERENCES contributor_profiles(id) ON DELETE CASCADE,
    titre       TEXT    NOT NULL,
    contenu     TEXT    NOT NULL,
    created_at  TEXT    DEFAULT (datetime('now')),
    reply_count INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX idx_forum_topics_created ON forum_topics(created_at DESC);
CREATE INDEX idx_forum_topics_author  ON forum_topics(author_id);

-- ── Forum : réponses ──────────────────────────────────────────────────────────
CREATE TABLE forum_replies (
    id         INTEGER PRIMARY KEY,
    topic_id   INTEGER NOT NULL REFERENCES forum_topics(id) ON DELETE CASCADE,
    author_id  INTEGER NOT NULL REFERENCES contributor_profiles(id) ON DELETE CASCADE,
    contenu    TEXT    NOT NULL,
    created_at TEXT    DEFAULT (datetime('now'))
);
CREATE INDEX idx_forum_replies_topic ON forum_replies(topic_id, created_at ASC);

-- ── Triggers : compteurs ──────────────────────────────────────────────────────
CREATE TRIGGER forum_topic_insert AFTER INSERT ON forum_topics BEGIN
    UPDATE contributor_profiles SET topics_count = topics_count + 1 WHERE id = NEW.author_id;
END;

CREATE TRIGGER forum_reply_insert AFTER INSERT ON forum_replies BEGIN
    UPDATE forum_topics         SET reply_count  = reply_count  + 1 WHERE id = NEW.topic_id;
    UPDATE contributor_profiles SET posts_count  = posts_count  + 1 WHERE id = NEW.author_id;
END;

CREATE TRIGGER forum_reply_delete BEFORE DELETE ON forum_replies BEGIN
    UPDATE forum_topics         SET reply_count = MAX(0, reply_count - 1) WHERE id = OLD.topic_id;
    UPDATE contributor_profiles SET posts_count  = MAX(0, posts_count  - 1) WHERE id = OLD.author_id;
END;
