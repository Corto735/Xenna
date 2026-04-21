import { invoke } from "@tauri-apps/api/core";

// ── État global ──────────────────────────────────────────────────────────────
let lastBulletin = null;

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
  document.body.classList.toggle("is-mobile", v === "mobile");
  document.body.classList.toggle("is-desktop", v === "desktop");
  document.getElementById("btn-desk").classList.toggle("active", v === "desktop");
  document.getElementById("btn-mob").classList.toggle("active",  v === "mobile");
  if (lastBulletin) renderAll(lastBulletin);
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
function getPasTaux() {
  const src = document.body.classList.contains("is-mobile") ? "m-pas" : "d-pas";
  const other = src === "m-pas" ? "d-pas" : "m-pas";
  const v = parseFloat(document.getElementById(src)?.value || document.getElementById(other)?.value || 0);
  return isNaN(v) ? 0 : v / 100;
}

// ── Catégorie → classe CSS ────────────────────────────────────────────────────
const CAT_CLASS = {
  "Sécurité Sociale":      "cat-ss",
  "CSG/CRDS":              "cat-csg",
  "Retraite complémentaire":"cat-ret",
  "Prévoyance":            "cat-prev",
  "Chômage":               "cat-cho",
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
  const tauxPas  = getPasTaux();
  const montPas  = parseFloat(b.net_imposable) * tauxPas;
  const netPayer = parseFloat(b.net_a_payer) - montPas;

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
        <div class="sb-lbl">▸ PAS (${(tauxPas * 100).toFixed(1)} %)</div>
        <div class="sb-val c-red">− ${fmt(montPas)}</div>
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
  const cotSal = cots.filter(c => parseFloat(c.montant_sal) > 0 || c.taux_sal !== "0");
  const cotPat = cots.filter(c => parseFloat(c.montant_pat) > 0);

  function buildRows(list, offset) {
    return list.map((c, i) => {
      const idx = offset + i;
      const catCls = CAT_CLASS[c.categorie] || "cat-ss";
      const salCls = parseFloat(c.montant_sal) > 0 ? "c-sal" : "c-dim";
      const patCls = parseFloat(c.montant_pat) > 0 ? "c-pat" : "c-dim";
      return `
        <tr class="data-row" id="row-${idx}" onclick="toggleExpl(${idx})">
          <td>
            <span class="expand-icon">▶</span>
            <span class="cat ${catCls}">[${c.categorie}]</span>
            <span>${c.libelle}</span>
          </td>
          <td class="r">${fmt(c.base)}</td>
          <td class="r">${fmtPct(c.taux_sal)}</td>
          <td class="r ${salCls}">${fmt(c.montant_sal)}</td>
          <td class="r">${fmtPct(c.taux_pat)}</td>
          <td class="r ${patCls}">${fmt(c.montant_pat)}</td>
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
          <td class="r c-pat">+ ${fmt(totalPat)}</td>
        </tr>
      </tbody>
    </table>`;

  el.innerHTML = summaryBar + `<div class="tbl-wrap">${tableSal}${tablePat}</div>`;
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
  const tauxPas   = getPasTaux();
  const montPas   = parseFloat(b.net_imposable) * tauxPas;
  const netPayer  = parseFloat(b.net_a_payer) - montPas;
  const superBrut = parseFloat(b.brut) + totalPat;

  const cotSalLines = cots
    .filter(c => parseFloat(c.montant_sal) > 0)
    .map(c => `
      <div class="mob-row">
        <span class="mob-lbl">${c.libelle}</span>
        <span class="mob-val c-red">− ${fmt(c.montant_sal)}</span>
      </div>`).join("");

  const cotPatLines = cots
    .filter(c => parseFloat(c.montant_pat) > 0)
    .map(c => `
      <div class="mob-row">
        <span class="mob-lbl">${c.libelle}</span>
        <span class="mob-val c-orange">+ ${fmt(c.montant_pat)}</span>
      </div>`).join("");

  el.innerHTML = `
    <div class="mob-bulletin">

      <!-- En-tête bulletin -->
      <div class="mob-head">
        <span class="mob-head-title">BULLETIN DE PAIE</span>
        <span class="mob-head-name">${esc(prn)} ${esc(nom).toUpperCase()}</span>
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
        <span class="mob-lbl">Prélèvement à la source (${(tauxPas * 100).toFixed(1)} %)</span>
        <span class="mob-val c-purple">− ${fmt(montPas)}</span>
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
        <span class="mob-lbl">TOTAL charges patronales</span>
        <span class="mob-val c-orange">+ ${fmt(totalPat)}</span>
      </div>

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

// ═════════════════════════════════════════════════════════════════════════════
// CALCUL
// ═════════════════════════════════════════════════════════════════════════════
async function calculate(source) {
  const isM = source === "mobile";
  const brut    = document.getElementById(isM ? "m-brut"   : "d-brut").value;
  const statut  = document.getElementById(isM ? "m-statut" : "d-statut").value;
  const nom     = document.getElementById(isM ? "m-nom"    : "d-nom").value   || "Dupont";
  const prenom  = document.getElementById(isM ? "m-prenom" : "d-prenom").value || "Marie";

  // Sync les deux formulaires
  ["d-brut","m-brut"].forEach(id => { const e = document.getElementById(id); if(e) e.value = brut; });
  ["d-statut","m-statut"].forEach(id => { const e = document.getElementById(id); if(e) e.value = statut; });
  ["d-nom","m-nom"].forEach(id => { const e = document.getElementById(id); if(e) e.value = nom; });
  ["d-prenom","m-prenom"].forEach(id => { const e = document.getElementById(id); if(e) e.value = prenom; });

  try {
    const bulletin = await invoke("calculer_bulletin", {
      salarie: { nom, prenom, salaire_brut: brut.toString(), statut },
    });
    lastBulletin = bulletin;
    renderAll(bulletin);
  } catch (e) {
    const errHtml = `<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">ERREUR : ${esc(String(e))}</div>`;
    document.getElementById("res-desktop").innerHTML = errHtml;
    document.getElementById("res-mobile").innerHTML  = errHtml;
  }
}

// ── Listeners ────────────────────────────────────────────────────────────────
document.getElementById("d-calc").addEventListener("click", () => calculate("desktop"));
document.getElementById("m-calc").addEventListener("click", () => calculate("mobile"));

// Sync taux PAS entre les deux vues
document.getElementById("d-pas").addEventListener("input", e => {
  const m = document.getElementById("m-pas"); if(m) m.value = e.target.value;
  if (lastBulletin) renderAll(lastBulletin);
});
document.getElementById("m-pas").addEventListener("input", e => {
  const d = document.getElementById("d-pas"); if(d) d.value = e.target.value;
  if (lastBulletin) renderAll(lastBulletin);
});
