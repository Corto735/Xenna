-- ============================================================
-- MIGRATION 0048 — email nullable dans users
-- ============================================================
-- La colonne `email` (TEXT NOT NULL) était la clé d'identification
-- originale. Elle a été remplacée par email_enc (chiffré AES-256-GCM)
-- et email_hash (HMAC-SHA256, pour la déduplication).
-- `INSERT OR IGNORE` ignorait silencieusement la violation NOT NULL,
-- empêchant toute création de compte.
-- SQLite ne supporte pas ALTER COLUMN → recréation de table.
-- ============================================================

PRAGMA foreign_keys = OFF;

CREATE TABLE users_new (
    id                 INTEGER PRIMARY KEY,
    email              TEXT    UNIQUE COLLATE NOCASE,   -- nullable : remplacé par email_enc + email_hash
    created_at         TEXT    NOT NULL DEFAULT (datetime('now')),
    email_verified     INTEGER NOT NULL DEFAULT 0,
    verification_token TEXT,
    token_expires_at   TEXT,
    email_enc          TEXT,
    email_hash         TEXT,
    password_hash      TEXT
);

INSERT INTO users_new
    SELECT id, email, created_at, email_verified, verification_token,
           token_expires_at, email_enc, email_hash, password_hash
    FROM users;

DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email_hash ON users(email_hash);

PRAGMA foreign_keys = ON;
