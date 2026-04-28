-- Niveau de connaissance de la paie française (optionnel sur le profil)
ALTER TABLE contributor_profiles
    ADD COLUMN paie_fr_niveau TEXT;

-- Expertises paie internationale (pays frontaliers + Royaume-Uni)
CREATE TABLE contributor_pays_expertises (
    id            INTEGER PRIMARY KEY,
    profile_id    INTEGER NOT NULL REFERENCES contributor_profiles(id) ON DELETE CASCADE,
    pays_code     TEXT    NOT NULL,  -- ex: 'BE', 'CH', 'GB'
    pays_libelle  TEXT    NOT NULL,  -- ex: 'Belgique', 'Suisse', 'Royaume-Uni'
    niveau        TEXT    NOT NULL CHECK (niveau IN ('Connue', 'Pratiquée', 'Maîtrisée')),
    UNIQUE (profile_id, pays_code)
);

CREATE INDEX idx_pays_expertises_profile ON contributor_pays_expertises(profile_id);
