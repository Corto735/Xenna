import { trStatic, CAT_DICT, trCat, COUNTRY_DICT } from './lang.js';

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

// ── Ouverture URL externe (Tauri shell ou window.open en web) ────────────────
window.openExternal = async function(url) {
  if (window.__TAURI__) {
    const { open } = await import('@tauri-apps/plugin-shell');
    await open(url);
  } else {
    window.open(url, '_blank', 'noopener');
  }
};

// ── État global ──────────────────────────────────────────────────────────────
let lastBulletin = null;
let _etpPrev = 100; // ETP de référence pour le recalcul brut proportionnel

function _fmLastDay(year, month) {           // month 1-indexed
  return new Date(Date.UTC(year, month, 0)); // day=0 → dernier jour du mois précédent
}
const _now   = new Date();
const _year  = _now.getFullYear();
const _month = _now.getMonth() + 1;           // 1-indexed

const DATE_MIN   = '2015-01-01';
const DATE_TODAY = _fmLastDay(_year, _month).toISOString().slice(0, 10);
const _nextYear  = _month === 12 ? _year + 1 : _year;
const _nextMonth = _month === 12 ? 1 : _month + 1;
const DATE_MAX   = _fmLastDay(_nextYear, _nextMonth).toISOString().slice(0, 10);
const TODAY      = DATE_TODAY;  // alias rétrocompatible

// ── Liste des pays : par défaut, seuls les pays frontaliers de la France + l'Angleterre
// sont visibles ; les autres sont masqués derrière le lien « autres pays européens ». ──
const PAYS_EXTRA = ['autriche', 'bulgarie', 'chypre', 'croatie', 'danemark', 'estonie',
  'finlande', 'grece', 'hongrie', 'irlande', 'lettonie', 'lituanie', 'malte', 'paysbas',
  'pologne', 'portugal', 'roumanie', 'slovaquie', 'slovenie', 'suede', 'tchequie',
  'canada', 'quebec', 'japon', 'chine', 'coree', 'australie', 'nouvellezelande', 'etatsunis', 'mexique',
  'bresil', 'emirats', 'inde'];

// Tague (classe pays-extra) les pays non frontaliers + les groupes Amérique/Asie/Océanie.
function _setupPaysExtra() {
  ['d', 'm'].forEach(p => {
    const panel = document.getElementById(`${p}-params`);
    if (!panel) return;
    PAYS_EXTRA.forEach(id => {
      const item = document.getElementById(`${p}-${id}`)?.closest('label.param-item');
      if (item) item.classList.add('pays-extra');
    });
    panel.querySelectorAll('.param-group-title').forEach(t => {
      if (['Amérique', 'Asie', 'Océanie'].includes(t.textContent.trim())) t.classList.add('pays-extra');
    });
  });
}

let _paysAllShown = false;
window.togglePaysExtra = function() {
  _paysAllShown = !_paysAllShown;
  ['d', 'm'].forEach(p => {
    document.getElementById(`${p}-params`)?.classList.toggle('pays-all', _paysAllShown);
    const lk = document.getElementById(`${p}-pays-more`);
    if (lk) lk.textContent = _paysAllShown ? '− masquer les autres pays' : '+ autres pays';
  });
};

document.addEventListener("DOMContentLoaded", () => {
  ["d-date", "m-date"].forEach(id => {
    const el = document.getElementById(id);
    if (el) { el.value = DATE_TODAY; el.min = DATE_MIN; el.max = DATE_MAX; }
  });
  _setupPaysExtra();
  const yearEl = document.getElementById('a-annee');
  if (yearEl) {
    yearEl.max = String(_nextYear);
    if (parseInt(yearEl.value, 10) > _nextYear) yearEl.value = String(_nextYear);
  }
  document.addEventListener("keydown", e => { if (e.key === "Escape") closeFmModal(); });

  // Tirage unique à l'arrivée — héros + genre initial (H : 49 %, F : 51 %)
  window._heroH = Math.random() < 0.015 ? { prenom: 'Jean-Noël', nom: 'Favari' } : _heroRandom(HEROS_H);
  window._heroF = _heroRandom(HEROS_F);

  // Amorce la mémoire par sexe avec le tirage au sort
  _noms.H = { prenom: window._heroH.prenom, nom: window._heroH.nom };
  _noms.F = { prenom: window._heroF.prenom, nom: window._heroF.nom };

  if (Math.random() < 0.51) {
    // Préselection F — écart tiré une fois, mémorisé pour toute la session
    _ecartActif = _drawEcartPct();
    _ecartTire  = true;
    _genre      = 'F';
    _applyNoms('F');
    const _e0 = _ecartActif / 100;
    const _a0 = Math.abs(_ecartActif);
    const _b0 = Math.abs(Math.round(_e0 / (1 + _e0) * 100));
    document.querySelectorAll('.genre-ecart-hint').forEach(el => {
      if (_ecartActif < 0) {
        el.dataset.textFh = `// −${_a0} % · écart salarial F/H`;
        el.dataset.textHf = `// +${_b0} % · écart salarial H/F`;
      } else if (_ecartActif > 0) {
        el.dataset.textFh = `// +${_a0} % · avantage F/H`;
        el.dataset.textHf = `// −${_b0} % · avantage F/H`;
      } else {
        el.dataset.textFh = `// ± 0 % · parité salariale`;
        el.dataset.textHf = `// ± 0 % · parité salariale`;
      }
    });
    _syncToggleUI('F', false);
  } else {
    _applyNoms('H');
    _syncToggleUI('H');
  }

  // Déverrouillage JNF si tiré à l'arrivée
  _checkJNF();

  // Quand l'utilisateur tape manuellement, le toggle est désactivé + check JNF
  ['d-prenom', 'm-prenom', 'd-nom', 'm-nom'].forEach(id => {
    document.getElementById(id)?.addEventListener('input', () => {
      const prefix = id.startsWith('d-') ? 'd' : 'm';
      _captureNoms(_genre, prefix);   // saisie permanente, mémorisée pour le sexe courant
      _checkJNF();
      const prenom = (_noms[_genre].prenom || '').trim();
      const isLeeloo = prenom.toLowerCase() === 'leeloo';
      document.getElementById('burger-login')?.style && (document.getElementById('burger-login').style.display = isLeeloo ? '' : 'none');
      const bAdmin = document.getElementById('burger-admin');
      if (bAdmin) {
        bAdmin.style.display = isLeeloo ? '' : 'none';
        // URL admin encodée : un scraper qui greppe les chemins dans le
        // bundle ne la voit pas en clair (le préfixe vit dans admin/routes.rs).
        bAdmin.onclick = () => { window.location.href = atob('L2FyY2hpdmVzLWJhcmVtZS0xOTk3'); window.closeBurger(); };
      }
    });
  });

  // Les hints sont initialisés au premier basculement (pays inconnu à l'init)

  // Détection automatique mobile / bureau — breakpoint identique au media query CSS
  const mq = window.matchMedia("(max-width: 680px)");
  const applyView = e => {
    const body = document.body;
    if (!body.classList.contains("is-annuel")     &&
        !body.classList.contains("is-apropos")    &&
        !body.classList.contains("is-gaabrielle") &&
        !body.classList.contains("is-hercule")    &&
        !body.classList.contains("is-quizz")      &&
        !body.classList.contains("is-ecart")      &&
        !body.classList.contains("is-mecenat"))
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
  if (localStorage.getItem('xenna-dyslexia')) {
    document.body.classList.add('dyslexia-mode');
    document.getElementById('dyslexia-switch')?.classList.add('on');
  }

  if (localStorage.getItem('xenna-hv')) {
    document.getElementById('a11y-hv-btn')?.classList.add('active');
  }
  if (localStorage.getItem('xenna-bw')) {
    document.body.classList.add('bw-mode');
    document.getElementById('bw-switch')?.classList.add('on');
    document.getElementById('a11y-bw-btn')?.classList.add('active');
  }
  if (localStorage.getItem('xenna-dactylo')) {
    _dactyloMode = true;
    document.getElementById('dactylo-switch')?.classList.add('on');
  }

  // Animation wakeup des boutons flottants
  const _wakeupColors = ['#ff6b6b','#ffd93d','#6bcb77','#4d96ff','#ff922b','#cc5de8','#20c997','#f06595'];
  const _shuffled = [..._wakeupColors].sort(() => Math.random() - 0.5);
  document.querySelectorAll('.a11y-float-btn').forEach((btn, i) => {
    btn.style.setProperty('--wakeup-color', _shuffled[i % _shuffled.length]);
    btn.classList.add('wakeup');
    btn.addEventListener('animationend', () => btn.classList.remove('wakeup'), { once: true });
  });

  // Ferme le panel a11y au clic extérieur
  document.addEventListener('click', e => {
    if (!e.target.closest('#a11y-btn') && !e.target.closest('#a11y-panel')) {
      document.getElementById('a11y-panel')?.classList.remove('open');
      document.getElementById('a11y-btn')?.classList.remove('open');
    }
  });
});

// ── Bascule Brut / Net (paye inversée) ────────────────────────────────────────
// En mode « net », la valeur saisie est le NET souhaité AVANT impôt à la source ;
// le backend reconstitue le brut par dichotomie (netCible) et renvoie le bulletin
// complet calculé dessus.
let _modeSaisie = 'brut';   // 'brut' | 'net'
let _labelsSalaire = { brut: 'SALAIRE BRUT (€)', brutM: 'BRUT (€)' };

// Applique le libellé du champ salaire selon la devise (posée par onTogglePays)
// ET le mode de saisie (BRUT ↔ NET).
function _appliquerLabelSalaire() {
  const net  = _modeSaisie === 'net';
  const lab  = net ? _labelsSalaire.brut.replace('BRUT', 'NET')  : _labelsSalaire.brut;
  const labM = net ? _labelsSalaire.brutM.replace('BRUT', 'NET') : _labelsSalaire.brutM;
  const dBrut = document.getElementById('d-brut');
  if (dBrut) { const l = dBrut.closest('.field')?.querySelector('label'); if (l) l.textContent = lab; }
  const mBrut = document.getElementById('m-brut');
  if (mBrut) { const l = mBrut.closest('.field')?.querySelector('label'); if (l) l.textContent = labM; }
}

// Ligne « Brut reconstitué : X € » sous le champ (mode net uniquement).
function _afficherBrutReconstitue(bulletin) {
  ['d-brut-reconst', 'm-brut-reconst'].forEach(id => {
    const el = document.getElementById(id);
    if (!el) return;
    if (!bulletin || _modeSaisie !== 'net') { el.hidden = true; return; }
    const b = el.querySelector('b');
    if (b) b.textContent = `${parseFloat(bulletin.brut).toFixed(2)} ${bulletin.devise}`;
    el.hidden = false;
  });
}

function _syncBnUI() {
  const net = _modeSaisie === 'net';
  ['d-bn', 'm-bn'].forEach(id => {
    const el = document.getElementById(id);
    if (!el) return;
    el.classList.toggle('is-brut', !net);
    el.classList.toggle('is-net', net);
    el.setAttribute('aria-checked', net ? 'true' : 'false');
  });
  // Le seuil min (SMIC) du champ desktop n'a pas de sens pour une saisie de net.
  const dBrut = document.getElementById('d-brut');
  if (dBrut) {
    if (net) { dBrut.dataset.minBrut = dBrut.min || ''; dBrut.removeAttribute('min'); }
    else if (dBrut.dataset.minBrut) { dBrut.min = dBrut.dataset.minBrut; }
  }
  if (!net) _afficherBrutReconstitue(null);
  _appliquerLabelSalaire();
}

window.toggleBrutNet = function () {
  _modeSaisie = _modeSaisie === 'net' ? 'brut' : 'net';
  _syncBnUI();
};

// ── Accessibilité ────────────────────────────────────────────────────────────
// ── Traduction ────────────────────────────────────────────────────────────────
let _currentLang = 'fr';
const _tradCache  = {};     // { 'en': Map<original, translated> }
const _origTexts  = new Map(); // node → texte original
let _lastCalcReq  = null;   // dernière requête calculer_bulletin (pour re-calcul au changement de langue)

// Re-calcule le bulletin courant dans la langue demandée : les libellés et
// explications de cotisation sont traduits côté backend (crate::i18n) et
// reviennent déjà localisés. Sans bulletin affiché, no-op.
async function _recalcForLang(lang) {
  if (!_lastCalcReq || !lastBulletin) return;
  _lastCalcReq = { ..._lastCalcReq, lang };
  try {
    const b = await api("calculer_bulletin", _lastCalcReq);
    lastBulletin = b;
    renderAll(b);
  } catch (e) {
    console.error('[recalc langue] échec :', e);
  }
}

function _getTranslatableNodes() {
  // .trad-skip + classes dédiées au contenu cotisation fourni (déjà traduit)
  // par le backend : on ne les repasse PAS par MyMemory, sinon double traduction
  // qui massacre le texte juridique. Voir crate::i18n côté Rust.
  const SKIP = 'script,style,input,select,textarea,.mob-val,.sb-val,.fm-val,.a11y-float,.trad-panel,#a11y-panel'
    + ',.trad-skip,.expl-txt,.expl-ref,.mob-exp-txt,.mob-exp-loi,.mob-cot-lbl,.fm-fillon,.fm-chiffres,#fm-title';
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

  // Retour au français : re-calcule (cotisations en FR via backend) puis
  // restaure les textes statiques d'origine.
  if (lang === 'fr') {
    _currentLang = 'fr';
    await _recalcForLang('fr');
    _origTexts.forEach((orig, node) => { if (node.isConnected) node.textContent = orig; });
    document.documentElement.lang = 'fr';
    return;
  }

  const btn = document.getElementById('trad-btn');
  btn.classList.add('loading');
  btn.textContent = '🌐 …';

  // Re-calcule d'abord pour que libellés/explications de cotisation reviennent
  // déjà traduits du backend (nœuds marqués .trad-skip → ignorés ci-dessous).
  _currentLang = lang;
  await _recalcForLang(lang);

  const nodes = _getTranslatableNodes();

  // Sauvegarde les originaux (une seule fois)
  nodes.forEach(n => { if (!_origTexts.has(n)) _origTexts.set(n, n.textContent); });

  // Textes à traduire (originaux français)
  const texts = nodes.map(n => _origTexts.get(n));

  // Cache par langue
  if (!_tradCache[lang]) _tradCache[lang] = new Map();
  const cache = _tradCache[lang];

  // Dictionnaire statique — pré-remplit le cache sans appel réseau
  texts.forEach(orig => {
    const trimmed = orig.trim();
    const tr = trStatic(trimmed, lang);
    if (tr !== undefined && !cache.has(orig)) {
      cache.set(orig, tr);
    }
  });

  // Noms de pays / sous-régions — traductions curées (évite MyMemory).
  const _ci = { en: 0, de: 1, nl: 2, it: 3, es: 4 }[lang];
  if (_ci !== undefined) {
    texts.forEach(orig => {
      const trimmed = orig.trim();
      if (COUNTRY_DICT[trimmed] && !cache.has(orig)) {
        cache.set(orig, COUNTRY_DICT[trimmed][_ci]);
      }
    });
  }

  const toFetch   = [...new Set(texts)].filter(t => !cache.has(t));

  try {
    if (toFetch.length > 0) {
      // MyMemory API — gratuite, open, sans clé, ~1000 mots/jour
      // Utilisée uniquement pour les chaînes absentes du dictionnaire statique.
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

// ── Mode dyslexie — coloriage par caractère ───────────────────────────────────
const DYS_PALETTE = [
  '#ff6b6b', '#ff9f43', '#ffd43b', '#a9e34b',
  '#69db7c', '#38d9a9', '#4dabf7', '#748ffc',
  '#cc5de8', '#f783ac', '#ff8787', '#74c0fc',
];

let _dysIdx = 0;

function _dysEsc(ch) {
  if (ch === '&') return '&amp;';
  if (ch === '<') return '&lt;';
  if (ch === '>') return '&gt;';
  if (ch === '"') return '&quot;';
  return ch;
}

function _dysWrapTextNode(textNode) {
  const chars = [...textNode.textContent];
  let html = '';
  for (const ch of chars) {
    if (/\s/.test(ch)) { html += ch; continue; }
    const color = DYS_PALETTE[_dysIdx++ % DYS_PALETTE.length];
    html += `<span style="color:${color}">${_dysEsc(ch)}</span>`;
  }
  const wrapper = document.createElement('span');
  wrapper.className = 'dys-wrap';
  wrapper.innerHTML = html;
  textNode.parentNode.replaceChild(wrapper, textNode);
}

function applyDyslexiaColors() {
  if (!document.body.classList.contains('dyslexia-mode')) return;
  _dysIdx = 0;
  // Desktop : le libellé est dans le 3e span enfant de td:first-child
  // Mobile  : le texte est un nœud texte direct dans .mob-lbl
  const targets = [
    ...document.querySelectorAll('.ascii-tbl td:first-child > span:last-child'),
    ...document.querySelectorAll('.mob-lbl'),
  ];
  targets.forEach(el => {
    if (el.querySelector('.dys-wrap')) return;
    Array.from(el.childNodes)
      .filter(n => n.nodeType === Node.TEXT_NODE && n.textContent.trim())
      .forEach(_dysWrapTextNode);
  });
}

function removeDyslexiaColors() {
  document.querySelectorAll('.dys-wrap').forEach(w => {
    w.replaceWith(document.createTextNode(w.textContent));
  });
}

window.toggleDyslexia = function() {
  const active = document.body.classList.toggle('dyslexia-mode');
  document.getElementById('dyslexia-switch')?.classList.toggle('on', active);
  if (active) {
    applyDyslexiaColors();
    localStorage.setItem('xenna-dyslexia', '1');
  } else {
    removeDyslexiaColors();
    localStorage.removeItem('xenna-dyslexia');
  }
};

const _scan67clicks = [];
window.scan67 = function() {
  const now = Date.now();
  _scan67clicks.push(now);
  while (_scan67clicks.length && now - _scan67clicks[0] > 1500) _scan67clicks.shift();

  const easterEgg = _scan67clicks.length >= 3;
  if (easterEgg) _scan67clicks.length = 0;

  const pattern = easterEgg ? /42/ : /67/;
  const selectors = '.mob-val, .sb-val, .ascii-tbl td, .fm-val, .fm-result td';
  const found = Array.from(document.querySelectorAll(selectors))
    .filter(el => pattern.test(el.textContent.replace(/[\s ]/g, '')) && el.offsetParent !== null);

  if (found.length === 0) return;

  const btn = document.getElementById('a11y-67-btn');
  btn.classList.add('active');

  if (easterEgg) {
    const vividColors = ['#ff0055','#ff6600','#ffcc00','#00ff88','#00ccff','#aa00ff','#ff00cc','#39ff14','#ff4444','#44ffff','#ff69b4','#7fff00'];
    found.forEach((el, i) => {
      setTimeout(() => {
        const color = vividColors[Math.floor(Math.random() * vividColors.length)];
        Object.assign(el.style, { background: color, color: '#000', outline: `2px solid ${color}`, borderRadius: '2px', transition: 'all 0.15s' });
        setTimeout(() => Object.assign(el.style, { background: '', color: '', outline: '', borderRadius: '' }), 900);
      }, i * 250);
    });
    setTimeout(() => btn.classList.remove('active'), found.length * 250 + 1000);
  } else {
    found.forEach((el, i) => {
      setTimeout(() => {
        el.classList.remove('flash-67');
        void el.offsetWidth;
        el.classList.add('flash-67');
        el.addEventListener('animationend', () => el.classList.remove('flash-67'), { once: true });
      }, i * 500);
    });
    setTimeout(() => btn.classList.remove('active'), found.length * 500 + 200);
  }
};

window.toggleBWMode = function() {
  const active = document.body.classList.toggle('bw-mode');
  document.getElementById('bw-switch')?.classList.toggle('on', active);
  document.getElementById('a11y-bw-btn')?.classList.toggle('active', active);
  localStorage.setItem('xenna-bw', active ? '1' : '');
};

// ── Mode dactylo (desktop uniquement) ────────────────────────────────────────
let _dactyloMode  = false;
let _dactyloRunId = 0;
const _sleep = ms => new Promise(r => setTimeout(r, ms));

window.toggleDactylo = function() {
  _dactyloMode = !_dactyloMode;
  document.getElementById('dactylo-switch')?.classList.toggle('on', _dactyloMode);
  localStorage.setItem('xenna-dactylo', _dactyloMode ? '1' : '');
};

async function typewriterDesktop(b) {
  const id = ++_dactyloRunId;
  const abort = () => id !== _dactyloRunId;

  const container = document.getElementById('res-desktop');
  const rows = container.querySelectorAll('tr.data-row');
  if (!rows.length) return;

  // Easter egg ë — vitesse ×2, flash néon aléatoire, flammes sur les totaux
  const nomComplet = ((b?.salarie?.prenom || '') + (b?.salarie?.nom || '')).toLowerCase();
  const ee = nomComplet.includes('ë');
  const msL = ee ? 2 : 4;
  const msV = ee ? 1 : 2;
  const NEONS = ['#ff00ff','#00ffff','#ff0066','#66ff00','#ff6600','#0066ff','#ff00cc','#00ff99','#ffff00','#ff3399'];
  const flashColor = () => ee ? NEONS[Math.floor(Math.random() * NEONS.length)] : '#ffe066';

  // Phase 1 — masquer la ligne TOTAUX
  const totalRow = container.querySelector('tr.tbl-total');
  let totalSalCell = null, totalPatCell = null;
  let totalSalText = '', totalPatText = '';
  if (totalRow) {
    const cells = totalRow.querySelectorAll('td');
    totalSalCell = cells[1];
    totalPatCell = cells[3];
    if (totalSalCell) { totalSalText = totalSalCell.textContent; totalSalCell.textContent = ''; }
    if (totalPatCell) { totalPatText = totalPatCell.textContent; totalPatCell.textContent = ''; }
  }

  // Phase 2 — collecter et vider les cibles
  const queue = [];
  for (const row of rows) {
    const libelleSpan = row.querySelector('td:first-child > span:last-child');
    if (libelleSpan) {
      const text = libelleSpan.textContent;
      libelleSpan.textContent = '';
      queue.push({ target: libelleSpan, text, ms: msL });
    }
    const otherCells = [...row.querySelectorAll('td:not(:first-child)')];
    if (ee) {
      const parallelItems = [];
      otherCells.forEach((cell, i) => {
        const node = [...cell.childNodes].find(n => n.nodeType === 3 && n.textContent.trim());
        if (node) {
          const text = node.textContent;
          node.textContent = '';
          if (i === 0) queue.push({ target: node, text, ms: msV });
          else parallelItems.push({ target: node, text, ms: msV });
        }
      });
      if (parallelItems.length) queue.push({ parallel: parallelItems });
    } else {
      otherCells.forEach(cell => {
        const node = [...cell.childNodes].find(n => n.nodeType === 3 && n.textContent.trim());
        if (node) {
          const text = node.textContent;
          node.textContent = '';
          queue.push({ target: node, text, ms: msV });
        }
      });
    }
    queue.push({ pause: ee ? 4 : 8 });
  }

  // Phase 3 — frappe char par char
  for (const item of queue) {
    if (abort()) return;
    if (item.pause) { await _sleep(item.pause); continue; }
    if (item.parallel) {
      await Promise.all(item.parallel.map(async sub => {
        for (const char of sub.text) {
          if (abort()) return;
          sub.target.textContent += char;
          await _sleep(sub.ms);
        }
      }));
      continue;
    }
    for (const char of item.text) {
      if (abort()) return;
      item.target.textContent += char;
      await _sleep(item.ms);
    }
  }

  if (abort()) return;
  await _sleep(80);

  // Phase 4 — scan salarial
  for (const row of rows) {
    if (abort()) return;
    const cell = row.querySelectorAll('td')[3];
    if (cell?.classList.contains('c-sal')) {
      const c = flashColor();
      cell.style.background = c;
      cell.style.color = '#000';
      await _sleep(ee ? 65 : 110);
      cell.style.background = '';
      cell.style.color = '';
      await _sleep(ee ? 10 : 20);
    }
  }

  if (totalSalCell) {
    if (ee) { totalSalCell.style.fontWeight = 'bold'; totalSalCell.style.fontSize = '1.05em'; }
    for (const char of totalSalText) {
      if (abort()) return;
      totalSalCell.textContent += char;
      await _sleep(msV);
    }
    if (ee) spawnFlames(totalSalCell);
  }

  if (abort()) return;
  await _sleep(80);

  // Phase 5 — scan patronal
  for (const row of rows) {
    if (abort()) return;
    const cell = row.querySelectorAll('td')[5];
    if (cell?.classList.contains('c-pat')) {
      const c = flashColor();
      cell.style.background = c;
      cell.style.color = '#000';
      await _sleep(ee ? 65 : 110);
      cell.style.background = '';
      cell.style.color = '';
      await _sleep(ee ? 10 : 20);
    }
  }

  if (totalPatCell) {
    if (ee) { totalPatCell.style.fontWeight = 'bold'; totalPatCell.style.fontSize = '1.05em'; }
    for (const char of totalPatText) {
      if (abort()) return;
      totalPatCell.textContent += char;
      await _sleep(msV);
    }
    if (ee) spawnFlames(totalPatCell);
  }
}

let _flameStyleInjected = false;
function ensureFlameStyle() {
  if (_flameStyleInjected) return;
  _flameStyleInjected = true;
  const s = document.createElement('style');
  s.textContent = '@keyframes flameRise{0%{transform:translateY(0) scale(1);opacity:1}100%{transform:translateY(-42px) scale(0);opacity:0}}';
  document.head.appendChild(s);
}

function spawnFlames(cell) {
  ensureFlameStyle();
  const rect = cell.getBoundingClientRect();
  const wrap = document.createElement('div');
  wrap.style.cssText = 'position:fixed;pointer-events:none;z-index:9999;overflow:visible;'
    + `left:${rect.left}px;top:${rect.top}px;width:${rect.width}px;height:${rect.height}px`;
  document.body.appendChild(wrap);

  const FIRE = ['#ff8800','#ffdd00','#ff5500','#ffaa00','#ff3300','#ffcc00','#ff6600'];
  const iv = setInterval(() => {
    const p = document.createElement('div');
    const sz = Math.floor(Math.random() * 3) + 1;
    const dur = (0.5 + Math.random() * 1.2).toFixed(2);
    p.style.cssText = `position:absolute;pointer-events:none;`
      + `left:${(Math.random() * rect.width).toFixed(1)}px;bottom:0;`
      + `width:${sz}px;height:${sz}px;`
      + `background:${FIRE[Math.floor(Math.random() * FIRE.length)]};`
      + `animation:flameRise ${dur}s ease-out forwards`;
    wrap.appendChild(p);
    setTimeout(() => p.remove(), parseFloat(dur) * 1000 + 50);
  }, 35);

  setTimeout(() => {
    clearInterval(iv);
    setTimeout(() => wrap.remove(), 1400);
  }, 3000);
}

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
  ['mobile', 'desktop', 'annuel', 'apropos', 'carnet', 'contact', 'gaabrielle', 'hercule', 'quizz', 'mecenat', 'meliinda'].forEach(name =>
    document.body.classList.toggle('is-' + name, v === name)
  );
  document.getElementById("btn-desk").classList.toggle("active", v === "desktop");
  document.getElementById("btn-mob") .classList.toggle("active", v === "mobile");
  document.getElementById("btn-ann") .classList.toggle("active", v === "annuel");
  if (lastBulletin && (v === 'desktop' || v === 'mobile')) renderAll(lastBulletin);
  if (v === 'desktop' || v === 'mobile') _applyNoms(_genre);
  if (v === 'quizz')      quizzInit();
  if (v === 'gaabrielle') gaabInit();
  if (v === 'hercule')    herculeInit();
  if (v === 'apropos')  { _mecenatStart(); _humanInputLoad(); }
  if (v === 'carnet')     _carnetLoad();
  if (v === 'meliinda')   meliindaInit();
};

// ── Human input — posts publiés depuis Méliinda sur la page À propos ──────────
let _hiTimers = [];

function _hiStopReplay() { _hiTimers.forEach(clearTimeout); _hiTimers = []; }

function _hiBuildStates(evts) {
  const downs = evts.filter(e => e.type === 'down');
  let buffer = [], states = [];
  for (const ev of downs) {
    if (ev.key === 'Backspace') {
      for (let i = buffer.length - 1; i >= 0; i--) {
        // On horodate l'effacement : le caractère barré disparaîtra 3 s plus tard.
        if (!buffer[i].deleted) { buffer[i].deleted = true; buffer[i].deletedAt = ev.t; break; }
      }
    } else if (ev.key.length === 1 || ev.key === 'Enter') {
      buffer.push({ char: ev.key === 'Enter' ? '\n' : ev.key, deleted: false });
    }
    states.push({ t: ev.t, snapshot: buffer.map(c => ({ ...c })) });
  }
  return states;
}

function _hiReplay(events, stage) {
  _hiStopReplay();
  stage.style.display = 'block';
  stage.innerHTML = '<span class="ml-cursor"></span>';
  const states = _hiBuildStates(events);
  // Instants de rendu = chaque frappe + chaque échéance de disparition d'un ghost
  // (effacement + ML_GHOST_MS), pour faire disparaître le caractère barré même sans
  // frappe à cet instant.
  const times = new Set(states.map(s => s.t));
  const last = states[states.length - 1];
  if (last) for (const c of last.snapshot) if (c.deleted) times.add(c.deletedAt + ML_GHOST_MS);
  for (const t of [...times].sort((a, b) => a - b)) {
    // Base = état de la dernière frappe à ou avant t ; les ghosts expirés sont filtrés au rendu.
    let base = null;
    for (const s of states) { if (s.t <= t) base = s.snapshot; else break; }
    if (!base) continue;
    _hiTimers.push(setTimeout(() => mlRenderSnapshot(base, stage, t), t));
  }
}

// Les posts « human input » de la page À propos (destination 'apropos').
async function _humanInputLoad() { await _hiLoadInto('apropos-human-input', 'apropos'); }

// Les posts du Carnet de bord (destination 'carnet').
async function _carnetLoad() { await _hiLoadInto('carnet-posts', 'carnet'); }

// Charge tous les posts publiés, ne garde que ceux de la destination voulue, et
// rend chacun avec DEUX boutons : « human input » (rejoue les frappes en temps réel)
// et « affiche » (montre le message complet d'un seul coup). Le message est masqué
// tant que le visiteur n'a pas choisi l'un des deux modes.
async function _hiLoadInto(boxId, destination) {
  const box = document.getElementById(boxId);
  if (!box) return;
  try {
    const res  = await fetch('/api/apropos/posts');
    const data = await res.json();
    const posts = data.filter(p => (p.destination || 'apropos') === destination);
    if (!posts.length) {
      box.innerHTML = '<div class="hi-empty">Aucun message pour le moment.</div>';
      return;
    }
    box.innerHTML = posts.map((p, i) => `
      <div class="hi-post">
        <div class="hi-post-date">${new Date(p.created_at).toLocaleDateString('fr-FR', { year:'numeric', month:'long', day:'numeric' })}</div>
        <div class="hi-post-actions">
          <button class="hi-replay-btn" data-i="${i}">▶ human input</button>
          <button class="hi-show-btn"   data-i="${i}">affiche</button>
        </div>
        <div class="hi-post-text" id="${boxId}-text-${i}" style="display:none">${esc(p.contenu)}</div>
        <div class="hi-stage" id="${boxId}-stage-${i}"></div>
      </div>`).join('');
    // « human input » : rejoue les frappes en temps réel (avec effacements barrés).
    box.querySelectorAll('.hi-replay-btn').forEach(btn => {
      btn.addEventListener('click', () => {
        const i = btn.dataset.i;
        document.getElementById(`${boxId}-text-${i}`).style.display = 'none';
        _hiReplay(posts[i].events || [], document.getElementById(`${boxId}-stage-${i}`));
      });
    });
    // « affiche » : montre le message complet d'un coup, sans animation.
    box.querySelectorAll('.hi-show-btn').forEach(btn => {
      btn.addEventListener('click', () => {
        const i = btn.dataset.i;
        _hiStopReplay();
        const stage = document.getElementById(`${boxId}-stage-${i}`);
        stage.style.display = 'none';
        stage.innerHTML = '';
        document.getElementById(`${boxId}-text-${i}`).style.display = 'block';
      });
    });
  } catch {
    box.innerHTML = '<div class="hi-empty">Chargement impossible.</div>';
  }
}

// ── Mécénat — déverrouillage silencieux 15 s après le premier passage sur À propos ──
let _mecenatUnlocked = false;
function _mecenatStart() {
  if (_mecenatUnlocked) return;
  _mecenatUnlocked = true;
  setTimeout(() => {
    const el = document.getElementById('burger-mecenat');
    if (el) el.style.display = '';
  }, 15_000);
}

// Déverrouillage immédiat si Jean-Noël Favari est dans les champs nom/prénom
function _checkJNF() {
  const p = (document.getElementById('d-prenom')?.value || document.getElementById('m-prenom')?.value || '').trim().toLowerCase();
  const n = (document.getElementById('d-nom')?.value    || document.getElementById('m-nom')?.value    || '').trim().toLowerCase();
  if (p === 'jean-noël' && n === 'favari') {
    const el = document.getElementById('burger-mecenat');
    if (el) el.style.display = '';
  }
}

// ── Devise courante (mise à jour à chaque renderAll) ─────────────────────────
let DEVISE = "EUR";

// ── Formatage ────────────────────────────────────────────────────────────────
function devSym() {
  if (DEVISE === "CHF") return " CHF";
  if (DEVISE === "CAD") return " CAD";
  if (DEVISE === "GBP") return " £";
  if (DEVISE === "JPY") return " ¥";
  if (DEVISE === "CNY") return " ¥";
  if (DEVISE === "KRW") return " ₩";
  if (DEVISE === "AUD") return " $";
  if (DEVISE === "NZD") return " $";
  if (DEVISE === "PLN") return " zł";
  if (DEVISE === "DKK") return " kr";
  if (DEVISE === "RON") return " lei";
  if (DEVISE === "BGN") return " лв";
  if (DEVISE === "USD") return " $";
  if (DEVISE === "MXN") return " $";
  if (DEVISE === "BRL") return " R$";
  if (DEVISE === "AED") return " AED";
  if (DEVISE === "INR") return " ₹";
  return " €";
}
const _DEVISE_0DP = ["JPY", "CNY", "KRW"];
function fmt(val) {
  const n = parseFloat(val);
  const dp = _DEVISE_0DP.includes(DEVISE) ? 0 : 2;
  return n.toLocaleString("fr-FR", { minimumFractionDigits: dp, maximumFractionDigits: dp }) + devSym();
}
function fmtS(val, sign = false) {
  const n = parseFloat(val);
  const dp = _DEVISE_0DP.includes(DEVISE) ? 0 : 2;
  const s = n.toLocaleString("fr-FR", { minimumFractionDigits: dp, maximumFractionDigits: dp }) + devSym();
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
  "Aide à l'emploi":        "cat-alleg",
  "Heures supplémentaires": "cat-alleg",
  // Suisse
  "1er pilier":             "cat-ss",
  "Assurance chômage":      "cat-cho",
  "Assurance accidents":    "cat-acc",
  "Prévoyance maladie":     "cat-prev",
  "Prévoyance (LPP)":       "cat-ret",
  // Luxembourg
  "Assurance pension":          "cat-ret",
  "Assurance maladie":          "cat-ss",
  "Assurance dépendance":       "cat-prev",
  "Mutualité des employeurs":   "cat-ss",
  // Italie
  "Previdenza sociale":         "cat-ss",
  "Disoccupazione":             "cat-cho",
  "Assicurazione infortuni":    "cat-acc",
  "Fine rapporto":              "cat-prev",
  "Allegement":                 "cat-alleg",
  "Bonus IRPEF":                "cat-alleg",
  "Imposta":                    "cat-csg",
  "Imposta regionale":          "cat-csg",
  // Canada / Québec
  "Retraite fédérale":          "cat-ret",
  "Retraite Québec":            "cat-ret",
  "Chômage fédéral":            "cat-cho",
  "Parentalité Québec":         "cat-ss",
  "Santé Québec":               "cat-ss",
  "Impôt fédéral":              "cat-csg",
  "Impôt provincial":           "cat-csg",
  "Autres":                     "cat-prev",
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

window.togglePasDetail = function(expandId) {
  const panel = document.getElementById(expandId);
  if (!panel) return;
  const isOpen = panel.style.display !== 'none';
  panel.style.display = isOpen ? 'none' : 'block';
  const arrow = document.getElementById(expandId + '-arrow');
  if (arrow) arrow.textContent = isOpen ? '▶' : '▼';
};

// Décompose un calcul dans l'encadré noir en 3 temps successifs, pour lever
// toute ambiguïté : (1) la formule avec le nom de chaque terme, (2) les mêmes
// termes remplacés par leurs valeurs, (3) le regroupement chiffré jusqu'au
// résultat. `steps` = [{ label, sym, num, grp }] : une ou plusieurs
// sous-formules empilées (ex. SJB puis IJ puis IJSS). Les `=` sont alignés.
function fmDecomp(steps) {
  const w = Math.max(...steps.map(s => s.label.length));
  const body = steps.map(s => {
    const pad = ' '.repeat(w);
    return `${s.label.padEnd(w)}  =  ${s.sym}\n${pad}  =  ${s.num}\n${pad}  =  ${s.grp}`;
  }).join('\n\n');
  return `<div class="fm-chiffres">${body}</div>`;
}

function buildAidePosteFormulaContent(d) {
  const forfaitAnnuel  = parseFloat(d.forfait_annuel);
  const forfaitMensuel = parseFloat(d.forfait_mensuel);
  const etp            = parseFloat(d.etp);
  const etpRatio       = parseFloat(d.etp_ratio);
  const frac           = parseFloat(d.absent_fraction);
  const partAbsente    = parseFloat(d.part_absente);   // 30 % SMIC × part absente (pour 1 ETP)
  const aide           = parseFloat(d.aide);

  // Base mensuelle pour 1 ETP avant proratisation du temps de travail.
  const partTravaillee = forfaitMensuel * (1 - frac);
  const baseAvantEtp   = partTravaillee + partAbsente;
  // Décote due au temps partiel (ETP < 100 %).
  const decoteTp       = baseAvantEtp - aide;

  const rows = [];
  rows.push(`<tr><td>Forfait annuel (${esc(d.tranche)})</td><td class="fm-op">=</td><td class="fm-val c-base">${fmt(forfaitAnnuel)}</td></tr>`);
  rows.push(`<tr><td>÷ 12 (versement mensuel)</td><td class="fm-op">=</td><td class="fm-val c-base">${fmt(forfaitMensuel)}</td></tr>`);

  if (frac > 0) {
    rows.push(`<tr><td>dont part travaillée (× ${fmtPct(1 - frac)})</td><td class="fm-op">=</td><td class="fm-val">${fmt(partTravaillee)}</td></tr>`);
    rows.push(`<tr><td>dont part en arrêt (30 % SMIC × ${fmtPct(frac)})</td><td class="fm-op">+</td><td class="fm-val">${fmt(partAbsente)}</td></tr>`);
    rows.push(`<tr><td>Base mensuelle (pour 1 ETP)</td><td class="fm-op">=</td><td class="fm-val c-base">${fmt(baseAvantEtp)}</td></tr>`);
  }

  rows.push(`<tr><td>Quotité de travail — ETP ${fmtPct(etp / 100)}</td><td class="fm-op">×</td><td class="fm-val c-taux">${etpRatio.toLocaleString('fr-FR')}</td></tr>`);
  if (Math.abs(decoteTp) >= 0.005) {
    rows.push(`<tr><td>Décote temps partiel (ETP &lt; 100 %)</td><td class="fm-op">−</td><td class="fm-val c-sal">${fmt(decoteTp)}</td></tr>`);
  }
  rows.push(`<tr class="fm-result fm-sep"><td>Aide au poste mensuelle</td><td class="fm-op">=</td><td class="fm-val c-alleg">${fmt(aide)}</td></tr>`);

  const steps = [{
    label: 'Forfait mensuel',
    sym: `Forfait annuel  ÷  12`,
    num: `${fmt(forfaitAnnuel)}  ÷  12`,
    grp: `${fmt(forfaitMensuel)}`,
  }];
  if (frac > 0) {
    steps.push({
      label: 'Base mensuelle',
      sym: `Part travaillée  +  Part en arrêt (30 % SMIC)`,
      num: `${fmt(partTravaillee)}  +  ${fmt(partAbsente)}`,
      grp: `${fmt(baseAvantEtp)}`,
    });
  }
  steps.push({
    label: 'Aide au poste',
    sym: `Base mensuelle  ×  Quotité ETP`,
    num: `${fmt(baseAvantEtp)}  ×  ${etpRatio.toLocaleString('fr-FR')}`,
    grp: `${fmt(aide)}`,
  });
  return `
    <div class="fm-generic">Forfait annuel  ÷ 12  ×  ETP</div>
    ${fmDecomp(steps)}
    <div class="fm-base-note">Aide forfaitaire de l'État (ASP) versée à l'employeur d'une entreprise adaptée
    au titre d'un travailleur RQTH. Elle ne touche ni le brut ni le net du salarié&nbsp;:
    elle <b>vient en déduction du coût employeur</b> (ligne patronale négative).</div>
    <table class="fm-calc">${rows.join('')}</table>`;
}

// Réduction de cotisations salariales sur heures supp/compl : gain majoré × taux
// (somme vieillesse + retraite compl., plafonné à 11,31 %).
function buildReducSalHsFormula(d) {
  const gain    = parseFloat(d.gain_total);
  const taux    = parseFloat(d.taux_reduc);
  const plafond = parseFloat(d.plafond_taux);
  const reduc   = parseFloat(d.reduction);
  const plafonne = Math.abs(taux - plafond) < 1e-6;
  const rows = [];
  rows.push(`<tr><td>Rémunération HS/HC majorée</td><td class="fm-op">=</td><td class="fm-val c-base">${fmt(gain)}</td></tr>`);
  rows.push(`<tr><td>Taux (vieillesse + retraite compl.${plafonne ? ', plafonné' : ''})</td><td class="fm-op">×</td><td class="fm-val c-taux">${fmtPct(taux)}</td></tr>`);
  rows.push(`<tr class="fm-result fm-sep"><td>Réduction salariale</td><td class="fm-op">=</td><td class="fm-val c-alleg">+ ${fmt(reduc)}</td></tr>`);
  return `
    <div class="fm-generic">Rémunération HS/HC  ×  min(taux vieillesse, ${fmtPct(plafond)})</div>
    ${fmDecomp([{
      label: 'Réduction',
      sym: `Rémunération HS/HC  ×  Taux${plafonne ? ' (plafonné)' : ''}`,
      num: `${fmt(gain)}  ×  ${fmtPct(taux)}`,
      grp: `${fmt(reduc)}`,
    }])}
    <div class="fm-base-note">Réduction des cotisations salariales d'assurance vieillesse sur les heures
    supplémentaires/complémentaires (loi du 24/12/2018). Elle <b>vient en déduction des cotisations
    salariales</b> : elle augmente le net à payer, sans changer le brut.</div>
    <table class="fm-calc">${rows.join('')}</table>`;
}

// Déduction forfaitaire patronale : nombre d'heures supp × tarif (€/h selon effectif).
function buildDfpHsFormula(d) {
  const heures = parseFloat(d.heures_supp);
  const tarif  = parseFloat(d.tarif);
  const ded    = parseFloat(d.deduction);
  const rows = [];
  rows.push(`<tr><td>Heures supplémentaires</td><td class="fm-op">=</td><td class="fm-val c-base">${heures.toLocaleString('fr-FR')} h</td></tr>`);
  rows.push(`<tr><td>Tarif forfaitaire (effectif)</td><td class="fm-op">×</td><td class="fm-val c-taux">${fmt(tarif)} / h</td></tr>`);
  rows.push(`<tr class="fm-result fm-sep"><td>Déduction patronale</td><td class="fm-op">=</td><td class="fm-val c-alleg">− ${fmt(ded)}</td></tr>`);
  return `
    <div class="fm-generic">Heures supp.  ×  tarif (€/h)</div>
    ${fmDecomp([{
      label: 'Déduction',
      sym: `Heures supplémentaires  ×  Tarif forfaitaire`,
      num: `${heures.toLocaleString('fr-FR')} h  ×  ${fmt(tarif)} €/h`,
      grp: `${fmt(ded)}`,
    }])}
    <div class="fm-base-note">Déduction forfaitaire de cotisations patronales par heure supplémentaire
    (1,50 € si &lt; 20 salariés, 0,50 € à partir de 20). Elle <b>réduit le coût employeur</b>,
    sans effet sur le net du salarié.</div>
    <table class="fm-calc">${rows.join('')}</table>`;
}

// Lignes d'absence rendues dans la grille du tableau des cotisations (desktop) :
// libellé sur les 3 premières colonnes, montant dans la 4e (PART SALARIÉ),
// colonnes patronales vides → alignement natif sur la colonne salariale.
const _ABS_COLGROUP = `<colgroup><col><col style="width:13%"><col style="width:9%"><col style="width:13%"><col style="width:9%"><col style="width:13%"></colgroup>`;

function _absenceRow(label, cls, sign, amount, fmkey) {
  return `<tr>
      <td colspan="3">${label}</td>
      <td class="r ${cls}" style="cursor:pointer" onclick="showFormula('${fmkey}')">${sign} ${fmt(amount)}${buildFormulaStar(fmkey)}</td>
      <td colspan="2"></td>
    </tr>`;
}

function _absenceEmbedTable(rows) {
  return `<table class="ascii-tbl abs-embed">${_ABS_COLGROUP}<tbody>${rows}</tbody></table>`;
}

// ── Formules des lignes d'absence maladie (retenue, maintien, IJSS, garantie
// du net) — même squelette que les autres panneaux f(x). Toutes les valeurs
// intermédiaires viennent du backend (AbsenceResult) : raisonnement transparent.
function buildAbsenceFormulaContent(which, a, b) {
  const n = v => parseFloat(v) || 0;
  // Base des calculs d'absence = le brut mensuel plein réellement utilisé par
  // le backend : brut saisi, ou brut RECONSTITUÉ (« Brut ») si l'utilisateur a
  // saisi un net (mode paie inversée).
  const baseRef = n(a.brut_mensuel) || (n(a.retenue) * n(a.diviseur_retenue) / (a.jours_absence || 1));

  if (which === 'retenue') {
    const div = n(a.diviseur_retenue);
    return `
      <div class="fm-generic">Retenue  =  Brut mensuel  ×  Jours d'absence  ÷  Diviseur mensuel</div>
      ${fmDecomp([{
        label: 'Retenue',
        sym: `Brut mensuel  ×  Jours d'absence  ÷  Diviseur`,
        num: `${fmt(baseRef)}  ×  ${a.jours_absence}  ÷  ${div}`,
        grp: `${fmt(baseRef * a.jours_absence)}  ÷  ${div}  =  ${fmt(a.retenue)}`,
      }])}
      <div class="fm-base-note">Base : Brut mensuel plein${_modeSaisie === 'net' ? ' (reconstitué à partir du net saisi)' : ''}.
      Méthode : ${esc(a.libelle)} — le diviseur dépend de la méthode de décompte choisie (jours du mois, 26 ouvrables, 21,67 ouvrés ou jours réels).</div>
      <table class="fm-calc">
        <tr><td>Brut mensuel</td><td class="fm-op">=</td><td class="fm-val c-base">${fmt(baseRef)}</td></tr>
        <tr><td>Jours d'absence comptés</td><td class="fm-op">×</td><td class="fm-val c-taux">${a.jours_absence}</td></tr>
        <tr><td>Diviseur mensuel</td><td class="fm-op">÷</td><td class="fm-val c-taux">${n(a.diviseur_retenue)}</td></tr>
        <tr class="fm-result fm-sep"><td>Retenue absence</td><td class="fm-op">=</td><td class="fm-val c-sal">− ${fmt(a.retenue)}</td></tr>
      </table>`;
  }

  if (which === 'maintien') {
    const perDay = n(a.per_day_maintien);
    const m1 = Math.round(a.jours_maintien_t1 * n(a.taux_maintien_t1) * perDay * 100) / 100;
    const m2 = Math.round((n(a.maintien) - m1) * 100) / 100;
    const t2 = a.jours_maintien_t2 > 0;
    return `
      <div class="fm-generic">Maintien  =  Σ  Jours indemnisés  ×  Taux  ×  Salaire journalier perdu</div>
      ${fmDecomp([{
        label: 'Maintien',
        sym: `( Jours T1 × Taux T1${t2 ? ' + Jours T2 × Taux T2' : ''} )  ×  Salaire journalier perdu`,
        num: `( ${a.jours_maintien_t1} × ${fmtPct(a.taux_maintien_t1)}${t2 ? ` + ${a.jours_maintien_t2} × ${fmtPct(a.taux_maintien_t2)}` : ''} )  ×  ${fmt(perDay)}`,
        grp: `${fmt(m1)}${t2 ? `  +  ${fmt(m2)}` : ''}  =  ${fmt(a.maintien)}`,
      }])}
      <div class="fm-base-note">Régime : ${esc(a.convention)} — carence de ${a.carence_maintien} jours d'arrêt.
      ${a.am_local
        ? `Alsace-Moselle (droit local, art. L1226-23 du Code du travail, ex-art. 616 du code civil local) :
           100 % du salaire dès le 1er jour, sans carence ni condition d'ancienneté, pendant 6 semaines (42 jours),
           puis relais du droit commun. IJSS déduites (l'employeur complète jusqu'à 100 %).`
        : `Barème selon l'ancienneté : &lt; 1 an aucun maintien · 1 à 3 ans régime légal (90 % 30 j puis 66,66 % 30 j, carence 7 j) ·
           ≥ 3 ans régime conventionnel IDCC 0016 (100 % puis 75 % dès le 6e jour, périodes allongées à 5 et 10 ans d'ancienneté).`}</div>
      <table class="fm-calc">
        <tr><td>Salaire journalier perdu (retenue ÷ ${a.jours_absence} j)</td><td class="fm-op">=</td><td class="fm-val c-base">${fmt(perDay)}</td></tr>
        <tr><td>Tranche 1 : ${a.jours_maintien_t1} j × ${fmtPct(a.taux_maintien_t1)}</td><td class="fm-op">=</td><td class="fm-val c-taux">${fmt(m1)}</td></tr>
        ${a.jours_maintien_t2 > 0 ? `<tr><td>Tranche 2 : ${a.jours_maintien_t2} j × ${fmtPct(a.taux_maintien_t2)}</td><td class="fm-op">+</td><td class="fm-val c-taux">${fmt(m2)}</td></tr>` : ''}
        <tr class="fm-result fm-sep"><td>Maintien de salaire</td><td class="fm-op">=</td><td class="fm-val c-alleg">+ ${fmt(a.maintien)}</td></tr>
      </table>`;
  }

  if (which === 'ijss') {
    return `
      <div class="fm-generic">IJ  =  50 %  ×  SJB      avec      SJB  =  Salaire de référence  ×  3  ÷  91,25</div>
      ${fmDecomp([
        {
          label: 'SJB',
          sym: `Salaire de référence  ×  3  ÷  91,25`,
          num: `${fmt(a.salaire_ref_ijss)}  ×  3  ÷  91,25`,
          grp: `${fmt(n(a.salaire_ref_ijss) * 3)}  ÷  91,25  =  ${fmt(a.sjb)}`,
        },
        {
          label: 'IJ journalière',
          sym: `50 %  ×  SJB`,
          num: `50 %  ×  ${fmt(a.sjb)}`,
          grp: `${fmt(a.ijss_jour)}`,
        },
        {
          label: 'IJSS brutes',
          sym: `IJ journalière  ×  Jours indemnisés`,
          num: `${fmt(a.ijss_jour)}  ×  ${a.jours_ijss}`,
          grp: `${fmt(a.ijss_brut)}`,
        },
      ])}
      <div class="fm-base-note">Salaire de référence plafonné à ${n(a.coeff_plafond_ijss)} × SMIC mensuel (CSS art. R323-4).
      Carence Sécurité sociale : 3 jours calendaires — IJ versée par jour calendaire dès le 4e jour.</div>
      <table class="fm-calc">
        <tr><td>Salaire de référence (min(brut ; ${n(a.coeff_plafond_ijss)} × SMIC))</td><td class="fm-op">=</td><td class="fm-val c-base">${fmt(a.salaire_ref_ijss)}</td></tr>
        <tr><td>SJB (× 3 ÷ 91,25)</td><td class="fm-op">=</td><td class="fm-val c-taux">${fmt(a.sjb)}</td></tr>
        <tr><td>IJ journalière (50 %)</td><td class="fm-op">=</td><td class="fm-val c-taux">${fmt(a.ijss_jour)}</td></tr>
        <tr><td>Jours indemnisés (${a.jours_ijss + 3} cal. − 3 j de carence)</td><td class="fm-op">×</td><td class="fm-val c-taux">${a.jours_ijss}</td></tr>
        <tr class="fm-result fm-sep"><td>IJSS brutes</td><td class="fm-op">=</td><td class="fm-val c-sal">− ${fmt(a.ijss_brut)}</td></tr>
      </table>
      <div class="fm-base-note">Déduites du brut soumis à cotisations (subrogation : l'employeur les perçoit de la CPAM).
      La CPAM précompte CSG 6,2 % + CRDS 0,5 % : le salarié reçoit en bas de bulletin les IJSS NETTES
      = ${fmt(a.ijss_brut)} × 0,933 = ${fmt(a.ijss_net)}.</div>`;
  }

  if (which === 'ajustement') {
    const assiette = n(b?.brut);
    const netApresCotis = n(b?.net_a_payer) - n(a.ijss_net);
    return `
      <div class="fm-generic">Les IJSS brutes déduites du brut échappent aux cotisations salariales : sans correction,
      le net dépasserait celui du bulletin de référence. L'ajustement retenu en haut de bulletin est résolu
      par dichotomie (paie inversée, ≈ 60 itérations) pour que :  net(assiette)  +  IJSS nettes  =  Net de référence.</div>
      ${fmDecomp([{
        label: 'Ajustement',
        sym: `Assiette de référence  −  IJSS brutes  −  Assiette résolue`,
        num: `${fmt(a.assiette_ref)}  −  ${fmt(a.ijss_brut)}  −  ${fmt(assiette)}`,
        grp: `${fmt(n(a.assiette_ref) - n(a.ijss_brut))}  −  ${fmt(assiette)}  =  ${fmt(a.ajustement_net)}`,
      }])}
      <table class="fm-calc">
        <tr><td>Assiette de référence (salaire − retenue + maintien)</td><td class="fm-op">=</td><td class="fm-val c-base">${fmt(a.assiette_ref)}</td></tr>
        <tr><td>Net de référence = net(assiette de référence)</td><td class="fm-op">=</td><td class="fm-val c-taux">${fmt(a.net_cible)}</td></tr>
        <tr><td>Cible hors IJSS (net de référence − IJSS nettes)</td><td class="fm-op">=</td><td class="fm-val c-taux">${fmt(n(a.net_cible) - n(a.ijss_net))}</td></tr>
        <tr><td>Assiette résolue par dichotomie</td><td class="fm-op">=</td><td class="fm-val c-base">${fmt(assiette)}</td></tr>
        <tr class="fm-result fm-sep"><td>Ajustement (réf. − IJSS brutes − assiette)</td><td class="fm-op">=</td><td class="fm-val c-sal">− ${fmt(a.ajustement_net)}</td></tr>
      </table>
      <div class="fm-base-note">Vérification : net(assiette) ${fmt(netApresCotis)} + IJSS nettes ${fmt(a.ijss_net)} = ${fmt(n(b?.net_a_payer))} = net de référence — la subrogation est neutre pour le salarié.</div>`;
  }

  // which === 'reintegration' — IJSS NETTES réintégrées en bas de bulletin.
  const netApresCotis = n(b?.net_a_payer) - n(a.ijss_net);
  return `
    <div class="fm-generic">Subrogation : l'employeur perçoit les IJSS de la CPAM (nettes de CSG/CRDS déjà précomptées)
    et les reverse au salarié en bas de bulletin. En les ajoutant au net après cotisations, on obtient
    le NET À PAYER AVANT IMPÔT (le prélèvement à la source est ensuite retranché de ce montant).</div>
    ${fmDecomp([
      {
        label: 'IJSS nettes',
        sym: `IJSS brutes  ×  0,933   (− CSG 6,2 % − CRDS 0,5 %)`,
        num: `${fmt(a.ijss_brut)}  ×  0,933`,
        grp: `${fmt(a.ijss_net)}`,
      },
      {
        label: 'Net avant impôt',
        sym: `Net après cotisations  +  IJSS nettes`,
        num: `${fmt(netApresCotis)}  +  ${fmt(a.ijss_net)}`,
        grp: `${fmt(n(b?.net_a_payer))}`,
      },
    ])}
    <table class="fm-calc">
      <tr><td>IJSS brutes</td><td class="fm-op">=</td><td class="fm-val c-base">${fmt(a.ijss_brut)}</td></tr>
      <tr><td>Coefficient net (− CSG 6,2 % − CRDS 0,5 %)</td><td class="fm-op">×</td><td class="fm-val c-taux">0,933</td></tr>
      <tr><td>IJSS nettes reversées</td><td class="fm-op">=</td><td class="fm-val c-alleg">${fmt(a.ijss_net)}</td></tr>
      <tr><td>Net après cotisations (Brut − Cotisations)</td><td class="fm-op">+</td><td class="fm-val c-base">${fmt(netApresCotis)}</td></tr>
      <tr class="fm-result fm-sep"><td>Net à payer avant impôt</td><td class="fm-op">=</td><td class="fm-val c-alleg">${fmt(n(b?.net_a_payer))}</td></tr>
    </table>
    <div class="fm-base-note">Volet fiscal : IJSS imposables sur les 60 premiers jours d'arrêt → ${fmt(a.ijss_imposable)}
    intégrées au net imposable (base du prélèvement à la source).</div>`;
}

function buildFormulaContent(c, type) {
  // Fillon : l'explication contient déjà la formule complète avec valeurs substituées.
  if (c.code === 'REDUCTION_FILLON') {
    return `<pre class="fm-fillon">${esc(c.explication)}</pre>`;
  }

  // Aide au poste (entreprise adaptée) : forfait annuel ÷ 12, décote temps partiel
  // (ETP ≠ 100 %) et, le cas échéant, minoration des absences. L'aide est versée à
  // l'employeur et vient en déduction du coût employeur.
  if (c.code === 'AIDE_POSTE_EA' && c.aidePosteDetail) {
    return buildAidePosteFormulaContent(c.aidePosteDetail);
  }

  // Heures supplémentaires : réduction de cotisations salariales (gain × taux ≤ 11,31 %)
  // et déduction forfaitaire patronale (heures × tarif).
  if (c.code === 'REDUC_SAL_HS' && c.formulaDetail) {
    return buildReducSalHsFormula(c.formulaDetail);
  }
  if (c.code === 'DFP_HS' && c.formulaDetail) {
    return buildDfpHsFormula(c.formulaDetail);
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

  if (entry.type === 'absence') {
    const meta = {
      retenue:       ['Retenue pour absence maladie',        '── Retenue sur salaire ──────────────────────', 'fm-type-sal'],
      maintien:      ['Maintien de salaire employeur',       '── Indemnité complémentaire employeur ───────', 'fm-type-alleg'],
      ijss:          ['IJSS brutes (Sécurité sociale)',      '── Indemnités journalières — subrogation ────', 'fm-type-sal'],
      ajustement:    ['Ajustement du net (garantie du net)', '── Neutralisation du gain de cotisations ────', 'fm-type-sal'],
      reintegration: ['IJSS nettes réintégrées',             '── Subrogation — bas de bulletin ────────────', 'fm-type-alleg'],
    }[entry.which];
    document.getElementById('fm-title').textContent = meta[0];
    document.getElementById('fm-badge').textContent = meta[1];
    fmBody.className = meta[2];
    fmBody.innerHTML = buildAbsenceFormulaContent(entry.which, entry.a, lastBulletin);
    document.getElementById('fm-modal').classList.add('open');
    document.querySelectorAll(`[data-fmkey="${key}"]`).forEach(el => el.classList.add('visited'));
    return;
  }
  const { c, type } = entry;
  const isSal = type === 'sal';
  const badge = c.code === 'REDUCTION_FILLON'
    ? '── Allègement patronal ──────────────────────'
    : c.code === 'AIDE_POSTE_EA'
      ? '── Aide de l\'État (ASP) — coût employeur ────'
      : c.code === 'REDUC_SAL_HS'
        ? '── Exonération salariale — heures supp. ─────'
        : c.code === 'DFP_HS'
          ? '── Déduction patronale — heures supp. ───────'
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

window.openCommentFillon = function() {
  document.getElementById('fm-title').textContent = '± 0,01 € · Note';
  document.getElementById('fm-badge').textContent = '';
  document.getElementById('fm-body').innerHTML = `<div style="font-size:0.77rem;line-height:2;color:var(--text);font-style:italic;max-width:500px">
    Je le laisse parce que ça me fait un peu marrer.<br>
    Je me demande si quelqu'un leur a déjà envoyé des centimes en guise de protestation
    face à l'absurdité du calcul incroyable de la réduction Fillon…<br>
    et pour au final favoriser l'emploi au plus proche possible des minimas sociaux...<br>
    non sans une pointe de zèle ironique… je ne sais pas…
    <div style="margin-top:1.2rem;font-size:0.63rem;color:var(--dim);font-style:normal;letter-spacing:0.08em">[2026/05/04] [JNF]</div>
  </div>`;
  document.getElementById('fm-modal').classList.add('open');
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
  const skipPas  = ['suisse', 'luxembourg', 'italia', 'espagne', 'portugal', 'belgique', 'allemagne', 'canada', 'quebec', 'angleterre', 'japon', 'chine', 'pays_bas', 'australie', 'nouvelle_zelande', 'pologne', 'coree_du_sud', 'andorre', 'monaco', 'danemark', 'finlande', 'suede', 'estonie', 'lettonie', 'lituanie', 'autriche', 'tchequie', 'slovaquie', 'hongrie', 'slovenie', 'grece', 'chypre', 'malte', 'croatie', 'irlande', 'roumanie', 'bulgarie', 'etats_unis', 'mexique', 'bresil', 'emirats', 'inde'].includes(b.salarie?.pays);
  const isItalie = b.salarie?.pays === 'italia';
  const totalSal = cots.reduce((s, c) => s + parseFloat(c.montant_sal), 0);
  const totalPat = cots.reduce((s, c) => s + parseFloat(c.montant_pat), 0);
  const pas      = skipPas ? { total: 0, taux_effectif: 0 } : calculerPas(b.net_imposable);
  // IJSS NETTES réintégrées au net à payer (subrogation : la CPAM précompte
  // CSG/CRDS, l'employeur reverse le net) — déjà incluses dans net_a_payer côté
  // backend ; affichées ici à titre informatif.
  const ijssNet = b.absence && parseFloat(b.absence.ijss_net) > 0 ? parseFloat(b.absence.ijss_net) : 0;
  if (ijssNet > 0) _fmStore['ABS_IJSS_REINT'] = { type: 'absence', which: 'reintegration', a: b.absence };
  const netPayer = parseFloat(b.net_a_payer) - pas.total;
  if (!skipPas) _fmStore['PAS'] = { type: 'pas', netImposable: parseFloat(b.net_imposable) };

  // Section IJSS réintégrées — bas de bulletin, avant les allègements.
  const ijssReintSection = ijssNet > 0 ? `
    <div class="tbl-section-head">── IJSS — SUBROGATION ──────────────────────────────────────────────</div>
    <div class="rem-section" style="padding:0.3rem 0.9rem 0.4rem">
      ${_absenceEmbedTable(_absenceRow('IJSS nettes (subrogation) — reversées au salarié', 'c-green', '+', ijssNet, 'ABS_IJSS_REINT'))}
    </div>` : '';

  // IS suisse — extrait pour l'afficher séparément dans la barre récap
  const isChCot  = b.salarie?.pays === 'suisse' ? cots.find(c => c.code === 'CH_IS') : null;
  const isChAmt  = isChCot ? parseFloat(isChCot.montant_sal) : 0;
  const isChTaux = isChCot ? parseFloat(isChCot.taux_sal) : 0;
  if (isChCot) _fmStore['CH_IS'] = { c: isChCot, type: 'sal' };

  // IRPEF italienne — extraite pour la barre récap (comme IS suisse)
  const itIrpefCot  = isItalie ? cots.find(c => c.code === 'IT_IRPEF') : null;
  const itIrpefAmt  = itIrpefCot ? parseFloat(itIrpefCot.montant_sal) : 0;
  const itIrpefTaux = itIrpefCot ? parseFloat(itIrpefCot.taux_sal) : 0;
  // Bonus cuneo (montant négatif = avantage salarié)
  const itBonusCot  = isItalie ? cots.find(c => c.code === 'IT_BONUS_CUNEO') : null;
  const itBonusAmt  = itBonusCot ? parseFloat(itBonusCot.montant_sal) : 0;

  // Total sal affiché hors IS suisse et hors IRPEF/bonus italien
  const totalSalSansIS    = totalSal - isChAmt;
  const totalSalCotSeules = totalSal - isChAmt - itIrpefAmt - itBonusAmt;

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
            <span style="color:#ffe033">− ${fmt(isItalie ? totalSalCotSeules : totalSalSansIS)}</span>
          </div>
          ${isChCot ? `<div class="sb-ded-row">
            <span>Impôt à la source (${(isChTaux * 100).toFixed(1)} %)</span>
            <span class="fm-val" style="color:var(--purple);cursor:pointer" onclick="showFormula('CH_IS')">− ${fmt(isChAmt)}${buildFormulaStar('CH_IS')}</span>
          </div>` : ''}
          ${itIrpefCot ? `<div class="sb-ded-row">
            <span>IRPEF (${(itIrpefTaux * 100).toFixed(1)} % eff.)</span>
            <span style="color:var(--purple)">− ${fmt(itIrpefAmt)}</span>
          </div>` : ''}
          ${itBonusCot ? `<div class="sb-ded-row">
            <span>Bonus cuneo fiscale</span>
            <span style="color:var(--green)">+ ${fmt(Math.abs(itBonusAmt))}</span>
          </div>` : ''}
          ${(b.heures_sup && parseFloat(b.heures_sup.exo_fiscale) > 0) ? `<div class="sb-ded-row" style="opacity:0.85" title="Heures supp/compl exonérées d'impôt sur le revenu — plafond ${fmt(b.heures_sup.exo_plafond)} / an">
            <span>Dont HS/HC exonérées d'impôt</span>
            <span style="color:var(--green)">base PAS − ${fmt(b.heures_sup.exo_fiscale)}</span>
          </div>` : ''}
          ${!skipPas ? `<div class="sb-ded-row">
            <span>PAS (${(pas.taux_effectif * 100).toFixed(1)} %)</span>
            <span class="fm-val" style="color:var(--purple);cursor:pointer" onclick="showFormula('PAS')">− ${fmt(pas.total)}${buildFormulaStar('PAS')}</span>
          </div>` : ''}
          <div class="sb-ded-total">
            <span>Total retenues</span>
            <span style="color:#ffe033">− ${fmt(totalSal + pas.total)}</span>
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
        <div class="sb-val c-eblue">${fmt(parseFloat(b.brut) + totalPat)}</div>
      </div>
    </div>`;

  // ── Table cotisations salariales ──
  const cotAll      = cots.filter(c => !["Allègement", "Aide à l'emploi", "Heures supplémentaires"].includes(c.categorie) &&
    (parseFloat(c.montant_sal) > 0 || c.taux_sal !== "0" || parseFloat(c.montant_pat) > 0));
  const cotAlleg    = cots.filter(c => ["Allègement", "Aide à l'emploi", "Heures supplémentaires"].includes(c.categorie));
  const totalPatBrut = cotAll.reduce((s, c) => s + parseFloat(c.montant_pat), 0);
  // Total salarial des seules cotisations affichées dans ce tableau (la réduction
  // salariale HS figure dans la section allègements, pas ici).
  const totalSalCot  = cotAll.reduce((s, c) => s + parseFloat(c.montant_sal), 0);

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
            <span class="cat trad-skip ${catCls}">[${trCat(c.categorie, _currentLang)}]</span>
            <span class="trad-skip">${c.libelle}</span>
          </td>
          <td class="r">${fmt(c.base)}</td>
          <td class="r">${parseFloat(c.taux_sal) > 0 ? '− ' : ''}${fmtPct(c.taux_sal)}</td>
          <td class="r ${salCls}"${hasFmSal ? ` onclick="event.stopPropagation();showFormula('${keySal}')" style="cursor:pointer"` : ''}>${hasFmSal ? '− ' : ''}${fmt(c.montant_sal)}${starSal}</td>
          <td class="r">${parseFloat(c.taux_pat) > 0 ? '− ' : ''}${fmtPct(c.taux_pat)}</td>
          <td class="r ${patCls}"${hasFmPat ? ` onclick="event.stopPropagation();showFormula('${keyPat}')" style="cursor:pointer"` : ''}>${hasFmPat ? '− ' : ''}${fmt(c.montant_pat)}${starPat}</td>
        </tr>
        <tr class="expl-row" id="expl-${idx}" style="display:none">
          <td colspan="6">
            <div class="expl-box">
              <div class="expl-txt trad-skip">▸ ${esc(c.explication)}</div>
              ${c.loi_ref ? `<div class="expl-ref trad-skip">§ ${esc(c.loi_ref)}</div>` : ""}
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

  const tableAll = `
    <div class="tbl-section-head">── COTISATIONS ────────────────────────────────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${thead}
      <tbody>
        ${buildRows(cotAll, 0)}
        <tr class="tbl-total">
          <td colspan="3">TOTAUX</td>
          <td class="r c-sal">= − ${fmt(totalSalCot)}</td>
          <td></td>
          <td class="r c-pat">= − ${fmt(totalPatBrut)}</td>
        </tr>
      </tbody>
    </table>`;

  const simBanner = `<div class="sim-period">
    SIMULATION AU <span class="sp-accent">${formatDate(getDatePaie())}</span>
    &nbsp;·&nbsp; PMSS en vigueur calculé depuis la base de données sans le moindre état d'âme
  </div>`;

  // Section allègements/exonérations — montants négatifs affichés en économie.
  // Une ligne peut être patronale (Fillon, EA, DFP) ou salariale (réduction HS).
  const totalAllegPat = cotAlleg.reduce((s, c) => s + parseFloat(c.montant_pat), 0); // négatif
  const totalAllegSal = cotAlleg.reduce((s, c) => s + parseFloat(c.montant_sal), 0); // négatif
  const tableAlleg = cotAlleg.length === 0 ? "" : `
    <div class="tbl-section-head">── ALLÈGEMENTS & EXONÉRATIONS ──────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${thead}
      <tbody>
        ${cotAlleg.map((c, i) => {
          const idx    = cotAll.length + i;
          const catCls = CAT_CLASS[c.categorie] || "cat-alleg";
          const isSalSide = Math.abs(parseFloat(c.montant_sal)) > 0; // réduction salariale
          const montant = Math.abs(parseFloat(isSalSide ? c.montant_sal : c.montant_pat));
          const taux    = Math.abs(parseFloat(isSalSide ? c.taux_sal : c.taux_pat));
          const keyAlleg = `${c.code}_alleg`;
          _fmStore[keyAlleg] = { c, type: 'alleg' };
          const cellMontant = `<td class="r c-alleg" onclick="event.stopPropagation();showFormula('${keyAlleg}')" style="cursor:pointer">− ${fmt(montant)}${buildFormulaStar(keyAlleg)}</td>`;
          const cellTaux = `<td class="r c-alleg">${fmtPct(taux)}</td>`;
          return `
            <tr class="data-row" id="row-${idx}" onclick="toggleExpl(${idx})">
              <td>
                <span class="expand-icon">▶</span>
                <span class="cat trad-skip ${catCls}">[${trCat(c.categorie, _currentLang)}]</span>
                <span class="trad-skip">${c.libelle}</span>
              </td>
              <td class="r">${fmt(c.base)}</td>
              ${isSalSide ? `${cellTaux}${cellMontant}<td class="r"></td><td class="r"></td>`
                          : `<td class="r"></td><td class="r"></td>${cellTaux}${cellMontant}`}
            </tr>
            <tr class="expl-row" id="expl-${idx}" style="display:none">
              <td colspan="6">
                <div class="expl-box">
                  <div class="expl-txt">▸ ${esc(c.explication)}</div>
                  ${c.loi_ref ? `<div class="expl-ref">§ ${esc(c.loi_ref)}</div>` : ""}
                </div>
              </td>
            </tr>`;
        }).join("")}
        <tr class="tbl-total">
          <td colspan="3">TOTAL ALLÈGEMENTS & EXONÉRATIONS</td>
          <td class="r c-alleg">${totalAllegSal < 0 ? '+ ' + fmt(Math.abs(totalAllegSal)) : ''}</td>
          <td></td>
          <td class="r c-alleg">${totalAllegPat < 0 ? '− ' + fmt(Math.abs(totalAllegPat)) : ''}</td>
        </tr>
      </tbody>
    </table>`;

  el.innerHTML = simBanner + summaryBar
    + `<div id="rem-result-d">${buildRemSection()}</div>`
    + `<div class="tbl-wrap">${tableAll}${ijssReintSection}${tableAlleg}</div>`;
}

// ─── Accordéon mobile ───────────────────────────────────────────────────────
// panel : 'why' (explication + loi) | 'how' (formule de calcul)
window.mobToggle = function(id, panel) {
  const wrap = document.getElementById('mob-expand-' + id);
  if (!wrap) return;
  const ALL_PANELS = ['why', 'how', 'sal', 'pat'];
  const isOpen = wrap.style.display !== 'none';
  const cur    = wrap.dataset.panel;
  const show = p => {
    const el = document.getElementById(`mob-expand-${id}-${p}`);
    if (el) el.style.display = p === panel ? 'block' : 'none';
  };
  if (!isOpen) {
    wrap.style.display = 'block';
    wrap.dataset.panel = panel;
    ALL_PANELS.forEach(show);
  } else if (cur === panel) {
    wrap.style.display = 'none';
  } else {
    wrap.dataset.panel = panel;
    ALL_PANELS.forEach(show);
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

  const skipPas  = ['suisse', 'luxembourg', 'italia', 'espagne', 'portugal', 'belgique', 'allemagne', 'canada', 'quebec', 'angleterre', 'japon', 'chine', 'pays_bas', 'australie', 'nouvelle_zelande', 'pologne', 'coree_du_sud', 'andorre', 'monaco', 'danemark', 'finlande', 'suede', 'estonie', 'lettonie', 'lituanie', 'autriche', 'tchequie', 'slovaquie', 'hongrie', 'slovenie', 'grece', 'chypre', 'malte', 'croatie', 'irlande', 'roumanie', 'bulgarie', 'etats_unis', 'mexique', 'bresil', 'emirats', 'inde'].includes(b.salarie?.pays);
  const isItalieMob = b.salarie?.pays === 'italia';
  const totalSal  = cots.reduce((s, c) => s + parseFloat(c.montant_sal), 0);
  const totalPat  = cots.reduce((s, c) => s + parseFloat(c.montant_pat), 0);
  const pas       = skipPas ? { total: 0, taux_effectif: 0 } : calculerPas(b.net_imposable);
  // IJSS NETTES réintégrées au net à payer (subrogation : CSG/CRDS précomptées
  // par la CPAM) — déjà incluses dans net_a_payer côté backend ; informatif.
  const ijssNet   = b.absence && parseFloat(b.absence.ijss_net) > 0 ? parseFloat(b.absence.ijss_net) : 0;
  if (ijssNet > 0) _fmStore['ABS_IJSS_REINT'] = { type: 'absence', which: 'reintegration', a: b.absence };
  const netPayer  = parseFloat(b.net_a_payer) - pas.total;
  const superBrut = parseFloat(b.brut) + totalPat;

  // IS suisse — extrait pour l'afficher en accordéon dédié (comme PAS pour la France)
  const isChCot  = b.salarie?.pays === 'suisse' ? cots.find(c => c.code === 'CH_IS') : null;
  const isChAmt  = isChCot ? parseFloat(isChCot.montant_sal) : 0;
  const isChTaux = isChCot ? parseFloat(isChCot.taux_sal) : 0;

  // IRPEF italienne — extraite pour accordéon dédié
  const itIrpefCotMob = isItalieMob ? cots.find(c => c.code === 'IT_IRPEF') : null;
  const itIrpefAmtMob = itIrpefCotMob ? parseFloat(itIrpefCotMob.montant_sal) : 0;
  const itIrpefTauxMob = itIrpefCotMob ? parseFloat(itIrpefCotMob.taux_sal) : 0;
  const itBonusCotMob = isItalieMob ? cots.find(c => c.code === 'IT_BONUS_CUNEO') : null;
  const itBonusAmtMob = itBonusCotMob ? parseFloat(itBonusCotMob.montant_sal) : 0;

  const totalSalSansIS = totalSal - isChAmt - itIrpefAmtMob - itBonusAmtMob;

  // CH_IS, IT_IRPEF, IT_BONUS_CUNEO retirés de la liste — affichés séparément
  const cotAllMob    = cots.filter(c => !["Allègement", "Aide à l'emploi", "Heures supplémentaires"].includes(c.categorie) && c.code !== 'CH_IS'
    && c.code !== 'IT_IRPEF' && c.code !== 'IT_BONUS_CUNEO' &&
    (parseFloat(c.montant_sal) > 0 || c.taux_sal !== "0" || parseFloat(c.montant_pat) > 0));
  const cotAllegMob  = cots.filter(c => ["Allègement", "Aide à l'emploi", "Heures supplémentaires"].includes(c.categorie));
  const totalPatBrutMob = cotAllMob.reduce((s, c) => s + parseFloat(c.montant_pat), 0);
  // Total salarial des seules lignes affichées (la réduction HS est en allègements).
  const totalSalCotMob  = cotAllMob.reduce((s, c) => s + parseFloat(c.montant_sal), 0);
  const totalAlleg      = cotAllegMob.reduce((s, c) => s + parseFloat(c.montant_pat), 0); // négatif
  const totalAllegSalMob = cotAllegMob.reduce((s, c) => s + parseFloat(c.montant_sal), 0); // négatif

  const cotLines = cotAllMob.map((c, i) => {
    const hasSal   = parseFloat(c.montant_sal) > 0;
    const hasPat   = parseFloat(c.montant_pat) > 0;
    const expandId = `${c.code}_u`;
    const isFillon = c.code === 'REDUCTION_FILLON';
    const salFormula = hasSal
      ? (isFillon ? `<pre class="fm-fillon">${esc(c.explication)}</pre>` : `<div class="fm-type-sal">${buildFormulaContent(c, 'sal')}</div>`)
      : '';
    const patFormula = hasPat
      ? (isFillon ? `<pre class="fm-fillon">${esc(c.explication)}</pre>` : `<div class="fm-type-pat">${buildFormulaContent(c, 'pat')}</div>`)
      : '';
    const whyHtml = `
      <div class="mob-exp-txt">${esc(c.explication)}</div>
      ${c.loi_ref ? `<div class="mob-exp-loi">§ ${esc(c.loi_ref)}</div>` : ''}`;
    const stripeCls = `mob-stripe-sal-${i % 2 === 0 ? 'a' : 'b'}`;
    const amtsSal = hasSal
      ? `<span class="mob-val mob-cot-amt" style="color:#ffe033" onclick="mobToggle('${expandId}','sal')">− ${fmt(c.montant_sal)}</span>`
      : `<span class="mob-val c-dim">0 ${devSym().trim()}</span>`;
    const amtsPat = hasPat
      ? `<span class="mob-val c-orange mob-cot-amt" onclick="mobToggle('${expandId}','pat')">− ${fmt(c.montant_pat)}</span>`
      : `<span class="mob-val c-dim">0 ${devSym().trim()}</span>`;
    return `
      <div class="${stripeCls}">
        <div class="mob-row">
          <span class="mob-lbl mob-cot-lbl" onclick="mobToggle('${expandId}','why')">${esc(c.libelle)}</span>
          <span style="display:flex;flex-direction:column;align-items:flex-end;gap:0.1rem">${amtsSal}${amtsPat}</span>
        </div>
        <div class="mob-expand" id="mob-expand-${expandId}" style="display:none">
          <div id="mob-expand-${expandId}-why">${whyHtml}</div>
          ${salFormula ? `<div id="mob-expand-${expandId}-sal" style="display:none">${salFormula}</div>` : ''}
          ${patFormula ? `<div id="mob-expand-${expandId}-pat" style="display:none">${patFormula}</div>` : ''}
        </div>
      </div>`;
  }).join('');

  const cotAllegLines = cotAllegMob
    .map((c, i) => {
      // Réduction salariale (HS) = crédit côté salarié (+) ; autres = économie patronale (−).
      const isSalSide = Math.abs(parseFloat(c.montant_sal)) > 0;
      const montant = Math.abs(parseFloat(isSalSide ? c.montant_sal : c.montant_pat));
      const mHtml = `${isSalSide ? '+' : '−'} ${fmt(montant)}`;
      return buildMobCotRow(c, `${c.code}_alleg`, mHtml, 'c-alleg', isSalSide ? 'sal' : 'alleg', i);
    })
    .join('');

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

      <!-- Rémunération -->
      <div id="rem-result-m">${buildRemSectionMobile()}</div>

      <!-- Cotisations unifiées (salariales + patronales sur une ligne) -->
      <div class="mob-row section"><span class="mob-lbl">── COTISATIONS ──</span><span style="display:flex;gap:0.75rem"><span class="mob-badge mob-badge-sal">Sal.</span><span class="mob-badge mob-badge-pat">Pat.</span></span></div>
      ${cotLines}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL cotisations salariales</span>
        <span class="mob-val c-yellow">− ${fmt(totalSalCotMob)}</span>
      </div>
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL charges patronales</span>
        <span class="mob-val c-orange">− ${fmt(totalPatBrutMob)}</span>
      </div>

      <!-- Impôt à la source suisse — accordéon dédié -->
      ${isChCot ? `<div class="mob-row pas-row" style="cursor:pointer" onclick="togglePasDetail('is-detail-mob')">
        <span class="mob-lbl">Impôt à la source (${(isChTaux * 100).toFixed(1)} %) <span id="is-detail-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-purple">− ${fmt(isChAmt)}</span>
      </div>
      <div id="is-detail-mob" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        <div class="fm-type-sal">${buildFormulaContent(isChCot, 'sal')}</div>
        <div class="mob-exp-txt" style="margin-top:0.5rem">${esc(isChCot.explication)}</div>
        ${isChCot.loi_ref ? `<div class="mob-exp-loi">§ ${esc(isChCot.loi_ref)}</div>` : ''}
      </div>` : ''}

      <!-- Net imposable (France / FPT) -->
      ${!skipPas ? `<div class="mob-row net-row">
        <span class="mob-lbl">NET IMPOSABLE</span>
        <span class="mob-val c-green">${fmt(b.net_imposable)}</span>
      </div>` : ''}
      ${(b.heures_sup && parseFloat(b.heures_sup.exo_fiscale) > 0) ? `<div class="mob-row" style="opacity:0.85">
        <span class="mob-lbl">dont HS/HC exonérées d'impôt (plafond ${fmt(b.heures_sup.exo_plafond)}/an)</span>
        <span class="mob-val c-green">− ${fmt(b.heures_sup.exo_fiscale)}</span>
      </div>` : ''}

      <!-- PAS (France / FPT) -->
      ${!skipPas ? `<div class="mob-row pas-row" style="cursor:pointer" onclick="togglePasDetail('pas-detail-mob')">
        <span class="mob-lbl">Prélèvement à la source (${(pas.taux_effectif * 100).toFixed(1)} %) <span id="pas-detail-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-purple">− ${fmt(pas.total)}</span>
      </div>
      <div id="pas-detail-mob" class="fm-type-pas" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        ${buildPasFormulaContent(parseFloat(b.net_imposable))}
      </div>` : ''}

      <!-- IRPEF italienne -->
      ${itIrpefCotMob ? `<div class="mob-row pas-row" style="cursor:pointer" onclick="togglePasDetail('irpef-detail-mob')">
        <span class="mob-lbl">IRPEF (${(itIrpefTauxMob * 100).toFixed(1)} % eff.) <span id="irpef-detail-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-purple">− ${fmt(itIrpefAmtMob)}</span>
      </div>
      <div id="irpef-detail-mob" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        <div class="mob-exp-txt">${esc(itIrpefCotMob.explication)}</div>
        ${itIrpefCotMob.loi_ref ? `<div class="mob-exp-loi">§ ${esc(itIrpefCotMob.loi_ref)}</div>` : ''}
      </div>` : ''}

      <!-- Bonus cuneo fiscale -->
      ${itBonusCotMob ? `<div class="mob-row" style="cursor:pointer" onclick="togglePasDetail('bonus-cuneo-mob')">
        <span class="mob-lbl">Bonus cuneo fiscale <span id="bonus-cuneo-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-green">+ ${fmt(Math.abs(itBonusAmtMob))}</span>
      </div>
      <div id="bonus-cuneo-mob" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        <div class="mob-exp-txt">${esc(itBonusCotMob.explication)}</div>
        ${itBonusCotMob.loi_ref ? `<div class="mob-exp-loi">§ ${esc(itBonusCotMob.loi_ref)}</div>` : ''}
      </div>` : ''}

      <!-- IJSS nettes (subrogation) -->
      ${ijssNet > 0 ? `<div class="mob-row">
        <span class="mob-lbl">IJSS nettes (subrogation)</span>
        <span class="mob-val c-green" style="cursor:pointer" onclick="showFormula('ABS_IJSS_REINT')">+ ${fmt(ijssNet)}${buildFormulaStar('ABS_IJSS_REINT')}</span>
      </div>` : ''}

      <!-- Net à payer -->
      <div class="mob-row final-row">
        <span class="mob-lbl">NET À PAYER</span>
        <span class="mob-val c-green">${fmt(netPayer)}</span>
      </div>

      <!-- Allègements & exonérations -->
      ${cotAllegLines.length ? `
      <div class="mob-row section"><span class="mob-lbl">── ALLÈGEMENTS & EXONÉRATIONS ──</span><span></span></div>
      ${cotAllegLines}
      ${totalAllegSalMob < 0 ? `<div class="mob-row subtot">
        <span class="mob-lbl">dont réduction salariale (→ net)</span>
        <span class="mob-val c-alleg">+ ${fmt(Math.abs(totalAllegSalMob))}</span>
      </div>` : ''}
      ${totalAlleg < 0 ? `<div class="mob-row subtot">
        <span class="mob-lbl">TOTAL allègements patronaux</span>
        <span class="mob-val c-alleg">− ${fmt(Math.abs(totalAlleg))}</span>
      </div>` : ''}` : ""}

      <!-- Super brut -->
      <div class="mob-row superbrut">
        <span class="mob-lbl">SUPER BRUT (coût employeur)</span>
        <span class="mob-val c-eblue">${fmt(superBrut)}</span>
      </div>

    </div>`;
}

// ═════════════════════════════════════════════════════════════════════════════
// RENDU GLOBAL (les deux vues)
// ═════════════════════════════════════════════════════════════════════════════
// Détache le bloc de détail « aide au poste » (préfixé par U+0001 côté backend)
// du texte d'explication : `c.explication` redevient propre pour l'affichage,
// et `c.aidePosteDetail` porte les valeurs chiffrées pour le modal de formule.
function extractAidePosteDetail(b) {
  (b.cotisations || []).forEach(c => {
    const i = typeof c.explication === "string" ? c.explication.indexOf("\u0001") : -1;
    if (i === -1) return;
    const raw = c.explication.slice(i + 1);
    c.explication = c.explication.slice(0, i).trim();
    let parsed = null;
    try { parsed = JSON.parse(raw); } catch { /* bloc illisible : on l'ignore */ }
    if (!parsed) return;
    if (c.code === 'AIDE_POSTE_EA') c.aidePosteDetail = parsed;
    else c.formulaDetail = parsed; // réduction salariale HS, DFP…
  });
}

function renderAll(b) {
  DEVISE = b.devise || "EUR";
  extractAidePosteDetail(b);
  renderDesktop(b);
  renderMobile(b);
  if (_dactyloMode) {
    typewriterDesktop(b).then(() => applyDyslexiaColors());
  } else {
    applyDyslexiaColors();
  }
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
  let date           = document.getElementById(isM ? "m-date"   : "d-date").value  || TODAY;
  const alsaceMoselle  = document.getElementById(isM ? "m-alsace-moselle" : "d-alsace-moselle")?.checked ?? false;
  const isSuisse       = document.getElementById(isM ? "m-suisse"      : "d-suisse")?.checked ?? false;
  const isLuxembourg   = document.getElementById(isM ? "m-luxembourg"  : "d-luxembourg")?.checked ?? false;
  const isFPT          = document.getElementById(isM ? "m-fpt"         : "d-fpt")?.checked ?? false;
  const isEA           = document.getElementById(isM ? "m-ea"          : "d-ea")?.checked ?? false;
  const eaTranche      = document.getElementById(isM ? "m-ea-tranche"  : "d-ea-tranche")?.value || "m50";
  const isItalie       = document.getElementById(isM ? "m-italie"      : "d-italie")?.checked ?? false;
  const isEspagne      = document.getElementById(isM ? "m-espagne"     : "d-espagne")?.checked ?? false;
  const isPortugal     = document.getElementById(isM ? "m-portugal"    : "d-portugal")?.checked ?? false;
  const isBelgique     = document.getElementById(isM ? "m-belgique"    : "d-belgique")?.checked ?? false;
  const beRegion       = document.getElementById(isM ? "m-be-region"   : "d-be-region")?.value || "bruxelles";
  const isAllemagne    = document.getElementById(isM ? "m-allemagne"   : "d-allemagne")?.checked ?? false;
  const isAngleterre   = document.getElementById(isM ? "m-angleterre"  : "d-angleterre")?.checked ?? false;
  const isJapon        = document.getElementById(isM ? "m-japon"       : "d-japon")?.checked ?? false;
  const isChine        = document.getElementById(isM ? "m-chine"       : "d-chine")?.checked ?? false;
  const isPaysBas      = document.getElementById(isM ? "m-paysbas"     : "d-paysbas")?.checked ?? false;
  const isAustralie    = document.getElementById(isM ? "m-australie"   : "d-australie")?.checked ?? false;
  const isNouvelleZl   = document.getElementById(isM ? "m-nouvellezelande" : "d-nouvellezelande")?.checked ?? false;
  const isPologne      = document.getElementById(isM ? "m-pologne"     : "d-pologne")?.checked ?? false;
  const isCoree        = document.getElementById(isM ? "m-coree"       : "d-coree")?.checked ?? false;
  const isAndorre      = document.getElementById(isM ? "m-andorre"     : "d-andorre")?.checked ?? false;
  const isMonaco       = document.getElementById(isM ? "m-monaco"      : "d-monaco")?.checked ?? false;
  const isDanemark     = document.getElementById(isM ? "m-danemark"    : "d-danemark")?.checked ?? false;
  const isFinlande     = document.getElementById(isM ? "m-finlande"    : "d-finlande")?.checked ?? false;
  const isSuede        = document.getElementById(isM ? "m-suede"       : "d-suede")?.checked ?? false;
  const isEstonie      = document.getElementById(isM ? "m-estonie"     : "d-estonie")?.checked ?? false;
  const isLettonie     = document.getElementById(isM ? "m-lettonie"    : "d-lettonie")?.checked ?? false;
  const isLituanie     = document.getElementById(isM ? "m-lituanie"    : "d-lituanie")?.checked ?? false;
  const isAutriche     = document.getElementById(isM ? "m-autriche"    : "d-autriche")?.checked ?? false;
  const isTchequie     = document.getElementById(isM ? "m-tchequie"    : "d-tchequie")?.checked ?? false;
  const isSlovaquie    = document.getElementById(isM ? "m-slovaquie"   : "d-slovaquie")?.checked ?? false;
  const isHongrie      = document.getElementById(isM ? "m-hongrie"     : "d-hongrie")?.checked ?? false;
  const isSlovenie     = document.getElementById(isM ? "m-slovenie"    : "d-slovenie")?.checked ?? false;
  const isGrece        = document.getElementById(isM ? "m-grece"       : "d-grece")?.checked ?? false;
  const isChypre       = document.getElementById(isM ? "m-chypre"      : "d-chypre")?.checked ?? false;
  const isMalte        = document.getElementById(isM ? "m-malte"       : "d-malte")?.checked ?? false;
  const isCroatie      = document.getElementById(isM ? "m-croatie"     : "d-croatie")?.checked ?? false;
  const isIrlande      = document.getElementById(isM ? "m-irlande"     : "d-irlande")?.checked ?? false;
  const isRoumanie     = document.getElementById(isM ? "m-roumanie"    : "d-roumanie")?.checked ?? false;
  const isBulgarie     = document.getElementById(isM ? "m-bulgarie"    : "d-bulgarie")?.checked ?? false;
  const isCanada       = document.getElementById(isM ? "m-canada"      : "d-canada")?.checked ?? false;
  const isQuebec       = document.getElementById(isM ? "m-quebec"      : "d-quebec")?.checked ?? false;
  const caProvince     = document.getElementById(isM ? "m-ca-province" : "d-ca-province")?.value || "ON";
  const isEtatsUnis    = document.getElementById(isM ? "m-etatsunis"   : "d-etatsunis")?.checked ?? false;
  const isMexique      = document.getElementById(isM ? "m-mexique"     : "d-mexique")?.checked ?? false;
  const isBresil       = document.getElementById(isM ? "m-bresil"      : "d-bresil")?.checked ?? false;
  const isEmirats      = document.getElementById(isM ? "m-emirats"     : "d-emirats")?.checked ?? false;
  const isInde         = document.getElementById(isM ? "m-inde"        : "d-inde")?.checked ?? false;
  const emiratiNational= document.getElementById(isM ? "m-emirati-national" : "d-emirati-national")?.checked ?? false;
  const indeRegime     = document.getElementById(isM ? "m-inde-regime" : "d-inde-regime")?.value || "nouveau";
  const usState        = document.getElementById(isM ? "m-us-state"    : "d-us-state")?.value || "TX";
  const steuerklasse   = document.getElementById(isM ? "m-steuerklasse"    : "d-steuerklasse")?.value || "1";
  const kinderlos      = document.getElementById(isM ? "m-kinderlos"       : "d-kinderlos")?.checked ?? false;
  const kirchenmitglied= document.getElementById(isM ? "m-kirchenmitglied" : "d-kirchenmitglied")?.checked ?? false;
  const deLand         = document.getElementById(isM ? "m-land"            : "d-land")?.value || "NW";
  const assujettiIS    = document.getElementById(isM ? "m-assujetti-is" : "d-assujetti-is")?.checked ?? false;
  const canton         = document.getElementById(isM ? "m-canton"       : "d-canton")?.value || null;
  const tarifIs        = document.getElementById(isM ? "m-tarif-is"     : "d-tarif-is")?.value || null;
  const effectif       = document.getElementById(isM ? "m-effectif"     : "d-effectif")?.value || "moins20";
  // Ancienneté (années entières 0-100) — conditionne le maintien de salaire maladie.
  const ancienneteRaw  = parseInt(document.getElementById(isM ? "m-anciennete" : "d-anciennete")?.value ?? "1", 10);
  const anciennete     = isNaN(ancienneteRaw) ? 1 : Math.min(100, Math.max(0, ancienneteRaw));
  const remHeures      = getRemHeures();

  // ── Validation côté JS ────────────────────────────────────────────────────
  // Si brut est vide ou non numérique, input[type="number"] retourne "".
  // Envoyer "" à Rust provoque une erreur de désérialisation Tauri muette.
  const brutVal  = parseFloat(brut);
  _remBase = brutVal || 0;
  const totalBrut = getRemTotal();
  if (!brut || isNaN(brutVal) || brutVal <= 0) {
    showInputError(_modeSaisie === 'net'
      ? "Salaire net invalide — saisir un montant positif."
      : "Salaire brut invalide — saisir un montant positif.");
    return;
  }
  // La date est forcée à TODAY si vide, mais on vérifie le format ISO au cas où.
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date)) {
    showInputError(`Date invalide : '${date}' (format attendu : YYYY-MM-DD).`);
    return;
  }
  if (date > DATE_MAX) {
    date = DATE_MAX;
    ["d-date", "m-date"].forEach(id => { const e = document.getElementById(id); if (e) e.value = DATE_MAX; });
  }

  // Sync les deux formulaires
  ["d-brut","m-brut"].forEach(id => { const e = document.getElementById(id); if(e) e.value = brut; });
  ["d-statut","m-statut"].forEach(id => { const e = document.getElementById(id); if(e) e.value = statut; });
  ["d-nom","m-nom"].forEach(id => { const e = document.getElementById(id); if(e) e.value = nom; });
  ["d-prenom","m-prenom"].forEach(id => { const e = document.getElementById(id); if(e) e.value = prenom; });
  ["d-date","m-date"].forEach(id => { const e = document.getElementById(id); if(e) e.value = date; });

  const paysEtranger = isSuisse ? "suisse" : isLuxembourg ? "luxembourg"
    : isItalie ? "italia" : isEspagne ? "espagne" : isPortugal ? "portugal"
    : isEtatsUnis ? "etats_unis"
    : isMexique ? "mexique"
    : isBresil ? "bresil"
    : isEmirats ? "emirats"
    : isInde ? "inde"
    : isBelgique ? "belgique" : isAllemagne ? "allemagne" : isCanada ? "canada" : isQuebec ? "quebec"
    : isAngleterre ? "angleterre" : isJapon ? "japon" : isChine ? "chine"
    : isPaysBas ? "pays_bas" : isAustralie ? "australie" : isNouvelleZl ? "nouvelle_zelande"
    : isPologne ? "pologne" : isCoree ? "coree_du_sud"
    : isAndorre ? "andorre" : isMonaco ? "monaco" : isDanemark ? "danemark"
    : isFinlande ? "finlande"
    : isSuede ? "suede" : isEstonie ? "estonie" : isLettonie ? "lettonie" : isLituanie ? "lituanie"
    : isAutriche ? "autriche" : isTchequie ? "tchequie" : isSlovaquie ? "slovaquie" : isHongrie ? "hongrie" : isSlovenie ? "slovenie"
    : isGrece ? "grece" : isChypre ? "chypre" : isMalte ? "malte" : isCroatie ? "croatie"
    : isIrlande ? "irlande" : isRoumanie ? "roumanie" : isBulgarie ? "bulgarie"
    : null;
  const datePaie = date;

  try {
    // Mémorisé pour pouvoir re-calculer à l'identique au changement de langue
    // (translateApp) sans relire les formulaires.
    _lastCalcReq = {
      salarie: {
        nom, prenom, salaire_brut: totalBrut.toString(), statut,
        etp: parseFloat(document.getElementById('d-etp')?.value ?? '100') || 100,
        alsace_moselle: alsaceMoselle,
        pays: paysEtranger ?? (isFPT ? "fonction_publique" : "france"),
        // Entreprise adaptée : France privé uniquement (jamais FPT ni étranger).
        entreprise_adaptee: isEA && !paysEtranger && !isFPT,
        tranche_age_ea: (isEA && !paysEtranger && !isFPT) ? eaTranche : null,
        // Heures supplémentaires/complémentaires : France privé uniquement. Le brut
        // de base (salaire_base) sert à dériver le taux horaire côté backend.
        salaire_base: _remBase.toString(),
        heures_supp: (!paysEtranger && !isFPT) ? remHeures.supp : 0,
        heures_comp: (!paysEtranger && !isFPT) ? remHeures.comp : 0,
        effectif,
        assujetti_is: assujettiIS,
        canton:   (isSuisse && assujettiIS && canton)  ? canton  : null,
        tarif_is: (isSuisse && assujettiIS && tarifIs) ? tarifIs : null,
        regione: null,
        contratto_termine: false,
        province: isCanada ? caProvince : null,
        us_state: isEtatsUnis ? usState : null,
        emirati_national: isEmirats ? emiratiNational : null,
        inde_regime: isInde ? indeRegime : null,
        steuerklasse: isAllemagne ? parseInt(steuerklasse, 10) : null,
        kinderlos:    isAllemagne ? kinderlos : null,
        kirchenmitglied: isAllemagne ? kirchenmitglied : null,
        land:         isAllemagne ? deLand : null,
        region_be:    isBelgique ? beRegion : null,
        anciennete,
      },
      datePaie,
      lang: _currentLang,
      absence: getAbsencePayload(),
      // Paye inversée : la saisie devient la cible de net (avant impôt à la
      // source) ; le backend ignore alors salaire_brut et reconstitue le brut.
      ...(_modeSaisie === 'net' ? { netCible: brutVal.toString() } : {}),
    };
    const bulletin = await api("calculer_bulletin", _lastCalcReq);
    lastBulletin = bulletin;
    // Mode net : la ligne « Salaire de base » (section RÉMUNÉRATION) et l'aperçu
    // de retenue d'absence doivent reposer sur le BRUT RECONSTITUÉ plein, pas
    // sur le net saisi. En cas d'absence, ce brut plein est `absence.brut_mensuel`
    // (bulletin.brut n'est alors que l'assiette après retenue/maintien/IJSS).
    if (_modeSaisie === 'net') {
      if (bulletin.absence) {
        _remBase = parseFloat(bulletin.absence.brut_mensuel) || 0;
      } else {
        const gainHs = bulletin.heures_sup
          ? (parseFloat(bulletin.heures_sup.gain_hs) || 0) + (parseFloat(bulletin.heures_sup.gain_hc) || 0)
          : 0;
        _remBase = Math.max(0, (parseFloat(bulletin.brut) || 0) - gainHs);
      }
    }
    renderAll(bulletin);
    _afficherBrutReconstitue(bulletin);
    _updateAnnuelBtn();
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

  // Tirage unique : un seul (i) parmi toutes les lignes ±0,01
  const _centIdx = rows.reduce((acc, r, i) => {
    if (parseFloat(Math.abs(parseFloat(r.fillon_regularise) - parseFloat(r.fillon_simple)).toFixed(2)) === 0.01) acc.push(i);
    return acc;
  }, []);
  const _centElu = _centIdx.length ? _centIdx[Math.floor(Math.random() * _centIdx.length)] : -1;

  const tbody = rows.map((r, i) => {
    const smicChange  = i > 0 && r.smic !== smics[i - 1];
    const is13e       = r.mois_libelle.includes("13e");
    const delta       = parseFloat(r.fillon_regularise) - parseFloat(r.fillon_simple);
    const infoBtn     = i === _centElu ? ` <button class="delta-info-btn" onclick="openCommentFillon()" title="Note JNF">(i)</button>` : '';
    const deltaTxt    = Math.abs(delta) < 0.005
      ? `<span style="color:var(--dim)">—</span>`
      : `<span class="delta-nonzero">${delta > 0 ? "+" : ""}${fmtS(delta.toFixed(2))}</span>${infoBtn}`;
    const rowCls = [smicChange ? "smic-change" : "", is13e ? "treizieme-mois" : ""].filter(Boolean).join(" ");

    return `<tr class="${rowCls}">
      <td>${r.mois_libelle}</td>
      <td>${fmt(r.smic)}</td>
      <td>${fmt(r.brut)}</td>
      <td class="c-sal">− ${fmt(r.total_sal)}</td>
      <td class="c-pat">+ ${fmt(r.total_pat_brut)}</td>
      <td class="c-alleg">− ${fmt(r.fillon_regularise)}</td>
      <td>${deltaTxt}</td>
      <td class="c-green">${fmt(r.net_a_payer)}</td>
      <td class="c-eblue">${fmt(r.cout_employeur)}</td>
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
      <td class="c-eblue">${fmt(sim.total_cout)}</td>
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
        <div style="color:var(--electric-blue);font-size:1.1rem;font-weight:bold">${fmt(sim.total_cout)}</div>
      </div>
    </div>`;

  el.innerHTML = `
    <div class="tbl-section-head">── SIMULATION ANNUELLE ${sim.annee} ────────────────────────────────────</div>
    <div style="font-size:0.70rem;color:var(--muted);margin-bottom:0.4rem">
      Décembre inclut un 13e mois (salaire doublé). Brut total = 13 mois. Fillon régularisé sur rémunération annuelle réelle.
    </div>
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
      etp: parseFloat(document.getElementById('a-etp')?.value ?? '100') || 100,
    });
    renderAnnuel(sim);
  } catch (e) {
    console.error("[simuler_annee] erreur brute :", e);
    el.innerHTML = `<div style="padding:1rem;color:var(--red);font-size:0.8rem">ERREUR : ${esc(errToStr(e))}</div>`;
  }
}

// ── Gestion multi-pays (Suisse / Luxembourg / FPT) ───────────────────────────
// Appelé depuis chaque checkbox pays ; gère l'exclusion mutuelle et l'UI commune.
// FPT est France (EUR, date libre, Alsace-Moselle compatible).
// Suisse/Luxembourg sont étrangers (date figée 2026, masque Alsace-Moselle).
window.onTogglePays = function(pays, checked) {
  const TOUS_PAYS    = ['france', 'suisse', 'luxembourg', 'fpt', 'italie', 'espagne', 'portugal', 'belgique', 'allemagne', 'canada', 'quebec', 'angleterre', 'japon', 'chine', 'paysbas', 'australie', 'nouvellezelande', 'pologne', 'coree', 'andorre', 'monaco', 'danemark', 'finlande', 'suede', 'estonie', 'lettonie', 'lituanie', 'autriche', 'tchequie', 'slovaquie', 'hongrie', 'slovenie', 'grece', 'chypre', 'malte', 'croatie', 'irlande', 'roumanie', 'bulgarie', 'etatsunis', 'mexique', 'bresil', 'emirats', 'inde'];
  const PAYS_ETR     = ['suisse', 'luxembourg', 'italie', 'espagne', 'portugal', 'belgique', 'allemagne', 'canada', 'quebec', 'angleterre', 'japon', 'chine', 'paysbas', 'australie', 'nouvellezelande', 'pologne', 'coree', 'andorre', 'monaco', 'danemark', 'finlande', 'suede', 'estonie', 'lettonie', 'lituanie', 'autriche', 'tchequie', 'slovaquie', 'hongrie', 'slovenie', 'grece', 'chypre', 'malte', 'croatie', 'irlande', 'roumanie', 'bulgarie', 'etatsunis', 'mexique', 'bresil', 'emirats', 'inde'];
  const AUTRES_PAYS  = TOUS_PAYS.filter(p => p !== pays);

  // Si on coche un régime, décocher tous les autres (exclusion mutuelle)
  if (checked) {
    AUTRES_PAYS.forEach(autre => {
      ['d', 'm'].forEach(p => {
        const el = document.getElementById(`${p}-${autre}`);
        if (el && el.checked) { el.checked = false; }
      });
    });
  }

  // France sous-menu (Alsace-Moselle + FPT) : visible quand France OU FPT est coché
  const isFranceEco = document.getElementById('d-france')?.checked || document.getElementById('d-fpt')?.checked;
  ['d', 'm'].forEach(p => {
    const subWrap = document.getElementById(`${p}-france-sub-wrap`);
    if (!subWrap) return;
    subWrap.style.display = isFranceEco ? '' : 'none';
    if (!isFranceEco) {
      const am = document.getElementById(`${p}-alsace-moselle`);
      if (am) am.checked = false;
    }
  });

  // Entreprise adaptée : France privé uniquement (incompatible FPT/étranger).
  // On décoche EA dès que France privé n'est plus le régime actif.
  const isFrancePrive = document.getElementById('d-france')?.checked
    && !document.getElementById('d-fpt')?.checked;
  ['d', 'm'].forEach(p => {
    const ea = document.getElementById(`${p}-ea`);
    if (ea && !isFrancePrive) ea.checked = false;
    const wrap = document.getElementById(`${p}-ea-tranche-wrap`);
    if (wrap) wrap.style.display = (ea && ea.checked) ? '' : 'none';
  });
  ['d-date', 'm-date'].forEach(id => {
    const el = document.getElementById(id);
    if (!el) return;
    el.disabled = false;
  });

  // Label devise : CHF (Suisse), CAD (Canada/Québec), GBP (Angleterre), JPY (Japon), CNY (Chine), EUR (autres)
  const isSuisse      = document.getElementById('d-suisse')?.checked;
  const isCA          = document.getElementById('d-canada')?.checked || document.getElementById('d-quebec')?.checked;
  const isAngleterre  = document.getElementById('d-angleterre')?.checked;
  const isJapon       = document.getElementById('d-japon')?.checked;
  const isChine       = document.getElementById('d-chine')?.checked;
  const isAustralie   = document.getElementById('d-australie')?.checked;
  const isNZ          = document.getElementById('d-nouvellezelande')?.checked;
  const isPologne     = document.getElementById('d-pologne')?.checked;
  const isCoree       = document.getElementById('d-coree')?.checked;
  const isDanemark    = document.getElementById('d-danemark')?.checked;
  const isSuede       = document.getElementById('d-suede')?.checked;
  const isTchequie    = document.getElementById('d-tchequie')?.checked;
  const isHongrie     = document.getElementById('d-hongrie')?.checked;
  const isRoumanie    = document.getElementById('d-roumanie')?.checked;
  const isBulgarie    = document.getElementById('d-bulgarie')?.checked;
  const isUsa         = document.getElementById('d-etatsunis')?.checked;
  const isMexique     = document.getElementById('d-mexique')?.checked;
  const isBresil      = document.getElementById('d-bresil')?.checked;
  const isEmirats     = document.getElementById('d-emirats')?.checked;
  const isInde        = document.getElementById('d-inde')?.checked;
  const labelBrut  = isBresil ? 'SALAIRE BRUT (BRL)' : isEmirats ? 'SALAIRE BRUT (AED)' : isInde ? 'SALAIRE BRUT (INR)' : isMexique ? 'SALAIRE BRUT (MXN)' : isUsa ? 'SALAIRE BRUT (USD)' : isSuisse ? 'SALAIRE BRUT (CHF)' : isCA ? 'SALAIRE BRUT (CAD)' : isAngleterre ? 'SALAIRE BRUT (GBP)' : isJapon ? 'SALAIRE BRUT (JPY)' : isChine ? 'SALAIRE BRUT (CNY)' : isAustralie ? 'SALAIRE BRUT (AUD)' : isNZ ? 'SALAIRE BRUT (NZD)' : isPologne ? 'SALAIRE BRUT (PLN)' : isCoree ? 'SALAIRE BRUT (KRW)' : isDanemark ? 'SALAIRE BRUT (DKK)' : isSuede ? 'SALAIRE BRUT (SEK)' : isTchequie ? 'SALAIRE BRUT (CZK)' : isHongrie ? 'SALAIRE BRUT (HUF)' : isRoumanie ? 'SALAIRE BRUT (RON)' : isBulgarie ? 'SALAIRE BRUT (BGN)' : 'SALAIRE BRUT (€)';
  const labelBrutM = isBresil ? 'BRUT (BRL)'         : isEmirats ? 'BRUT (AED)'        : isInde ? 'BRUT (INR)'          : isMexique ? 'BRUT (MXN)'        : isUsa ? 'BRUT (USD)'            : isSuisse ? 'BRUT (CHF)'         : isCA ? 'BRUT (CAD)'         : isAngleterre ? 'BRUT (GBP)'         : isJapon ? 'BRUT (JPY)'         : isChine ? 'BRUT (CNY)'         : isAustralie ? 'BRUT (AUD)'         : isNZ ? 'BRUT (NZD)'         : isPologne ? 'BRUT (PLN)'         : isCoree ? 'BRUT (KRW)'         : isDanemark ? 'BRUT (DKK)'         : isSuede ? 'BRUT (SEK)'         : isTchequie ? 'BRUT (CZK)'         : isHongrie ? 'BRUT (HUF)'         : isRoumanie ? 'BRUT (RON)'         : isBulgarie ? 'BRUT (BGN)'         : 'BRUT (€)';
  // Mémorise les libellés devise et laisse _appliquerLabelSalaire arbitrer
  // l'affichage BRUT/NET selon le mode de saisie courant.
  _labelsSalaire = { brut: labelBrut, brutM: labelBrutM };
  _appliquerLabelSalaire();

  // Affiche/masque le bloc IS Suisse (et réinitialise si on décoche Suisse)
  ['d', 'm'].forEach(p => {
    const isWrap = document.getElementById(`${p}-ch-is-wrap`);
    if (!isWrap) return;
    if (isSuisse) {
      isWrap.style.display = '';
    } else {
      isWrap.style.display = 'none';
      // Réinitialiser les champs IS quand on décoche Suisse
      const cbIS = document.getElementById(`${p}-assujetti-is`);
      if (cbIS) cbIS.checked = false;
      const detail = document.getElementById(`${p}-ch-is-detail`);
      if (detail) detail.style.display = 'none';
    }
  });

  // Canada sous-menu (province + Québec) : visible quand Canada OU Québec est coché
  const isCanadaEco = document.getElementById('d-canada')?.checked || document.getElementById('d-quebec')?.checked;
  ['d', 'm'].forEach(p => {
    const subWrap = document.getElementById(`${p}-ca-sub-wrap`);
    if (subWrap) subWrap.style.display = isCanadaEco ? 'flex' : 'none';
  });

  // Province Canada : visible uniquement quand Canada est coché (pas Québec seul)
  const isCanadaChecked = document.getElementById('d-canada')?.checked;
  ['d', 'm'].forEach(p => {
    const wrap = document.getElementById(`${p}-ca-province-wrap`);
    if (wrap) wrap.style.display = isCanadaChecked ? '' : 'none';
  });

  // Sélecteur d'État américain : visible quand États-Unis est coché
  const isUsChecked = document.getElementById('d-etatsunis')?.checked;
  ['d', 'm'].forEach(p => {
    const wrap = document.getElementById(`${p}-us-state-wrap`);
    if (wrap) wrap.style.display = isUsChecked ? '' : 'none';
  });

  // Option « national émirati » (GPSSA) : visible quand Émirats est coché
  const isEmiratsChecked = document.getElementById('d-emirats')?.checked;
  ['d', 'm'].forEach(p => {
    const wrap = document.getElementById(`${p}-ae-sub-wrap`);
    if (wrap) wrap.style.display = isEmiratsChecked ? '' : 'none';
    if (!isEmiratsChecked) { const cb = document.getElementById(`${p}-emirati-national`); if (cb) cb.checked = false; }
  });

  // Sélecteur de régime d'impôt indien : visible quand Inde est cochée
  const isIndeChecked = document.getElementById('d-inde')?.checked;
  ['d', 'm'].forEach(p => {
    const wrap = document.getElementById(`${p}-in-regime-wrap`);
    if (wrap) wrap.style.display = isIndeChecked ? '' : 'none';
  });

  // Région Belgique : visible quand Belgique est coché
  const isBelgiqueChecked = document.getElementById('d-belgique')?.checked;
  ['d', 'm'].forEach(p => {
    const wrap = document.getElementById(`${p}-be-region-wrap`);
    if (wrap) wrap.style.display = isBelgiqueChecked ? '' : 'none';
  });

  // Affiche/masque le panneau Allemagne (Steuerklasse, Kinderlos, Kirchensteuer)
  const isAllemagneChecked = document.getElementById('d-allemagne')?.checked;
  ['d', 'm'].forEach(p => {
    const wrap = document.getElementById(`${p}-de-wrap`);
    if (!wrap) return;
    wrap.style.display = isAllemagneChecked ? 'flex' : 'none';
    if (!isAllemagneChecked) {
      const cbK = document.getElementById(`${p}-kirchenmitglied`);
      if (cbK) cbK.checked = false;
      const kirche = document.getElementById(`${p}-de-kirche-detail`);
      if (kirche) kirche.style.display = 'none';
    }
  });

  // France cochée par défaut si aucun autre régime actif
  const aucunActif = TOUS_PAYS.filter(p => p !== 'france')
    .every(p => !document.getElementById(`d-${p}`)?.checked);
  ['d', 'm'].forEach(p => {
    const fr = document.getElementById(`${p}-france`);
    if (fr) fr.checked = aucunActif;
  });

  // Nouveau pays → prochain basculement H/F tire depuis le bon pool
  _ecartTire = false;
};

// Entreprise adaptée (aide au poste) : régime France privé.
// Incompatible avec la Fonction publique et tout pays étranger ;
// cumulable avec Alsace-Moselle. Affiche le sélecteur de tranche d'âge.
window.onToggleEA = function(prefix, checked) {
  if (checked) {
    // EA = France privé : décocher FPT et basculer sur France via onTogglePays
    // (qui décoche aussi tout pays étranger et réaffiche le sous-menu France).
    ['d', 'm'].forEach(p => {
      const fpt = document.getElementById(`${p}-fpt`);
      if (fpt) fpt.checked = false;
      const fr = document.getElementById(`${p}-france`);
      if (fr) fr.checked = true;
    });
    window.onTogglePays('france', true);
    // onTogglePays ne touche pas EA en France privé : on garde la case cochée.
    ['d', 'm'].forEach(p => {
      const ea = document.getElementById(`${p}-ea`);
      if (ea) ea.checked = true;
    });
  }
  // Sélecteur de tranche d'âge visible uniquement quand EA est coché.
  ['d', 'm'].forEach(p => {
    const ea = document.getElementById(`${p}-ea`);
    const wrap = document.getElementById(`${p}-ea-tranche-wrap`);
    if (wrap) wrap.style.display = (ea && ea.checked) ? '' : 'none';
  });
};

window.toggleDeKircheDetail = function(prefix, checked) {
  const detail = document.getElementById(`${prefix}-de-kirche-detail`);
  if (detail) detail.style.display = checked ? '' : 'none';
  // Sync l'autre formulaire
  const other = prefix === 'd' ? 'm' : 'd';
  const cbOther = document.getElementById(`${other}-kirchenmitglied`);
  if (cbOther) cbOther.checked = checked;
  const detailOther = document.getElementById(`${other}-de-kirche-detail`);
  if (detailOther) detailOther.style.display = checked ? '' : 'none';
};

// ── Paramètres avancés ───────────────────────────────────────────────────────
window.toggleParams = function(prefix) {
  const panel  = document.getElementById(`${prefix}-params`);
  const toggle = document.getElementById(`${prefix}-params-toggle`);
  if (!panel) return;
  const open = panel.style.display !== 'none';
  panel.style.display  = open ? 'none' : 'block';
  toggle.classList.toggle('open', !open);
};

window.toggleDuree = function(prefix) {
  const panel  = document.getElementById(`${prefix}-duree`);
  const toggle = document.getElementById(`${prefix}-duree-toggle`);
  if (!panel) return;
  const open = panel.style.display !== 'none';
  panel.style.display = open ? 'none' : 'block';
  toggle.classList.toggle('open', !open);
};

// Aligne le <select> fraction sur la valeur ETP calculée (ou "" si hors preset).
function _syncEtpSel(prefix, etpVal) {
  const sel = document.getElementById(`${prefix}-etp-sel`);
  if (!sel) return;
  const opt = [...sel.options].find(o => o.value !== '' && Math.abs(Number(o.value) - etpVal) < 0.01);
  sel.value = opt ? opt.value : '';
}

window.onEtpSelChange = function(prefix) {
  const sel = document.getElementById(`${prefix}-etp-sel`);
  const inp = document.getElementById(`${prefix}-etp`);
  if (sel.value !== '') {
    inp.value = sel.value;
    onDureeChange(prefix, 'etp');
  }
};

window.onDureeChange = function(prefix, field) {
  const etpEl  = document.getElementById(`${prefix}-etp`);
  const semEl  = document.getElementById(`${prefix}-h-semaine`);
  const moisEl = document.getElementById(`${prefix}-h-mois`);
  const BASE   = 35;
  const r2     = v => Math.round(v * 100) / 100;

  let etp  = parseFloat(etpEl.value);
  let sem  = parseFloat(semEl.value);
  let mois = parseFloat(moisEl.value);

  if (field === 'etp' && !isNaN(etp)) {
    sem  = r2(etp / 100 * BASE);
    mois = r2(sem * 52 / 12);
    semEl.value  = sem;
    moisEl.value = mois;
    _syncEtpSel(prefix, etp);
  } else if (field === 'semaine' && !isNaN(sem)) {
    etp  = r2(sem / BASE * 100);
    mois = r2(sem * 52 / 12);
    etpEl.value  = etp;
    moisEl.value = mois;
    _syncEtpSel(prefix, etp);
  } else if (field === 'mois' && !isNaN(mois)) {
    sem  = r2(mois * 12 / 52);
    etp  = r2(sem / BASE * 100);
    etpEl.value = etp;
    semEl.value = sem;
    _syncEtpSel(prefix, etp);
  }

  const other = prefix === 'd' ? 'm' : 'd';
  [['etp', etpEl], ['h-semaine', semEl], ['h-mois', moisEl]].forEach(([id, src]) => {
    const dst = document.getElementById(`${other}-${id}`);
    if (dst) dst.value = src.value;
  });
  _syncEtpSel(other, parseFloat(etpEl.value));

  const isChecked = document.getElementById('d-apply-brut-chk')?.checked;
  if (isChecked && !isNaN(etp) && _etpPrev > 0 && Math.abs(etp - _etpPrev) > 0.001) {
    const factor = etp / _etpPrev;
    ['d-brut', 'm-brut'].forEach(id => {
      const el = document.getElementById(id);
      if (!el) return;
      const v = parseFloat(el.value);
      if (!isNaN(v) && v > 0) el.value = Math.round(v * factor * 100) / 100;
    });
    _etpPrev = etp;
  }

  // Les heures supp (temps plein) et complémentaires (temps partiel) sont
  // mutuellement exclusives : on bascule le type des lignes selon l'ETP, puis on
  // rafraîchit la rémunération (options + indices) et on relance le calcul.
  const isFullTime = !isNaN(etp) && Math.abs(etp - 100) < 0.01;
  let switched = false;
  _remLines.forEach(l => {
    if (isFullTime && l.type === 'hc') { l.type = 'hs'; switched = true; }
    else if (!isFullTime && l.type === 'hs') { l.type = 'hc'; switched = true; }
  });
  _reRenderRemInPlace();
  if (switched || _remLines.some(l => HEURE_TYPES.has(l.type))) _triggerRecalculate();
};

window.onApplyBrutChk = function(prefix) {
  const other = prefix === 'd' ? 'm' : 'd';
  const src = document.getElementById(`${prefix}-apply-brut-chk`);
  const dst = document.getElementById(`${other}-apply-brut-chk`);
  if (src && dst) dst.checked = src.checked;
  if (src?.checked) {
    _etpPrev = parseFloat(document.getElementById(`${prefix}-etp`)?.value) || 100;
  }
};

// ── Section Rémunération (état persisté, rendu dans le résultat) ─────────────

let _remLines = []; // [{ id, type, amount }]
let _remBase  = 0;  // salaire de base saisi dans le formulaire

function getRemOptions(etp) {
  const common = [
    { value: 'prime',      label: 'Prime' },
    { value: 'coupure_50', label: 'Coupures 50%' },
  ];
  // Heures supp (temps plein) / complémentaires (temps partiel) : saisie EN HEURES,
  // la majoration par tranche est calculée (8 h à +25 % / +50 % ; 1/10 à +10 % / +25 %).
  if (Math.abs(parseFloat(etp) - 100) < 0.01) {
    return [ { value: 'hs', label: 'Heures supp.' }, ...common ];
  }
  return [ { value: 'hc', label: 'Heures compl.' }, ...common ];
}

// Types de ligne saisis en HEURES (et non en euros).
const HEURE_TYPES = new Set(['hs', 'hc']);
const HEURES_TEMPS_PLEIN = 151.67;

// Taux horaire dérivé du salaire de base : base / (151,67 × ETP/100).
function _tauxHoraire(base, etp) {
  const h = HEURES_TEMPS_PLEIN * (parseFloat(etp) / 100);
  return h > 0 ? base / h : 0;
}
// Gain brut majoré (preview live ; le backend reste autoritaire).
function _gainHeures(type, hours, etp) {
  const taux = _tauxHoraire(_remBase, etp);
  const h = parseFloat(hours) || 0;
  if (type === 'hs') {
    return Math.min(h, 8) * taux * 1.25 + Math.max(0, h - 8) * taux * 1.50;
  }
  if (type === 'hc') {
    const seuil = HEURES_TEMPS_PLEIN * (parseFloat(etp) / 100) * 0.10;
    return Math.min(h, seuil) * taux * 1.10 + Math.max(0, h - seuil) * taux * 1.25;
  }
  return 0;
}

function getRemTotal() {
  // Brut de base envoyé au backend = salaire de base + éléments variables EN EUROS
  // (primes, coupures). Les heures supp/compl partent en heures via getRemHeures()
  // et leur majoration est ajoutée au brut côté backend.
  return _remBase + _remLines
    .filter(l => !HEURE_TYPES.has(l.type))
    .reduce((s, l) => s + (parseFloat(l.amount) || 0), 0);
}

// Heures supplémentaires/complémentaires saisies (sommées par type).
function getRemHeures() {
  let supp = 0, comp = 0;
  _remLines.forEach(l => {
    const v = parseFloat(l.amount) || 0;
    if (l.type === 'hs') supp += v;
    else if (l.type === 'hc') comp += v;
  });
  return { supp, comp };
}

// Total brut affiché (base + euros + majoration estimée des heures), pour le live.
function getRemDisplayTotal(etp) {
  const e = parseFloat(etp ?? document.getElementById('d-etp')?.value ?? '100') || 100;
  const extra = _remLines.reduce((s, l) =>
    HEURE_TYPES.has(l.type) ? s + _gainHeures(l.type, l.amount, e) : s, 0);
  return getRemTotal() + extra;
}

// Spec d'absence envoyée au backend (snake_case, comme les champs de salarie).
function getAbsencePayload() {
  if (!_absence?.active) return null;
  return {
    type_arret:      _absence.type || 'maladie',
    date_debut:      _absence.dateDebut || '',
    date_fin:        _absence.dateFin || '',
    methode:         _absence.methode || 'moyens',
    jours_type:      _absence.joursType || 'ouvres',
    heures_mois:     parseFloat(document.getElementById('d-h-mois')?.value) || 151.67,
    convention_idcc: _absence.conventionIDCC || '0016',
  };
}

// Rendu d'une ligne de rémunération. Pour les types « heures » la saisie est en
// heures (pas de pas 0,01 €) avec un indice du gain majoré estimé à côté.
function _remLineHtml(l, opts, etp) {
  const selOpts = opts.map(o =>
    `<option value="${o.value}"${o.value === l.type ? ' selected' : ''}>${o.label}</option>`
  ).join('');
  const isHour = HEURE_TYPES.has(l.type);
  const hint = isHour
    ? `<span class="rem-h-hint" title="Taux horaire ${fmt(_tauxHoraire(_remBase, etp))} — majoration incluse">h · ≈ ${fmt(_gainHeures(l.type, l.amount, etp))}</span>`
    : '';
  return `
      <div class="rem-line">
        <select class="rem-type-sel" onchange="onRemTypeChange('${l.id}',this.value)">${selOpts}</select>
        <input type="number" class="rem-amt-inp" value="${l.amount || ''}" placeholder="${isHour ? 'heures' : '0.00'}" min="0" step="${isHour ? '0.5' : '0.01'}"
               oninput="onRemAmountChange('${l.id}',this.value)" />
        ${hint}
        <button class="btn-rm-rem" type="button" onclick="removeRemLineResult('${l.id}')">×</button>
      </div>`;
}

function buildRemSection() {
  const etp  = parseFloat(document.getElementById('d-etp')?.value ?? '100') || 100;
  const opts = getRemOptions(etp);
  const lines = _remLines.map(l => _remLineHtml(l, opts, etp)).join('');
  const isFrance = !lastBulletin || lastBulletin.salarie?.pays === 'france';
  const addBtn = isFrance
    ? `<button class="btn-add-rem" type="button" onclick="addRemLineResult()" title="Ajouter un élément">+</button>`
    : '';
  const absenceBtn = isFrance
    ? `<button class="btn-absence-toggle${_absence ? ' active' : ''}" id="btn-absence-d" type="button" title="Retenue pour absence" onclick="toggleAbsencePanel('d')">−</button>`
    : '';
  const absencePanel = (_absence !== null && isFrance) ? _buildAbsencePanel(false) : '';
  const absInfo = (_absence?.active && lastBulletin?.absence) ? lastBulletin.absence : null;
  if (absInfo) {
    _fmStore['ABS_RETENUE']  = { type: 'absence', which: 'retenue',    a: absInfo };
    _fmStore['ABS_MAINTIEN'] = { type: 'absence', which: 'maintien',   a: absInfo };
    _fmStore['ABS_IJSS']     = { type: 'absence', which: 'ijss',       a: absInfo };
    _fmStore['ABS_AJUST']    = { type: 'absence', which: 'ajustement', a: absInfo };
  }
  const absRows = absInfo ? [
    _absenceRow(`Retenue absence (${esc(absInfo.libelle)})`, 'c-red', '−', absInfo.retenue, 'ABS_RETENUE'),
    parseFloat(absInfo.maintien) > 0 ? _absenceRow(`Maintien de salaire (${esc(absInfo.convention)})`, 'c-green', '+', absInfo.maintien, 'ABS_MAINTIEN') : '',
    parseFloat(absInfo.ijss_brut) > 0 ? _absenceRow('IJSS brutes (subrogation)', 'c-red', '−', absInfo.ijss_brut, 'ABS_IJSS') : '',
    parseFloat(absInfo.ajustement_net) > 0 ? _absenceRow('Ajustement du net (garantie du net)', 'c-red', '−', absInfo.ajustement_net, 'ABS_AJUST') : '',
  ].join('') : '';
  const absenceLine = absRows ? _absenceEmbedTable(absRows) : '';
  const total = lastBulletin ? parseFloat(lastBulletin.brut) : getRemDisplayTotal(etp);
  // Salaire de base et Total brut rendus en tableau embed (même colgroup que
  // .ascii-tbl) : leurs montants tombent dans la 4ᵉ colonne, alignés sur la
  // colonne PART SALARIÉ du tableau des cotisations ci-dessous.
  const baseRow = `<table class="ascii-tbl abs-embed rem-embed">${_ABS_COLGROUP}<tbody>
      <tr>
        <td colspan="3"><div class="rem-embed-head">${addBtn}${absenceBtn}<span class="rem-base-lbl">Salaire de base</span></div></td>
        <td class="r rem-embed-val">${fmt(_remBase)}</td>
        <td colspan="2"></td>
      </tr>
    </tbody></table>`;
  const totalRow = (_remLines.length > 0 || _absence?.active) ? `<table class="ascii-tbl abs-embed rem-embed rem-embed-total">${_ABS_COLGROUP}<tbody>
      <tr>
        <td colspan="3"><span class="rem-total-lbl">Total brut</span></td>
        <td class="r rem-total-val">${fmt(total)}</td>
        <td colspan="2"></td>
      </tr>
    </tbody></table>` : '';
  return `
    <div class="tbl-section-head">── RÉMUNÉRATION ────────────────────────────────────────────────────────────────────</div>
    <div class="rem-section">
      ${baseRow}
      ${absencePanel}${lines}${absenceLine}${totalRow}
    </div>`;
}

function buildRemSectionMobile() {
  const etp  = parseFloat(document.getElementById('d-etp')?.value ?? '100') || 100;
  const opts = getRemOptions(etp);
  const lines = _remLines.map(l => _remLineHtml(l, opts, etp)).join('');
  const isFrance = !lastBulletin || lastBulletin.salarie?.pays === 'france';
  const addBtn = isFrance
    ? `<button class="btn-add-rem" type="button" onclick="addRemLineResult()" title="Ajouter">+</button>`
    : '';
  const absenceBtn = isFrance
    ? `<button class="btn-absence-toggle${_absence ? ' active' : ''}" id="btn-absence-m" type="button" title="Retenue pour absence" onclick="toggleAbsencePanel('m')">−</button>`
    : '';
  const absencePanel = (_absence !== null && isFrance) ? _buildAbsencePanel(true) : '';
  const absInfo = (_absence?.active && lastBulletin?.absence) ? lastBulletin.absence : null;
  if (absInfo) {
    _fmStore['ABS_RETENUE']  = { type: 'absence', which: 'retenue',    a: absInfo };
    _fmStore['ABS_MAINTIEN'] = { type: 'absence', which: 'maintien',   a: absInfo };
    _fmStore['ABS_IJSS']     = { type: 'absence', which: 'ijss',       a: absInfo };
    _fmStore['ABS_AJUST']    = { type: 'absence', which: 'ajustement', a: absInfo };
  }
  const absenceLine = absInfo ? `
    <div class="rem-absence-line"><span style="flex:1">Retenue absence (${esc(absInfo.libelle)})</span><span class="c-red" style="cursor:pointer" onclick="showFormula('ABS_RETENUE')">− ${fmt(absInfo.retenue)}${buildFormulaStar('ABS_RETENUE')}</span></div>
    ${parseFloat(absInfo.maintien) > 0 ? `<div class="rem-absence-line"><span style="flex:1">Maintien de salaire (${esc(absInfo.convention)})</span><span class="c-green" style="cursor:pointer" onclick="showFormula('ABS_MAINTIEN')">+ ${fmt(absInfo.maintien)}${buildFormulaStar('ABS_MAINTIEN')}</span></div>` : ''}
    ${parseFloat(absInfo.ijss_brut) > 0 ? `<div class="rem-absence-line"><span style="flex:1">IJSS brutes (subrogation)</span><span class="c-red" style="cursor:pointer" onclick="showFormula('ABS_IJSS')">− ${fmt(absInfo.ijss_brut)}${buildFormulaStar('ABS_IJSS')}</span></div>` : ''}
    ${parseFloat(absInfo.ajustement_net) > 0 ? `<div class="rem-absence-line"><span style="flex:1">Ajustement du net (garantie du net)</span><span class="c-red" style="cursor:pointer" onclick="showFormula('ABS_AJUST')">− ${fmt(absInfo.ajustement_net)}${buildFormulaStar('ABS_AJUST')}</span></div>` : ''}` : '';
  const total = lastBulletin ? parseFloat(lastBulletin.brut) : getRemDisplayTotal(etp);
  const totalRow = (_remLines.length > 0 || _absence?.active) ? `
    <div class="rem-total-row" style="display:flex;margin-left:0">
      <span class="rem-total-lbl">Total brut</span>
      <span class="rem-total-val">${fmt(total)}</span>
    </div>` : '';
  return `
    <div class="mob-row section"><span class="mob-lbl">── RÉMUNÉRATION ──</span></div>
    <div class="rem-section" style="padding:0.3rem 0.9rem 0.4rem">
      <div class="rem-base-row">
        ${addBtn}
        ${absenceBtn}
        <span class="rem-base-lbl">Salaire de base</span>
        <span style="font-size:0.68rem;color:var(--fg)">${fmt(_remBase)}</span>
      </div>
      ${absencePanel}${lines}${absenceLine}${totalRow}
    </div>`;
}

window.addRemLineResult = function() {
  const etp  = parseFloat(document.getElementById('d-etp')?.value ?? '100') || 100;
  const opts = getRemOptions(etp);
  _remLines.push({ id: `rl-${Date.now()}`, type: opts[0].value, amount: 0 });
  _reRenderRemInPlace();
};

window.removeRemLineResult = function(id) {
  _remLines = _remLines.filter(l => l.id !== id);
  if (_remLines.every(l => !l.amount)) {
    _reRenderRemInPlace();
  } else {
    _triggerRecalculate();
  }
};

window.onRemTypeChange = function(id, val) {
  const l = _remLines.find(l => l.id === id);
  if (l) l.type = val;
  // Le passage euros ↔ heures change la nature de la ligne (placeholder, indice) :
  // on re-rend en place puis on relance le calcul.
  _reRenderRemInPlace();
  _triggerRecalculate();
};

window.onRemAmountChange = function(id, val) {
  const l = _remLines.find(l => l.id === id);
  if (l) l.amount = parseFloat(val) || 0;
  // Mise à jour immédiate du total sans re-render (évite de tuer le focus input)
  const total = getRemDisplayTotal();
  ['d', 'm'].forEach(p => {
    const row = document.querySelector(`#rem-result-${p} .rem-total-row`);
    if (!row) return;
    row.style.display = 'flex';
    const valEl = row.querySelector('.rem-total-val');
    if (valEl) valEl.textContent = fmt(total);
  });
  _triggerRecalculate();
};

function _reRenderRemInPlace() {
  const dBlock = document.getElementById('rem-result-d');
  if (dBlock) dBlock.innerHTML = buildRemSection();
  const mBlock = document.getElementById('rem-result-m');
  if (mBlock) mBlock.innerHTML = buildRemSectionMobile();
}

let _recalcTimer = null;
function _triggerRecalculate() {
  clearTimeout(_recalcTimer);
  _recalcTimer = setTimeout(() => calculate('desktop'), 350);
}
// Exposé pour les handlers inline (onchange) du HTML (module → scope non global).
window._triggerRecalculate = _triggerRecalculate;

// Synchronise un paramètre entre les deux formulaires (desktop ↔ mobile).
// Gère les checkboxes ET les selects (canton, tarif-is).
window.syncParam = function(paramName, value) {
  ['d', 'm'].forEach(prefix => {
    const el = document.getElementById(`${prefix}-${paramName}`);
    if (!el) return;
    if (el.type === 'checkbox') {
      if (el.checked !== value) el.checked = value;
    } else {
      // select ou autre input
      if (el.value !== value) el.value = value;
    }
  });
};

// ── Absence maladie ordinaire ─────────────────────────────────────────────────

let _absence = null;
// { active, type, dateDebut, dateFin, methode, joursType, heuresMois }

// Calcule Pâques (algorithme de Gauss/Meeus)
function _paquesDate(annee) {
  const a = annee % 19, b = Math.floor(annee / 100), c = annee % 100;
  const d = Math.floor(b / 4), e = b % 4, f = Math.floor((b + 8) / 25);
  const g = Math.floor((b - f + 1) / 3), h = (19 * a + b - d - g + 15) % 30;
  const i = Math.floor(c / 4), k = c % 4;
  const l = (32 + 2 * e + 2 * i - h - k) % 7;
  const m = Math.floor((a + 11 * h + 22 * l) / 451);
  const mois = Math.floor((h + l - 7 * m + 114) / 31);
  const jour = ((h + l - 7 * m + 114) % 31) + 1;
  return new Date(annee, mois - 1, jour);
}

function _joursFeries(annee) {
  const paques = _paquesDate(annee);
  const add = (d, n) => { const r = new Date(d); r.setDate(r.getDate() + n); return r; };
  const fmt = d => `${d.getFullYear()}-${String(d.getMonth()+1).padStart(2,'0')}-${String(d.getDate()).padStart(2,'0')}`;
  return new Set([
    `${annee}-01-01`, `${annee}-05-01`, `${annee}-05-08`,
    `${annee}-07-14`, `${annee}-08-15`, `${annee}-11-01`,
    `${annee}-11-11`, `${annee}-12-25`,
    fmt(add(paques, 1)),   // Lundi de Pâques
    fmt(add(paques, 39)),  // Ascension
    fmt(add(paques, 50)),  // Lundi de Pentecôte
  ]);
}

function _countJoursCalendaires(debut, fin) {
  const d1 = new Date(debut), d2 = new Date(fin);
  return Math.round((d2 - d1) / 86400000) + 1;
}

function _countJoursOuvrables(debut, fin) {
  const d1 = new Date(debut), d2 = new Date(fin);
  const feries = _joursFeries(d1.getFullYear());
  let n = 0, cur = new Date(d1);
  while (cur <= d2) {
    const dow = cur.getDay();
    const key = `${cur.getFullYear()}-${String(cur.getMonth()+1).padStart(2,'0')}-${String(cur.getDate()).padStart(2,'0')}`;
    if (dow !== 0 && !feries.has(key)) n++; // 0 = dimanche
    cur.setDate(cur.getDate() + 1);
  }
  return n;
}

function _countJoursOuvres(debut, fin) {
  const d1 = new Date(debut), d2 = new Date(fin);
  const feries = _joursFeries(d1.getFullYear());
  let n = 0, cur = new Date(d1);
  while (cur <= d2) {
    const dow = cur.getDay();
    const key = `${cur.getFullYear()}-${String(cur.getMonth()+1).padStart(2,'0')}-${String(cur.getDate()).padStart(2,'0')}`;
    if (dow !== 0 && dow !== 6 && !feries.has(key)) n++; // 0 = dim, 6 = sam
    cur.setDate(cur.getDate() + 1);
  }
  return n;
}

function _joursCalMois(dateStr) {
  const d = new Date(dateStr);
  return new Date(d.getFullYear(), d.getMonth() + 1, 0).getDate();
}

function _calcRetenue(brut, abs) {
  if (!abs || !abs.dateDebut || !abs.dateFin) return 0;
  const heuresMois = parseFloat(document.getElementById('d-h-mois')?.value) || 151.67;
  let nbJours, diviseur;
  switch (abs.methode) {
    case 'calendaire':
      nbJours  = _countJoursCalendaires(abs.dateDebut, abs.dateFin);
      diviseur = _joursCalMois(abs.dateDebut);
      break;
    // Jours moyens : le diviseur dépend du choix ouvré/ouvrable (toggle).
    case 'moyens':
      if (abs.joursType === 'ouvrables') {
        nbJours  = _countJoursOuvrables(abs.dateDebut, abs.dateFin);
        diviseur = 26;
      } else {
        nbJours  = _countJoursOuvres(abs.dateDebut, abs.dateFin);
        diviseur = 21.67;
      }
      break;
    // Anciens codes conservés par compatibilité (réglages mémorisés)
    case 'ouvrables':
      nbJours  = _countJoursOuvrables(abs.dateDebut, abs.dateFin);
      diviseur = 26;
      break;
    case 'ouvres':
      nbJours  = _countJoursOuvres(abs.dateDebut, abs.dateFin);
      diviseur = 21.67;
      break;
    case 'heures': {
      const joursRef = abs.joursType === 'ouvrables'
        ? _countJoursOuvrables(abs.dateDebut, abs.dateFin)
        : _countJoursOuvres(abs.dateDebut, abs.dateFin);
      nbJours  = joursRef;
      diviseur = abs.joursType === 'ouvrables'
        ? _countJoursOuvrables(
            `${new Date(abs.dateDebut).getFullYear()}-${String(new Date(abs.dateDebut).getMonth()+1).padStart(2,'0')}-01`,
            new Date(new Date(abs.dateDebut).getFullYear(), new Date(abs.dateDebut).getMonth()+1, 0).toISOString().slice(0,10)
          )
        : _countJoursOuvres(
            `${new Date(abs.dateDebut).getFullYear()}-${String(new Date(abs.dateDebut).getMonth()+1).padStart(2,'0')}-01`,
            new Date(new Date(abs.dateDebut).getFullYear(), new Date(abs.dateDebut).getMonth()+1, 0).toISOString().slice(0,10)
          );
      // retenue = (brut / diviseur_jours) × jours_absence = brut / heuresMois × heures_absence
      // Équivalent : (brut × joursAbsence) / joursRefMois
      break;
    }
    default:
      return 0;
  }
  if (diviseur <= 0) return 0;
  return Math.round(brut * nbJours / diviseur * 100) / 100;
}

function _absenceStats(abs) {
  if (!abs || !abs.dateDebut || !abs.dateFin) return null;
  const cal  = _countJoursCalendaires(abs.dateDebut, abs.dateFin);
  const ouv  = _countJoursOuvrables(abs.dateDebut, abs.dateFin);
  const ouvr = _countJoursOuvres(abs.dateDebut, abs.dateFin);
  const heuresMois = parseFloat(document.getElementById('d-h-mois')?.value) || 151.67;
  const salaireH   = _remBase > 0 ? (_remBase / heuresMois) : 0;
  const diviseurJ  = abs.methode === 'calendaire' ? _joursCalMois(abs.dateDebut)
                   : abs.methode === 'moyens'     ? (abs.joursType === 'ouvrables' ? 26 : 21.67)
                   : abs.methode === 'ouvrables'  ? 26
                   : abs.methode === 'ouvres'     ? 21.67
                   : (abs.joursType === 'ouvrables' ? ouv : ouvr); // heures : diviseur = jours ref mois
  const salaireJ   = diviseurJ > 0 ? Math.round(_remBase / diviseurJ * 100) / 100 : 0;
  return { cal, ouv, ouvr, salaireH: Math.round(salaireH * 100) / 100, salaireJ };
}

function _absenceLibelle(abs) {
  if (!abs) return '';
  if (abs.methode === 'moyens') {
    return `maladie · ${abs.joursType === 'ouvrables' ? '÷26 ouvrables' : '÷21,67 ouvrés'}`;
  }
  const labels = { calendaire: 'jours cal.', ouvrables: '÷26 ouvrables', ouvres: '÷21,67 ouvrés', heures: 'heures réelles' };
  return `maladie · ${labels[abs.methode] || abs.methode}`;
}

function _buildAbsencePanel(isMob) {
  const p = isMob ? 'm' : 'd';
  const abs = _absence || {};
  const type    = abs.type    || 'maladie';
  const methode = abs.methode || 'moyens';
  const jType   = abs.joursType || 'ouvres';
  const conv    = abs.conventionIDCC || '0016';
  if (!abs.joursType) { if (_absence) _absence.joursType = jType; }
  if (!abs.conventionIDCC && _absence) _absence.conventionIDCC = conv;

  const heuresMois = parseFloat(document.getElementById('d-h-mois')?.value) || 151.67;
  const retenue    = _absence ? _calcRetenue(_remBase, { ..._absence, joursType: jType }) : 0;
  const stats      = _absence?.dateDebut && _absence?.dateFin
    ? _absenceStats({ ..._absence, methode, joursType: jType }) : null;

  const baseNet = _modeSaisie === 'net';
  const previewHtml = stats ? `
    <div class="absence-preview">
      <span>Base : <b>${fmt(_remBase)}</b> ${baseNet ? '(brut reconstitué)' : '(brut)'}</span>
      <span>Jours calendaires : <b>${stats.cal}</b> &nbsp;|&nbsp; Ouvrables : <b>${stats.ouv}</b> &nbsp;|&nbsp; Ouvrés : <b>${stats.ouvr}</b></span>
      <span>Salaire horaire : <b>${fmt(stats.salaireH)}</b> &nbsp;|&nbsp; Salaire journalier : <b>${fmt(stats.salaireJ)}</b></span>
      <span>Retenue estimée : <b class="retenue-val">− ${fmt(retenue)}</b></span>
    </div>` : '';

  const toggleHtml = (methode !== 'calendaire') ? `
    <div class="absence-toggle-row">
      <span>Jours d'absence comptés en :</span>
      <button class="absence-toggle-btn${jType==='ouvres'?' active':''}" type="button"
        onclick="onAbsenceJoursType('${p}','ouvres')">Ouvrés (L–V)</button>
      <button class="absence-toggle-btn${jType==='ouvrables'?' active':''}" type="button"
        onclick="onAbsenceJoursType('${p}','ouvrables')">Ouvrables (L–S)</button>
    </div>` : '';

  return `
    <div class="absence-panel" id="absence-panel-${p}">
      <div class="absence-type-row">
        <label><input type="radio" name="abs-type-${p}" value="maladie" ${type==='maladie'?'checked':''} onchange="onAbsenceTypeChange('${p}','maladie')"> Absence maladie ordinaire</label>
        <label style="opacity:0.45" title="Bientôt disponible"><input type="radio" name="abs-type-${p}" value="conge" disabled> Congé payé</label>
      </div>
      ${type === 'maladie' ? `
      <div class="absence-dates-row">
        <span>Du</span>
        <input type="date" id="abs-debut-${p}" value="${abs.dateDebut||''}" oninput="onAbsenceChange('${p}')">
        <span>au</span>
        <input type="date" id="abs-fin-${p}" value="${abs.dateFin||''}" oninput="onAbsenceChange('${p}')">
      </div>
      <div class="absence-conv-row">
        <span>Convention collective (maintien) :</span>
        <select id="abs-conv-${p}" onchange="onAbsenceConvention('${p}', this.value)">
          <option value="0016" ${conv==='0016'?'selected':''}>IDCC 0016 — Transport routier</option>
        </select>
      </div>
      <div class="absence-methode-row">
        ${[
          ['calendaire', `Jours calendaires (÷ ${abs.dateDebut ? _joursCalMois(abs.dateDebut) : 'jours du mois'})`],
          ['moyens',     `Jours moyens (÷ ${jType === 'ouvrables' ? '26' : '21,67'})`],
          ['heures',     `Heures réelles (÷ ${Math.round(heuresMois)} h/mois)`],
        ].map(([v, lbl]) =>
          `<label><input type="radio" name="abs-methode-${p}" value="${v}" ${methode===v?'checked':''} onchange="onAbsenceChange('${p}')"> ${lbl}</label>`
        ).join('')}
      </div>
      ${toggleHtml}
      ${previewHtml}
      <div class="absence-actions">
        <button class="btn-absence-apply" onclick="appliquerAbsence('${p}')">Appliquer</button>
        <button class="btn-absence-clear" onclick="effacerAbsence()">Effacer</button>
      </div>` : ''}
    </div>`;
}

window.toggleAbsencePanel = function(p) {
  if (!_absence) {
    _absence = { active: false, type: 'maladie', dateDebut: '', dateFin: '',
      methode: 'moyens', joursType: 'ouvres', conventionIDCC: '0016' };
    _reRenderRemInPlace(); // insère le panneau dans le DOM
    return;
  }
  const panel = document.getElementById(`absence-panel-${p}`);
  if (panel) {
    const visible = panel.style.display !== 'none';
    panel.style.display = visible ? 'none' : '';
    document.getElementById(`btn-absence-${p}`)?.classList.toggle('active', !visible);
  }
};

window.onAbsenceTypeChange = function(p, val) {
  if (!_absence) return;
  _absence.type = val;
  _refreshAbsencePanel(p);
};

window.onAbsenceChange = function(p) {
  if (!_absence) return;
  const methodeEl = document.querySelector(`input[name="abs-methode-${p}"]:checked`);
  _absence.methode  = methodeEl ? methodeEl.value : 'moyens';
  _absence.dateDebut = document.getElementById(`abs-debut-${p}`)?.value || '';
  _absence.dateFin   = document.getElementById(`abs-fin-${p}`)?.value   || '';
  _refreshAbsencePanel(p);
};

window.onAbsenceJoursType = function(p, val) {
  if (!_absence) return;
  _absence.joursType = val;
  _refreshAbsencePanel(p);
};

window.onAbsenceConvention = function(p, val) {
  if (!_absence) return;
  _absence.conventionIDCC = val;
  _refreshAbsencePanel(p);
};

function _refreshAbsencePanel(p) {
  const existing = document.getElementById(`absence-panel-${p}`);
  if (!existing) return;
  const temp = document.createElement('div');
  temp.innerHTML = _buildAbsencePanel(p === 'm');
  existing.replaceWith(temp.firstElementChild);
}

window.appliquerAbsence = function(p) {
  if (!_absence?.dateDebut || !_absence?.dateFin) return;
  _absence.active = true;
  _reRenderRemInPlace();
  _triggerRecalculate();
};

window.effacerAbsence = function() {
  _absence = null;
  _reRenderRemInPlace();
  _triggerRecalculate();
};

// Affiche/masque les sélecteurs canton + tarif IS selon la checkbox assujetti-IS
window.onToggleAssujetti = function(checked) {
  ['d', 'm'].forEach(p => {
    const detail = document.getElementById(`${p}-ch-is-detail`);
    if (detail) detail.style.display = checked ? '' : 'none';
  });
};

// ── Listeners ────────────────────────────────────────────────────────────────
document.getElementById("d-calc").addEventListener("click", () => calculate("desktop"));
document.getElementById("m-calc").addEventListener("click", () => calculate("mobile"));
document.getElementById("a-calc").addEventListener("click", calculerAnnee);

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

// Pools par pays — valeurs signées en % : négatif = F gagne moins, positif = F gagne plus
// Sources : INSEE Focus 377 (FR 2024), OFS LSE (CH 2024), STATEC/Eurostat (LU 2024),
//           ISTAT SES 2022 / Eurostat 2023 (IT), Statistique Canada EPA 2024 (CA/QC)
const ECART_POOLS = {
  france:     [-4, -9, -13, -17, -22],
  suisse:     [-8, -10, -12, -14, -16],
  luxembourg: [+1, +1,  +1,   0,  -1],
  italie:     [-2, -4,  -4,  -6],
  espagne:    [-6, -10, -13, -16, -19],
  portugal:   [-8, -11, -13, -15],
  belgique:   [-6, -9, -11, -13, -15],
  allemagne:  [-14, -16, -18, -20, -22],
  canada:     [-6, -10, -12, -14, -17],
  quebec:     [-6, -10, -12, -14, -17],
  angleterre: [-7, -12, -15, -18, -21],
  japon:      [-20, -23, -25, -27, -30],
  chine:      [-10, -14, -18, -22, -25],
  paysbas:    [-6, -10, -13, -15, -18],
  australie:  [-8, -11, -13, -15, -18],
  nouvellezelande: [-6, -9, -11, -13, -16],
  pologne:    [-3, -5, -7, -9, -12],
  coree:      [-15, -20, -25, -28, -31],
  andorre:    [-5, -8, -10, -12, -15],
  monaco:     [-6, -9, -11, -13, -16],
  danemark:   [-3, -6, -8, -10, -13],
  finlande:   [-4, -7, -9, -11, -14],
  suede:      [-5, -8, -10, -12, -14],
  estonie:    [-12, -16, -20, -23, -27],
  lettonie:   [-9, -13, -17, -20, -24],
  lituanie:   [-7, -10, -13, -15, -18],
  autriche:   [-12, -16, -19, -22, -25],
  tchequie:   [-12, -15, -18, -21, -24],
  slovaquie:  [-8, -11, -14, -16, -19],
  hongrie:    [-10, -13, -16, -18, -21],
  slovenie:   [-3, -5, -7, -9, -12],
  grece:      [-6, -9, -11, -13, -16],
  chypre:     [-8, -11, -14, -17, -20],
  malte:      [-7, -10, -12, -14, -17],
  croatie:    [-6, -9, -11, -13, -16],
  // Eurostat gender pay gap (forme non ajustée) : IE ≈ 10 %, RO ≈ 4 % (un des plus bas UE), BG ≈ 13 %.
  irlande:    [-6, -9, -11, -13, -16],
  roumanie:   [-3, -5, -7, -9, -12],
  bulgarie:   [-6, -10, -13, -16, -19],
  // Écart de rémunération H/F (formes non ajustées) : US ≈ 17 %, MX ≈ 14 %,
  // BR ≈ 21 %, AE ≈ variable, IN ≈ 20 % (écarts déclarés, sources OIT/OCDE).
  etatsunis:  [-8, -12, -15, -18, -21],
  mexique:    [-6, -10, -13, -15, -18],
  bresil:     [-8, -12, -16, -19, -22],
  emirats:    [-10, -14, -18, -22, -26],
  inde:       [-10, -15, -20, -24, -28],
};

function _getActivePays() {
  for (const p of ['suisse', 'luxembourg', 'italie', 'espagne', 'portugal', 'belgique', 'allemagne', 'canada', 'quebec', 'angleterre', 'japon', 'chine', 'paysbas', 'australie', 'nouvellezelande', 'pologne', 'coree', 'andorre', 'monaco', 'danemark', 'finlande', 'suede', 'estonie', 'lettonie', 'lituanie', 'autriche', 'tchequie', 'slovaquie', 'hongrie', 'slovenie', 'grece', 'chypre', 'malte', 'croatie', 'irlande', 'roumanie', 'bulgarie', 'etatsunis', 'mexique', 'bresil', 'emirats', 'inde']) {
    if (document.getElementById('d-' + p)?.checked) return p;
    if (document.getElementById('m-' + p)?.checked) return p;
  }
  return 'france';
}

function _drawEcartPct() {
  const pool = ECART_POOLS[_getActivePays()];
  return pool[Math.floor(Math.random() * pool.length)];
}

let _ecartActif = 0;    // mémorise l'écart appliqué H→F pour le retour F→H
let _ecartTire  = false; // tirage unique pour toute la session

let _genre = 'H';

// Mémoire des noms par sexe — chaque sexe conserve son propre couple prénom/nom.
// Initialisé au tirage au sort, puis écrasé de façon permanente dès saisie.
let _noms = { H: { prenom: '', nom: '' }, F: { prenom: '', nom: '' } };

function _heroRandom(list) {
  return list[Math.floor(Math.random() * list.length)];
}

function _setNomFields(prenom, nom) {
  ['d-prenom', 'm-prenom'].forEach(id => { const el = document.getElementById(id); if (el) el.value = prenom; });
  ['d-nom',    'm-nom'   ].forEach(id => { const el = document.getElementById(id); if (el) el.value = nom;    });
}

// Affiche le couple mémorisé pour un sexe donné dans les deux vues (desktop + mobile).
function _applyNoms(genre) {
  _setNomFields(_noms[genre].prenom, _noms[genre].nom);
}

// Enregistre, pour le sexe donné, les valeurs saisies dans la vue active.
function _captureNoms(genre, prefix) {
  const px = prefix || (document.body.classList.contains('is-mobile') ? 'm' : 'd');
  const prenom = document.getElementById(px + '-prenom')?.value ?? '';
  const nom    = document.getElementById(px + '-nom')?.value ?? '';
  _noms[genre] = { prenom, nom };
}

function _syncToggleUI(genre, showHint = false) {
  const onH = genre === 'H';
  ['d-hf', 'm-hf'].forEach(id => {
    const el = document.getElementById(id);
    if (!el) return;
    el.classList.toggle('is-h', onH);
    el.classList.toggle('is-f', !onH);
    el.setAttribute('aria-checked', onH ? 'false' : 'true');
  });
  if (showHint) {
    document.querySelectorAll('.genre-ecart-hint').forEach(el => {
      el.textContent = onH ? el.dataset.textHf : el.dataset.textFh;
      el.style.display = 'inline';
    });
  }
}

window.toggleGenre = function() { window.setGenre(_genre === 'H' ? 'F' : 'H'); };

window.setGenre = function(genre) {
  if (genre === _genre) return;

  // Mémorise les noms de la vue active pour le sexe quitté, puis affiche
  // ceux mémorisés (tirage au sort ou saisie) pour le sexe demandé.
  _captureNoms(_genre);
  _applyNoms(genre);

  const simuleEcart = !!document.getElementById('d-ecart-actif')?.checked
                   || !!document.getElementById('m-ecart-actif')?.checked;

  if (simuleEcart) {
    // Premier passage (quel que soit le sens) : tirage unique dans le pool du pays actif
    if (!_ecartTire) { _ecartActif = _drawEcartPct(); _ecartTire = true; }
    const e = _ecartActif / 100;
    const facteur = genre === 'F' ? (1 + e) : (1 / (1 + e));

    const abs = Math.abs(_ecartActif);
    const absHF = Math.abs(Math.round(e / (1 + e) * 100));
    document.querySelectorAll('.genre-ecart-hint').forEach(el => {
      if (_ecartActif < 0) {
        el.dataset.textFh = `// −${abs} % · écart salarial F/H`;
        el.dataset.textHf = `// +${absHF} % · écart salarial H/F`;
      } else if (_ecartActif > 0) {
        el.dataset.textFh = `// +${abs} % · avantage F/H`;
        el.dataset.textHf = `// −${absHF} % · avantage F/H`;
      } else {
        el.dataset.textFh = `// ± 0 % · parité salariale`;
        el.dataset.textHf = `// ± 0 % · parité salariale`;
      }
    });

    ['d-brut', 'm-brut'].forEach(id => {
      const el = document.getElementById(id);
      if (el) el.value = Math.round(parseFloat(el.value) * facteur);
    });
  }

  _genre = genre;
  _syncToggleUI(genre, simuleEcart);
};

window.onToggleEcartActif = function(checked) {
  if (checked) {
    _syncToggleUI(_genre, true);
  } else {
    _ecartTire = false; // prochain recochage → nouveau tirage dans le pool
    document.querySelectorAll('.genre-ecart-hint').forEach(el => { el.style.display = 'none'; });
  }
};

// ── Annuel — disponible uniquement pour un bulletin France secteur privé ──────
function _updateAnnuelBtn() {
  const isFrPriv = lastBulletin?.salarie?.pays === 'france';
  const btn = document.getElementById('btn-ann');
  if (btn) btn.style.display = isFrPriv ? '' : 'none';
  if (!isFrPriv && document.body.classList.contains('is-annuel')) setView('desktop');
}

// ── Hercule Compta ────────────────────────────────────────────────────────────
function herculeInit() {
  const gate    = document.getElementById('herc-gate');
  const content = document.getElementById('herc-content');
  const isFR    = lastBulletin?.salarie?.pays === 'france';

  if (!isFR) {
    const msg = document.getElementById('herc-gate-msg');
    if (msg) msg.textContent = lastBulletin
      ? 'Hercule n\'est disponible que pour les bulletins France secteur privé (y compris Alsace-Moselle).'
      : 'Calculez d\'abord un bulletin France secteur privé pour accéder au journal de paie.';
    if (gate)    gate.style.display    = '';
    if (content) content.style.display = 'none';
    return;
  }
  if (gate)    gate.style.display    = 'none';
  if (content) content.style.display = '';

  const b    = lastBulletin;
  const cots = b.cotisations;
  const brut = parseFloat(b.brut);
  const net  = parseFloat(b.net_a_payer);
  const pas  = calculerPas(parseFloat(b.net_imposable)).total;

  // ── Agrégation par code ───────────────────────────────────────────────────
  const SS   = ['SS_MALADIE','SS_VIEILLESSE_PLAF','SS_VIEILLESSE_DEPLAF','FAMILLE','AT_MP','ALSACE_MOSELLE_MALADIE'];
  const CSG  = ['CSG_DEDUCTIBLE','CSG_NON_DEDUCTIBLE','CRDS'];
  const PREV = ['PREVOYANCE_CADRE_MIN'];
  const RCC  = ['AGIRC_ARRCO_T1','AGIRC_ARRCO_T2','AGIRC_ARRCO_CEG_T1'];

  const sumSal = codes => cots.filter(c => codes.includes(c.code)).reduce((a,c) => a + Math.abs(parseFloat(c.montant_sal)), 0);
  const sumPat = codes => cots.filter(c => codes.includes(c.code)).reduce((a,c) => a + Math.abs(parseFloat(c.montant_pat)), 0);
  const sumCode = (code, field) => { const c = cots.find(x => x.code === code); return c ? Math.abs(parseFloat(c[field])) : 0; };

  const ss431_sal   = sumSal(SS);
  const csg4378_sal = sumSal(CSG);
  const cho4379_sal = sumCode('CHOMAGE', 'montant_sal');
  const rcc437_sal  = sumSal([...RCC, ...PREV]);

  const ss431_pat   = sumPat(SS);
  const cho4379_pat = sumCode('CHOMAGE', 'montant_pat');
  const rcc437_pat  = sumPat(RCC);
  const prev6452    = sumPat(PREV);
  const fillon6419  = sumCode('REDUCTION_FILLON', 'montant_pat');
  const urssaf6451  = ss431_pat - fillon6419;   // net Fillon

  // ── Helpers ───────────────────────────────────────────────────────────────
  const fmtE = v => v > 0.005 ? v.toLocaleString('fr-FR',{minimumFractionDigits:2,maximumFractionDigits:2}) + ' €' : '—';
  const row = (sens, num, lib, d, c) =>
    `<tr><td class="${sens==='D'?'herc-d':'herc-c'}">${sens}</td><td><span class="herc-num">${num}</span></td><td>${lib}</td><td>${fmtE(d)}</td><td>${fmtE(c)}</td></tr>`;
  const rowTotal = (label, d, c) =>
    `<tr class="herc-total"><td colspan="3">${label}</td><td>${fmtE(d)}</td><td>${fmtE(c)}</td></tr>`;

  // ── Info bulletin ─────────────────────────────────────────────────────────
  const info = document.getElementById('herc-bulletin-info');
  if (info) info.innerHTML = `Bulletin de <strong>${b.salarie.prenom} ${b.salarie.nom}</strong> · Brut ${fmtE(brut)} · ${b.salarie.alsace_moselle ? 'Alsace-Moselle · ' : ''}${b.salarie.entreprise_adaptee ? '♿ Entreprise adaptée · ' : ''}${b.salarie.statut === 'cadre' ? 'Cadre' : 'Non-cadre'}`;

  // ── Écriture 1 : Constatation de la rémunération ──────────────────────────
  const totalRetenues = ss431_sal + csg4378_sal + cho4379_sal + rcc437_sal + pas;
  const tbody1 = [
    row('D', '641', 'Rémunérations du personnel',                brut, 0),
    row('C', '421', 'Personnel — Rémunérations dues',             0, net),
    ss431_sal   > 0.005 ? row('C', '431', 'Sécurité sociale (part salariale)',           0, ss431_sal)   : '',
    csg4378_sal > 0.005 ? row('C', '4378','CSG · CRDS',                                  0, csg4378_sal) : '',
    cho4379_sal > 0.005 ? row('C', '4379','Assurance chômage (part salariale)',           0, cho4379_sal) : '',
    rcc437_sal  > 0.005 ? row('C', '437', 'Retraite complémentaire · Prévoyance (sal.)', 0, rcc437_sal)  : '',
    pas         > 0.005 ? row('C', '444', 'État — Prélèvement à la source',              0, pas)         : '',
    rowTotal('TOTAL', brut, net + totalRetenues),
  ].join('');
  document.querySelector('#herc-jnl-1 tbody').innerHTML = tbody1;
  const diff1 = Math.abs(brut - (net + totalRetenues));
  document.getElementById('herc-eq-1').textContent =
    diff1 < 0.02 ? '∑ Débit = ∑ Crédit ✓' : `Δ = ${fmtE(diff1)} (écart d'arrondi)`;

  // ── Écriture 2 : Charges patronales ──────────────────────────────────────
  const debit2  = urssaf6451 + prev6452 + rcc437_pat + cho4379_pat;
  const credit2 = ss431_pat + rcc437_pat + prev6452 + cho4379_pat - fillon6419;
  const tbody2 = [
    urssaf6451  > 0.005 ? row('D', '6451','Cotisations URSSAF (net réduction Fillon)',        urssaf6451,  0) : '',
    prev6452    > 0.005 ? row('D', '6452','Cotisations prévoyance (part patronale)',           prev6452,    0) : '',
    rcc437_pat  > 0.005 ? row('D', '6453','Cotisations retraite complémentaire (pat.)',        rcc437_pat,  0) : '',
    cho4379_pat > 0.005 ? row('D', '6454','Cotisations France Travail (part patronale)',       cho4379_pat, 0) : '',
    fillon6419  > 0.005 ? row('C', '6419','Remboursement réduction Fillon',                   0, fillon6419)  : '',
    ss431_pat   > 0.005 ? row('C', '431', 'Sécurité sociale (part patronale)',                0, ss431_pat)   : '',
    rcc437_pat  > 0.005 ? row('C', '437', 'Retraite complémentaire · Prévoyance (pat.)',      0, rcc437_pat + prev6452) : '',
    cho4379_pat > 0.005 ? row('C', '4379','Assurance chômage (part patronale)',               0, cho4379_pat) : '',
    rowTotal('TOTAL', debit2, credit2),
  ].join('');
  document.querySelector('#herc-jnl-2 tbody').innerHTML = tbody2;
  const diff2 = Math.abs(debit2 - credit2);
  document.getElementById('herc-eq-2').textContent =
    diff2 < 0.02 ? '∑ Débit = ∑ Crédit ✓' : `Δ = ${fmtE(diff2)} (écart d'arrondi)`;

  // ── Plan comptable — tous comptes classés par numéro ─────────────────────
  const pca = [
    { num:'421',  lib:'Personnel — Rémunérations dues',          categ:'Passif salarié',      d:0,          c:net },
    { num:'431',  lib:'Sécurité sociale',                        categ:'Passif social',       d:0,          c:ss431_sal + ss431_pat },
    { num:'437',  lib:'Retraite complémentaire · Prévoyance',    categ:'Passif social',       d:0,          c:rcc437_sal + rcc437_pat + prev6452 },
    { num:'4378', lib:'CSG · CRDS',                              categ:'Passif social',       d:0,          c:csg4378_sal },
    { num:'4379', lib:'Assurance chômage',                       categ:'Passif social',       d:0,          c:cho4379_sal + cho4379_pat },
    pas > 0.005 ?
    { num:'444',  lib:'État — Prélèvement à la source',          categ:'Impôt retenu',        d:0,          c:pas } : null,
    { num:'641',  lib:'Rémunérations du personnel',              categ:'Charge salariale',    d:brut,       c:0 },
    urssaf6451 > 0.005 ?
    { num:'6419', lib:'Remboursement réduction Fillon',          categ:'Allègement',          d:0,          c:fillon6419 } : null,
    urssaf6451 > 0.005 ?
    { num:'6451', lib:'Cotisations URSSAF (net Fillon)',         categ:'Charge patronale',    d:urssaf6451, c:0 } : null,
    prev6452 > 0.005 ?
    { num:'6452', lib:'Cotisations prévoyance',                  categ:'Charge patronale',    d:prev6452,   c:0 } : null,
    rcc437_pat > 0.005 ?
    { num:'6453', lib:'Cotisations retraite complémentaire',     categ:'Charge patronale',    d:rcc437_pat, c:0 } : null,
    cho4379_pat > 0.005 ?
    { num:'6454', lib:'Cotisations France Travail',              categ:'Charge patronale',    d:cho4379_pat,c:0 } : null,
  ].filter(Boolean);

  pca.sort((a, b) => a.num.localeCompare(b.num, undefined, { numeric: true }));

  const totalD = pca.reduce((a,x) => a+x.d, 0);
  const totalC = pca.reduce((a,x) => a+x.c, 0);

  const PCG_CLASSES = {
    '4': 'Classe 4 — Comptes de tiers',
    '6': 'Classe 6 — Charges de personnel',
  };
  const RA = 'text-align:right;white-space:nowrap;font-variant-numeric:tabular-nums';

  let lastClasse = '';
  const pcaRows = [];
  for (const x of pca) {
    const cl = x.num[0];
    if (cl !== lastClasse && PCG_CLASSES[cl]) {
      pcaRows.push(`<tr class="herc-pca-classe"><td colspan="5">${PCG_CLASSES[cl]}</td></tr>`);
      lastClasse = cl;
    }
    pcaRows.push(`<tr><td><span class="herc-num">${x.num}</span></td><td>${x.lib}</td><td style="color:var(--dim);font-size:0.6rem;text-align:left">${x.categ}</td><td style="${RA}">${fmtE(x.d)}</td><td style="${RA}">${fmtE(x.c)}</td></tr>`);
  }
  pcaRows.push(`<tr class="herc-total"><td colspan="3">TOTAL</td><td style="${RA}">${fmtE(totalD)}</td><td style="${RA}">${fmtE(totalC)}</td></tr>`);
  document.querySelector('#herc-pca tbody').innerHTML = pcaRows.join('');

  // ── Virements par organisme ───────────────────────────────────────────────
  const VIR_ORGS = [
    {
      id: 'urssaf',
      nom: 'URSSAF',
      desc: 'Sécurité sociale · CSG · CRDS',
      categ: ['Sécurité Sociale', 'CSG/CRDS'],
      reduction: fillon6419,
      reductionLib: 'Réduction Fillon déduite',
    },
    {
      id: 'agirc',
      nom: 'AGIRC-ARRCO',
      desc: 'Retraite complémentaire',
      categ: ['Retraite complémentaire'],
      reduction: 0,
    },
    {
      id: 'ft',
      nom: 'France Travail',
      desc: 'Assurance chômage',
      categ: ['Chômage'],
      reduction: 0,
    },
    {
      id: 'prev',
      nom: 'Prévoyance',
      desc: 'Prévoyance collective',
      categ: ['Prévoyance'],
      reduction: 0,
    },
  ];

  const fmtPct = v => parseFloat(v) === 0 ? '—' : parseFloat(v).toFixed(2).replace('.', ',') + ' %';

  const virCartes = [];
  for (const org of VIR_ORGS) {
    const lignes = cots.filter(c => org.categ.includes(c.categorie));
    const brut_total = lignes.reduce((a, c) => a + Math.abs(parseFloat(c.montant_sal)) + Math.abs(parseFloat(c.montant_pat)), 0);
    const total = brut_total - org.reduction;
    if (total <= 0.005) continue;

    const detailRows = lignes.map(c => {
      const sal = Math.abs(parseFloat(c.montant_sal));
      const pat = Math.abs(parseFloat(c.montant_pat));
      const base = parseFloat(c.base);
      return `<tr>
        <td>${c.libelle}</td>
        <td>${fmtE(base)}</td>
        <td>${fmtPct(c.taux_sal)}</td>
        <td>${sal > 0.005 ? fmtE(sal) : '—'}</td>
        <td>${fmtPct(c.taux_pat)}</td>
        <td>${pat > 0.005 ? fmtE(pat) : '—'}</td>
        <td>${fmtE(sal + pat)}</td>
      </tr>`;
    }).join('');

    const reductionRow = org.reduction > 0.005
      ? `<tr class="herc-vir-reduc"><td>${org.reductionLib}</td><td>—</td><td>—</td><td>—</td><td>—</td><td>—</td><td>− ${fmtE(org.reduction)}</td></tr>`
      : '';

    const totalRow = `<tr class="herc-vir-subtot"><td colspan="6">Total virement</td><td>${fmtE(total)}</td></tr>`;

    virCartes.push(`
      <div class="herc-vir-card" id="herc-vir-${org.id}">
        <div class="herc-vir-hdr" onclick="hercToggleVir('${org.id}')">
          <span class="herc-vir-nom">${org.nom}</span>
          <span class="herc-vir-desc">${org.desc}</span>
          <span class="herc-vir-amt">${fmtE(total)}</span>
          <span class="herc-vir-caret">▶</span>
        </div>
        <div class="herc-vir-detail">
          <table class="herc-vir-table">
            <thead><tr><th>Cotisation</th><th>Base</th><th>Tx sal.</th><th>Mt sal.</th><th>Tx pat.</th><th>Mt pat.</th><th>Total</th></tr></thead>
            <tbody>${detailRows}${reductionRow}${totalRow}</tbody>
          </table>
        </div>
      </div>`);
  }

  document.getElementById('herc-virements').innerHTML = virCartes.join('');
}

window.hercToggleVir = function(id) {
  document.getElementById('herc-vir-' + id)?.classList.toggle('open');
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

// ── Gaabrielle RH ─────────────────────────────────────────────────────────────
// Effectif fictif. sexe H/F, naissance ISO (= embauche − âge à l'embauche), rqth :
// 11/20 RQTH = 55 %, cohérent avec le seuil légal d'une entreprise adaptée.
const GAAB_EMPLOYES = [
  { mat:'XN-001', nom:'de Riv',        prenom:'Geralt',   sexe:'H', naissance:'1978-02-04', rqth:true,  embauche:'2016-04-11', poste:'Chasseur de Bugs Mutants — Résilience Maximale',                 bh:44.00, etp:100 },
  { mat:'XN-002', nom:'Belacqua',      prenom:'Lyra',     sexe:'F', naissance:'2001-08-18', rqth:false, embauche:'2023-09-01', poste:'Cartographe des Mondes Parallèles Contractuels',                bh:16.50, etp:80  },
  { mat:'XN-003', nom:'Vimes',         prenom:'Sam',      sexe:'H', naissance:'1971-08-17', rqth:true,  embauche:'2017-02-20', poste:'Commissaire aux Anomalies Comportementales',                    bh:38.50, etp:100 },
  { mat:'XN-004', nom:'Granger',       prenom:'Hermione', sexe:'F', naissance:'1992-09-01', rqth:true,  embauche:'2020-09-01', poste:'Directrice des Procédures Magiques et de la Conformité',        bh:31.00, etp:100 },
  { mat:'XN-005', nom:'de Melniboné',  prenom:'Elric',    sexe:'H', naissance:'1985-02-24', rqth:false, embauche:'2018-06-15', poste:'Canalisateur des Flux d\'Entropie Numérique',                   bh:51.00, etp:100 },
  { mat:'XN-006', nom:'du Rohan',      prenom:'Eowyn',    sexe:'F', naissance:'1988-03-12', rqth:true,  embauche:'2019-11-03', poste:'Cheffe de Projet Déconstruction des Obstacles Genrés',          bh:24.50, etp:100 },
  { mat:'XN-007', nom:'la Légende',    prenom:'Druss',    sexe:'H', naissance:'1962-07-22', rqth:true,  embauche:'2014-08-30', poste:'Directeur des Opérations Physiques Irréversibles',              bh:48.00, etp:100 },
  { mat:'XN-008', nom:'Caldin',        prenom:'Ellana',   sexe:'F', naissance:'1994-05-24', rqth:false, embauche:'2021-03-08', poste:'Analyste en Trajectoires Inconventionnelles',                   bh:19.75, etp:80  },
  { mat:'XN-009', nom:'Neuf-Doigts',   prenom:'Logen',    sexe:'H', naissance:'1982-08-17', rqth:true,  embauche:'2020-01-13', poste:'Expert en Gestion de Crises Légèrement Incontrôlables',         bh:29.00, etp:100 },
  { mat:'XN-010', nom:'Maljinn',       prenom:'Ferro',    sexe:'F', naissance:'1992-06-12', rqth:false, embauche:'2022-05-16', poste:'Responsable de la Désintégration des Processus Obsolètes',      bh:21.50, etp:100 },
  { mat:'XN-011', nom:'Grands-Pas',    prenom:'Aragorn',  sexe:'H', naissance:'1974-07-11', rqth:false, embauche:'2015-07-22', poste:'Directeur Général des Transitions de Paradigme',                bh:58.00, etp:100 },
  { mat:'XN-012', nom:'Garlick',       prenom:'Magrat',   sexe:'F', naissance:'1983-10-15', rqth:true,  embauche:'2018-04-01', poste:'Consultante en Phytothérapie Algorithmique',                    bh:20.00, etp:80  },
  { mat:'XN-013', nom:'Shannow',       prenom:'Jon',      sexe:'H', naissance:'1974-10-07', rqth:false, embauche:'2019-06-09', poste:'Pisteur de Tendances Post-Apocalyptiques',                      bh:33.75, etp:100 },
  { mat:'XN-014', nom:"Gil'Sayan",     prenom:'Ewilan',   sexe:'F', naissance:'2002-11-13', rqth:true,  embauche:'2024-02-19', poste:'Ingénieure en Dessin de Réalités Augmentées',                   bh:15.50, etp:80  },
  { mat:'XN-015', nom:'Dhibi',         prenom:'Salim',    sexe:'H', naissance:'1991-07-06', rqth:false, embauche:'2021-10-04', poste:'Archiviste des Compétences Émergentes Non-Homologuées',         bh:26.50, etp:100 },
  { mat:'XN-016', nom:'la Guerrière',  prenom:'Sigarni',  sexe:'F', naissance:'1965-05-18', rqth:true,  embauche:'2013-11-18', poste:'Directrice des Restructurations Stratégiques',                  bh:38.00, etp:100 },
  { mat:'XN-017', nom:'le Magi',       prenom:'Bayaz',    sexe:'H', naissance:'1950-03-01', rqth:false, embauche:'2011-03-01', poste:'Conseiller Exécutif en Manipulation des Lois Fondamentales',    bh:62.00, etp:100 },
  { mat:'XN-018', nom:'la Nord',       prenom:'Rikke',    sexe:'F', naissance:'1995-03-13', rqth:true,  embauche:'2022-08-22', poste:'Prévisionniste des Déviations Sociales Involontaires',          bh:22.25, etp:100 },
  { mat:'XN-019', nom:"l'Enchanteur",  prenom:'Merlin',   sexe:'H', naissance:'1942-01-22', rqth:false, embauche:'2009-05-12', poste:'Président du Conseil des Visions à Long Terme',                 bh:55.00, etp:100 },
  { mat:'XN-020', nom:'la Magicienne', prenom:'Tanaquil', sexe:'F', naissance:'1981-08-12', rqth:true,  embauche:'2017-09-25', poste:'Architecte des Sortilèges Organisationnels',                    bh:27.50, etp:80  },
];

function _gaabSalStr(bh, type) {
  const bm = bh * 151.67;
  const nm = bm * 0.79;
  const nh = nm / 151.67;
  const h = v => v.toLocaleString('fr-FR', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
  switch (type) {
    case 'bh': return h(bh)  + ' €/h';
    case 'bm': return h(bm)  + ' €';
    case 'nm': return h(nm)  + ' €';
    case 'nh': return h(nh)  + ' €/h';
  }
}

let _gaabInited = false;

function gaabInit() {
  if (_gaabInited) return;
  _gaabInited = true;

  // Initiales avec tooltip CSS
  const initDiv = document.getElementById('gaab-initiales');
  if (initDiv) {
    initDiv.innerHTML = GAAB_EMPLOYES.map((e, i) => {
      const init = e.prenom[0] + '.' + e.nom[0];
      const sep  = i > 0 ? '<span class="gaab-sep"> ; </span>' : '';
      return sep + `<span class="gaab-init" data-name="${e.prenom} ${e.nom}">${init}</span>`;
    }).join('');
  }

  // Tableau
  const tbody = document.getElementById('gaab-tbody');
  if (tbody) {
    tbody.innerHTML = GAAB_EMPLOYES.map(e => {
      const etpCls = e.etp < 100 ? 'style="color:var(--yellow)"' : 'style="color:var(--dim)"';
      return `<tr>
        <td class="gaab-mat">${e.mat}</td>
        <td>${e.nom}</td>
        <td>${e.prenom}</td>
        <td>${e.embauche}</td>
        <td class="gaab-age">${formatDate(e.naissance)}</td>
        <td class="gaab-poste">${e.poste}</td>
        <td class="gaab-etp" ${etpCls}>${e.etp} %</td>
        <td class="gaab-sal" data-bh="${e.bh}">${_gaabSalStr(e.bh, 'bh')}</td>
      </tr>`;
    }).join('');
  }

  gaabRenderRqth();
  gaabRenderPyramide();
}

// Âge courant (années révolues) à partir de la date de naissance ISO.
function _gaabAge(naissanceIso) {
  const [y, m, d] = naissanceIso.split('-').map(Number);
  const [ty, tm, td] = DATE_TODAY.split('-').map(Number);
  let age = ty - y;
  if (tm < m || (tm === m && td < d)) age--;
  return age;
}

// ── Taux de RQTH ───────────────────────────────────────────────────────────────
function gaabRenderRqth() {
  const host = document.getElementById('gaab-rqth');
  if (!host) return;
  const total  = GAAB_EMPLOYES.length;
  const nRqth  = GAAB_EMPLOYES.filter(e => e.rqth).length;
  const taux   = total ? (nRqth / total) * 100 : 0;
  const txtTaux = taux.toLocaleString('fr-FR', { minimumFractionDigits: 1, maximumFractionDigits: 1 });
  const SEUIL_EA = 55;
  const atteint  = taux >= SEUIL_EA;
  const w = Math.min(100, taux / SEUIL_EA * 100).toFixed(1); // jauge : 100 % = seuil EA atteint
  host.innerHTML = `
    <div class="gaab-rqth-row">
      <span class="gaab-rqth-pct">${txtTaux} %</span>
      <span class="gaab-rqth-frac">${nRqth} / ${total} salariés RQTH</span>
      <span class="gaab-rqth-badge ${atteint ? 'ok' : 'ko'}">${atteint ? '✓ seuil entreprise adaptée atteint' : '✗ sous le seuil entreprise adaptée'}</span>
    </div>
    <div class="gaab-rqth-bar"><span style="width:${w}%"></span><i class="gaab-rqth-seuil" title="Seuil EA 55 %"></i></div>
    <div class="gaab-rqth-note">Entreprise adaptée : au moins <b>55 %</b> de l'effectif en situation de handicap (RQTH).
    Pour mémoire, l'obligation d'emploi (OETH) des entreprises ordinaires d'au moins 20 salariés est de <b>6 %</b>.</div>`;
}

// ── Pyramide des âges H/F ──────────────────────────────────────────────────────
function gaabRenderPyramide() {
  const host = document.getElementById('gaab-pyramide');
  if (!host) return;

  // Tranches de 5 ans, de la plus âgée (en haut) à la plus jeune (en bas).
  const bands = [
    { lbl: '60 +',    lo: 60, hi: 200 },
    { lbl: '55–59',   lo: 55, hi: 59  },
    { lbl: '50–54',   lo: 50, hi: 54  },
    { lbl: '45–49',   lo: 45, hi: 49  },
    { lbl: '40–44',   lo: 40, hi: 44  },
    { lbl: '35–39',   lo: 35, hi: 39  },
    { lbl: '30–34',   lo: 30, hi: 34  },
    { lbl: '25–29',   lo: 25, hi: 29  },
    { lbl: '< 25',    lo: 0,  hi: 24  },
  ];
  const ages = GAAB_EMPLOYES.map(e => ({ age: _gaabAge(e.naissance), sexe: e.sexe }));
  const rows = bands.map(b => ({
    lbl: b.lbl,
    h: ages.filter(a => a.sexe === 'H' && a.age >= b.lo && a.age <= b.hi).length,
    f: ages.filter(a => a.sexe === 'F' && a.age >= b.lo && a.age <= b.hi).length,
  }));
  const totH = ages.filter(a => a.sexe === 'H').length;
  const totF = ages.filter(a => a.sexe === 'F').length;
  const moyH = totH ? ages.filter(a => a.sexe === 'H').reduce((s, a) => s + a.age, 0) / totH : 0;
  const moyF = totF ? ages.filter(a => a.sexe === 'F').reduce((s, a) => s + a.age, 0) / totF : 0;
  const maxC = Math.max(1, ...rows.map(r => Math.max(r.h, r.f)));

  // Géométrie SVG : axe central, H à gauche (miroir), F à droite.
  const W = 1000, cx = 500, lblW = 96, gap = 56;
  const xMaxL = lblW + 10, xMaxR = W - 10;     // bornes des barres
  const halfL = cx - gap / 2 - xMaxL;          // largeur dispo côté H
  const halfR = xMaxR - (cx + gap / 2);        // largeur dispo côté F
  const rowH = 30, top = 24, barH = 20;
  const H = top + rows.length * rowH + 46;

  const svgRows = rows.map((r, i) => {
    const y = top + i * rowH;
    const wH = (r.h / maxC) * halfL;
    const wF = (r.f / maxC) * halfR;
    const xH = cx - gap / 2 - wH;
    const xF = cx + gap / 2;
    const barHtml =
      (r.h ? `<rect x="${xH.toFixed(1)}" y="${y}" width="${wH.toFixed(1)}" height="${barH}" class="gp-bar-h"/>
              <text x="${(xH - 6).toFixed(1)}" y="${y + barH / 2 + 4}" text-anchor="end" class="gp-cnt">${r.h}</text>` : '')
      + (r.f ? `<rect x="${xF.toFixed(1)}" y="${y}" width="${wF.toFixed(1)}" height="${barH}" class="gp-bar-f"/>
              <text x="${(xF + wF + 6).toFixed(1)}" y="${y + barH / 2 + 4}" text-anchor="start" class="gp-cnt">${r.f}</text>` : '');
    return `${barHtml}
      <text x="${cx}" y="${y + barH / 2 + 4}" text-anchor="middle" class="gp-band">${r.lbl}</text>`;
  }).join('');

  const yBase = top + rows.length * rowH + 4;
  host.innerHTML = `
  <svg viewBox="0 0 ${W} ${H}" class="gaab-pyr-svg" preserveAspectRatio="xMidYMid meet">
    <text x="${(xMaxL + (cx - gap / 2)) / 2}" y="16" text-anchor="middle" class="gp-head gp-h">Hommes (${totH})</text>
    <text x="${((cx + gap / 2) + xMaxR) / 2}" y="16" text-anchor="middle" class="gp-head gp-f">Femmes (${totF})</text>
    ${svgRows}
    <line x1="${cx}" y1="${top - 4}" x2="${cx}" y2="${yBase}" class="gp-axis"/>
    <text x="${(xMaxL + (cx - gap / 2)) / 2}" y="${yBase + 26}" text-anchor="middle" class="gp-moy">âge moyen ${moyH.toFixed(0)} ans</text>
    <text x="${((cx + gap / 2) + xMaxR) / 2}" y="${yBase + 26}" text-anchor="middle" class="gp-moy">âge moyen ${moyF.toFixed(0)} ans</text>
  </svg>`;
}

let _gaabEditMode = false;
window.gaabToggleEdit = function() {
  _gaabEditMode = !_gaabEditMode;
  const btn  = document.getElementById('gaab-edit-toggle');
  const icon = document.getElementById('gaab-lock-icon');
  btn?.classList.toggle('active', _gaabEditMode);
  if (icon) icon.textContent = _gaabEditMode ? '🔓' : '🔒';
  document.getElementById('gaab-table')?.classList.toggle('gaab-edit-mode', _gaabEditMode);
  document.querySelectorAll('#gaab-tbody td').forEach(td => {
    td.contentEditable = _gaabEditMode ? 'true' : 'false';
  });
};

let _gaabSalTimer = null;
let _gaabSalTick  = 0;

function _gaabHideSal() {
  clearInterval(_gaabSalTimer);
  _gaabSalTimer = null;
  document.getElementById('gaab-table')?.classList.remove('gaab-sal-visible');
  document.getElementById('gaab-box-section')?.classList.remove('revealed');
  const sel = document.getElementById('gaab-saltype');
  if (sel) sel.value = '';
  const cd = document.getElementById('gaab-countdown');
  if (cd) cd.textContent = '';
}

window.gaabUpdateSal = function(ticks) {
  const type = document.getElementById('gaab-saltype')?.value;
  if (!type) { _gaabHideSal(); return; }

  document.querySelectorAll('.gaab-sal').forEach(cell => {
    cell.textContent = _gaabSalStr(parseFloat(cell.dataset.bh), type);
  });

  document.getElementById('gaab-table')?.classList.add('gaab-sal-visible');

  // Boîte à moustaches — même base que la colonne révélée
  gaabRenderBox(type);
  document.getElementById('gaab-box-section')?.classList.add('revealed');

  clearInterval(_gaabSalTimer);
  _gaabSalTick = ticks ?? 10;
  const cd = document.getElementById('gaab-countdown');
  if (cd) cd.textContent = _gaabSalTick;
  _gaabSalTimer = setInterval(() => {
    _gaabSalTick--;
    if (cd) cd.textContent = _gaabSalTick > 0 ? _gaabSalTick : '';
    if (_gaabSalTick <= 0) _gaabHideSal();
  }, 1000);
};

window.gaabRevealSal = function() {
  const sel = document.getElementById('gaab-saltype');
  if (sel && !sel.value) sel.value = 'bm';
  if (_gaabSalTimer) {
    _gaabSalTick += 20;
    const cd = document.getElementById('gaab-countdown');
    if (cd) cd.textContent = _gaabSalTick;
  } else {
    gaabUpdateSal(10);
  }
};

// ── Répartition des salaires — boîte à moustaches ──────────────────────────────
const _GAAB_BASIS_LBL = { bh: 'brut horaire', bm: 'brut mensuel', nm: 'net mensuel', nh: 'net horaire' };

function _gaabSalVal(bh, type) {
  const bm = bh * 151.67;
  const nm = bm * 0.79;
  switch (type) {
    case 'bh': return bh;
    case 'nh': return nm / 151.67;
    case 'nm': return nm;
    default:   return bm;            // 'bm'
  }
}

// Quantile par interpolation linéaire (méthode dite « type 7 », celle d'Excel/R par défaut)
function _gaabQuantile(sorted, q) {
  const pos  = (sorted.length - 1) * q;
  const base = Math.floor(pos);
  const rest = pos - base;
  return sorted[base + 1] !== undefined
    ? sorted[base] + rest * (sorted[base + 1] - sorted[base])
    : sorted[base];
}

window.gaabRenderBox = function(type) {
  type = type || document.getElementById('gaab-saltype')?.value || 'bm';
  const host = document.getElementById('gaab-box-plot');
  if (!host) return;

  const vals = GAAB_EMPLOYES.map(e => _gaabSalVal(e.bh, type)).sort((a, b) => a - b);
  const n    = vals.length;
  const min  = vals[0], max = vals[n - 1];
  const q1   = _gaabQuantile(vals, 0.25);
  const med  = _gaabQuantile(vals, 0.50);
  const q3   = _gaabQuantile(vals, 0.75);
  const iqr  = q3 - q1;
  const mean = vals.reduce((s, v) => s + v, 0) / n;

  // Moustaches de Tukey : extrêmes contenus dans [Q1−1,5·IQR ; Q3+1,5·IQR]
  const loFence = q1 - 1.5 * iqr;
  const hiFence = q3 + 1.5 * iqr;
  const wLo = vals.find(v => v >= loFence);
  const wHi = [...vals].reverse().find(v => v <= hiFence);
  const outliers = vals.filter(v => v < loFence || v > hiFence);

  // Échelle horizontale avec marge
  const lo  = Math.min(min, wLo);
  const hi  = Math.max(max, wHi);
  const pad = (hi - lo) * 0.08 || 1;
  const dLo = lo - pad, dHi = hi + pad;

  const W = 1000, x0 = 70, x1 = 960;
  const sx = v => (x0 + (v - dLo) / (dHi - dLo) * (x1 - x0)).toFixed(1);
  const cy = 78, bTop = 50, bBot = 106, capTop = 60, capBot = 96;

  const dec  = (type === 'bh' || type === 'nh') ? 2 : 0;
  const fmt  = v => v.toLocaleString('fr-FR', { minimumFractionDigits: dec, maximumFractionDigits: dec });
  const unit = (type === 'bh' || type === 'nh') ? ' €/h' : ' €';

  // Points individuels (jitter déterministe, donc stable d'un rendu à l'autre)
  const dots = vals.map((v, i) => {
    const j     = (((i * 37 + 11) % 23) / 23 - 0.5) * 30;
    const isOut = v < loFence || v > hiFence;
    return `<circle cx="${sx(v)}" cy="${(cy + j).toFixed(1)}" r="3.2" class="${isOut ? 'gb-out' : 'gb-dot'}"/>`;
  }).join('');

  const lbl = (v, txt) =>
    `<text x="${sx(v)}" y="138" text-anchor="middle" class="gb-tick">${txt}</text>
     <text x="${sx(v)}" y="158" text-anchor="middle" class="gb-val">${fmt(v)}</text>`;

  host.innerHTML = `
  <svg viewBox="0 0 ${W} 200" class="gaab-box-svg" preserveAspectRatio="xMidYMid meet">
    <line x1="${sx(wLo)}" y1="${cy}" x2="${sx(q1)}" y2="${cy}" class="gb-whisk"/>
    <line x1="${sx(q3)}" y1="${cy}" x2="${sx(wHi)}" y2="${cy}" class="gb-whisk"/>
    <line x1="${sx(wLo)}" y1="${capTop}" x2="${sx(wLo)}" y2="${capBot}" class="gb-whisk"/>
    <line x1="${sx(wHi)}" y1="${capTop}" x2="${sx(wHi)}" y2="${capBot}" class="gb-whisk"/>
    <rect x="${sx(q1)}" y="${bTop}" width="${(sx(q3) - sx(q1)).toFixed(1)}" height="${bBot - bTop}" class="gb-box"/>
    <line x1="${sx(med)}"  y1="${bTop}"     x2="${sx(med)}"  y2="${bBot}"     class="gb-med"/>
    <line x1="${sx(mean)}" y1="${bTop - 6}" x2="${sx(mean)}" y2="${bBot + 6}" class="gb-mean"/>
    ${dots}
    ${lbl(wLo, 'min')}
    ${lbl(q1,  'Q1')}
    ${lbl(med, 'méd.')}
    ${lbl(q3,  'Q3')}
    ${lbl(wHi, 'max')}
  </svg>`;

  const basis = document.getElementById('gaab-box-basis');
  if (basis) basis.textContent = '// ' + (_GAAB_BASIS_LBL[type] || 'brut mensuel');

  const leg = document.getElementById('gaab-box-legend');
  if (leg) {
    const ratio = (q3 / q1).toFixed(2).replace('.', ',');
    leg.innerHTML = `
      <span><b>${n}</b> salariés</span>
      <span class="gb-l-med">— médiane <b class="gb-num">${fmt(med)}${unit}</b></span>
      <span class="gb-l-mean">┄ moyenne <b class="gb-num">${fmt(mean)}${unit}</b></span>
      <span>écart interquartile <b class="gb-num">${fmt(iqr)}${unit}</b></span>
      <span>ratio Q3/Q1 <b>${ratio}</b></span>
      ${outliers.length ? `<span class="gb-l-out">${outliers.length} atypique${outliers.length > 1 ? 's' : ''}</span>` : ''}`;
  }
};

// ── Quizz Paie ────────────────────────────────────────────────────────────────
const QUIZZ_DATA = [
  // ── France (régime général) ───────────────────────────────────────────────
  { id:  1, pays:'fr', q: "Quel est le taux global de la CSG sur les revenus d'activité ?",
    rep: "9,2 %",          mr: ["9,4 %", "9,6 %", "9,8 %"],                              src: "CSS, art. L136-8" },
  { id:  2, pays:'fr', q: "Quelle part de la CSG est déductible de l'impôt sur le revenu ?",
    rep: "6,8 %",          mr: ["8,4 %", "7,4 %", "8,2 %"],                              src: "CGI, art. 154 quinquies" },
  { id:  3, pays:'fr', q: "Quelle part de la CSG est non déductible ?",
    rep: "2,4 %",          mr: ["3,2 %", "2,8 %", "3,0 %"],                              src: "CGI, art. 154 quinquies" },
  { id:  4, pays:'fr', q: "Quel est le taux de la CRDS ?",
    rep: "0,5 %",          mr: ["0,4 %", "0,37 %", "0,45 %"],                            src: "Ordonnance n°96-50 du 24/01/1996" },
  { id:  5, pays:'fr', q: "Sur quelle base se calcule la CSG/CRDS ?",
    rep: "98,25 % du brut", mr: ["97,25 % du brut", "98,75 % du brut", "99,25 % du brut"], src: "CSS, art. L136-2" },
  { id:  6, pays:'fr', q: "Quel est le PMSS mensuel 2024 ?",
    rep: "3 864 €",        mr: ["3 666 €", "3 925 €", "3 428 €"],                        src: "Arrêté du 19/12/2023" },
  { id:  7, pays:'fr', q: "Quel est le PMSS annuel 2024 ?",
    rep: "46 368 €",       mr: ["43 992 €", "46 836 €", "47 004 €"],                     src: "Arrêté du 19/12/2023" },
  { id:  8, pays:'fr', q: "Quel est le nombre de jours de carence avant versement des IJSS maladie ?",
    rep: "3 jours",        mr: ["1 jour", "7 jours", "5 jours"],                         src: "CSS, art. R323-1" },
  { id:  9, pays:'fr', q: "Quel est le taux de base des IJSS maladie ?",
    rep: "50 %",           mr: ["60 %", "66,66 %", "45 %"],                              src: "CSS, art. R323-4" },
  { id: 10, pays:'fr', q: "Quelle est la période de référence retenue pour calculer les IJSS ?",
    rep: "3 mois",         mr: ["6 mois", "12 mois", "1 mois"],                          src: "CSS, art. R323-4" },
  { id: 11, pays:'fr', q: "Quel est le plafond des IJSS maladie ?",
    rep: "1,8 SMIC",       mr: ["1,5 SMIC", "2 SMIC", "1,6 SMIC"],                      src: "CSS, art. R323-4" },
  { id: 12, pays:'fr', q: "Quel est le taux de la cotisation vieillesse plafonnée salarié ?",
    rep: "6,90 %",         mr: ["6,70 %", "7,10 %", "6,60 %"],                          src: "CSS, art. D242-4" },
  { id: 13, pays:'fr', q: "Quel est le taux de la cotisation vieillesse déplafonnée salarié ?",
    rep: "0,40 %",         mr: ["0,30 %", "0,50 %", "0,45 %"],                          src: "CSS, art. D242-4" },
  { id: 14, pays:'fr', q: "Quel est le taux de la cotisation vieillesse plafonnée employeur ?",
    rep: "8,55 %",         mr: ["8,45 %", "8,75 %", "8,20 %"],                          src: "CSS, art. D242-4" },
  { id: 15, pays:'fr', q: "Quel est le taux normal des allocations familiales ?",
    rep: "5,25 %",         mr: ["5,40 %", "4,90 %", "5,10 %"],                          src: "CSS, art. L241-6" },
  { id: 16, pays:'fr', q: "En dessous de quel seuil (en SMIC) s'applique le taux réduit des allocations familiales ?",
    rep: "3,5 SMIC",       mr: ["3 SMIC", "2,5 SMIC", "4 SMIC"],                        src: "CSS, art. L241-6-1" },
  { id: 17, pays:'fr', q: "Quel est le taux réduit des allocations familiales ?",
    rep: "3,45 %",         mr: ["3,25 %", "3,75 %", "3,60 %"],                          src: "CSS, art. D241-3-2" },
  { id: 18, pays:'fr', q: "Quel est le taux de la contribution solidarité autonomie (CSA) ?",
    rep: "0,30 %",         mr: ["0,10 %", "0,50 %", "0,25 %"],                          src: "CASF, art. L14-10-4" },
  { id: 19, pays:'fr', q: "Quel est le taux du FNAL pour les entreprises de moins de 50 salariés ?",
    rep: "0,10 %",         mr: ["0,30 %", "0,20 %", "0,50 %"],                          src: "CSS, art. L834-1" },
  { id: 20, pays:'fr', q: "Quel est le taux du FNAL pour les entreprises d'au moins 50 salariés ?",
    rep: "0,50 %",         mr: ["0,30 %", "0,10 %", "0,40 %"],                          src: "CSS, art. L834-1" },
  { id: 21, pays:'fr', q: "Comment est déterminé le taux AT/MP ?",
    rep: "Variable",       mr: ["Fixé à 0,70 % pour tous", "Forfait de 2 % du brut", "Fixé légalement à 1 %"], src: "CSS, art. L242-5" },
  { id: 22, pays:'fr', q: "Quel est le SMIC mensuel brut 2024 (base 35h) ?",
    rep: "1 766,92 €",     mr: ["1 709,28 €", "1 801,80 €", "1 747,20 €"],              src: "Décret n°2023-1216" },
  { id: 23, pays:'fr', q: "Quelle est la durée mensuelle de travail pour 35h hebdomadaires ?",
    rep: "151,67 h",       mr: ["152,25 h", "150,50 h", "153,33 h"],                    src: "Code du travail, art. L3121-27" },
  { id: 24, pays:'fr', q: "Quel est le taux de majoration pour les 8 premières heures supplémentaires ?",
    rep: "25 %",           mr: ["10 %", "20 %", "30 %"],                                src: "Code du travail, art. L3121-36" },
  { id: 25, pays:'fr', q: "Quel est le taux de majoration pour les heures supplémentaires au-delà des 8 premières ?",
    rep: "50 %",           mr: ["25 %", "40 %", "75 %"],                                src: "Code du travail, art. L3121-36" },
  { id: 26, pays:'fr', q: "Quel est le plafond annuel d'exonération fiscale et sociale sur les heures supplémentaires ?",
    rep: "7 500 €",        mr: ["5 000 €", "7 000 €", "8 000 €"],                       src: "CGI, art. 81 quater" },
  { id: 27, pays:'fr', q: "La réduction générale de cotisations patronales est-elle fixe ou variable ?",
    rep: "Variable",       mr: ["Fixée à 16 % du brut", "Plafonnée à 26 % pour tous", "Identique quel que soit l'effectif"], src: "CSS, art. L241-13" },
  { id: 28, pays:'fr', q: "Quel est le taux maximum de la réduction Fillon ?",
    rep: "~32 %",          mr: ["~26 %", "~28 %", "~35 %"],                             src: "CSS, art. D241-7" },
  { id: 29, pays:'fr', q: "Quel est le taux de cotisation chômage à la charge du salarié ?",
    rep: "0 %",            mr: ["2,40 %", "0,95 %", "1,20 %"],                          src: "Loi n°2018-771" },
  { id: 30, pays:'fr', q: "Quel est le taux de cotisation chômage à la charge de l'employeur ?",
    rep: "4,05 %",         mr: ["3,45 %", "4,40 %", "3,90 %"],                          src: "Convention Unédic" },

  // ── Suisse ────────────────────────────────────────────────────────────────
  { id: 31, pays:'ch', q: "Quel est le taux AVS salarié en Suisse ?",
    rep: "4,35 %",         mr: ["4,50 %", "5,15 %", "4,20 %"],                          src: "LAVS, RS 831.10" },
  { id: 32, pays:'ch', q: "Quel est le taux AVS total (salarié + employeur) en Suisse ?",
    rep: "8,70 %",         mr: ["8,40 %", "9,20 %", "8,50 %"],                          src: "LAVS, RS 831.10" },
  { id: 33, pays:'ch', q: "Quel est le taux AI (assurance invalidité) salarié en Suisse ?",
    rep: "0,70 %",         mr: ["0,50 %", "0,80 %", "0,75 %"],                          src: "LAI, RS 831.20" },
  { id: 34, pays:'ch', q: "Quel est le taux APG (allocations perte de gain) salarié en Suisse ?",
    rep: "0,25 %",         mr: ["0,20 %", "0,30 %", "0,35 %"],                          src: "LAPG, RS 834.1" },
  { id: 35, pays:'ch', q: "Quel est le taux total des cotisations du 1er pilier (AVS+AI+APG) côté salarié ?",
    rep: "5,30 %",         mr: ["5,05 %", "5,50 %", "4,95 %"],                          src: "LAVS / LAI / LAPG" },
  { id: 36, pays:'ch', q: "Quel est le taux AC (assurance-chômage) salarié en Suisse ?",
    rep: "1,10 %",         mr: ["1,20 %", "1,00 %", "1,25 %"],                          src: "LACI, RS 837.0" },
  { id: 37, pays:'ch', q: "Quel est le plafond annuel soumis à cotisation AC en Suisse ?",
    rep: "148 200 CHF",    mr: ["126 000 CHF", "139 200 CHF", "156 000 CHF"],            src: "LACI, RS 837.0" },
  { id: 38, pays:'ch', q: "Quel est le plafond mensuel AC (148 200 CHF / 12) ?",
    rep: "12 350 CHF",     mr: ["10 500 CHF", "11 600 CHF", "13 000 CHF"],               src: "LACI, RS 837.0" },
  { id: 39, pays:'ch', q: "L'AANP (accidents non professionnels) est-elle à la charge de qui ?",
    rep: "Salarié uniquement", mr: ["Employeur uniquement", "Partagée 50/50", "État suisse"], src: "LAA, RS 832.20" },
  { id: 40, pays:'ch', q: "L'AAP (accidents professionnels) est-elle à la charge de qui ?",
    rep: "Employeur uniquement", mr: ["Salarié uniquement", "Partagée 50/50", "Caisse nationale"], src: "LAA, RS 832.20" },
  { id: 41, pays:'ch', q: "Quel est le taux LPP minimum légal pour la tranche d'âge 35-44 ans (total sal+pat) ?",
    rep: "10 %",           mr: ["7 %", "12 %", "8 %"],                                  src: "LPP, RS 831.40, art. 16" },
  { id: 42, pays:'ch', q: "Quel est le taux LPP salarié (35-44 ans) ?",
    rep: "5 %",            mr: ["3,5 %", "6 %", "7,5 %"],                               src: "LPP, RS 831.40, art. 16" },
  { id: 43, pays:'ch', q: "Quel est le montant de la déduction de coordination LPP (2025) ?",
    rep: "25 725 CHF",     mr: ["22 050 CHF", "28 080 CHF", "24 885 CHF"],               src: "OPP 2, art. 8" },
  { id: 44, pays:'ch', q: "Quel est le seuil d'entrée LPP (salaire annuel minimum pour être affilié, 2025) ?",
    rep: "22 050 CHF",     mr: ["18 000 CHF", "25 725 CHF", "20 000 CHF"],               src: "LPP, art. 2 al. 1" },
  { id: 45, pays:'ch', q: "Sur quelle base est calculée la cotisation LPP ?",
    rep: "Salaire coordonné (brut − déduction coordination)", mr: ["Salaire brut total", "Salaire net", "Salaire limité au PMSS"], src: "LPP, art. 7-8" },
  { id: 46, pays:'ch', q: "Quel organisme collecte les cotisations AVS/AI/APG en Suisse ?",
    rep: "Caisse de compensation (OFAS)", mr: ["SUVA", "SECO", "CCSS"],                  src: "LAVS, art. 53 ss" },
  { id: 47, pays:'ch', q: "Quel organisme gère l'assurance-accidents obligatoire (SUVA) en Suisse ?",
    rep: "SUVA (Caisse nationale d'assurance accidents)", mr: ["OFAS", "SECO", "CNA France"], src: "LAA, art. 61" },
  { id: 48, pays:'ch', q: "Combien de piliers compte le système de prévoyance suisse ?",
    rep: "3",              mr: ["2", "4", "5"],                                          src: "Constitution fédérale, art. 111" },
  { id: 49, pays:'ch', q: "Le 1er pilier suisse (AVS) est-il plafonné ?",
    rep: "Non, pas de plafond de cotisation", mr: ["Oui, plafonné à 148 200 CHF/an", "Oui, au PMSS français", "Oui, à 88 200 CHF"], src: "LAVS, art. 5" },
  { id: 50, pays:'ch', q: "Loi fédérale qui régit l'assurance-chômage en Suisse ?",
    rep: "LACI (RS 837.0)", mr: ["LAA (RS 832.20)", "LAVS (RS 831.10)", "LPP (RS 831.40)"], src: "LACI, RS 837.0" },
  { id: 51, pays:'ch', q: "Le taux LPP légal augmente-t-il avec l'âge ?",
    rep: "Oui (7/10/15/18 % selon les tranches)", mr: ["Non, fixe à 10 % pour tous", "Non, fixe à 8 %", "Oui mais uniquement la part patronale"], src: "LPP, art. 16" },
  { id: 52, pays:'ch', q: "Taux total 1er pilier (AVS+AI+APG) toutes parts (sal+pat) confondues ?",
    rep: "10,60 %",        mr: ["10,20 %", "11,00 %", "9,80 %"],                        src: "LAVS / LAI / LAPG" },
  { id: 53, pays:'ch', q: "Montant minimum légal fédéral des allocations familiales pour enfant jusqu'à 16 ans ?",
    rep: "200 CHF/mois",   mr: ["250 CHF/mois", "300 CHF/mois", "160 CHF/mois"],         src: "LAFam, RS 836.2, art. 5" },
  { id: 54, pays:'ch', q: "Les allocations familiales sont-elles uniformes dans tous les cantons ?",
    rep: "Non, elles varient par canton (minimum fédéral 200 CHF)", mr: ["Oui, uniformes à 250 CHF partout", "Oui, uniformes à 300 CHF", "Non, elles sont purement privées"], src: "LAFam, art. 3 al. 2" },
  { id: 55, pays:'ch', q: "Taux total côté employeur : AVS+AI+APG+AC (jusqu'au plafond) ?",
    rep: "6,40 %",         mr: ["5,75 %", "7,10 %", "6,00 %"],                          src: "LAVS / LAI / LAPG / LACI" },
  { id: 56, pays:'ch', q: "Quelle réforme a harmonisé l'âge de la retraite femmes à 65 ans en Suisse ?",
    rep: "AVS 21",         mr: ["LPP 22", "Réforme Dini", "AVS 2004"],                   src: "Réforme AVS 21, en vigueur 01/01/2024" },
  { id: 57, pays:'ch', q: "L'impôt à la source (IS) en Suisse concerne principalement qui ?",
    rep: "Résidents étrangers sans permis C", mr: ["Tous les salariés", "Frontaliers uniquement", "Cadres supérieurs uniquement"], src: "LIFD, art. 83-98" },
  { id: 58, pays:'ch', q: "Le taux IJM (indemnités journalières maladie) salarié indicatif est de ?",
    rep: "0,75 %",         mr: ["1,50 %", "0,50 %", "1,00 %"],                          src: "LCA / LAMAL (plan collectif type)" },
  { id: 59, pays:'ch', q: "Le plafond LAA (accidents) annuel est-il le même que celui de l'AC ?",
    rep: "Oui, 148 200 CHF/an", mr: ["Non, 126 900 CHF pour la LAA", "Non, 168 000 CHF pour la LAA", "Non, la LAA n'a pas de plafond"], src: "LAA, art. 15 ; LACI, art. 23" },
  { id: 60, pays:'ch', q: "Quel sigle désigne la prévoyance vieillesse du 1er pilier suisse ?",
    rep: "AVS (en allemand : AHV)", mr: ["LPP / BVG", "LAA / UVG", "LACI / AVIG"],      src: "LAVS, RS 831.10" },

  // ── Luxembourg ────────────────────────────────────────────────────────────
  { id: 61, pays:'lu', q: "Quel est le taux d'assurance pension (AP) salarié au Luxembourg ?",
    rep: "8 %",            mr: ["10 %", "6 %", "8,5 %"],                                src: "CSS LU, Livre II (CNAP)" },
  { id: 62, pays:'lu', q: "Quel est le taux total assurance pension (AP) au Luxembourg ?",
    rep: "16 %",           mr: ["20 %", "12 %", "14 %"],                                src: "CSS LU, Livre II (CNAP)" },
  { id: 63, pays:'lu', q: "Quel est le taux assurance maladie-maternité (AM) salarié au Luxembourg ?",
    rep: "3,05 %",         mr: ["2,80 %", "3,50 %", "2,50 %"],                          src: "CSS LU, art. 10 (CNS)" },
  { id: 64, pays:'lu', q: "De quoi se compose le taux AM 3,05 % au Luxembourg ?",
    rep: "Soins 2,80 % + indemnités pécuniaires 0,25 %", mr: ["Soins 2,50 % + ind. 0,55 %", "Soins 3,00 % + ind. 0,05 %", "Soins 2,80 % + ind. 0,30 %"], src: "CSS LU, art. 10 (CNS)" },
  { id: 65, pays:'lu', q: "Quel est le taux de l'assurance dépendance (AD) salarié au Luxembourg ?",
    rep: "1,40 %",         mr: ["0,70 % partagé", "1,40 % partagé", "2,80 % salarié"],  src: "Loi du 19/06/1998 (assurance dépendance)" },
  { id: 66, pays:'lu', q: "L'assurance dépendance (AD) est-elle partagée entre salarié et employeur ?",
    rep: "Non, salarié uniquement", mr: ["Oui, 50/50", "Oui, 70/30 en faveur du salarié", "Non, patronal uniquement"], src: "Loi du 19/06/1998" },
  { id: 67, pays:'lu', q: "Quel est le plafond de cotisations sociales au Luxembourg ?",
    rep: "5 × SSM (~13 518 €/mois)", mr: ["3 × SSM", "4 × PMSS français", "148 200 € / an"], src: "CSS LU, Livre II" },
  { id: 68, pays:'lu', q: "Quel organisme collecte toutes les cotisations sociales au Luxembourg ?",
    rep: "CCSS (Centre commun de la sécurité sociale)", mr: ["CNAP", "CNS", "URSSAF"],  src: "CSS LU" },
  { id: 69, pays:'lu', q: "Quel est le taux indicatif de l'assurance accidents (AA) employeur au Luxembourg ?",
    rep: "0,75 % patronal uniquement", mr: ["1,00 % partagé", "0,50 % salarié", "1,40 % patronal"], src: "CSS LU, Livre III (AAA)" },
  { id: 70, pays:'lu', q: "Qu'est-ce que la mutualité des employeurs (ME) au Luxembourg ?",
    rep: "Rembourse à l'employeur les salaires maintenus pendant la maladie", mr: ["Assurance pension complémentaire", "Cotisation chômage", "Assurance invalidité"], src: "CSS LU, Livre II" },
  { id: 71, pays:'lu', q: "Pendant combien de jours la mutualité des employeurs (ME) rembourse-t-elle les salaires maintenus ?",
    rep: "77 jours (jours 1-77)", mr: ["30 jours", "52 jours", "90 jours"],              src: "CSS LU, art. ME" },
  { id: 72, pays:'lu', q: "Quel est le taux total cotisations salarié LU (AP+AM+AD) ?",
    rep: "12,45 %",        mr: ["11,00 %", "14,00 %", "10,50 %"],                       src: "CSS LU (8+3,05+1,40)" },
  { id: 73, pays:'lu', q: "Quel est le taux total assurance maladie LU toutes parts confondues ?",
    rep: "6,10 %",         mr: ["5,60 %", "7,00 %", "6,00 %"],                          src: "CSS LU (3,05×2)" },
  { id: 74, pays:'lu', q: "Quelle institution gère la pension au Luxembourg ?",
    rep: "CNAP (Caisse nationale d'assurance pension)", mr: ["CCSS", "CNS", "URSSAF"],  src: "CSS LU, Livre II" },
  { id: 75, pays:'lu', q: "Quelle institution gère la santé au Luxembourg ?",
    rep: "CNS (Caisse nationale de santé)", mr: ["CNAP", "CCSS", "AAA"],                src: "CSS LU, art. 10" },
  { id: 76, pays:'lu', q: "Quelle institution gère les accidents du travail au Luxembourg ?",
    rep: "AAA (Association d'assurance accident)", mr: ["CNAP", "CNS", "CCSS"],         src: "CSS LU, Livre III" },
  { id: 77, pays:'lu', q: "Y a-t-il une cotisation chômage salarié au Luxembourg ?",
    rep: "Non, 0 % salarié", mr: ["Oui, 0,50 %", "Oui, 1,20 %", "Oui, 2,40 %"],        src: "CSS LU" },
  { id: 78, pays:'lu', q: "Quel est le salaire social minimum (SSM) non-qualifié estimé 2026 au Luxembourg ?",
    rep: "~2 703 €/mois",  mr: ["~2 256 €/mois", "~3 100 €/mois", "~2 143 €/mois"],    src: "CCSS LU, indexation Jan 2025" },
  { id: 79, pays:'lu', q: "Comment s'appelle le salaire minimum au Luxembourg ?",
    rep: "SSM (Salaire Social Minimum)", mr: ["SMIC", "SMG", "SMN"],                    src: "Loi du 12/03/1969 LU" },
  { id: 80, pays:'lu', q: "L'assurance dépendance (AD) est-elle plafonnée ?",
    rep: "Oui, à 5 × SSM comme les autres cotisations", mr: ["Non, sans plafond", "Oui, à 10 × SSM", "Non, seulement au PMSS"], src: "CSS LU" },
  { id: 81, pays:'lu', q: "Depuis quand existe l'assurance dépendance au Luxembourg ?",
    rep: "1999 (loi du 19/06/1998)", mr: ["1985", "2010", "2005"],                      src: "Loi du 19/06/1998 LU" },
  { id: 82, pays:'lu', q: "L'assurance accidents (AA) est-elle à la charge exclusive de l'employeur au Luxembourg ?",
    rep: "Oui",            mr: ["Non, partagée", "Non, salarié uniquement", "Non, financée par l'État"], src: "CSS LU, Livre III" },
  { id: 83, pays:'lu', q: "La mutualité des employeurs (ME) est-elle obligatoire pour tous les employeurs LU ?",
    rep: "Oui, obligatoire pour tous", mr: ["Non, optionnelle", "Non, >50 salariés seulement", "Non, réservée au secteur privé"], src: "CSS LU" },
  { id: 84, pays:'lu', q: "Taux ME (mutualité employeurs) indicatif moyen national ?",
    rep: "1,40 % patronal uniquement", mr: ["0,70 % partagé", "2,80 % patronal", "1,00 % salarié"], src: "CCSS LU" },
  { id: 85, pays:'lu', q: "Taux total pension LU en comparaison avec la France ?",
    rep: "Supérieur (16 % LU vs ~15,45 % FR vieillesse)", mr: ["Inférieur (12 %)", "Identique (15,45 %)", "Très inférieur (8 %)"], src: "CSS LU / CSS FR" },
  { id: 86, pays:'lu', q: "Quel sigle désigne l'organisme qui collecte TOUTES les cotisations LU ?",
    rep: "CCSS",           mr: ["CNAP", "CNS", "AAA"],                                  src: "CSS LU" },
  { id: 87, pays:'lu', q: "L'assurance pension LU est-elle symétrique (même taux sal et pat) ?",
    rep: "Oui, 8 % chacun", mr: ["Non, patronal supérieur", "Non, salarié supérieur", "Variable selon ancienneté"], src: "CSS LU, Livre II" },
  { id: 88, pays:'lu', q: "Le plafond cotisable LU (5 × SSM) est-il mensuel ou annuel ?",
    rep: "Mensuel (~13 518 €/mois)", mr: ["Annuel (148 200 €/an)", "Hebdomadaire", "Trimestriel"], src: "CSS LU" },
  { id: 89, pays:'lu', q: "Le Luxembourg finance-t-il le chômage via une cotisation explicite sur le bulletin ?",
    rep: "Non, le fonds de l'emploi est alimenté différemment (budget État/entreprises)", mr: ["Oui, 1,00 % salarié", "Oui, 2,40 % patronal", "Oui, 0,50 % partagé"], src: "CSS LU" },
  { id: 90, pays:'lu', q: "Taux patronal total indicatif LU (AP+AM+AA+ME) ?",
    rep: "~13,25 %",       mr: ["~10,00 %", "~16,00 %", "~20,00 %"],                    src: "CSS LU (8+3,05+0,75+1,40)" },

  // ── Italie ────────────────────────────────────────────────────────────────
  { id: 91, pays:'it', q: "Quel est le taux IVS salarié en Italie ?",
    rep: "9,19 %",         mr: ["8,84 %", "9,75 %", "10,00 %"],                         src: "INPS, Circ. — L. 335/1995" },
  { id: 92, pays:'it', q: "Quel est le taux IVS employeur en Italie ?",
    rep: "23,81 %",        mr: ["21,50 %", "25,00 %", "20,00 %"],                       src: "INPS, Circ. — L. 335/1995" },
  { id: 93, pays:'it', q: "Quel est le taux total IVS (sal+pat) en Italie ?",
    rep: "33 %",           mr: ["30 %", "35 %", "32 %"],                                src: "INPS — L. 335/1995" },
  { id: 94, pays:'it', q: "Quel est le taux NASpI (chômage) salarié en Italie ?",
    rep: "0 %",            mr: ["0,50 %", "1,00 %", "2,40 %"],                          src: "L. 228/2012 ; D.Lgs. 22/2015" },
  { id: 95, pays:'it', q: "Quel est le taux NASpI employeur en Italie ?",
    rep: "1,61 %",         mr: ["2,22 %", "1,40 %", "2,44 %"],                          src: "D.Lgs. 22/2015" },
  { id: 96, pays:'it', q: "Quelle majoration s'applique au NASpI pour les CDD en Italie ?",
    rep: "+1,40 % patronal", mr: ["+0,80 %", "+2,00 %", "+0,50 %"],                     src: "L. 92/2012, art. 2 c. 28-29" },
  { id: 97, pays:'it', q: "Quel est le taux d'accrual mensuel du TFR en Italie ?",
    rep: "6,91 %",         mr: ["7,50 %", "6,67 %", "8,00 %"],                          src: "L. 297/1982" },
  { id: 98, pays:'it', q: "Quelle est la formule de calcul du TFR mensuel ?",
    rep: "Salaire brut / 13,5", mr: ["Salaire brut / 12", "Salaire brut × 6,91 %", "Salaire brut / 15"], src: "L. 297/1982 (6,91 % ≈ 1/13,5)" },
  { id: 99, pays:'it', q: "Quel est le taux INAIL indicatif pour un poste de bureau en Italie ?",
    rep: "0,65 % patronal", mr: ["0,30 %", "1,50 %", "2,00 %"],                         src: "INAIL — voce tariffa terziario" },
  { id:100, pays:'it', q: "L'INAIL (accidents du travail) est-elle à la charge de qui en Italie ?",
    rep: "Employeur uniquement", mr: ["Salarié uniquement", "Partagée 50/50", "État"],   src: "INAIL — D.P.R. 1124/1965" },
  { id:101, pays:'it', q: "Quel organisme collecte les cotisations sociales (hors accidents) en Italie ?",
    rep: "INPS",           mr: ["INAIL", "Agenzia Entrate", "CCSS"],                     src: "D.Lgs. 509/1994" },
  { id:102, pays:'it', q: "Quel organisme gère les accidents du travail en Italie ?",
    rep: "INAIL",          mr: ["INPS", "Agenzia Entrate", "ISTAT"],                     src: "D.P.R. 1124/1965" },
  { id:103, pays:'it', q: "Qu'est-ce que la 'carenza' en Italie ?",
    rep: "Les 3 premiers jours de maladie non indemnisés par l'INPS", mr: ["Le délai de carence AT", "La période d'essai", "Une retenue disciplinaire"], src: "D.P.R. 663/1979 ; CCNL" },
  { id:104, pays:'it', q: "Durée légale du congé maternité en Italie ?",
    rep: "5 mois",         mr: ["3 mois", "6 mois", "4 mois"],                           src: "D.Lgs. 151/2001, art. 16-17" },
  { id:105, pays:'it', q: "Durée minimale du congé paternité obligatoire en Italie (depuis L. 160/2019) ?",
    rep: "10 jours",       mr: ["3 jours", "5 jours", "14 jours"],                       src: "L. 160/2019 ; L. 234/2021" },
  { id:106, pays:'it', q: "Taux d'indemnisation des IJ maternité INPS en Italie ?",
    rep: "80 %",           mr: ["50 %", "66,66 %", "100 %"],                             src: "D.Lgs. 151/2001, art. 22" },
  { id:107, pays:'it', q: "Taux cotisation maternité/paternité (maternità) employeur en Italie ?",
    rep: "0,46 % patronal", mr: ["0,25 % partagé", "0,86 % patronal", "0,20 % salarié"], src: "INPS — stable" },
  { id:108, pays:'it', q: "Taux fondo di garanzia TFR en Italie ?",
    rep: "0,20 % patronal", mr: ["0,50 %", "0,10 %", "0,30 %"],                         src: "L. 297/1982, art. 2 — code F24 GFFT" },
  { id:109, pays:'it', q: "Pour quelles entreprises le TFR doit-il être versé au Fondo Tesoreria INPS ?",
    rep: "Entreprises > 50 salariés", mr: ["> 10 salariés", "> 100 salariés", "Toutes les entreprises"], src: "L. 296/2006, art. 1 c. 755" },
  { id:110, pays:'it', q: "Qu'est-ce que l'auto-liquidazione INAIL en Italie ?",
    rep: "Le versement annuel de la prime INAIL au 16 février", mr: ["Le remboursement auto des IJSS", "La liquidation du TFR", "La régularisation fiscale annuelle IR"], src: "D.P.R. 1124/1965" },
  { id:111, pays:'it', q: "Taux malattia (IJ maladie) employeur indicatif en Italie ?",
    rep: "2,22 % patronal", mr: ["0,46 % partagé", "1,61 % salarié", "2,50 % patronal"], src: "INPS — secteur commercio/industria" },
  { id:112, pays:'it', q: "Quelle loi a réformé le système de retraite IT en introduisant le calcul par capitalisation ?",
    rep: "L. 335/1995 (réforme Dini)", mr: ["L. 297/1982", "D.Lgs. 22/2015", "L. 92/2012"], src: "L. 335/1995" },
  { id:113, pays:'it', q: "Le massimale contributivo IVS s'applique-t-il à tous les salariés ?",
    rep: "Non, uniquement aux salariés sans ancienneté INPS au 31/12/1995", mr: ["Oui, à tous", "Non, uniquement aux cadres", "Oui, depuis 2015"], src: "L. 335/1995" },
  { id:114, pays:'it', q: "Depuis quelle loi la cotisation chômage salarié est-elle à 0 % en Italie ?",
    rep: "L. 228/2012",    mr: ["L. 335/1995", "D.Lgs. 22/2015", "L. 92/2012"],         src: "L. 228/2012 (en vigueur 01/01/2013)" },
  { id:115, pays:'it', q: "Qu'est-ce que le 'taglio del cuneo fiscale' en Italie ?",
    rep: "Une réduction de la cotisation IVS salarié sur les bas salaires", mr: ["Une réduction IS patronale", "Une prime nette exonérée", "Un abattement fiscal pur IR"], src: "DL 115/2022 ; L. 197/2022" },
  { id:116, pays:'it', q: "Taux indicatif total des charges patronales IT (IVS+NASpI+malattia+maternità+fondo+INAIL) ?",
    rep: "~28,95 %",       mr: ["~22,00 %", "~35,00 %", "~32,00 %"],                    src: "INPS / INAIL (23,81+1,61+2,22+0,46+0,20+0,65)" },
  { id:117, pays:'it', q: "La NASpI remplace quelle prestation créée en 2012 en Italie ?",
    rep: "L'ASpI (assurance sociale pour l'emploi, L. 92/2012)", mr: ["La CIG (cassa integrazione)", "L'indennità di mobilità", "Le TFR différé"], src: "D.Lgs. 22/2015" },
  { id:118, pays:'it', q: "Les cotisations IVS ont-elles changé depuis la réforme Dini de 1995 ?",
    rep: "Non, le total 33 % est stable depuis les années 1990", mr: ["Oui, hausse à 36 % en 2015", "Oui, baisse à 30 % en 2012", "Oui, suppression part salarié en 2013"], src: "L. 335/1995" },
  { id:119, pays:'it', q: "Quelle est la base de calcul du TFR annuel selon la loi 297/1982 ?",
    rep: "Rémunération annuelle / 13,5", mr: ["Rémunération annuelle / 12", "Brut mensuel × 13", "Brut mensuel × 6,91 %"], src: "L. 297/1982, art. 2" },
  { id:120, pays:'it', q: "Le versement du TFR en cas de licenciement est-il soumis à la tassazione separata ?",
    rep: "Oui", mr: ["Non, imposé comme revenu ordinaire", "Non, totalement exonéré", "Oui, mais uniquement pour les CDI"], src: "TUIR, art. 17 c. 1 lett. a)" },

  // ── Canada (hors Québec — province de référence : Ontario) ───────────────
  { id:121, pays:'ca', q: "Quel est le taux RPC salarié en 2024 ?",
    rep: "5,95 %",         mr: ["4,95 %", "6,40 %", "5,70 %"],                          src: "L.C. 2013 ch. 33 ; ARC T4001" },
  { id:122, pays:'ca', q: "Le taux RPC employeur est-il identique au taux salarié ?",
    rep: "Oui, 5,95 % chacun", mr: ["Non, employeur paie plus", "Non, 4,95 % patronal", "Non, 7 % patronal"], src: "L.C. 2013 ch. 33" },
  { id:123, pays:'ca', q: "Quelle est l'exonération de base du RPC ?",
    rep: "3 500 CAD/an",   mr: ["5 000 CAD", "2 500 CAD", "1 000 CAD"],                 src: "ARC T4001 2024" },
  { id:124, pays:'ca', q: "Quel est le maximum des gains annuels (MGA) pour le RPC en 2024 ?",
    rep: "68 500 CAD",     mr: ["63 200 CAD", "73 200 CAD", "56 300 CAD"],               src: "ARC T4001 2024" },
  { id:125, pays:'ca', q: "Quel est le taux AE salarié général (hors Québec) en 2024 ?",
    rep: "1,66 %",         mr: ["1,31 %", "1,80 %", "2,40 %"],                          src: "ARC T4001 2024" },
  { id:126, pays:'ca', q: "Quel est le rapport entre le taux AE patronal et salarié ?",
    rep: "Employeur = salarié × 1,4", mr: ["× 1,2", "× 1,5", "× 2"],                   src: "LAE, art. 68" },
  { id:127, pays:'ca', q: "Quel est le taux AE employeur (hors Québec) en 2024 ?",
    rep: "2,324 %",        mr: ["2,00 %", "2,60 %", "1,834 %"],                         src: "ARC T4001 2024 (1,66×1,4)" },
  { id:128, pays:'ca', q: "Quel est le maximum de la rémunération assurable (MAGA) pour l'AE en 2024 ?",
    rep: "63 200 CAD",     mr: ["68 500 CAD", "94 000 CAD", "56 300 CAD"],               src: "ARC T4001 2024" },
  { id:129, pays:'ca', q: "Quel est le taux RPC2 (bonification phase 2) salarié dès 2024 ?",
    rep: "4 %",            mr: ["5,95 %", "2 %", "8 %"],                                src: "L.C. 2018 ch. 12" },
  { id:130, pays:'ca', q: "Sur quelle assiette se calcule le RPC2 ?",
    rep: "Gains entre le MGA et le MGAP2", mr: ["Totalité du salaire brut", "Gains au-delà du MGAP2", "Gains jusqu'au MGA"], src: "L.C. 2018 ch. 12" },
  { id:131, pays:'ca', q: "Quel est le MGAP2 (2ème maximum des gains) en 2024 ?",
    rep: "73 200 CAD",     mr: ["68 500 CAD", "80 000 CAD", "94 000 CAD"],               src: "ARC T4001 2024" },
  { id:132, pays:'ca', q: "Quel est le 1er taux du barème fédéral canadien ?",
    rep: "15 %",           mr: ["18 %", "20,5 %", "12 %"],                              src: "LIR fédérale 2024" },
  { id:133, pays:'ca', q: "Quel est le taux marginal fédéral maximum au Canada ?",
    rep: "33 %",           mr: ["29 %", "39,6 %", "43 %"],                              src: "LIR fédérale (art. 117)" },
  { id:134, pays:'ca', q: "Quel est le montant personnel de base (MPB) fédéral 2024 ?",
    rep: "15 705 CAD",     mr: ["12 000 CAD", "15 000 CAD", "13 521 CAD"],              src: "ARC T4001 2024" },
  { id:135, pays:'ca', q: "Quel formulaire l'employé remplit-il pour ses crédits fédéraux ?",
    rep: "TD1",            mr: ["TP-1015.3", "T4001", "T2200"],                          src: "ARC — formulaire TD1" },
  { id:136, pays:'ca', q: "Quelle province a son propre régime de pensions (hors RPC) ?",
    rep: "Québec (RRQ)",   mr: ["Ontario", "Alberta", "Colombie-Britannique"],           src: "Loi sur le RRQ (Québec)" },
  { id:137, pays:'ca', q: "Quelle est la province de référence dans le simulateur pour Canada hors Québec ?",
    rep: "Ontario",        mr: ["Colombie-Britannique", "Alberta", "Saskatchewan"],      src: "Xenna — seed CA" },
  { id:138, pays:'ca', q: "Quel est le 1er taux du barème provincial ontarien ?",
    rep: "5,05 %",         mr: ["5,50 %", "4,50 %", "6,00 %"],                          src: "Loi de l'impôt sur le revenu (Ontario) 2024" },
  { id:139, pays:'ca', q: "Quel est le taux marginal provincial maximum en Ontario ?",
    rep: "13,16 %",        mr: ["12,00 %", "15,00 %", "11,16 %"],                       src: "Loi de l'impôt (Ontario) 2024" },
  { id:140, pays:'ca', q: "Quel est le montant personnel de base provincial ontarien 2024 ?",
    rep: "11 865 CAD",     mr: ["15 705 CAD", "10 000 CAD", "13 000 CAD"],              src: "ARC — TD1ON 2024" },
  { id:141, pays:'ca', q: "L'exonération de base (3 500 CAD) s'applique-t-elle au RPC2 ?",
    rep: "Non",            mr: ["Oui, identique", "Oui, doublée", "Oui, réduite à 1 750 CAD"], src: "L.C. 2018 ch. 12" },
  { id:142, pays:'ca', q: "Quel formulaire identifie les crédits provinciaux en Ontario ?",
    rep: "TD1ON",          mr: ["TP-1015.3", "T4001", "TD1"],                            src: "ARC — formulaire TD1ON" },
  { id:143, pays:'ca', q: "Comment s'appelle le relevé annuel de paie au Canada ?",
    rep: "Feuillet T4",    mr: ["TP-1 (Québec)", "W-2 (USA)", "Feuillet A"],             src: "ARC — T4" },
  { id:144, pays:'ca', q: "Les cotisations RPC sont-elles déductibles d'impôt ?",
    rep: "Oui, crédit d'impôt non remboursable", mr: ["Non", "Oui, déduction du revenu brut", "Oui, remboursement direct"], src: "LIR fédérale, art. 118.7" },
  { id:145, pays:'ca', q: "Quel était le taux RPC avant la bonification (avant 2019) ?",
    rep: "4,95 %",         mr: ["5,25 %", "5,70 %", "4,80 %"],                          src: "RPC — taux historique" },
  { id:146, pays:'ca', q: "Le RPC2 comporte-t-il une exonération de base ?",
    rep: "Non",            mr: ["Oui, 3 500 CAD comme le RPC", "Oui, 1 750 CAD", "Oui, le même MGAP2"], src: "L.C. 2018 ch. 12" },
  { id:147, pays:'ca', q: "Depuis quelle année la bonification du RPC (phase 1) est-elle en cours ?",
    rep: "2019",           mr: ["2015", "2021", "2017"],                                 src: "L.C. 2018 ch. 12" },
  { id:148, pays:'ca', q: "L'AE au Canada est-elle de compétence fédérale ou provinciale ?",
    rep: "Fédérale",       mr: ["Chaque province gère son AE", "Mixte fédéral/provincial", "Provinciale depuis 2019"], src: "LAE (L.C. 1996 ch. 23)" },
  { id:149, pays:'ca', q: "L'AE comporte-t-elle une exonération de base comme le RPC ?",
    rep: "Non, 0 $ d'exonération", mr: ["Oui, 3 500 CAD", "Oui, 500 CAD", "Oui, identique au RPC"], src: "LAE (L.C. 1996 ch. 23)" },
  { id:150, pays:'ca', q: "Taux fédéral entre 111 733 CAD et 154 906 CAD (3ème tranche) ?",
    rep: "26 %",           mr: ["29 %", "20,5 %", "33 %"],                              src: "LIR fédérale 2024" },

  // ── Québec ────────────────────────────────────────────────────────────────
  { id:151, pays:'qc', q: "Quel est le taux RRQ salarié en 2024 ?",
    rep: "6,40 %",         mr: ["5,95 %", "7,00 %", "6,00 %"],                          src: "Retraite Québec — RRQ 2024" },
  { id:152, pays:'qc', q: "Le taux RRQ est-il supérieur au taux RPC ?",
    rep: "Oui (6,40 % vs 5,95 %)", mr: ["Non, identiques depuis 2023", "Non, RRQ inférieur", "Oui depuis 2019 uniquement"], src: "Retraite Québec / ARC 2024" },
  { id:153, pays:'qc', q: "Quel est le taux RQAP salarié en 2024 ?",
    rep: "0,494 %",        mr: ["0,692 %", "0,500 %", "0,400 %"],                       src: "RQAP 2024" },
  { id:154, pays:'qc', q: "Quel est le taux RQAP employeur en 2024 ?",
    rep: "0,692 %",        mr: ["0,494 %", "0,500 %", "1,000 %"],                       src: "RQAP 2024" },
  { id:155, pays:'qc', q: "Quel est le plafond de la rémunération assurable RQAP 2024 ?",
    rep: "94 000 CAD",     mr: ["68 500 CAD", "63 200 CAD", "78 000 CAD"],               src: "RQAP 2024" },
  { id:156, pays:'qc', q: "Quel est le taux AE salarié réduit au Québec en 2024 ?",
    rep: "1,31 %",         mr: ["1,66 %", "1,20 %", "1,45 %"],                          src: "ARC T4001 2024 — taux Québec" },
  { id:157, pays:'qc', q: "Pourquoi le taux AE est-il réduit au Québec ?",
    rep: "Car le RQAP couvre les prestations parentales", mr: ["Car le RRQ est plus élevé", "Car le Québec finance son propre chômage", "Car le MAGA est plus bas au QC"], src: "LAE, art. 69" },
  { id:158, pays:'qc', q: "Durée maximale du congé maternité dans le régime de base RQAP ?",
    rep: "18 semaines",    mr: ["12 semaines", "15 semaines", "26 semaines"],            src: "RQAP — régime de base" },
  { id:159, pays:'qc', q: "Durée maximale du congé parental (régime de base RQAP) ?",
    rep: "40 semaines",    mr: ["25 semaines", "52 semaines", "35 semaines"],            src: "RQAP — régime de base" },
  { id:160, pays:'qc', q: "Durée du congé paternité dans le RQAP ?",
    rep: "5 semaines",     mr: ["2 semaines", "3 semaines", "10 semaines"],              src: "RQAP — congé paternité" },
  { id:161, pays:'qc', q: "Quel est le 1er taux du barème de l'impôt provincial québécois ?",
    rep: "14 %",           mr: ["15 %", "16 %", "12 %"],                                src: "LIQ (Revenu Québec) 2024" },
  { id:162, pays:'qc', q: "Quel est le taux marginal supérieur de l'impôt provincial QC ?",
    rep: "25,75 %",        mr: ["26 %", "24 %", "29 %"],                                src: "LIQ 2024" },
  { id:163, pays:'qc', q: "Quel est le montant personnel de base (MPB) provincial QC 2024 ?",
    rep: "17 183 CAD",     mr: ["15 705 CAD", "16 000 CAD", "18 500 CAD"],              src: "Revenu Québec — TP-1015.3 2024" },
  { id:164, pays:'qc', q: "Quel organisme perçoit l'impôt provincial au Québec ?",
    rep: "Revenu Québec",  mr: ["ARC (Agence du revenu du Canada)", "EDSC", "Retraite Québec"], src: "Loi sur l'administration fiscale (Québec)" },
  { id:165, pays:'qc', q: "Quel organisme gère le RQAP ?",
    rep: "Conseil de gestion de l'assurance parentale", mr: ["ARC", "Retraite Québec", "CNESST"], src: "Loi sur l'assurance parentale (Québec)" },
  { id:166, pays:'qc', q: "Le FSS (Fonds des services de santé QC) est-il à la charge de qui ?",
    rep: "Employeur uniquement", mr: ["Salarié uniquement", "Partagé 50/50", "État fédéral"], src: "Loi sur la Régie de l'assurance maladie du Québec" },
  { id:167, pays:'qc', q: "Quel est le taux de contribution CNT (normes du travail) employeur ?",
    rep: "0,06 %",         mr: ["0,06 % partagé", "0,50 %", "0,10 %"],                 src: "Loi sur les normes du travail (CNESST) 2024" },
  { id:168, pays:'qc', q: "Quel formulaire l'employé remplit-il pour ses retenues à la source provinciales QC ?",
    rep: "TP-1015.3",      mr: ["TD1", "T4001", "TP-1"],                                src: "Revenu Québec" },
  { id:169, pays:'qc', q: "Comment s'appelle le relevé annuel de paie provincial QC ?",
    rep: "Relevé 1 (R-1)", mr: ["T4", "Feuillet A", "TP-1"],                            src: "Revenu Québec" },
  { id:170, pays:'qc', q: "Que finance la CNESST au Québec ?",
    rep: "Accidents du travail et maladies professionnelles", mr: ["Le chômage", "Les retraites", "L'assurance maladie"], src: "LATMP (Québec)" },
  { id:171, pays:'qc', q: "Quel est le taux RRQ2 salarié dès 2024 ?",
    rep: "4 %",            mr: ["5,95 %", "6,40 %", "2 %"],                             src: "Retraite Québec — RRQ2 2024" },
  { id:172, pays:'qc', q: "Sur quelle assiette se calcule le RRQ2 ?",
    rep: "Gains entre le MGA et le MGAP2", mr: ["Totalité du salaire", "Gains au-delà de MGAP2", "Gains jusqu'au MGA"], src: "Retraite Québec — RRQ2" },
  { id:173, pays:'qc', q: "Le Québec gère-t-il son propre régime de retraite public ?",
    rep: "Oui, via Retraite Québec (RRQ)", mr: ["Non, géré par EDSC", "Non, géré par ARC", "Non, géré par la CNESST"], src: "Loi sur le RRQ (Québec)" },
  { id:174, pays:'qc', q: "Quel est le taux AE employeur réduit au Québec en 2024 ?",
    rep: "1,834 %",        mr: ["2,324 %", "1,31 %", "2,00 %"],                         src: "ARC T4001 2024 (1,31×1,4)" },
  { id:175, pays:'qc', q: "Taux de remplacement dans le régime particulier RQAP (vs 70 % régime de base) ?",
    rep: "75 %",           mr: ["70 %", "80 %", "90 %"],                                src: "RQAP — régime particulier" },
  { id:176, pays:'qc', q: "Durée du congé parental dans le régime particulier RQAP ?",
    rep: "25 semaines à 75 %", mr: ["40 semaines à 70 %", "18 semaines à 80 %", "30 semaines à 75 %"], src: "RQAP — régime particulier" },
  { id:177, pays:'qc', q: "Le FSS QC varie-t-il selon la masse salariale totale de l'employeur ?",
    rep: "Oui, de 1,65 % à 4,26 %", mr: ["Non, fixe à 2,05 %", "Non, fixe à 1,65 %", "Oui, de 0,06 % à 2,05 %"], src: "Loi RAMQ — FSS QC" },
  { id:178, pays:'qc', q: "La contribution CNT est-elle plafonnée par salarié ?",
    rep: "Oui, jusqu'au plafond RQAP (94 000 CAD)", mr: ["Non, sans plafond", "Oui, jusqu'au MGA (68 500 CAD)", "Oui, plafonnée à 63 200 CAD"], src: "CNESST 2024" },
  { id:179, pays:'qc', q: "Quel est le 2ème taux du barème provincial QC (de ~51 780 à ~103 545 CAD) ?",
    rep: "19 %",           mr: ["20 %", "24 %", "17 %"],                                src: "LIQ 2024" },
  { id:180, pays:'qc', q: "Le RQAP couvre-t-il aussi les travailleurs autonomes ?",
    rep: "Oui, à un taux différent", mr: ["Non, uniquement les salariés", "Non, uniquement les employeurs", "Oui, au même taux que les salariés"], src: "Loi sur l'assurance parentale (Québec)" },

  // ── Fonction Publique Territoriale (FPT) ─────────────────────────────────
  { id:181, pays:'fpt', q: "Quel est le taux CNRACL salarié applicable depuis 2019 ?",
    rep: "11,10 %",        mr: ["10,56 %", "11,40 %", "10,83 %"],                       src: "Décret n°2011-291 — taux cible 2019" },
  { id:182, pays:'fpt', q: "Quel est le taux CNRACL employeur (collectivité) ?",
    rep: "30,65 %",        mr: ["28,12 %", "32,00 %", "26,50 %"],                       src: "Décret n°2011-291" },
  { id:183, pays:'fpt', q: "Les fonctionnaires FPT titulaires cotisent-ils à l'assurance chômage ?",
    rep: "Non",            mr: ["Oui, au même taux que le privé", "Oui, à taux réduit 0,50 %", "Non, mais ils cotisent au FNPE"], src: "Statut FPT — L. 84-53" },
  { id:184, pays:'fpt', q: "La réduction Fillon s'applique-t-elle aux employeurs publics FPT ?",
    rep: "Non",            mr: ["Oui, au même niveau que le privé", "Oui, uniquement pour les contractuels", "Non, mais un abattement spécifique existe"], src: "CSS, art. L241-13 — inapplicable au secteur public" },
  { id:185, pays:'fpt', q: "Quel régime de retraite remplace l'AGIRC-ARRCO pour les agents FPT titulaires ?",
    rep: "CNRACL",         mr: ["IRCANTEC", "AGIRC-ARRCO aussi", "Régime général SS"],   src: "Décret n°2003-1306" },
  { id:186, pays:'fpt', q: "L'IRCANTEC concerne-t-il les agents titulaires de la FPT ?",
    rep: "Non, il couvre les contractuels et non-titulaires", mr: ["Oui, tous les agents FPT", "Non, personne en FPT", "Oui, uniquement catégorie A"], src: "Décret n°70-1277" },
  { id:187, pays:'fpt', q: "Quel était le taux CNRACL salarié en 2016 ?",
    rep: "10,29 %",        mr: ["11,10 %", "10,56 %", "9,82 %"],                        src: "Décret n°2011-291" },
  { id:188, pays:'fpt', q: "Quel était le taux CNRACL salarié en 2017 ?",
    rep: "10,56 %",        mr: ["10,29 %", "10,83 %", "11,10 %"],                       src: "Décret n°2011-291" },
  { id:189, pays:'fpt', q: "Quel était le taux CNRACL salarié en 2018 ?",
    rep: "10,83 %",        mr: ["10,56 %", "11,10 %", "10,29 %"],                       src: "Décret n°2011-291" },
  { id:190, pays:'fpt', q: "Quel décret a encadré la convergence des taux CNRACL vers le régime général ?",
    rep: "Décret n°2011-291", mr: ["Décret n°2014-1326", "Décret n°2003-1306", "Loi n°2010-1330"], src: "Décret n°2011-291 du 15/03/2011" },
  { id:191, pays:'fpt', q: "La CSG/CRDS est-elle prélevée sur les agents FPT ?",
    rep: "Oui, aux mêmes taux qu'en secteur privé", mr: ["Non, exonération pour les fonctionnaires", "Oui, mais à taux réduit 6 %", "Non, remplacée par une contribution spécifique"], src: "CSS, art. L136-1 — applicable à la FP" },
  { id:192, pays:'fpt', q: "Les allocations familiales patronales sont-elles dues en FPT ?",
    rep: "Oui, aux mêmes taux",  mr: ["Non, prises en charge par la CAF", "Oui, à taux réduit", "Non, régime spécial"], src: "CSS, art. L241-6 — applicable" },
  { id:193, pays:'fpt', q: "Le taux CNRACL patronal représente combien de fois le taux salarié ?",
    rep: "Presque 3 fois (30,65 % vs 11,10 %)", mr: ["Identique au taux salarié", "2 fois le taux salarié", "La moitié du taux salarié"], src: "Décret n°2011-291" },
  { id:194, pays:'fpt', q: "Le plan de convergence CNRACL visait le rapprochement vers quel régime ?",
    rep: "Le régime général des salariés du privé", mr: ["Le régime des fonctionnaires d'État", "L'AGIRC-ARRCO", "Le régime agricole MSA"], src: "Décret n°2011-291 — exposé des motifs" },
  { id:195, pays:'fpt', q: "Le taux AT/MP suit-il les mêmes règles en FPT qu'en secteur privé ?",
    rep: "Oui, variable selon le risque", mr: ["Non, taux fixe à 0,70 %", "Non, l'État assume directement", "Oui, mais plafonné à 1 %"], src: "CSS, art. L242-5 — applicable FP" },
  { id:196, pays:'fpt', q: "Le FNAL est-il dû par les employeurs territoriaux ?",
    rep: "Oui", mr: ["Non, exonération totale", "Oui, uniquement le taux 0,10 %", "Non, remplacé par une taxe spécifique"], src: "CSS, art. L834-1" },
  { id:197, pays:'fpt', q: "L'assiette de la CNRACL est-elle plafonnée ?",
    rep: "Non, traitement brut total sans plafond", mr: ["Oui, plafonnée au PMSS", "Oui, plafonnée à 4 × PMSS", "Oui, plafonnée au SMIC × 3,5"], src: "Décret n°2003-1306" },
  { id:198, pays:'fpt', q: "Les agents contractuels en FPT cotisent-ils à la CNRACL ?",
    rep: "Non, ils relèvent de l'IRCANTEC", mr: ["Oui, comme les titulaires", "Oui, à taux réduit", "Non, ils relèvent du régime général"], src: "Décret n°70-1277 — IRCANTEC" },
  { id:199, pays:'fpt', q: "Quel est le taux total CNRACL (sal + pat) depuis 2019 ?",
    rep: "41,75 %",        mr: ["33,00 %", "38,00 %", "44,00 %"],                       src: "Décret n°2011-291 (11,10+30,65)" },
  { id:200, pays:'fpt', q: "La CNRACL a-t-elle été créée avant ou après le régime général ?",
    rep: "Avant — elle date de 1947", mr: ["Après, créée en 1975", "Après, créée en 1955", "En même temps, 1945"], src: "Ordonnance n°45-993 / Loi n°47-1465" },
  { id:201, pays:'fpt', q: "Le taux CNRACL patronal a-t-il bougé pendant le plan de convergence ?",
    rep: "Non, maintenu à 30,65 % tout au long", mr: ["Oui, il a augmenté avec le salarié", "Oui, il a baissé progressivement", "Non, il était à 28 % avant 2012"], src: "Décret n°2011-291" },
  { id:202, pays:'fpt', q: "Le décret n°2011-291 visait un taux salarié cible de combien en 2019 ?",
    rep: "11,10 %",        mr: ["12,00 %", "10,00 %", "11,30 %"],                       src: "Décret n°2011-291" },
  { id:203, pays:'fpt', q: "La CNRACL couvre-t-elle aussi l'invalidité des agents FPT ?",
    rep: "Oui, retraite, invalidité et réversion", mr: ["Non, géré par la SS", "Non, par la MGEN", "Oui mais uniquement la retraite"], src: "Décret n°2003-1306" },
  { id:204, pays:'fpt', q: "La CNRACL est gérée par quelle institution nationale ?",
    rep: "Caisse des Dépôts et Consignations", mr: ["CNAM", "AGIRC-ARRCO", "Caisse nationale d'assurance retraite"], src: "Art. L222-1 CSS / CDC" },
  { id:205, pays:'fpt', q: "Le régime local Alsace-Moselle s'applique-t-il aux agents FPT de ces départements ?",
    rep: "Oui, comme pour le secteur privé", mr: ["Non, les fonctionnaires en sont exonérés", "Non, uniquement pour les contractuels", "Oui mais à taux réduit 0,65 %"], src: "Droit local Alsace-Moselle — applicable à la FP" },
  { id:206, pays:'fpt', q: "Quel était le taux salarié CNRACL au lancement du plan de convergence (2012) ?",
    rep: "8,39 %",         mr: ["9,00 %", "10,00 %", "7,85 %"],                         src: "Décret n°2011-291 — entrée en vigueur 01/01/2012" },
  { id:207, pays:'fpt', q: "Le PAS (prélèvement à la source) s'applique-t-il aux agents FPT ?",
    rep: "Oui, depuis 2019 comme pour tous les salariés", mr: ["Non, les fonctionnaires déclarent séparément", "Non, impôt prélevé trimestriellement", "Oui, depuis 2024 uniquement"], src: "CGI, art. 204 A — applicable" },
  { id:208, pays:'fpt', q: "En FPT, quel est le traitement de référence pour le calcul de la CNRACL ?",
    rep: "Traitement indiciaire brut (hors primes et indemnités)", mr: ["Traitement brut total primes incluses", "Traitement net imposable", "Traitement indiciaire + NBI"], src: "Décret n°2003-1306 — assiette CNRACL" },
  { id:209, pays:'fpt', q: "La cotisation maladie employeur en FPT est-elle identique au secteur privé ?",
    rep: "Oui, même taux de base (7 %)", mr: ["Non, exonération totale en FPT", "Non, taux réduit à 5 %", "Oui, mais uniquement pour les collectivités > 50 agents"], src: "CSS, art. L241-2 — applicable" },
  { id:210, pays:'fpt', q: "La part déductible de la CSG est-elle identique pour les agents FPT et les salariés du privé ?",
    rep: "Oui, 6,8 % déductibles sur les 9,2 % totaux", mr: ["Non, 7,5 % déductibles pour les fonctionnaires", "Non, la totalité est non-déductible en FP", "Non, taux réduit 5 %"], src: "CGI, art. 154 quinquies — applicable FP" },

  // ── Gestion de la paie ────────────────────────────────────────────────────
  { id:211, pays:'fr', q: "Combien de temps l'employeur est-il tenu de conserver les doubles des bulletins de paie ?",
    rep: "5 ans", mr: ["3 ans", "7 ans", "10 ans"], src: "C. trav., art. L3243-4" },
  { id:212, pays:'fr', q: "Quel est le délai de prescription pour réclamer un rappel de salaire ?",
    rep: "3 ans", mr: ["1 an", "2 ans", "5 ans"], src: "C. trav., art. L3245-1" },
  { id:213, pays:'fr', q: "Dans quel délai maximum avant l'embauche la DPAE peut-elle être transmise à l'URSSAF ?",
    rep: "8 jours", mr: ["24 heures", "15 jours", "48 heures"], src: "CSS, art. R1221-1" },
  { id:214, pays:'fr', q: "Quel est le délai de carence légal de la Sécurité sociale avant versement des indemnités journalières maladie ?",
    rep: "3 jours", mr: ["1 jour", "5 jours", "7 jours"], src: "CSS, art. L323-1" },

  // ── France — histoire de la paie et des cotisations ────────────────────────
  //    30 questions : congés payés, temps de travail, salaire minimum,
  //    bulletin, Sécurité sociale, cotisations, retraites.

  // ─ Congés payés & temps de travail ─
  { id:215, pays:'frh', q: "Quel chef du gouvernement du Front populaire fait voter les premiers congés payés en 1936 ?",
    rep: "Léon Blum", mr: ["Édouard Daladier", "Pierre Laval", "Raymond Poincaré"], src: "Loi du 20 juin 1936" },
  { id:216, pays:'frh', q: "Combien de semaines de congés payés la loi de 1936 accorde-t-elle ?",
    rep: "2 semaines", mr: ["1 semaine", "3 semaines", "4 semaines"], src: "Loi du 20 juin 1936" },
  { id:217, pays:'frh', q: "En quelle année la troisième semaine de congés payés est-elle généralisée ?",
    rep: "1956", mr: ["1948", "1962", "1969"], src: "Loi du 27 mars 1956" },
  { id:218, pays:'frh', q: "En quelle année la quatrième semaine de congés payés est-elle généralisée ?",
    rep: "1969", mr: ["1956", "1973", "1982"], src: "Loi du 16 mai 1969" },
  { id:219, pays:'frh', q: "Sous quel Premier ministre est instaurée la cinquième semaine de congés payés en 1982 ?",
    rep: "Pierre Mauroy", mr: ["Raymond Barre", "Jacques Chirac", "Laurent Fabius"], src: "Ordonnance du 16 janvier 1982" },
  { id:220, pays:'frh', q: "En quelle année la semaine de 40 heures est-elle instaurée ?",
    rep: "1936", mr: ["1919", "1946", "1968"], src: "Loi du 21 juin 1936" },
  { id:221, pays:'frh', q: "Quelle ministre porte les lois sur les 35 heures (1998-2000) ?",
    rep: "Martine Aubry", mr: ["Élisabeth Guigou", "Ségolène Royal", "Nicole Notat"], src: "Lois Aubry I et II" },

  // ─ Salaire minimum & bulletin de paie ─
  { id:222, pays:'frh', q: "En quelle année le SMIG (salaire minimum interprofessionnel garanti) est-il créé ?",
    rep: "1950", mr: ["1936", "1945", "1958"], src: "Loi du 11 février 1950" },
  { id:223, pays:'frh', q: "En quelle année le SMIC remplace-t-il le SMIG ?",
    rep: "1970", mr: ["1968", "1975", "1981"], src: "Loi du 2 janvier 1970" },
  { id:224, pays:'frh', q: "De combien les accords de Grenelle (mai 1968) augmentent-ils le SMIG ?",
    rep: "+35 %", mr: ["+10 %", "+20 %", "+50 %"], src: "Accords de Grenelle, mai 1968" },
  { id:225, pays:'frh', q: "Depuis quelle année la remise d'un bulletin de paie au salarié est-elle obligatoire ?",
    rep: "1931", mr: ["1910", "1945", "1958"], src: "Loi du 4 mars 1931" },
  { id:226, pays:'frh', q: "En quelle année la mensualisation du salaire est-elle généralisée par la loi ?",
    rep: "1978", mr: ["1968", "1973", "1982"], src: "Loi du 19 janvier 1978" },
  { id:227, pays:'frh', q: "En quelle année le prélèvement à la source de l'impôt sur le revenu entre-t-il en vigueur ?",
    rep: "2019", mr: ["2017", "2018", "2020"], src: "Loi de finances 2017, en vigueur au 01/01/2019" },

  // ─ Sécurité sociale & cotisations ─
  { id:228, pays:'frh', q: "Quel ministre communiste du Travail est le père de la Sécurité sociale en 1945 ?",
    rep: "Ambroise Croizat", mr: ["Maurice Thorez", "Benoît Frachon", "Jacques Duclos"], src: "Ordonnances des 4 et 19 octobre 1945" },
  { id:229, pays:'frh', q: "En quelle année la loi sur l'indemnisation des accidents du travail (ancêtre de la cotisation AT/MP) est-elle votée ?",
    rep: "1898", mr: ["1884", "1910", "1928"], src: "Loi du 9 avril 1898" },
  { id:230, pays:'frh', q: "Quelles lois créent les assurances sociales (maladie, maternité, vieillesse) avant la Sécurité sociale ?",
    rep: "Les lois de 1928 et 1930", mr: ["Les lois de 1900 et 1902", "Les lois de 1919 et 1920", "Les lois de 1936 et 1937"], src: "Lois du 5 avril 1928 et du 30 avril 1930" },
  { id:231, pays:'frh', q: "En quelle année les allocations familiales sont-elles généralisées à tous les salariés ?",
    rep: "1932", mr: ["1913", "1928", "1945"], src: "Loi Landry du 11 mars 1932" },
  { id:232, pays:'frh', q: "Quel Premier ministre crée la CSG en 1991 ?",
    rep: "Michel Rocard", mr: ["Pierre Bérégovoy", "Édith Cresson", "Lionel Jospin"], src: "Loi de finances du 29 décembre 1990" },
  { id:233, pays:'frh', q: "Quel était le taux de la CSG à sa création en 1991 ?",
    rep: "1,1 %", mr: ["0,5 %", "2,4 %", "3,4 %"], src: "Loi de finances du 29 décembre 1990" },
  { id:234, pays:'frh', q: "Quel Premier ministre crée la CRDS en 1996 ?",
    rep: "Alain Juppé", mr: ["Édouard Balladur", "Lionel Jospin", "Michel Rocard"], src: "Ordonnance du 24 janvier 1996" },
  { id:235, pays:'frh', q: "Quelle cotisation salariale est supprimée en 2018, compensée par une hausse de la CSG ?",
    rep: "La cotisation chômage (2,40 %)", mr: ["La cotisation vieillesse plafonnée", "La cotisation retraite complémentaire", "La cotisation APEC"], src: "LFSS 2018" },
  { id:236, pays:'frh', q: "Quel Premier ministre instaure la journée de solidarité (lundi de Pentecôte travaillé) en 2004 ?",
    rep: "Jean-Pierre Raffarin", mr: ["Dominique de Villepin", "François Fillon", "Lionel Jospin"], src: "Loi du 30 juin 2004" },
  { id:237, pays:'frh', q: "En quelle année la réduction générale de cotisations patronales dite « Fillon » est-elle créée ?",
    rep: "2003", mr: ["1995", "2007", "2010"], src: "Loi Fillon du 17 janvier 2003" },

  // ─ Retraites ─
  { id:238, pays:'frh', q: "En quelle année la première retraite obligatoire (retraites ouvrières et paysannes) est-elle créée ?",
    rep: "1910", mr: ["1898", "1930", "1945"], src: "Loi du 5 avril 1910" },
  { id:239, pays:'frh', q: "Quel régime généralise la retraite par répartition en 1941 ?",
    rep: "Le régime de Vichy", mr: ["Le Front populaire", "Le GPRF", "La IIIe République"], src: "Loi du 14 mars 1941 (AVTS)" },
  { id:240, pays:'frh', q: "En quelle année l'AGIRC (retraite complémentaire des cadres) est-elle créée ?",
    rep: "1947", mr: ["1945", "1958", "1961"], src: "Convention collective nationale du 14 mars 1947" },
  { id:241, pays:'frh', q: "Quel président instaure la retraite à 60 ans en 1982 ?",
    rep: "François Mitterrand", mr: ["Valéry Giscard d'Estaing", "Georges Pompidou", "Charles de Gaulle"], src: "Ordonnance du 26 mars 1982" },
  { id:242, pays:'frh', q: "Quelle réforme de 1993 fait passer le calcul de la retraite des 10 aux 25 meilleures années ?",
    rep: "La réforme Balladur", mr: ["La réforme Juppé", "La réforme Fillon", "La réforme Touraine"], src: "Loi du 22 juillet 1993" },
  { id:243, pays:'frh', q: "Quel ministre du Travail porte la réforme repoussant l'âge de la retraite de 60 à 62 ans en 2010 ?",
    rep: "Éric Woerth", mr: ["Xavier Bertrand", "Jean-Louis Borloo", "François Fillon"], src: "Loi du 9 novembre 2010" },
  { id:244, pays:'frh', q: "Quelle Première ministre fait adopter par 49.3 le report de l'âge de la retraite à 64 ans en 2023 ?",
    rep: "Élisabeth Borne", mr: ["Édith Cresson", "Ségolène Royal", "Aurélie Filippetti"], src: "Loi du 14 avril 2023" },
];

const QZ_SESSION_KEY = 'xenna-qz-pending';
const QZ_START_LABELS = {
  fr: 'Démarrer', fpt: 'Démarrer', ch: 'Starten',
  lu: 'Ufänken',  it: 'Inizia',    ca: 'Start', qc: 'Commencer',
  frh: 'Démarrer',
};

let _qzPays       = 'fr';
let _qzPlaying    = false;
let _qzCurrent    = null;
let _qzTimer      = null;
let _qzStart      = 0;
let _qzOk         = 0;
let _qzTotal      = 0;
let _qzLastId     = -1;
let _qzAnswered   = false;
let _qzFiftyTimer = null;
let _qzFiftyReady = false;
let _qzFiftyUsed  = false;
let _qzFiftyCount = 0;

function _qzNorm(s) {
  return String(s).toLowerCase()
    .replace(/\s+/g, '').replace(/,/g, '.').replace(/[€%°~]/g, '').trim();
}
function _qzMatch(correct, input) {
  if (!input.trim()) return false;
  const nc = _qzNorm(correct), nu = _qzNorm(input);
  if (nc === nu) return true;
  const fc = parseFloat(nc), fu = parseFloat(nu);
  return !isNaN(fc) && !isNaN(fu) && Math.abs(fc - fu) < 0.001;
}
function _qzShuffle(arr) {
  const a = [...arr];
  for (let i = a.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [a[i], a[j]] = [a[j], a[i]];
  }
  return a;
}
function _qzFmt(ms) {
  return (ms / 1000).toFixed(1) + 's';
}

function _qzStopTimer() {
  if (_qzTimer) { clearInterval(_qzTimer); _qzTimer = null; }
  return Date.now() - _qzStart;
}

function _qzLoadQuestion(q) {
  sessionStorage.setItem(QZ_SESSION_KEY, JSON.stringify({ pays: q.pays, id: q.id }));
  _qzCurrent  = q;
  _qzAnswered = false;

  document.getElementById('qz-num').textContent    = 'Q.' + String(q.id).padStart(2, '0');
  document.getElementById('qz-q').textContent      = q.q;
  document.getElementById('qz-clock').textContent  = '0.0s';
  document.getElementById('qz-input').value        = '';
  document.getElementById('qz-input').disabled     = false;
  document.getElementById('qz-result').style.display = 'none';
  document.getElementById('qz-saisie').style.opacity = '1';
  const cb = document.getElementById('qz-carre-cb');
  cb.checked = false;
  document.getElementById('qz-carre').style.display  = 'none';
  document.getElementById('qz-carre').style.opacity  = '1';
  document.getElementById('qz-fifty').style.display  = 'none';

  // 50/50 : reset + timer 8s
  if (_qzFiftyTimer) clearTimeout(_qzFiftyTimer);
  _qzFiftyReady = false;
  _qzFiftyUsed  = false;
  _qzFiftyTimer = setTimeout(() => {
    _qzFiftyReady = true;
    if (!_qzAnswered && !_qzFiftyUsed &&
        document.getElementById('qz-carre').style.display !== 'none') {
      document.getElementById('qz-fifty').style.display = 'inline-flex';
    }
  }, 15000);

  // Carré : 4 choix mélangés
  const choices = _qzShuffle([q.rep, ...q.mr]);
  const zone = document.getElementById('qz-choix');
  zone.innerHTML = '';
  choices.forEach(c => {
    const btn = document.createElement('button');
    btn.className = 'qz-choice';
    btn.textContent = c;
    btn.onclick = () => quizzChoix(c);
    zone.appendChild(btn);
  });

  // Timer
  if (_qzTimer) clearInterval(_qzTimer);
  _qzStart = Date.now();
  _qzTimer = setInterval(() => {
    if (!_qzAnswered)
      document.getElementById('qz-clock').textContent = _qzFmt(Date.now() - _qzStart);
  }, 100);

  document.getElementById('qz-input').focus();
}

function _qzShowResult(isCorrect, elapsed, via) {
  _qzAnswered = true;
  sessionStorage.removeItem(QZ_SESSION_KEY);
  document.querySelector('.qz-pays-bar').classList.remove('locked');
  _qzStopTimer();
  _qzTotal++;
  if (isCorrect) _qzOk++;

  if (_qzFiftyTimer) { clearTimeout(_qzFiftyTimer); _qzFiftyTimer = null; }
  document.getElementById('qz-fifty').style.display = 'none';
  const scoreSuffix = _qzFiftyCount > 0 ? ' · ½ ×' + _qzFiftyCount : '';
  document.getElementById('qz-score').textContent = _qzOk + ' / ' + _qzTotal + scoreSuffix;

  const verdict = document.getElementById('qz-verdict');
  verdict.textContent = isCorrect ? '✓ JUSTE' : '✗ FAUX';
  verdict.className   = 'qz-verdict ' + (isCorrect ? 'ok' : 'ko');

  const ansLine = document.getElementById('qz-ans-line');
  if (isCorrect) {
    ansLine.innerHTML = 'Bonne réponse : <strong>' + _qzCurrent.rep + '</strong>';
  } else {
    ansLine.innerHTML = 'Réponse correcte : <strong>' + _qzCurrent.rep + '</strong>';
  }

  const fiftyTag = _qzFiftyUsed ? ' · ½' : '';
  document.getElementById('qz-time-line').textContent = '⏱ ' + _qzFmt(elapsed) + ' (' + via + fiftyTag + ')';
  document.getElementById('qz-src-line').textContent  = _qzCurrent.src;
  document.getElementById('qz-result').style.display  = 'block';

  // Désactive la saisie
  document.getElementById('qz-input').disabled = true;
  document.getElementById('qz-saisie').style.opacity = '0.4';

  // Colore les boutons carré (bonne réponse en vert, le reste grisé)
  document.querySelectorAll('.qz-choice').forEach(btn => {
    btn.disabled = true;
    if (btn.textContent === _qzCurrent.rep) btn.classList.add('qz-correct');
  });
}

function quizzToggleCarre(cb) {
  document.getElementById('qz-carre').style.display = cb.checked ? 'block' : 'none';
  if (cb.checked && _qzFiftyReady && !_qzAnswered && !_qzFiftyUsed)
    document.getElementById('qz-fifty').style.display = 'inline-flex';
}

function quizzFiftyFifty() {
  if (_qzFiftyUsed || _qzAnswered) return;
  _qzFiftyUsed = true;
  _qzFiftyCount++;
  document.getElementById('qz-fifty').style.display = 'none';

  const allBtns  = Array.from(document.querySelectorAll('.qz-choice'));
  const wrongBtns = allBtns.filter(b => b.textContent !== _qzCurrent.rep);
  _qzShuffle(wrongBtns).slice(0, 2).forEach(btn => {
    btn.disabled = true;
    btn.style.opacity = '0.12';
    btn.style.pointerEvents = 'none';
  });

  const s = _qzFiftyCount > 0 ? ' · ½ ×' + _qzFiftyCount : '';
  document.getElementById('qz-score').textContent = _qzOk + ' / ' + _qzTotal + s;
}

function quizzValider() {
  if (_qzAnswered) return;
  const val = document.getElementById('qz-input').value;
  const elapsed = Date.now() - _qzStart;
  const ok = _qzMatch(_qzCurrent.rep, val);
  document.querySelectorAll('.qz-choice').forEach(btn => {
    btn.disabled = true;
    if (btn.textContent === _qzCurrent.rep) btn.classList.add('qz-correct');
  });
  _qzShowResult(ok, elapsed, 'saisie');
}

function quizzChoix(choice) {
  if (_qzAnswered) return;
  const elapsed = Date.now() - _qzStart;
  const ok = (choice === _qzCurrent.rep);
  document.querySelectorAll('.qz-choice').forEach(btn => {
    btn.disabled = true;
    if (btn.textContent === _qzCurrent.rep)      btn.classList.add('qz-correct');
    else if (btn.textContent === choice && !ok)  btn.classList.add('qz-wrong');
  });
  _qzShowResult(ok, elapsed, 'carré');
}

function quizzNext() {
  _qzPlaying = true;
  document.querySelector('.qz-pays-bar').classList.add('locked');

  // Anti-F5 : si une question était en cours, on la recharge
  const raw = sessionStorage.getItem(QZ_SESSION_KEY);
  if (raw) {
    try {
      const { pays, id } = JSON.parse(raw);
      if (pays === _qzPays) {
        const q = QUIZZ_DATA.find(x => x.id === id);
        if (q) { _qzLastId = q.id; _qzLoadQuestion(q); return; }
      }
    } catch (_) {}
    sessionStorage.removeItem(QZ_SESSION_KEY);
  }

  const pool = QUIZZ_DATA.filter(q => q.pays === _qzPays && q.id !== _qzLastId);
  const next = pool[Math.floor(Math.random() * pool.length)];
  _qzLastId = next.id;
  _qzLoadQuestion(next);
}

function quizzStart() {
  document.getElementById('qz-start-wrap').style.display = 'none';
  document.getElementById('qz-play').style.display = 'block';
  quizzNext();
}

function quizzSetPays(p) {
  if (_qzPlaying && !_qzAnswered) return; // question en cours — bloqué
  _qzPays    = p;
  _qzLastId  = -1;
  _qzOk      = 0;
  _qzTotal   = 0;
  _qzPlaying = false;
  sessionStorage.removeItem(QZ_SESSION_KEY);
  document.getElementById('qz-score').textContent = '0 / 0';
  document.querySelectorAll('.qz-pays-btn').forEach(b =>
    b.classList.toggle('on', b.dataset.pays === p));
  document.getElementById('qz-start-btn').textContent = QZ_START_LABELS[p] || 'Démarrer';
  document.getElementById('qz-start-wrap').style.display = 'flex';
  document.getElementById('qz-play').style.display = 'none';
  document.querySelector('.qz-pays-bar').classList.remove('locked');
  if (_qzLbOpen)   quizzChargerLb();
  if (_qzVoteOpen) quizzChargerVotes();
}

function quizzInit() {
  const raw = sessionStorage.getItem(QZ_SESSION_KEY);
  if (raw) {
    try {
      const { pays, id } = JSON.parse(raw);
      const q = QUIZZ_DATA.find(x => x.id === id);
      if (q) {
        _qzPays    = pays;
        _qzLastId  = q.id;
        _qzPlaying = true;
        document.querySelectorAll('.qz-pays-btn').forEach(b =>
          b.classList.toggle('on', b.dataset.pays === pays));
        document.getElementById('qz-start-btn').textContent = QZ_START_LABELS[pays] || 'Démarrer';
        document.getElementById('qz-start-wrap').style.display = 'none';
        document.getElementById('qz-play').style.display = 'block';
        document.querySelector('.qz-pays-bar').classList.add('locked');
        _qzLoadQuestion(q);
        return;
      }
    } catch (_) {}
    sessionStorage.removeItem(QZ_SESSION_KEY);
  }
  document.getElementById('qz-start-btn').textContent = QZ_START_LABELS[_qzPays] || 'Démarrer';
}

// Exposition pour les onclick HTML
window.quizzValider     = quizzValider;
window.quizzChoix       = quizzChoix;
window.quizzNext        = quizzNext;
window.quizzStart       = quizzStart;
window.quizzSetPays     = quizzSetPays;
window.quizzToggleCarre = quizzToggleCarre;
window.quizzFiftyFifty  = quizzFiftyFifty;


// ── Quizz community — Leaderboard & Suggestions ──────────────────────────────

const QZ_PAYS_LABELS = {
  fr:'FRANCE', fpt:'FONCTION PUBLIQUE', ch:'SUISSE',
  lu:'LUXEMBOURG', it:'ITALIE', ca:'CANADA', qc:'QUÉBEC',
  frh:'HISTOIRE DE LA PAIE',
};
let _qzLbOpen = false, _qzSuggOpen = false;

function quizzToggleLb() {
  _qzLbOpen = !_qzLbOpen;
  document.getElementById('qz-lb-panel').classList.toggle('open', _qzLbOpen);
  if (_qzLbOpen) quizzChargerLb();
}

async function quizzChargerLb() {
  document.getElementById('qz-lb-pays-label').textContent =
    QZ_PAYS_LABELS[_qzPays] || _qzPays.toUpperCase();

  const submitWrap = document.getElementById('qz-lb-submit');
  if (_qzTotal >= 5 && _qzPlaying) {
    document.getElementById('qz-lb-score-display').textContent =
      `${_qzOk} / ${_qzTotal} (${Math.round(_qzOk * 100 / _qzTotal)} %)`;
    submitWrap.style.display = 'block';
  } else {
    submitWrap.style.display = 'none';
  }

  const body = document.getElementById('qz-lb-body');
  body.innerHTML = `<div class="qz-lb-empty">chargement…</div>`;
  try {
    const r = await fetch(`/quizz/leaderboard/${encodeURIComponent(_qzPays)}`);
    if (!r.ok) throw new Error(`HTTP ${r.status}`);
    const rows = await r.json();
    body.innerHTML = rows.length
      ? `<table class="qz-lb-table"><thead><tr>
           <th>#</th><th>Pseudo</th><th>Score</th><th>%</th><th>Date</th>
         </tr></thead><tbody>${rows.map(e => `<tr>
           <td>${e.rang}</td><td>${esc(e.pseudo)}</td>
           <td>${e.score} / ${e.total}</td><td>${e.pct} %</td><td>${esc(e.date)}</td>
         </tr>`).join('')}</tbody></table>`
      : `<div class="qz-lb-empty">Aucune entrée pour ce pays.</div>`;
  } catch (e) {
    body.innerHTML = `<div class="qz-lb-empty" style="color:var(--red)">${esc(errToStr(e))}</div>`;
  }
}

async function quizzSoumettrScore() {
  const pseudo = document.getElementById('qz-lb-pseudo').value.trim();
  const msgEl  = document.getElementById('qz-lb-msg');
  msgEl.textContent = ''; msgEl.className = 'qz-lb-msg';
  if (!pseudo) { msgEl.textContent = 'Pseudo requis.'; msgEl.className += ' err'; return; }
  if (_qzTotal < 5) { msgEl.textContent = 'Minimum 5 questions requises.'; msgEl.className += ' err'; return; }
  try {
    const r = await fetch('/quizz/score', {
      method:  'POST',
      headers: { 'Content-Type': 'application/json' },
      body:    JSON.stringify({ pseudo, pays: _qzPays, score: _qzOk, total: _qzTotal }),
    });
    if (!r.ok) throw new Error(await r.text() || `HTTP ${r.status}`);
    msgEl.textContent = '✓ Score soumis !'; msgEl.className += ' ok';
    document.getElementById('qz-lb-submit').style.display = 'none';
    quizzChargerLb();
  } catch (e) { msgEl.textContent = errToStr(e); msgEl.className += ' err'; }
}

function quizzToggleSugg() {
  _qzSuggOpen = !_qzSuggOpen;
  document.getElementById('qz-sugg-panel').classList.toggle('open', _qzSuggOpen);
}

async function quizzEnvoyerSuggestion() {
  const question = document.getElementById('qz-sugg-q').value.trim();
  const msgEl    = document.getElementById('qz-sugg-msg');
  msgEl.textContent = ''; msgEl.className = 'qz-lb-msg';
  if (!question) { msgEl.textContent = 'Question requise.'; msgEl.className += ' err'; return; }

  const alt1 = document.getElementById('qz-sugg-alt1').value.trim();
  const alt2 = document.getElementById('qz-sugg-alt2').value.trim();
  const repsAlt = [alt1, alt2].filter(Boolean).join(' | ') || null;

  const mr1 = document.getElementById('qz-sugg-mr1').value.trim();
  const mr2 = document.getElementById('qz-sugg-mr2').value.trim();
  const mr3 = document.getElementById('qz-sugg-mr3').value.trim();
  const mauvaiseRep = [mr1, mr2, mr3].filter(Boolean).join(' | ') || null;

  try {
    const r = await fetch('/quizz/suggestion', {
      method:  'POST',
      headers: { 'Content-Type': 'application/json' },
      body:    JSON.stringify({
        pays:         _qzPays,
        question,
        reponse:      document.getElementById('qz-sugg-rep').value.trim()    || null,
        repsAlt,
        mauvaiseRep,
        source:       document.getElementById('qz-sugg-src').value.trim()    || null,
        pseudo:       document.getElementById('qz-sugg-pseudo').value.trim() || null,
      }),
    });
    if (!r.ok) throw new Error(await r.text() || `HTTP ${r.status}`);
    msgEl.textContent = '✓ Merci pour votre contribution !'; msgEl.className += ' ok';
    ['qz-sugg-q','qz-sugg-rep','qz-sugg-alt1','qz-sugg-alt2',
     'qz-sugg-mr1','qz-sugg-mr2','qz-sugg-mr3','qz-sugg-src','qz-sugg-pseudo'].forEach(id => {
      document.getElementById(id).value = '';
    });
    if (_qzVoteOpen) quizzChargerVotes();
  } catch (e) { msgEl.textContent = errToStr(e); msgEl.className += ' err'; }
}

// ── Quizz — Vote communautaire ────────────────────────────────────────────────

const QZ_VOTER_KEY  = 'xenna-voter-id';
const QZ_VOTED_KEY  = 'xenna-qz-voted-ids';
let _qzVoteOpen = false;

function _qzGetVoterId() {
  let id = localStorage.getItem(QZ_VOTER_KEY);
  if (!id) {
    try { id = crypto.randomUUID(); } catch (_) {}
    if (!id) id = Math.random().toString(36).slice(2, 10) + Date.now().toString(36);
    localStorage.setItem(QZ_VOTER_KEY, id);
  }
  return id;
}

function _qzVotedIds() {
  try { return new Set(JSON.parse(localStorage.getItem(QZ_VOTED_KEY) || '[]')); }
  catch { return new Set(); }
}

function _qzMarkVoted(id) {
  const ids = _qzVotedIds();
  ids.add(id);
  localStorage.setItem(QZ_VOTED_KEY, JSON.stringify([...ids]));
}

function quizzToggleVote() {
  _qzVoteOpen = !_qzVoteOpen;
  document.getElementById('qz-vote-panel').classList.toggle('open', _qzVoteOpen);
  if (_qzVoteOpen) quizzChargerVotes();
}

async function quizzChargerVotes() {
  document.getElementById('qz-vote-pays-label').textContent =
    QZ_PAYS_LABELS[_qzPays] || _qzPays.toUpperCase();
  const body  = document.getElementById('qz-vote-body');
  const voted = _qzVotedIds();
  body.innerHTML = `<div class="qz-vote-empty">chargement…</div>`;
  try {
    const r = await fetch(`/quizz/suggestions/${encodeURIComponent(_qzPays)}`);
    if (!r.ok) throw new Error(`HTTP ${r.status}`);
    const rows = await r.json();
    if (!rows.length) {
      body.innerHTML = `<div class="qz-vote-empty">Aucune suggestion pour ce pays.</div>`;
      return;
    }
    body.innerHTML = rows.map(s => {
      const hasVoted = voted.has(s.id);
      return `<div class="qz-vote-card">
        <div class="qz-vote-q">${esc(s.question)}</div>
        <div class="qz-vote-right">
          <span class="qz-vote-count">${s.votes} vote${s.votes !== 1 ? 's' : ''}</span>
          <button class="qz-vote-btn${hasVoted ? ' voted' : ''}"
                  ${hasVoted ? 'disabled' : `onclick="quizzVoter(${s.id},this)"`}>
            ${hasVoted ? '✓ voté' : '👍 voter'}
          </button>
        </div>
      </div>`;
    }).join('');
  } catch (e) {
    body.innerHTML = `<div class="qz-vote-empty" style="color:var(--red)">${esc(errToStr(e))}</div>`;
  }
}

async function quizzVoter(id, btn) {
  btn.disabled = true;
  try {
    const r = await fetch(`/quizz/vote/${id}`, {
      method:  'POST',
      headers: { 'Content-Type': 'application/json' },
      body:    JSON.stringify({ voterId: _qzGetVoterId() }),
    });
    if (!r.ok) throw new Error(await r.text() || `HTTP ${r.status}`);
    const votes = await r.json();
    _qzMarkVoted(id);
    btn.textContent = '✓ voté';
    btn.classList.add('voted');
    const countEl = btn.closest('.qz-vote-card')?.querySelector('.qz-vote-count');
    if (countEl) countEl.textContent = `${votes} vote${votes !== 1 ? 's' : ''}`;
  } catch (e) {
    btn.disabled = false;
    btn.textContent = `⚠ ${errToStr(e)}`;
  }
}

window.quizzToggleLb          = quizzToggleLb;
window.quizzSoumettrScore     = quizzSoumettrScore;
window.quizzToggleSugg        = quizzToggleSugg;
window.quizzEnvoyerSuggestion = quizzEnvoyerSuggestion;
window.quizzToggleVote        = quizzToggleVote;
window.quizzVoter             = quizzVoter;


// ── Meliinda ─────────────────────────────────────────────────────────────────
let _mlInited = false;
let _mlReplay = null;

function meliindaInit() {
  if (_mlInited) { mlLoadLibrary(); return; }
  _mlInited = true;

  let mlEvents    = [];
  let mlStartTime = null;
  let mlTimers    = [];

  const editor      = document.getElementById('ml-editor');
  const btnSave     = document.getElementById('ml-btn-save');
  const btnReplay   = document.getElementById('ml-btn-replay');
  const btnNew      = document.getElementById('ml-btn-new');
  const btnStop     = document.getElementById('ml-btn-stop');
  const labelInp    = document.getElementById('ml-label-input');
  const statusEl    = document.getElementById('ml-status');
  const replayWrap  = document.getElementById('ml-replay-wrap');
  const replayStage = document.getElementById('ml-replay-stage');

  // ── Capture ────────────────────────────────────────────────────────────────
  editor.addEventListener('keydown', e => {
    if (mlStartTime === null) mlStartTime = performance.now();
    mlEvents.push({ key: e.key, t: performance.now() - mlStartTime, type: 'down' });
    mlUpdateStats();
    btnSave.disabled = btnReplay.disabled = false;
  });

  editor.addEventListener('keyup', e => {
    if (mlStartTime === null) return;
    mlEvents.push({ key: e.key, t: performance.now() - mlStartTime, type: 'up' });
  });

  function mlUpdateStats() {
    const downs = mlEvents.filter(e => e.type === 'down');
    const backs = downs.filter(e => e.key === 'Backspace').length;
    const dur   = mlStartTime ? ((performance.now() - mlStartTime) / 1000).toFixed(1) : 0;
    let pauses  = 0, prev = null;
    for (const e of downs) { if (prev !== null && (e.t - prev) > 1000) pauses++; prev = e.t; }
    document.getElementById('ml-stat-keys').textContent   = downs.length;
    document.getElementById('ml-stat-back').textContent   = backs;
    document.getElementById('ml-stat-dur').textContent    = dur + 's';
    document.getElementById('ml-stat-pauses').textContent = pauses;
  }

  // ── Sauvegarde ────────────────────────────────────────────────────────────
  btnSave.addEventListener('click', async () => {
    mlSetStatus('Sauvegarde en cours…', '');
    try {
      const res = await fetch('/api/meliinda/record', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ label: labelInp.value || null, events: mlEvents }),
      });
      if (!res.ok) throw new Error(await res.text());
      const { id } = await res.json();
      mlSetStatus(`Sauvegardé · id : ${id}`, 'ok');
      mlLoadLibrary();
    } catch (err) { mlSetStatus(`Erreur : ${err.message}`, 'err'); }
  });

  // ── Nouvelle session ───────────────────────────────────────────────────────
  btnNew.addEventListener('click', () => {
    mlStopReplay();
    mlEvents = []; mlStartTime = null;
    editor.value = ''; labelInp.value = '';
    btnSave.disabled = btnReplay.disabled = true;
    mlUpdateStats();
    mlSetStatus('', '');
    editor.focus();
  });

  // ── Replay ────────────────────────────────────────────────────────────────
  _mlReplay = mlStartReplay;
  document.getElementById('ml-btn-replay').addEventListener('click', () => mlStartReplay(mlEvents));
  btnStop.addEventListener('click', mlStopReplay);

  function mlBuildStates(evts) {
    const downs = evts.filter(e => e.type === 'down');
    let buffer = [], states = [];
    for (const ev of downs) {
      if (ev.key === 'Backspace') {
        for (let i = buffer.length - 1; i >= 0; i--) {
          // On horodate l'effacement : le caractère barré disparaîtra 3 s plus tard.
          if (!buffer[i].deleted) { buffer[i].deleted = true; buffer[i].deletedAt = ev.t; break; }
        }
      } else if (ev.key.length === 1 || ev.key === 'Enter') {
        buffer.push({ char: ev.key === 'Enter' ? '\n' : ev.key, deleted: false });
      }
      states.push({ t: ev.t, snapshot: buffer.map(c => ({ ...c })) });
    }
    return states;
  }

  function mlStartReplay(evts) {
    mlStopReplay();
    replayWrap.style.display = 'block';
    replayStage.innerHTML = '<span class="ml-cursor"></span>';
    const states = mlBuildStates(evts);
    // Instants de rendu = chaque frappe + chaque échéance de disparition d'un ghost
    // (effacement + ML_GHOST_MS). Ce rendu supplémentaire fait disparaître le caractère
    // barré même si aucune frappe n'a lieu à ce moment-là (ex. dernière touche du replay).
    const times = new Set(states.map(s => s.t));
    const last = states[states.length - 1];
    if (last) for (const c of last.snapshot) if (c.deleted) times.add(c.deletedAt + ML_GHOST_MS);
    for (const t of [...times].sort((a, b) => a - b)) {
      // Base = état de la dernière frappe à ou avant t ; les ghosts expirés sont filtrés au rendu.
      let base = null;
      for (const s of states) { if (s.t <= t) base = s.snapshot; else break; }
      if (!base) continue;
      mlTimers.push(setTimeout(() => mlRenderSnapshot(base, replayStage, t), t));
    }
  }

  function mlStopReplay() {
    mlTimers.forEach(clearTimeout); mlTimers = [];
    replayWrap.style.display = 'none';
  }

  function mlSetStatus(msg, cls) { statusEl.textContent = msg; statusEl.className = cls; }

  mlLoadLibrary();
}

// Durée d'affichage d'un caractère effacé (rouge barré) avant sa disparition.
const ML_GHOST_MS = 3000;

function mlRenderSnapshot(snapshot, target, now) {
  let parts = [], run = null;
  for (const c of snapshot) {
    // Un caractère effacé disparaît ML_GHOST_MS après son effacement.
    if (c.deleted && now != null && now - c.deletedAt >= ML_GHOST_MS) continue;
    if (!run || run.deleted !== c.deleted) { run = { deleted: c.deleted, chars: [] }; parts.push(run); }
    run.chars.push(c.char === '\n' ? '↵\n' : c.char);
  }
  target.innerHTML = parts.map(p => {
    const txt = p.chars.join('').replace(/&/g,'&amp;').replace(/</g,'&lt;');
    return p.deleted ? `<span class="ml-ghost">${txt}</span>` : txt;
  }).join('') + '<span class="ml-cursor"></span>';
}

async function mlLoadLibrary() {
  const list = document.getElementById('ml-seq-list');
  if (!list) return;
  try {
    const res  = await fetch('/api/meliinda/sequences');
    const data = await res.json();
    if (!data.length) {
      list.innerHTML = '<span style="font-size:0.65rem;color:var(--dim)">Aucune séquence enregistrée.</span>';
      return;
    }
    list.innerHTML = data.map(s => `
      <div class="ml-seq-item" data-id="${s.id}">
        <span class="ml-seq-lbl">${s.label || '(sans label)'}</span>
        <span class="ml-seq-id">${s.id.slice(0,8)}…</span>
        <span class="ml-seq-date">${new Date(s.created_at).toLocaleString('fr-FR')}</span>
        <button class="ml-btn ml-danger ml-del" data-id="${s.id}" style="padding:0.25rem 0.6rem;font-size:0.58rem">✕</button>
      </div>`).join('');
    list.querySelectorAll('.ml-seq-item').forEach(el => {
      el.addEventListener('click', ev => {
        if (ev.target.classList.contains('ml-del')) return;
        mlLoadAndReplay(el.dataset.id);
      });
    });
    list.querySelectorAll('.ml-del').forEach(btn =>
      btn.addEventListener('click', async ev => {
        ev.stopPropagation();
        if (!confirm('Supprimer cette séquence ?')) return;
        await fetch(`/api/meliinda/sequence/${btn.dataset.id}`, { method: 'DELETE' });
        mlLoadLibrary();
      })
    );
  } catch { list.innerHTML = '<span style="font-size:0.65rem;color:var(--red)">Erreur de chargement.</span>'; }
}

async function mlLoadAndReplay(id) {
  const statusEl = document.getElementById('ml-status');
  statusEl.textContent = 'Chargement…'; statusEl.className = '';
  try {
    const res = await fetch(`/api/meliinda/sequence/${id}`);
    if (!res.ok) throw new Error(await res.text());
    const seq = await res.json();
    statusEl.textContent = `Replay · ${seq.label || id.slice(0,8)}`; statusEl.className = 'ok';
    if (_mlReplay) _mlReplay(seq.events);
  } catch (err) { statusEl.textContent = `Erreur : ${err.message}`; statusEl.className = 'err'; }
}
