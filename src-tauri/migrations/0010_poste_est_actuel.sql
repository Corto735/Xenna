-- Indique si le poste renseigné est le poste actuel (1) ou visé (0).
-- DEFAULT 1 : les profils existants sont considérés comme "poste actuel".
ALTER TABLE contributor_profiles
    ADD COLUMN poste_est_actuel INTEGER NOT NULL DEFAULT 1;
