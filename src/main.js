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
});

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
  document.body.classList.toggle("is-mobile",  v === "mobile");
  document.body.classList.toggle("is-desktop", v === "desktop");
  document.body.classList.toggle("is-annuel",  v === "annuel");
  document.getElementById("btn-desk").classList.toggle("active", v === "desktop");
  document.getElementById("btn-mob") .classList.toggle("active", v === "mobile");
  document.getElementById("btn-ann") .classList.toggle("active", v === "annuel");
  if (lastBulletin && v !== "annuel") renderAll(lastBulletin);
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

function buildFormulaStar(key) {
  return `<span class="formula-star" onclick="event.stopPropagation();showFormula('${key}')">*</span>`;
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

  if (entry.type === 'pas') {
    document.getElementById('fm-title').textContent = 'Prélèvement à la Source (PAS)';
    document.getElementById('fm-badge').textContent = '── Détail par tranche — barème neutre mensuel DGFIP ─────────';
    document.getElementById('fm-body').innerHTML = buildPasFormulaContent(entry.netImposable);
    document.getElementById('fm-modal').classList.add('open');
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
  document.getElementById('fm-body').innerHTML = buildFormulaContent(c, type);
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
        <div class="sb-lbl">▸ NET IMPOSABLE</div>
        <div class="sb-val c-green">${fmt(b.net_imposable)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ PAS (${(pas.taux_effectif * 100).toFixed(1)} %)</div>
        <div class="sb-val c-red">− ${fmt(pas.total)}${buildFormulaStar('PAS')}</div>
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
      _fmStore[keySal] = { c, type: 'sal' };
      _fmStore[keyPat] = { c, type: 'pat' };
      const starSal = parseFloat(c.montant_sal) > 0 ? buildFormulaStar(keySal) : '';
      const starPat = parseFloat(c.montant_pat) > 0 ? buildFormulaStar(keyPat) : '';

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
    .map(c => `
      <div class="mob-row">
        <span class="mob-lbl">${c.libelle}</span>
        <span class="mob-val c-red">− ${fmt(c.montant_sal)}</span>
      </div>`).join("");

  const cotPatFiltered = cots.filter(c => parseFloat(c.montant_pat) > 0);
  const cotPatLines = cotPatFiltered
    .map(c => `
      <div class="mob-row">
        <span class="mob-lbl">${c.libelle}</span>
        <span class="mob-val c-orange">+ ${fmt(c.montant_pat)}</span>
      </div>`).join("");
  const totalPatBrutMob = cotPatFiltered.reduce((s, c) => s + parseFloat(c.montant_pat), 0);

  const cotAllegLines = cots
    .filter(c => c.categorie === "Allègement")
    .map(c => `
      <div class="mob-row">
        <span class="mob-lbl">${c.libelle}</span>
        <span class="mob-val c-alleg">− ${fmt(Math.abs(parseFloat(c.montant_pat)))}</span>
      </div>`).join("");

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
  const brut    = document.getElementById(isM ? "m-brut"   : "d-brut").value;
  const statut  = document.getElementById(isM ? "m-statut" : "d-statut").value;
  const nom     = document.getElementById(isM ? "m-nom"    : "d-nom").value   || "Dupont";
  const prenom  = document.getElementById(isM ? "m-prenom" : "d-prenom").value || "Marie";
  const date    = document.getElementById(isM ? "m-date"   : "d-date").value  || TODAY;

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
      salarie: { nom, prenom, salaire_brut: brut.toString(), statut },
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

// ── Listeners ────────────────────────────────────────────────────────────────
document.getElementById("d-calc").addEventListener("click", () => calculate("desktop"));
document.getElementById("m-calc").addEventListener("click", () => calculate("mobile"));
document.getElementById("a-calc").addEventListener("click", calculerAnnee);

