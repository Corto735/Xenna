-- ============================================================
-- NOUVELLE FORMULE FILLON (officielle URSSAF)
-- ============================================================
-- Coefficient = Tmin + (Tdelta × [(1/2) × (3 × SMIC_annuel / remun_annuelle − 1)]^P)
--
-- Paramètres officiels (base 2025) :
--   Tmin   = 0,0200
--   Tdelta = 0,3781   → Tmax = Tmin + Tdelta = 0,3981
--   P      = 1,75
--   Seuil  = 3 SMIC annuel
--
-- La formule s'annule à 3 SMIC (inner=0 → coeff=Tmin).
-- Elle est plafonnée à Tmax=0,3981 (inner≥1 → au SMIC ou en dessous).
-- Le code Rust détecte la formule à appliquer selon la présence de `tmin` et `puissance`.

ALTER TABLE allegement_param ADD COLUMN tmin TEXT;
ALTER TABLE allegement_param ADD COLUMN puissance TEXT;

-- Mise à jour des paramètres 2019+ avec les valeurs officielles URSSAF.
-- Les anciennes valeurs (coeff_max=0.3214, seuil=1.6) correspondaient
-- à l'ancienne formule linéaire C = (T/0,6) × (1,6×SMIC/brut − 1).
UPDATE allegement_param
SET coeff_max_fillon = '0.3981',
    seuil_smic_coeff = '3.0',
    tmin             = '0.0200',
    puissance        = '1.75',
    formule_texte    = 'C = Tmin + (Tdelta × [(1/2) × (3 × SMIC_annuel / Remun_annuelle − 1)]^P) ; Tmin=0,0200 ; Tdelta=0,3781 ; Tmax=0,3981 ; P=1,75 ; Seuil=3 SMIC'
WHERE allegement_type_id = (SELECT id FROM allegement_type WHERE code = 'FILLON')
  AND date_debut = '2019-01-01';

-- Les paramètres 2015-2018 conservent tmin=NULL et puissance=NULL :
-- le code Rust utilisera l'ancienne formule linéaire pour ces dates.
