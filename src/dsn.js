// ═════════════════════════════════════════════════════════════════════════════
// DSN — Déclaration Sociale Nominative (norme NEODeS)
// ═════════════════════════════════════════════════════════════════════════════
//
// Ce module fabrique un EXTRAIT de DSN mensuelle à partir d'un bulletin déjà
// calculé, puis l'affiche annoté : chaque ligne du fichier est accompagnée du
// libellé officiel de la rubrique, de la signification de la valeur, et — pour
// les rubriques à liste fermée — de la liste complète des valeurs autorisées.
//
// ── Pourquoi côté front ? ────────────────────────────────────────────────────
// La DSN n'est pas un calcul : c'est une TRADUCTION du bulletin déjà calculé
// par le back Rust. Or deux données nécessaires ne vivent que côté front : le
// prélèvement à la source (calculé par calculerPas(), cf. main.js) et la date
// de paie du formulaire. Le module se contente donc de relire un Bulletin.
//
// ── Ce que ce module N'EST PAS ───────────────────────────────────────────────
// Le fichier produit n'est PAS déposable sur net-entreprises : il est marqué
// « envoi de test » (S10.G00.00.005 = 01), les identifiants (SIREN, NIC, SIRET
// des organismes) sont des zéros non attribuables, et l'individu est déclaré
// sous un NTT et non sous un NIR. Voir la section « lacunes » du rendu, qui
// énumère explicitement les blocs et rubriques absents.
//
// ── Sources du référentiel ───────────────────────────────────────────────────
// Libellés de blocs, de rubriques et listes de valeurs : cahier technique
// NEODeS CT2026.1 (norme P26V01, en production depuis le 21/01/2026), tel que
// publié rubrique par rubrique sur payrooll.com/dsn (miroir documentaire du
// cahier technique net-entreprises).
// Codes types de personnel (CTP) : table Urssaf en open data,
// open.urssaf.fr — dataset « histocodestypescsv », millésime en vigueur.
// Codification Agirc-Arrco (codes 105/106, rattachement à la base 03,
// répartition Urssaf / Agirc-Arrco de la réduction générale) : « Cahier d'aide
// à la codification pour la retraite complémentaire Agirc-Arrco ».
//
// Les tables ci-dessous ont été GÉNÉRÉES depuis ces sources, pas retapées :
// ne pas les corriger à la main sans revenir à la source.
// ═════════════════════════════════════════════════════════════════════════════

export const DSN_BLOCS = {
  'S10.G00.00': 'Envoi',
  'S10.G00.01': 'Emetteur',
  'S10.G00.02': 'Contact Emetteur',
  'S20.G00.05': 'Déclaration',
  'S20.G00.07': 'Contact chez le déclaré',
  'S21.G00.06': 'Entreprise',
  'S21.G00.11': 'Établissement',
  'S21.G00.15': 'Adhésion Prévoyance',
  'S21.G00.20': 'Versement organisme de protection sociale',
  'S21.G00.22': 'Bordereau de cotisation due',
  'S21.G00.23': 'Cotisation agrégée',
  'S21.G00.30': 'Individu',
  'S21.G00.40': 'Contrat (contrat de travail, convention, mandat)',
  'S21.G00.50': 'Versement individu',
  'S21.G00.51': 'Rémunération',
  'S21.G00.52': 'Prime, gratification et indemnité',
  'S21.G00.54': 'Autre élément de revenu brut',
  'S21.G00.62': 'Fin du contrat',
  'S21.G00.70': 'Affiliation Prévoyance',
  'S21.G00.71': 'Retraite complémentaire',
  'S21.G00.78': 'Base assujettie',
  'S21.G00.79': 'Composant de base assujettie',
  'S21.G00.81': 'Cotisation individuelle',
  'S21.G00.86': 'Ancienneté',
  'S90.G00.90': 'Total de l\'envoi',
};

export const DSN_RUBRIQUES = {
  'S10.G00.00.001': 'Nom du logiciel utilisé',
  'S10.G00.00.002': 'Nom de l\'éditeur',
  'S10.G00.00.003': 'Numéro de version du logiciel utilisé',
  'S10.G00.00.004': 'Code de conformité en pré-contrôle',
  'S10.G00.00.005': 'Code envoi du fichier d\'essai ou réel',
  'S10.G00.00.006': 'Numéro de version de la norme utilisée',
  'S10.G00.00.007': 'Point de dépôt',
  'S10.G00.00.008': 'Type de l\'envoi',
  'S10.G00.01.001': 'Siren de l\'émetteur de l\'envoi',
  'S10.G00.01.002': 'Nic de l\'émetteur de l\'envoi',
  'S10.G00.01.003': 'Nom ou raison sociale de l\'émetteur',
  'S10.G00.01.004': 'Numéro, extension, nature et libellé de la voie',
  'S10.G00.01.005': 'Code postal',
  'S10.G00.01.006': 'Localité',
  'S10.G00.01.007': 'Code pays',
  'S10.G00.01.008': 'Code de distribution à l\'étranger',
  'S10.G00.01.009': 'Complément de la localisation de la construction',
  'S10.G00.01.010': 'Service de distribution, complément de localisation de la voie',
  'S10.G00.02.001': 'Code civilité',
  'S10.G00.02.002': 'Nom et prénom de la personne à contacter',
  'S10.G00.02.004': 'Adresse email du contact émetteur',
  'S10.G00.02.005': 'Adresse téléphonique',
  'S10.G00.02.006': 'Adresse fax',
  'S20.G00.05.001': 'Nature de la déclaration',
  'S20.G00.05.002': 'Type de la déclaration',
  'S20.G00.05.003': 'Numéro de fraction de déclaration',
  'S20.G00.05.004': 'Numéro d\'ordre de la déclaration',
  'S20.G00.05.005': 'Date du mois principal déclaré',
  'S20.G00.05.006': 'Identifiant de la déclaration annulée ou remplacée',
  'S20.G00.05.007': 'Date de constitution du fichier',
  'S20.G00.05.008': 'Champ de la déclaration',
  'S20.G00.05.009': 'Identifiant métier',
  'S20.G00.05.010': 'Devise de la déclaration',
  'S20.G00.05.011': 'Nature de l\'événement déclencheur du signalement',
  'S20.G00.05.012': 'Dernier SIRET connu pour ancien numéro de contrat',
  'S20.G00.05.013': 'Type de nature de la DSN de substitution',
  'S20.G00.07.001': 'Nom et prénom du contact',
  'S20.G00.07.002': 'Adresse téléphonique',
  'S20.G00.07.003': 'Adresse email du contact',
  'S20.G00.07.004': 'Type',
  'S21.G00.06.001': 'SIREN',
  'S21.G00.06.002': 'NIC du siège',
  'S21.G00.06.003': 'Code APEN',
  'S21.G00.06.004': 'Numéro, extension, nature et libellé de la voie',
  'S21.G00.06.005': 'Code postal',
  'S21.G00.06.006': 'Localité',
  'S21.G00.06.007': 'Complément de la localisation de la construction',
  'S21.G00.06.008': 'Service de distribution, complément de localisation de la voie',
  'S21.G00.06.010': 'Code pays',
  'S21.G00.06.011': 'Code de distribution à l\'étranger',
  'S21.G00.06.012': 'Implantation de l\'entreprise',
  'S21.G00.06.015': 'Code convention collective applicable',
  'S21.G00.11.001': 'NIC',
  'S21.G00.11.002': 'Code APET',
  'S21.G00.11.003': 'Numéro, extension, nature et libellé de la voie',
  'S21.G00.11.004': 'Code postal',
  'S21.G00.11.005': 'Localité',
  'S21.G00.11.006': 'Complément de la localisation de la construction',
  'S21.G00.11.007': 'Service de distribution, complément de localisation de la voie',
  'S21.G00.11.009': 'Type de rémunération soumise à contributions d\'Assurance chômage pour expatriés',
  'S21.G00.11.015': 'Code pays',
  'S21.G00.11.016': 'Code de distribution à l\'étranger',
  'S21.G00.11.017': 'Nature juridique de l\'employeur',
  'S21.G00.11.019': 'Date d\'effet de l\'adhésion au dispositif TESE/CEA',
  'S21.G00.11.020': 'Date d\'effet de la sortie du dispositif TESE/CEA',
  'S21.G00.11.022': 'Code convention collective principale',
  'S21.G00.11.023': 'Opérateur de compétences (OPCO)',
  'S21.G00.11.024': 'Demande de sortie de la DSN',
  'S21.G00.11.025': 'Identifiant du Service de Prévention et de Santé au Travail (SPST)',
  'S21.G00.15.001': 'Référence du contrat de Prévoyance',
  'S21.G00.15.002': 'Code organisme de Prévoyance',
  'S21.G00.15.003': 'Code délégataire de gestion',
  'S21.G00.15.004': 'Personnel couvert',
  'S21.G00.15.005': 'Identifiant technique Adhésion',
  'S21.G00.20.001': 'Identifiant Organisme de Protection Sociale',
  'S21.G00.20.002': 'Entité d\'affectation des opérations',
  'S21.G00.20.003': 'BIC',
  'S21.G00.20.004': 'IBAN',
  'S21.G00.20.005': 'Montant du versement',
  'S21.G00.20.006': 'Date de début de période de rattachement',
  'S21.G00.20.007': 'Date de fin de période de rattachement',
  'S21.G00.20.008': 'Code délégataire de gestion',
  'S21.G00.20.010': 'Mode de paiement',
  'S21.G00.20.011': 'Date de paiement',
  'S21.G00.20.012': 'SIRET Payeur',
  'S21.G00.20.013': 'Identifiant du CRM à l\'origine de la régularisation',
  'S21.G00.20.014': 'Identifiant du versement',
  'S21.G00.22.001': 'Identifiant Organisme de Protection Sociale',
  'S21.G00.22.002': 'Entité d\'affectation des opérations',
  'S21.G00.22.003': 'Date de début de période de rattachement',
  'S21.G00.22.004': 'Date de fin de période de rattachement',
  'S21.G00.22.005': 'Montant total de cotisations',
  'S21.G00.22.006': 'Identifiant du CRM à l\'origine de la régularisation',
  'S21.G00.23.001': 'Code de cotisation',
  'S21.G00.23.002': 'Qualifiant d\'assiette',
  'S21.G00.23.003': 'Taux de cotisation',
  'S21.G00.23.004': 'Montant d\'assiette',
  'S21.G00.23.005': 'Montant de cotisation',
  'S21.G00.23.006': 'Code INSEE commune',
  'S21.G00.23.007': 'Identifiant du CRM à l\'origine de la régularisation',
  'S21.G00.30.001': 'Numéro d\'inscription au répertoire (NIR)',
  'S21.G00.30.002': 'Nom de famille',
  'S21.G00.30.003': 'Nom d\'usage',
  'S21.G00.30.004': 'Prénoms',
  'S21.G00.30.005': 'Sexe',
  'S21.G00.30.006': 'Date de naissance',
  'S21.G00.30.007': 'Lieu de naissance',
  'S21.G00.30.008': 'Numéro, extension, nature et libellé de la voie',
  'S21.G00.30.009': 'Code postal',
  'S21.G00.30.010': 'Localité',
  'S21.G00.30.011': 'Code pays',
  'S21.G00.30.012': 'Code de distribution à l\'étranger',
  'S21.G00.30.013': 'Codification UE',
  'S21.G00.30.014': 'Code département de naissance',
  'S21.G00.30.015': 'Code pays de naissance',
  'S21.G00.30.016': 'Complément de la localisation de la construction',
  'S21.G00.30.017': 'Service de distribution, complément de localisation de la voie',
  'S21.G00.30.018': 'Adresse email',
  'S21.G00.30.019': 'Matricule de l\'individu dans l\'entreprise',
  'S21.G00.30.020': 'Numéro technique temporaire',
  'S21.G00.30.022': 'Statut à l\'étranger au sens fiscal',
  'S21.G00.30.023': 'Cumul emploi retraite',
  'S21.G00.30.024': 'Niveau de formation le plus élevé obtenu par l\'individu',
  'S21.G00.30.025': 'Niveau de diplôme préparé par l\'individu',
  'S21.G00.30.029': 'Libellé du pays de naissance',
  'S21.G00.30.030': 'Identifiant du Service de Prévention et de Santé au Travail (SPST)',
  'S21.G00.40.001': 'Date de début du contrat',
  'S21.G00.40.002': 'Statut du salarié (conventionnel)',
  'S21.G00.40.003': 'Code statut catégoriel Retraite Complémentaire obligatoire',
  'S21.G00.40.004': 'Code profession et catégorie socioprofessionnelle (PCS-ESE)',
  'S21.G00.40.005': 'Code complément PCS-ESE (pour la fonction publique : référentiels NEH, NET et grade de la NNE)',
  'S21.G00.40.006': 'Libellé de l\'emploi',
  'S21.G00.40.007': 'Nature du contrat',
  'S21.G00.40.008': 'Dispositif de politique publique et conventionnel',
  'S21.G00.40.009': 'Numéro du contrat',
  'S21.G00.40.010': 'Date de fin prévisionnelle du contrat',
  'S21.G00.40.011': 'Unité de mesure de la quotité de travail',
  'S21.G00.40.012': 'Quotité de travail de référence de l\'entreprise pour la catégorie de salarié',
  'S21.G00.40.013': 'Quotité de travail du contrat',
  'S21.G00.40.014': 'Modalité d\'exercice du temps de travail',
  'S21.G00.40.016': 'Complément de base au régime obligatoire',
  'S21.G00.40.017': 'Code convention collective applicable',
  'S21.G00.40.018': 'Code régime de base risque maladie',
  'S21.G00.40.019': 'Identifiant du lieu de travail',
  'S21.G00.40.020': 'Code régime de base risque vieillesse',
  'S21.G00.40.021': 'Motif de recours',
  'S21.G00.40.022': 'Code caisse professionnelle de congés payés',
  'S21.G00.40.023': 'Taux de déduction forfaitaire spécifique pour frais professionnels',
  'S21.G00.40.024': 'Travailleur à l\'étranger au sens du code de la Sécurité Sociale',
  'S21.G00.40.025': 'Motif d\'exclusion DSN',
  'S21.G00.40.026': 'Statut d\'emploi du salarié',
  'S21.G00.40.027': 'Code affectation Assurance chômage',
  'S21.G00.40.028': 'Numéro interne employeur public',
  'S21.G00.40.029': 'Type de gestion de l\'Assurance chômage',
  'S21.G00.40.030': 'Date d\'adhésion',
  'S21.G00.40.031': 'Date de dénonciation',
  'S21.G00.40.032': 'Date d\'effet de la convention de gestion',
  'S21.G00.40.033': 'Numéro de convention de gestion',
  'S21.G00.40.035': 'Code délégataire du risque maladie',
  'S21.G00.40.036': 'Code emplois multiples',
  'S21.G00.40.037': 'Code employeurs multiples',
  'S21.G00.40.039': 'Code régime de base risque accident du travail',
  'S21.G00.40.040': 'Code risque accident du travail',
  'S21.G00.40.041': 'Positionnement dans la convention collective',
  'S21.G00.40.042': 'Code statut catégoriel APECITA',
  'S21.G00.40.043': 'Taux de cotisation accident du travail',
  'S21.G00.40.044': 'Salarié à temps partiel cotisant à temps plein',
  'S21.G00.40.045': 'Rémunération au pourboire',
  'S21.G00.40.046': 'Identifiant de l\'établissement utilisateur',
  'S21.G00.40.048': 'Numéro de label « Prestataire de services du spectacle vivant »',
  'S21.G00.40.049': 'Numéro de licence entrepreneur spectacle',
  'S21.G00.40.050': 'Numéro objet spectacle',
  'S21.G00.40.051': 'Statut organisateur spectacle',
  'S21.G00.40.052': '[FP] Code complément PCS-ESE pour la fonction publique d\'Etat (emploi de la NNE)',
  'S21.G00.40.053': 'Nature du poste',
  'S21.G00.40.054': '[FP] Quotité de travail de référence de l\'entreprise pour la catégorie de salarié dans l\'hypothèse d\'un poste à temps complet',
  'S21.G00.40.055': 'Taux de travail à temps partiel',
  'S21.G00.40.056': 'Code catégorie de service',
  'S21.G00.40.057': '[FP] Indice brut',
  'S21.G00.40.058': '[FP] Indice majoré',
  'S21.G00.40.059': '[FP] Nouvelle bonification indiciaire (NBI)',
  'S21.G00.40.060': '[FP] Indice brut d\'origine',
  'S21.G00.40.061': '[FP] Indice brut de cotisation dans un emploi supérieur (article 15)',
  'S21.G00.40.062': '[FP] Ancien employeur public',
  'S21.G00.40.063': '[FP] Indice brut d\'origine ancien salarié employeur public',
  'S21.G00.40.064': '[FP] Indice brut d\'origine sapeur-pompier professionnel (SPP)',
  'S21.G00.40.065': '[FP] Maintien du traitement d\'origine d\'un contractuel titulaire',
  'S21.G00.40.066': '[FP] Type de détachement',
  'S21.G00.40.067': 'Genre de navigation',
  'S21.G00.40.068': 'Taux de service actif',
  'S21.G00.40.069': 'Niveau de rémunération',
  'S21.G00.40.070': 'Echelon',
  'S21.G00.40.071': 'Coefficient hiérarchique',
  'S21.G00.40.072': 'Statut BOETH',
  'S21.G00.40.073': 'Complément de dispositif de politique publique',
  'S21.G00.40.074': 'Cas de mise à disposition externe d\'un individu de l\'établissement',
  'S21.G00.40.075': 'Catégorie de classement finale',
  'S21.G00.40.076': 'Identifiant du contrat d\'engagement maritime',
  'S21.G00.40.077': 'Collège (CNIEG)',
  'S21.G00.40.078': 'Forme d\'aménagement du temps de travail dans le cadre de l\'activité partielle',
  'S21.G00.40.079': 'Grade',
  'S21.G00.40.080': '[FP] Indice complément de traitement indiciaire (CTI)',
  'S21.G00.40.081': 'FINESS géographique',
  'S21.G00.40.082': 'Nombre de jours de période d\'essai',
  'S21.G00.40.083': 'Heure prévisible d\'embauche',
  'S21.G00.50.001': 'Date de versement',
  'S21.G00.50.002': 'Rémunération nette fiscale',
  'S21.G00.50.003': 'Numéro de versement',
  'S21.G00.50.004': 'Montant net versé',
  'S21.G00.50.006': 'Taux de prélèvement à la source',
  'S21.G00.50.007': 'Type du taux de prélèvement à la source',
  'S21.G00.50.008': 'Identifiant du taux de prélèvement à la source',
  'S21.G00.50.009': 'Montant de prélèvement à la source',
  'S21.G00.50.011': 'Montant de la part non imposable du revenu',
  'S21.G00.50.012': 'Montant de l\'abattement sur la base fiscale (non déduit de la rémunération nette fiscale)',
  'S21.G00.50.013': 'Montant soumis au PAS',
  'S21.G00.50.020': 'Mois de la DSN mensuelle de rattachement des éléments déclarés dans le FCTU',
  'S21.G00.51.001': 'Date de début de période de paie',
  'S21.G00.51.002': 'Date de fin de période de paie',
  'S21.G00.51.010': 'Numéro du contrat',
  'S21.G00.51.011': 'Type',
  'S21.G00.51.012': 'Nombre d\'heures',
  'S21.G00.51.013': 'Montant',
  'S21.G00.51.014': '[FP] Taux de rémunération de la situation administrative',
  'S21.G00.51.015': 'Taux de conduite centrale nucléaire',
  'S21.G00.51.016': 'Taux de majoration',
  'S21.G00.51.019': 'Taux de rémunération cotisée',
  'S21.G00.51.020': 'Taux de majoration ex-apprenti/ex-élève',
  'S21.G00.52.001': 'Type',
  'S21.G00.52.002': 'Montant',
  'S21.G00.52.003': 'Date de début de la période de rattachement',
  'S21.G00.52.004': 'Date de fin de la période de rattachement',
  'S21.G00.52.006': 'Numéro du contrat',
  'S21.G00.52.007': 'Date de versement d\'origine',
  'S21.G00.54.001': 'Type',
  'S21.G00.54.002': 'Montant',
  'S21.G00.54.003': 'Date de début de période de rattachement',
  'S21.G00.54.004': 'Date de fin de période de rattachement',
  'S21.G00.54.005': 'Numéro de contrat',
  'S21.G00.62.001': 'Date de fin du contrat',
  'S21.G00.62.002': 'Motif de la rupture du contrat',
  'S21.G00.62.003': 'Date de notification de la rupture de contrat',
  'S21.G00.62.004': 'Date de signature de la convention de rupture',
  'S21.G00.62.005': 'Date d\'engagement de la procédure de licenciement',
  'S21.G00.62.006': 'Dernier jour travaillé et payé au salaire habituel',
  'S21.G00.62.008': 'Transaction en cours',
  'S21.G00.62.011': 'Nombre de mois de préavis utilisés dans le cadre du calcul CSP',
  'S21.G00.62.013': 'Montant de l\'indemnité de préavis qui aurait été versée',
  'S21.G00.62.014': 'Statut particulier du salarié',
  'S21.G00.62.016': 'Maintien de l\'affiliation du salarié au contrat collectif',
  'S21.G00.62.017': 'Modalité de déclaration de la fin du contrat d\'usage',
  'S21.G00.62.018': 'Nombre de mois de préavis utilisés dans le cadre du calcul PAP',
  'S21.G00.62.019': 'Solde de congés acquis et non pris (ENIM)',
  'S21.G00.62.020': 'Mois de la DSN mensuelle portant les derniers éléments déclarés dans le FCTU',
  'S21.G00.62.021': 'Refus de la proposition d\'un CDI suite à CDD ou contrat de mission',
  'S21.G00.70.004': 'Code option retenue par le salarié',
  'S21.G00.70.005': 'Code population de rattachement',
  'S21.G00.70.007': 'Nombre d\'enfants à charge',
  'S21.G00.70.008': 'Nombre d\'ayants-droit conjoint / concubin',
  'S21.G00.70.009': 'Nombre d\'ayants-droit attachés au salarié pour le contrat Prévoyance mentionné',
  'S21.G00.70.010': 'Nombre d\'ayants-droit autres (ascendants, collatéraux...)',
  'S21.G00.70.011': 'Nombre d\'enfants ayants-droit',
  'S21.G00.70.012': 'Identifiant technique Affiliation',
  'S21.G00.70.013': 'Identifiant technique Adhésion',
  'S21.G00.70.014': 'Date de début de l\'affiliation',
  'S21.G00.70.015': 'Date de fin de l\'affiliation',
  'S21.G00.71.002': 'Code régime Retraite Complémentaire',
  'S21.G00.78.001': 'Code de base assujettie',
  'S21.G00.78.002': 'Date de début de période de rattachement',
  'S21.G00.78.003': 'Date de fin de période de rattachement',
  'S21.G00.78.004': 'Montant',
  'S21.G00.78.005': 'Identifiant technique Affiliation',
  'S21.G00.78.006': 'Numéro du contrat',
  'S21.G00.78.007': 'Identifiant du CRM à l\'origine de la régularisation',
  'S21.G00.79.001': 'Type de composant de base assujettie',
  'S21.G00.79.004': 'Montant de composant de base assujettie',
  'S21.G00.79.005': 'Identifiant du CRM à l\'origine de la régularisation',
  'S21.G00.81.001': 'Code de cotisation',
  'S21.G00.81.002': 'Identifiant Organisme de Protection Sociale',
  'S21.G00.81.003': 'Montant d\'assiette',
  'S21.G00.81.004': 'Montant de cotisation',
  'S21.G00.81.005': 'Code INSEE commune',
  'S21.G00.81.006': 'Identifiant du CRM à l\'origine de la régularisation',
  'S21.G00.81.007': 'Taux de cotisation',
  'S21.G00.86.001': 'Type',
  'S21.G00.86.002': 'Unité de mesure',
  'S21.G00.86.003': 'Valeur',
  'S21.G00.86.005': 'Numéro du contrat',
  'S90.G00.90.001': 'Nombre total de rubriques',
  'S90.G00.90.002': 'Nombre de DSN',
};

export const DSN_COT = {
  '001': 'Exonération de cotisations au titre de l\'emploi d\'un apprenti (loi de 1979)',
  '002': 'Exonération de cotisations au titre de l\'emploi d\'un apprenti (loi de 1987)',
  '003': 'Exonération de cotisations au titre de l\'emploi d\'un apprenti (loi de 1992)',
  '004': 'Exonération de cotisations au titre de l\'emploi d\'un salarié en contrat d\'accès à l\'emploi',
  '006': 'Exonération de cotisations au titre de l\'emploi d\'un salarié en contrat d\'accompagnement dans l\'emploi',
  '008': 'Exonération de cotisations au titre de l\'emploi d\'un salarié en contrat de professionnalisation',
  '009': 'Exonération de cotisations applicable aux associations intermédiaires',
  '010': 'Exonération de cotisations applicable aux entreprises des bassins d\'emploi à redynamiser',
  '011': 'Exonération de cotisations applicable au créateur ou repreneur d\'entreprise',
  '012': 'Exonération de cotisations applicable dans les DOM',
  '013': 'Exonération de cotisations applicable aux entreprises et associations d\'aide à domicile',
  '014': 'Exonérations de cotisations applicable aux entreprises innovantes, universitaires ou de croissance',
  '015': 'Exonération de cotisations applicable aux entreprises en zones franches urbaines',
  '016': 'Exonération de cotisations applicable aux organismes d\'intérêt général en zones de revitalisation rurale',
  '017': 'Exonération de cotisations applicable aux structures agréées de l\'aide sociale',
  '018': 'Réduction générale des cotisations patronales de sécurité sociale et d\'assurance chômage',
  '019': 'Réduction de cotisations applicable aux entreprises des zones de restructuration de la défense',
  '020': 'Réduction de cotisations au titre de l\'embauche du 1er au 50ème salarié en zones de revitalisation rurale',
  '021': 'Déduction patronale au titre des heures supplémentaires',
  '022': 'Exonération de cotisations applicable à une gratification de stage',
  '023': 'Exonération de cotisation des sommes provenant d\'un CET et réaffectées à un plan d\'épargne retraite (PERCO, PERECO, PEREO) ou à un régime de retraite supplémentaire',
  '025': 'Exonération de cotisations au titre de l’emploi d’un salarié en chantier et atelier d\'insertion',
  '027': 'Exonération Personnel technique CUMA, hors ateliers',
  '028': 'Réduction Travailleur Occasionnel',
  '029': 'Réduction employeurs petit pool IEG',
  '030': 'Cotisation employeurs régime spécial maladie IEG Complémentaire',
  '031': 'Cotisation salariés régime spécial maladie IEG Complémentaire',
  '032': 'Cotisation salariés régime maladie IEG spécial Solidarité',
  '033': 'Cotisation employeurs complément d\'invalidité IEG',
  '034': 'Cotisation employeurs régime de droit commun IEG (population adossée)',
  '035': 'Cotisation employeurs régime spécial IEG (population adossée)',
  '036': 'Cotisation employeurs régime spécial IEG (population non adossée)',
  '037': 'Cotisation salariés régime de droit commun IEG (population adossée)',
  '038': 'Cotisation salariés régime spécial IEG (population non adossée)',
  '039': 'Cotisations employeurs petit pool IEG',
  '040': 'Cotisation AC : assurance chômage sur rémunérations brutes après déduction',
  '041': 'Cotisation AC majorée 1 : application d’une majoration AC + 0,5% sur les contrats d’usage inférieurs ou égaux à 3 mois',
  '042': 'Cotisation AC majorée 2 : application d’une majoration AC + 3% sur les contrats d’accroissement temporaire d’activité inférieurs ou égaux à 1 mois',
  '043': 'Cotisation AC majorée 3 : application d’une majoration AC + 1,5% sur les contrats d’accroissement temporaire d’activité supérieurs à 1 mois mais inférieurs ou égaux à 3 mois',
  '044': 'Exonération de cotisation chômage pour les moins de 26 ans',
  '045': 'Cotisation Accident du travail',
  '046': 'Cotisation AEF Bourse de l\'emploi',
  '047': 'Cotisation AEF CESA',
  '048': 'Cotisation AGS : assurance garantie des salaires sur rémunérations brutes après déduction',
  '049': 'Cotisation Allocation de logement (FNAL)',
  '051': 'Cotisation Formation professionnelle ADEFA',
  '053': 'Cotisation Formation professionnelle additionnelle FAFSEA',
  '054': 'Cotisation Formation professionnelle AREFA',
  '056': 'Cotisation Formation professionnelle FAFSEA',
  '057': 'Cotisation Formation professionnelle FAFSEA CDD',
  '058': 'Cotisation Formation professionnelle FAFSEA des communes forestières',
  '059': 'Cotisation individuelle Prévoyance-Assurance-Mutuelle pour la période et l\'affiliation concernées',
  '060': 'Cotisation IRCANTEC Tranche A',
  '061': 'Cotisation IRCANTEC Tranche B',
  '065': 'Cotisation CRPCEN',
  '066': 'Cotisation caisse de congés spectacles',
  '068': 'Contribution solidarité autonomie',
  '069': 'Contribution sur avantage de pré-retraite entreprise à dater du 11/10/2007 (CAPE)',
  '070': 'Contribution sur avantage de pré-retraite entreprise aux taux normal (CAPE)',
  '071': 'Contribution forfait social',
  '072': 'Contribution sociale généralisée/salaires partiellement déductibles',
  '073': 'CSG/CRDS sur participation intéressement épargne salariale',
  '074': 'Cotisation Allocation familiale - taux normal',
  '075': 'Cotisation Assurance Maladie',
  '076': 'Cotisation Assurance Vieillesse',
  '077': 'Montant de la retenue à la source effectuée sur les salaires versés aux personnes domiciliées hors de France',
  '078': 'Pénalité de 1% emploi sénior',
  '079': 'Remboursement de la dette sociale',
  '081': 'Versement mobilité',
  '082': 'Versement mobilité additionnel',
  '088': 'Exonération versement mobilité',
  '089': 'Exonération Contrat Initiative Emploi',
  '090': 'Exonération accueillants familiaux',
  '091': 'Cotisation Service de santé au travail',
  '092': 'Cotisation Association pour l\'emploi des cadres ingénieurs et techniciens de l\'agriculture (APECITA)',
  '093': 'Contribution sur indemnités de mise à la retraite ou de rupture conventionnelle individuelle',
  '094': 'Exonération cotisations Allocations familiales (SICAE)',
  '096': 'Cotisation CRPNPAC au fonds de retraite',
  '097': 'Cotisation CRPNPAC au fonds d\'assurance',
  '098': 'Cotisation CRPNPAC au fonds de majoration',
  '099': 'Contribution stock options',
  '100': 'Contribution au financement du dialogue social',
  '101': 'Association Mutualisation du Coût Inaptitude',
  '102': 'Complément de cotisation Allocation Familiale',
  '103': 'Contribution actions gratuites',
  '104': 'Pénibilité Cotisation de base',
  '105': 'Montant de cotisation Régime Unifié Agirc-Arrco, y compris Apec',
  '106': 'Réduction générale des cotisations patronales de retraite complémentaire',
  '107': 'Forfait marin',
  '108': 'Demi-rôle marin',
  '109': 'Exonération de cotisations salariales de retraite complémentaire au titre de l\'emploi d\'un apprenti',
  '110': 'Exonération de cotisations patronales de retraite complémentaire applicable dans les DOM (LODEOM) SMIC 130% à 220%',
  '111': 'Exonération de cotisations de retraite complémentaire applicable aux entreprises et associations d\'aide à domicile',
  '112': 'Exonération de cotisations patronales de retraite complémentaire applicable dans les DOM (LODEOM) SMIC 170% à 270%',
  '113': 'Exonération de cotisations patronales de retraite complémentaire applicable dans les DOM (LODEOM) SMIC 170% à 350%',
  '114': 'Montant de réduction des heures supplémentaires/complémentaires',
  '115': 'Cotisation Assurance Maladie pour le Régime Local Alsace Moselle',
  '116': 'Cotisation absente de la norme en cas de régularisation prud\'homale',
  '128': 'Contribution à la formation professionnelle (CFP)',
  '129': 'Contribution dédiée au financement du Compte Personnel de Formation pour les titulaires de CDD (CPF-CDD)',
  '130': 'Part principale de la taxe d\'apprentissage',
  '131': 'Cotisation régime unifié Agirc-Arrco',
  '132': 'Cotisation Apec',
  '133': 'Contribution maladie spécifique Mayotte',
  '140': 'Contribution conventionnelle au financement du dialogue social',
  '141': 'Contribution conventionnelle à la formation professionnelle',
  '142': 'Cotisation régime unifié Agirc-Arrco (part patronale tranche T1)',
  '143': 'Contribution au compte professionnel de prévention (C2P)',
  '144': 'Cotisation CAVEC - classe C',
  '145': 'Cotisation CAVEC - classe D',
  '146': 'Cotisation régime unifié Agirc-Arrco (part patronale tranche T2)',
  '300': '[FP] Cotisations normales (part salariale)',
  '301': '[FP] Cotisations normales (part patronale)',
  '302': '[FP] Surcotisation huit trimestres (part salariale)',
  '303': '[FP] Validation de services (part salariale)',
  '304': '[FP] Cotisations Nouvelle Bonification Indiciaire (part salariale)',
  '305': '[FP] Cotisations Nouvelle Bonification Indiciaire (part patronale)',
  '306': '[FP] Cotisations sur indemnité de feu (part salariale)',
  '307': '[FP] Cotisations sur indemnité de feu (part patronale)',
  '308': '[FP] Cotisation sur bonification sapeur pompier (part salariale)',
  '309': '[FP] Cotisation sur prime sur sujetion des aides soignantes (part salariale)',
  '310': '[FP] Cotisation sur prime sur sujetion des aides soignantes (part patronale)',
  '311': '[FP] Cotisation RAFP (part salariale)',
  '312': '[FP] Cotisation RAFP (part patronale)',
  '313': '[FP] Cotisations pour pension sur ISS ou PSS (part salariale)',
  '314': '[FP] Cotisations pour pension sur ISS ou PSS (part patronale)',
  '315': '[FP] Cotisations pour pension sur IR (part salariale)',
  '316': '[FP] Cotisations pour pension sur IR (part patronale)',
  '317': '[FP] Cotisations pour pension sur IMT (part salariale)',
  '318': '[FP] Cotisations pour pension sur IMT (part patronale)',
  '319': '[FP] Cotisations pour l\'allocation temporaire d\'invalidité (part patronale)',
  '320': '[FP] Surcotisation (part salariale)',
  '321': '[FP] Rachat des années d\'études (part salariale)',
  '322': '[FP] Exonération de cotisation pour heures d’aide à domicile (part patronale)',
  '323': '[FP] Cotisation RAEP (part patronale)',
  '324': '[FP] Cotisation RAEP (part salariale)',
  '325': '[FP] Validation de services (part patronale)',
  '326': '[FP] Régularisation de service (part salariale)',
  '327': '[FP] Régularisation de service (part patronale)',
  '330': 'Régime de base forfaitaire CNBF',
  '331': 'Régime de base proportionnelle CNBF',
  '332': 'Régime complémentaire CNBF',
  '333': 'Cotisation contrat d’emploi pénitentiaire 1',
  '334': 'Cotisation contrat d’emploi pénitentiaire 2',
  '901': 'Cotisation épargne retraite',
  '902': 'Contribution à la formation professionnelle des Artisans assimilés salariés',
  '903': 'Cotisation AFNCA',
  '904': 'Cotisation ANEFA',
  '905': 'Cotisation ASCPA',
  '906': 'Cotisation PROVEA',
  '907': 'Complément de cotisation Assurance Maladie',
  '908': 'Taxe forfaitaire CDDU Assurance Chômage',
  '909': 'Cotisation au titre du financement des régimes de retraites supplémentaires à prestation définies',
  '910': 'Exonération de cotisations patronales pour les entreprises affectées par la crise sanitaire',
  '911': 'Réduction de cotisations patronales pour les entreprises du secteur de la vigne affectées par la crise sanitaire',
  '912': 'Exonération du forfait social à 10%',
  '913': 'Indemnité inflation',
  '914': 'Réduction sapeurs-pompiers volontaires sur les contributions recouvrées par la MSA, l’Urssaf ou France Travail',
  '915': 'Réduction sapeurs-pompiers volontaires sur les cotisations et contributions recouvrées par l’Agirc-Arrco',
  '916': 'Exonération de cotisations applicable en Zones France Ruralités Revitalisation (ZFRR)',
  '917': 'Potentielle nouvelle cotisation D',
  '918': 'Versement mobilité régional et rural',
  '919': 'Exonération de contribution patronale sur les indemnités de mise à la retraite d’un salarié en contrat de valorisation de l’expérience (CVE)',
  '920': 'Potentielle nouvelle cotisation B',
  '921': 'Potentielle nouvelle cotisation C',
};

export const DSN_BASE = {
  '02': 'Assiette  brute plafonnée',
  '03': 'Assiette brute déplafonnée',
  '04': 'Assiette de la contribution sociale généralisée',
  '05': 'Assiette du forfait social',
  '07': 'Assiette des contributions d\'Assurance Chômage',
  '08': 'Assiette retraite CPRPF',
  '09': 'Assiette de compensation bilatérale maladie CPRPF',
  '10': 'Base brute fiscale',
  '11': 'Base forfaitaire soumise aux cotisations de Sécurité Sociale',
  '12': 'Assiette du crédit d\'impôt compétitivité-emploi',
  '13': 'Assiette du forfait social à 8%',
  '14': 'Assiette du forfait social à 20%',
  '15': 'Assiette brute du régime spécial IEG',
  '16': 'Assiette brute du complément invalidité IEG',
  '17': 'Assiette brute du petit pool IEG',
  '18': 'Assiette brute plafonnée régime maladie IEG',
  '19': 'Assiette CRPCEN',
  '20': 'Caisses de congés payés (CIBTP, Transport, Manutention portuaire) - Base brute de cotisations congés payés',
  '21': 'CIBTP - Base brute de cotisations OPPBTP permanents',
  '22': 'Base brute spécifique',
  '23': 'Base exceptionnelle (Agirc Arrco)',
  '24': 'Base plafonnée spécifique',
  '25': 'Assiette de contribution libératoire',
  '27': 'Assiette Caisse de congés spectacles',
  '28': 'Base IRCANTEC cotisée',
  '31': 'Eléments de cotisation Prévoyance, Santé, retraite supplémentaire',
  '33': 'Assiette Contribution sur les avantages de préretraite entreprise',
  '34': 'CIBTP - Base plafonnée de cotisations intempéries gros oeuvre travaux publics',
  '35': 'CIBTP - Base plafonnée de cotisations intempéries second oeuvre',
  '36': 'CIBTP - Base de cotisations dérogatoire Bâtiment',
  '38': 'Rémunération pour le calcul de la réduction Travailleur Occasionnel',
  '39': 'CIBTP - Base de cotisations dérogatoire Travaux Publics',
  '40': 'CIBTP - Base de cotisations dérogatoire Partenaires Bâtiment',
  '41': 'CRPNPAC-Assiette soumise au taux normal (non-plafonnée)',
  '42': 'CRPNPAC-Assiette soumise au taux majoré (non-plafonnée)',
  '43': 'Base plafonnée exceptionnelle Agirc Arrco',
  '44': 'Assiette du forfait social à 16%',
  '45': 'Base plafonnée ICP Agirc-Arrco',
  '46': '[FP] SRE – Base brute pension civile et militaire',
  '47': '[FP] SRE – Base brute accessoires pension civile et militaire',
  '48': '[FP] CNRACL – Base brute avant abattement',
  '49': '[FP] RAFP – Base brute avant abattement',
  '50': '[FP] FSPOEIE - Base brute avant abattement',
  '52': 'Assiette spécifique Tranche 2 régime spécial RATP',
  '53': 'Assiette spécifique régime spécial RATP',
  '54': 'Assiette du forfait social à 10%',
  '55': 'Assiette de pénibilité conventionnelle de la Manutention portuaire',
  '56': 'Potentielle nouvelle base assujettie A',
  '57': 'Assiette du versement mobilité',
  '59': 'Base contrat d’emploi pénitentiaire 1',
  '60': 'Base contrat d’emploi pénitentiaire 2',
  '61': 'Potentielle nouvelle base assujettie D',
  '62': 'Potentielle nouvelle base assujettie E',
};

export const DSN_REM = {
  '001': 'Rémunération brute non plafonnée',
  '002': 'Salaire brut servant aux calculs des droits de l\'Assurance chômage',
  '003': 'Salaire rétabli – reconstitué',
  '010': 'Salaire de base',
  '012': 'Heures d’équivalence',
  '013': 'Heures d’habillage, déshabillage, pause',
  '016': '[FP] Heures affectées à un travail d’aide à domicile',
  '017': 'Heures supplémentaires ou complémentaires aléatoires',
  '018': 'Heures supplémentaires structurelles',
  '019': 'Heures d\'activité partielle',
  '020': 'Heures affectées à un travail d’aide à domicile de publics fragiles',
  '021': '[FP] Taux de rémunération de la situation administrative',
  '022': '[FP] Complément de traitement indiciaire (CTI)',
  '023': 'Jours de RTT monétisés',
  '025': 'Heures correspondant à du chômage intempéries',
  '028': 'Rémunération perçue hors éléments non affectés par l’absence',
  '029': 'Rémunération habituelle mois complet hors éléments non affectés par l’absence',
  '030': 'Potentiel nouveau type de rémunération A',
  '031': 'Potentiel nouveau type de rémunération D',
  '032': 'Potentiel nouveau type de rémunération E',
  '034': 'Potentiel nouveau type de rémunération B',
  '035': 'Potentiel nouveau type de rémunération C',
};

export const DSN_COMPOSANT = {
  '01': 'Montant du SMIC retenu pour le calcul de la Réduction générale des cotisations patronales de sécurité sociale, de retraite complémentaire et d\'assurance chômage',
  '02': 'Montant du SMIC retenu pour le calcul du crédit d\'impôt compétitivité-emploi',
  '03': 'Contributions patronales à des régimes complémentaires de retraite',
  '04': 'Contributions patronales destinées au financement des prestations de prévoyance complémentaire',
  '05': 'Contributions patronales destinées au financement des prestations de retraite supplémentaire',
  '06': 'Plafond calculé pour salarié poly-employeurs',
  '07': 'Plafond de Sécurité Sociale appliqué',
  '10': 'Salaire brut Prévoyance',
  '11': 'Tranche A Prévoyance',
  '12': 'Tranche 2 Prévoyance',
  '13': 'Tranche B Prévoyance',
  '14': 'Tranche C Prévoyance',
  '15': 'Tranche D Prévoyance',
  '16': 'Tranche D1 Prévoyance',
  '17': 'Base spécifique Prévoyance',
  '18': 'Base forfaitaire Prévoyance',
  '19': 'Base fictive Prévoyance reconstituée',
  '20': 'Montant forfaitaire Prévoyance',
  '21': 'Montant Prévoyance libre ou exceptionnel',
  '22': 'Montant des indemnités journalières CRPCEN',
  '23': 'Sans composant de base assujettie en paie',
  '24': 'Tranche 2 Unifiée Prévoyance',
  '25': 'Potentiel nouveau type de composant de base assujettie A',
  '26': 'Potentiel nouveau type de composant de base assujettie B',
  '27': 'Potentiel nouveau type de composant de base assujettie C',
  '28': 'Potentiel nouveau type de composant de base assujettie D',
  '29': 'Potentiel nouveau type de composant de base assujettie E',
  '90': 'Retenue sur salaire',
  '91': 'Base de taxe sur les salaires au taux normal',
};

export const DSN_NATURE_CONTRAT = {
  '01': 'Contrat de travail à durée indéterminée de droit privé',
  '02': 'Contrat de travail à durée déterminée de droit privé',
  '03': 'Contrat  de mission (contrat de travail temporaire)',
  '07': 'Contrat à durée indéterminée intermittent',
  '08': 'Contrat à durée indéterminée intérimaire',
  '09': 'Contrat de travail à durée indéterminée de droit public',
  '10': 'Contrat de travail à durée déterminée de droit public',
  '20': '[FP] Détachement d’un agent d’une Fonction Publique donnant lieu à pension (ECP)',
  '21': '[FP] Détachement d’un agent d’une Fonction Publique ne donnant pas lieu à pension (ENCP)',
  '29': 'Convention de stage (hors formation professionnelle)',
  '32': 'Contrat d’appui au projet d’entreprise',
  '50': 'Nomination dans la fonction publique (par arrêté, par décision,…)',
  '51': 'Contrat de mission d’un collaborateur occasionnel du service public (COSP) ou assimilé',
  '52': '[FP] Cumul d’activité à titre accessoire',
  '53': 'Contrat d\'emploi pénitentiaire',
  '54': 'Contrat d\'emploi pénitentiaire en apprentissage',
  '60': 'Contrat d\'engagement éducatif',
  '70': 'Contrat de soutien et d\'aide par le travail',
  '80': 'Mandat social',
  '81': 'Mandat d\'élu',
  '82': 'Contrat de travail à durée indéterminée de Chantier ou d\'opération',
  '89': 'Volontariat de service civique',
  '90': 'Autre nature de contrat, convention, mandat (hors mandat social)',
  '91': 'Contrat d\'engagement maritime à durée indéterminée',
  '92': 'Contrat d\'engagement maritime à durée déterminée',
  '93': 'Ligne de service',
};

export const DSN_STATUT_CONV = {
  '01': 'agriculteur salarié de son exploitation',
  '02': 'artisan ou commerçant salarié de son entreprise',
  '03': 'cadre dirigeant (votant au collège employeur des élections prud\'homales)',
  '04': 'autres cadres au sens de la convention collective (ou du statut pour les régimes spéciaux)',
  '05': 'profession intermédiaire (technicien, contremaître, agent de maîtrise, clergé)',
  '06': 'employé administratif d\'entreprise, de commerce, agent de service',
  '07': 'ouvriers qualifiés et non qualifiés y compris ouvriers agricoles',
  '08': 'agent de la fonction publique d\'Etat',
  '09': 'agent de la fonction publique hospitalière',
  '10': 'agent de la fonction publique territoriale',
};

export const DSN_STATUT_RC = {
  '01': 'cadre (article 4 et 4bis)',
  '02': 'extension cadre pour retraite complémentaire',
  '04': 'non cadre',
  '98': 'retraite complémentaire ne définissant pas de statut cadre ou non-cadre',
  '99': 'pas de retraite complémentaire',
};

export const DSN_UNITE_QUOTITE = {
  '10': 'heure',
  '12': 'journée',
  '20': 'forfait jour',
  '21': 'forfait heure',
  '31': 'à la pige',
  '32': 'à la vacation',
  '33': 'à la tâche',
  '34': 'au SMIC',
  '35': 'à la part',
  '99': 'salarié non concerné',
};

export const DSN_MODALITE_TT = {
  '10': 'Temps plein',
  '20': 'Temps partiel',
  '30': 'Temps alterné - personnel navigant de l\'aéronautique civile',
  '40': 'CPA 2004',
  '41': 'Temps partiel de droit',
  '42': 'Temps partiel de droit pour enfant',
  '99': 'Salarié non concerné',
};

export const DSN_REGIME_MALADIE = {
  '134': 'régime spécial de la SNCF',
  '135': 'régime spécial de la RATP',
  '136': 'établissement des invalides de la marine (ENIM)',
  '137': 'mineurs ou assimilés (CANSSM)',
  '138': 'militaires de carrière (CNMSS)',
  '140': 'clercs et employés de notaires (CRPCEN)',
  '141': 'chambre de commerce et d\'industrie de Paris',
  '144': 'Assemblée Nationale',
  '145': 'Sénat',
  '146': 'port autonome de Bordeaux',
  '147': 'régime spécial des industries électriques et gazières (CAMIEG)',
  '149': 'régimes des cultes (CAVIMAC)',
  '200': 'régime général (CNAM)',
  '300': 'régime agricole (MSA)',
  '400': 'régime spécial Banque de France',
  '900': 'autre régime (réservé Polynésie Française, Nouvelle Calédonie)',
  '909': 'travailleur étranger non assujetti à un régime de base risque maladie en France',
  '999': 'sans régime obligatoire',
};

export const DSN_REGIME_VIEILLESSE = {
  '120': 'retraite des agents des collectivités locales (CNRACL)',
  '121': 'pensions des ouvriers des établissements industriels de l\'Etat (FSPOEIE)',
  '122': 'pensions civiles et militaires de retraite de l\'Etat (SRE)',
  '134': 'régime spécial de la branche ferroviaire',
  '135': 'régime spécial de la RATP',
  '136': 'établissement des invalides de la marine (ENIM)',
  '137': 'mineurs ou assimilés (fonds Caisse des Dépôts)',
  '139': 'Banque de France',
  '140': 'clercs et employés de notaires (CRPCEN)',
  '141': 'chambre de commerce et d\'industrie de Paris',
  '144': 'Assemblée Nationale',
  '145': 'Sénat',
  '147': 'régime spécial des industries électriques et gazières (CNIEG)',
  '149': 'régime des cultes (CAVIMAC)',
  '157': 'régime de retraite des avocats (CNBF)',
  '158': 'SEITA',
  '159': 'Comédie Française',
  '160': 'Opéra de Paris',
  '200': 'régime général (CNAV)',
  '300': 'régime agricole (MSA)',
  '900': 'autre régime (réservé Polynésie Française, Nouvelle Calédonie)',
  '904': 'principauté de Monaco',
  '905': 'Régime général pour la déclaration des individus de Mayotte affiliés à la CSSM',
  '909': 'travailleur non assujetti à un régime de base risque vieillesse en France',
  '994': 'absence d\'affiliation pour le personnel médical hospitalier universitaire',
  '995': 'absence d’affiliation pour un fonctionnaire en cumul d’activité à titre accessoire',
  '996': 'absence d’affiliation pour un expatrié',
  '997': 'absence d’affiliation pour un stagiaire hors formation professionnelle',
  '998': 'absence d’affiliation pour un élu',
};

export const DSN_REGIME_RC = {
  'RETA': 'Retraite complémentaire ARRCO',
  'RETC': 'Retraite complémentaire ARRCO et AGIRC',
  'RUAA': 'Régime unifié AGIRC-ARRCO',
  'CAVEC': 'Caisse d\'assurance vieillesse des experts-comptables et des commissaires aux comptes',
  'CNBF': 'Caisse nationale des Barreaux Français',
  'CRPCEN': 'Clercs et employés de notaire',
  'CRPNPAC': 'Caisse de Retraite du Personnel Navigant Professionnel de l\'Aéronautique Civile',
  'IRCANTEC': 'Institution de retraite complémentaire des agents non titulaires de l’État et des collectivités publiques',
  '90000': 'pas de régime complémentaire',
};

export const DSN_TYPE_PAS = {
  '01': 'Taux transmis par la DGFIP',
  '13': 'Barème mensuel métropole',
  '17': 'Barème mathématique sur base mensuelle métropole',
  '23': 'Barème mensuel Guadeloupe, Réunion et Martinique',
  '27': 'Barème mathématique sur base mensuelle Guadeloupe, Réunion et Martinique',
  '33': 'Barème mensuel Guyane et Mayotte',
  '37': 'Barème mathématique sur base mensuelle Guyane et Mayotte',
  '99': 'Indu relatif à un exercice antérieur – pas de taux de PAS',
};

export const DSN_NATURE_DECL = {
  '01': 'DSN Mensuelle',
  '04': 'Signalement Arrêt de travail',
  '05': 'Signalement Reprise suite à arrêt de travail',
  '07': 'Signalement Fin du contrat de travail Unique',
  '08': 'Signalement Amorçage des données variables',
  '09': 'DSN de substitution',
  '10': 'Signalement Déclaration préalable à l\'embauche',
};

export const DSN_TYPE_DECL = {
  '01': 'déclaration normale',
  '02': 'déclaration normale sans individu',
  '03': 'déclaration annule et remplace intégral',
  '04': 'déclaration annule',
  '05': 'annule et remplace sans individu',
};

export const DSN_CHAMP_DECL = {
  '01': 'déclaration totale',
  '02': 'déclaration partielle régime agricole',
  '03': 'déclaration partielle régime général',
};

export const DSN_DEVISE = {
  '01': 'euro',
  '02': 'franc Pacifique',
};

export const DSN_ESSAI_REEL = {
  '01': 'envoi fichier test',
  '02': 'envoi fichier réel',
};

export const DSN_POINT_DEPOT = {
  '01': 'Net-entreprises',
  '02': 'MSA',
};

export const DSN_TYPE_ENVOI = {
  '01': 'envoi normal',
  '02': 'envoi néant',
};

export const DSN_CIVILITE = {
  '01': 'monsieur',
  '02': 'madame',
};

export const DSN_NATURE_JURIDIQUE = {
  '01': 'Privée',
  '02': 'Publique',
  '03': 'Etablissement privé à capitaux majoritaires publics',
};

export const DSN_QUALIFIANT_ASSIETTE = {
  '920': 'Autre assiette',
  '921': 'Assiette plafonnée',
};

export const DSN_SEXE = {
  '01': 'masculin',
  '02': 'féminin',
};

export const DSN_AUTRE_REVENU = {
  '01': 'Somme versée par un tiers',
  '02': 'Avantage en nature : repas',
  '03': 'Avantage en nature : logement',
  '04': 'Avantage en nature : véhicule',
  '05': 'Avantage en nature : NTIC',
  '06': 'Avantage en nature : autres',
  '07': 'Frais professionnels remboursés au forfait',
  '08': 'Frais professionnels pris en charge par l\'employeur',
  '09': 'Frais professionnels remboursés au réel',
  '10': 'Déduction forfaitaire spécifique',
  '11': 'Participation y compris supplément',
  '12': 'Intéressement y compris supplément',
  '14': 'Abondement au plan d\'épargne entreprise (PEE)',
  '15': 'Abondement au plan d\'épargne interentreprises (PEI)',
  '16': 'Abondement à un plan d\'épargne pour la retraite collectif (PERCO, PERECO)',
  '17': 'Participation patronale au financement des titres-restaurant',
  '18': 'Participation patronale aux frais de transports publics',
  '19': 'Participation patronale aux frais de transports personnels',
  '25': 'Droit d\'auteur',
  '26': 'Droit de doublage',
  '27': 'Droit de rediffusion',
  '31': 'Avantages de préretraite versés par l’employeur',
  '33': 'Sommes provenant d\'un CET et réaffectées à un plan d\'épargne retraite (PERCO, PERECO, PEREO) ou à un régime de retraite supplémentaire',
  '34': '[FP] Prestations familiales',
  '35': 'Parts des frais professionnels réintégrées dans l\'assiette de contribution du fait de l\'application d\'une DFS',
  '36': 'Sommes provenant d\'un CET et converties en points retraite RAFP',
  '37': 'Participation et intéressement versés immédiatement et directement par l\'employeur',
  '50': 'Potentiel nouveau type d\'autre élément de revenu brut A',
  '51': 'Potentiel nouveau type d\'autre élément de revenu brut B',
  '90': 'Participation au financement des services à la personne',
  '91': 'Montant de la participation de l\'employeur aux chèques vacances',
  '92': 'Cotisation patronale frais de santé',
  '93': 'Cotisation prévoyance et retraite supplémentaire',
  '94': 'Potentiel nouveau type d\'autre élément de revenu brut C',
  '95': 'Potentiel nouveau type d\'autre élément de revenu brut D',
  '96': 'Potentiel nouveau type d\'autre élément de revenu brut E',
};

export const DSN_STATUT_EMPLOI = {
  '01': '[FP] Fonctionnaire',
  '02': '[FP] Contractuel de la Fonction publique',
  '03': 'Statutaire',
  '04': 'Non statutaire',
  '06': 'Personnel médical hospitalier',
  '07': 'Médecin sans statut hospitalier',
  '08': '[FP] Fonctionnaire stagiaire',
  '09': '[FP] Ouvrier d\'Etat',
  '10': '[FP] Militaire',
  '11': '[FP] Parcours d\'accès aux carrières (Pacte)',
  '12': '[FP] Militaire de réserve',
  '99': 'Non concerné',
};


// ── Codes types de personnel (CTP) — table Urssaf ────────────────────────────
// La maille AGRÉGÉE (bloc S21.G00.23) ne parle pas le langage des cotisations
// individuelles : elle parle CTP. Un CTP porte un couple de taux (plafonné /
// déplafonné) et regroupe plusieurs risques. Extrait de la table en vigueur,
// source open.urssaf.fr — les taux servent ici de POINT DE COMPARAISON avec ce
// que le simulateur calcule, pas de valeur déclarée.
export const DSN_CTP = {
  '100': { lib: 'RG CAS GENERAL',                     plaf: 15.45, deplaf: 13.26 },
  '260': { lib: 'CSG CRDS REGIME GENERAL',            plaf: null,  deplaf: 9.70  },
  '772': { lib: 'CONTRIBUTIONS ASSURANCE CHOMAGE U2', plaf: null,  deplaf: 4.00  },
  '668': { lib: 'REDUCTION GENERALE ETENDUE U2',      plaf: 100.0, deplaf: null  },
  '671': { lib: 'REDUCTION GENERALE SANS CHOMAGE',    plaf: 100.0, deplaf: null  },
  '381': { lib: 'MAJO ALSACE MOSELLE - SECTEUR PRIVE', plaf: null, deplaf: 1.30  },
  '003': { lib: 'REDUCTION SALARIALE HEURES SUP',     plaf: 100.0, deplaf: null  },
  '004': { lib: 'DEDUCTION PATRONALE HEURES SUP',     plaf: 100.0, deplaf: null  },
  '332': { lib: 'FNAL PLAFONNE',                      plaf: 0.10,  deplaf: null  },
  '236': { lib: 'FNAL TOTALITE',                      plaf: null,  deplaf: 0.50  },
  '863': { lib: 'RG MANDATAIRES SOCIAUX',             plaf: 15.45, deplaf: 21.06 },
};

// ── Rôle pédagogique de chaque bloc émis ─────────────────────────────────────
// Une phrase par bloc : à quelle question ce bloc répond dans le fichier.
export const DSN_BLOC_ROLE = {
  'S10.G00.00': "Enveloppe technique de l'envoi : avec quel logiciel, sous quelle version de norme, en test ou en réel, et sur quel point de dépôt. Aucune donnée de paie.",
  'S10.G00.01': "Qui dépose le fichier. L'émetteur n'est pas forcément l'employeur : c'est souvent l'expert-comptable ou le concentrateur.",
  'S10.G00.02': "Qui appeler chez l'émetteur quand le dépôt casse. Point d'entrée TECHNIQUE, distinct du contact métier (S20.G00.07).",
  'S20.G00.05': "Carte d'identité de la déclaration : sa nature (mensuelle ? signalement ?), son type (normale ? annule et remplace ?), et le mois de paie sur lequel elle porte.",
  'S21.G00.06': "Quelle entreprise est déclarée : SIREN, NIC du siège, code APEN, convention collective.",
  'S21.G00.11': "L'établissement de rattachement du salarié — la maille SIRET. C'est sous lui que viennent se ranger les individus, les bordereaux et les cotisations.",
  'S21.G00.20': "Le paiement : combien l'établissement verse, à quel organisme, au titre de quelle période. À ne pas confondre avec ce qui est DÛ (bloc 22).",
  'S21.G00.22': "La dette agrégée envers un organisme, sur une période. Vue consolidée, sans détail par salarié.",
  'S21.G00.23': "Le détail du bordereau, ligne par Code Type de Personnel (CTP). Bloc réservé aux Urssaf : c'est la maille AGRÉGÉE, qui doit se réconcilier au centime avec la maille nominative.",
  'S21.G00.30': "L'individu : le bloc pivot de l'identification. Tout le reste (contrat, rémunération, cotisations) se rattache à lui.",
  'S21.G00.40': "Le contrat — le bloc le plus riche de la norme (79 rubriques). Il porte l'emploi, la quotité, les régimes de protection sociale, et sert de clé d'accrochage aux rémunérations.",
  'S21.G00.71': "À quel régime de retraite complémentaire le contrat est affilié.",
  'S21.G00.50': "Ce qui a été VERSÉ à l'individu, en net, et tout le prélèvement à la source.",
  'S21.G00.51': "La décomposition du brut : rémunération brute, salaire de base, heures supplémentaires… Une ligne par nature.",
  'S21.G00.78': "Une base assujettie : la somme des montants soumis de façon homogène à une ou plusieurs cotisations. C'est le pivot entre le versement et les cotisations.",
  'S21.G00.79': "Le détail d'une base quand le montant global ne suffit pas — typiquement le SMIC retenu pour la réduction générale.",
  'S21.G00.81': "La cotisation individuelle : la maille NOMINATIVE des charges. Toujours rattachée à une base assujettie du bloc 78.",
  'S90.G00.90': "Fermeture du fichier : deux nombres de contrôle qui permettent au destinataire de vérifier qu'il a tout reçu.",
};

// ═════════════════════════════════════════════════════════════════════════════
// FABRIQUE DE L'EXTRAIT
// ═════════════════════════════════════════════════════════════════════════════

// Identifiants fictifs. Le SIREN 000 000 000 satisfait la clé de Luhn (somme
// nulle) mais n'est JAMAIS attribué par l'Insee : format valide, collision
// impossible avec une entreprise réelle. Même raisonnement pour le NIC et pour
// les SIRET d'organismes.
const FAUX_SIREN = '000000000';
const FAUX_NIC   = '00000';
const FAUX_SIRET_OPS = '00000000000000';

const _2 = n => String(n).padStart(2, '0');

/** ISO `YYYY-MM-DD` → `JJMMAAAA`, le format de date de toute la norme. */
function dtDsn(iso) {
  const [y, m, d] = String(iso).split('-');
  return `${d}${m}${y}`;
}
/** Montant DSN : point décimal, 2 décimales, signe « − » conservé. */
function mtDsn(v) {
  const n = Number(v);
  return (Number.isFinite(n) ? n : 0).toFixed(2);
}
/** Taux DSN : exprimé en POURCENTAGE, 2 décimales (0,0920 → « 9.20 »). */
function txDsn(v) {
  const n = Number(v);
  return ((Number.isFinite(n) ? n : 0) * 100).toFixed(2);
}
/** Dernier jour du mois d'une date ISO. */
function finDeMois(iso) {
  const [y, m] = String(iso).split('-').map(Number);
  return `${y}-${_2(m)}-${_2(new Date(y, m, 0).getDate())}`;
}
/** Le 5 du mois suivant : date de dépôt de la DSN pour les entreprises ≥ 50. */
function cinqDuMoisSuivant(iso) {
  const [y, m] = String(iso).split('-').map(Number);
  const ny = m === 12 ? y + 1 : y;
  const nm = m === 12 ? 1 : m + 1;
  return `${ny}-${_2(nm)}-05`;
}

/**
 * NTT — Numéro Technique Temporaire (S21.G00.30.020).
 * Composition normative : code sexe (1 ou 2) + SIREN de l'entreprise +
 * identifiant unique et pérenne de l'individu dans l'entreprise. 11 à 40
 * caractères, sans espace.
 */
function construireNtt(sexe, matricule) {
  return (sexe + FAUX_SIREN + matricule).replace(/[^A-Za-z0-9]/g, '').slice(0, 40);
}

/**
 * Clé du NIR : 97 − (NIR sur 13 chiffres modulo 97). Non déclarée en DSN
 * (S21.G00.30.001 ne porte que les 13 chiffres) mais exposée dans le glossaire :
 * c'est l'un des contrôles que tout gestionnaire de paie doit savoir refaire.
 */
export function cleNir(nir13) {
  const n = BigInt(String(nir13).replace(/\D/g, ''));
  return String(97n - (n % 97n)).padStart(2, '0');
}

/**
 * Récupère le SMIC mensuel retenu pour la réduction générale.
 *
 * Heuristique assumée : le back Rust n'expose pas le SMIC dans le Bulletin, mais
 * l'explication de la ligne REDUCTION_FILLON le cite en toutes lettres. Si la
 * phrase change — ou si l'utilisateur a basculé l'application dans une autre
 * langue — la lecture échoue et le composant de base assujettie S21.G00.79 est
 * simplement omis, avec une lacune déclarée. Jamais de valeur inventée.
 */
function smicDepuisFillon(cotFillon) {
  if (!cotFillon || typeof cotFillon.explication !== 'string') return null;
  const m = cotFillon.explication.match(/retenu\s*:\s*([\d   ]*\d(?:[.,]\d+)?)\s*€/);
  if (!m) return null;
  const v = parseFloat(m[1].replace(/[  \s]/g, '').replace(',', '.'));
  return Number.isFinite(v) && v > 0 ? v : null;
}

/**
 * Fabrique l'extrait de DSN mensuelle correspondant à un bulletin France.
 *
 * @param {object} b   Bulletin renvoyé par le back (`calculer_bulletin`).
 * @param {object} opt { datePaie ISO, pasTotal, pasTaux, versionLogiciel }
 * @returns {{blocs:Array, lacunes:Array<string>, raw:string, nbRub:number}}
 */
export function buildDsn(b, opt = {}) {
  const cots = b.cotisations || [];
  const cot  = code => cots.find(c => c.code === code) || null;
  const num  = v => { const n = parseFloat(v); return Number.isFinite(n) ? n : 0; };
  // Montant TOTAL d'une ligne (salarial + patronal) : c'est ce qu'attend la DSN
  // pour une cotisation individuelle. Le bulletin, lui, sépare les deux parts.
  const tot     = c => c ? num(c.montant_sal) + num(c.montant_pat) : 0;
  const totTaux = c => c ? num(c.taux_sal) + num(c.taux_pat) : 0;

  const datePaie = opt.datePaie || new Date().toISOString().slice(0, 10);
  const [an, moisNum] = datePaie.split('-');
  const debutMois = `${an}-${moisNum}-01`;
  const finMois   = finDeMois(datePaie);
  const cadre     = b.salarie?.statut === 'cadre';
  const etp       = Number(b.salarie?.etp) > 0 ? Number(b.salarie.etp) : 100;
  const anciennete = Number.isFinite(Number(b.salarie?.anciennete)) ? Number(b.salarie.anciennete) : 1;
  const matricule = 'XENNA0001';
  // Le premier caractère du NTT est le code sexe. Le simulateur ne collecte pas
  // cette donnée : la valeur est arbitraire et sans rapport avec la saisie.
  const SEXE_PAR_DEFAUT = '1';

  const brut       = num(b.brut);
  const basePlaf   = cot('SS_VIEILLESSE_PLAF')?.base ? num(cot('SS_VIEILLESSE_PLAF').base)
                   : cot('AGIRC_ARRCO_T1')?.base ? num(cot('AGIRC_ARRCO_T1').base) : brut;
  const baseCsg    = cot('CSG_DEDUCTIBLE') ? num(cot('CSG_DEDUCTIBLE').base) : 0;
  const baseCho    = cot('CHOMAGE') ? num(cot('CHOMAGE').base) : 0;
  const netImpo    = num(b.net_imposable);
  const csgNd      = cot('CSG_NON_DEDUCTIBLE') ? num(cot('CSG_NON_DEDUCTIBLE').montant_sal) : 0;
  const crds       = cot('CRDS') ? num(cot('CRDS').montant_sal) : 0;
  const pasTotal   = num(opt.pasTotal);
  const pasTaux    = num(opt.pasTaux);
  // Montant net versé = RNF − CSG non déductible − CRDS. Le PAS ne s'en déduit
  // PAS (S21.G00.50.004) : c'est un piège classique de la rubrique.
  const netVerse   = +(netImpo - csgNd - crds).toFixed(2);

  const blocs = [];
  const lacunes = [];
  const listesUtilisees = new Set();

  /** Ajoute un bloc ; `rubs` = [[suffixeRubrique, valeur, signification, listeOuNull]] */
  const bloc = (code, suffixe, rubs) => {
    blocs.push({
      code, suffixe: suffixe || '',
      lib: DSN_BLOCS[code] || code,
      role: DSN_BLOC_ROLE[code] || '',
      rubs: rubs.filter(Boolean).map(([n, v, sens, liste]) => {
        const full = `${code}.${n}`;
        if (liste) listesUtilisees.add(full);
        return { code: full, lib: DSN_RUBRIQUES[full] || '(rubrique hors référentiel chargé)', val: String(v), sens };
      }),
    });
  };

  // ── S10 — Entête ───────────────────────────────────────────────────────────
  // La version de norme suit l'année de la paie : P25V01 en 2025, P26V01 en 2026.
  const normeVersion = `P${String(an).slice(2)}V01`;
  bloc('S10.G00.00', '', [
    ['001', 'XENNA PAIE', "Nom du logiciel qui a produit le fichier."],
    ['002', 'XENNA', "Nom de l'éditeur."],
    ['003', opt.versionLogiciel || '0.0.0', "Version du logiciel — sert au support à reproduire un bug sur la bonne version."],
    ['005', '01', "01 = envoi de TEST. Ce simulateur ne produit jamais autre chose : un extrait pédagogique n'a rien à faire dans un flux réel.", true],
    ['006', normeVersion, `Version de la norme NEODeS applicable à l'année de paie (${an}). La norme change tous les ans.`],
    ['007', '01', "Point de dépôt : net-entreprises (régime général) plutôt que MSA (régime agricole).", true],
    ['008', '01', "Envoi normal, par opposition à l'envoi « néant » d'un employeur sans salarié à déclarer.", true],
  ]);

  bloc('S10.G00.01', '', [
    ['001', FAUX_SIREN, "SIREN de l'émetteur. 000 000 000 satisfait la clé de Luhn mais n'est jamais attribué par l'Insee : format valide, entreprise inexistante."],
    ['002', FAUX_NIC, "NIC de l'émetteur — les 5 chiffres qui, accolés au SIREN, forment le SIRET."],
    ['003', 'XENNA PAIE (SIMULATION)', "Raison sociale de l'émetteur."],
    ['004', '1 RUE FICTIVE', "Adresse : numéro, extension, nature et libellé de la voie."],
    ['005', '00000', "Code postal."],
    ['006', 'LOCALITE FICTIVE', "Localité."],
  ]);

  bloc('S10.G00.02', '', [
    ['001', '01', "Code civilité du contact technique.", true],
    ['002', 'CONTACT FICTIF', "Nom et prénom de la personne à joindre chez l'émetteur."],
    ['004', 'contact@example.invalid', "Adresse e-mail. Le TLD .invalid est réservé par la RFC 2606 : il ne peut jamais exister."],
    ['005', '0000000000', "Adresse téléphonique."],
  ]);

  // ── S20 — Déclaration ──────────────────────────────────────────────────────
  bloc('S20.G00.05', '', [
    ['001', '01', "Nature : DSN mensuelle. C'est cette valeur qui commande le modèle de message applicable au reste du fichier.", true],
    ['002', '01', "Déclaration normale (par opposition à « annule et remplace »).", true],
    ['003', '11', "Numéro de fraction, lu « n sur d » : 11 = fraction 1 sur 1, déclaration non fractionnée."],
    ['004', '00001', "Numéro d'ordre dans le mois. Remis à zéro chaque 1er du mois pour une DSN mensuelle."],
    ['005', dtDsn(debutMois), "Mois principal déclaré, toujours au premier jour du mois civil."],
    ['007', dtDsn(cinqDuMoisSuivant(datePaie)), "Date de constitution du fichier — ici le 5 du mois suivant, échéance des entreprises d'au moins 50 salariés (le 15 pour les autres)."],
    ['008', '01', "Déclaration totale : l'entreprise ne relève pas à la fois du régime général et du régime agricole.", true],
    ['010', '01', "Devise de la déclaration : euro. Elle s'applique à TOUS les montants du fichier.", true],
  ]);

  // ── S21 — Données paie et RH ───────────────────────────────────────────────
  bloc('S21.G00.06', '', [
    ['001', FAUX_SIREN, "SIREN de l'entreprise déclarée."],
    ['002', FAUX_NIC, "NIC du siège."],
    ['003', '0000Z', "Code APEN — activité principale de l'entreprise, attribué par l'Insee."],
    ['004', '1 RUE FICTIVE', "Voie."],
    ['005', '00000', "Code postal."],
    ['006', 'LOCALITE FICTIVE', "Localité."],
    ['015', '0016', "Code IDCC de la convention collective. 0016 = transport routier, la convention par défaut du simulateur pour le maintien de salaire. L'IDCC réapparaît au niveau établissement (S21.G00.11.022) et au niveau contrat (S21.G00.40.017) : trois mailles, trois rubriques."],
  ]);

  bloc('S21.G00.11', '', [
    ['001', FAUX_NIC, "NIC de l'établissement d'affectation du salarié."],
    ['002', '0000Z', "Code APET — activité de l'ÉTABLISSEMENT, qui peut différer de l'APEN de l'entreprise."],
    ['003', '1 RUE FICTIVE', "Voie."],
    ['004', '00000', "Code postal."],
    ['005', 'LOCALITE FICTIVE', "Localité."],
    ['017', '01', "Nature juridique de l'employeur : privée.", true],
    ['022', '0016', "Convention collective principale de l'établissement."],
  ]);

  // ── Maille AGRÉGÉE : bordereau Urssaf + lignes CTP ─────────────────────────
  // Les cotisations recouvrées par l'Urssaf, hors retraite complémentaire
  // (Agirc-Arrco) qui a son propre circuit.
  const cMaladie   = cot('SS_MALADIE');
  const cVieilPlaf = cot('SS_VIEILLESSE_PLAF');
  const cVieilDep  = cot('SS_VIEILLESSE_DEPLAF');
  const cFamille   = cot('FAMILLE');
  const cAtmp      = cot('AT_MP');
  const cChomage   = cot('CHOMAGE');
  const cCsgD      = cot('CSG_DEDUCTIBLE');
  const cCsgNd     = cot('CSG_NON_DEDUCTIBLE');
  const cCrds      = cot('CRDS');
  const cAm        = cot('ALSACE_MOSELLE_MALADIE');
  const cFillon    = cot('REDUCTION_FILLON');
  const cRedSalHs  = cot('REDUC_SAL_HS');
  const cDfpHs     = cot('DFP_HS');
  const cotsAA     = cots.filter(c => c.categorie === 'Retraite complémentaire');

  const mtCsg      = tot(cCsgD) + tot(cCsgNd);
  const txCsg      = totTaux(cCsgD) + totTaux(cCsgNd);
  const mtAA       = cotsAA.reduce((s, c) => s + tot(c), 0);
  // Une réduction se déclare en NÉGATIF : sans le signe, l'organisme la lit
  // comme la régularisation d'un « trop allégé » précédemment déclaré.
  const mtFillon   = cFillon ? -Math.abs(tot(cFillon)) : 0;
  const mtRedSalHs = cRedSalHs ? -Math.abs(tot(cRedSalHs)) : 0;
  const mtDfpHs    = cDfpHs ? -Math.abs(tot(cDfpHs)) : 0;

  // Le « cas général » agrégé : tout ce qui tombe sur l'assiette déplafonnée
  // hors AT/MP (dont le taux est notifié établissement par établissement et
  // reste à 0 dans la table CTP 100).
  const mtCtp100Dep = tot(cMaladie) + tot(cVieilDep) + tot(cFamille);
  const txCtp100Dep = totTaux(cMaladie) + totTaux(cVieilDep) + totTaux(cFamille);
  const totalUrssaf = +(mtCtp100Dep + tot(cVieilPlaf) + tot(cAtmp) + mtCsg + tot(cCrds)
                      + tot(cChomage) + tot(cAm) + mtFillon + mtRedSalHs + mtDfpHs).toFixed(2);

  bloc('S21.G00.20', '', [
    ['001', FAUX_SIRET_OPS, "Identifiant de l'organisme destinataire : pour l'Urssaf, son SIRET."],
    ['005', mtDsn(totalUrssaf), "Montant réellement versé à l'organisme."],
    ['006', dtDsn(debutMois), "Début de la période de rattachement du paiement."],
    ['007', dtDsn(finMois), "Fin de la période de rattachement."],
  ]);

  bloc('S21.G00.22', '', [
    ['001', FAUX_SIRET_OPS, "Organisme auprès duquel l'établissement est redevable."],
    ['003', dtDsn(debutMois), "Début de période. Une déclaration Urssaf courante ne peut porter qu'un seul mois civil."],
    ['004', dtDsn(finMois), "Fin de période."],
    ['005', mtDsn(totalUrssaf), "Montant total de cotisations dues — la somme des lignes CTP qui suivent."],
  ]);

  // Une ligne agrégée par CTP. `off` = taux officiel de la table Urssaf, affiché
  // en regard pour montrer l'écart avec ce que le simulateur calcule.
  const lignesCtp = [
    { ctp: '100', q: '921', assiette: basePlaf, montant: tot(cVieilPlaf), taux: totTaux(cVieilPlaf),
      sens: "Cas général, part plafonnée : la vieillesse plafonnée." },
    { ctp: '100', q: '920', assiette: brut, montant: mtCtp100Dep, taux: txCtp100Dep,
      sens: "Cas général, part déplafonnée : maladie, vieillesse déplafonnée et allocations familiales." },
    cAtmp && { ctp: '100', q: '920', assiette: num(cAtmp.base), montant: tot(cAtmp), taux: totTaux(cAtmp),
      sens: "Accidents du travail. Le taux n'est pas dans la table CTP : il est notifié à l'établissement par la Carsat, en fonction de sa sinistralité." },
    mtCsg + tot(cCrds) ? { ctp: '260', q: '920', assiette: baseCsg, montant: mtCsg + tot(cCrds), taux: txCsg + totTaux(cCrds),
      sens: "CSG et CRDS regroupées sur l'assiette abattue de 1,75 %." } : null,
    cChomage && tot(cChomage) ? { ctp: '772', q: '920', assiette: baseCho, montant: tot(cChomage), taux: totTaux(cChomage),
      sens: "Contribution d'assurance chômage, sur l'assiette limitée à 4 plafonds." } : null,
    cAm && tot(cAm) ? { ctp: '381', q: '920', assiette: num(cAm.base), montant: tot(cAm), taux: totTaux(cAm),
      sens: "Cotisation salariale du régime local d'Alsace-Moselle." } : null,
    mtFillon ? { ctp: '668', q: '920', assiette: brut, montant: mtFillon, taux: null,
      sens: "Réduction générale. CTP 668 « étendue » parce qu'elle couvre aussi le chômage ; le CTP 671 sert au cas où la contribution chômage n'est pas due." } : null,
    mtRedSalHs ? { ctp: '003', q: '920', assiette: brut, montant: mtRedSalHs, taux: null,
      sens: "Réduction salariale sur les heures supplémentaires." } : null,
    mtDfpHs ? { ctp: '004', q: '920', assiette: brut, montant: mtDfpHs, taux: null,
      sens: "Déduction forfaitaire patronale sur les heures supplémentaires." } : null,
  ].filter(Boolean);

  lignesCtp.forEach(l => {
    const ref = DSN_CTP[l.ctp];
    const off = ref ? (l.q === '921' ? ref.plaf : ref.deplaf) : null;
    const comparaison = (off != null && l.taux != null)
      ? ` Taux officiel de la table Urssaf pour ce CTP : ${off.toFixed(2)} % — l'écart avec le taux ci-dessus mesure exactement ce que le simulateur ne modélise pas (CSA, FNAL, contributions formation…).`
      : '';
    bloc('S21.G00.23', `— CTP ${l.ctp} ${ref ? ref.lib : ''}`, [
      ['001', l.ctp, `Code Type de Personnel. ${l.sens}${comparaison}`],
      ['002', l.q, `Qualifiant d'assiette : ${DSN_QUALIFIANT_ASSIETTE[l.q] || l.q}.`, true],
      l.taux != null ? ['003', txDsn(l.taux), "Taux de cotisation appliqué à l'assiette."] : null,
      ['004', mtDsn(l.assiette), "Montant d'assiette."],
      ['005', mtDsn(l.montant), "Montant de cotisation. Négatif pour une réduction."],
    ]);
  });

  // ── Individu ───────────────────────────────────────────────────────────────
  const ntt = construireNtt(SEXE_PAR_DEFAUT, matricule);
  bloc('S21.G00.30', '', [
    ['002', (b.salarie?.nom || '').toUpperCase(), "Nom de famille (nom de naissance), en majuscules."],
    ['004', (b.salarie?.prenom || '').toUpperCase(), "Prénoms."],
    ['005', '01', "Sexe. Rubrique normalement facultative — elle devient obligatoire ici PARCE QUE l'individu est déclaré sous NTT et non sous NIR : sans NIR, plus rien ne porte cette information. Le simulateur ne collecte pas le sexe : la valeur est arbitraire.", true],
    ['006', '01011985', "Date de naissance au format JJMMAAAA. Valeur fictive : le simulateur ne la collecte pas."],
    ['007', 'LIEU FICTIF', "Lieu de naissance."],
    ['019', matricule, "Matricule dans l'entreprise — l'identifiant interne de paie."],
    ['020', ntt, "Numéro Technique Temporaire. Composition normative : code sexe + SIREN + identifiant pérenne de l'individu. Le NTT est la SEULE réponse correcte quand ni le NIR ni le NIA ne sont connus ; il n'ouvre aucun droit et doit être reporté sur la première DSN où le NIR est attribué, pour relier les deux identités."],
  ]);

  // ── Contrat ────────────────────────────────────────────────────────────────
  const dateDebutContrat = `${Number(an) - Math.max(0, anciennete)}-${moisNum}-01`;
  const quotiteContrat = (151.67 * etp / 100).toFixed(2);
  bloc('S21.G00.40', '', [
    ['001', dtDsn(dateDebutContrat), `Date de début du contrat, reconstituée depuis l'ancienneté saisie (${anciennete} an${anciennete > 1 ? 's' : ''}).`],
    ['002', cadre ? '04' : '06', `Statut conventionnel. ${cadre ? "04 = autres cadres au sens de la convention collective." : "06 = employé administratif. Attention : le simulateur ne connaît que « cadre » et « non cadre », alors que la norme distingue trois statuts non-cadres (05 profession intermédiaire, 06 employé, 07 ouvrier)."}`, true],
    ['003', cadre ? '01' : '04', `Statut catégoriel au regard de la RETRAITE COMPLÉMENTAIRE — une notion distincte du statut conventionnel ci-dessus. ${cadre ? "01 = cadre au sens des articles 4 et 4 bis de la CCN de 1947." : "04 = non cadre."}`, true],
    ['007', '01', "Nature du contrat : CDI de droit privé. Le simulateur ne modélise pas d'autre nature — c'est la rubrique qui compte 25 valeurs, du contrat de mission au mandat d'élu.", true],
    ['009', 'CTR0000001', "Numéro du contrat : la clé à laquelle s'accrochent rémunérations et bases assujetties. Doit rester STABLE d'un mois sur l'autre, sinon les déclarations ne se reconstituent plus."],
    ['011', '10', "Unité de mesure de la quotité de travail : l'heure. Un forfait jours prendrait la valeur 20.", true],
    ['012', '151.67', "Quotité de référence de l'entreprise pour la catégorie : le temps plein, soit 35 h × 52 / 12."],
    ['013', quotiteContrat, `Quotité du contrat : ${etp} % du temps plein.`],
    ['014', etp >= 100 ? '10' : '20', etp >= 100 ? "Temps plein." : "Temps partiel.", true],
    ['017', '0016', "Convention collective applicable au contrat."],
    ['018', '200', "Régime de base du risque MALADIE : régime général (CNAM). Le régime local d'Alsace-Moselle ne change pas cette valeur — c'est une cotisation complémentaire, pas un autre régime de base.", true],
    ['020', '200', "Régime de base du risque VIEILLESSE : régime général (CNAV).", true],
    ['026', '99', "Statut d'emploi : non concerné (rubrique du champ fonction publique).", true],
    cAtmp ? ['043', txDsn(num(cAtmp.taux_pat)), "Taux de cotisation accident du travail notifié à l'établissement."] : null,
  ]);

  bloc('S21.G00.71', '', [
    ['002', 'RUAA', "Régime unifié AGIRC-ARRCO, né de la fusion 2019. Les codes RETA (Arrco seul) et RETC (Arrco + Agirc) ne valent plus que pour des périodes antérieures.", true],
  ]);

  // ── Versement individu + prélèvement à la source ───────────────────────────
  bloc('S21.G00.50', '', [
    ['001', dtDsn(finMois), "Date de versement. C'est elle qui fixe le millésime d'imposition pour la DGFiP, pas la période de paie."],
    ['002', mtDsn(netImpo), "Rémunération nette fiscale : le brut moins les cotisations salariales obligatoires, CSG déductible comprise — mais SANS déduire la CSG non déductible ni la CRDS."],
    ['003', '1', "Numéro de versement dans le mois."],
    ['004', mtDsn(netVerse), "Montant net versé = RNF − CSG non déductible − CRDS. Piège classique : le prélèvement à la source ne s'en déduit PAS."],
    ['006', txDsn(pasTaux), "Taux de prélèvement à la source appliqué."],
    ['007', '13', "Type de taux : barème mensuel métropole, c'est-à-dire la grille de taux par défaut — le cas de l'employeur à qui la DGFiP n'a transmis aucun taux personnalisé. Le code 01 signalerait un taux transmis par la DGFiP.", true],
    ['009', mtDsn(pasTotal), "Montant du prélèvement à la source retenu."],
    ['013', mtDsn(netImpo), "Montant effectivement soumis au PAS. Toujours renseigné, même égal à la RNF."],
  ]);

  // ── Rémunérations ──────────────────────────────────────────────────────────
  const hs = b.heures_sup || null;
  const gainHs = hs ? num(hs.gain_hs) + num(hs.gain_hc) : 0;
  const heuresHs = hs ? num(hs.h_supp_25) + num(hs.h_supp_50) + num(hs.h_comp_10) + num(hs.h_comp_25) : 0;
  const salaireBase = +(brut - gainHs).toFixed(2);

  const remunerations = [
    { t: '001', montant: brut, heures: null,
      sens: "Rémunération brute non plafonnée : le total soumis à cotisations. C'est la ligne que consultent la plupart des organismes." },
    { t: '002', montant: baseCho || brut, heures: null,
      sens: "Salaire brut servant au calcul des droits à l'assurance chômage. Ne doit jamais inclure les indemnités de rupture, qui relèvent du bloc S21.G00.52." },
    { t: '010', montant: salaireBase, heures: null,
      sens: "Salaire de base : la rémunération habituelle hors compléments — généralement la première ligne du bulletin." },
    heuresHs > 0 ? { t: '017', montant: gainHs, heures: heuresHs,
      sens: "Heures supplémentaires ou complémentaires aléatoires, majorations comprises. Les heures structurelles (inscrites au contrat) relèveraient du type 018." } : null,
  ].filter(Boolean);

  remunerations.forEach(r => {
    bloc('S21.G00.51', `— type ${r.t} ${DSN_REM[r.t] || ''}`, [
      ['001', dtDsn(debutMois), "Début de la période de paie."],
      ['002', dtDsn(finMois), "Fin de la période de paie."],
      ['010', 'CTR0000001', "Numéro du contrat auquel la rémunération se rattache."],
      ['011', r.t, `Type de rémunération. ${r.sens}`, true],
      r.heures != null ? ['012', r.heures.toFixed(2), "Nombre d'heures."] : null,
      ['013', mtDsn(r.montant), "Montant."],
    ]);
  });

  // ── Bases assujetties et cotisations individuelles ─────────────────────────
  // Chaque cotisation individuelle est FILLE d'une base assujettie : c'est le
  // couple base + cotisation qui fait sens, jamais la cotisation seule.
  const smic = smicDepuisFillon(cFillon);

  const bases = [
    { c: '02', montant: basePlaf,
      sens: "Assiette brute plafonnée : la part du brut sous le plafond mensuel de sécurité sociale.",
      composants: [],
      cotis: [
        tot(cVieilPlaf) ? { code: '076', assiette: basePlaf, montant: tot(cVieilPlaf), taux: totTaux(cVieilPlaf),
          sens: "Assurance vieillesse, part plafonnée. Le code 076 apparaît DEUX fois dans un même fichier — une fois sous la base 02, une fois sous la base 03 : c'est normal, ce sont deux assiettes différentes du même risque." } : null,
      ].filter(Boolean) },
    { c: '03', montant: brut,
      sens: "Assiette brute déplafonnée : la totalité du brut. C'est la base la plus partagée de la norme, et celle à laquelle l'Agirc-Arrco rattache ses cotisations.",
      composants: smic ? [{ t: '01', montant: smic,
          sens: "Montant du SMIC retenu pour le calcul de la réduction générale. Sans ce composant, l'organisme ne peut pas refaire le calcul du coefficient." }] : [],
      cotis: [
        tot(cMaladie) ? { code: '075', assiette: brut, montant: tot(cMaladie), taux: totTaux(cMaladie),
          sens: "Assurance maladie, maternité, invalidité, décès." } : null,
        tot(cVieilDep) ? { code: '076', assiette: brut, montant: tot(cVieilDep), taux: totTaux(cVieilDep),
          sens: "Assurance vieillesse, part déplafonnée." } : null,
        tot(cFamille) ? { code: '074', assiette: brut, montant: tot(cFamille), taux: totTaux(cFamille),
          sens: "Allocations familiales, taux normal." } : null,
        tot(cAtmp) ? { code: '045', assiette: num(cAtmp.base), montant: tot(cAtmp), taux: totTaux(cAtmp),
          sens: "Accidents du travail et maladies professionnelles." } : null,
        tot(cAm) ? { code: '115', assiette: num(cAm.base), montant: tot(cAm), taux: totTaux(cAm),
          sens: "Régime local d'Alsace-Moselle. La norme lui réserve un code distinct : ce n'est pas une variante du code 075." } : null,
        mtAA ? { code: '105', assiette: brut, montant: mtAA, taux: null,
          sens: "Cotisation du régime unifié Agirc-Arrco. Le code 105 regroupe TOUT : tranche 1, tranche 2, CEG T1, CEG T2, CET et Apec. Il se rattache à la base 03, jamais à la base 02, et porte le montant AVANT réduction générale." } : null,
        mtFillon ? { code: '018', assiette: brut, montant: mtFillon, taux: null,
          sens: "Réduction générale des cotisations patronales. Montant négatif obligatoire : sans le signe, l'organisme le lit comme la régularisation d'un « trop allégé »." } : null,
        mtDfpHs ? { code: '021', assiette: brut, montant: mtDfpHs, taux: null,
          sens: "Déduction patronale au titre des heures supplémentaires." } : null,
        mtRedSalHs ? { code: '114', assiette: brut, montant: mtRedSalHs, taux: null,
          sens: "Réduction de cotisations salariales sur les heures supplémentaires et complémentaires." } : null,
      ].filter(Boolean) },
    baseCsg ? { c: '04', montant: baseCsg,
      sens: "Assiette de la CSG : 98,25 % du brut, l'abattement de 1,75 % pour frais professionnels.",
      composants: [],
      cotis: [
        { code: '072', assiette: baseCsg, montant: mtCsg, taux: txCsg,
          sens: "CSG sur salaires partiellement déductibles. Un seul code pour les deux fractions : la norme ne distingue pas la CSG déductible de la CSG non déductible — cette séparation est FISCALE, pas déclarative." },
        tot(cCrds) ? { code: '079', assiette: baseCsg, montant: tot(cCrds), taux: totTaux(cCrds),
          sens: "CRDS — « Remboursement de la dette sociale » dans le référentiel." } : null,
      ].filter(Boolean) } : null,
    baseCho && tot(cChomage) ? { c: '07', montant: baseCho,
      sens: "Assiette des contributions d'assurance chômage : le brut limité à 4 plafonds.",
      composants: [],
      cotis: [
        { code: '040', assiette: baseCho, montant: tot(cChomage), taux: totTaux(cChomage),
          sens: "Contribution d'assurance chômage sur rémunérations brutes après déduction." },
      ] } : null,
  ].filter(Boolean);

  bases.forEach(base => {
    bloc('S21.G00.78', `— code ${base.c} ${DSN_BASE[base.c] || ''}`, [
      ['001', base.c, `Code de base assujettie. ${base.sens}`, true],
      ['002', dtDsn(debutMois), "Début de la période de rattachement."],
      ['003', dtDsn(finMois), "Fin de la période de rattachement."],
      ['004', mtDsn(base.montant), "Montant de la base."],
      ['006', 'CTR0000001', "Numéro du contrat concerné."],
    ]);
    base.composants.forEach(k => {
      bloc('S21.G00.79', `— type ${k.t} ${DSN_COMPOSANT[k.t] || ''}`, [
        ['001', k.t, `Type de composant. ${k.sens}`, true],
        ['004', mtDsn(k.montant), "Montant du composant."],
      ]);
    });
    base.cotis.forEach(k => {
      bloc('S21.G00.81', `— code ${k.code} ${DSN_COT[k.code] || ''}`, [
        ['001', k.code, `Code de cotisation. ${k.sens}`, true],
        ['002', FAUX_SIRET_OPS, "Organisme destinataire."],
        ['003', mtDsn(k.assiette), "Montant d'assiette."],
        ['004', mtDsn(k.montant), "Montant de cotisation — part salariale ET part patronale confondues, contrairement au bulletin qui les sépare."],
        k.taux != null ? ['007', txDsn(k.taux), "Taux de cotisation. Obligatoire pour l'Urssaf et la MSA, hors réductions et contributions forfaitaires."] : null,
      ]);
    });
  });

  // ── S90 — Totaux ───────────────────────────────────────────────────────────
  // Le compte inclut les deux rubriques de S90 lui-même.
  const nbRub = blocs.reduce((s, bl) => s + bl.rubs.length, 0) + 2;
  bloc('S90.G00.90', '', [
    ['001', String(nbRub), "Nombre total de rubriques du fichier — S90 comprise. Un écart avec le contenu réel fait échouer le dépôt avant toute analyse métier."],
    ['002', '1', "Nombre de DSN contenues dans l'envoi."],
  ]);

  // ── Ce que cet extrait ne contient pas ─────────────────────────────────────
  // Énumérer les manques vaut mieux que les combler avec des valeurs inventées.
  lacunes.push(
    "**Identification réelle.** SIREN, NIC, SIRET des organismes, adresses, contacts : tout est fictif. Le SIREN 000 000 000 et le SIRET 000…000 passent la clé de Luhn mais ne sont jamais attribués. Aucune donnée saisie n'identifie une entreprise ou une personne réelle.",
    "**NIR.** L'individu est déclaré sous NTT (S21.G00.30.020) et non sous NIR (S21.G00.30.001), parce que le simulateur ne collecte ni numéro de sécurité sociale ni date de naissance. C'est la conduite correcte dans ce cas — pas un pis-aller.",
    "**Emploi.** Le code PCS-ESE (S21.G00.40.004) et le libellé de l'emploi (S21.G00.40.006), obligatoires dans une DSN réelle, sont absents : le simulateur ne demande pas le poste occupé.",
    "**Contributions non modélisées.** FNAL (code 049), contribution solidarité autonomie (068), AGS (048), versement mobilité (081), contribution au dialogue social (100), formation professionnelle (128) et taxe d'apprentissage (130) ne figurent pas au bulletin, donc pas ici. C'est la principale cause d'écart entre les taux CTP officiels affichés en regard et ceux du simulateur.",
    "**Répartition de la réduction générale.** Le montant entier part sous le code 018 (Urssaf). Une DSN réelle le scinde : la part Agirc-Arrco se déclare séparément sous le code 106, selon la règle « réduction Urssaf = réduction globale × (Σ taux Urssaf / Σ taux) ». Le simulateur ne produit qu'un montant global.",
    "**Prévoyance.** La cotisation de prévoyance cadre (1,50 % sur la tranche A, article 7 de la CCN de 1947) devrait se déclarer sous le code 059, rattachée à une base 31 et chaînée à une adhésion (S21.G00.15) puis à une affiliation (S21.G00.70). Le simulateur ne modélise aucun contrat collectif : la déclarer sans son chaînage produirait un fichier que l'organisme ne saurait pas rattacher.",
    "**Aide au poste (entreprise adaptée).** Versée par l'ASP, elle n'est pas une cotisation : aucun code DSN ne lui correspond dans le bloc 81.",
    "**Événements.** Arrêt de travail (S21.G00.60), fin de contrat (S21.G00.62), autre suspension (S21.G00.65) : une absence saisie dans le simulateur modifie les montants mais ne déclenche pas ici les blocs événementiels qui, en vrai, ouvrent les droits du salarié.",
    "**Maille agrégée approximative.** Les lignes CTP sont reconstruites à titre pédagogique. Les CTP réellement applicables dépendent de la situation de l'établissement (taux AT notifié, effectif, taux FNAL, exonérations) que le simulateur ne connaît pas. En production, l'agrégé doit se réconcilier au centime avec le nominatif.",
    "**Fonction publique.** Le bouton ne s'affiche que pour la France du secteur privé : la DSN publique passe par les rubriques marquées [FP], d'autres régimes (CNRACL 120, SRE 122, RAFP) et d'autres codes de cotisation (série 300).",
  );
  if (!smic) {
    lacunes.push("**Composant SMIC.** Le montant du SMIC retenu pour la réduction générale (S21.G00.79, type 01) n'a pas pu être relu depuis l'explication de la ligne Fillon — il est donc omis plutôt qu'inventé. Cas attendu lorsque l'application est affichée dans une autre langue que le français.");
  }

  // ── Fichier brut ───────────────────────────────────────────────────────────
  // Format d'une ligne : code de rubrique, virgule, valeur entre apostrophes.
  // Séparateur CRLF, encodage ISO 8859-1 (jamais UTF-8).
  const raw = blocs.flatMap(bl => bl.rubs.map(r => `${r.code},'${r.val}'`)).join('\r\n') + '\r\n';

  return { blocs, lacunes, raw, nbRub, listes: [...listesUtilisees] };
}

// ═════════════════════════════════════════════════════════════════════════════
// RENDU
// ═════════════════════════════════════════════════════════════════════════════

// Rubriques à liste fermée → table des valeurs autorisées. C'est le cœur du
// glossaire : une rubrique de ce type n'accepte RIEN d'autre que ces codes.
export const DSN_LISTES = {
  'S10.G00.00.005': DSN_ESSAI_REEL,
  'S10.G00.00.007': DSN_POINT_DEPOT,
  'S10.G00.00.008': DSN_TYPE_ENVOI,
  'S10.G00.02.001': DSN_CIVILITE,
  'S20.G00.05.001': DSN_NATURE_DECL,
  'S20.G00.05.002': DSN_TYPE_DECL,
  'S20.G00.05.008': DSN_CHAMP_DECL,
  'S20.G00.05.010': DSN_DEVISE,
  'S21.G00.11.017': DSN_NATURE_JURIDIQUE,
  'S21.G00.23.002': DSN_QUALIFIANT_ASSIETTE,
  'S21.G00.30.005': DSN_SEXE,
  'S21.G00.40.002': DSN_STATUT_CONV,
  'S21.G00.40.003': DSN_STATUT_RC,
  'S21.G00.40.007': DSN_NATURE_CONTRAT,
  'S21.G00.40.011': DSN_UNITE_QUOTITE,
  'S21.G00.40.014': DSN_MODALITE_TT,
  'S21.G00.40.018': DSN_REGIME_MALADIE,
  'S21.G00.40.020': DSN_REGIME_VIEILLESSE,
  'S21.G00.40.026': DSN_STATUT_EMPLOI,
  'S21.G00.50.007': DSN_TYPE_PAS,
  'S21.G00.51.011': DSN_REM,
  'S21.G00.54.001': DSN_AUTRE_REVENU,
  'S21.G00.71.002': DSN_REGIME_RC,
  'S21.G00.78.001': DSN_BASE,
  'S21.G00.79.001': DSN_COMPOSANT,
  'S21.G00.81.001': DSN_COT,
};

const _e = s => String(s)
  .replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
  .replace(/"/g, '&quot;').replace(/'/g, '&#039;');
// Gras minimal `**…**` dans les textes de lacunes — pas de moteur markdown ici.
const _g = s => _e(s).replace(/\*\*(.+?)\*\*/g, '<b>$1</b>');

// Textes bruts des panneaux, pour le bouton « copier ». Clé = identifiant du
// panneau ('d' bureau, 'm' mobile).
const _dsnRaw = new Map();

/**
 * Contenu du panneau : quatre onglets. Construit à la DEMANDE (premier clic sur
 * le bouton), pas à chaque calcul de bulletin — le glossaire complet pèse près
 * de deux mille lignes de tableau, inutile de les poser dans le DOM tant que
 * personne ne les regarde.
 * @param {object} b   Bulletin France.
 * @param {object} opt { id:'d'|'m', datePaie, pasTotal, pasTaux, versionLogiciel }
 */
function contenuDsn(b, opt = {}) {
  const id = opt.id || 'd';
  const d = buildDsn(b, opt);
  _dsnRaw.set(id, d.raw);

  // Valeurs effectivement retenues, pour les surligner dans le glossaire.
  const retenues = new Map();
  d.blocs.forEach(bl => bl.rubs.forEach(r => {
    if (!DSN_LISTES[r.code]) return;
    if (!retenues.has(r.code)) retenues.set(r.code, new Set());
    retenues.get(r.code).add(r.val);
  }));

  // ── Onglet « annoté » ──
  const annote = d.blocs.map(bl => `
    <div class="dsn-bloc">
      <div class="dsn-bloc-head">
        <span class="dsn-bloc-code">${_e(bl.code)}</span>
        <span class="dsn-bloc-lib">${_e(bl.lib)}${bl.suffixe ? ' ' + _e(bl.suffixe) : ''}</span>
      </div>
      ${bl.role ? `<div class="dsn-bloc-role">${_e(bl.role)}</div>` : ''}
      ${bl.rubs.map(r => `
        <div class="dsn-rub">
          <div class="dsn-rub-line"><span class="dsn-code">${_e(r.code)}</span><span class="dsn-virg">,</span><span class="dsn-val">'${_e(r.val)}'</span></div>
          <div class="dsn-rub-lib">${_e(r.lib)}</div>
          <div class="dsn-rub-sens">${_e(r.sens)}</div>
        </div>`).join('')}
    </div>`).join('');

  // ── Onglet « fichier brut » ──
  const brut = `
    <div class="dsn-note">Une ligne par rubrique : <code>code,'valeur'</code>. Séparateur CRLF, encodage ISO 8859-1 (Latin-1) — jamais UTF-8, un « é » mal encodé suffit à faire rejeter un dépôt.</div>
    <button class="dsn-copy" onclick="dsnCopier('${id}', this)">⧉ copier les ${d.nbRub} lignes</button>
    <pre class="dsn-raw">${_e(d.raw)}</pre>`;

  // ── Onglet « tous les codes » ──
  // Les tables sont des objets : JavaScript hisse en tête les clés qui ressemblent
  // à des indices de tableau ('10' avant '01'…). On rétablit l'ordre du cahier
  // technique — croissant, les codes non numériques à la fin.
  const ordreCode = ([a], [b]) => {
    const na = /^\d+$/.test(a), nb = /^\d+$/.test(b);
    if (na && nb) return Number(a) - Number(b);
    if (na !== nb) return na ? -1 : 1;
    return a.localeCompare(b, 'fr');
  };

  const listeHtml = (rub, table) => {
    const used = retenues.get(rub);
    const n = Object.keys(table).length;
    const rows = Object.entries(table).sort(ordreCode).map(([c, l]) => `
      <tr class="${used && used.has(c) ? 'dsn-on' : ''}">
        <td class="dsn-c">${_e(c)}</td>
        <td>${_e(l)}${used && used.has(c) ? ' <span class="dsn-tag">← retenu ici</span>' : ''}</td>
      </tr>`).join('');
    const tbl = `<table class="dsn-tbl"><tbody>${rows}</tbody></table>`;
    const titre = `<span class="dsn-code">${_e(rub)}</span> · ${_e(DSN_RUBRIQUES[rub] || '')} <span class="dsn-n">${n} valeurs</span>`;
    return n > 20
      ? `<details class="dsn-det"><summary>${titre}</summary>${tbl}</details>`
      : `<div class="dsn-det"><div class="dsn-det-t">${titre}</div>${tbl}</div>`;
  };

  const ctpRows = Object.entries(DSN_CTP).sort(ordreCode).map(([c, v]) => `
    <tr><td class="dsn-c">${_e(c)}</td><td>${_e(v.lib)}</td>
        <td class="dsn-r">${v.plaf != null ? v.plaf.toFixed(2) + ' %' : '—'}</td>
        <td class="dsn-r">${v.deplaf != null ? v.deplaf.toFixed(2) + ' %' : '—'}</td></tr>`).join('');

  const rubRows = Object.entries(DSN_RUBRIQUES).map(([c, l]) =>
    `<tr><td class="dsn-c">${_e(c)}</td><td>${_e(l)}</td></tr>`).join('');

  const codes = `
    <div class="dsn-note">Une rubrique « à liste fermée » n'accepte aucune valeur en dehors de sa table. Les valeurs employées dans l'extrait sont surlignées. Référentiel : cahier technique NEODeS CT2026.1.</div>
    ${Object.entries(DSN_LISTES).map(([r, t]) => listeHtml(r, t)).join('')}
    <details class="dsn-det"><summary><span class="dsn-code">Codes types de personnel</span> · maille agrégée Urssaf <span class="dsn-n">extrait</span></summary>
      <div class="dsn-note">Taux en vigueur, source open.urssaf.fr. Affichés pour comparaison : ce ne sont pas les taux déclarés par le simulateur.</div>
      <table class="dsn-tbl"><thead><tr><th>CTP</th><th>Libellé Urssaf</th><th class="dsn-r">Plafonné</th><th class="dsn-r">Déplafonné</th></tr></thead><tbody>${ctpRows}</tbody></table>
    </details>
    <details class="dsn-det"><summary><span class="dsn-code">Toutes les rubriques</span> · des blocs couverts <span class="dsn-n">${Object.keys(DSN_RUBRIQUES).length} rubriques</span></summary>
      <table class="dsn-tbl"><tbody>${rubRows}</tbody></table>
    </details>`;

  // ── Onglet « lacunes » ──
  const lacunes = `
    <div class="dsn-note">Ce fichier n'est pas déposable. Voici précisément pourquoi — énumérer les manques vaut mieux que les combler avec des valeurs inventées.</div>
    <ul class="dsn-lac">${d.lacunes.map(l => `<li>${_g(l)}</li>`).join('')}</ul>`;

  return `
    <div class="dsn-warn">Extrait pédagogique — <b>envoi de test</b> (S10.G00.00.005 = 01), identifiants fictifs, individu déclaré sous NTT. Non déposable sur net-entreprises.</div>
    <div class="dsn-tabs">
      <button class="dsn-tab is-on" data-v="annote"  onclick="dsnVue('${id}','annote')">ANNOTÉ</button>
      <button class="dsn-tab"       data-v="brut"    onclick="dsnVue('${id}','brut')">FICHIER BRUT</button>
      <button class="dsn-tab"       data-v="codes"   onclick="dsnVue('${id}','codes')">TOUS LES CODES</button>
      <button class="dsn-tab"       data-v="lacunes" onclick="dsnVue('${id}','lacunes')">LACUNES</button>
    </div>
    <div class="dsn-vue" id="dsn-${id}-annote">${annote}</div>
    <div class="dsn-vue" id="dsn-${id}-brut"    style="display:none">${brut}</div>
    <div class="dsn-vue" id="dsn-${id}-codes"   style="display:none">${codes}</div>
    <div class="dsn-vue" id="dsn-${id}-lacunes" style="display:none">${lacunes}</div>`;
}

// Bulletin + contexte du dernier rendu, par panneau. Sert au montage paresseux.
const _dsnCtx = new Map();

/**
 * Amorce du panneau DSN : le chapeau explicatif et le bouton. Le contenu n'est
 * fabriqué qu'au premier dépliage (cf. window.dsnBascule).
 * Ne rendre que pour la France du secteur privé — la DSN publique parle une
 * autre langue (rubriques [FP], régimes CNRACL/SRE/RAFP, cotisations série 300).
 * @param {object} b   Bulletin France.
 * @param {object} opt { id:'d'|'m', datePaie, pasTotal, pasTaux, versionLogiciel }
 */
export function renderDsnPanel(b, opt = {}) {
  const id = opt.id || 'd';
  _dsnCtx.set(id, { b, opt: { ...opt, id } });
  return `
  <div class="dsn-wrap trad-skip">
    <div class="tbl-section-head">── DSN — DÉCLARATION SOCIALE NOMINATIVE ───────────────────────────</div>
    <div class="dsn-intro">
      Depuis 2017, ce bulletin ne reste pas dans l'entreprise : il part chaque mois,
      salarié par salarié, vers l'Urssaf, l'Agirc-Arrco, France Travail, la DGFiP et
      les organismes complémentaires, sous la forme d'un fichier texte normalisé — la
      DSN. Voici à quoi ressemble la traduction de CE bulletin, rubrique par rubrique.
    </div>
    <button class="dsn-btn" id="dsn-btn-${id}" onclick="dsnBascule('${id}')">▸ VOIR L'EXTRAIT DE DSN</button>
    <div class="dsn-panel" id="dsn-panel-${id}" style="display:none"></div>
  </div>`;
}

// ── Handlers exposés au HTML inline (même convention que le reste du front) ──
window.dsnBascule = function(id) {
  const p = document.getElementById(`dsn-panel-${id}`);
  const b = document.getElementById(`dsn-btn-${id}`);
  if (!p) return;
  const ouvert = p.style.display !== 'none';
  if (!ouvert && !p.dataset.monte) {
    const ctx = _dsnCtx.get(id);
    if (!ctx) return;
    try {
      p.innerHTML = contenuDsn(ctx.b, ctx.opt);
      p.dataset.monte = '1';
    } catch (err) {
      console.error('[DSN] construction impossible :', err);
      p.innerHTML = `<div class="dsn-note">Extrait DSN indisponible : ${_e(err.message || err)}</div>`;
      p.dataset.monte = '1';
    }
  }
  p.style.display = ouvert ? 'none' : 'block';
  if (b) b.textContent = ouvert ? "▸ VOIR L'EXTRAIT DE DSN" : "▾ MASQUER L'EXTRAIT DE DSN";
};

window.dsnVue = function(id, vue) {
  ['annote', 'brut', 'codes', 'lacunes'].forEach(v => {
    const el = document.getElementById(`dsn-${id}-${v}`);
    if (el) el.style.display = v === vue ? 'block' : 'none';
  });
  document.querySelectorAll(`#dsn-panel-${id} .dsn-tab`).forEach(t => {
    t.classList.toggle('is-on', t.dataset.v === vue);
  });
};

window.dsnCopier = async function(id, btn) {
  const txt = _dsnRaw.get(id);
  if (!txt) return;
  const libelle = btn ? btn.textContent : '';
  try {
    await navigator.clipboard.writeText(txt);
    if (btn) btn.textContent = '✓ copié';
  } catch {
    if (btn) btn.textContent = '✗ presse-papier refusé';
  }
  if (btn) setTimeout(() => { btn.textContent = libelle; }, 2000);
};
