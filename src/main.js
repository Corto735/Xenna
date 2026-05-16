import { STATIC_DICT } from './lang.js';

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

const DATE_MAX = '2026-01-31';
const TODAY    = DATE_MAX;   // alias pour les appels existants
document.addEventListener("DOMContentLoaded", () => {
  ["d-date", "m-date"].forEach(id => {
    const el = document.getElementById(id);
    if (el) { el.value = DATE_MAX; el.max = DATE_MAX; }
  });
  document.addEventListener("keydown", e => { if (e.key === "Escape") closeFmModal(); });

  // Tirage unique à l'arrivée — héros + genre initial (H : 49 %, F : 51 %)
  window._heroH = Math.random() < 0.015 ? { prenom: 'Jean-Noël', nom: 'Favari' } : _heroRandom(HEROS_H);
  window._heroF = _heroRandom(HEROS_F);

  if (Math.random() < 0.51) {
    // Préselection F — écart tiré une fois, mémorisé pour toute la session
    _ecartActif = _drawEcartPct();
    _ecartTire  = true;
    _genre      = 'F';
    _setNomFields(window._heroF.prenom, window._heroF.nom);
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
    _setNomFields(window._heroH.prenom, window._heroH.nom);
    _syncToggleUI('H');
  }

  // Déverrouillage JNF si tiré à l'arrivée
  _checkJNF();

  // Quand l'utilisateur tape manuellement, le toggle est désactivé + check JNF
  ['d-prenom', 'm-prenom', 'd-nom', 'm-nom'].forEach(id => {
    document.getElementById(id)?.addEventListener('input', () => { _nomPersonnalise = true; _checkJNF(); });
  });

  // Les hints sont initialisés au premier basculement (pays inconnu à l'init)

  // Détection automatique mobile / bureau — breakpoint identique au media query CSS
  const mq = window.matchMedia("(max-width: 680px)");
  const applyView = e => {
    const body = document.body;
    if (!body.classList.contains("is-annuel")     &&
        !body.classList.contains("is-forge")      &&
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

  // Dictionnaire statique — pré-remplit le cache sans appel réseau
  const staticLang = STATIC_DICT[lang] || {};
  texts.forEach(orig => {
    const trimmed = orig.trim();
    if (staticLang[trimmed] !== undefined && !cache.has(orig)) {
      cache.set(orig, staticLang[trimmed]);
    }
  });

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
  ['mobile', 'desktop', 'annuel', 'forge', 'apropos', 'contact', 'gaabrielle', 'hercule', 'quizz', 'mecenat', 'meliinda'].forEach(name =>
    document.body.classList.toggle('is-' + name, v === name)
  );
  document.getElementById("btn-desk").classList.toggle("active", v === "desktop");
  document.getElementById("btn-mob") .classList.toggle("active", v === "mobile");
  document.getElementById("btn-ann") .classList.toggle("active", v === "annuel");
  if (lastBulletin && (v === 'desktop' || v === 'mobile')) renderAll(lastBulletin);
  if (v === 'forge')       forgeInit();
  if (v === 'quizz')      quizzInit();
  if (v === 'gaabrielle') gaabInit();
  if (v === 'hercule')    herculeInit();
  if (v === 'apropos')    _mecenatStart();
  if (v === 'meliinda')   meliindaInit();
};

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
function fmt(val) {
  const n = parseFloat(val);
  const sym = DEVISE === "CHF" ? " CHF" : " €";
  return n.toLocaleString("fr-FR", { minimumFractionDigits: 2, maximumFractionDigits: 2 }) + sym;
}
function fmtS(val, sign = false) {
  const n = parseFloat(val);
  const sym = DEVISE === "CHF" ? " CHF" : " €";
  const s = n.toLocaleString("fr-FR", { minimumFractionDigits: 2, maximumFractionDigits: 2 }) + sym;
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
  const skipPas  = ['suisse', 'luxembourg', 'italia', 'espagne', 'portugal', 'belgique', 'allemagne', 'canada', 'quebec'].includes(b.salarie?.pays);
  const isItalie = b.salarie?.pays === 'italia';
  const totalSal = cots.reduce((s, c) => s + parseFloat(c.montant_sal), 0);
  const totalPat = cots.reduce((s, c) => s + parseFloat(c.montant_pat), 0);
  const pas      = skipPas ? { total: 0, taux_effectif: 0 } : calculerPas(b.net_imposable);
  const netPayer = parseFloat(b.net_a_payer) - pas.total;
  if (!skipPas) _fmStore['PAS'] = { type: 'pas', netImposable: parseFloat(b.net_imposable) };

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
            <span style="color:var(--red)">− ${fmt(isItalie ? totalSalCotSeules : totalSalSansIS)}</span>
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
          ${!skipPas ? `<div class="sb-ded-row">
            <span>PAS (${(pas.taux_effectif * 100).toFixed(1)} %)</span>
            <span class="fm-val" style="color:var(--purple);cursor:pointer" onclick="showFormula('PAS')">− ${fmt(pas.total)}${buildFormulaStar('PAS')}</span>
          </div>` : ''}
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
  const cotAll      = cots.filter(c => c.categorie !== "Allègement" &&
    (parseFloat(c.montant_sal) > 0 || c.taux_sal !== "0" || parseFloat(c.montant_pat) > 0));
  const cotAlleg    = cots.filter(c => c.categorie === "Allègement");
  const totalPatBrut = cotAll.reduce((s, c) => s + parseFloat(c.montant_pat), 0);

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
          <td class="r">${parseFloat(c.taux_sal) > 0 ? '− ' : ''}${fmtPct(c.taux_sal)}</td>
          <td class="r ${salCls}"${hasFmSal ? ` onclick="event.stopPropagation();showFormula('${keySal}')" style="cursor:pointer"` : ''}>${hasFmSal ? '− ' : ''}${fmt(c.montant_sal)}${starSal}</td>
          <td class="r">${parseFloat(c.taux_pat) > 0 ? '− ' : ''}${fmtPct(c.taux_pat)}</td>
          <td class="r ${patCls}"${hasFmPat ? ` onclick="event.stopPropagation();showFormula('${keyPat}')" style="cursor:pointer"` : ''}>${hasFmPat ? '− ' : ''}${fmt(c.montant_pat)}${starPat}</td>
        </tr>
        <tr class="expl-row" id="expl-${idx}" style="display:none">
          <td colspan="6">
            <div class="expl-box">
              <div class="expl-txt">▸ ${esc(c.explication)}</div>
              ${c.loi_ref ? `<div class="expl-ref">§ ${esc(c.loi_ref)}</div>` : ""}
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
          <td class="r c-sal">= − ${fmt(totalSal)}</td>
          <td></td>
          <td class="r c-pat">= − ${fmt(totalPatBrut)}</td>
        </tr>
      </tbody>
    </table>`;

  const simBanner = `<div class="sim-period">
    SIMULATION AU <span class="sp-accent">${formatDate(getDatePaie())}</span>
    &nbsp;·&nbsp; PMSS en vigueur calculé depuis la base de données sans le moindre état d'âme
  </div>`;

  // Section allègements (Fillon, etc.) — montants négatifs affichés en économie
  const totalAlleg = cotAlleg.reduce((s, c) => s + parseFloat(c.montant_pat), 0); // négatif
  const tableAlleg = cotAlleg.length === 0 ? "" : `
    <div class="tbl-section-head">── ALLÈGEMENTS PATRONAUX ───────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${thead}
      <tbody>
        ${cotAlleg.map((c, i) => {
          const idx    = cotAll.length + i;
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
              <td class="r c-alleg" onclick="event.stopPropagation();showFormula('${keyAlleg}')" style="cursor:pointer">− ${fmt(montant)}${buildFormulaStar(keyAlleg)}</td>
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
          <td colspan="5">TOTAL ALLÈGEMENTS PATRONAUX</td>
          <td class="r c-alleg">− ${fmt(Math.abs(totalAlleg))}</td>
        </tr>
      </tbody>
    </table>`;

  el.innerHTML = simBanner + summaryBar + `<div class="tbl-wrap">${tableAll}${tableAlleg}</div>`;
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

  const skipPas  = ['suisse', 'luxembourg', 'italia', 'espagne', 'portugal', 'belgique', 'allemagne', 'canada', 'quebec'].includes(b.salarie?.pays);
  const isItalieMob = b.salarie?.pays === 'italia';
  const totalSal  = cots.reduce((s, c) => s + parseFloat(c.montant_sal), 0);
  const totalPat  = cots.reduce((s, c) => s + parseFloat(c.montant_pat), 0);
  const pas       = skipPas ? { total: 0, taux_effectif: 0 } : calculerPas(b.net_imposable);
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
  const cotAllMob    = cots.filter(c => c.categorie !== "Allègement" && c.code !== 'CH_IS'
    && c.code !== 'IT_IRPEF' && c.code !== 'IT_BONUS_CUNEO' &&
    (parseFloat(c.montant_sal) > 0 || c.taux_sal !== "0" || parseFloat(c.montant_pat) > 0));
  const cotAllegMob  = cots.filter(c => c.categorie === "Allègement");
  const totalPatBrutMob = cotAllMob.reduce((s, c) => s + parseFloat(c.montant_pat), 0);
  const totalAlleg      = cotAllegMob.reduce((s, c) => s + parseFloat(c.montant_pat), 0);

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
      : `<span class="mob-val c-dim">0 ${DEVISE === 'CHF' ? 'CHF' : '€'}</span>`;
    const amtsPat = hasPat
      ? `<span class="mob-val c-orange mob-cot-amt" onclick="mobToggle('${expandId}','pat')">− ${fmt(c.montant_pat)}</span>`
      : `<span class="mob-val c-dim">0 ${DEVISE === 'CHF' ? 'CHF' : '€'}</span>`;
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
    .map((c, i) => buildMobCotRow(c, `${c.code}_alleg`, `− ${fmt(Math.abs(parseFloat(c.montant_pat)))}`, 'c-alleg', 'alleg', i))
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

      <!-- Brut -->
      <div class="mob-row final-row">
        <span class="mob-lbl">Salaire de base brut</span>
        <span class="mob-val c-green">${fmt(b.brut)}</span>
      </div>

      <!-- Cotisations unifiées (salariales + patronales sur une ligne) -->
      <div class="mob-row section"><span class="mob-lbl">── COTISATIONS ──</span><span style="display:flex;gap:0.75rem"><span class="mob-badge mob-badge-sal">Sal.</span><span class="mob-badge mob-badge-pat">Pat.</span></span></div>
      ${cotLines}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL cotisations salariales</span>
        <span class="mob-val c-yellow">− ${fmt(totalSalSansIS)}</span>
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

      <!-- Net à payer -->
      <div class="mob-row final-row">
        <span class="mob-lbl">NET À PAYER</span>
        <span class="mob-val c-green">${fmt(netPayer)}</span>
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
  DEVISE = b.devise || "EUR";
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
  const isItalie       = document.getElementById(isM ? "m-italie"      : "d-italie")?.checked ?? false;
  const isEspagne      = document.getElementById(isM ? "m-espagne"     : "d-espagne")?.checked ?? false;
  const isPortugal     = document.getElementById(isM ? "m-portugal"    : "d-portugal")?.checked ?? false;
  const isBelgique     = document.getElementById(isM ? "m-belgique"    : "d-belgique")?.checked ?? false;
  const beRegion       = document.getElementById(isM ? "m-be-region"   : "d-be-region")?.value || "bruxelles";
  const isAllemagne    = document.getElementById(isM ? "m-allemagne"   : "d-allemagne")?.checked ?? false;
  const isCanada       = document.getElementById(isM ? "m-canada"      : "d-canada")?.checked ?? false;
  const isQuebec       = document.getElementById(isM ? "m-quebec"      : "d-quebec")?.checked ?? false;
  const caProvince     = document.getElementById(isM ? "m-ca-province" : "d-ca-province")?.value || "ON";
  const steuerklasse   = document.getElementById(isM ? "m-steuerklasse"    : "d-steuerklasse")?.value || "1";
  const kinderlos      = document.getElementById(isM ? "m-kinderlos"       : "d-kinderlos")?.checked ?? false;
  const kirchenmitglied= document.getElementById(isM ? "m-kirchenmitglied" : "d-kirchenmitglied")?.checked ?? false;
  const deLand         = document.getElementById(isM ? "m-land"            : "d-land")?.value || "NW";
  const assujettiIS    = document.getElementById(isM ? "m-assujetti-is" : "d-assujetti-is")?.checked ?? false;
  const canton         = document.getElementById(isM ? "m-canton"       : "d-canton")?.value || null;
  const tarifIs        = document.getElementById(isM ? "m-tarif-is"     : "d-tarif-is")?.value || null;

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
    : isBelgique ? "belgique" : isAllemagne ? "allemagne" : isCanada ? "canada" : isQuebec ? "quebec" : null;
  const datePaie = date;

  try {
    const bulletin = await api("calculer_bulletin", {
      salarie: {
        nom, prenom, salaire_brut: brut.toString(), statut,
        alsace_moselle: alsaceMoselle,
        pays: paysEtranger ?? (isFPT ? "fonction_publique" : "france"),
        assujetti_is: assujettiIS,
        canton:   (isSuisse && assujettiIS && canton)  ? canton  : null,
        tarif_is: (isSuisse && assujettiIS && tarifIs) ? tarifIs : null,
        regione: null,
        contratto_termine: false,
        province: isCanada ? caProvince : null,
        steuerklasse: isAllemagne ? parseInt(steuerklasse, 10) : null,
        kinderlos:    isAllemagne ? kinderlos : null,
        kirchenmitglied: isAllemagne ? kirchenmitglied : null,
        land:         isAllemagne ? deLand : null,
        region_be:    isBelgique ? beRegion : null,
      },
      datePaie,
    });
    lastBulletin = bulletin;
    renderAll(bulletin);
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
  const TOUS_PAYS    = ['france', 'suisse', 'luxembourg', 'fpt', 'italie', 'espagne', 'portugal', 'belgique', 'allemagne', 'canada', 'quebec'];
  const PAYS_ETR     = ['suisse', 'luxembourg', 'italie', 'espagne', 'portugal', 'belgique', 'allemagne', 'canada', 'quebec'];
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
  ['d-date', 'm-date'].forEach(id => {
    const el = document.getElementById(id);
    if (!el) return;
    el.disabled = false;
  });

  // Label devise : CHF (Suisse), CAD (Canada/Québec), EUR (autres)
  const isSuisse  = document.getElementById('d-suisse')?.checked;
  const isCA = document.getElementById('d-canada')?.checked || document.getElementById('d-quebec')?.checked;
  const labelBrut  = isSuisse ? 'SALAIRE BRUT (CHF)' : isCA ? 'SALAIRE BRUT (CAD)' : 'SALAIRE BRUT (€)';
  const labelBrutM = isSuisse ? 'BRUT (CHF)'         : isCA ? 'BRUT (CAD)'         : 'BRUT (€)';
  const dBrut = document.getElementById('d-brut');
  if (dBrut) { const l = dBrut.closest('.field')?.querySelector('label'); if (l) l.textContent = labelBrut; }
  const mBrut = document.getElementById('m-brut');
  if (mBrut) { const l = mBrut.closest('.field')?.querySelector('label'); if (l) l.textContent = labelBrutM; }

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

  ['d', 'm'].forEach(p => {
    const btn = document.getElementById(`${p}-apply-brut`);
    if (btn) btn.style.display = '';
  });
};

window.applyBrut = function(prefix) {
  const hMois = parseFloat(document.getElementById(`${prefix}-h-mois`)?.value);
  if (!hMois || hMois <= 0) return;
  const factor = hMois / 151.67;
  ['d-brut', 'm-brut'].forEach(id => {
    const el = document.getElementById(id);
    if (!el) return;
    const v = parseFloat(el.value);
    if (!isNaN(v)) el.value = Math.round(v * factor * 100) / 100;
  });
  ['d', 'm'].forEach(p => {
    const btn = document.getElementById(`${p}-apply-brut`);
    if (btn) btn.style.display = 'none';
  });
};

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
};

function _getActivePays() {
  for (const p of ['suisse', 'luxembourg', 'italie', 'espagne', 'portugal', 'belgique', 'allemagne', 'canada', 'quebec']) {
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
  if (info) info.innerHTML = `Bulletin de <strong>${b.salarie.prenom} ${b.salarie.nom}</strong> · Brut ${fmtE(brut)} · ${b.salarie.alsace_moselle ? 'Alsace-Moselle · ' : ''}${b.salarie.statut === 'cadre' ? 'Cadre' : 'Non-cadre'}`;

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
const GAAB_EMPLOYES = [
  // ── H ──
  { mat:'XN-001', nom:'de Riv',        prenom:'Geralt',   embauche:'2016-04-11', ageEmb:{a:38,m:2, j:7 }, poste:'Chasseur de Bugs Mutants — Résilience Maximale',                 bh:44.00, etp:100 },
  // ── F ──
  { mat:'XN-002', nom:'Belacqua',      prenom:'Lyra',     embauche:'2023-09-01', ageEmb:{a:22,m:0, j:14}, poste:'Cartographe des Mondes Parallèles Contractuels',                bh:16.50, etp:80  },
  // ── H ──
  { mat:'XN-003', nom:'Vimes',         prenom:'Sam',      embauche:'2017-02-20', ageEmb:{a:45,m:6, j:3 }, poste:'Commissaire aux Anomalies Comportementales',                    bh:38.50, etp:100 },
  // ── F ──
  { mat:'XN-004', nom:'Granger',       prenom:'Hermione', embauche:'2020-09-01', ageEmb:{a:28,m:0, j:0 }, poste:'Directrice des Procédures Magiques et de la Conformité',        bh:31.00, etp:100 },
  // ── H ──
  { mat:'XN-005', nom:'de Melniboné',  prenom:'Elric',    embauche:'2018-06-15', ageEmb:{a:33,m:3, j:19}, poste:'Canalisateur des Flux d\'Entropie Numérique',                   bh:51.00, etp:100 },
  // ── F ──
  { mat:'XN-006', nom:'du Rohan',      prenom:'Eowyn',    embauche:'2019-11-03', ageEmb:{a:31,m:7, j:22}, poste:'Cheffe de Projet Déconstruction des Obstacles Genrés',          bh:24.50, etp:100 },
  // ── H ──
  { mat:'XN-007', nom:'la Légende',    prenom:'Druss',    embauche:'2014-08-30', ageEmb:{a:52,m:1, j:8 }, poste:'Directeur des Opérations Physiques Irréversibles',              bh:48.00, etp:100 },
  // ── F ──
  { mat:'XN-008', nom:'Caldin',        prenom:'Ellana',   embauche:'2021-03-08', ageEmb:{a:26,m:9, j:15}, poste:'Analyste en Trajectoires Inconventionnelles',                   bh:19.75, etp:80  },
  // ── H ──
  { mat:'XN-009', nom:'Neuf-Doigts',   prenom:'Logen',    embauche:'2020-01-13', ageEmb:{a:37,m:4, j:27}, poste:'Expert en Gestion de Crises Légèrement Incontrôlables',         bh:29.00, etp:100 },
  // ── F ──
  { mat:'XN-010', nom:'Maljinn',       prenom:'Ferro',    embauche:'2022-05-16', ageEmb:{a:29,m:11,j:4 }, poste:'Responsable de la Désintégration des Processus Obsolètes',      bh:21.50, etp:100 },
  // ── H ──
  { mat:'XN-011', nom:'Grands-Pas',    prenom:'Aragorn',  embauche:'2015-07-22', ageEmb:{a:41,m:0, j:11}, poste:'Directeur Général des Transitions de Paradigme',                bh:58.00, etp:100 },
  // ── F ──
  { mat:'XN-012', nom:'Garlick',       prenom:'Magrat',   embauche:'2018-04-01', ageEmb:{a:34,m:5, j:17}, poste:'Consultante en Phytothérapie Algorithmique',                    bh:20.00, etp:80  },
  // ── H ──
  { mat:'XN-013', nom:'Shannow',       prenom:'Jon',      embauche:'2019-06-09', ageEmb:{a:44,m:8, j:2 }, poste:'Pisteur de Tendances Post-Apocalyptiques',                      bh:33.75, etp:100 },
  // ── F ──
  { mat:'XN-014', nom:"Gil'Sayan",     prenom:'Ewilan',   embauche:'2024-02-19', ageEmb:{a:21,m:3, j:6 }, poste:'Ingénieure en Dessin de Réalités Augmentées',                   bh:15.50, etp:80  },
  // ── H ──
  { mat:'XN-015', nom:'Dhibi',         prenom:'Salim',    embauche:'2021-10-04', ageEmb:{a:30,m:2, j:29}, poste:'Archiviste des Compétences Émergentes Non-Homologuées',         bh:26.50, etp:100 },
  // ── F ──
  { mat:'XN-016', nom:'la Guerrière',  prenom:'Sigarni',  embauche:'2013-11-18', ageEmb:{a:48,m:6, j:0 }, poste:'Directrice des Restructurations Stratégiques',                  bh:38.00, etp:100 },
  // ── H ──
  { mat:'XN-017', nom:'le Magi',       prenom:'Bayaz',    embauche:'2011-03-01', ageEmb:{a:61,m:0, j:0 }, poste:'Conseiller Exécutif en Manipulation des Lois Fondamentales',    bh:62.00, etp:100 },
  // ── F ──
  { mat:'XN-018', nom:'la Nord',       prenom:'Rikke',    embauche:'2022-08-22', ageEmb:{a:27,m:5, j:9 }, poste:'Prévisionniste des Déviations Sociales Involontaires',          bh:22.25, etp:100 },
  // ── H ──
  { mat:'XN-019', nom:"l'Enchanteur",  prenom:'Merlin',   embauche:'2009-05-12', ageEmb:{a:67,m:3, j:21}, poste:'Président du Conseil des Visions à Long Terme',                 bh:55.00, etp:100 },
  // ── F ──
  { mat:'XN-020', nom:'la Magicienne', prenom:'Tanaquil', embauche:'2017-09-25', ageEmb:{a:36,m:1, j:13}, poste:'Architecte des Sortilèges Organisationnels',                    bh:27.50, etp:80  },
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
      const { a, m, j } = e.ageEmb;
      const age    = `${a} a ${m} m ${j} j`;
      const etpCls = e.etp < 100 ? 'style="color:var(--yellow)"' : 'style="color:var(--dim)"';
      return `<tr>
        <td class="gaab-mat">${e.mat}</td>
        <td>${e.nom}</td>
        <td>${e.prenom}</td>
        <td>${e.embauche}</td>
        <td class="gaab-age">${age}</td>
        <td class="gaab-poste">${e.poste}</td>
        <td class="gaab-etp" ${etpCls}>${e.etp} %</td>
        <td class="gaab-sal" data-bh="${e.bh}">${_gaabSalStr(e.bh, 'bh')}</td>
      </tr>`;
    }).join('');
  }
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
];

const QZ_SESSION_KEY = 'xenna-qz-pending';
const QZ_START_LABELS = {
  fr: 'Démarrer', fpt: 'Démarrer', ch: 'Starten',
  lu: 'Ufänken',  it: 'Inizia',    ca: 'Start', qc: 'Commencer',
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
  const btnExport   = document.getElementById('ml-btn-export');
  const labelInp    = document.getElementById('ml-label-input');
  const statusEl    = document.getElementById('ml-status');
  const replayWrap  = document.getElementById('ml-replay-wrap');
  const replayStage = document.getElementById('ml-replay-stage');

  // ── Capture ────────────────────────────────────────────────────────────────
  editor.addEventListener('keydown', e => {
    if (mlStartTime === null) mlStartTime = performance.now();
    mlEvents.push({ key: e.key, t: performance.now() - mlStartTime, type: 'down' });
    mlUpdateStats();
    btnSave.disabled = btnReplay.disabled = btnExport.disabled = false;
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
    btnSave.disabled = btnReplay.disabled = btnExport.disabled = true;
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
          if (!buffer[i].deleted) { buffer[i].deleted = true; break; }
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
    for (const { t, snapshot } of states) {
      mlTimers.push(setTimeout(() => mlRenderSnapshot(snapshot, replayStage), t));
    }
  }

  function mlStopReplay() {
    mlTimers.forEach(clearTimeout); mlTimers = [];
    replayWrap.style.display = 'none';
  }

  // ── Export vidéo (canvas + MediaRecorder → .webm) ─────────────────────────
  btnExport.addEventListener('click', () => mlExportVideo(mlEvents, labelInp.value));

  function mlSetStatus(msg, cls) { statusEl.textContent = msg; statusEl.className = cls; }

  mlLoadLibrary();
}

function mlRenderSnapshot(snapshot, target) {
  let parts = [], run = null;
  for (const c of snapshot) {
    if (!run || run.deleted !== c.deleted) { run = { deleted: c.deleted, chars: [] }; parts.push(run); }
    run.chars.push(c.char === '\n' ? '↵\n' : c.char);
  }
  target.innerHTML = parts.map(p => {
    const txt = p.chars.join('').replace(/&/g,'&amp;').replace(/</g,'&lt;');
    return p.deleted ? `<span class="ml-ghost">${txt}</span>` : txt;
  }).join('') + '<span class="ml-cursor"></span>';
}

function mlExportVideo(evts, label) {
  if (!evts.length) return;
  const statusEl = document.getElementById('ml-status');
  const downs    = evts.filter(e => e.type === 'down');
  const duration = downs[downs.length - 1].t + 800;

  const W = 720, H = 320, PAD = 24, FONT = 15;
  const canvas = document.createElement('canvas');
  canvas.width = W; canvas.height = H;
  const ctx = canvas.getContext('2d');

  const stream   = canvas.captureStream(30);
  const recorder = new MediaRecorder(stream, { mimeType: 'video/webm;codecs=vp9' });
  const chunks   = [];
  recorder.ondataavailable = e => { if (e.data.size) chunks.push(e.data); };
  recorder.onstop = () => {
    const blob = new Blob(chunks, { type: 'video/webm' });
    const url  = URL.createObjectURL(blob);
    const a    = document.createElement('a');
    a.href = url;
    a.download = (label || 'meliinda') + '.webm';
    a.click();
    URL.revokeObjectURL(url);
    statusEl.textContent = 'Vidéo exportée.'; statusEl.className = 'ok';
  };

  // Construit les états et dessine frame par frame
  let buffer = [], frameTimers = [];

  function drawState(snapshot, cursorOn) {
    ctx.fillStyle = '#141417';
    ctx.fillRect(0, 0, W, H);

    // label en haut
    ctx.font = `10px monospace`;
    ctx.fillStyle = '#555566';
    ctx.fillText('// Meliinda · empreinte de frappe', PAD, PAD);

    ctx.font = `${FONT}px monospace`;
    let x = PAD, y = PAD + 28;
    for (const c of snapshot) {
      const ch = c.char === '\n' ? '' : c.char;
      ctx.fillStyle = c.deleted ? '#e05c5c' : '#eeeef2';
      if (c.deleted) {
        ctx.fillText(ch, x, y);
        ctx.fillStyle = '#e05c5c';
        ctx.fillRect(x, y - FONT * 0.6, ctx.measureText(ch).width, 1);
      } else {
        ctx.fillText(ch, x, y);
      }
      x += ctx.measureText(ch || ' ').width;
      if (c.char === '\n' || x > W - PAD * 2) { x = PAD; y += FONT + 6; }
    }
    // curseur
    if (cursorOn) {
      ctx.fillStyle = '#5b8dee';
      ctx.fillRect(x, y - FONT, 2, FONT + 2);
    }
  }

  // Blink cursor
  let cursorOn = true;
  const blinkTimer = setInterval(() => { cursorOn = !cursorOn; }, 450);

  // Planifie chaque frame
  for (const ev of downs) {
    if (ev.key === 'Backspace') {
      for (let i = buffer.length - 1; i >= 0; i--) {
        if (!buffer[i].deleted) { buffer[i].deleted = true; break; }
      }
    } else if (ev.key.length === 1 || ev.key === 'Enter') {
      buffer.push({ char: ev.key === 'Enter' ? '\n' : ev.key, deleted: false });
    }
    const snap = buffer.map(c => ({ ...c }));
    frameTimers.push(setTimeout(() => drawState(snap, cursorOn), ev.t));
  }

  recorder.start();
  statusEl.textContent = 'Export vidéo en cours…'; statusEl.className = '';

  setTimeout(() => {
    frameTimers.forEach(clearTimeout);
    clearInterval(blinkTimer);
    recorder.stop();
  }, duration);
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
