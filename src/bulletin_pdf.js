// ═════════════════════════════════════════════════════════════════════════════
// BULLETIN DE PAIE — composition du PDF
// ═════════════════════════════════════════════════════════════════════════════
//
// Ce module traduit un bulletin déjà calculé par le back Rust en un DOCUMENT,
// présenté comme un bulletin de logiciel de paie du marché (disposition relevée
// sur un bulletin réel) : en-tête employeur et salarié, cadre d'adresse, grille
// « Éléments de paie · Base · Taux · À déduire · À payer · Charges patronales »,
// pied fixe des cumuls, des congés et du net payé. Le Rust reçoit ce document
// tout composé et n'en fait que la mise en page.
//
// ── Pourquoi côté front ? ────────────────────────────────────────────────────
// Même raison que pour la DSN (cf. dsn.js) : ce n'est pas un calcul, c'est une
// TRADUCTION d'un bulletin déjà produit. Deux données nécessaires ne vivent que
// côté front — le prélèvement à la source (calculerPas) et la date du
// formulaire.
//
// ── Les deux modèles réglementaires ──────────────────────────────────────────
// L'arrêté du 25 février 2016 fixe les libellés, l'ordre et le regroupement des
// informations du bulletin ; l'arrêté du 31 janvier 2023 institue un modèle
// RÉNOVÉ, obligatoire au 1er janvier 2027 (report par l'arrêté du 11 août 2025).
// La grille suit le regroupement par risque commun aux deux ; la date de bascule
// reste déclarée ici, une seule fois, et l'annexe dit lequel s'applique.
//
// ── Ce que ce document N'EST PAS ─────────────────────────────────────────────
// Ce n'est pas un bulletin de paie. C'est la sortie d'un simulateur, imprimée
// pour qu'on puisse la lire. L'employeur, ses identifiants, l'adresse du salarié
// et sa classification sont TIRÉS AU SORT — le simulateur ne les connaît pas.
// D'où le filigrane SPÉCIMEN. Ce qu'un mois isolé ne peut pas savoir (cumuls
// annuels, compteur de congés N-1, taux de PAS personnalisé) reste vide, et
// l'annexe le dit.
// ═════════════════════════════════════════════════════════════════════════════

/** Date d'entrée en vigueur obligatoire du modèle rénové (arrêté du 11/08/2025). */
export const MODELE_BASCULE = '2027-01-01';

/** 'adapte' jusqu'au 31/12/2026, 'renove' à partir du 01/01/2027. */
export function modeleApplicable(dateIso) {
  return (dateIso || '') >= MODELE_BASCULE ? 'renove' : 'adapte';
}

// ── Regroupement des cotisations ─────────────────────────────────────────────
//
// Chaque groupe porte son titre et ses postes ; chaque poste, son libellé et les
// codes de cotisation du moteur qui l'alimentent. `seul` : groupe d'un seul
// poste, imprimé sur une seule ligne en gras (« Famille », « Assurance
// chômage ») — c'est ainsi que les logiciels de paie l'affichent. `fusion` :
// plusieurs lignes du moteur ramenées à une seule (même assiette, taux et
// montants additionnés), comme la CEG avec la tranche 1 de l'Agirc-Arrco.

const GROUPES = [
  { titre: 'Santé', postes: [
    { lbl: 'Sécurité Sociale - Mal. Mat. Inval. Décès',
      codes: ['SS_MALADIE', 'ALSACE_MOSELLE_MALADIE'], fusion: true },
    { lbl: 'Complémentaire - Incap. Inval. Décès', codes: ['PREVOYANCE_CADRE_MIN'] },
  ] },
  { titre: 'Accidents du travail & mal. professionnelles', seul: true, postes: [
    { codes: ['AT_MP'] },
  ] },
  { titre: 'Retraite', postes: [
    { lbl: 'Sécurité Sociale plafonnée',   codes: ['SS_VIEILLESSE_PLAF'] },
    { lbl: 'Sécurité Sociale déplafonnée', codes: ['SS_VIEILLESSE_DEPLAF'] },
    { lbl: 'Complémentaire Tranche 1', codes: ['AGIRC_ARRCO_T1', 'AGIRC_ARRCO_CEG_T1'], fusion: true },
    { lbl: 'Complémentaire Tranche 2', codes: ['AGIRC_ARRCO_T2', 'AGIRC_ARRCO_CEG_T2'], fusion: true },
    { lbl: 'Retraite (CNRACL)', codes: ['FPT_CNRACL'] },
  ] },
  { titre: 'Famille', seul: true, postes: [{ codes: ['FAMILLE'] }] },
  { titre: 'Assurance chômage', seul: true, postes: [
    { codes: ['CHOMAGE', 'AGS'], fusion: true },
  ] },
  // Les cotisations que les groupes ci-dessus ne connaissent pas atterrissent
  // ici (cf. `autres`) plutôt que de disparaître du document.
  { titre: "Autres contributions dues par l'employeur", autres: true, postes: [] },
  { titre: "CSG déduct. de l'impôt sur le revenu", seul: true, postes: [
    { codes: ['CSG_DEDUCTIBLE'] },
  ] },
  { titre: "CSG/CRDS non déduct. de l'impôt sur le revenu", seul: true, postes: [
    { codes: ['CSG_NON_DEDUCTIBLE', 'CRDS'], fusion: true },
  ] },
];

/** Allègements patronaux : une seule ligne, montant en charges patronales. */
const EXO_PATRONALES = ['REDUCTION_FILLON', 'DFP_HS'];
/** Réduction salariale des heures supp. : une seule ligne, en « à déduire ». */
const EXO_SALARIALES = ['REDUC_SAL_HS'];
/** Aides à l'emploi : chacune sur sa ligne, sous son propre libellé. */
const AIDES = ['AIDE_POSTE_EA', 'ESAT_AIDE_POSTE', 'ESAT_COMPENSATION'];

// ── Mise en forme ────────────────────────────────────────────────────────────
// Le back ne formate rien. Présentation des logiciels de paie : point décimal,
// espace entre les milliers, signe « - » détaché, taux en pourcentage à quatre
// décimales sans le signe %.

const _n = v => {
  const x = parseFloat(v);
  return Number.isFinite(x) ? x : 0;
};

const _num = (v, dec = 2) => {
  const x = _n(v);
  const [ent, frac] = Math.abs(x).toFixed(dec).split('.');
  const groupe = ent.replace(/\B(?=(\d{3})+(?!\d))/g, ' ');
  const signe = x < 0 && Math.abs(x) >= 0.5 * 10 ** -dec ? '- ' : '';
  return signe + groupe + (frac ? '.' + frac : '');
};

/** Montant, ou chaîne vide si nul — une colonne vide vaut mieux qu'un « 0.00 ». */
const _m = v => (Math.abs(_n(v)) < 0.005 ? '' : _num(v));
/** Taux exprimé en fraction (0,069) → « 6.9000 ». */
const _t = v => (Math.abs(_n(v)) < 1e-9 ? '' : _num(_n(v) * 100, 4));
/** Taux horaire : quatre décimales, sinon « nombre × base » ne retombe pas sur le montant. */
const _th = v => _num(v, 4);

const MOIS = ['Janvier', 'Février', 'Mars', 'Avril', 'Mai', 'Juin',
  'Juillet', 'Août', 'Septembre', 'Octobre', 'Novembre', 'Décembre'];

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
/** Une adresse tirée au sort s'écrit « 14, allée … — 92130 Ville » : deux lignes. */
const _lignesAdresse = a => (a || '').split(' — ').map(s => s.trim()).filter(Boolean);

/**
 * Date d'entrée cohérente avec l'ancienneté saisie dans les Paramètres : le
 * jour et le mois de la date tirée au sort, l'année qui donne `anc` années
 * révolues à la date de paie. Rend aussi l'ancienneté « x ans et y mois ».
 */
function _entree(datePaie, dateTiree, anc) {
  const [py, pm, pd] = (datePaie || '').split('-').map(Number);
  if (!py) return { entree: dateTiree || '', anciennete: '' };
  let [, m, d] = (dateTiree || `${py}-01-01`).split('-').map(Number);
  if (!Number.isFinite(anc)) {
    const [ty] = (dateTiree || '').split('-').map(Number);
    anc = Math.max(0, py - (ty || py));
  }
  let y = py - anc;
  // Jour/mois postérieurs à la date de paie : l'anniversaire n'est pas encore
  // passé cette année, l'entrée remonte d'un an de plus.
  if (m > pm || (m === pm && d > pd)) y -= 1;
  const mois = (py - y) * 12 + (pm - m) - (pd < d ? 1 : 0);
  const ans = Math.floor(mois / 12), reste = mois % 12;
  const iso = `${y}-${String(m).padStart(2, '0')}-${String(d).padStart(2, '0')}`;
  const txt = `${ans} an${ans > 1 ? 's' : ''}` + (reste ? ` et ${reste} mois` : '');
  return { entree: iso, anciennete: txt };
}

// ── Composition ──────────────────────────────────────────────────────────────

/**
 * Fabrique le document envoyé au moteur Rust.
 *
 * @param {object} b   Bulletin France ou fonction publique, tel que rendu par le back.
 * @param {object} opt {
 *   datePaie, pas, identite, remBase, remLignes, etp, heuresMois, pmss,
 *   anciennete, ccn, versionLogiciel
 * }
 */
export function composerBulletinPdf(b, opt = {}) {
  const id       = opt.identite || {};
  const datePaie = opt.datePaie || '';
  const pas      = opt.pas || { total: 0, taux_effectif: 0 };
  const fpt      = b.salarie?.pays === 'fonction_publique';
  const cots     = Array.isArray(b.cotisations) ? b.cotisations : [];
  const etp      = Math.min(_n(opt.etp) || 100, 100);
  const quotite  = Math.round(151.67 * etp) / 100;
  const L = [];   // lignes de la grille
  const vide = () => L.push({ vide: true });

  // ── Éléments de rémunération ──────────────────────────────────────────────
  // Une ligne d'heures se lit « nombre × taux = à payer » ; une retenue tombe
  // dans « à déduire », la colonne disant le sens.
  const base = _n(opt.remBase);
  const hs = b.heures_sup;
  const tauxH = hs ? _n(hs.taux_horaire)
    : (quotite > 0 ? Math.round(base / quotite * 1e4) / 1e4 : 0);
  L.push({
    libelle: fpt ? 'Traitement indiciaire brut' : 'Salaire de base',
    base: _num(quotite), taux: tauxH ? _th(tauxH) : '', a_payer: _m(base),
  });

  for (const l of (opt.remLignes || [])) {
    // Seuls les éléments en euros vont ici : heures supp. détaillées plus bas,
    // avantages en nature depuis le bulletin, frais versés en net.
    if (l.type !== 'prime' && l.type !== 'coupure_50') continue;
    const m = _n(l.amount);
    if (m) L.push({ libelle: l.type === 'coupure_50' ? 'Majoration pour coupure (50 %)' : 'Prime', a_payer: _m(m) });
  }

  if (hs) {
    const th = _n(hs.taux_horaire);
    const heures = (h, maj, lbl) => {
      if (!h) return;
      L.push({
        libelle: lbl, base: _num(h), taux: _th(th * maj),
        // Arrondie au centime ligne par ligne, comme le moteur (heures_sup.rs).
        a_payer: _m(Math.round(h * th * maj * 100) / 100),
      });
    };
    heures(hs.h_struct_25, 1.25, 'Heures supplémentaires structurelles 25 %');
    heures(hs.h_struct_50, 1.50, 'Heures supplémentaires structurelles 50 %');
    heures(hs.h_supp_25, 1.25, 'Heures supplémentaires 25 %');
    heures(hs.h_supp_50, 1.50, 'Heures supplémentaires 50 %');
    heures(hs.h_comp_10, 1.10, 'Heures complémentaires 10 %');
    heures(hs.h_comp_25, 1.25, 'Heures complémentaires 25 %');
  }

  const abs = b.absence;
  if (abs) {
    const j = _n(abs.jours_absence);
    L.push({ libelle: `Absence ${abs.libelle || 'arrêt de travail'}${j ? ` (${j} jours)` : ''}`,
             base: j ? _num(-j) : '', a_deduire: _m(abs.retenue) });
    if (_n(abs.maintien) > 0) {
      L.push({ libelle: `Maintien de salaire (${abs.convention || 'régime légal'})`, a_payer: _m(abs.maintien) });
    }
    if (_n(abs.ijss_brut) > 0) {
      L.push({ libelle: 'Indemnités journalières brutes (subrogation)', a_deduire: _m(abs.ijss_brut) });
    }
    if (_n(abs.ajustement_net) > 0) {
      L.push({ libelle: 'Ajustement garantie du net', a_deduire: _m(abs.ajustement_net) });
    }
  }

  const cp = b.conges;
  if (cp) {
    const j = _n(cp.jours_pris);
    L.push({ libelle: `Congés payés pris (${j} jours)`, base: j ? _num(-j) : '',
             taux: j ? _th(_n(cp.retenue) / j) : '', a_deduire: _m(cp.retenue) });
    L.push({ libelle: `Indemnité congés payés (${j} jours)`, a_payer: _m(cp.indemnite) });
  }

  // Avantages en nature : ajoutés au brut ici, retenus sur le net plus bas.
  const avantages = b.salarie?.pays === 'france' ? (b.avantages_nature || []) : [];
  for (const a of avantages) L.push({ libelle: a.libelle, a_payer: _m(a.montant) });

  // Comme sur le modèle : une ligne blanche sépare le salaire de base des
  // éléments qui le suivent, quand il y en a.
  if (L.length > 1) L.splice(1, 0, { vide: true });
  L.push({ libelle: fpt ? 'Rémunération brute' : 'Salaire brut', a_payer: _m(b.brut), fort: true });
  vide();

  // ── Cotisations et contributions ──────────────────────────────────────────
  const utilises = new Set([...EXO_PATRONALES, ...EXO_SALARIALES, ...AIDES]);
  GROUPES.forEach(g => g.postes.forEach(p => p.codes.forEach(c => utilises.add(c))));
  const orphelines = cots.filter(c => !utilises.has(c.code));

  for (const g of GROUPES) {
    const lignes = [];
    if (g.autres) {
      orphelines.forEach(c => lignes.push(ligneDe([c], c.libelle || c.code)));
    }
    for (const p of g.postes) {
      const trouvees = cots.filter(c => p.codes.includes(c.code));
      if (!trouvees.length) continue;
      if (p.fusion) {
        lignesFusionnees(trouvees, p.lbl || g.titre).forEach(l => lignes.push(l));
      } else {
        trouvees.forEach(c => lignes.push(ligneDe([c], trouvees.length > 1 ? c.libelle : (p.lbl || g.titre))));
      }
    }
    if (!lignes.length) continue;
    if (g.seul && lignes.length === 1) {
      L.push({ ...lignes[0], libelle: g.titre, fort: true });
    } else {
      L.push({ libelle: g.titre, fort: true });
      lignes.forEach(l => L.push(l));
    }
  }

  const somme = (codes, champ) => cots.filter(c => codes.includes(c.code))
    .reduce((s, c) => s + _n(c[champ]), 0);
  const exoPat = somme(EXO_PATRONALES, 'montant_pat');
  if (Math.abs(exoPat) >= 0.005) {
    L.push({ libelle: 'Exonérations de cotisations employeur', montant_pat: _m(exoPat), fort: true });
  }
  const exoSal = somme(EXO_SALARIALES, 'montant_sal');
  if (Math.abs(exoSal) >= 0.005) {
    L.push({ libelle: 'Exonérations de cotisations salariales', a_deduire: _m(exoSal), fort: true });
  }
  cots.filter(c => AIDES.includes(c.code))
    .forEach(c => L.push({ ...ligneDe([c], c.libelle || c.code), fort: true }));
  vide();

  const totalSal = cots.reduce((s, c) => s + _n(c.montant_sal), 0);
  const totalPat = cots.reduce((s, c) => s + _n(c.montant_pat), 0);
  L.push({ libelle: 'Total des cotisations et contributions',
           a_deduire: _m(totalSal), montant_pat: _m(totalPat), fort: true });
  vide();

  // ── Du net social au net payé ─────────────────────────────────────────────
  // Montant net social : brut diminué des seules cotisations et contributions
  // sociales obligatoires (le simulateur n'en calcule pas d'autres).
  const netSocial = _n(b.brut) - totalSal;
  const netAvantImpot = _n(b.net_a_payer);
  const netImposable = _n(b.net_imposable);
  const pasTotal = _n(pas.total);
  const netPaye = netAvantImpot - pasTotal;

  if (hs && _n(hs.exo_fiscale) > 0) {
    L.push({ libelle: 'Exonération sur HC/HS : net fiscal du mois', base: _m(hs.exo_fiscale) });
  }
  L.push({ libelle: 'Montant net social', base: _num(netSocial) });

  // Éléments versés ou retenus après le net social, déjà compris dans le net
  // à payer calculé par le moteur.
  const ijssNet = abs ? _n(abs.ijss_net) : 0;
  const totalAvantages = avantages.reduce((s, a) => s + _n(a.montant), 0);
  const frais = b.salarie?.pays === 'france' ? (b.frais_professionnels || []) : [];
  const apres = [];
  if (ijssNet > 0) apres.push({ libelle: 'Indemnités journalières nettes (subrogation)', a_payer: _m(ijssNet) });
  if (totalAvantages > 0) apres.push({ libelle: 'Avantages en nature (retenue)', a_deduire: _m(totalAvantages) });
  for (const f of frais) {
    apres.push({ libelle: f.libelle, base: _num(f.nombre),
                 taux: f.montant_unitaire == null ? '' : _num(f.montant_unitaire), a_payer: _m(f.montant) });
  }
  if (apres.length) { vide(); apres.forEach(l => L.push(l)); }
  vide();

  L.push({ libelle: 'Net à payer avant impôt sur le revenu', a_payer: _num(netAvantImpot), grand: true });
  L.push({ libelle: 'Impôt sur le revenu prélevé à la source - PAS',
           base: _num(netImposable), taux: _num(_n(pas.taux_effectif) * 100, 4), a_deduire: _num(pasTotal) });
  L.push({ libelle: 'Taux neutre (barème DGFiP)', note: true, centre: true });
  L.push({ libelle: 'Net payé', a_payer: _num(netPaye), fort: true });
  vide();
  L.push({ libelle: 'Simulation Xenna Paie : employeur, identifiants et adresse fictifs. '
                  + 'Cumuls annuels et congés N-1 non simulés — détail en annexe.', note: true });

  // ── Pied : cumuls et congés ───────────────────────────────────────────────
  const heuresSupp = hs ? ['h_struct_25', 'h_struct_50', 'h_supp_25', 'h_supp_50', 'h_comp_10', 'h_comp_25']
    .reduce((s, k) => s + _n(hs[k]), 0) : 0;
  const totalFrais = frais.reduce((s, f) => s + _n(f.montant), 0);
  const coutGlobal = _n(b.brut) + totalPat;
  const allegements = -exoPat;
  const mensuel = [
    _num(quotite), heuresSupp ? _num(heuresSupp) : '', _num(b.brut),
    opt.pmss ? _num(opt.pmss) : '', _num(netImposable), _num(totalPat),
    _num(coutGlobal), _num(coutGlobal + totalFrais), allegements > 0.005 ? _num(allegements) : '',
  ];

  // Congés N : 2,5 jours ouvrables par mois depuis le 1er juin, début de la
  // période de référence légale (C. trav. art. L3141-3 et R3141-4). Le
  // compteur N-1 n'est pas connu d'un mois isolé : seuls les jours pris ce
  // mois-ci s'y inscrivent. La fonction publique relève d'un autre décompte.
  const moisPaie = parseInt((datePaie || '').split('-')[1], 10) || 0;
  const moisAcquis = moisPaie >= 6 ? moisPaie - 5 : moisPaie + 7;
  const acquisN = !fpt && moisPaie ? _num(2.5 * moisAcquis) : '';
  const prisN1 = cp ? _num(cp.jours_pris) : '';

  // ── En-tête ───────────────────────────────────────────────────────────────
  const nom = (b.salarie?.nom || id.sal_nom || '').trim();
  const prenom = (b.salarie?.prenom || id.sal_prenoms || '').trim();
  const cadre = b.salarie?.statut === 'cadre';
  const { entree, anciennete } = _entree(datePaie, id.ctr_date_effet,
    Number.isFinite(opt.anciennete) ? opt.anciennete : undefined);

  const blocs = fpt ? [
    [{ l: 'Matricule', v: id.sal_matricule || '' }],
    [{ l: 'Emploi', v: id.pos_intitule || '' },
     { l: 'Grade', v: id.fpt_grade || '' },
     { l: 'Echelon', v: id.fpt_echelon || '' }],
    [{ l: 'Entrée', v: _dateFr(entree) },
     { l: 'Ancienneté', v: `${anciennete}  ${_dateFr(entree)}`.trim() }],
  ] : [
    [{ l: 'Matricule', v: id.sal_matricule || '' }],
    [{ l: 'Emploi', v: id.pos_intitule || '' },
     { l: 'Statut', v: cadre ? 'Cadre' : 'Non-cadre' },
     { l: 'Echelon', v: id.pos_echelon || '' },
     { l: 'Niveau', v: id.pos_niveau || '' },
     { l: 'Coefficient', v: id.pos_coefficient || '' }],
    [{ l: 'Entrée', v: _dateFr(entree) },
     { l: 'Ancienneté', v: `${anciennete}  ${_dateFr(entree)}`.trim() }],
  ];

  const annexe = {
    titre: 'ANNEXE — DÉTAIL DES COTISATIONS ET CONTRIBUTIONS',
    chapeau: [
      'Le bulletin regroupe les cotisations par risque couvert. Cette annexe les redonne ligne à '
      + 'ligne, telles que le moteur de calcul les produit, avec leur code interne et la référence '
      + 'du texte qui les fonde. Elle ne fait pas partie du bulletin.',
      'Document produit par Xenna Paie ' + (opt.versionLogiciel || '') + ', simulateur de paie : ce '
      + 'n’est pas un bulletin de paie. L’employeur, ses identifiants, la classification et l’adresse '
      + 'du salarié sont tirés au sort ; la convention collective est celle appliquée au calcul.',
      'Cumuls annuels : un seul mois est simulé, le cumul depuis janvier n’est pas connu et la ligne '
      + '« Annuel » reste vide. Congés : le compteur N est estimé à 2,5 jours ouvrables par mois '
      + 'depuis le 1er juin ; le compteur N-1 n’est pas connu.',
      'Prélèvement à la source : taux neutre du barème mensuel de la DGFiP (personne seule). Le taux '
      + 'personnalisé transmis par l’administration n’est pas simulé.',
      fpt
        ? 'Fonction publique territoriale : le modèle réglementaire du bulletin (art. R. 3243-2 du code '
          + 'du travail) régit les salariés de droit privé ; sa présentation est reprise pour la seule '
          + 'commodité de la lecture.'
        : 'Modèle réglementaire applicable à cette date : ' + (modeleApplicable(datePaie) === 'renove'
            ? 'modèle rénové (arrêté du 31 janvier 2023), obligatoire à compter du 1er janvier 2027.'
            : 'modèle adapté (arrêté du 25 février 2016, complété du montant net social), utilisable '
              + 'jusqu’au 31 décembre 2026 (arrêté du 11 août 2025).')
        + ' Ne sont pas modélisés : FNAL, versement mobilité, contribution au dialogue social, '
        + 'formation professionnelle, taxe d’apprentissage, complémentaire santé, régimes facultatifs.',
    ],
    colonnes: ['Cotisation', 'Base', 'Taux', 'Montant', 'Taux', 'Montant'],
    groupes: [
      { titre: 'PART SALARIÉ',   de: 2, a: 3 },
      { titre: 'PART EMPLOYEUR', de: 4, a: 5 },
    ],
    lignes: cots.map(c => ({
      libelle: c.libelle || c.code,
      code: c.code || '',
      base: _m(c.base),
      taux_sal: _t(c.taux_sal),
      montant_sal: _m(c.montant_sal),
      taux_pat: _t(c.taux_pat),
      montant_pat: _m(c.montant_pat),
      reference: c.loi_ref || '',
    })),
  };

  return {
    titre: 'BULLETIN DE SALAIRE',
    periode: _moisFr(datePaie),
    reference: '',
    filigrane: 'SPÉCIMEN',
    employeur: (fpt ? id.fpt_collectivite : id.emp_raison_sociale) || 'Xenna Paie',
    employeur_adresse: _lignesAdresse(id.emp_adresse_etab || id.emp_adresse_siege),
    identifiants: fpt
      ? [[{ l: 'Siret', v: id.emp_siret || '' }, { l: 'Code Ape', v: '8411Z' }]]
      : [[{ l: 'Siret', v: (id.emp_siret || '').replace(/\s/g, '') },
          { l: 'Code Naf', v: (id.emp_ape || '').replace('.', '') }],
         [{ l: 'Urssaf/Msa', v: id.emp_urssaf_num || '' }]],
    blocs,
    convention: fpt
      ? { l: 'Statut', v: 'Fonction publique territoriale — agent titulaire' }
      : { l: 'Convention collective', v: opt.ccn || 'Néant — régime légal' },
    destinataire: [`${nom.toUpperCase()} ${prenom}`.trim(), ..._lignesAdresse(id.sal_adresse)],
    colonnes: ['Eléments de paie', 'Base', 'Taux', 'A déduire', 'A payer', 'Charges patronales'],
    lignes: L,
    pied_entetes: ['', 'Heures', 'Heures suppl.', 'Brut', 'Plafond S.S.', 'Net imposable',
                   'Ch. patronales', 'Coût Global', 'Total versé', 'Allègements'],
    pied_lignes: [
      { l: 'Mensuel', v: mensuel },
      { l: 'Annuel', v: [] },
    ],
    conges_entetes: ['', 'Congés N-1', 'Congés N'],
    conges_lignes: [
      { l: 'Acquis', v: ['', acquisN] },
      { l: 'Pris',   v: [prisN1, ''] },
      { l: 'Solde',  v: ['', ''] },
    ],
    net_paye: `Net payé : ${_num(netPaye)} euros`,
    paiement: `Paiement le ${_dateFr(_finDeMois(datePaie))} par Virement`,
    mention: 'Dans votre intérêt, et pour vous aider à faire valoir vos droits, conservez ce bulletin '
      + 'de paie sans limitation de durée. Informations complémentaires : www.service-public.fr',
    annexe,
  };
}

/**
 * Une ou plusieurs lignes du moteur sur une même ligne de grille. Base, taux et
 * montant s'impriment côté salarié et côté employeur pour la part qui existe :
 * la ligne « Famille » n'a pas de part salariale, et cette absence est une
 * information. Une réduction (allègement) figure en négatif.
 */
function ligneDe(lignes, libelle) {
  const somme = champ => lignes.reduce((s, c) => s + _n(c[champ]), 0);
  const base = lignes[0].base;
  const sal = Math.abs(somme('montant_sal')) >= 0.005;
  const pat = Math.abs(somme('montant_pat')) >= 0.005;
  return {
    libelle,
    base: sal ? _m(base) : '',
    taux: sal ? _t(somme('taux_sal')) : '',
    a_deduire: sal ? _m(somme('montant_sal')) : '',
    base_pat: pat ? _m(base) : '',
    taux_pat: pat ? _t(somme('taux_pat')) : '',
    montant_pat: pat ? _m(somme('montant_pat')) : '',
  };
}

/**
 * Plusieurs lignes du moteur ramenées à une seule : taux et montants
 * additionnés, sur l'assiette qu'elles partagent. Si elles ne la partagent PAS,
 * chaque ligne garde la sienne — additionner des taux portant sur des
 * assiettes différentes produirait un chiffre qui ne veut rien dire.
 */
function lignesFusionnees(lignes, libelle) {
  const memeBase = lignes.every(c => Math.abs(_n(c.base) - _n(lignes[0].base)) < 0.005);
  if (memeBase) return [ligneDe(lignes, libelle)];
  return lignes.map((c, i) => ligneDe([c], i === 0 ? libelle : (c.libelle || c.code)));
}

/** Nom de fichier : lisible, trié par date, sans accent ni espace. */
export function nomFichierBulletin(b, datePaie) {
  const parts = ['bulletin', (datePaie || '').slice(0, 7),
                 b.salarie?.nom || '', b.salarie?.prenom || ''];
  return parts.filter(Boolean).join('_')
    .normalize('NFD').replace(/[̀-ͯ]/g, '')
    .replace(/[^A-Za-z0-9_-]+/g, '_').replace(/_+/g, '_') + '.pdf';
}
