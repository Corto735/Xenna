-- ============================================================
-- ITALIE — Cotisations sociales INPS / INAIL
-- Secteur privé de référence : commercio / industria générique
-- Les taux CCNL-spécifiques (métallurgie, bâtiment, etc.) varient.
-- Sources : Circolari INPS, Delibere INAIL, TUIR.
-- ============================================================

-- ── Définitions ─────────────────────────────────────────────
INSERT INTO cotisation (code, libelle, organisme_id, categorie,
  applicable_cadre, applicable_non_cadre, type_assiette, notes) VALUES

  ('IT_IVS', 'IVS — Invalidità, Vecchiaia, Superstiti (INPS)',
    (SELECT id FROM organisme WHERE code='INPS'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_PLAFONNÉ',
    'Régime pension obligatoire. Total 33 % (9,19 % sal + 23,81 % pat). '
    'Massimale contributivo applicable aux seuls salariés sans ancienneté INPS au 31/12/1995 '
    '(L. 335/1995). Salariés pré-1996 : calcul sur salaire brut total sans plafond.'),

  ('IT_NASPI', 'NASpI — Nuova Assicurazione Sociale per l''Impiego (chômage)',
    (SELECT id FROM organisme WHERE code='INPS'),
    'CHOMAGE', 1, 1, 'BRUT_TOTAL',
    'Assurance chômage ordinaire. 0 % salarié (supprimée L. 228/2012 depuis 01/01/2013). '
    '1,61 % exclusivement patronal. Aucun plafond. '
    'En vigueur depuis le 01/05/2015 (D.Lgs. 22/2015), remplace l''ASpI au même taux.'),

  ('IT_NASPI_TERMINE', 'NASpI — Contributo addizionale CDD (+1,40 % pat.)',
    (SELECT id FROM organisme WHERE code='INPS'),
    'CHOMAGE', 1, 1, 'BRUT_TOTAL',
    'Majoration pour contrats à durée déterminée (contratti a tempo determinato). '
    '+1,40 % patronal en sus de la cotisation ordinaire. '
    'Remboursée en cas de transformation du CDD en CDI dans les 6 mois. '
    'Non applicable : CDD de remplacement, CDD saisonniers, apprentissage. '
    'Source : L. 92/2012 art. 2 c. 28-29.'),

  ('IT_MALATTIA', 'Malattia — Indemnités journalières maladie (INPS)',
    (SELECT id FROM organisme WHERE code='INPS'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
    'Couverture IJ maladie. Exclusivement patronal. '
    'Taux indicatif référence commercio/industria : 2,22 %. Varie selon CCNL applicable. '
    'INPS verse les IJ à partir du 4e jour d''arrêt ; jours 1–3 (carenza) à charge employeur ou CCNL.'),

  ('IT_MATERNITA', 'Maternità e Paternità — Congés parentaux (INPS)',
    (SELECT id FROM organisme WHERE code='INPS'),
    'SECURITE_SOCIALE', 1, 1, 'BRUT_TOTAL',
    'Financement des congés maternité (5 mois, 80 % salaire) et paternité obligatoire '
    '(10 jours depuis L. 160/2019, 80 %). Taux : 0,46 % exclusivement patronal.'),

  ('IT_FONDO_GARANZIA', 'Fondo di Garanzia TFR — INPS (L. 297/1982)',
    (SELECT id FROM organisme WHERE code='INPS'),
    'PREVOYANCE', 1, 1, 'BRUT_TOTAL',
    'Garantit le paiement du TFR en cas de faillite de l''employeur. '
    '0,20 % exclusivement patronal. '
    'Distinct du versement mensuel du TFR au Fondo Tesoreria INPS '
    '(aziende > 50 salariés, obligatoire depuis L. 296/2006 art. 1 c. 755).'),

  ('IT_INAIL', 'INAIL — Assicurazione Infortuni sul Lavoro e Malattie Professionali',
    (SELECT id FROM organisme WHERE code='INAIL'),
    'ACCIDENT_TRAVAIL', 1, 1, 'BRUT_TOTAL',
    'Accidents du travail et maladies professionnelles. 100 % patronal. '
    'Taux fixé par l''INAIL selon voce di tariffa (code ATECO + type de risque). '
    'Taux indicatif ici : 0,65 % (bureau / terziario, voce 0111). '
    'Secteurs à risque (BTP, chimie) : 2 à 10 %. '
    'Auto-liquidazione annuelle au 16 février (déclaration 1030-INAIL).'),

  ('IT_TFR', 'TFR — Trattamento Fine Rapporto (accrual mensuel 6,91 %)',
    (SELECT id FROM organisme WHERE code='INPS'),
    'INDEMNITE_FIN_CONTRAT', 1, 1, 'SPECIFIQUE',
    'Rémunération différée : 6,91 % = retribuzione annua / 13,5 (L. 297/1982). '
    'Aziende ≤ 50 sal. : TFR provisionné chez l''employeur. '
    'Aziende > 50 sal. : versé mensuellement au Fondo Tesoreria INPS ou fonds de pension '
    '(L. 296/2006 art. 1 c. 755 — obligatoire depuis 01/01/2007). '
    'Payé au salarié à la fin du contrat (démission, licenciement, retraite). '
    'Fiscalité : tassazione separata (taux moyen sur 5 ans < taux marginal IRPEF).'),

  ('IT_ESONERO_2022', 'Esonero contributivo H2 2022 — Taglio cuneo salarié (−0,8 %)',
    (SELECT id FROM organisme WHERE code='INPS'),
    'ALLEGEMENT', 1, 1, 'BRUT_TOTAL',
    'Réduction de la cotisation IVS salarié. Reddito annuel estimé ≤ 35 000 €. '
    'Juillet–décembre 2022 uniquement. Taux : −0,80 point de %. '
    'DL 115/2022 art. 20, conv. L. 142/2022.'),

  ('IT_ESONERO_2023', 'Esonero contributivo 2023 — Taglio cuneo salarié (−2/−3 %)',
    (SELECT id FROM organisme WHERE code='INPS'),
    'ALLEGEMENT', 1, 1, 'BRUT_TOTAL',
    'Réduction de la cotisation IVS salarié. '
    '−3 pp si reddito annuel estimé ≤ 25 000 € ; −2 pp si ≤ 35 000 €. '
    'Le taux effectif est calculé en Rust selon l''estimation de revenu. '
    'L. 197/2022 art. 1 c. 281-286.');

-- ── Taux historiques ─────────────────────────────────────────

-- IVS : stable depuis la réforme Dini (L. 335/1995)
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_IVS'),
   '2015-01-01', NULL, '0.0919', '0.2381',
   'Total 33 % inchangé depuis les années 1990. '
   'Salariés post-1995 : base plafonnée au massimale contributivo annuel. '
   'Salariés pré-1996 : base = rémunération totale sans plafond.');

-- NASpI (même taux que l''ASpI depuis L. 92/2012)
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_NASPI'),
   '2015-01-01', NULL, '0.0000', '0.0161',
   'Taux inchangé depuis 2012 (création ASpI). '
   'Cotisation salarié = 0 depuis L. 228/2012 (01/01/2013).');

-- NASpI CDD (en vigueur depuis L. 92/2012 — 01/01/2013)
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_NASPI_TERMINE'),
   '2013-01-01', NULL, '0.0000', '0.0140',
   'Applicable uniquement aux CDD. '
   'Non applicable : CDD de remplacement, saisonniers, apprentissage, '
   'stagiaires, travailleurs intermittents.');

-- Malattia (taux indicatif commercio/industria)
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_MALATTIA'),
   '2015-01-01', NULL, '0.0000', '0.0222',
   'Taux indicatif secteur commercio / industria générique. '
   'Le CCNL applicable détermine le taux réel. '
   'Certains CCNL intègrent la malattia dans l''aliquota globale INPS.');

-- Maternità
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_MATERNITA'),
   '2015-01-01', NULL, '0.0000', '0.0046',
   'Stable depuis plusieurs décennies. Couvre maternité + paternité.');

-- Fondo di garanzia TFR
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_FONDO_GARANZIA'),
   '2015-01-01', NULL, '0.0000', '0.0020',
   'L. 297/1982 art. 2. Stable. '
   'Versé à l''INPS (code F24 tributo GFFT). '
   'Le versement du TFR au Fondo Tesoreria INPS (aziende > 50 sal.) '
   'est distinct et représente 6,91 % du brut mensuel.');

-- INAIL (taux indicatif bureau)
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_INAIL'),
   '2015-01-01', NULL, '0.0000', '0.0065',
   'Taux indicatif voce tariffa bureau / terziario (ATECO 63.11 / 70.22). '
   'Fixé par DM tariffe INAIL (révision triennale). '
   'Fluctue selon sinistralité de l''entreprise et secteur.');

-- TFR (accrual 6,91 %)
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_TFR'),
   '2015-01-01', NULL, '0.0000', '0.0691',
   'Taux = 1/13,5 ≈ 6,91 %. Stable depuis L. 297/1982. '
   'Ligne indicative de l''accrual mensuel — non versée directement à l''INPS '
   'pour les aziende ≤ 50 salariés (provisionnée en comptabilité).');

-- Esonero H2 2022 : −0,80 % salarié uniquement
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_ESONERO_2022'),
   '2022-07-01', '2022-12-31', '-0.0080', '0.0000',
   'Applicable si reddito annuel estimé ≤ 35 000 €. '
   'Conditionnel : le Rust vérifie le seuil de revenu avant application. '
   'DL 115/2022 art. 20 (conv. L. 142/2022).');

-- Esonero 2023 : taux max stocké (−3 pp), ajusté en Rust selon revenu
INSERT INTO cotisation_taux (cotisation_id, date_debut, date_fin,
  taux_salarial, taux_patronal, notes) VALUES
  ((SELECT id FROM cotisation WHERE code='IT_ESONERO_2023'),
   '2023-01-01', '2023-12-31', '-0.0300', '0.0000',
   'Taux maximum stocké (−3 pp pour reddito ≤ 25 000 €). '
   'Le Rust applique −2 pp pour reddito 25 001–35 000 €, '
   'et 0 au-delà de 35 000 €. L. 197/2022 art. 1 c. 281-286.');
