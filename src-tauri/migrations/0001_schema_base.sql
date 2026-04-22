-- Extension nécessaire pour stocker des dates ISO en TEXT (SQLite n'a pas de type DATE natif)
-- Convention : toutes les dates sont au format 'YYYY-MM-DD'
-- date_fin NULL = en vigueur actuellement

-- ============================================================
-- ORGANISMES COLLECTEURS
-- ============================================================
CREATE TABLE organisme (
    id      INTEGER PRIMARY KEY,
    code    TEXT NOT NULL UNIQUE,   -- 'URSSAF', 'AGIRC_ARRCO', 'FRANCE_TRAVAIL'
    libelle TEXT NOT NULL,
    url     TEXT
);

-- ============================================================
-- PARTENAIRES SOCIAUX
-- ============================================================
CREATE TABLE partenaire_social (
    id      INTEGER PRIMARY KEY,
    code    TEXT NOT NULL UNIQUE,   -- 'CFDT', 'CGT', 'MEDEF'
    libelle TEXT NOT NULL,
    type    TEXT NOT NULL CHECK (type IN ('SYNDICAT_SALARIE', 'SYNDICAT_PATRONAL', 'ETAT')),
    actif   INTEGER NOT NULL DEFAULT 1  -- 0 = dissous/fusionné
);

-- ============================================================
-- TEXTES DE LOI ET REFERENCES LEGALES
-- ============================================================
CREATE TABLE texte_loi (
    id             INTEGER PRIMARY KEY,
    code           TEXT NOT NULL UNIQUE,   -- 'LFSS_2018', 'ANI_2017_AGIRC'
    type           TEXT NOT NULL CHECK (type IN ('LOI', 'DECRET', 'ARRETE', 'CIRCULAIRE', 'ANI', 'ACCORD_BRANCHE', 'ORDONNANCE')),
    titre          TEXT NOT NULL,
    numero         TEXT,                   -- 'n°2017-1836'
    date_parution  TEXT,                   -- 'YYYY-MM-DD'
    date_vigueur   TEXT,                   -- date d'entrée en vigueur
    jorf           TEXT,                   -- référence Journal Officiel
    url_legifrance TEXT,
    resume         TEXT                    -- explication courte pour la visualisation
);

-- Signataires d'un texte (ANI, accords de branche...)
CREATE TABLE texte_loi_signataire (
    texte_loi_id       INTEGER NOT NULL REFERENCES texte_loi(id),
    partenaire_id      INTEGER NOT NULL REFERENCES partenaire_social(id),
    a_signe            INTEGER NOT NULL DEFAULT 1,  -- 0 = présent mais n'a pas signé
    PRIMARY KEY (texte_loi_id, partenaire_id)
);

-- ============================================================
-- PLAFONDS DE REFERENCE (PMSS, PASS, SMIC)
-- ============================================================
CREATE TABLE plafond_reference (
    id         INTEGER PRIMARY KEY,
    code       TEXT NOT NULL,        -- 'PMSS', 'PASS', 'SMIC_HORAIRE', 'SMIC_MENSUEL'
    date_debut TEXT NOT NULL,        -- 'YYYY-MM-DD'
    date_fin   TEXT,                 -- NULL = en vigueur
    valeur     TEXT NOT NULL,        -- NUMERIC stocké en TEXT pour précision exacte
    periodicite TEXT NOT NULL CHECK (periodicite IN ('MENSUEL', 'ANNUEL', 'HORAIRE')),
    texte_loi_id INTEGER REFERENCES texte_loi(id),
    UNIQUE (code, date_debut)
);

-- Trigger anti-chevauchement pour plafond_reference
CREATE TRIGGER plafond_no_overlap
BEFORE INSERT ON plafond_reference
BEGIN
    SELECT RAISE(ABORT, 'Chevauchement de période pour ce plafond')
    WHERE EXISTS (
        SELECT 1 FROM plafond_reference
        WHERE code = NEW.code
          AND date_debut < COALESCE(NEW.date_fin, '9999-12-31')
          AND COALESCE(date_fin, '9999-12-31') > NEW.date_debut
          AND id != COALESCE(NEW.id, -1)
    );
END;

-- ============================================================
-- COTISATIONS — DEFINITION
-- ============================================================
CREATE TABLE cotisation (
    id              INTEGER PRIMARY KEY,
    code            TEXT NOT NULL UNIQUE,  -- 'SS_MALADIE', 'AGIRC_ARRCO_T1'
    libelle         TEXT NOT NULL,
    organisme_id    INTEGER REFERENCES organisme(id),
    categorie       TEXT NOT NULL CHECK (categorie IN (
                        'SECURITE_SOCIALE', 'RETRAITE_COMPLEMENTAIRE',
                        'CHOMAGE', 'PREVOYANCE', 'CSG_CRDS', 'FORMATION',
                        'TRANSPORT', 'AUTRES'
                    )),
    -- Population concernée
    applicable_cadre     INTEGER NOT NULL DEFAULT 1,
    applicable_non_cadre INTEGER NOT NULL DEFAULT 1,
    -- Type d'assiette de base
    type_assiette   TEXT NOT NULL CHECK (type_assiette IN (
                        'BRUT_TOTAL',       -- sur tout le brut
                        'BRUT_PLAFONNÉ',    -- brut limité à N × PMSS
                        'TRANCHE_A',        -- 0 → 1 PMSS
                        'TRANCHE_B',        -- 1 → 4 PMSS (ancienne AGIRC)
                        'TRANCHE_1',        -- 0 → 1 PMSS (ancienne ARRCO)
                        'TRANCHE_2',        -- 1 → 3 PMSS (ancienne ARRCO)
                        'CSG',              -- 98,25% du brut
                        'SPECIFIQUE'        -- calcul personnalisé (AT/MP, Fillon...)
                    )),
    -- Multiplicateur de plafond si applicable (ex: 4.0 pour le chômage = 4 PMSS)
    plafond_coeff_min TEXT NOT NULL DEFAULT '0',
    plafond_coeff_max TEXT,   -- NULL = pas de plafond
    -- Code DSN (pour la future génération DSN)
    code_dsn_sal    TEXT,
    code_dsn_pat    TEXT,
    notes           TEXT
);

-- ============================================================
-- TAUX DE COTISATION — HISTORIQUE
-- ============================================================
CREATE TABLE cotisation_taux (
    id              INTEGER PRIMARY KEY,
    cotisation_id   INTEGER NOT NULL REFERENCES cotisation(id),
    date_debut      TEXT NOT NULL,   -- 'YYYY-MM-DD'
    date_fin        TEXT,            -- NULL = en vigueur
    taux_salarial   TEXT NOT NULL DEFAULT '0',  -- ex: '0.0690'
    taux_patronal   TEXT NOT NULL DEFAULT '0',  -- ex: '0.0855'
    texte_loi_id    INTEGER REFERENCES texte_loi(id),
    notes           TEXT,
    UNIQUE (cotisation_id, date_debut)
);

-- Trigger anti-chevauchement pour cotisation_taux
CREATE TRIGGER cotisation_taux_no_overlap
BEFORE INSERT ON cotisation_taux
BEGIN
    SELECT RAISE(ABORT, 'Chevauchement de période pour cette cotisation')
    WHERE EXISTS (
        SELECT 1 FROM cotisation_taux
        WHERE cotisation_id = NEW.cotisation_id
          AND date_debut < COALESCE(NEW.date_fin, '9999-12-31')
          AND COALESCE(date_fin, '9999-12-31') > NEW.date_debut
    );
END;

-- ============================================================
-- PARAMETRES ALLEGEMENTS (REDUCTION FILLON ET AUTRES)
-- ============================================================
CREATE TABLE allegement_type (
    id      INTEGER PRIMARY KEY,
    code    TEXT NOT NULL UNIQUE,   -- 'FILLON', 'ZEROFSS', 'APPRENTISSAGE'
    libelle TEXT NOT NULL,
    notes   TEXT
);

CREATE TABLE allegement_param (
    id                INTEGER PRIMARY KEY,
    allegement_type_id INTEGER NOT NULL REFERENCES allegement_type(id),
    date_debut        TEXT NOT NULL,
    date_fin          TEXT,
    -- Paramètres Fillon (NULL pour les autres types)
    coeff_max_fillon  TEXT,   -- ex: '0.3214' (avec prévoyance) ou '0.3194'
    seuil_smic_coeff  TEXT,   -- ex: '1.6' (seuil à 1,6 SMIC)
    -- Formule lisible pour la visualisation
    formule_texte     TEXT,   -- 'C = (T/0.6) × (1.6 × SMIC_annuel/Rémun_annuelle - 1)'
    texte_loi_id      INTEGER REFERENCES texte_loi(id),
    UNIQUE (allegement_type_id, date_debut)
);

-- ============================================================
-- INDEX
-- ============================================================
CREATE INDEX idx_plafond_ref_code_date    ON plafond_reference(code, date_debut);
CREATE INDEX idx_cotisation_taux_cot_date ON cotisation_taux(cotisation_id, date_debut);
CREATE INDEX idx_allegement_param_date    ON allegement_param(allegement_type_id, date_debut);
