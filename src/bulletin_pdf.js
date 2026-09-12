// ═════════════════════════════════════════════════════════════════════════════
// BULLETIN DE PAIE — modèle réglementaire et composition du PDF
// ═════════════════════════════════════════════════════════════════════════════
//
// Ce module traduit un bulletin déjà calculé par le back Rust en un DOCUMENT :
// le modèle réglementaire du bulletin de paie français, rubrique par rubrique,
// tel que l'impose l'article R. 3243-2 du code du travail. Le Rust reçoit
// ensuite ce document tout composé et n'en fait que la mise en page.
//
// ── Pourquoi côté front ? ────────────────────────────────────────────────────
// Même raison que pour la DSN (cf. dsn.js) : ce n'est pas un calcul, c'est une
// TRADUCTION d'un bulletin déjà produit. Deux données nécessaires ne vivent que
// côté front — le prélèvement à la source (calculerPas) et la date du
// formulaire — et le regroupement dépend du modèle applicable à cette date.
//
// ── Les deux modèles ─────────────────────────────────────────────────────────
// L'arrêté du 25 février 2016 fixe « les libellés, l'ordre et le regroupement
// des informations figurant sur le bulletin de paie ». Il a été modifié par
// l'arrêté du 31 janvier 2023, qui institue un modèle RÉNOVÉ ; l'arrêté du
// 11 août 2025 a reporté au 1er janvier 2027 l'obligation de l'appliquer.
// Jusqu'au 31 décembre 2026, le modèle ADAPTÉ reste utilisable : celui de 2016,
// complété de la seule ligne « Montant net social ».
//
// Le simulateur couvre les dates de paie depuis 2015 : la bascule se fait donc
// sur la date, pas sur l'année en cours. MODELE_BASCULE ci-dessous est la seule
// déclaration de cette frontière.
//
// ── Ce que ce document N'EST PAS ─────────────────────────────────────────────
// Ce n'est pas un bulletin de paie. C'est la sortie d'un simulateur, imprimée
// selon le modèle réglementaire pour qu'on puisse la lire. L'employeur, ses
// identifiants, la convention collective et la classification sont TIRÉS AU
// SORT — le simulateur ne les connaît pas et n'a pas à les inventer en douce.
// D'où le filigrane SPÉCIMEN, le bandeau d'avertissement, et les rubriques
// réglementaires laissées explicitement vides avec la mention de ce qui manque,
// exactement comme l'onglet « lacunes » de la DSN.
// ═════════════════════════════════════════════════════════════════════════════

/** Date d'entrée en vigueur obligatoire du modèle rénové (arrêté du 11/08/2025). */
export const MODELE_BASCULE = '2027-01-01';

/** 'adapte' jusqu'au 31/12/2026, 'renove' à partir du 01/01/2027. */
export function modeleApplicable(dateIso) {
  return (dateIso || '') >= MODELE_BASCULE ? 'renove' : 'adapte';
}

// ── Regroupement réglementaire ───────────────────────────────────────────────
//
// Chaque poste porte le libellé du modèle et la liste des codes de cotisation du
// moteur qui l'alimentent. Un poste dont aucun code n'est présent ne s'imprime
// pas ; une RUBRIQUE entière sans poste s'imprime quand même, suivie de la
// mention de ce qu'elle aurait dû contenir — une rubrique absente se remarque
// moins qu'une rubrique vide, et c'est bien le contraire qu'on veut.
//
// `sans_modele` : rubrique qu'aucune ligne du simulateur ne peut alimenter.

const POSTE_SANTE = [
  // `fusion` : le modèle n'ouvre qu'UNE ligne là où le moteur en calcule
  // plusieurs sur la même assiette. On additionne alors taux et montants, comme
  // le fait tout logiciel de paie — c'est le cas du régime local d'Alsace-Moselle,
  // qui majore la cotisation maladie sans constituer une ligne à part.
  { lbl: 'Sécurité sociale - Maladie Maternité Invalidité Décès',
    codes: ['SS_MALADIE', 'ALSACE_MOSELLE_MALADIE'], fusion: true },
  { lbl: 'Complémentaire Incapacité Invalidité Décès',
    codes: ['PREVOYANCE_CADRE_MIN'] },
  { lbl: 'Complémentaire Santé', codes: [] },
];

const POSTE_ATMP = [
  { lbl: 'Accidents du travail - Maladies professionnelles', codes: ['AT_MP'] },
];

const POSTE_RETRAITE = [
  { lbl: 'Sécurité sociale plafonnée',   codes: ['SS_VIEILLESSE_PLAF'] },
  { lbl: 'Sécurité sociale déplafonnée', codes: ['SS_VIEILLESSE_DEPLAF'] },
  { lbl: 'Complémentaire Tranche 1',     codes: ['AGIRC_ARRCO_T1', 'AGIRC_ARRCO_CEG_T1'] },
  { lbl: 'Complémentaire Tranche 2',     codes: ['AGIRC_ARRCO_T2'] },
  { lbl: 'Retraite (CNRACL)',            codes: ['FPT_CNRACL'] },
];

const POSTE_FAMILLE = [{ lbl: 'Famille', codes: ['FAMILLE'] }];
const POSTE_CHOMAGE = [
  { lbl: 'Chômage', codes: ['CHOMAGE'] },
  { lbl: 'APEC',    codes: [] },
];

const POSTE_CSG = [
  { lbl: "CSG déductible de l'impôt sur le revenu", codes: ['CSG_DEDUCTIBLE'] },
  { lbl: "CSG/CRDS non déductible de l'impôt sur le revenu",
    codes: ['CSG_NON_DEDUCTIBLE', 'CRDS'], fusion: true },
];

const POSTE_ALLEGEMENTS = [
  { lbl: 'Réduction générale des cotisations patronales', codes: ['REDUCTION_FILLON'] },
  { lbl: 'Déduction forfaitaire patronale (heures supplémentaires)', codes: ['DFP_HS'] },
  { lbl: 'Réduction salariale (heures supplémentaires et complémentaires)', codes: ['REDUC_SAL_HS'] },
  { lbl: "Aide au poste (entreprise adaptée)", codes: ['AIDE_POSTE_EA'] },
];

const LACUNE_AUTRES =
  "Le simulateur ne modélise pas ces contributions : FNAL, versement mobilité, "
  + "contribution au dialogue social, formation professionnelle, taxe d'apprentissage, "
  + "contribution solidarité autonomie, forfait social. Cette rubrique est donc vide.";

const LACUNE_STATUTAIRES =
  "Aucune convention collective n'est appliquée : le simulateur calcule le régime légal. "
  + "Les cotisations conventionnelles éventuelles (prévoyance, retraite supplémentaire, "
  + "frais de santé au-delà du panier de soins) ne figurent pas ici.";

const LACUNE_FACULTATIVES =
  "Le simulateur ne modélise aucun régime facultatif (retraite supplémentaire, "
  + "prévoyance ou frais de santé au-delà des obligations légales et conventionnelles).";

// La fonction publique territoriale ne cotise pas aux mêmes guichets que le
// privé : lui servir les lacunes du privé serait aussi faux que de lui servir
// ses cotisations. Un agent titulaire n'a pas de convention collective, pas de
// taxe d'apprentissage, et son employeur verse au CNFPT et au centre de gestion.
const LACUNE_AUTRES_FPT =
  "Le simulateur ne modélise pas ces contributions : cotisation au CNFPT, "
  + "contribution au centre de gestion, FNAL, versement mobilité, contribution "
  + "solidarité autonomie, allocation temporaire d'invalidité. Cette rubrique est donc vide.";

const LACUNE_STATUTAIRES_FPT =
  "Un agent titulaire ne relève d'aucune convention collective. Ne sont pas modélisés : "
  + "la retraite additionnelle (RAFP, assise sur les primes), la nouvelle bonification "
  + "indiciaire, le supplément familial de traitement, l'indemnité de résidence, ni le "
  + "régime indemnitaire (RIFSEEP).";

/** Modèle « adapté » — arrêté du 25 février 2016, complété du montant net social. */
const modeleAdapte = fpt => [
  { titre: 'SANTÉ', postes: POSTE_SANTE },
  { titre: 'ACCIDENTS DU TRAVAIL - MALADIES PROFESSIONNELLES', postes: POSTE_ATMP },
  { titre: 'RETRAITE', postes: POSTE_RETRAITE },
  { titre: 'FAMILLE', postes: POSTE_FAMILLE },
  { titre: 'ASSURANCE CHÔMAGE', postes: POSTE_CHOMAGE },
  { titre: "AUTRES CONTRIBUTIONS DUES PAR L'EMPLOYEUR", postes: [],
    lacune: fpt ? LACUNE_AUTRES_FPT : LACUNE_AUTRES },
  { titre: fpt ? 'COTISATIONS STATUTAIRES'
               : 'COTISATIONS STATUTAIRES OU PRÉVUES PAR LA CONVENTION COLLECTIVE',
    postes: [], lacune: fpt ? LACUNE_STATUTAIRES_FPT : LACUNE_STATUTAIRES },
  // Les lignes de CSG ne portent pas de bandeau dans le modèle de 2016 : elles
  // suivent directement les rubriques ci-dessus. Les allègements, eux, forment
  // un bloc identifié — c'est là que se lit ce que l'employeur ne paie pas.
  { titre: '', postes: POSTE_CSG },
  { titre: 'EXONÉRATIONS ET ALLÈGEMENTS DE COTISATIONS', postes: POSTE_ALLEGEMENTS },
];

/** Modèle « rénové » — arrêté du 31 janvier 2023, obligatoire au 01/01/2027. */
const modeleRenove = fpt => [
  // `bandeau` : rubrique qui n'est qu'un titre — elle chapeaute les sous-groupes
  // qui suivent et s'imprime même sans ligne à elle.
  { titre: 'COTISATIONS ET CONTRIBUTIONS SOCIALES OBLIGATOIRES', postes: [], bandeau: true },
  { sous: 'Santé', postes: POSTE_SANTE },
  { sous: 'Accidents du travail - Maladies professionnelles', postes: POSTE_ATMP },
  { sous: 'Retraite', postes: POSTE_RETRAITE },
  { sous: 'Famille', postes: POSTE_FAMILLE },
  { sous: 'Assurance chômage', postes: POSTE_CHOMAGE },
  { sous: "Autres contributions dues par l'employeur", postes: [],
    lacune: fpt ? LACUNE_AUTRES_FPT : LACUNE_AUTRES },
  { sous: 'CSG et CRDS', postes: POSTE_CSG },
  { titre: 'COTISATIONS ET CONTRIBUTIONS SOCIALES FACULTATIVES',
    postes: [], lacune: fpt ? LACUNE_STATUTAIRES_FPT : LACUNE_FACULTATIVES },
  { titre: 'EXONÉRATIONS, ALLÈGEMENTS ET AIDES', postes: POSTE_ALLEGEMENTS },
];

// ── Mise en forme ────────────────────────────────────────────────────────────
// Le back ne formate rien : tout ce qui suit produit les chaînes telles qu'elles
// seront imprimées.

const _n = v => {
  const x = parseFloat(v);
  return Number.isFinite(x) ? x : 0;
};

const _eur = v => _n(v).toLocaleString('fr-FR', {
  minimumFractionDigits: 2, maximumFractionDigits: 2,
});

/** Quantité (heures) : deux décimales, « 151,67 ». */
const _qte = v => _n(v).toLocaleString('fr-FR', {
  minimumFractionDigits: 2, maximumFractionDigits: 2,
});

/** Taux horaire : quatre décimales, sinon « nombre × base » ne retombe pas sur le montant. */
const _tauxH = v => _n(v).toLocaleString('fr-FR', {
  minimumFractionDigits: 4, maximumFractionDigits: 4,
});

/** Montant, ou chaîne vide si nul — une colonne vide vaut mieux qu'un « 0,00 ». */
const _eurOuRien = v => (Math.abs(_n(v)) < 0.005 ? '' : _eur(v));

/** Taux exprimé en fraction (0,0245) → « 2,450 % ». Trois décimales : la CEG et
 *  la CSG en ont besoin, et un taux faux de 0,001 se voit tout de suite. */
const _pct = v => {
  if (Math.abs(_n(v)) < 1e-9) return '';
  return (_n(v) * 100).toLocaleString('fr-FR', {
    minimumFractionDigits: 3, maximumFractionDigits: 3,
  }) + ' %';
};

const MOIS = ['janvier', 'février', 'mars', 'avril', 'mai', 'juin',
  'juillet', 'août', 'septembre', 'octobre', 'novembre', 'décembre'];

const _jj = iso => (iso || '').split('-')[2] || '';
const _dateFr = iso => {
  if (!iso) return '';
  const [y, m, d] = iso.split('-');
  return `${d}/${m}/${y}`;
};
const _moisFr = iso => {
  if (!iso) return '';
  const [y, m] = iso.split('-');
  return `${MOIS[parseInt(m, 10) - 1] || ''} ${y}`;
};
const _finDeMois = iso => {
  const [y, m] = (iso || '').split('-').map(Number);
  if (!y || !m) return iso || '';
  return `${y}-${String(m).padStart(2, '0')}-${String(new Date(y, m, 0).getDate()).padStart(2, '0')}`;
};
const _debutDeMois = iso => `${(iso || '').slice(0, 7)}-01`;

// ── Composition ──────────────────────────────────────────────────────────────

/**
 * Fabrique le document envoyé au moteur Rust.
 *
 * @param {object} b   Bulletin France ou fonction publique, tel que rendu par le back.
 * @param {object} opt {
 *   datePaie, pas, identite, remBase, remLignes, etp, heuresMois, versionLogiciel
 * }
 */
export function composerBulletinPdf(b, opt = {}) {
  const id       = opt.identite || {};
  const datePaie = opt.datePaie || '';
  const pas      = opt.pas || { total: 0, taux_effectif: 0 };
  const fpt      = b.salarie?.pays === 'fonction_publique';
  const modele   = modeleApplicable(datePaie);
  const cots     = Array.isArray(b.cotisations) ? b.cotisations : [];
  const etp      = _n(opt.etp) || 100;
  const heures   = _n(opt.heuresMois) || 151.67;
  const quotite  = heures * etp / 100;

  // ── Haut de bulletin : la rémunération ─────────────────────────────────────
  // Disposition commune des logiciels de paie : une ligne d'heures se lit
  // « nombre × base × taux = à payer » ; une retenue tombe dans « à déduire »,
  // sans signe — c'est la colonne qui dit le sens.
  const rem = [];
  const base = _n(opt.remBase);
  const hs = b.heures_sup;
  // Taux horaire de base à quatre décimales, comme le calcule le moteur.
  const tauxH = hs ? _n(hs.taux_horaire)
    : (quotite > 0 ? Math.round(base / quotite * 1e4) / 1e4 : 0);
  rem.push({
    libelle: fpt ? 'Traitement indiciaire brut' : 'Salaire de base',
    nombre: _qte(quotite),
    base: tauxH ? _tauxH(tauxH) : '',
    a_payer: _eur(base),
  });

  for (const l of (opt.remLignes || [])) {
    if (/^h[sc]\d+$/.test(l.type)) continue; // heures supp/compl : détaillées plus bas
    const m = _n(l.amount);
    if (!m) continue;
    rem.push({
      libelle: l.type === 'coupure_50' ? 'Majoration pour coupure (50 %)' : 'Prime',
      a_payer: _eur(m),
    });
  }

  if (hs) {
    const th = _n(hs.taux_horaire);
    const ligneHeures = (h, maj, lbl) => {
      if (!h) return;
      rem.push({
        libelle: lbl,
        nombre: _qte(h),
        base: _tauxH(th),
        taux_sal: (maj * 100).toLocaleString('fr-FR', { minimumFractionDigits: 2 }) + ' %',
        // Arrondie au centime ligne par ligne, comme le moteur (heures_sup.rs).
        a_payer: _eur(Math.round(h * th * maj * 100) / 100),
      });
    };
    ligneHeures(hs.h_supp_25, 1.25, 'Heures supplémentaires à 25 %');
    ligneHeures(hs.h_supp_50, 1.50, 'Heures supplémentaires à 50 %');
    ligneHeures(hs.h_comp_10, 1.10, 'Heures complémentaires à 10 %');
    ligneHeures(hs.h_comp_25, 1.25, 'Heures complémentaires à 25 %');
  }

  const abs = b.absence;
  if (abs) {
    rem.push({ libelle: `Absence — ${abs.libelle || 'arrêt de travail'}`,
               a_deduire: _eur(abs.retenue) });
    if (_n(abs.maintien) > 0) {
      rem.push({ libelle: `Maintien de salaire (${abs.convention || 'régime légal'})`,
                 a_payer: _eur(abs.maintien) });
    }
    if (_n(abs.ijss_brut) > 0) {
      rem.push({ libelle: 'Indemnités journalières de sécurité sociale (subrogation)',
                 a_deduire: _eur(abs.ijss_brut) });
    }
    if (_n(abs.ajustement_net) > 0) {
      rem.push({ libelle: 'Ajustement au titre de la garantie du net',
                 a_deduire: _eur(abs.ajustement_net) });
    }
  }

  const cp = b.conges;
  if (cp) {
    rem.push({ libelle: `Absence — ${cp.libelle || 'congés payés'}`,
               a_deduire: _eur(cp.retenue) });
    rem.push({
      libelle: 'Indemnité de congés payés ('
        + (cp.methode_indemnite === 'dixieme' ? 'règle du dixième' : 'maintien de salaire') + ')',
      a_payer: _eur(cp.indemnite),
    });
  }

  rem.push({ libelle: fpt ? 'RÉMUNÉRATION BRUTE' : 'SALAIRE BRUT',
             a_payer: _eur(b.brut), fort: true });

  // ── Corps : les rubriques réglementaires ──────────────────────────────────
  const gabarit = modele === 'renove' ? modeleRenove(fpt) : modeleAdapte(fpt);
  const rubriques = [{ titre: '', lignes: rem }];
  const utilises = new Set();

  for (const r of gabarit) {
    const lignes = [];
    if (r.sous) {
      lignes.push({ libelle: r.sous, fort: true });
    }
    for (const p of r.postes) {
      const trouvees = cots.filter(c => p.codes.includes(c.code));
      trouvees.forEach(c => utilises.add(c.code));
      if (!trouvees.length) continue;
      if (p.fusion && trouvees.length > 1) {
        lignes.push(ligneFusionnee(trouvees, p.lbl));
      } else {
        // Plusieurs lignes distinctes sous un même poste gardent le libellé du
        // moteur : « Complémentaire Tranche 1 » et la CEG sont deux
        // contributions, pas deux façons de dire la même chose.
        for (const c of trouvees) {
          lignes.push(ligneDe(c, trouvees.length > 1 ? c.libelle : p.lbl));
        }
      }
    }
    const vide = lignes.length === (r.sous ? 1 : 0);
    if (vide && r.lacune) {
      lignes.push({ libelle: r.lacune, note: true });
    }
    if (vide && !r.lacune && !r.bandeau) continue;
    rubriques.push({ titre: r.titre || '', lignes });
  }

  // Filet de sécurité : une cotisation que le gabarit ne connaît pas ne doit pas
  // disparaître du document. Elle atterrit ici plutôt que dans le silence — le
  // jour où le moteur gagne une ligne, elle se voit.
  const orphelines = cots.filter(c => !utilises.has(c.code));
  if (orphelines.length) {
    rubriques.push({
      titre: 'AUTRES COTISATIONS ET CONTRIBUTIONS',
      lignes: [
        { libelle: 'Lignes calculées par le simulateur qu’aucune rubrique du modèle '
                 + 'réglementaire ne recouvre. Leur place sur un bulletin réel est à vérifier.',
          note: true },
        ...orphelines.map(c => ligneDe(c, c.libelle)),
      ],
    });
  }

  const totalSal = cots.reduce((s, c) => s + _n(c.montant_sal), 0);
  const totalPat = cots.reduce((s, c) => s + _n(c.montant_pat), 0);
  rubriques.push({
    titre: '',
    lignes: [{
      libelle: 'TOTAL DES COTISATIONS ET CONTRIBUTIONS',
      a_deduire: _eur(totalSal),
      montant_pat: _eur(totalPat),
      fort: true,
    }],
  });

  // ── Bas de bulletin ───────────────────────────────────────────────────────
  // Montant net social : rémunération brute diminuée des seules cotisations et
  // contributions sociales OBLIGATOIRES. Le simulateur n'en calcule pas d'autres,
  // et les IJSS versées par la caisse n'en font pas partie — elles sont déclarées
  // par l'organisme qui les verse, pas par l'employeur subrogé.
  const ijssNet = abs ? _n(abs.ijss_net) : 0;
  const netSocial = _n(b.brut) - totalSal;
  const netAvantImpot = _n(b.net_a_payer);
  const netImposable = _n(b.net_imposable);
  const pasTotal = _n(pas.total);
  const netPaye = netAvantImpot - pasTotal;

  const totaux = [];
  if (ijssNet > 0) {
    totaux.push({
      libelle: 'Indemnités journalières nettes réintégrées (subrogation)',
      valeur: _eur(ijssNet),
      note: 'Versées par la caisse à l’employeur, qui les reverse au salarié. '
          + 'La CSG et la CRDS y ont déjà été précomptées.',
    });
  }
  totaux.push({
    libelle: 'Montant net social',
    valeur: _eur(netSocial),
    poids: 1,
    note: 'Montant à déclarer pour le RSA et la prime d’activité : rémunération brute '
        + 'diminuée des cotisations et contributions sociales obligatoires.',
  });
  totaux.push({
    libelle: "Net à payer avant impôt sur le revenu",
    valeur: _eur(netAvantImpot),
    poids: 1,
  });
  totaux.push({
    libelle: 'Net imposable',
    valeur: _eur(netImposable),
    note: hs && _n(hs.exo_fiscale) > 0
      ? `Déduction faite de ${_eur(hs.exo_fiscale)} € d’heures supplémentaires exonérées d’impôt sur le revenu.`
      : '',
  });
  totaux.push({
    libelle: `Impôt sur le revenu prélevé à la source — base ${_eur(netImposable)} €, taux ${
      (_n(pas.taux_effectif) * 100).toLocaleString('fr-FR', { minimumFractionDigits: 2, maximumFractionDigits: 2 })} %`,
    valeur: '− ' + _eur(pasTotal),
    note: 'Taux neutre du barème mensuel de la DGFiP (personne seule, aucune part '
        + 'supplémentaire). Le taux personnalisé transmis par l’administration n’est pas simulé.',
  });
  totaux.push({ libelle: 'NET PAYÉ EN EUROS', valeur: _eur(netPaye), poids: 2 });
  totaux.push({
    libelle: "Montant total versé par l'employeur",
    valeur: _eur(b.cout_total_employeur),
    note: 'Rémunération brute augmentée des cotisations et contributions patronales, '
        + 'diminuée des exonérations et allègements.',
  });

  // ── Cumuls ────────────────────────────────────────────────────────────────
  // Un mois isolé n'a pas d'historique : le cumul annuel d'un simulateur qui ne
  // connaît qu'un bulletin serait ce bulletin, présenté comme une année. On
  // n'imprime donc que ce qui est vrai — la période et le compteur de congés.
  const cumuls = [
    { l: 'Période', v: _moisFr(datePaie) },
    { l: 'Brut du mois', v: _eur(b.brut) + ' €' },
    { l: 'Net imposable du mois', v: _eur(netImposable) + ' €' },
    { l: 'Coût employeur du mois', v: _eur(b.cout_total_employeur) + ' €' },
  ];
  // 2,5 jours ouvrables par mois de travail effectif (C. trav. art. L3141-3).
  // La fonction publique territoriale relève d'un autre décompte (5 fois les
  // obligations hebdomadaires de service) : on se tait plutôt que d'y appliquer
  // une règle du privé.
  if (!fpt) {
    cumuls.push({ l: 'Congés acquis (mois)', v: '2,50 j ouvrables' });
  }

  // ── Mentions légales obligatoires ─────────────────────────────────────────
  const mentions = [
    'Dans votre intérêt et pour vous aider à faire valoir vos droits, conservez ce bulletin '
    + 'de paie sans limitation de durée.',
    'Pour toute information complémentaire sur le bulletin de paie : www.service-public.fr',
    fpt
      ? 'Le modèle réglementaire du bulletin de paie (art. R. 3243-2 du code du travail, '
        + 'arrêté du 25 février 2016) régit les salariés de droit privé : il ne s’applique '
        + 'pas aux agents titulaires. Sa présentation est reprise ici pour la seule commodité '
        + 'de la lecture et de la comparaison.'
      : 'Modèle appliqué : ' + (modele === 'renove'
          ? 'modèle rénové (arrêté du 31 janvier 2023 modifiant l’arrêté du 25 février 2016), '
            + 'obligatoire pour les paies établies à compter du 1er janvier 2027.'
          : 'modèle adapté (arrêté du 25 février 2016, complété du montant net social), '
            + 'utilisable jusqu’au 31 décembre 2026 en application de l’arrêté du 11 août 2025.'),
    'Document produit par Xenna Paie ' + (opt.versionLogiciel || '') + ' — simulateur de paie. '
    + (fpt
        ? 'Les informations relatives à la collectivité, à ses identifiants, au grade et à '
          + 'l’échelon de l’agent sont tirées au sort : le simulateur ne les connaît pas.'
        : 'Les informations relatives à l’employeur, à ses identifiants, à la convention '
          + 'collective et à la classification du salarié sont tirées au sort : le simulateur '
          + 'ne les connaît pas.'),
  ];

  // ── Annexe : le détail que le modèle réglementaire regroupe ───────────────
  const annexe = {
    titre: 'ANNEXE — DÉTAIL DES COTISATIONS ET CONTRIBUTIONS',
    chapeau: 'Le modèle réglementaire regroupe les cotisations par risque couvert. Cette annexe '
      + 'les redonne ligne à ligne, telles que le moteur de calcul les produit, avec leur code '
      + 'interne et la référence du texte qui les fonde. Elle ne fait pas partie du bulletin.',
    colonnes: ['Cotisation', 'Base', 'Taux', 'Montant', 'Taux', 'Montant'],
    groupes: [
      { titre: 'PART SALARIÉ',   de: 2, a: 3 },
      { titre: 'PART EMPLOYEUR', de: 4, a: 5 },
    ],
    lignes: cots.map(c => ({
      libelle: c.libelle || c.code,
      code: c.code || '',
      base: _eurOuRien(c.base),
      taux_sal: _pct(c.taux_sal),
      montant_sal: _eurOuRien(c.montant_sal),
      taux_pat: _pct(c.taux_pat),
      montant_pat: _eurOuRien(c.montant_pat),
      reference: c.loi_ref || '',
    })),
  };

  // ── En-têtes d'identité ───────────────────────────────────────────────────
  const nom = (b.salarie?.nom || id.sal_nom || '').trim();
  const prenom = (b.salarie?.prenom || id.sal_prenoms || '').trim();
  const cadre = b.salarie?.statut === 'cadre';

  const employeur = fpt ? [
    { l: 'Collectivité', v: id.fpt_collectivite || '' },
    { l: 'Adresse',      v: id.emp_adresse_etab || '' },
    { l: 'SIRET',        v: id.emp_siret || '' },
    { l: 'Code APE',     v: '84.11Z' },
    { l: 'Statut',       v: 'Fonction publique territoriale — agent titulaire' },
    { l: 'Retraite',     v: 'CNRACL' },
  ] : [
    { l: 'Raison sociale', v: id.emp_raison_sociale || '' },
    { l: 'Adresse',        v: id.emp_adresse_etab || id.emp_adresse_siege || '' },
    { l: 'SIRET',          v: id.emp_siret || '' },
    { l: 'Code APE / NAF', v: id.emp_ape || '' },
    { l: 'URSSAF',         v: `${id.emp_urssaf || ''} — ${id.emp_urssaf_num || ''}`.trim() },
    { l: 'Convention collective', v: id.emp_ccn ? `${id.emp_ccn} (IDCC ${id.emp_idcc || '—'})` : '' },
    { l: 'Retraite compl.', v: id.emp_retraite || '' },
  ];

  const salarie = fpt ? [
    { l: 'Nom et prénom', v: `${nom} ${prenom}`.trim() },
    { l: 'Matricule',     v: id.sal_matricule || '' },
    { l: 'Grade',         v: id.fpt_grade || '' },
    { l: 'Échelon',       v: id.fpt_echelon || '' },
    { l: 'Emploi',        v: id.pos_intitule || '' },
    { l: 'Quotité',       v: `${etp} %` },
  ] : [
    { l: 'Nom et prénom', v: `${nom} ${prenom}`.trim() },
    { l: 'Matricule',     v: id.sal_matricule || '' },
    { l: 'Emploi',        v: id.pos_intitule || '' },
    { l: 'Classification', v: [
        cadre ? 'Cadre' : (id.pos_statut && id.pos_statut !== 'Cadre' ? id.pos_statut : 'Non-cadre'),
        id.pos_niveau ? `niveau ${id.pos_niveau}` : '',
        id.pos_coefficient ? `coefficient ${id.pos_coefficient}` : '',
      ].filter(Boolean).join(' — ') },
    { l: 'Entrée',        v: _dateFr(id.ctr_date_effet || '') },
    { l: 'Quotité',       v: `${etp} % — ${quotite.toLocaleString('fr-FR', { minimumFractionDigits: 2, maximumFractionDigits: 2 })} h` },
  ];

  const periode = [
    { l: 'Période d’emploi', v: `du ${_jj(_debutDeMois(datePaie))} au ${_jj(_finDeMois(datePaie))} ${_moisFr(datePaie)}` },
    { l: 'Date de paiement', v: _dateFr(datePaie) },
    { l: 'Mode de paiement', v: 'Virement' },
  ];

  return {
    titre: 'BULLETIN DE PAIE',
    sous_titre: fpt
      ? 'Fonction publique territoriale — agent titulaire'
      : `Modèle ${modele === 'renove' ? 'rénové' : 'adapté'} — art. R. 3243-2 du code du travail`,
    filigrane: 'SPÉCIMEN',
    avertissement:
      'SPÉCIMEN — sortie d’un simulateur de paie, sans valeur de bulletin de paie. '
      + 'L’employeur et ses identifiants sont fictifs.',
    employeur,
    salarie,
    periode,
    // Disposition commune des bulletins (Sage, Cegid, Silae…) : la part salarié
    // sépare ce qui s'ajoute (à payer) de ce qui se retranche (à déduire) ; la
    // part employeur n'a qu'un montant, elle ne touche pas au net.
    colonnes: ['Désignation', 'Nombre', 'Base', 'Taux', 'À payer', 'À déduire', 'Taux', 'Montant'],
    groupes: [
      { titre: 'PART SALARIÉ',   de: 3, a: 5 },
      { titre: 'PART EMPLOYEUR', de: 6, a: 7 },
    ],
    rubriques,
    totaux,
    cumuls,
    mentions,
    annexe,
    pied: (fpt ? id.fpt_collectivite : id.emp_raison_sociale) || 'Xenna Paie',
  };
}

/**
 * Une ligne de cotisation du moteur, mise au format de la grille. La part
 * salariale va toujours dans « à déduire » : une réduction (heures supp) y
 * figure en négatif, comme l'allègement dans la part employeur — le total des
 * cotisations reste ainsi la somme de sa colonne.
 */
function ligneDe(c, libelle) {
  return {
    libelle: libelle || c.libelle || c.code,
    base: _eurOuRien(c.base),
    taux_sal: _pct(c.taux_sal),
    a_deduire: _eurOuRien(c.montant_sal),
    taux_pat: _pct(c.taux_pat),
    montant_pat: _eurOuRien(c.montant_pat),
  };
}

/**
 * Plusieurs lignes du moteur ramenées à une seule ligne du modèle : les taux et
 * les montants s'additionnent, l'assiette est celle qu'elles partagent. Si elles
 * ne la partagent PAS, on ne fusionne rien — additionner des taux portant sur
 * des assiettes différentes produirait un chiffre qui ne veut rien dire.
 */
function ligneFusionnee(lignes, libelle) {
  const memeBase = lignes.every(c => Math.abs(_n(c.base) - _n(lignes[0].base)) < 0.005);
  if (!memeBase) {
    return ligneDe(lignes[0], libelle);
  }
  const somme = champ => lignes.reduce((s, c) => s + _n(c[champ]), 0);
  return {
    libelle,
    base: _eurOuRien(lignes[0].base),
    taux_sal: _pct(somme('taux_sal')),
    a_deduire: _eurOuRien(somme('montant_sal')),
    taux_pat: _pct(somme('taux_pat')),
    montant_pat: _eurOuRien(somme('montant_pat')),
  };
}

/** Nom de fichier : lisible, trié par date, sans accent ni espace. */
export function nomFichierBulletin(b, datePaie) {
  const parts = ['bulletin', (datePaie || '').slice(0, 7),
                 b.salarie?.nom || '', b.salarie?.prenom || ''];
  return parts.filter(Boolean).join('_')
    .normalize('NFD').replace(/[\u0300-\u036f]/g, '')
    .replace(/[^A-Za-z0-9_-]+/g, '_').replace(/_+/g, '_') + '.pdf';
}
