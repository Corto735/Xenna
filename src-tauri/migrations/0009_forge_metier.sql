-- ============================================================
-- FORGE MÉTIER — Espace communautaire (gestionnaires de paie)
-- ============================================================
-- Séparé intentionnellement du moteur de paie (tables sans
-- relation avec cotisation, taux, plafonds, etc.)

-- Table utilisateurs (minimale — sera enrichie par le système
-- d'auth quand il existera ; user_id est la colle entre les deux)
CREATE TABLE IF NOT EXISTS users (
    id         INTEGER PRIMARY KEY,
    email      TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    created_at TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Profils publics des contributeurs
CREATE TABLE contributor_profiles (
    id              INTEGER PRIMARY KEY,
    user_id         INTEGER NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    pseudo          TEXT    NOT NULL UNIQUE COLLATE NOCASE,
    linkedin_url    TEXT,                   -- NULL si non renseigné
    poste           TEXT    NOT NULL,       -- "Gestionnaire paie", "DRH", etc.

    -- Compteurs d'engagement (dénormalisés pour affichage rapide)
    posts_count     INTEGER NOT NULL DEFAULT 0,
    topics_count    INTEGER NOT NULL DEFAULT 0,
    votes_received  INTEGER NOT NULL DEFAULT 0,
    votes_given     INTEGER NOT NULL DEFAULT 0,

    created_at      TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Expertises CCN par profil
-- Table liée plutôt que JSON : queryable, indexable, contrainte
-- d'intégrité sur 'niveau', et cohérent avec le reste du schéma.
CREATE TABLE contributor_ccn_expertises (
    id           INTEGER PRIMARY KEY,
    profile_id   INTEGER NOT NULL REFERENCES contributor_profiles(id) ON DELETE CASCADE,
    ccn_idcc     TEXT    NOT NULL,  -- Code IDCC ex: '0016', '3239', '1090'
    ccn_libelle  TEXT    NOT NULL,  -- Libellé court ex: 'Métallurgie', 'Syntec'
    niveau       TEXT    NOT NULL CHECK (niveau IN ('Connue', 'Pratiquée', 'Maîtrisée')),
    UNIQUE (profile_id, ccn_idcc)   -- Une seule entrée par CCN par profil
);

-- Source de vérité des votes
-- La contrainte UNIQUE empêche le double-vote.
-- Le trigger ci-dessous gère les compteurs de façon atomique.
CREATE TABLE forge_votes (
    id               INTEGER PRIMARY KEY,
    voter_id         INTEGER NOT NULL REFERENCES contributor_profiles(id),
    topic_author_id  INTEGER NOT NULL REFERENCES contributor_profiles(id),
    topic_id         INTEGER NOT NULL,  -- ID du sujet (table forum à venir)
    created_at       TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE (voter_id, topic_id)         -- Un vote par sujet par utilisateur
);

-- ============================================================
-- TRIGGERS
-- ============================================================

-- updated_at automatique (sur les colonnes métier uniquement,
-- pour éviter une récursion sur la colonne updated_at elle-même)
CREATE TRIGGER contributor_profiles_updated_at
AFTER UPDATE OF pseudo, linkedin_url, poste, posts_count, topics_count
ON contributor_profiles
BEGIN
    UPDATE contributor_profiles
        SET updated_at = datetime('now')
        WHERE id = NEW.id;
END;

-- Incrémentation atomique des compteurs lors d'un vote.
-- Pas de transaction explicite nécessaire : un trigger SQLite
-- est déjà exécuté dans le contexte de la transaction parente.
CREATE TRIGGER forge_vote_increment
AFTER INSERT ON forge_votes
BEGIN
    UPDATE contributor_profiles
        SET votes_given = votes_given + 1
        WHERE id = NEW.voter_id;
    UPDATE contributor_profiles
        SET votes_received = votes_received + 1
        WHERE id = NEW.topic_author_id;
END;

-- ============================================================
-- INDEX
-- ============================================================
CREATE INDEX idx_contributor_profiles_pseudo ON contributor_profiles(pseudo);
CREATE INDEX idx_ccn_expertises_profile       ON contributor_ccn_expertises(profile_id);
CREATE INDEX idx_ccn_expertises_niveau        ON contributor_ccn_expertises(niveau);
CREATE INDEX idx_forge_votes_topic            ON forge_votes(topic_id);
CREATE INDEX idx_forge_votes_voter            ON forge_votes(voter_id);
