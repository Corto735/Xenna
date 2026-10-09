// ── Glossaire du bulletin ────────────────────────────────────────────────────
// Le vocabulaire de la paie est un mur pour qui lit son premier bulletin :
// assiette, tranche, plafond, subrogation… Chaque terme reçoit ici une
// définition courte, en clair et sans chiffre (un chiffre se périme ; le
// bulletin et ses f(x) donnent ceux du mois). Après le rendu, `poserGlossaire`
// repère la première occurrence de chaque terme dans une zone et l'enveloppe
// d'un <dfn> : bulle au survol, ou au toucher (focus) sur mobile.
//
// Français seulement : en traduction, découper les nœuds de texte casserait la
// correspondance exacte des dictionnaires statiques de lang.js.

// Ordre = priorité : une expression longue avant ses fragments (« CSG non
// déductible » avant « CSG »), sinon le terme court la masquerait.
export const GLOSSAIRE = [
  { cle: 'net_social', terme: 'Montant net social', motif: /montant net social/i,
    def: "Salaire brut moins toutes les cotisations et contributions sociales obligatoires du salarié. C'est le montant à déclarer pour la prime d'activité et le RSA." },
  { cle: 'net_imposable', terme: 'Net imposable', motif: /net imposable/i,
    def: "Revenu soumis à l'impôt : le net, plus la CSG non déductible et la CRDS, qui sont retenues sur le salaire mais restent imposables. C'est la base du prélèvement à la source." },
  { cle: 'net_avant_impot', terme: 'Net à payer avant impôt', motif: /net à payer avant impôt/i,
    def: "Ce que vous recevriez sans prélèvement à la source : le brut moins les cotisations, plus ou moins les éléments versés ou retenus hors cotisations." },
  { cle: 'pas', terme: 'Prélèvement à la source (PAS)', motif: /[Pp]rélèvement à la source|\bPAS\b/,
    def: "Impôt sur le revenu retenu par l'employeur et reversé à l'administration fiscale. Taux personnalisé transmis par l'administration ou, à défaut, taux de la grille par défaut, appliqué à tout le net imposable." },
  { cle: 'cout_employeur', terme: 'Coût total employeur', motif: /coût total employeur/i,
    def: "Ce que le salarié coûte à l'employeur : le brut, plus les cotisations patronales, moins les allègements. On dit parfois « super brut »." },
  { cle: 'assiette', terme: 'Assiette', motif: /\bassiettes?\b/i,
    def: "Montant sur lequel on applique un taux de cotisation. Souvent le brut, parfois une partie seulement (plafonnée, par tranche) ou le brut après abattement (CSG, CRDS)." },
  { cle: 'base', terme: 'Base', motif: /\bBASE\b/,
    def: "Dans le tableau des cotisations : l'assiette, c'est-à-dire le montant auquel s'applique le taux de la ligne." },
  { cle: 'pmss', terme: 'PMSS', motif: /\bPMSS\b|plafond mensuel de la sécurité sociale/i,
    def: "Plafond mensuel de la Sécurité sociale. Revalorisé chaque année, il borne les cotisations « plafonnées » et sépare la tranche 1 de la tranche 2 de la retraite complémentaire." },
  { cle: 'pass', terme: 'PASS', motif: /\bPASS\b/,
    def: "Plafond annuel de la Sécurité sociale : douze fois le plafond mensuel. Il sert de seuil à de nombreuses exonérations et limites." },
  { cle: 'deplafonnee', terme: 'Déplafonnée', motif: /\bdéplafonnée\b/i,
    def: "Cotisation calculée sur la totalité du salaire brut, sans limite." },
  { cle: 'plafonnee', terme: 'Plafonnée', motif: /\bplafonnée\b/i,
    def: "Cotisation calculée sur le salaire dans la limite du plafond de la Sécurité sociale : au-delà, rien n'est cotisé sur cette ligne." },
  { cle: 'tranche1', terme: 'Tranche 1', motif: /\btranche 1\b/i,
    def: "Part du salaire jusqu'au plafond de la Sécurité sociale. La retraite complémentaire y applique un taux, et un autre au-delà." },
  { cle: 'tranche2', terme: 'Tranche 2', motif: /\btranche 2\b/i,
    def: "Part du salaire comprise entre une et huit fois le plafond de la Sécurité sociale, cotisée à la retraite complémentaire à un taux plus élevé que la tranche 1." },
  { cle: 'trimestre', terme: 'Trimestre', motif: /\btrimestres?\b/i,
    def: "Unité de durée d'assurance retraite. On en valide au plus 4 par an, selon le salaire de l'année ; leur nombre décide de l'âge du taux plein." },
  { cle: 'points', terme: 'Points de retraite', motif: /\bpoints\b/i,
    def: "La retraite complémentaire se compte en points : les cotisations achètent des points chaque année ; à la retraite, leur total multiplié par la valeur du point donne la pension." },
  { cle: 'agirc', terme: 'Agirc-Arrco', motif: /agirc[- ]arrco/i,
    def: "Régime de retraite complémentaire obligatoire des salariés du privé. Les cotisations s'y convertissent en points, qui feront la pension complémentaire." },
  { cle: 'ceg', terme: "Contribution d'équilibre général (CEG)", motif: /contribution d'équilibre général|\bCEG\b/i,
    def: "Cotisation de retraite complémentaire qui finance l'équilibre du régime Agirc-Arrco. Elle n'attribue pas de points." },
  { cle: 'csg_nd', terme: 'CSG non déductible', motif: /csg non déductible|csg\/crds non déduct/i,
    def: "Part de la CSG qui reste dans votre revenu imposable : elle est retenue sur le salaire, mais vous payez l'impôt dessus." },
  { cle: 'csg_d', terme: 'CSG déductible', motif: /csg déductible|csg déduct\./i,
    def: "Part de la CSG retirée de votre revenu imposable : elle diminue la base de l'impôt sur le revenu." },
  { cle: 'crds', terme: 'CRDS', motif: /\bCRDS\b/,
    def: "Contribution au remboursement de la dette sociale. Retenue sur le salaire, elle reste imposable." },
  { cle: 'csg', terme: 'CSG', motif: /\bCSG\b/,
    def: "Contribution sociale généralisée : un impôt qui finance la Sécurité sociale, prélevé sur presque tous les revenus. Elle n'ouvre pas de droits." },
  { cle: 'atmp', terme: 'AT/MP', motif: /\bAT\/MP\b|accidents du travail/i,
    def: "Accidents du travail et maladies professionnelles. Cotisation à la charge de l'employeur seul, dont le taux dépend des risques de l'entreprise." },
  { cle: 'ags', terme: 'AGS', motif: /\bAGS\b/,
    def: "Assurance de garantie des salaires : elle paie les salaires dus si l'employeur fait faillite. À la charge de l'employeur." },
  { cle: 'reduction_generale', terme: 'Réduction générale', motif: /réduction générale/i,
    def: "Allègement de cotisations patronales sur les salaires modestes. Plus le salaire est proche du SMIC, plus l'allègement est fort. Il ne change rien au net du salarié." },
  { cle: 'allegement', terme: 'Allègement', motif: /\ballègements?\b/i,
    def: "Réduction de cotisations accordée par la loi, le plus souvent à l'employeur. Elle diminue le coût du travail sans changer le net." },
  { cle: 'subrogation', terme: 'Subrogation', motif: /\bsubrogation\b/i,
    def: "En arrêt de travail, l'employeur continue de vous payer et perçoit à votre place les indemnités journalières de la Sécurité sociale." },
  { cle: 'ijss', terme: 'IJSS', motif: /\bIJSS\b|indemnités journalières/i,
    def: "Indemnités journalières de la Sécurité sociale : ce qu'elle verse pendant un arrêt de travail pour compenser la perte de salaire." },
  { cle: 'carence', terme: 'Carence', motif: /\bcarence\b/i,
    def: "Premiers jours d'arrêt non indemnisés. La Sécurité sociale et l'employeur ont chacun leur délai." },
  { cle: 'maintien', terme: 'Maintien de salaire', motif: /maintien de salaire/i,
    def: "Complément versé par l'employeur pendant un arrêt, en plus des indemnités journalières, selon la loi ou la convention collective et l'ancienneté." },
  { cle: 'avantages_nature', terme: 'Avantages en nature', motif: /avantages? en nature/i,
    def: "Biens ou services fournis par l'employeur (logement, véhicule, repas…). Ils sont ajoutés au brut pour être cotisés et imposés, puis retirés du versement puisqu'ils sont déjà fournis." },
  { cle: 'patronale', terme: 'Part patronale', motif: /part patronale|charges patronales/i,
    def: "Cotisations payées par l'employeur en plus du salaire brut. Elles n'apparaissent pas dans votre net mais financent vos droits." },
  { cle: 'salariale', terme: 'Part salarié', motif: /part salarié|cotisations salariales/i,
    def: "Cotisations retenues sur votre salaire brut. C'est la différence entre le brut et le net social." },
];

const PAR_CLE = new Map(GLOSSAIRE.map(g => [g.cle, g]));

// Pas de bulle dans les contrôles, les montants cliquables ni dans une bulle déjà posée.
const EXCLUS = 'button, input, select, textarea, a, dfn, label, script, style, pre, .fm-val, .sb-val, .mob-val, .cc-val, .formula-star, .vc-bascule, .glo-liste';

const _echap = s => String(s).replace(/[&<>"]/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' }[c]));

/**
 * Enveloppe la première occurrence de chaque terme trouvée dans `racine`.
 * Idempotent : un terme déjà posé dans la zone n'est pas reposé.
 */
export function poserGlossaire(racine) {
  if (!racine) return;
  const dejaPoses = new Set([...racine.querySelectorAll('dfn.glo')].map(d => d.dataset.glo));
  const walker = document.createTreeWalker(racine, NodeFilter.SHOW_TEXT, {
    acceptNode: n => (n.textContent.trim().length > 2 && !n.parentElement?.closest(EXCLUS))
      ? NodeFilter.FILTER_ACCEPT : NodeFilter.FILTER_REJECT,
  });
  const noeuds = [];
  while (walker.nextNode()) noeuds.push(walker.currentNode);

  for (const g of GLOSSAIRE) {
    if (dejaPoses.has(g.cle)) continue;
    for (let i = 0; i < noeuds.length; i++) {
      const n = noeuds[i];
      if (!n.isConnected) continue;
      const m = g.motif.exec(n.textContent);
      if (!m) continue;
      // Découpe : avant | terme | après. Le reste du nœud reste cherchable.
      const apres = n.splitText(m.index);
      const reste = apres.splitText(m[0].length);
      const dfn = document.createElement('dfn');
      dfn.className = 'glo';
      dfn.dataset.glo = g.cle;
      dfn.tabIndex = 0;
      dfn.setAttribute('role', 'button');
      dfn.setAttribute('aria-label', `${m[0]} : ${g.def}`);
      dfn.textContent = m[0];
      apres.replaceWith(dfn);
      noeuds.splice(i + 1, 0, reste);
      dejaPoses.add(g.cle);
      break;
    }
  }
}

// Une seule bulle pour toute la page, positionnée sous le terme visé : une
// bulle en CSS pur déborderait des tableaux à défilement horizontal.
let _bulle = null;
function _bulleEl() {
  if (!_bulle) {
    _bulle = document.createElement('div');
    _bulle.className = 'glo-bulle';
    _bulle.setAttribute('role', 'tooltip');
    _bulle.hidden = true;
    document.body.appendChild(_bulle);
  }
  return _bulle;
}

let _cible = null;   // terme dont la bulle est ouverte

function _montrer(dfn) {
  const g = PAR_CLE.get(dfn.dataset.glo);
  _cible = dfn;
  if (!g) return;
  const b = _bulleEl();
  b.innerHTML = `<strong>${_echap(g.terme)}</strong> — ${_echap(g.def)}`;
  b.hidden = false;
  const r = dfn.getBoundingClientRect();
  const larg = Math.min(320, window.innerWidth - 24);
  b.style.width = larg + 'px';
  b.style.left = Math.max(12, Math.min(r.left, window.innerWidth - larg - 12)) + window.scrollX + 'px';
  const enBas = r.bottom + 8 + b.offsetHeight < window.innerHeight;
  b.style.top = (enBas ? r.bottom + 6 : r.top - b.offsetHeight - 6) + window.scrollY + 'px';
}

function _cacher() { _cible = null; if (_bulle) _bulle.hidden = true; }

let _ecoute = false;
/** Branche les écouteurs une fois pour toutes (délégation sur le document). */
export function activerGlossaire() {
  if (_ecoute) return;
  _ecoute = true;
  document.addEventListener('mouseover', e => { const d = e.target.closest?.('dfn.glo'); if (d) _montrer(d); });
  document.addEventListener('mouseout', e => { if (e.target.closest?.('dfn.glo')) _cacher(); });
  document.addEventListener('focusin', e => { const d = e.target.closest?.('dfn.glo'); if (d) _montrer(d); else _cacher(); });
  // Toucher un terme ne doit pas déplier la ligne qui le contient.
  document.addEventListener('click', e => {
    const d = e.target.closest?.('dfn.glo');
    if (d) { e.stopPropagation(); _montrer(d); } else _cacher();
  }, true);
  document.addEventListener('keydown', e => { if (e.key === 'Escape') _cacher(); });
  // Le focus clavier fait défiler la page : la bulle suit son terme au lieu
  // de disparaître ; elle se ferme si le terme a quitté le document.
  window.addEventListener('scroll', () => {
    if (!_cible) return;
    if (_cible.isConnected) _montrer(_cible); else _cacher();
  }, { passive: true });
}

/** Liste complète, pour la page « À propos ». */
export function glossaireHtml() {
  return `<dl class="glo-liste">${[...GLOSSAIRE]
    .sort((a, b) => a.terme.localeCompare(b.terme, 'fr'))
    .map(g => `<div><dt>${_echap(g.terme)}</dt><dd>${_echap(g.def)}</dd></div>`).join('')}</dl>`;
}
