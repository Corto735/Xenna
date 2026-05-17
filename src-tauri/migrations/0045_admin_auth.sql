-- ── Admin users ───────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS admin_users (
    id            INTEGER PRIMARY KEY,
    username      TEXT    NOT NULL UNIQUE,
    password_hash TEXT    NOT NULL,
    created_at    TEXT    DEFAULT (datetime('now'))
);

-- Seed admin initial (argon2id)
INSERT OR IGNORE INTO admin_users (username, password_hash)
VALUES (
    'Corto735',
    '$argon2id$v=19$m=19456,t=2,p=1$3q3ReulD4lFlljaOp31nwg$YP4uyK8+nVFTKZuMgrdsvfUHx2V70vaSm+/51fjQjx0'
);

-- ── Statut de modération sur les profils contributeurs ────────────────────────
ALTER TABLE contributor_profiles
    ADD COLUMN status TEXT NOT NULL DEFAULT 'pending'
    CHECK(status IN ('pending', 'approved', 'rejected'));

-- ── Vérification email ────────────────────────────────────────────────────────
ALTER TABLE users ADD COLUMN email_verified    INTEGER NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN verification_token TEXT;
ALTER TABLE users ADD COLUMN token_expires_at  TEXT;

-- ── Chiffrement email AES-256-GCM ─────────────────────────────────────────────
ALTER TABLE users ADD COLUMN email_enc  TEXT;
ALTER TABLE users ADD COLUMN email_hash TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email_hash ON users(email_hash);

-- ── Statut de modération sur les suggestions de quizz ────────────────────────
ALTER TABLE quizz_suggestions
    ADD COLUMN admin_status TEXT NOT NULL DEFAULT 'pending'
    CHECK(admin_status IN ('pending', 'approved', 'rejected'));
