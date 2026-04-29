// ── Couche API : Tauri invoke en desktop, HTTP POST en web ───────────────────
//
// Architecture duale : la même UI tourne à la fois dans l'app Tauri (desktop)
// et dans le serveur Axum (Railway/web).
//
// Détection Tauri v1 vs v2 :
//   - Tauri v1 injectait window.__TAURI__
//   - Tauri v2 injecte window.__TAURI_INTERNALS__ (window.__TAURI__ n'existe plus)
//   → toujours tester __TAURI_INTERNALS__ en v2, sinon on tombe dans la branche
//     web et fetch() appelle Vite qui répond 404 → body vide → erreur muette "".
//
// Nommage des arguments : Tauri convertit snake_case Rust → camelCase JS.
//   ex. date_paie (Rust) → datePaie (JS)
//       salaire_brut (Rust) → salaireBrut (JS)
async function api(command, args = {}) {
  if (window.__TAURI_INTERNALS__) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke(command, args);
  }
  const r = await fetch(`/api/${command}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(args),
  });
  if (!r.ok) throw await r.text();
  return r.json();
}

// ── Sérialisation d'erreur ───────────────────────────────────────────────────
// Tauri v2 peut rejeter une Promise avec :
//   - une string  → erreur Rust propagée normalement (e.g. "Date invalide : …")
//   - une string vide ""  → panic Rust intercepté sans message (e.g. état non géré)
//   - un objet    → erreur interne Tauri (désérialisation d'args, commande inconnue…)
//   - null/undefined → cas rare, erreur silencieuse complète
// String(objet) donnerait "[object Object]" — on utilise JSON.stringify à la place.
function errToStr(e) {
  if (e === null || e === undefined) {
    return "(erreur nulle — redémarre l'app ou ouvre DevTools Ctrl+Shift+I)";
  }
  if (typeof e === "string") {
    // Chaîne vide = panic Rust sans message ou erreur interne Tauri muette
    return e || "(erreur muette — ouvre DevTools Ctrl+Shift+I et consulte la Console)";
  }
  if (e instanceof Error) {
    return e.message || e.toString();
  }
  // Objet Tauri interne : on sérialise en JSON pour voir la structure complète
  try { return JSON.stringify(e, null, 2); } catch { return String(e); }
}

// ── État global ──────────────────────────────────────────────────────────────
let lastBulletin = null;

// ── Initialisation date par défaut = aujourd'hui ─────────────────────────────
const TODAY = new Date().toISOString().slice(0, 10);
document.addEventListener("DOMContentLoaded", () => {
  ["d-date", "m-date"].forEach(id => {
    const el = document.getElementById(id);
    if (el) { el.value = TODAY; el.max = TODAY; }
  });
  document.addEventListener("keydown", e => { if (e.key === "Escape") closeFmModal(); });

  // Tirage unique à l'arrivée — la paire H/F et l'écart salarial sont fixés pour la session
  window._heroH = _heroRandom(HEROS_H);
  window._heroF = _heroRandom(HEROS_F);
  _setNomFields(window._heroH.prenom, window._heroH.nom);
  _syncToggleUI('H');

  // Quand l'utilisateur tape manuellement, le toggle est désactivé
  ['d-prenom', 'm-prenom', 'd-nom', 'm-nom'].forEach(id => {
    document.getElementById(id)?.addEventListener('input', () => { _nomPersonnalise = true; });
  });

  const _pctFH = Math.round(_tauxEcart * 100);
  const _pctHF = Math.round(_tauxEcart / (1 - _tauxEcart) * 100);
  document.querySelectorAll('.genre-ecart-hint').forEach(el => {
    el.dataset.textFh = `// −${_pctFH} % · écart salarial F/H`;
    el.dataset.textHf = `// +${_pctHF} % · écart salarial H/F`;
  });

  // Détection automatique mobile / bureau — breakpoint identique au media query CSS
  const mq = window.matchMedia("(max-width: 680px)");
  const applyView = e => {
    const body = document.body;
    if (!body.classList.contains("is-annuel")     &&
        !body.classList.contains("is-forge")      &&
        !body.classList.contains("is-apropos")    &&
        !body.classList.contains("is-gaabrielle") &&
        !body.classList.contains("is-hercule"))
      setView(e.matches ? "mobile" : "desktop");
  };
  mq.addEventListener("change", applyView);
  applyView(mq);

  // Restaure les préférences d'accessibilité
  if (localStorage.getItem('xenna-hv')) {
    document.body.classList.add('hv-mode');
    document.getElementById('hv-switch')?.classList.add('on');
  }
  if (localStorage.getItem('xenna-zoom')) {
    document.body.classList.add('zoom-mode');
    document.documentElement.style.zoom = '200%';
    document.getElementById('zoom-switch')?.classList.add('on');
    document.getElementById('a11y-magnifier')?.classList.add('active');
  }
  const savedFont = localStorage.getItem('xenna-font');
  if (savedFont) setAppFont(savedFont, true);

  if (localStorage.getItem('xenna-hv')) {
    document.getElementById('a11y-hv-btn')?.classList.add('active');
  }
  if (localStorage.getItem('xenna-bw')) {
    document.body.classList.add('bw-mode');
    document.getElementById('bw-switch')?.classList.add('on');
    document.getElementById('a11y-bw-btn')?.classList.add('active');
  }

  // Ferme le panel a11y au clic extérieur
  document.addEventListener('click', e => {
    if (!e.target.closest('#a11y-btn') && !e.target.closest('#a11y-panel')) {
      document.getElementById('a11y-panel')?.classList.remove('open');
      document.getElementById('a11y-btn')?.classList.remove('open');
    }
  });
});

// ── Accessibilité ────────────────────────────────────────────────────────────
// ── Traduction ────────────────────────────────────────────────────────────────
let _currentLang = 'fr';
const _tradCache  = {};     // { 'en': Map<original, translated> }
const _origTexts  = new Map(); // node → texte original

function _getTranslatableNodes() {
  const SKIP = 'script,style,input,select,textarea,.mob-val,.sb-val,.fm-val,.a11y-float,.trad-panel,#a11y-panel';
  const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT, {
    acceptNode(n) {
      const t = n.textContent.trim();
      if (!t || t.length < 2) return NodeFilter.FILTER_REJECT;
      if (/^[\d\s,.\-+%€×\/:()[\]]+$/.test(t)) return NodeFilter.FILTER_REJECT;
      if (n.parentElement?.closest(SKIP)) return NodeFilter.FILTER_REJECT;
      return NodeFilter.FILTER_ACCEPT;
    }
  });
  const nodes = [];
  while (walker.nextNode()) nodes.push(walker.currentNode);
  return nodes;
}

window.toggleTradPanel = function() {
  const panel = document.getElementById('trad-panel');
  const btn   = document.getElementById('trad-btn');
  const open  = panel.classList.toggle('open');
  btn.classList.toggle('open', open);
};

window.translateApp = async function(lang) {
  // Ferme le panel
  document.getElementById('trad-panel')?.classList.remove('open');
  document.getElementById('trad-btn')?.classList.remove('open');

  // Marque le bouton actif
  document.querySelectorAll('.trad-lang-btn').forEach(b => b.classList.remove('active'));
  document.querySelector(`.trad-lang-btn[onclick="translateApp('${lang}')"]`)?.classList.add('active');

  // Retour au français : restaure les textes originaux
  if (lang === 'fr') {
    _origTexts.forEach((orig, node) => { if (node.isConnected) node.textContent = orig; });
    document.documentElement.lang = 'fr';
    _currentLang = 'fr';
    return;
  }

  const btn = document.getElementById('trad-btn');
  btn.classList.add('loading');
  btn.textContent = '🌐 …';

  const nodes = _getTranslatableNodes();

  // Sauvegarde les originaux (une seule fois)
  nodes.forEach(n => { if (!_origTexts.has(n)) _origTexts.set(n, n.textContent); });

  // Textes à traduire (originaux français)
  const texts = nodes.map(n => _origTexts.get(n));

  // Cache par langue
  if (!_tradCache[lang]) _tradCache[lang] = new Map();
  const cache = _tradCache[lang];

  const toFetch   = [...new Set(texts)].filter(t => !cache.has(t));

  try {
    if (toFetch.length > 0) {
      // MyMemory API — gratuite, open, sans clé, ~1000 mots/jour
      const CHUNK = 20;
      for (let i = 0; i < toFetch.length; i += CHUNK) {
        const chunk = toFetch.slice(i, i + CHUNK);
        const joined = chunk.join('\n\n');
        const url = `https://api.mymemory.translated.net/get?q=${encodeURIComponent(joined)}&langpair=fr|${lang}`;
        const r = await fetch(url);
        if (!r.ok) throw new Error('HTTP ' + r.status);
        const data = await r.json();
        const translated = data.responseData.translatedText.split('\n\n');
        chunk.forEach((orig, j) => cache.set(orig, translated[j] ?? orig));
      }
    }

    // Applique
    nodes.forEach(n => {
      const orig = _origTexts.get(n);
      if (n.isConnected && cache.has(orig)) n.textContent = cache.get(orig);
    });

    document.documentElement.lang = lang;
    _currentLang = lang;
  } catch(e) {
    console.error('Traduction échouée :', e);
    btn.textContent = '🌐 ✗';
    setTimeout(() => { btn.textContent = '🌐 LANGUE'; btn.classList.remove('loading'); }, 2000);
    return;
  }

  btn.textContent = '🌐 LANGUE';
  btn.classList.remove('loading');
};

// Ferme le panel traduction au clic extérieur
document.addEventListener('click', e => {
  if (!e.target.closest('#trad-btn') && !e.target.closest('#trad-panel')) {
    document.getElementById('trad-panel')?.classList.remove('open');
    document.getElementById('trad-btn')?.classList.remove('open');
  }
});

window.toggleA11yPanel = function() {
  const panel = document.getElementById('a11y-panel');
  const btn   = document.getElementById('a11y-btn');
  const open  = panel.classList.toggle('open');
  btn.classList.toggle('open', open);
};

window.toggleHVMode = function() {
  const active = document.body.classList.toggle('hv-mode');
  document.getElementById('hv-switch')?.classList.toggle('on', active);
  document.getElementById('a11y-hv-btn')?.classList.toggle('active', active);
  localStorage.setItem('xenna-hv', active ? '1' : '');
};

window.toggleZoom = function() {
  const active = document.body.classList.toggle('zoom-mode');
  document.documentElement.style.zoom = active ? '200%' : '';
  document.getElementById('zoom-switch')?.classList.toggle('on', active);
  document.getElementById('a11y-magnifier')?.classList.toggle('active', active);
  localStorage.setItem('xenna-zoom', active ? '1' : '');
};

const _fontCache = new Set();
window.setAppFont = function(fontName, restore = false) {
  if (!fontName) {
    document.body.classList.remove('custom-font');
    document.documentElement.style.removeProperty('--app-font');
    localStorage.removeItem('xenna-font');
    const picker = document.getElementById('font-picker');
    if (picker) picker.value = '';
    return;
  }

  // Charge la police depuis Google Fonts si pas encore chargée
  const key = fontName.replace(/ /g, '+');
  if (!_fontCache.has(key)) {
    const link = document.createElement('link');
    link.rel  = 'stylesheet';
    link.href = `https://fonts.googleapis.com/css2?family=${key}&display=swap`;
    document.head.appendChild(link);
    _fontCache.add(key);
  }

  document.documentElement.style.setProperty('--app-font', `'${fontName}', monospace`);
  document.body.classList.add('custom-font');
  localStorage.setItem('xenna-font', fontName);

  const picker = document.getElementById('font-picker');
  if (picker && restore) picker.value = fontName;
};

window.scan67 = function() {
  const selectors = '.mob-val, .sb-val, .ascii-tbl td, .fm-val, .fm-result td';
  const found = Array.from(document.querySelectorAll(selectors))
    .filter(el => /67/.test(el.textContent.replace(/[\s ]/g, '')) && el.offsetParent !== null);

  if (found.length === 0) return;

  const btn = document.getElementById('a11y-67-btn');
  btn.classList.add('active');

  found.forEach((el, i) => {
    setTimeout(() => {
      el.classList.remove('flash-67');
      void el.offsetWidth; // force reflow pour relancer l'animation
      el.classList.add('flash-67');
      el.addEventListener('animationend', () => el.classList.remove('flash-67'), { once: true });
    }, i * 500);
  });

  setTimeout(() => btn.classList.remove('active'), found.length * 500 + 200);
};

window.toggleBWMode = function() {
  const active = document.body.classList.toggle('bw-mode');
  document.getElementById('bw-switch')?.classList.toggle('on', active);
  document.getElementById('a11y-bw-btn')?.classList.toggle('active', active);
  localStorage.setItem('xenna-bw', active ? '1' : '');
};

// ── Sécurité : neutralise tout HTML dans les entrées utilisateur ─────────────
function esc(str) {
  return String(str)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

// ── Vue active ───────────────────────────────────────────────────────────────
window.setView = function (v) {
  ['mobile', 'desktop', 'annuel', 'forge', 'apropos', 'gaabrielle', 'hercule'].forEach(name =>
    document.body.classList.toggle('is-' + name, v === name)
  );
  document.getElementById("btn-desk").classList.toggle("active", v === "desktop");
  document.getElementById("btn-mob") .classList.toggle("active", v === "mobile");
  document.getElementById("btn-ann") .classList.toggle("active", v === "annuel");
  if (lastBulletin && (v === 'desktop' || v === 'mobile')) renderAll(lastBulletin);
  if (v === 'forge') forgeInit();
};

// ── Formatage ────────────────────────────────────────────────────────────────
function fmt(val) {
  const n = parseFloat(val);
  return n.toLocaleString("fr-FR", { minimumFractionDigits: 2, maximumFractionDigits: 2 }) + " €";
}
function fmtS(val, sign = false) {
  const n = parseFloat(val);
  const s = n.toLocaleString("fr-FR", { minimumFractionDigits: 2, maximumFractionDigits: 2 }) + " €";
  return sign && n > 0 ? "+" + s : s;
}
function fmtPct(val) {
  return (parseFloat(val) * 100).toFixed(2) + " %";
}
// Lit la date de simulation depuis le formulaire actif (format ISO YYYY-MM-DD)
function getDatePaie() {
  const src   = document.body.classList.contains("is-mobile") ? "m-date" : "d-date";
  const other = src === "m-date" ? "d-date" : "m-date";
  return document.getElementById(src)?.value || document.getElementById(other)?.value || TODAY;
}

// Formate une date ISO en DD/MM/YYYY pour l'affichage
function formatDate(iso) {
  if (!iso) return "—";
  const [y, m, d] = iso.split("-");
  return `${d}/${m}/${y}`;
}

// ── Barème PAS mensuel neutre (DGFIP — situation personne seule, 0 part) ────
// Source : Bulletin Officiel des Finances Publiques (BOFIP), barème 2025.
// Chaque taux s'applique UNIQUEMENT à la fraction de revenu dans la tranche.
const PAS_TRANCHES = [
  { min:     0, max:  1620, taux: 0.000 },
  { min:  1620, max:  1683, taux: 0.005 },
  { min:  1683, max:  1791, taux: 0.013 },
  { min:  1791, max:  1911, taux: 0.021 },
  { min:  1911, max:  2042, taux: 0.029 },
  { min:  2042, max:  2151, taux: 0.035 },
  { min:  2151, max:  2294, taux: 0.041 },
  { min:  2294, max:  2714, taux: 0.053 },
  { min:  2714, max:  3107, taux: 0.075 },
  { min:  3107, max:  3539, taux: 0.099 },
  { min:  3539, max:  3983, taux: 0.119 },
  { min:  3983, max:  4648, taux: 0.138 },
  { min:  4648, max:  5574, taux: 0.158 },
  { min:  5574, max:  6974, taux: 0.179 },
  { min:  6974, max:  8711, taux: 0.200 },
  { min:  8711, max: 12091, taux: 0.240 },
  { min: 12091, max: 16376, taux: 0.280 },
  { min: 16376, max: 25706, taux: 0.330 },
  { min: 25706, max: 55062, taux: 0.380 },
  { min: 55062, max: Infinity, taux: 0.430 },
];

function calculerPas(netImposable) {
  const n = parseFloat(netImposable);
  if (isNaN(n) || n <= 0) return { total: 0, taux_effectif: 0, details: [] };
  let total = 0;
  const details = [];
  for (const t of PAS_TRANCHES) {
    if (n <= t.min) break;
    const upper  = t.max === Infinity ? n : Math.min(n, t.max);
    const base   = +(upper - t.min).toFixed(2);
    const montant = base * t.taux;
    details.push({ min: t.min, max: t.max === Infinity ? null : t.max, taux: t.taux, base, montant: +montant.toFixed(2) });
    total += montant;
    if (t.max === Infinity || n <= t.max) break;
  }
  return { total: +total.toFixed(2), taux_effectif: n > 0 ? total / n : 0, details };
}

// ── Catégorie → classe CSS ────────────────────────────────────────────────────
const CAT_CLASS = {
  "Sécurité Sociale":       "cat-ss",
  "CSG/CRDS":               "cat-csg",
  "Retraite complémentaire":"cat-ret",
  "Prévoyance":             "cat-prev",
  "Chômage":                "cat-cho",
  "Allègement":             "cat-alleg",
};

// ── Registre formules ────────────────────────────────────────────────────────
// Alimenté à chaque render ; clé stable = code_cotisation + '_' + type.
const _fmStore = {};

// Assiette : formule de calcul selon le code de cotisation.
const BASE_FORMULES = {
  SS_VIEILLESSE_PLAF:   'min(Salaire brut, Plafond Mensuel Sécurité Sociale — PMSS)',
  CHOMAGE:              'min(Salaire brut, 4 × PMSS)',
  AGS:                  'min(Salaire brut, 4 × PMSS)',
  CSG_DEDUCTIBLE:       'Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels (CSS art. L136-2)',
  CSG_NON_DEDUCTIBLE:   'Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels',
  CRDS:                 'Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels',
  AGIRC_ARRCO_T1:       'min(Salaire brut, PMSS)  — Tranche 1 (entre 0 et 1 PMSS)',
  AGIRC_ARRCO_CEG_T1:   'min(Salaire brut, PMSS)  — Tranche 1',
  PREVOYANCE_CADRE_MIN: 'min(Salaire brut, PMSS)  — Tranche A',
  AGIRC_ARRCO_T2:       'Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2',
  AGIRC_ARRCO_CEG_T2:   'Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2',
};

function buildFormulaStar(key, interactive = true) {
  const spans = ['f','(','x',')']
    .map((ch, i) => `<span style="animation-delay:${i * 45}ms">${ch}</span>`)
    .join('');
  if (!interactive) {
    return `<span class="formula-star" aria-hidden="true">${spans}</span>`;
  }
  return `<span class="formula-star" data-fmkey="${key}" onclick="event.stopPropagation();showFormula('${key}')">${spans}</span>`;
}

function buildFormulaContent(c, type) {
  // Fillon : l'explication contient déjà la formule complète avec valeurs substituées.
  if (c.code === 'REDUCTION_FILLON') {
    return `<pre class="fm-fillon">${esc(c.explication)}</pre>`;
  }

  const isSal     = type === 'sal';
  const taux      = isSal ? c.taux_sal : c.taux_pat;
  const base      = parseFloat(c.base);
  const montant   = isSal ? parseFloat(c.montant_sal) : Math.abs(parseFloat(c.montant_pat));
  const tauxLbl   = isSal ? 'Taux salarial' : 'Taux patronal';
  const montLbl   = isSal ? 'Montant salarial' : type === 'alleg' ? 'Montant allègement' : 'Montant patronal';
  const resCls    = isSal ? 'c-sal' : type === 'alleg' ? 'c-alleg' : 'c-pat';

  const baseNote = BASE_FORMULES[c.code]
    ? `<div class="fm-base-note">Assiette  =  ${esc(BASE_FORMULES[c.code])}</div>`
    : '';

  return `
    <div class="fm-generic">Montant  =  Assiette  ×  ${tauxLbl}</div>
    ${baseNote}
    <table class="fm-calc">
      <tr>
        <td>Assiette</td>
        <td class="fm-op">=</td>
        <td class="fm-val c-base">${fmt(base)}</td>
      </tr>
      <tr>
        <td>${tauxLbl}</td>
        <td class="fm-op">×</td>
        <td class="fm-val c-taux">${fmtPct(taux)}</td>
      </tr>
      <tr class="fm-result fm-sep">
        <td>${montLbl}</td>
        <td class="fm-op">=</td>
        <td class="fm-val ${resCls}">${fmt(montant)}</td>
      </tr>
    </table>`;
}

function buildPasFormulaContent(netImposable) {
  const r = calculerPas(netImposable);
  const rows = r.details.map(d => {
    const minStr = d.min.toLocaleString('fr-FR') + ' €';
    const maxStr = d.max === null ? '∞' : d.max.toLocaleString('fr-FR') + ' €';
    const zero   = d.taux === 0;
    return `
      <tr class="${zero ? 'pas-zero' : ''}">
        <td>${minStr} → ${maxStr}</td>
        <td class="r">${fmt(d.base)}</td>
        <td class="r ${zero ? 'c-dim' : ''}">${(d.taux * 100).toFixed(1).replace('.', ',')} %</td>
        <td class="r ${zero ? 'c-dim' : 'c-purple'}">${zero ? '—' : fmt(d.montant)}</td>
      </tr>`;
  }).join('');

  return `
    <div class="fm-generic">Calcul progressif tranche par tranche</div>
    <div class="fm-base-note">Barème neutre mensuel DGFIP — situation : personne seule, 0 part (célibataire sans charge de famille).<br>
    Chaque taux s'applique uniquement à la fraction de revenu dans la tranche,<br>
    pas à la totalité du net imposable. Source : BOFIP — barème 2025.</div>
    <table class="pas-tbl">
      <thead>
        <tr>
          <th>Tranche mensuelle</th>
          <th class="r">Base imposée</th>
          <th class="r">Taux</th>
          <th class="r">Retenue</th>
        </tr>
      </thead>
      <tbody>${rows}</tbody>
      <tfoot>
        <tr>
          <td>Net imposable</td>
          <td class="r c-gray">${fmt(netImposable)}</td>
          <td class="r c-taux">${(r.taux_effectif * 100).toFixed(2)} %&nbsp;<span style="color:var(--dim);font-size:0.7em">(taux effectif)</span></td>
          <td class="r c-purple" style="font-weight:bold">${fmt(r.total)}</td>
        </tr>
      </tfoot>
    </table>`;
}

window.showFormula = function(key) {
  const entry = _fmStore[key];
  if (!entry) return;

  const fmBody = document.getElementById('fm-body');

  if (entry.type === 'pas') {
    document.getElementById('fm-title').textContent = 'Prélèvement à la Source (PAS)';
    document.getElementById('fm-badge').textContent = '── Détail par tranche — barème neutre mensuel DGFIP ─────────';
    fmBody.className = 'fm-type-pas';
    fmBody.innerHTML = buildPasFormulaContent(entry.netImposable);
    document.getElementById('fm-modal').classList.add('open');
    document.querySelectorAll(`[data-fmkey="${key}"]`).forEach(el => el.classList.add('visited'));
    return;
  }

  const { c, type } = entry;
  const isSal = type === 'sal';
  const badge = c.code === 'REDUCTION_FILLON'
    ? '── Allègement patronal ──────────────────────'
    : isSal
      ? '── Part salariale ───────────────────────────'
      : '── Part patronale ───────────────────────────';
  document.getElementById('fm-title').textContent = c.libelle;
  document.getElementById('fm-badge').textContent = badge;
  fmBody.className = `fm-type-${type}`;
  fmBody.innerHTML = buildFormulaContent(c, type);
  document.getElementById('fm-modal').classList.add('open');
  document.querySelectorAll(`[data-fmkey="${key}"]`).forEach(el => el.classList.add('visited'));
};

window.closeFmModal = function() {
  document.getElementById('fm-modal').classList.remove('open');
};

// ── TOGGLE EXPLICATION (bureau) ───────────────────────────────────────────────
window.toggleExpl = function (i) {
  const row  = document.getElementById(`row-${i}`);
  const expl = document.getElementById(`expl-${i}`);
  if (!row || !expl) return;
  const open = expl.style.display !== "none";
  expl.style.display = open ? "none" : "table-row";
  row.classList.toggle("open", !open);
};

// ═════════════════════════════════════════════════════════════════════════════
// RENDU VUE BUREAU
// ═════════════════════════════════════════════════════════════════════════════
function renderDesktop(b) {
  const el = document.getElementById("res-desktop");
  const cots = b.cotisations;
  const totalSal = cots.reduce((s, c) => s + parseFloat(c.montant_sal), 0);
  const totalPat = cots.reduce((s, c) => s + parseFloat(c.montant_pat), 0);
  const pas      = calculerPas(b.net_imposable);
  const netPayer = parseFloat(b.net_a_payer) - pas.total;
  _fmStore['PAS'] = { type: 'pas', netImposable: parseFloat(b.net_imposable) };

  // ── Barre récap ──
  const summaryBar = `
    <div class="summary-bar">
      <div class="sb-cell">
        <div class="sb-lbl">▸ SALAIRE BRUT</div>
        <div class="sb-val c-gray">${fmt(b.brut)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ RETENUES</div>
        <div class="sb-ded">
          <div class="sb-ded-row">
            <span>Cot. salariales</span>
            <span style="color:var(--red)">− ${fmt(totalSal)}</span>
          </div>
          <div class="sb-ded-row">
            <span>PAS (${(pas.taux_effectif * 100).toFixed(1)} %)</span>
            <span style="color:var(--purple)">− ${fmt(pas.total)}${buildFormulaStar('PAS')}</span>
          </div>
          <div class="sb-ded-total">
            <span>Total retenues</span>
            <span style="color:var(--red)">− ${fmt(totalSal + pas.total)}</span>
          </div>
        </div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ NET À PAYER</div>
        <div class="sb-val c-green">${fmt(netPayer)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ CHARGES PAT.</div>
        <div class="sb-val c-orange">${fmt(totalPat)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ SUPER BRUT</div>
        <div class="sb-val c-yellow">${fmt(parseFloat(b.brut) + totalPat)}</div>
      </div>
    </div>`;

  // ── Table cotisations salariales ──
  const cotSal      = cots.filter(c => parseFloat(c.montant_sal) > 0 || c.taux_sal !== "0");
  const cotPat      = cots.filter(c => parseFloat(c.montant_pat) > 0);
  const cotAlleg    = cots.filter(c => c.categorie === "Allègement");
  // Brut patronal = somme des lignes affichées (positives seulement, hors Fillon).
  // totalPat (net) est réservé au super brut dans la barre récap.
  const totalPatBrut = cotPat.reduce((s, c) => s + parseFloat(c.montant_pat), 0);

  function buildRows(list, offset) {
    return list.map((c, i) => {
      const idx    = offset + i;
      const catCls = CAT_CLASS[c.categorie] || "cat-ss";
      const salCls = parseFloat(c.montant_sal) > 0 ? "c-sal" : "c-dim";
      const patCls = parseFloat(c.montant_pat) > 0 ? "c-pat" : "c-dim";

      const keySal = `${c.code}_sal`;
      const keyPat = `${c.code}_pat`;
      const hasFmSal = parseFloat(c.montant_sal) > 0;
      const hasFmPat = parseFloat(c.montant_pat) > 0;
      if (hasFmSal) _fmStore[keySal] = { c, type: 'sal' };
      if (hasFmPat) _fmStore[keyPat] = { c, type: 'pat' };
      const starSal = buildFormulaStar(keySal, hasFmSal);
      const starPat = buildFormulaStar(keyPat, hasFmPat);

      return `
        <tr class="data-row" id="row-${idx}" onclick="toggleExpl(${idx})">
          <td>
            <span class="expand-icon">▶</span>
            <span class="cat ${catCls}">[${c.categorie}]</span>
            <span>${c.libelle}</span>
          </td>
          <td class="r">${fmt(c.base)}</td>
          <td class="r">${fmtPct(c.taux_sal)}</td>
          <td class="r ${salCls}">${fmt(c.montant_sal)}${starSal}</td>
          <td class="r">${fmtPct(c.taux_pat)}</td>
          <td class="r ${patCls}">${fmt(c.montant_pat)}${starPat}</td>
        </tr>
        <tr class="expl-row" id="expl-${idx}" style="display:none">
          <td colspan="6">
            <div class="expl-box">
              <div class="expl-txt">▸ ${c.explication}</div>
              ${c.loi_ref ? `<div class="expl-ref">§ ${c.loi_ref}</div>` : ""}
            </div>
          </td>
        </tr>`;
    }).join("");
  }

  const thead = `
    <colgroup>
      <col>
      <col style="width:13%">
      <col style="width:9%">
      <col style="width:13%">
      <col style="width:9%">
      <col style="width:13%">
    </colgroup>
    <thead>
      <tr>
        <th>COTISATION</th>
        <th class="r">BASE</th>
        <th class="r">TAUX SAL.</th>
        <th class="r">PART SALARIÉ</th>
        <th class="r">TAUX PAT.</th>
        <th class="r">PART PATRONALE</th>
      </tr>
    </thead>`;

  const tableSal = `
    <div class="tbl-section-head">── COTISATIONS SALARIALES ─────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${thead}
      <tbody>
        ${buildRows(cotSal, 0)}
        <tr class="tbl-total">
          <td colspan="3">TOTAL COTISATIONS SALARIALES</td>
          <td class="r c-sal">− ${fmt(totalSal)}</td>
          <td></td><td></td>
        </tr>
      </tbody>
    </table>`;

  const tablePat = `
    <div class="tbl-section-head">── COTISATIONS PATRONALES ──────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${thead}
      <tbody>
        ${buildRows(cotPat, cotSal.length)}
        <tr class="tbl-total">
          <td colspan="5">TOTAL COTISATIONS PATRONALES</td>
          <td class="r c-pat">+ ${fmt(totalPatBrut)}</td>
        </tr>
      </tbody>
    </table>`;

  const simBanner = `<div class="sim-period">
    SIMULATION AU <span class="sp-accent">${formatDate(getDatePaie())}</span>
    &nbsp;·&nbsp; PMSS en vigueur calculé depuis la base de données
  </div>`;

  // Section allègements (Fillon, etc.) — montants négatifs affichés en économie
  const totalAlleg = cotAlleg.reduce((s, c) => s + parseFloat(c.montant_pat), 0); // négatif
  const tableAlleg = cotAlleg.length === 0 ? "" : `
    <div class="tbl-section-head">── ALLÈGEMENTS PATRONAUX ───────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${thead}
      <tbody>
        ${cotAlleg.map((c, i) => {
          const idx    = cotSal.length + cotPat.length + i;
          const catCls = CAT_CLASS[c.categorie] || "cat-alleg";
          const montant = Math.abs(parseFloat(c.montant_pat));
          const keyAlleg = `${c.code}_alleg`;
          _fmStore[keyAlleg] = { c, type: 'alleg' };
          return `
            <tr class="data-row" id="row-${idx}" onclick="toggleExpl(${idx})">
              <td>
                <span class="expand-icon">▶</span>
                <span class="cat ${catCls}">[${c.categorie}]</span>
                <span>${c.libelle}</span>
              </td>
              <td class="r">${fmt(c.base)}</td>
              <td class="r"></td>
              <td class="r"></td>
              <td class="r c-alleg">${fmtPct(Math.abs(parseFloat(c.taux_pat)))}</td>
              <td class="r c-alleg">− ${fmt(montant)}${buildFormulaStar(keyAlleg)}</td>
            </tr>
            <tr class="expl-row" id="expl-${idx}" style="display:none">
              <td colspan="6">
                <div class="expl-box">
                  <div class="expl-txt">▸ ${c.explication}</div>
                  ${c.loi_ref ? `<div class="expl-ref">§ ${c.loi_ref}</div>` : ""}
                </div>
              </td>
            </tr>`;
        }).join("")}
        <tr class="tbl-total">
          <td colspan="5">TOTAL ALLÈGEMENTS PATRONAUX</td>
          <td class="r c-alleg">− ${fmt(Math.abs(totalAlleg))}</td>
        </tr>
      </tbody>
    </table>`;

  el.innerHTML = simBanner + summaryBar + `<div class="tbl-wrap">${tableSal}${tablePat}${tableAlleg}</div>`;
}

// ─── Accordéon mobile ───────────────────────────────────────────────────────
// panel : 'why' (explication + loi) | 'how' (formule de calcul)
window.mobToggle = function(id, panel) {
  const wrap = document.getElementById('mob-expand-' + id);
  const why  = document.getElementById('mob-expand-' + id + '-why');
  const how  = document.getElementById('mob-expand-' + id + '-how');
  if (!wrap) return;
  const isOpen = wrap.style.display !== 'none';
  const cur    = wrap.dataset.panel;
  if (!isOpen) {
    wrap.style.display = 'block';
    wrap.dataset.panel = panel;
    why.style.display = panel === 'why' ? 'block' : 'none';
    how.style.display = panel === 'how' ? 'block' : 'none';
  } else if (cur === panel) {
    wrap.style.display = 'none';
  } else {
    wrap.dataset.panel = panel;
    why.style.display = panel === 'why' ? 'block' : 'none';
    how.style.display = panel === 'how' ? 'block' : 'none';
  }
};

function buildMobCotRow(c, id, montantHtml, valCls, type, idx = 0) {
  const formulaHtml = c.code === 'REDUCTION_FILLON'
    ? `<pre class="fm-fillon">${esc(c.explication)}</pre>`
    : `<div class="fm-type-${type}">${buildFormulaContent(c, type)}</div>`;
  const whyHtml = `
    <div class="mob-exp-txt">${esc(c.explication)}</div>
    ${c.loi_ref ? `<div class="mob-exp-loi">§ ${esc(c.loi_ref)}</div>` : ''}`;
  const stripeCls = `mob-stripe-${type}-${idx % 2 === 0 ? 'a' : 'b'}`;
  return `
    <div class="${stripeCls}">
      <div class="mob-row">
        <span class="mob-lbl mob-cot-lbl"
              title="Explication et référence légale"
              onclick="mobToggle('${id}','why')">${esc(c.libelle)}</span>
        <span class="mob-val ${valCls} mob-cot-amt"
              title="Formule de calcul"
              onclick="mobToggle('${id}','how')">${montantHtml}</span>
      </div>
      <div class="mob-expand" id="mob-expand-${id}" style="display:none">
        <div id="mob-expand-${id}-why">${whyHtml}</div>
        <div id="mob-expand-${id}-how" style="display:none">${formulaHtml}</div>
      </div>
    </div>`;
}

// ═════════════════════════════════════════════════════════════════════════════
// RENDU VUE MOBILE
// ═════════════════════════════════════════════════════════════════════════════
function renderMobile(b) {
  const el  = document.getElementById("res-mobile");
  const nom = document.getElementById("m-nom")?.value || document.getElementById("d-nom")?.value || "";
  const prn = document.getElementById("m-prenom")?.value || document.getElementById("d-prenom")?.value || "";
  const cots = b.cotisations;

  const totalSal  = cots.reduce((s, c) => s + parseFloat(c.montant_sal), 0);
  const totalPat  = cots.reduce((s, c) => s + parseFloat(c.montant_pat), 0);
  const pas       = calculerPas(b.net_imposable);
  const netPayer  = parseFloat(b.net_a_payer) - pas.total;
  const superBrut = parseFloat(b.brut) + totalPat;

  const cotSalLines = cots
    .filter(c => parseFloat(c.montant_sal) > 0)
    .map((c, i) => buildMobCotRow(c, `${c.code}_sal`, `− ${fmt(c.montant_sal)}`, 'c-red', 'sal', i))
    .join('');

  const cotPatFiltered = cots.filter(c => parseFloat(c.montant_pat) > 0);
  const cotPatLines = cotPatFiltered
    .map((c, i) => buildMobCotRow(c, `${c.code}_pat`, `+ ${fmt(c.montant_pat)}`, 'c-orange', 'pat', i))
    .join('');
  const totalPatBrutMob = cotPatFiltered.reduce((s, c) => s + parseFloat(c.montant_pat), 0);

  const cotAllegLines = cots
    .filter(c => c.categorie === "Allègement")
    .map((c, i) => buildMobCotRow(c, `${c.code}_alleg`, `− ${fmt(Math.abs(parseFloat(c.montant_pat)))}`, 'c-alleg', 'alleg', i))
    .join('');

  const totalAlleg = cots
    .filter(c => c.categorie === "Allègement")
    .reduce((s, c) => s + parseFloat(c.montant_pat), 0);

  el.innerHTML = `
    <div class="mob-bulletin">

      <!-- En-tête bulletin -->
      <div class="mob-head">
        <span class="mob-head-title">BULLETIN DE PAYE</span>
        <div style="text-align:right">
          <div class="mob-head-name">${esc(prn)} ${esc(nom).toUpperCase()}</div>
          <div class="mob-head-date">simulation au ${formatDate(getDatePaie())}</div>
        </div>
      </div>

      <!-- Brut -->
      <div class="mob-row" style="margin-top:0.15rem">
        <span class="mob-lbl">Salaire de base brut</span>
        <span class="mob-val c-gray">${fmt(b.brut)}</span>
      </div>

      <!-- Section cotisations salariales -->
      <div class="mob-row section"><span class="mob-lbl">── COTISATIONS SALARIALES ──</span><span></span></div>
      ${cotSalLines}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL retenues salariales</span>
        <span class="mob-val c-red">− ${fmt(totalSal)}</span>
      </div>

      <!-- Net imposable -->
      <div class="mob-row net-row">
        <span class="mob-lbl">NET IMPOSABLE</span>
        <span class="mob-val c-green">${fmt(b.net_imposable)}</span>
      </div>

      <!-- PAS -->
      <div class="mob-row pas-row">
        <span class="mob-lbl">Prélèvement à la source (${(pas.taux_effectif * 100).toFixed(1)} %)</span>
        <span class="mob-val c-purple">− ${fmt(pas.total)}${buildFormulaStar('PAS')}</span>
      </div>

      <!-- Net à payer -->
      <div class="mob-row final-row">
        <span class="mob-lbl">NET À PAYER</span>
        <span class="mob-val c-green">${fmt(netPayer)}</span>
      </div>

      <!-- Section cotisations patronales -->
      <div class="mob-row section"><span class="mob-lbl">── COTISATIONS PATRONALES ──</span><span></span></div>
      ${cotPatLines}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL charges patronales brutes</span>
        <span class="mob-val c-orange">+ ${fmt(totalPatBrutMob)}</span>
      </div>

      <!-- Allègements -->
      ${cotAllegLines.length ? `
      <div class="mob-row section"><span class="mob-lbl">── ALLÈGEMENTS PATRONAUX ──</span><span></span></div>
      ${cotAllegLines}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL allègements</span>
        <span class="mob-val c-alleg">− ${fmt(Math.abs(totalAlleg))}</span>
      </div>` : ""}

      <!-- Super brut -->
      <div class="mob-row superbrut">
        <span class="mob-lbl">SUPER BRUT (coût employeur)</span>
        <span class="mob-val c-blue">${fmt(superBrut)}</span>
      </div>

    </div>`;
}

// ═════════════════════════════════════════════════════════════════════════════
// RENDU GLOBAL (les deux vues)
// ═════════════════════════════════════════════════════════════════════════════
function renderAll(b) {
  renderDesktop(b);
  renderMobile(b);
}

// ── Affichage d'erreur de saisie (avant l'appel API) ─────────────────────────
// Utilisé pour les validations côté JS — évite d'envoyer des args invalides à
// Rust, ce qui provoque des erreurs opaques de désérialisation dans Tauri.
function showInputError(msg) {
  const errHtml = `<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">⚠ ${esc(msg)}</div>`;
  document.getElementById("res-desktop").innerHTML = errHtml;
  document.getElementById("res-mobile").innerHTML  = errHtml;
}

// ═════════════════════════════════════════════════════════════════════════════
// CALCUL
// ═════════════════════════════════════════════════════════════════════════════
async function calculate(source) {
  const isM = source === "mobile";
  const brut         = document.getElementById(isM ? "m-brut"   : "d-brut").value;
  const statut       = document.getElementById(isM ? "m-statut" : "d-statut").value;
  const nom          = document.getElementById(isM ? "m-nom"    : "d-nom").value   || "Dupont";
  const prenom       = document.getElementById(isM ? "m-prenom" : "d-prenom").value || "Marie";
  const date         = document.getElementById(isM ? "m-date"   : "d-date").value  || TODAY;
  const alsaceMoselle = document.getElementById(isM ? "m-alsace-moselle" : "d-alsace-moselle")?.checked ?? false;

  // ── Validation côté JS ────────────────────────────────────────────────────
  // Si brut est vide ou non numérique, input[type="number"] retourne "".
  // Envoyer "" à Rust provoque une erreur de désérialisation Tauri muette.
  const brutVal = parseFloat(brut);
  if (!brut || isNaN(brutVal) || brutVal <= 0) {
    showInputError("Salaire brut invalide — saisir un montant positif.");
    return;
  }
  // La date est forcée à TODAY si vide, mais on vérifie le format ISO au cas où.
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date)) {
    showInputError(`Date invalide : '${date}' (format attendu : YYYY-MM-DD).`);
    return;
  }

  // Sync les deux formulaires
  ["d-brut","m-brut"].forEach(id => { const e = document.getElementById(id); if(e) e.value = brut; });
  ["d-statut","m-statut"].forEach(id => { const e = document.getElementById(id); if(e) e.value = statut; });
  ["d-nom","m-nom"].forEach(id => { const e = document.getElementById(id); if(e) e.value = nom; });
  ["d-prenom","m-prenom"].forEach(id => { const e = document.getElementById(id); if(e) e.value = prenom; });
  ["d-date","m-date"].forEach(id => { const e = document.getElementById(id); if(e) e.value = date; });

  try {
    const bulletin = await api("calculer_bulletin", {
      salarie: { nom, prenom, salaire_brut: brut.toString(), statut, alsace_moselle: alsaceMoselle },
      datePaie: date,
    });
    lastBulletin = bulletin;
    renderAll(bulletin);
  } catch (e) {
    // console.error permet de voir l'objet brut dans DevTools (F12 → Console)
    // même quand l'affichage UI est tronqué.
    console.error("[calculer_bulletin] erreur brute :", e);
    const msg     = errToStr(e);
    const errHtml = `<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">ERREUR : ${esc(msg)}</div>`;
    document.getElementById("res-desktop").innerHTML = errHtml;
    document.getElementById("res-mobile").innerHTML  = errHtml;
  }
}

// ═════════════════════════════════════════════════════════════════════════════
// VUE ANNUELLE
// ═════════════════════════════════════════════════════════════════════════════
function renderAnnuel(sim) {
  const el   = document.getElementById("res-annuel");
  const rows = sim.lignes;

  // Détecte les changements de SMIC pour les mettre en évidence
  const smics = rows.map(r => r.smic);

  const thead = `
    <thead><tr>
      <th style="text-align:left">MOIS</th>
      <th>SMIC</th>
      <th>BRUT</th>
      <th>RETENUES SAL.</th>
      <th>CHARGES PAT.</th>
      <th>FILLON</th>
      <th title="Différence Fillon régularisé − Fillon mensuel simple">Δ RÉGUL.</th>
      <th>NET</th>
      <th>COÛT EMPL.</th>
    </tr></thead>`;

  const tbody = rows.map((r, i) => {
    const smicChange = i > 0 && r.smic !== smics[i - 1];
    const delta      = parseFloat(r.fillon_regularise) - parseFloat(r.fillon_simple);
    const deltaTxt   = Math.abs(delta) < 0.005
      ? `<span style="color:var(--dim)">—</span>`
      : `<span class="delta-nonzero">${delta > 0 ? "+" : ""}${fmtS(delta.toFixed(2))}</span>`;

    return `<tr class="${smicChange ? "smic-change" : ""}">
      <td>${r.mois_libelle}</td>
      <td>${fmt(r.smic)}</td>
      <td>${fmt(r.brut)}</td>
      <td class="c-sal">− ${fmt(r.total_sal)}</td>
      <td class="c-pat">+ ${fmt(r.total_pat_brut)}</td>
      <td class="c-alleg">− ${fmt(r.fillon_regularise)}</td>
      <td>${deltaTxt}</td>
      <td class="c-green">${fmt(r.net_a_payer)}</td>
      <td class="c-yellow">${fmt(r.cout_employeur)}</td>
    </tr>`;
  }).join("");

  const tfoot = `
    <tr class="ann-total">
      <td>TOTAL ${sim.annee}</td>
      <td></td>
      <td>${fmt(sim.total_brut)}</td>
      <td class="c-sal">− ${fmt(sim.total_sal)}</td>
      <td class="c-pat">+ ${fmt(sim.total_pat_brut)}</td>
      <td class="c-alleg">− ${fmt(sim.total_fillon)}</td>
      <td></td>
      <td class="c-green">${fmt(sim.total_net)}</td>
      <td class="c-yellow">${fmt(sim.total_cout)}</td>
    </tr>`;

  // Récap pédagogique
  const totalPat = parseFloat(sim.total_pat_brut);
  const fillon   = parseFloat(sim.total_fillon);
  const recap = `
    <div style="display:flex;gap:1rem;flex-wrap:wrap;margin-top:0.75rem;font-size:0.72rem">
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">ÉCONOMIE FILLON (annuelle)</div>
        <div style="color:var(--green);font-size:1.1rem;font-weight:bold">− ${fmt(sim.total_fillon)}</div>
      </div>
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">TAUX FILLON MOYEN</div>
        <div style="color:var(--blue);font-size:1.1rem;font-weight:bold">
          ${totalPat > 0 ? ((fillon / parseFloat(sim.total_brut)) * 100).toFixed(2) + " %" : "—"}
        </div>
      </div>
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">COÛT EMPLOYEUR ANNUEL</div>
        <div style="color:var(--yellow);font-size:1.1rem;font-weight:bold">${fmt(sim.total_cout)}</div>
      </div>
    </div>`;

  el.innerHTML = `
    <div class="tbl-section-head">── SIMULATION ANNUELLE ${sim.annee} ────────────────────────────────────</div>
    <table class="ann-tbl">
      ${thead}
      <tbody>${tbody}</tbody>
      ${tfoot}
    </table>
    ${recap}`;
}

async function calculerAnnee() {
  const annee  = parseInt(document.getElementById("a-annee").value);
  const brut   = document.getElementById("a-brut").value;
  const statut = document.getElementById("a-statut").value;

  const el = document.getElementById("res-annuel");

  // ── Validation côté JS ────────────────────────────────────────────────────
  // parseInt("") → NaN ; JSON.stringify({annee: NaN}) → {"annee":null}
  // Tauri ne peut pas désérialiser null en i32 → erreur muette.
  if (isNaN(annee) || annee < 1900 || annee > 2100) {
    el.innerHTML = `<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Année invalide.</div>`;
    return;
  }
  const brutVal = parseFloat(brut);
  if (!brut || isNaN(brutVal) || brutVal <= 0) {
    el.innerHTML = `<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Salaire brut invalide — saisir un montant positif.</div>`;
    return;
  }

  el.innerHTML = `<div style="color:var(--muted);padding:1rem;font-size:0.78rem">Calcul en cours…</div>`;

  try {
    const sim = await api("simuler_annee", {
      annee,
      salaireBrut: brut.toString(),
      statut,
    });
    renderAnnuel(sim);
  } catch (e) {
    console.error("[simuler_annee] erreur brute :", e);
    el.innerHTML = `<div style="padding:1rem;color:var(--red);font-size:0.8rem">ERREUR : ${esc(errToStr(e))}</div>`;
  }
}

// ── Paramètres avancés ───────────────────────────────────────────────────────
window.toggleParams = function(prefix) {
  const panel  = document.getElementById(`${prefix}-params`);
  const toggle = document.getElementById(`${prefix}-params-toggle`);
  if (!panel) return;
  const open = panel.style.display !== 'none';
  panel.style.display  = open ? 'none' : 'block';
  toggle.classList.toggle('open', !open);
};

// Synchronise un paramètre checkbox entre les deux formulaires (desktop ↔ mobile)
window.syncParam = function(paramName, checked) {
  ['d', 'm'].forEach(prefix => {
    const el = document.getElementById(`${prefix}-${paramName}`);
    if (el && el.checked !== checked) el.checked = checked;
  });
};

// ── Listeners ────────────────────────────────────────────────────────────────
document.getElementById("d-calc").addEventListener("click", () => calculate("desktop"));
document.getElementById("m-calc").addEventListener("click", () => calculate("mobile"));
document.getElementById("a-calc").addEventListener("click", calculerAnnee);

// ══════════════════════════════════════════════════════════════════════════════
// FORGE MÉTIER
// ══════════════════════════════════════════════════════════════════════════════

// Conventions collectives proposées dans le formulaire (triées alphabétiquement)
const LISTE_CCN = [
  { idcc: '1261', libelle: 'Acteurs du lien social et familial (ALISFA)' },
  { idcc: '2941', libelle: 'Aide, accompagnement, soins et services à domicile' },
  { idcc: '1747', libelle: 'Activités industrielles de boulangerie et de pâtisserie' },
  { idcc: '2149', libelle: 'Activités du déchet' },
  { idcc: '2335', libelle: 'Agences générales d\'assurances' },
  { idcc: '1686', libelle: 'Audiovisuel, électronique et équipement ménager' },
  { idcc: '2120', libelle: 'Banque' },
  { idcc: '3210', libelle: 'Banque Populaire' },
  { idcc: '0567', libelle: 'Bijouterie, joaillerie, orfèvrerie (obsolète)' },
  { idcc: '0158', libelle: 'Bois et scieries' },
  { idcc: '0992', libelle: 'Boucherie' },
  { idcc: '0843', libelle: 'Boulangerie-pâtisserie artisanales' },
  { idcc: '1606', libelle: 'Bricolage' },
  { idcc: '1486', libelle: 'Bureaux d\'études techniques et sociétés de conseils (Syntec)' },
  { idcc: '0787', libelle: 'Cabinets d\'experts-comptables et de commissaires aux comptes' },
  { idcc: '2332', libelle: 'Cabinets d\'architectes' },
  { idcc: '1619', libelle: 'Cabinets dentaires' },
  { idcc: '2420', libelle: 'Cadres du bâtiment' },
  { idcc: '3212', libelle: 'Cadres des travaux publics' },
  { idcc: '1256', libelle: 'Cadres des entreprises de gestion d\'équipements thermiques et de climatisation' },
  { idcc: '0211', libelle: 'Cadres des industries de carrières et matériaux (obsolète)' },
  { idcc: '0045', libelle: 'Caoutchouc' },
  { idcc: '2257', libelle: 'Casinos' },
  { idcc: '0783', libelle: 'Centres d\'hébergement et de réadaptation sociale' },
  { idcc: '0953', libelle: 'Charcuterie de détail' },
  { idcc: '1580', libelle: 'Chaussure' },
  { idcc: '2060', libelle: 'Chaînes de cafétérias' },
  { idcc: '1557', libelle: 'Commerce des articles de sports et d\'équipements de loisirs' },
  { idcc: '2216', libelle: 'Commerce de détail et de gros à prédominance alimentaire' },
  { idcc: '1505', libelle: 'Commerce de détail alimentaire non spécialisé' },
  { idcc: '2198', libelle: 'Commerce à distance et E-commerce' },
  { idcc: '1483', libelle: 'Commerce de détail de l\'habillement' },
  { idcc: '1487', libelle: 'Commerce de détail de l\'horlogerie-bijouterie' },
  { idcc: '3237', libelle: 'Commerce de détail alimentaire spécialisé' },
  { idcc: '1225', libelle: 'Commerce de la Réunion' },
  { idcc: '0468', libelle: 'Commerce succursaliste de la chaussure' },
  { idcc: '0573', libelle: 'Commerces de gros' },
  { idcc: '1517', libelle: 'Commerces de détail non alimentaires (Codena)' },
  { idcc: '0500', libelle: 'Commerces de gros de l\'habillement, mercerie, chaussure et jouet' },
  { idcc: '3243', libelle: 'Commerces de quincaillerie, fournitures industrielles, fers, métaux et équipement de la maison' },
  { idcc: '2596', libelle: 'Coiffure' },
  { idcc: '1611', libelle: 'Communication écrite directe' },
  { idcc: '1286', libelle: 'Confiserie, chocolaterie, biscuiterie' },
  { idcc: '2583', libelle: 'Concessionnaires et exploitants d\'autoroutes ou d\'ouvrages routiers' },
  { idcc: '3217', libelle: 'Convention collective nationale de la branche ferroviaire' },
  { idcc: '2272', libelle: 'Convention collective nationale de l\'assainissement et de la maintenance industrielle' },
  { idcc: '2002', libelle: 'Convention collective interrégionale de la blanchisserie, laverie, location de linge, nettoyage à sec, pressing et teinturerie du 17 novembre 1997' },
  { idcc: '2247', libelle: 'Courtage d\'assurances et/ou de réassurances' },
  { idcc: '0303', libelle: 'Couture parisienne et autres métiers de la mode' },
  { idcc: '0733', libelle: 'Détaillants en chaussures' },
  { idcc: '1605', libelle: 'Désinfection, désinsectisation, dératisation' },
  { idcc: '1536', libelle: 'Distributeurs conseils hors domicile' },
  { idcc: '2372', libelle: 'Distribution directe' },
  { idcc: '1408', libelle: 'Distribution, Logistique et Services des Energies de Proximité' },
  { idcc: '2121', libelle: 'Édition' },
  { idcc: '1518', libelle: 'Education, culture, loisirs et animation agissant pour l\'utilité sociale et environnementale, au service des territoires (ECLAT)' },
  { idcc: '2609', libelle: 'Employés, techniciens et agents de maîtrise du bâtiment' },
  { idcc: '2614', libelle: 'Employés, techniciens et agents de maîtrise des travaux publics' },
  { idcc: '0135', libelle: 'Employés techniciens et agents de maîtrise des industries de carrières et de matériaux (obsolète)' },
  { idcc: '3218', libelle: 'Enseignement privé non lucratif' },
  { idcc: '2691', libelle: 'Enseignement privé hors contrat' },
  { idcc: '3043', libelle: 'Entreprises de propreté' },
  { idcc: '3127', libelle: 'Entreprises de services à la personne' },
  { idcc: '1285', libelle: 'Entreprises artistiques et culturelles' },
  { idcc: '1539', libelle: 'Entreprises du bureau et du numérique - Commerces et services (Eben)' },
  { idcc: '1412', libelle: 'Entreprises d\'installation sans fabrication de matériel aéraulique, thermique, frigorifique' },
  { idcc: '2717', libelle: 'Entreprises techniques au service de la création et de l\'évènement' },
  { idcc: '3032', libelle: 'Esthétique' },
  { idcc: '0029', libelle: 'Établissements privés d\'hospitalisation, de soins, de cure et de garde à but non lucratif (CCN 51 - FEHAP)' },
  { idcc: '0413', libelle: 'Établissements et services pour personnes inadaptées et handicapées (CCN 66)' },
  { idcc: '0405', libelle: 'Établissements médico-sociaux de l\'union intersyndicale des secteurs sanitaires et sociaux (CCN 65)' },
  { idcc: '0478', libelle: 'Établissements financiers' },
  { idcc: '0915', libelle: 'Expertises en matière d\'évaluations industrielles et commerciales' },
  { idcc: '1307', libelle: 'Exploitation cinématographique' },
  { idcc: '1405', libelle: 'Expédition et exportation de fruits et légumes' },
  { idcc: '1411', libelle: 'Fabrication de l\'ameublement' },
  { idcc: '0669', libelle: 'Fabrication mécanique du verre' },
  { idcc: '1821', libelle: 'Fabrication du verre à la main, semi-automatique et mixte' },
  { idcc: '1031', libelle: 'Fédération nationale des associations familiales rurales' },
  { idcc: '1978', libelle: 'Fleuristes, vente et services des animaux familiers' },
  { idcc: '0200', libelle: 'Froid' },
  { idcc: '1043', libelle: 'Gardiens d\'immeubles' },
  { idcc: '2543', libelle: 'Géomètres et experts-fonciers' },
  { idcc: '2021', libelle: 'Golf' },
  { idcc: '2156', libelle: 'Grands magasins' },
  { idcc: '2336', libelle: 'Habitat et du Logement Accompagnés' },
  { idcc: '1631', libelle: 'Hôtellerie de plein air' },
  { idcc: '1979', libelle: 'Hôtels, cafés, restaurants (HCR)' },
  { idcc: '2264', libelle: 'Hospitalisation privée (FHP)' },
  { idcc: '1921', libelle: 'Huissiers de justice' },
  { idcc: '0044', libelle: 'Industries chimiques' },
  { idcc: '1534', libelle: 'Industrie et commerces en gros des viandes' },
  { idcc: '3233', libelle: 'Industrie de la fabrication des ciments' },
  { idcc: '2089', libelle: 'Industrie des panneaux à base de bois' },
  { idcc: '0176', libelle: 'Industrie pharmaceutique' },
  { idcc: '1388', libelle: 'Industrie du pétrole' },
  { idcc: '0112', libelle: 'Industrie laitière' },
  { idcc: '0018', libelle: 'Industrie textile' },
  { idcc: '3236', libelle: 'Industrie et services nautiques' },
  { idcc: '3109', libelle: 'Industries alimentaires diverses' },
  { idcc: '0247', libelle: 'Industries de l\'habillement' },
  { idcc: '2542', libelle: 'Industries métallurgiques, mécaniques et connexes de l\'Aisne (obsolète)' },
  { idcc: '3209', libelle: 'Industries métallurgiques, mécaniques et connexes du Doubs (obsolète)' },
  { idcc: '2003', libelle: 'Industries métallurgiques, électriques et électroniques des Vosges (obsolète)' },
  { idcc: '2630', libelle: 'Industries métallurgiques des Bouches-du-Rhône et Alpes-de-Haute-Provence (obsolète)' },
  { idcc: '1396', libelle: 'Industries de produits alimentaires élaborés' },
  { idcc: '0489', libelle: 'Industries du cartonnage' },
  { idcc: '0637', libelle: 'Industries et commerce de la récupération' },
  { idcc: '1938', libelle: 'Industries de la transformation des volailles' },
  { idcc: '1586', libelle: 'Industries charcutières' },
  { idcc: '0184', libelle: 'Imprimerie de labeur et industries graphiques' },
  { idcc: '0043', libelle: 'Import-export et commerce international' },
  { idcc: '1527', libelle: 'Immobilier' },
  { idcc: '0650', libelle: 'Ingénieurs et cadres de la métallurgie (obsolète)' },
  { idcc: '1679', libelle: 'Inspection d\'assurance' },
  { idcc: '1794', libelle: 'Institutions de retraite complémentaire' },
  { idcc: '1760', libelle: 'Jardineries et graineteries' },
  { idcc: '1480', libelle: 'Journalistes' },
  { idcc: '0959', libelle: 'Laboratoires de biologie médicale extra-hospitaliers' },
  { idcc: '3013', libelle: 'Librairie' },
  { idcc: '1404', libelle: 'Machines et matériels agricoles et de travaux publics (SDLM)' },
  { idcc: '0675', libelle: 'Maisons à succursales de vente au détail d\'habillement' },
  { idcc: '0538', libelle: 'Manutention ferroviaire' },
  { idcc: '2528', libelle: 'Maroquinerie' },
  { idcc: '1589', libelle: 'Mareyeurs-expéditeurs' },
  { idcc: '2931', libelle: 'Marchés financiers' },
  { idcc: '3222', libelle: 'Menuiseries charpentes et constructions industrialisées et des portes planes' },
  { idcc: '0822', libelle: 'Mensuels de la métallurgie de la Savoie (obsolète)' },
  { idcc: '1387', libelle: 'Mensuels de la métallurgie des Flandres (obsolète)' },
  { idcc: '0914', libelle: 'Mensuels de la métallurgie de l\'Ain (obsolète)' },
  { idcc: '1930', libelle: 'Meunerie' },
  { idcc: '2190', libelle: 'Missions locales et PAIO des maisons de l\'emploi et PLIE' },
  { idcc: '1499', libelle: 'Miroiterie, transformation et négoce du verre' },
  { idcc: '0827', libelle: 'Métallurgie des Ardennes (obsolète)' },
  { idcc: '0863', libelle: 'Métallurgie d\'Ille-et-Vilaine et du Morbihan (obsolète)' },
  { idcc: '1867', libelle: 'Métallurgie de la Drôme et de l\'Ardèche (obsolète)' },
  { idcc: '0984', libelle: 'Métallurgie d\'Eure-et-Loir (obsolète)' },
  { idcc: '2992', libelle: 'Métallurgie d\'Indre-et-Loire (obsolète)' },
  { idcc: '0898', libelle: 'Métallurgie de l\'Allier (obsolète)' },
  { idcc: '1572', libelle: 'Métallurgie de la Charente (obsolète)' },
  { idcc: '1885', libelle: 'Métallurgie de la Côte-d\'Or (obsolète)' },
  { idcc: '1635', libelle: 'Métallurgie de la Gironde et des Landes (obsolète)' },
  { idcc: '1578', libelle: 'Métallurgie de la Loire et de l\'arrondissement d\'Yssingeaux (obsolète)' },
  { idcc: '0828', libelle: 'Métallurgie de la Manche (obsolète)' },
  { idcc: '0899', libelle: 'Métallurgie de la Marne (obsolète)' },
  { idcc: '1813', libelle: 'Métallurgie de la région de Maubeuge (obsolète)' },
  { idcc: '1525', libelle: 'Métallurgie de la région dunkerquoise (obsolète)' },
  { idcc: '0930', libelle: 'Métallurgie de la Sarthe (obsolète)' },
  { idcc: '0920', libelle: 'Métallurgie de la Vienne (obsolète)' },
  { idcc: '3053', libelle: 'Métallurgie de Haute-Saône (obsolète)' },
  { idcc: '1576', libelle: 'Métallurgie du Cher (obsolète)' },
  { idcc: '0943', libelle: 'Métallurgie du Calvados (obsolète)' },
  { idcc: '0860', libelle: 'Métallurgie du Finistère (obsolète)' },
  { idcc: '2126', libelle: 'Métallurgie du Gard et de la Lozère (obsolète)' },
  { idcc: '1912', libelle: 'Métallurgie du Haut-Rhin (obsolète)' },
  { idcc: '0836', libelle: 'Métallurgie de la Haute-Savoie (obsolète)' },
  { idcc: '0937', libelle: 'Métallurgie de la Haute-Vienne et de la Creuse (obsolète)' },
  { idcc: '1577', libelle: 'Métallurgie de l\'Hérault, de l\'Aude et des Pyrénées-Orientales (obsolète)' },
  { idcc: '2221', libelle: 'Métallurgie de l\'Isère et des Hautes-Alpes' },
  { idcc: '1369', libelle: 'Métallurgie de Loire-Atlantique (obsolète)' },
  { idcc: '2579', libelle: 'Métallurgie du Loir-et-Cher (obsolète)' },
  { idcc: '1966', libelle: 'Métallurgie du Loiret (obsolète)' },
  { idcc: '1902', libelle: 'Métallurgie du Maine-et-Loire (obsolète)' },
  { idcc: '2266', libelle: 'Métallurgie de la Mayenne (obsolète)' },
  { idcc: '1365', libelle: 'Métallurgie de Meurthe-et-Moselle (obsolète)' },
  { idcc: '2755', libelle: 'Industries de la métallurgie de Belfort/Montbéliard (obsolète)' },
  { idcc: '1059', libelle: 'Métallurgie des Midi-Pyrénées (obsolète)' },
  { idcc: '0714', libelle: 'Métallurgie de la Moselle (obsolète)' },
  { idcc: '0948', libelle: 'Métallurgie de l\'Orne (obsolète)' },
  { idcc: '2700', libelle: 'Métallurgie de l\'Oise (obsolète)' },
  { idcc: '1472', libelle: 'Métallurgie du Pas-de-Calais (obsolète)' },
  { idcc: '2615', libelle: 'Métallurgie des Pyrénées-Atlantiques et du Seignanx (obsolète)' },
  { idcc: '0878', libelle: 'Métallurgie du Rhône (obsolète)' },
  { idcc: '1604', libelle: 'Métallurgie de Rouen et de Dieppe (obsolète)' },
  { idcc: '1564', libelle: 'Métallurgie de Saône-et-Loire (obsolète)' },
  { idcc: '0911', libelle: 'Métallurgie de Seine-et-Marne (obsolète)' },
  { idcc: '2980', libelle: 'Métallurgie de la Somme (obsolète)' },
  { idcc: '1592', libelle: 'Métallurgie du Valenciennois et du Cambrésis (obsolète)' },
  { idcc: '2489', libelle: 'Métallurgie de la Vendée (obsolète)' },
  { idcc: '1634', libelle: 'Métallurgie des Côtes-d\'Armor (obsolète)' },
  { idcc: '2630', libelle: 'Métallurgie des Bouches-du-Rhône (obsolète)' },
  { idcc: '1315', libelle: 'Industries métallurgiques et mécaniques de la Haute-Marne et de la Meuse (obsolète)' },
  { idcc: '1732', libelle: 'Métallurgie de l\'Yonne (obsolète)' },
  { idcc: '1560', libelle: 'Métallurgiques des Alpes-Maritimes (obsolète)' },
  { idcc: '0979', libelle: 'Métallurgiques de l\'arrondissement du Havre (obsolète)' },
  { idcc: '2128', libelle: 'Mutualité' },
  { idcc: '1077', libelle: 'Négoce et industrie des produits du sol, engrais et produits connexes' },
  { idcc: '1880', libelle: 'Négoce de l\'ameublement' },
  { idcc: '1982', libelle: 'Négoce et prestations de services dans les domaines médico-techniques' },
  { idcc: '1947', libelle: 'Négoce de bois d\'oeuvre et produits dérivés (obsolète)' },
  { idcc: '0054', libelle: 'Non-cadres des industries métallurgiques et mécaniques de la région parisienne (obsolète)' },
  { idcc: '0998', libelle: 'Non-cadres de l\'exploitation d\'équipements thermiques et de génie climatique' },
  { idcc: '2205', libelle: 'Notaires' },
  { idcc: '3220', libelle: 'Offices publics de l\'habitat' },
  { idcc: '3245', libelle: 'Opérateurs de voyages et guides' },
  { idcc: '1431', libelle: 'Optique-lunetterie de détail' },
  { idcc: '1316', libelle: 'Organismes de tourisme social et familial' },
  { idcc: '1909', libelle: 'Organismes de tourisme' },
  { idcc: '1516', libelle: 'Organismes de formation' },
  { idcc: '1790', libelle: 'Parcs de loisirs et d\'attractions' },
  { idcc: '1267', libelle: 'Pâtisserie' },
  { idcc: '1000', libelle: 'Personnel des cabinets d\'avocats' },
  { idcc: '1147', libelle: 'Personnel des cabinets médicaux' },
  { idcc: '0275', libelle: 'Personnel au sol du transport aérien' },
  { idcc: '2046', libelle: 'Personnel non médical des centres de lutte contre le cancer' },
  { idcc: '2972', libelle: 'Personnel sédentaire des entreprises de navigation' },
  { idcc: '1558', libelle: 'Personnel des industries céramiques' },
  { idcc: '1996', libelle: 'Pharmacie d\'officine' },
  { idcc: '1504', libelle: 'Poissonnerie' },
  { idcc: '0759', libelle: 'Pompes funèbres' },
  { idcc: '2683', libelle: 'Portage de presse' },
  { idcc: '3017', libelle: 'Ports et Manutention' },
  { idcc: '3230', libelle: 'Presse (Information spécialisée [ETAM et cadres])' },
  { idcc: '3242', libelle: 'Presse quotidienne et hebdomadaire en régions' },
  { idcc: '2098', libelle: 'Prestataires de services du secteur tertiaire' },
  { idcc: '1351', libelle: 'Prévention et sécurité' },
  { idcc: '1512', libelle: 'Promotion immobilière' },
  { idcc: '0292', libelle: 'Plasturgie' },
  { idcc: '3168', libelle: 'Professions de la photographie' },
  { idcc: '3244', libelle: 'Professions réglementées auprès des juridictions' },
  { idcc: '1555', libelle: 'Produits à usage pharmaceutique, parapharmaceutique et vétérinaire' },
  { idcc: '1513', libelle: 'Production des eaux embouteillées, des boissons rafraîchissantes sans alcool et de bière' },
  { idcc: '2642', libelle: 'Production audiovisuelle' },
  { idcc: '3238', libelle: 'Production et transformation des papiers et cartons' },
  { idcc: '0653', libelle: 'Producteurs salariés de base des services extérieurs de production des sociétés d\'assurances' },
  { idcc: '0993', libelle: 'Prothèse dentaire' },
  { idcc: '0086', libelle: 'Publicité' },
  { idcc: '1621', libelle: 'Répartition pharmaceutique' },
  { idcc: '0454', libelle: 'Remontées mécaniques et domaines skiables' },
  { idcc: '1266', libelle: 'Restauration de collectivités' },
  { idcc: '1501', libelle: 'Restauration rapide' },
  { idcc: '1413', libelle: 'Salariés permanents des entreprises de travail temporaire' },
  { idcc: '3216', libelle: 'Salariés du négoce des matériaux de construction' },
  { idcc: '3219', libelle: 'Salariés en portage salarial' },
  { idcc: '1875', libelle: 'Salariés des cabinets et cliniques vétérinaires' },
  { idcc: '0897', libelle: 'Services de prévention et de santé au travail interentreprises' },
  { idcc: '1090', libelle: 'Services de l\'automobile' },
  { idcc: '2147', libelle: 'Services d\'eau et d\'assainissement' },
  { idcc: '2344', libelle: 'Sidérurgie (Nord, Moselle, Meurthe-et-Moselle)' },
  { idcc: '1672', libelle: 'Sociétés d\'assurances' },
  { idcc: '1801', libelle: 'Sociétés d\'assistance' },
  { idcc: '2150', libelle: 'Sociétés anonymes et fondations d\'HLM' },
  { idcc: '3090', libelle: 'Spectacle vivant (secteur privé)' },
  { idcc: '2511', libelle: 'Sport' },
  { idcc: '2728', libelle: 'Sucreries, sucreries-distilleries et raffineries de sucre' },
  { idcc: '2219', libelle: 'Taxis parisiens salariés' },
  { idcc: '2148', libelle: 'Télécommunications' },
  { idcc: '3241', libelle: 'Télédiffusion' },
  { idcc: '1424', libelle: 'Transports publics' },
  { idcc: '0016', libelle: 'Transports routiers et activités auxiliaires du transport' },
  { idcc: '1170', libelle: 'Tuiles et briques (obsolète)' },
  { idcc: '0087', libelle: 'Ouvriers des industries de carrières et de matériaux (obsolète)' },
  { idcc: '1702', libelle: 'Ouvriers de travaux publics' },
  { idcc: '1596', libelle: 'Ouvriers des entreprises du bâtiment de moins de 10 salariés' },
  { idcc: '1597', libelle: 'Ouvriers des entreprises du bâtiment de plus de 10 salariés' },
  { idcc: '2389', libelle: 'Ouvriers du bâtiment et des travaux publics région de La Réunion' },
  { idcc: '2328', libelle: 'Ouvriers du bâtiment et des travaux publics de la Guadeloupe et dépendances' },
  { idcc: '2564', libelle: 'Vétérinaires praticiens salariés' },
  { idcc: '0493', libelle: 'Vins, cidres, jus de fruits, sirops, spiritueux et liqueurs de France' },
].sort((a, b) => a.libelle.localeCompare(b.libelle, 'fr'));

// Options HTML précalculées (une seule fois)
const CCN_OPTIONS = '<option value="">— Choisir une CCN —</option>' +
  LISTE_CCN.map(c => `<option value="${c.idcc}">${c.idcc} — ${c.libelle}</option>`).join('');

let forgeCache = []; // profils chargés, pour éviter un re-fetch sur clic carte

// ── Navigation interne à la forge ────────────────────────────────────────────
window.forgeNav = function(etat) {
  ['liste', 'detail', 'creer'].forEach(e => {
    document.getElementById('forge-' + e).style.display = e === etat ? 'block' : 'none';
  });
};

// ── Chargement de l'annuaire ──────────────────────────────────────────────────
async function forgeInit() {
  forgeNav('liste');
  const cards = document.getElementById('forge-cards');
  const sub   = document.getElementById('forge-subtitle');

  cards.innerHTML = `<div style="color:var(--muted);font-size:0.75rem;padding:0.5rem 0">chargement…</div>`;
  try {
    const r = await fetch('/forge/contributeurs');
    if (!r.ok) {
      const body = await r.text();
      throw new Error(`HTTP ${r.status} — ${body || r.statusText}`);
    }
    forgeCache = await r.json();
    const n = forgeCache.length;
    sub.textContent = n === 0
      ? 'aucun contributeur pour l\'instant'
      : `${n} contributeur${n > 1 ? 's' : ''} · ${forgeCache.reduce((s, p) => s + p.expertises.length, 0)} expertises CCN`;
    cards.innerHTML = n === 0
      ? `<div style="color:var(--muted);font-size:0.75rem">Aucun profil encore — sois le premier à rejoindre.</div>`
      : forgeCache.map(renderCarteForge).join('');
  } catch(e) {
    cards.innerHTML = `<div style="color:var(--red);font-size:0.75rem">Erreur : ${esc(errToStr(e))}</div>`;
  }
}

// ── Carte annuaire ────────────────────────────────────────────────────────────
function renderCarteForge(p) {
  const badges = p.expertises.slice(0, 5).map(e => {
    const cls = e.niveau === 'Maîtrisée' ? 'm' : e.niveau === 'Pratiquée' ? 'p' : 'c';
    return `<span class="ccn-badge ${cls}" title="${esc(e.niveau)}">${esc(e.ccn_libelle)}</span>`;
  }).join('');
  const plus = p.expertises.length > 5
    ? `<span class="ccn-badge c">+${p.expertises.length - 5}</span>` : '';

  return `
    <div class="forge-card" onclick="forgeAfficherProfil('${esc(p.pseudo)}')">
      <div class="forge-card-pseudo">${esc(p.pseudo)}</div>
      <div class="forge-card-poste">${esc(p.poste)} <span style="color:var(--dim);font-size:0.6em">${p.poste_est_actuel ? 'actuel' : 'visé'}</span></div>
      <div class="forge-card-ccn">${badges}${plus}</div>
      <div class="forge-card-stats">
        <span><span class="forge-stat-val">${p.votes_received}</span> votes</span>
        <span><span class="forge-stat-val">${p.topics_count}</span> sujets</span>
        <span><span class="forge-stat-val">${p.posts_count}</span> réponses</span>
      </div>
    </div>`;
}

// ── Fiche profil ──────────────────────────────────────────────────────────────
async function forgeAfficherProfil(pseudo) {
  forgeNav('detail');
  const el = document.getElementById('forge-profil-content');
  el.innerHTML = `<div style="color:var(--muted);font-size:0.75rem">chargement…</div>`;
  try {
    // Utilise le cache si disponible
    let p = forgeCache.find(x => x.pseudo.toLowerCase() === pseudo.toLowerCase());
    if (!p) {
      const r = await fetch(`/profil/${encodeURIComponent(pseudo)}`);
      if (!r.ok) throw new Error(`HTTP ${r.status} — ${await r.text() || r.statusText}`);
      p = await r.json();
    }
    el.innerHTML = renderFicheProfil(p);
  } catch(e) {
    el.innerHTML = `<div style="color:var(--red);font-size:0.75rem">Erreur : ${esc(errToStr(e))}</div>`;
  }
}

function renderFicheProfil(p) {
  const linkedin = p.linkedin_url
    ? `<a class="profil-linkedin" href="${esc(p.linkedin_url)}" target="_blank" rel="noopener noreferrer">↗ LinkedIn</a>`
    : '';

  // Regrouper les CCN par niveau
  const groupes = [
    { niveau: 'Maîtrisée', cls: 'm', items: p.expertises.filter(e => e.niveau === 'Maîtrisée') },
    { niveau: 'Pratiquée', cls: 'p', items: p.expertises.filter(e => e.niveau === 'Pratiquée') },
    { niveau: 'Connue',    cls: 'c', items: p.expertises.filter(e => e.niveau === 'Connue')    },
  ].filter(g => g.items.length > 0);

  const lignesCcn = groupes.map(g => `
    <tr class="profil-ccn-section"><td colspan="3">${esc(g.niveau)}</td></tr>
    ${g.items.map(e => `
    <tr>
      <td class="profil-ccn-idcc">${esc(e.ccn_idcc)}</td>
      <td>${esc(e.ccn_libelle)}</td>
      <td><span class="ccn-badge ${g.cls}">${esc(g.niveau)}</span></td>
    </tr>`).join('')}`).join('');

  const tableCcn = p.expertises.length === 0
    ? `<div style="color:var(--muted);font-size:0.72rem">Aucune CCN renseignée.</div>`
    : `<table class="profil-ccn-tbl">${lignesCcn}</table>`;

  const dateCreation = p.created_at ? formatDate(p.created_at.slice(0, 10)) : '—';

  return `
    <div class="profil-head">
      <div>
        <div class="profil-pseudo">${esc(p.pseudo)}</div>
        <div class="profil-poste">${esc(p.poste)} <span style="color:var(--dim);font-size:0.85em">(${p.poste_est_actuel ? 'poste actuel' : 'poste visé'})</span></div>
        ${linkedin}
      </div>
      <div class="profil-since">membre depuis le ${dateCreation}</div>
    </div>

    <div class="profil-body">
      <div class="sect-label">PAIE FRANÇAISE</div>
      ${p.paie_fr_niveau
        ? `<span class="ccn-badge ${p.paie_fr_niveau === 'Maîtrisée' ? 'm' : p.paie_fr_niveau === 'Pratiquée' ? 'p' : 'c'}" style="font-size:0.75rem;padding:0.2rem 0.6rem">${esc(p.paie_fr_niveau)}</span>`
        : `<span style="color:var(--dim);font-size:0.7rem">non renseigné</span>`}

      ${p.pays && p.pays.length > 0 ? `
      <div class="sect-label" style="margin-top:1rem">PAIE INTERNATIONALE</div>
      <table class="profil-ccn-tbl">
        ${[
            { niveau: 'Maîtrisée', cls: 'm', items: p.pays.filter(x => x.niveau === 'Maîtrisée') },
            { niveau: 'Pratiquée', cls: 'p', items: p.pays.filter(x => x.niveau === 'Pratiquée') },
            { niveau: 'Connue',    cls: 'c', items: p.pays.filter(x => x.niveau === 'Connue')    },
          ].filter(g => g.items.length > 0).map(g => `
            <tr class="profil-ccn-section"><td colspan="3">${esc(g.niveau)}</td></tr>
            ${g.items.map(x => `
            <tr>
              <td class="profil-ccn-idcc">${esc(x.pays_code)}</td>
              <td>${esc(x.pays_libelle)}</td>
              <td><span class="ccn-badge ${g.cls}">${esc(g.niveau)}</span></td>
            </tr>`).join('')}`).join('')}
      </table>` : ''}

      <div class="sect-label" style="margin-top:1rem">EXPERTISES CCN</div>
      ${tableCcn}
    </div>

    <div class="profil-stats">
      <div class="profil-stat">
        <div class="profil-stat-val">${p.votes_received}</div>
        <div class="profil-stat-lbl">votes reçus</div>
      </div>
      <div class="profil-stat">
        <div class="profil-stat-val">${p.votes_given}</div>
        <div class="profil-stat-lbl">votes donnés</div>
      </div>
      <div class="profil-stat">
        <div class="profil-stat-val">${p.topics_count}</div>
        <div class="profil-stat-lbl">sujets</div>
      </div>
      <div class="profil-stat">
        <div class="profil-stat-val">${p.posts_count}</div>
        <div class="profil-stat-lbl">réponses</div>
      </div>
    </div>`;
}

// ── Toggle poste actuel / visé ────────────────────────────────────────────────
window.setPosteType = function(estActuel) {
  document.getElementById('poste_est_actuel_input').value = estActuel ? '1' : '0';
  document.getElementById('ptog-actuel').className = 'ptog ' + (estActuel  ? 'ptog-on' : 'ptog-off');
  document.getElementById('ptog-vise')  .className = 'ptog ' + (!estActuel ? 'ptog-on' : 'ptog-off');
};

// ── Pays frontaliers + Royaume-Uni ────────────────────────────────────────────
const LISTE_PAYS = [
  { code: 'BE', libelle: 'Belgique' },
  { code: 'LU', libelle: 'Luxembourg' },
  { code: 'DE', libelle: 'Allemagne' },
  { code: 'CH', libelle: 'Suisse' },
  { code: 'IT', libelle: 'Italie' },
  { code: 'MC', libelle: 'Monaco' },
  { code: 'ES', libelle: 'Espagne' },
  { code: 'AD', libelle: 'Andorre' },
  { code: 'GB', libelle: 'Royaume-Uni' },
];
const PAYS_OPTIONS = LISTE_PAYS.map(p => `<option value="${p.code}">${esc(p.libelle)}</option>`).join('');

let forgePaysIdx = 0;

window.forgeAjouterPays = function() {
  const id  = ++forgePaysIdx;
  const row = document.createElement('div');
  row.className = 'forge-ccn-row';
  row.id = 'forge-pays-' + id;
  row.innerHTML = `
    <select class="forge-pays-select">${PAYS_OPTIONS}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprPays(${id})" title="Supprimer">×</button>`;
  document.getElementById('forge-pays-list').appendChild(row);
};

window.forgeSupprPays = function(id) {
  document.getElementById('forge-pays-' + id)?.remove();
};

// ── Formulaire création — gestion des lignes CCN ──────────────────────────────
let forgeCcnIdx = 0;

window.forgeAjouterCcn = function() {
  const id  = ++forgeCcnIdx;
  const row = document.createElement('div');
  row.className = 'forge-ccn-row';
  row.id = 'forge-ccn-' + id;
  row.innerHTML = `
    <select class="forge-ccn-select">${CCN_OPTIONS}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprCcn(${id})" title="Supprimer">×</button>`;
  document.getElementById('forge-ccn-list').appendChild(row);
};

window.forgeSupprCcn = function(id) {
  document.getElementById('forge-ccn-' + id)?.remove();
};

// ── Soumission du formulaire de création ──────────────────────────────────────
window.forgeSoumettre = async function(event) {
  event.preventDefault();
  const form   = document.getElementById('forge-form');
  const errEl  = document.getElementById('forge-form-err');
  const btnEl  = document.getElementById('forge-submit-btn');
  errEl.textContent = '';

  // Collecte des pays sélectionnés
  const pays = [];
  document.querySelectorAll('[id^="forge-pays-"]').forEach(row => {
    const code   = row.querySelector('.forge-pays-select')?.value;
    const niveau = row.querySelector('.forge-ccn-niveau')?.value;
    const p = LISTE_PAYS.find(x => x.code === code);
    if (code && p) pays.push({ pays_code: code, pays_libelle: p.libelle, niveau });
  });

  // Collecte des CCN sélectionnées
  const expertises = [];
  document.querySelectorAll('.forge-ccn-row:not([id^="forge-pays-"])').forEach(row => {
    const idcc   = row.querySelector('.forge-ccn-select').value;
    const niveau = row.querySelector('.forge-ccn-niveau').value;
    const ccn    = LISTE_CCN.find(c => c.idcc === idcc);
    if (idcc && ccn) expertises.push({ ccn_idcc: idcc, ccn_libelle: ccn.libelle, niveau });
  });

  const payload = {
    email:            form.querySelector('[name="email"]').value.trim(),
    pseudo:           form.querySelector('[name="pseudo"]').value.trim(),
    poste:            form.querySelector('[name="poste"]').value.trim(),
    linkedin_url:     form.querySelector('[name="linkedin_url"]').value.trim() || null,
    poste_est_actuel: form.querySelector('[name="poste_est_actuel"]').value !== '0',
    paie_fr_niveau:   form.querySelector('[name="paie_fr_niveau"]').value || null,
    pays,
    expertises,
  };

  // Validation JS basique
  if (!payload.email)  { errEl.textContent = 'Email requis.'; return; }
  if (!payload.pseudo) { errEl.textContent = 'Pseudo requis.'; return; }
  if (!payload.poste)  { errEl.textContent = 'Poste requis.'; return; }

  btnEl.disabled = true;
  btnEl.textContent = '[ envoi… ]';

  try {
    const r = await fetch('/forge/profil', {
      method:  'POST',
      headers: { 'Content-Type': 'application/json' },
      body:    JSON.stringify(payload),
    });
    if (!r.ok) throw new Error(`HTTP ${r.status} — ${await r.text() || r.statusText}`);
    const profil = await r.json();

    // Ajouter en tête du cache et afficher la fiche
    forgeCache.unshift(profil);
    form.reset();
    document.getElementById('forge-pays-list').innerHTML = '';
    document.getElementById('forge-ccn-list').innerHTML  = '';
    forgePaysIdx = 0;
    forgeCcnIdx  = 0;
    forgeAfficherProfil(profil.pseudo);
  } catch(e) {
    errEl.textContent = errToStr(e);
    btnEl.disabled = false;
    btnEl.textContent = '[ Rejoindre la Forge ]';
  }
};

// ── Toggle H / F — écart salarial moyen ──────────────────────────────────────
//
// Easter egg pédagogique : basculer sur F applique −X % sur le brut (X tiré aléatoirement
// parmi 11–17 %, pondéré autour de 14–15 %), reflet de l'écart salarial F/H constaté.
// Les noms par défaut sont des héros de fantasy d'auteurs européens.
// Dès qu'un nom est saisi manuellement, le toggle n'a plus d'effet.

const HEROS_H = [
  { prenom: 'Geralt',   nom: 'de Riv' },          // Sapkowski (polonais)
  { prenom: 'Sam',      nom: 'Vimes' },            // Pratchett (britannique)
  { prenom: 'Elric',    nom: 'de Melniboné' },     // Moorcock (britannique)
  { prenom: 'Druss',    nom: 'la Légende' },       // Gemmell (britannique)
  { prenom: 'Logen',    nom: 'Neuf-Doigts' },      // Abercrombie (britannique)
  { prenom: 'Aragorn',  nom: 'Grands-Pas' },       // Tolkien (britannique)
  { prenom: 'Jon',      nom: 'Shannow' },          // Gemmell (britannique)
  { prenom: 'Salim',    nom: 'Dhibi' },            // Bottero (français)
  { prenom: 'Bayaz',    nom: 'le Magi' },          // Abercrombie (britannique)
  { prenom: 'Merlin',   nom: "l'Enchanteur" },     // tradition arthurienne européenne
];

const HEROS_F = [
  { prenom: 'Lyra',     nom: 'Belacqua' },         // Pullman (britannique)
  { prenom: 'Hermione', nom: 'Granger' },           // Rowling (britannique)
  { prenom: 'Eowyn',    nom: 'du Rohan' },          // Tolkien (britannique)
  { prenom: 'Ellana',   nom: 'Caldin' },            // Bottero (français)
  { prenom: 'Ferro',    nom: 'Maljinn' },           // Abercrombie (britannique)
  { prenom: 'Magrat',   nom: 'Garlick' },           // Pratchett (britannique)
  { prenom: 'Ewilan',   nom: "Gil'Sayan" },         // Bottero (français)
  { prenom: 'Sigarni',  nom: 'la Guerrière' },      // Gemmell (britannique)
  { prenom: 'Rikke',    nom: 'la Nord' },           // Abercrombie (britannique)
  { prenom: 'Tanaquil', nom: 'la Magicienne' },     // Tanith Lee (britannique)
];

// Tirage pondéré de l'écart salarial F/H pour la session
// Distribution : 17→1/12, 16→2/12, 15→3/12, 14→3/12, 13→2/12, 11→1/12
const _ECART_POOL = [17, 16, 16, 15, 15, 15, 14, 14, 14, 13, 13, 11];
const _tauxEcart  = _ECART_POOL[Math.floor(Math.random() * _ECART_POOL.length)] / 100;

let _genre          = 'H';
let _nomPersonnalise = false;

function _heroRandom(list) {
  return list[Math.floor(Math.random() * list.length)];
}

function _setNomFields(prenom, nom) {
  ['d-prenom', 'm-prenom'].forEach(id => { const el = document.getElementById(id); if (el) el.value = prenom; });
  ['d-nom',    'm-nom'   ].forEach(id => { const el = document.getElementById(id); if (el) el.value = nom;    });
  _nomPersonnalise = false;
}

function _syncToggleUI(genre, showHint = false) {
  const onH = genre === 'H';
  ['d-hf-h', 'm-hf-h'].forEach(id => {
    document.getElementById(id)?.classList.toggle('ptog-on',  onH);
    document.getElementById(id)?.classList.toggle('ptog-off', !onH);
  });
  ['d-hf-f', 'm-hf-f'].forEach(id => {
    document.getElementById(id)?.classList.toggle('ptog-on',  !onH);
    document.getElementById(id)?.classList.toggle('ptog-off', onH);
  });
  if (showHint) {
    document.querySelectorAll('.genre-ecart-hint').forEach(el => {
      el.textContent = onH ? el.dataset.textHf : el.dataset.textFh;
      el.style.display = 'inline';
    });
  }
}

window.setGenre = function(genre) {
  if (genre === _genre) return;

  if (!_nomPersonnalise) {
    const hero = genre === 'F' ? window._heroF : window._heroH;
    _setNomFields(hero.prenom, hero.nom);
  }

  const facteur = genre === 'F' ? (1 - _tauxEcart) : (1 / (1 - _tauxEcart));
  ['d-brut', 'm-brut'].forEach(id => {
    const el = document.getElementById(id);
    if (el) el.value = Math.round(parseFloat(el.value) * facteur);
  });

  _genre = genre;
  _syncToggleUI(genre, true);
};

// ── Burger menu ───────────────────────────────────────────────────────────────
const burgerBtn  = document.getElementById('burger-btn');
const burgerMenu = document.getElementById('burger-menu');

function openBurger()  {
  burgerBtn.classList.add('open');
  burgerMenu.classList.add('open');
}
window.closeBurger = function() {
  burgerBtn.classList.remove('open');
  burgerMenu.classList.remove('open');
};

burgerBtn.addEventListener('click', e => {
  e.stopPropagation();
  burgerMenu.classList.contains('open') ? closeBurger() : openBurger();
});

// Ferme le menu sur clic en dehors
document.addEventListener('click', () => closeBurger());
// Empêche la fermeture immédiate sur clic à l'intérieur du menu
burgerMenu.addEventListener('click', e => e.stopPropagation());

