(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const s of document.querySelectorAll('link[rel="modulepreload"]'))i(s);new MutationObserver(s=>{for(const n of s)if(n.type==="childList")for(const a of n.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&i(a)}).observe(document,{childList:!0,subtree:!0});function l(s){const n={};return s.integrity&&(n.integrity=s.integrity),s.referrerPolicy&&(n.referrerPolicy=s.referrerPolicy),s.crossOrigin==="use-credentials"?n.credentials="include":s.crossOrigin==="anonymous"?n.credentials="omit":n.credentials="same-origin",n}function i(s){if(s.ep)return;s.ep=!0;const n=l(s);fetch(s.href,n)}})();const ve="modulepreload",ye=function(e){return"/"+e},V={},he=function(t,l,i){let s=Promise.resolve();if(l&&l.length>0){let a=function(d){return Promise.all(d.map(c=>Promise.resolve(c).then(g=>({status:"fulfilled",value:g}),g=>({status:"rejected",reason:g}))))};document.getElementsByTagName("link");const o=document.querySelector("meta[property=csp-nonce]"),m=o?.nonce||o?.getAttribute("nonce");s=a(l.map(d=>{if(d=ye(d),d in V)return;V[d]=!0;const c=d.endsWith(".css"),g=c?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${d}"]${g}`))return;const v=document.createElement("link");if(v.rel=c?"stylesheet":ve,c||(v.as="script"),v.crossOrigin="",v.href=d,m&&v.setAttribute("nonce",m),document.head.appendChild(v),c)return new Promise((h,E)=>{v.addEventListener("load",h),v.addEventListener("error",()=>E(new Error(`Unable to preload CSS for ${d}`)))})}))}function n(a){const o=new Event("vite:preloadError",{cancelable:!0});if(o.payload=a,window.dispatchEvent(o),!o.defaultPrevented)throw a}return s.then(a=>{for(const o of a||[])o.status==="rejected"&&n(o.reason);return t().catch(n)})};async function te(e,t={}){if(window.__TAURI_INTERNALS__){const{invoke:i}=await he(async()=>{const{invoke:s}=await import("./core-DV6XEvTN.js");return{invoke:s}},[]);return i(e,t)}const l=await fetch(`/api/${e}`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(t)});if(!l.ok)throw await l.text();return l.json()}function M(e){if(e==null)return"(erreur nulle — redémarre l'app ou ouvre DevTools Ctrl+Shift+I)";if(typeof e=="string")return e||"(erreur muette — ouvre DevTools Ctrl+Shift+I et consulte la Console)";if(e instanceof Error)return e.message||e.toString();try{return JSON.stringify(e,null,2)}catch{return String(e)}}let R=null;const P=new Date().toISOString().slice(0,10);document.addEventListener("DOMContentLoaded",()=>{["d-date","m-date"].forEach(n=>{const a=document.getElementById(n);a&&(a.value=P,a.max=P)}),document.addEventListener("keydown",n=>{n.key==="Escape"&&closeFmModal()}),window._heroH=ee(Fe),window._heroF=ee(Ne),de(window._heroH.prenom,window._heroH.nom),ue("H"),["d-prenom","m-prenom","d-nom","m-nom"].forEach(n=>{document.getElementById(n)?.addEventListener("input",()=>{k=!0})});const e=Math.round(C*100),t=Math.round(C/(1-C)*100);document.querySelectorAll(".genre-ecart-hint").forEach(n=>{n.dataset.textFh=`// −${e} % · écart salarial F/H`,n.dataset.textHf=`// +${t} % · écart salarial H/F`});const l=window.matchMedia("(max-width: 680px)"),i=n=>{const a=document.body;!a.classList.contains("is-annuel")&&!a.classList.contains("is-forge")&&!a.classList.contains("is-apropos")&&!a.classList.contains("is-gaabrielle")&&!a.classList.contains("is-hercule")&&setView(n.matches?"mobile":"desktop")};l.addEventListener("change",i),i(l),localStorage.getItem("xenna-hv")&&(document.body.classList.add("hv-mode"),document.getElementById("hv-switch")?.classList.add("on")),localStorage.getItem("xenna-zoom")&&(document.body.classList.add("zoom-mode"),document.documentElement.style.zoom="200%",document.getElementById("zoom-switch")?.classList.add("on"),document.getElementById("a11y-magnifier")?.classList.add("active"));const s=localStorage.getItem("xenna-font");s&&setAppFont(s,!0),localStorage.getItem("xenna-hv")&&document.getElementById("a11y-hv-btn")?.classList.add("active"),localStorage.getItem("xenna-bw")&&(document.body.classList.add("bw-mode"),document.getElementById("bw-switch")?.classList.add("on"),document.getElementById("a11y-bw-btn")?.classList.add("active")),document.addEventListener("click",n=>{!n.target.closest("#a11y-btn")&&!n.target.closest("#a11y-panel")&&(document.getElementById("a11y-panel")?.classList.remove("open"),document.getElementById("a11y-btn")?.classList.remove("open"))})});let Y="fr";const N={},L=new Map;function Ee(){const e="script,style,input,select,textarea,.mob-val,.sb-val,.fm-val,.a11y-float,.trad-panel,#a11y-panel",t=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT,{acceptNode(i){const s=i.textContent.trim();return!s||s.length<2||/^[\d\s,.\-+%€×\/:()[\]]+$/.test(s)||i.parentElement?.closest(e)?NodeFilter.FILTER_REJECT:NodeFilter.FILTER_ACCEPT}}),l=[];for(;t.nextNode();)l.push(t.currentNode);return l}window.toggleTradPanel=function(){const e=document.getElementById("trad-panel"),t=document.getElementById("trad-btn"),l=e.classList.toggle("open");t.classList.toggle("open",l)};window.translateApp=async function(e){if(document.getElementById("trad-panel")?.classList.remove("open"),document.getElementById("trad-btn")?.classList.remove("open"),document.querySelectorAll(".trad-lang-btn").forEach(a=>a.classList.remove("active")),document.querySelector(`.trad-lang-btn[onclick="translateApp('${e}')"]`)?.classList.add("active"),e==="fr"){L.forEach((a,o)=>{o.isConnected&&(o.textContent=a)}),document.documentElement.lang="fr",Y="fr";return}const t=document.getElementById("trad-btn");t.classList.add("loading"),t.textContent="🌐 …";const l=Ee();l.forEach(a=>{L.has(a)||L.set(a,a.textContent)});const i=l.map(a=>L.get(a));N[e]||(N[e]=new Map);const s=N[e],n=[...new Set(i)].filter(a=>!s.has(a));try{if(n.length>0)for(let o=0;o<n.length;o+=20){const m=n.slice(o,o+20),d=m.join(`

`),c=`https://api.mymemory.translated.net/get?q=${encodeURIComponent(d)}&langpair=fr|${e}`,g=await fetch(c);if(!g.ok)throw new Error("HTTP "+g.status);const h=(await g.json()).responseData.translatedText.split(`

`);m.forEach((E,S)=>s.set(E,h[S]??E))}l.forEach(a=>{const o=L.get(a);a.isConnected&&s.has(o)&&(a.textContent=s.get(o))}),document.documentElement.lang=e,Y=e}catch(a){console.error("Traduction échouée :",a),t.textContent="🌐 ✗",setTimeout(()=>{t.textContent="🌐 LANGUE",t.classList.remove("loading")},2e3);return}t.textContent="🌐 LANGUE",t.classList.remove("loading")};document.addEventListener("click",e=>{!e.target.closest("#trad-btn")&&!e.target.closest("#trad-panel")&&(document.getElementById("trad-panel")?.classList.remove("open"),document.getElementById("trad-btn")?.classList.remove("open"))});window.toggleA11yPanel=function(){const e=document.getElementById("a11y-panel"),t=document.getElementById("a11y-btn"),l=e.classList.toggle("open");t.classList.toggle("open",l)};window.toggleHVMode=function(){const e=document.body.classList.toggle("hv-mode");document.getElementById("hv-switch")?.classList.toggle("on",e),document.getElementById("a11y-hv-btn")?.classList.toggle("active",e),localStorage.setItem("xenna-hv",e?"1":"")};window.toggleZoom=function(){const e=document.body.classList.toggle("zoom-mode");document.documentElement.style.zoom=e?"200%":"",document.getElementById("zoom-switch")?.classList.toggle("on",e),document.getElementById("a11y-magnifier")?.classList.toggle("active",e),localStorage.setItem("xenna-zoom",e?"1":"")};const J=new Set;window.setAppFont=function(e,t=!1){if(!e){document.body.classList.remove("custom-font"),document.documentElement.style.removeProperty("--app-font"),localStorage.removeItem("xenna-font");const s=document.getElementById("font-picker");s&&(s.value="");return}const l=e.replace(/ /g,"+");if(!J.has(l)){const s=document.createElement("link");s.rel="stylesheet",s.href=`https://fonts.googleapis.com/css2?family=${l}&display=swap`,document.head.appendChild(s),J.add(l)}document.documentElement.style.setProperty("--app-font",`'${e}', monospace`),document.body.classList.add("custom-font"),localStorage.setItem("xenna-font",e);const i=document.getElementById("font-picker");i&&t&&(i.value=e)};window.scan67=function(){const t=Array.from(document.querySelectorAll(".mob-val, .sb-val, .ascii-tbl td, .fm-val, .fm-result td")).filter(i=>/67/.test(i.textContent.replace(/[\s ]/g,""))&&i.offsetParent!==null);if(t.length===0)return;const l=document.getElementById("a11y-67-btn");l.classList.add("active"),t.forEach((i,s)=>{setTimeout(()=>{i.classList.remove("flash-67"),i.offsetWidth,i.classList.add("flash-67"),i.addEventListener("animationend",()=>i.classList.remove("flash-67"),{once:!0})},s*500)}),setTimeout(()=>l.classList.remove("active"),t.length*500+200)};window.toggleBWMode=function(){const e=document.body.classList.toggle("bw-mode");document.getElementById("bw-switch")?.classList.toggle("on",e),document.getElementById("a11y-bw-btn")?.classList.toggle("active",e),localStorage.setItem("xenna-bw",e?"1":"")};function u(e){return String(e).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#039;")}window.setView=function(e){["mobile","desktop","annuel","forge","apropos","gaabrielle","hercule"].forEach(t=>document.body.classList.toggle("is-"+t,e===t)),document.getElementById("btn-desk").classList.toggle("active",e==="desktop"),document.getElementById("btn-mob").classList.toggle("active",e==="mobile"),document.getElementById("btn-ann").classList.toggle("active",e==="annuel"),R&&(e==="desktop"||e==="mobile")&&se(R),e==="forge"&&Me()};function r(e){return parseFloat(e).toLocaleString("fr-FR",{minimumFractionDigits:2,maximumFractionDigits:2})+" €"}function $e(e,t=!1){const l=parseFloat(e),i=l.toLocaleString("fr-FR",{minimumFractionDigits:2,maximumFractionDigits:2})+" €";return t&&l>0?"+"+i:i}function T(e){return(parseFloat(e)*100).toFixed(2)+" %"}function le(){const e=document.body.classList.contains("is-mobile")?"m-date":"d-date",t=e==="m-date"?"d-date":"m-date";return document.getElementById(e)?.value||document.getElementById(t)?.value||P}function O(e){if(!e)return"—";const[t,l,i]=e.split("-");return`${i}/${l}/${t}`}const xe=[{min:0,max:1620,taux:0},{min:1620,max:1683,taux:.005},{min:1683,max:1791,taux:.013},{min:1791,max:1911,taux:.021},{min:1911,max:2042,taux:.029},{min:2042,max:2151,taux:.035},{min:2151,max:2294,taux:.041},{min:2294,max:2714,taux:.053},{min:2714,max:3107,taux:.075},{min:3107,max:3539,taux:.099},{min:3539,max:3983,taux:.119},{min:3983,max:4648,taux:.138},{min:4648,max:5574,taux:.158},{min:5574,max:6974,taux:.179},{min:6974,max:8711,taux:.2},{min:8711,max:12091,taux:.24},{min:12091,max:16376,taux:.28},{min:16376,max:25706,taux:.33},{min:25706,max:55062,taux:.38},{min:55062,max:1/0,taux:.43}];function H(e){const t=parseFloat(e);if(isNaN(t)||t<=0)return{total:0,taux_effectif:0,details:[]};let l=0;const i=[];for(const s of xe){if(t<=s.min)break;const a=+((s.max===1/0?t:Math.min(t,s.max))-s.min).toFixed(2),o=a*s.taux;if(i.push({min:s.min,max:s.max===1/0?null:s.max,taux:s.taux,base:a,montant:+o.toFixed(2)}),l+=o,s.max===1/0||t<=s.max)break}return{total:+l.toFixed(2),taux_effectif:t>0?l/t:0,details:i}}const X={"Sécurité Sociale":"cat-ss","CSG/CRDS":"cat-csg","Retraite complémentaire":"cat-ret",Prévoyance:"cat-prev",Chômage:"cat-cho",Allègement:"cat-alleg"},_={},K={SS_VIEILLESSE_PLAF:"min(Salaire brut, Plafond Mensuel Sécurité Sociale — PMSS)",CHOMAGE:"min(Salaire brut, 4 × PMSS)",AGS:"min(Salaire brut, 4 × PMSS)",CSG_DEDUCTIBLE:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels (CSS art. L136-2)",CSG_NON_DEDUCTIBLE:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels",CRDS:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels",AGIRC_ARRCO_T1:"min(Salaire brut, PMSS)  — Tranche 1 (entre 0 et 1 PMSS)",AGIRC_ARRCO_CEG_T1:"min(Salaire brut, PMSS)  — Tranche 1",PREVOYANCE_CADRE_MIN:"min(Salaire brut, PMSS)  — Tranche A",AGIRC_ARRCO_T2:"Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2",AGIRC_ARRCO_CEG_T2:"Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2"};function w(e,t=!0){const l=["f","(","x",")"].map((i,s)=>`<span style="animation-delay:${s*45}ms">${i}</span>`).join("");return t?`<span class="formula-star" data-fmkey="${e}" onclick="event.stopPropagation();showFormula('${e}')">${l}</span>`:`<span class="formula-star" aria-hidden="true">${l}</span>`}function ie(e,t){if(e.code==="REDUCTION_FILLON")return`<pre class="fm-fillon">${u(e.explication)}</pre>`;const l=t==="sal",i=l?e.taux_sal:e.taux_pat,s=parseFloat(e.base),n=l?parseFloat(e.montant_sal):Math.abs(parseFloat(e.montant_pat)),a=l?"Taux salarial":"Taux patronal",o=l?"Montant salarial":t==="alleg"?"Montant allègement":"Montant patronal",m=l?"c-sal":t==="alleg"?"c-alleg":"c-pat",d=K[e.code]?`<div class="fm-base-note">Assiette  =  ${u(K[e.code])}</div>`:"";return`
    <div class="fm-generic">Montant  =  Assiette  ×  ${a}</div>
    ${d}
    <table class="fm-calc">
      <tr>
        <td>Assiette</td>
        <td class="fm-op">=</td>
        <td class="fm-val c-base">${r(s)}</td>
      </tr>
      <tr>
        <td>${a}</td>
        <td class="fm-op">×</td>
        <td class="fm-val c-taux">${T(i)}</td>
      </tr>
      <tr class="fm-result fm-sep">
        <td>${o}</td>
        <td class="fm-op">=</td>
        <td class="fm-val ${m}">${r(n)}</td>
      </tr>
    </table>`}function Se(e){const t=H(e);return`
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
      <tbody>${t.details.map(i=>{const s=i.min.toLocaleString("fr-FR")+" €",n=i.max===null?"∞":i.max.toLocaleString("fr-FR")+" €",a=i.taux===0;return`
      <tr class="${a?"pas-zero":""}">
        <td>${s} → ${n}</td>
        <td class="r">${r(i.base)}</td>
        <td class="r ${a?"c-dim":""}">${(i.taux*100).toFixed(1).replace(".",",")} %</td>
        <td class="r ${a?"c-dim":"c-purple"}">${a?"—":r(i.montant)}</td>
      </tr>`}).join("")}</tbody>
      <tfoot>
        <tr>
          <td>Net imposable</td>
          <td class="r c-gray">${r(e)}</td>
          <td class="r c-taux">${(t.taux_effectif*100).toFixed(2)} %&nbsp;<span style="color:var(--dim);font-size:0.7em">(taux effectif)</span></td>
          <td class="r c-purple" style="font-weight:bold">${r(t.total)}</td>
        </tr>
      </tfoot>
    </table>`}window.showFormula=function(e){const t=_[e];if(!t)return;const l=document.getElementById("fm-body");if(t.type==="pas"){document.getElementById("fm-title").textContent="Prélèvement à la Source (PAS)",document.getElementById("fm-badge").textContent="── Détail par tranche — barème neutre mensuel DGFIP ─────────",l.className="fm-type-pas",l.innerHTML=Se(t.netImposable),document.getElementById("fm-modal").classList.add("open"),document.querySelectorAll(`[data-fmkey="${e}"]`).forEach(o=>o.classList.add("visited"));return}const{c:i,type:s}=t,n=s==="sal",a=i.code==="REDUCTION_FILLON"?"── Allègement patronal ──────────────────────":n?"── Part salariale ───────────────────────────":"── Part patronale ───────────────────────────";document.getElementById("fm-title").textContent=i.libelle,document.getElementById("fm-badge").textContent=a,l.className=`fm-type-${s}`,l.innerHTML=ie(i,s),document.getElementById("fm-modal").classList.add("open"),document.querySelectorAll(`[data-fmkey="${e}"]`).forEach(o=>o.classList.add("visited"))};window.closeFmModal=function(){document.getElementById("fm-modal").classList.remove("open")};window.toggleExpl=function(e){const t=document.getElementById(`row-${e}`),l=document.getElementById(`expl-${e}`);if(!t||!l)return;const i=l.style.display!=="none";l.style.display=i?"none":"table-row",t.classList.toggle("open",!i)};function Ie(e){const t=document.getElementById("res-desktop"),l=e.cotisations,i=l.reduce((b,$)=>b+parseFloat($.montant_sal),0),s=l.reduce((b,$)=>b+parseFloat($.montant_pat),0),n=H(e.net_imposable),a=parseFloat(e.net_a_payer)-n.total;_.PAS={type:"pas",netImposable:parseFloat(e.net_imposable)};const o=`
    <div class="summary-bar">
      <div class="sb-cell">
        <div class="sb-lbl">▸ SALAIRE BRUT</div>
        <div class="sb-val c-gray">${r(e.brut)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ RETENUES</div>
        <div class="sb-ded">
          <div class="sb-ded-row">
            <span>Cot. salariales</span>
            <span style="color:var(--red)">− ${r(i)}</span>
          </div>
          <div class="sb-ded-row">
            <span>PAS (${(n.taux_effectif*100).toFixed(1)} %)</span>
            <span style="color:var(--purple)">− ${r(n.total)}${w("PAS")}</span>
          </div>
          <div class="sb-ded-total">
            <span>Total retenues</span>
            <span style="color:var(--red)">− ${r(i+n.total)}</span>
          </div>
        </div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ NET À PAYER</div>
        <div class="sb-val c-green">${r(a)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ CHARGES PAT.</div>
        <div class="sb-val c-orange">${r(s)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ SUPER BRUT</div>
        <div class="sb-val c-yellow">${r(parseFloat(e.brut)+s)}</div>
      </div>
    </div>`,m=l.filter(b=>parseFloat(b.montant_sal)>0||b.taux_sal!=="0"),d=l.filter(b=>parseFloat(b.montant_pat)>0),c=l.filter(b=>b.categorie==="Allègement"),g=d.reduce((b,$)=>b+parseFloat($.montant_pat),0);function v(b,$){return b.map((f,F)=>{const I=$+F,A=X[f.categorie]||"cat-ss",pe=parseFloat(f.montant_sal)>0?"c-sal":"c-dim",be=parseFloat(f.montant_pat)>0?"c-pat":"c-dim",U=`${f.code}_sal`,j=`${f.code}_pat`,G=parseFloat(f.montant_sal)>0,z=parseFloat(f.montant_pat)>0;G&&(_[U]={c:f,type:"sal"}),z&&(_[j]={c:f,type:"pat"});const ge=w(U,G),fe=w(j,z);return`
        <tr class="data-row" id="row-${I}" onclick="toggleExpl(${I})">
          <td>
            <span class="expand-icon">▶</span>
            <span class="cat ${A}">[${f.categorie}]</span>
            <span>${f.libelle}</span>
          </td>
          <td class="r">${r(f.base)}</td>
          <td class="r">${T(f.taux_sal)}</td>
          <td class="r ${pe}">${r(f.montant_sal)}${ge}</td>
          <td class="r">${T(f.taux_pat)}</td>
          <td class="r ${be}">${r(f.montant_pat)}${fe}</td>
        </tr>
        <tr class="expl-row" id="expl-${I}" style="display:none">
          <td colspan="6">
            <div class="expl-box">
              <div class="expl-txt">▸ ${f.explication}</div>
              ${f.loi_ref?`<div class="expl-ref">§ ${f.loi_ref}</div>`:""}
            </div>
          </td>
        </tr>`}).join("")}const h=`
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
    </thead>`,E=`
    <div class="tbl-section-head">── COTISATIONS SALARIALES ─────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${h}
      <tbody>
        ${v(m,0)}
        <tr class="tbl-total">
          <td colspan="3">TOTAL COTISATIONS SALARIALES</td>
          <td class="r c-sal">− ${r(i)}</td>
          <td></td><td></td>
        </tr>
      </tbody>
    </table>`,S=`
    <div class="tbl-section-head">── COTISATIONS PATRONALES ──────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${h}
      <tbody>
        ${v(d,m.length)}
        <tr class="tbl-total">
          <td colspan="5">TOTAL COTISATIONS PATRONALES</td>
          <td class="r c-pat">+ ${r(g)}</td>
        </tr>
      </tbody>
    </table>`,p=`<div class="sim-period">
    SIMULATION AU <span class="sp-accent">${O(le())}</span>
    &nbsp;·&nbsp; PMSS en vigueur calculé depuis la base de données
  </div>`,y=c.reduce((b,$)=>b+parseFloat($.montant_pat),0),me=c.length===0?"":`
    <div class="tbl-section-head">── ALLÈGEMENTS PATRONAUX ───────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${h}
      <tbody>
        ${c.map((b,$)=>{const f=m.length+d.length+$,F=X[b.categorie]||"cat-alleg",I=Math.abs(parseFloat(b.montant_pat)),A=`${b.code}_alleg`;return _[A]={c:b,type:"alleg"},`
            <tr class="data-row" id="row-${f}" onclick="toggleExpl(${f})">
              <td>
                <span class="expand-icon">▶</span>
                <span class="cat ${F}">[${b.categorie}]</span>
                <span>${b.libelle}</span>
              </td>
              <td class="r">${r(b.base)}</td>
              <td class="r"></td>
              <td class="r"></td>
              <td class="r c-alleg">${T(Math.abs(parseFloat(b.taux_pat)))}</td>
              <td class="r c-alleg">− ${r(I)}${w(A)}</td>
            </tr>
            <tr class="expl-row" id="expl-${f}" style="display:none">
              <td colspan="6">
                <div class="expl-box">
                  <div class="expl-txt">▸ ${b.explication}</div>
                  ${b.loi_ref?`<div class="expl-ref">§ ${b.loi_ref}</div>`:""}
                </div>
              </td>
            </tr>`}).join("")}
        <tr class="tbl-total">
          <td colspan="5">TOTAL ALLÈGEMENTS PATRONAUX</td>
          <td class="r c-alleg">− ${r(Math.abs(y))}</td>
        </tr>
      </tbody>
    </table>`;t.innerHTML=p+o+`<div class="tbl-wrap">${E}${S}${me}</div>`}window.mobToggle=function(e,t){const l=document.getElementById("mob-expand-"+e),i=document.getElementById("mob-expand-"+e+"-why"),s=document.getElementById("mob-expand-"+e+"-how");if(!l)return;const n=l.style.display!=="none",a=l.dataset.panel;n?a===t?l.style.display="none":(l.dataset.panel=t,i.style.display=t==="why"?"block":"none",s.style.display=t==="how"?"block":"none"):(l.style.display="block",l.dataset.panel=t,i.style.display=t==="why"?"block":"none",s.style.display=t==="how"?"block":"none")};function q(e,t,l,i,s,n=0){const a=e.code==="REDUCTION_FILLON"?`<pre class="fm-fillon">${u(e.explication)}</pre>`:`<div class="fm-type-${s}">${ie(e,s)}</div>`,o=`
    <div class="mob-exp-txt">${u(e.explication)}</div>
    ${e.loi_ref?`<div class="mob-exp-loi">§ ${u(e.loi_ref)}</div>`:""}`;return`
    <div class="${`mob-stripe-${s}-${n%2===0?"a":"b"}`}">
      <div class="mob-row">
        <span class="mob-lbl mob-cot-lbl"
              title="Explication et référence légale"
              onclick="mobToggle('${t}','why')">${u(e.libelle)}</span>
        <span class="mob-val ${i} mob-cot-amt"
              title="Formule de calcul"
              onclick="mobToggle('${t}','how')">${l}</span>
      </div>
      <div class="mob-expand" id="mob-expand-${t}" style="display:none">
        <div id="mob-expand-${t}-why">${o}</div>
        <div id="mob-expand-${t}-how" style="display:none">${a}</div>
      </div>
    </div>`}function Le(e){const t=document.getElementById("res-mobile"),l=document.getElementById("m-nom")?.value||document.getElementById("d-nom")?.value||"",i=document.getElementById("m-prenom")?.value||document.getElementById("d-prenom")?.value||"",s=e.cotisations,n=s.reduce((p,y)=>p+parseFloat(y.montant_sal),0),a=s.reduce((p,y)=>p+parseFloat(y.montant_pat),0),o=H(e.net_imposable),m=parseFloat(e.net_a_payer)-o.total,d=parseFloat(e.brut)+a,c=s.filter(p=>parseFloat(p.montant_sal)>0).map((p,y)=>q(p,`${p.code}_sal`,`− ${r(p.montant_sal)}`,"c-red","sal",y)).join(""),g=s.filter(p=>parseFloat(p.montant_pat)>0),v=g.map((p,y)=>q(p,`${p.code}_pat`,`+ ${r(p.montant_pat)}`,"c-orange","pat",y)).join(""),h=g.reduce((p,y)=>p+parseFloat(y.montant_pat),0),E=s.filter(p=>p.categorie==="Allègement").map((p,y)=>q(p,`${p.code}_alleg`,`− ${r(Math.abs(parseFloat(p.montant_pat)))}`,"c-alleg","alleg",y)).join(""),S=s.filter(p=>p.categorie==="Allègement").reduce((p,y)=>p+parseFloat(y.montant_pat),0);t.innerHTML=`
    <div class="mob-bulletin">

      <!-- En-tête bulletin -->
      <div class="mob-head">
        <span class="mob-head-title">BULLETIN DE PAYE</span>
        <div style="text-align:right">
          <div class="mob-head-name">${u(i)} ${u(l).toUpperCase()}</div>
          <div class="mob-head-date">simulation au ${O(le())}</div>
        </div>
      </div>

      <!-- Brut -->
      <div class="mob-row" style="margin-top:0.15rem">
        <span class="mob-lbl">Salaire de base brut</span>
        <span class="mob-val c-gray">${r(e.brut)}</span>
      </div>

      <!-- Section cotisations salariales -->
      <div class="mob-row section"><span class="mob-lbl">── COTISATIONS SALARIALES ──</span><span></span></div>
      ${c}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL retenues salariales</span>
        <span class="mob-val c-red">− ${r(n)}</span>
      </div>

      <!-- Net imposable -->
      <div class="mob-row net-row">
        <span class="mob-lbl">NET IMPOSABLE</span>
        <span class="mob-val c-green">${r(e.net_imposable)}</span>
      </div>

      <!-- PAS -->
      <div class="mob-row pas-row">
        <span class="mob-lbl">Prélèvement à la source (${(o.taux_effectif*100).toFixed(1)} %)</span>
        <span class="mob-val c-purple">− ${r(o.total)}${w("PAS")}</span>
      </div>

      <!-- Net à payer -->
      <div class="mob-row final-row">
        <span class="mob-lbl">NET À PAYER</span>
        <span class="mob-val c-green">${r(m)}</span>
      </div>

      <!-- Section cotisations patronales -->
      <div class="mob-row section"><span class="mob-lbl">── COTISATIONS PATRONALES ──</span><span></span></div>
      ${v}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL charges patronales brutes</span>
        <span class="mob-val c-orange">+ ${r(h)}</span>
      </div>

      <!-- Allègements -->
      ${E.length?`
      <div class="mob-row section"><span class="mob-lbl">── ALLÈGEMENTS PATRONAUX ──</span><span></span></div>
      ${E}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL allègements</span>
        <span class="mob-val c-alleg">− ${r(Math.abs(S))}</span>
      </div>`:""}

      <!-- Super brut -->
      <div class="mob-row superbrut">
        <span class="mob-lbl">SUPER BRUT (coût employeur)</span>
        <span class="mob-val c-blue">${r(d)}</span>
      </div>

    </div>`}function se(e){Ie(e),Le(e)}function W(e){const t=`<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">⚠ ${u(e)}</div>`;document.getElementById("res-desktop").innerHTML=t,document.getElementById("res-mobile").innerHTML=t}async function ae(e){const t=e==="mobile",l=document.getElementById(t?"m-brut":"d-brut").value,i=document.getElementById(t?"m-statut":"d-statut").value,s=document.getElementById(t?"m-nom":"d-nom").value||"Dupont",n=document.getElementById(t?"m-prenom":"d-prenom").value||"Marie",a=document.getElementById(t?"m-date":"d-date").value||P,o=document.getElementById(t?"m-alsace-moselle":"d-alsace-moselle")?.checked??!1,m=parseFloat(l);if(!l||isNaN(m)||m<=0){W("Salaire brut invalide — saisir un montant positif.");return}if(!/^\d{4}-\d{2}-\d{2}$/.test(a)){W(`Date invalide : '${a}' (format attendu : YYYY-MM-DD).`);return}["d-brut","m-brut"].forEach(d=>{const c=document.getElementById(d);c&&(c.value=l)}),["d-statut","m-statut"].forEach(d=>{const c=document.getElementById(d);c&&(c.value=i)}),["d-nom","m-nom"].forEach(d=>{const c=document.getElementById(d);c&&(c.value=s)}),["d-prenom","m-prenom"].forEach(d=>{const c=document.getElementById(d);c&&(c.value=n)}),["d-date","m-date"].forEach(d=>{const c=document.getElementById(d);c&&(c.value=a)});try{const d=await te("calculer_bulletin",{salarie:{nom:s,prenom:n,salaire_brut:l.toString(),statut:i,alsace_moselle:o},datePaie:a});R=d,se(d)}catch(d){console.error("[calculer_bulletin] erreur brute :",d);const c=M(d),g=`<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">ERREUR : ${u(c)}</div>`;document.getElementById("res-desktop").innerHTML=g,document.getElementById("res-mobile").innerHTML=g}}function _e(e){const t=document.getElementById("res-annuel"),l=e.lignes,i=l.map(c=>c.smic),s=`
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
    </tr></thead>`,n=l.map((c,g)=>{const v=g>0&&c.smic!==i[g-1],h=parseFloat(c.fillon_regularise)-parseFloat(c.fillon_simple),E=Math.abs(h)<.005?'<span style="color:var(--dim)">—</span>':`<span class="delta-nonzero">${h>0?"+":""}${$e(h.toFixed(2))}</span>`;return`<tr class="${v?"smic-change":""}">
      <td>${c.mois_libelle}</td>
      <td>${r(c.smic)}</td>
      <td>${r(c.brut)}</td>
      <td class="c-sal">− ${r(c.total_sal)}</td>
      <td class="c-pat">+ ${r(c.total_pat_brut)}</td>
      <td class="c-alleg">− ${r(c.fillon_regularise)}</td>
      <td>${E}</td>
      <td class="c-green">${r(c.net_a_payer)}</td>
      <td class="c-yellow">${r(c.cout_employeur)}</td>
    </tr>`}).join(""),a=`
    <tr class="ann-total">
      <td>TOTAL ${e.annee}</td>
      <td></td>
      <td>${r(e.total_brut)}</td>
      <td class="c-sal">− ${r(e.total_sal)}</td>
      <td class="c-pat">+ ${r(e.total_pat_brut)}</td>
      <td class="c-alleg">− ${r(e.total_fillon)}</td>
      <td></td>
      <td class="c-green">${r(e.total_net)}</td>
      <td class="c-yellow">${r(e.total_cout)}</td>
    </tr>`,o=parseFloat(e.total_pat_brut),m=parseFloat(e.total_fillon),d=`
    <div style="display:flex;gap:1rem;flex-wrap:wrap;margin-top:0.75rem;font-size:0.72rem">
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">ÉCONOMIE FILLON (annuelle)</div>
        <div style="color:var(--green);font-size:1.1rem;font-weight:bold">− ${r(e.total_fillon)}</div>
      </div>
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">TAUX FILLON MOYEN</div>
        <div style="color:var(--blue);font-size:1.1rem;font-weight:bold">
          ${o>0?(m/parseFloat(e.total_brut)*100).toFixed(2)+" %":"—"}
        </div>
      </div>
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">COÛT EMPLOYEUR ANNUEL</div>
        <div style="color:var(--yellow);font-size:1.1rem;font-weight:bold">${r(e.total_cout)}</div>
      </div>
    </div>`;t.innerHTML=`
    <div class="tbl-section-head">── SIMULATION ANNUELLE ${e.annee} ────────────────────────────────────</div>
    <table class="ann-tbl">
      ${s}
      <tbody>${n}</tbody>
      ${a}
    </table>
    ${d}`}async function we(){const e=parseInt(document.getElementById("a-annee").value),t=document.getElementById("a-brut").value,l=document.getElementById("a-statut").value,i=document.getElementById("res-annuel");if(isNaN(e)||e<1900||e>2100){i.innerHTML='<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Année invalide.</div>';return}const s=parseFloat(t);if(!t||isNaN(s)||s<=0){i.innerHTML='<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Salaire brut invalide — saisir un montant positif.</div>';return}i.innerHTML='<div style="color:var(--muted);padding:1rem;font-size:0.78rem">Calcul en cours…</div>';try{const n=await te("simuler_annee",{annee:e,salaireBrut:t.toString(),statut:l});_e(n)}catch(n){console.error("[simuler_annee] erreur brute :",n),i.innerHTML=`<div style="padding:1rem;color:var(--red);font-size:0.8rem">ERREUR : ${u(M(n))}</div>`}}window.toggleParams=function(e){const t=document.getElementById(`${e}-params`),l=document.getElementById(`${e}-params-toggle`);if(!t)return;const i=t.style.display!=="none";t.style.display=i?"none":"block",l.classList.toggle("open",!i)};window.syncParam=function(e,t){["d","m"].forEach(l=>{const i=document.getElementById(`${l}-${e}`);i&&i.checked!==t&&(i.checked=t)})};document.getElementById("d-calc").addEventListener("click",()=>ae("desktop"));document.getElementById("m-calc").addEventListener("click",()=>ae("mobile"));document.getElementById("a-calc").addEventListener("click",we);const ne=[{idcc:"1261",libelle:"Acteurs du lien social et familial (ALISFA)"},{idcc:"2941",libelle:"Aide, accompagnement, soins et services à domicile"},{idcc:"1747",libelle:"Activités industrielles de boulangerie et de pâtisserie"},{idcc:"2149",libelle:"Activités du déchet"},{idcc:"2335",libelle:"Agences générales d'assurances"},{idcc:"1686",libelle:"Audiovisuel, électronique et équipement ménager"},{idcc:"2120",libelle:"Banque"},{idcc:"3210",libelle:"Banque Populaire"},{idcc:"0567",libelle:"Bijouterie, joaillerie, orfèvrerie (obsolète)"},{idcc:"0158",libelle:"Bois et scieries"},{idcc:"0992",libelle:"Boucherie"},{idcc:"0843",libelle:"Boulangerie-pâtisserie artisanales"},{idcc:"1606",libelle:"Bricolage"},{idcc:"1486",libelle:"Bureaux d'études techniques et sociétés de conseils (Syntec)"},{idcc:"0787",libelle:"Cabinets d'experts-comptables et de commissaires aux comptes"},{idcc:"2332",libelle:"Cabinets d'architectes"},{idcc:"1619",libelle:"Cabinets dentaires"},{idcc:"2420",libelle:"Cadres du bâtiment"},{idcc:"3212",libelle:"Cadres des travaux publics"},{idcc:"1256",libelle:"Cadres des entreprises de gestion d'équipements thermiques et de climatisation"},{idcc:"0211",libelle:"Cadres des industries de carrières et matériaux (obsolète)"},{idcc:"0045",libelle:"Caoutchouc"},{idcc:"2257",libelle:"Casinos"},{idcc:"0783",libelle:"Centres d'hébergement et de réadaptation sociale"},{idcc:"0953",libelle:"Charcuterie de détail"},{idcc:"1580",libelle:"Chaussure"},{idcc:"2060",libelle:"Chaînes de cafétérias"},{idcc:"1557",libelle:"Commerce des articles de sports et d'équipements de loisirs"},{idcc:"2216",libelle:"Commerce de détail et de gros à prédominance alimentaire"},{idcc:"1505",libelle:"Commerce de détail alimentaire non spécialisé"},{idcc:"2198",libelle:"Commerce à distance et E-commerce"},{idcc:"1483",libelle:"Commerce de détail de l'habillement"},{idcc:"1487",libelle:"Commerce de détail de l'horlogerie-bijouterie"},{idcc:"3237",libelle:"Commerce de détail alimentaire spécialisé"},{idcc:"1225",libelle:"Commerce de la Réunion"},{idcc:"0468",libelle:"Commerce succursaliste de la chaussure"},{idcc:"0573",libelle:"Commerces de gros"},{idcc:"1517",libelle:"Commerces de détail non alimentaires (Codena)"},{idcc:"0500",libelle:"Commerces de gros de l'habillement, mercerie, chaussure et jouet"},{idcc:"3243",libelle:"Commerces de quincaillerie, fournitures industrielles, fers, métaux et équipement de la maison"},{idcc:"2596",libelle:"Coiffure"},{idcc:"1611",libelle:"Communication écrite directe"},{idcc:"1286",libelle:"Confiserie, chocolaterie, biscuiterie"},{idcc:"2583",libelle:"Concessionnaires et exploitants d'autoroutes ou d'ouvrages routiers"},{idcc:"3217",libelle:"Convention collective nationale de la branche ferroviaire"},{idcc:"2272",libelle:"Convention collective nationale de l'assainissement et de la maintenance industrielle"},{idcc:"2002",libelle:"Convention collective interrégionale de la blanchisserie, laverie, location de linge, nettoyage à sec, pressing et teinturerie du 17 novembre 1997"},{idcc:"2247",libelle:"Courtage d'assurances et/ou de réassurances"},{idcc:"0303",libelle:"Couture parisienne et autres métiers de la mode"},{idcc:"0733",libelle:"Détaillants en chaussures"},{idcc:"1605",libelle:"Désinfection, désinsectisation, dératisation"},{idcc:"1536",libelle:"Distributeurs conseils hors domicile"},{idcc:"2372",libelle:"Distribution directe"},{idcc:"1408",libelle:"Distribution, Logistique et Services des Energies de Proximité"},{idcc:"2121",libelle:"Édition"},{idcc:"1518",libelle:"Education, culture, loisirs et animation agissant pour l'utilité sociale et environnementale, au service des territoires (ECLAT)"},{idcc:"2609",libelle:"Employés, techniciens et agents de maîtrise du bâtiment"},{idcc:"2614",libelle:"Employés, techniciens et agents de maîtrise des travaux publics"},{idcc:"0135",libelle:"Employés techniciens et agents de maîtrise des industries de carrières et de matériaux (obsolète)"},{idcc:"3218",libelle:"Enseignement privé non lucratif"},{idcc:"2691",libelle:"Enseignement privé hors contrat"},{idcc:"3043",libelle:"Entreprises de propreté"},{idcc:"3127",libelle:"Entreprises de services à la personne"},{idcc:"1285",libelle:"Entreprises artistiques et culturelles"},{idcc:"1539",libelle:"Entreprises du bureau et du numérique - Commerces et services (Eben)"},{idcc:"1412",libelle:"Entreprises d'installation sans fabrication de matériel aéraulique, thermique, frigorifique"},{idcc:"2717",libelle:"Entreprises techniques au service de la création et de l'évènement"},{idcc:"3032",libelle:"Esthétique"},{idcc:"0029",libelle:"Établissements privés d'hospitalisation, de soins, de cure et de garde à but non lucratif (CCN 51 - FEHAP)"},{idcc:"0413",libelle:"Établissements et services pour personnes inadaptées et handicapées (CCN 66)"},{idcc:"0405",libelle:"Établissements médico-sociaux de l'union intersyndicale des secteurs sanitaires et sociaux (CCN 65)"},{idcc:"0478",libelle:"Établissements financiers"},{idcc:"0915",libelle:"Expertises en matière d'évaluations industrielles et commerciales"},{idcc:"1307",libelle:"Exploitation cinématographique"},{idcc:"1405",libelle:"Expédition et exportation de fruits et légumes"},{idcc:"1411",libelle:"Fabrication de l'ameublement"},{idcc:"0669",libelle:"Fabrication mécanique du verre"},{idcc:"1821",libelle:"Fabrication du verre à la main, semi-automatique et mixte"},{idcc:"1031",libelle:"Fédération nationale des associations familiales rurales"},{idcc:"1978",libelle:"Fleuristes, vente et services des animaux familiers"},{idcc:"0200",libelle:"Froid"},{idcc:"1043",libelle:"Gardiens d'immeubles"},{idcc:"2543",libelle:"Géomètres et experts-fonciers"},{idcc:"2021",libelle:"Golf"},{idcc:"2156",libelle:"Grands magasins"},{idcc:"2336",libelle:"Habitat et du Logement Accompagnés"},{idcc:"1631",libelle:"Hôtellerie de plein air"},{idcc:"1979",libelle:"Hôtels, cafés, restaurants (HCR)"},{idcc:"2264",libelle:"Hospitalisation privée (FHP)"},{idcc:"1921",libelle:"Huissiers de justice"},{idcc:"0044",libelle:"Industries chimiques"},{idcc:"1534",libelle:"Industrie et commerces en gros des viandes"},{idcc:"3233",libelle:"Industrie de la fabrication des ciments"},{idcc:"2089",libelle:"Industrie des panneaux à base de bois"},{idcc:"0176",libelle:"Industrie pharmaceutique"},{idcc:"1388",libelle:"Industrie du pétrole"},{idcc:"0112",libelle:"Industrie laitière"},{idcc:"0018",libelle:"Industrie textile"},{idcc:"3236",libelle:"Industrie et services nautiques"},{idcc:"3109",libelle:"Industries alimentaires diverses"},{idcc:"0247",libelle:"Industries de l'habillement"},{idcc:"2542",libelle:"Industries métallurgiques, mécaniques et connexes de l'Aisne (obsolète)"},{idcc:"3209",libelle:"Industries métallurgiques, mécaniques et connexes du Doubs (obsolète)"},{idcc:"2003",libelle:"Industries métallurgiques, électriques et électroniques des Vosges (obsolète)"},{idcc:"2630",libelle:"Industries métallurgiques des Bouches-du-Rhône et Alpes-de-Haute-Provence (obsolète)"},{idcc:"1396",libelle:"Industries de produits alimentaires élaborés"},{idcc:"0489",libelle:"Industries du cartonnage"},{idcc:"0637",libelle:"Industries et commerce de la récupération"},{idcc:"1938",libelle:"Industries de la transformation des volailles"},{idcc:"1586",libelle:"Industries charcutières"},{idcc:"0184",libelle:"Imprimerie de labeur et industries graphiques"},{idcc:"0043",libelle:"Import-export et commerce international"},{idcc:"1527",libelle:"Immobilier"},{idcc:"0650",libelle:"Ingénieurs et cadres de la métallurgie (obsolète)"},{idcc:"1679",libelle:"Inspection d'assurance"},{idcc:"1794",libelle:"Institutions de retraite complémentaire"},{idcc:"1760",libelle:"Jardineries et graineteries"},{idcc:"1480",libelle:"Journalistes"},{idcc:"0959",libelle:"Laboratoires de biologie médicale extra-hospitaliers"},{idcc:"3013",libelle:"Librairie"},{idcc:"1404",libelle:"Machines et matériels agricoles et de travaux publics (SDLM)"},{idcc:"0675",libelle:"Maisons à succursales de vente au détail d'habillement"},{idcc:"0538",libelle:"Manutention ferroviaire"},{idcc:"2528",libelle:"Maroquinerie"},{idcc:"1589",libelle:"Mareyeurs-expéditeurs"},{idcc:"2931",libelle:"Marchés financiers"},{idcc:"3222",libelle:"Menuiseries charpentes et constructions industrialisées et des portes planes"},{idcc:"0822",libelle:"Mensuels de la métallurgie de la Savoie (obsolète)"},{idcc:"1387",libelle:"Mensuels de la métallurgie des Flandres (obsolète)"},{idcc:"0914",libelle:"Mensuels de la métallurgie de l'Ain (obsolète)"},{idcc:"1930",libelle:"Meunerie"},{idcc:"2190",libelle:"Missions locales et PAIO des maisons de l'emploi et PLIE"},{idcc:"1499",libelle:"Miroiterie, transformation et négoce du verre"},{idcc:"0827",libelle:"Métallurgie des Ardennes (obsolète)"},{idcc:"0863",libelle:"Métallurgie d'Ille-et-Vilaine et du Morbihan (obsolète)"},{idcc:"1867",libelle:"Métallurgie de la Drôme et de l'Ardèche (obsolète)"},{idcc:"0984",libelle:"Métallurgie d'Eure-et-Loir (obsolète)"},{idcc:"2992",libelle:"Métallurgie d'Indre-et-Loire (obsolète)"},{idcc:"0898",libelle:"Métallurgie de l'Allier (obsolète)"},{idcc:"1572",libelle:"Métallurgie de la Charente (obsolète)"},{idcc:"1885",libelle:"Métallurgie de la Côte-d'Or (obsolète)"},{idcc:"1635",libelle:"Métallurgie de la Gironde et des Landes (obsolète)"},{idcc:"1578",libelle:"Métallurgie de la Loire et de l'arrondissement d'Yssingeaux (obsolète)"},{idcc:"0828",libelle:"Métallurgie de la Manche (obsolète)"},{idcc:"0899",libelle:"Métallurgie de la Marne (obsolète)"},{idcc:"1813",libelle:"Métallurgie de la région de Maubeuge (obsolète)"},{idcc:"1525",libelle:"Métallurgie de la région dunkerquoise (obsolète)"},{idcc:"0930",libelle:"Métallurgie de la Sarthe (obsolète)"},{idcc:"0920",libelle:"Métallurgie de la Vienne (obsolète)"},{idcc:"3053",libelle:"Métallurgie de Haute-Saône (obsolète)"},{idcc:"1576",libelle:"Métallurgie du Cher (obsolète)"},{idcc:"0943",libelle:"Métallurgie du Calvados (obsolète)"},{idcc:"0860",libelle:"Métallurgie du Finistère (obsolète)"},{idcc:"2126",libelle:"Métallurgie du Gard et de la Lozère (obsolète)"},{idcc:"1912",libelle:"Métallurgie du Haut-Rhin (obsolète)"},{idcc:"0836",libelle:"Métallurgie de la Haute-Savoie (obsolète)"},{idcc:"0937",libelle:"Métallurgie de la Haute-Vienne et de la Creuse (obsolète)"},{idcc:"1577",libelle:"Métallurgie de l'Hérault, de l'Aude et des Pyrénées-Orientales (obsolète)"},{idcc:"2221",libelle:"Métallurgie de l'Isère et des Hautes-Alpes"},{idcc:"1369",libelle:"Métallurgie de Loire-Atlantique (obsolète)"},{idcc:"2579",libelle:"Métallurgie du Loir-et-Cher (obsolète)"},{idcc:"1966",libelle:"Métallurgie du Loiret (obsolète)"},{idcc:"1902",libelle:"Métallurgie du Maine-et-Loire (obsolète)"},{idcc:"2266",libelle:"Métallurgie de la Mayenne (obsolète)"},{idcc:"1365",libelle:"Métallurgie de Meurthe-et-Moselle (obsolète)"},{idcc:"2755",libelle:"Industries de la métallurgie de Belfort/Montbéliard (obsolète)"},{idcc:"1059",libelle:"Métallurgie des Midi-Pyrénées (obsolète)"},{idcc:"0714",libelle:"Métallurgie de la Moselle (obsolète)"},{idcc:"0948",libelle:"Métallurgie de l'Orne (obsolète)"},{idcc:"2700",libelle:"Métallurgie de l'Oise (obsolète)"},{idcc:"1472",libelle:"Métallurgie du Pas-de-Calais (obsolète)"},{idcc:"2615",libelle:"Métallurgie des Pyrénées-Atlantiques et du Seignanx (obsolète)"},{idcc:"0878",libelle:"Métallurgie du Rhône (obsolète)"},{idcc:"1604",libelle:"Métallurgie de Rouen et de Dieppe (obsolète)"},{idcc:"1564",libelle:"Métallurgie de Saône-et-Loire (obsolète)"},{idcc:"0911",libelle:"Métallurgie de Seine-et-Marne (obsolète)"},{idcc:"2980",libelle:"Métallurgie de la Somme (obsolète)"},{idcc:"1592",libelle:"Métallurgie du Valenciennois et du Cambrésis (obsolète)"},{idcc:"2489",libelle:"Métallurgie de la Vendée (obsolète)"},{idcc:"1634",libelle:"Métallurgie des Côtes-d'Armor (obsolète)"},{idcc:"2630",libelle:"Métallurgie des Bouches-du-Rhône (obsolète)"},{idcc:"1315",libelle:"Industries métallurgiques et mécaniques de la Haute-Marne et de la Meuse (obsolète)"},{idcc:"1732",libelle:"Métallurgie de l'Yonne (obsolète)"},{idcc:"1560",libelle:"Métallurgiques des Alpes-Maritimes (obsolète)"},{idcc:"0979",libelle:"Métallurgiques de l'arrondissement du Havre (obsolète)"},{idcc:"2128",libelle:"Mutualité"},{idcc:"1077",libelle:"Négoce et industrie des produits du sol, engrais et produits connexes"},{idcc:"1880",libelle:"Négoce de l'ameublement"},{idcc:"1982",libelle:"Négoce et prestations de services dans les domaines médico-techniques"},{idcc:"1947",libelle:"Négoce de bois d'oeuvre et produits dérivés (obsolète)"},{idcc:"0054",libelle:"Non-cadres des industries métallurgiques et mécaniques de la région parisienne (obsolète)"},{idcc:"0998",libelle:"Non-cadres de l'exploitation d'équipements thermiques et de génie climatique"},{idcc:"2205",libelle:"Notaires"},{idcc:"3220",libelle:"Offices publics de l'habitat"},{idcc:"3245",libelle:"Opérateurs de voyages et guides"},{idcc:"1431",libelle:"Optique-lunetterie de détail"},{idcc:"1316",libelle:"Organismes de tourisme social et familial"},{idcc:"1909",libelle:"Organismes de tourisme"},{idcc:"1516",libelle:"Organismes de formation"},{idcc:"1790",libelle:"Parcs de loisirs et d'attractions"},{idcc:"1267",libelle:"Pâtisserie"},{idcc:"1000",libelle:"Personnel des cabinets d'avocats"},{idcc:"1147",libelle:"Personnel des cabinets médicaux"},{idcc:"0275",libelle:"Personnel au sol du transport aérien"},{idcc:"2046",libelle:"Personnel non médical des centres de lutte contre le cancer"},{idcc:"2972",libelle:"Personnel sédentaire des entreprises de navigation"},{idcc:"1558",libelle:"Personnel des industries céramiques"},{idcc:"1996",libelle:"Pharmacie d'officine"},{idcc:"1504",libelle:"Poissonnerie"},{idcc:"0759",libelle:"Pompes funèbres"},{idcc:"2683",libelle:"Portage de presse"},{idcc:"3017",libelle:"Ports et Manutention"},{idcc:"3230",libelle:"Presse (Information spécialisée [ETAM et cadres])"},{idcc:"3242",libelle:"Presse quotidienne et hebdomadaire en régions"},{idcc:"2098",libelle:"Prestataires de services du secteur tertiaire"},{idcc:"1351",libelle:"Prévention et sécurité"},{idcc:"1512",libelle:"Promotion immobilière"},{idcc:"0292",libelle:"Plasturgie"},{idcc:"3168",libelle:"Professions de la photographie"},{idcc:"3244",libelle:"Professions réglementées auprès des juridictions"},{idcc:"1555",libelle:"Produits à usage pharmaceutique, parapharmaceutique et vétérinaire"},{idcc:"1513",libelle:"Production des eaux embouteillées, des boissons rafraîchissantes sans alcool et de bière"},{idcc:"2642",libelle:"Production audiovisuelle"},{idcc:"3238",libelle:"Production et transformation des papiers et cartons"},{idcc:"0653",libelle:"Producteurs salariés de base des services extérieurs de production des sociétés d'assurances"},{idcc:"0993",libelle:"Prothèse dentaire"},{idcc:"0086",libelle:"Publicité"},{idcc:"1621",libelle:"Répartition pharmaceutique"},{idcc:"0454",libelle:"Remontées mécaniques et domaines skiables"},{idcc:"1266",libelle:"Restauration de collectivités"},{idcc:"1501",libelle:"Restauration rapide"},{idcc:"1413",libelle:"Salariés permanents des entreprises de travail temporaire"},{idcc:"3216",libelle:"Salariés du négoce des matériaux de construction"},{idcc:"3219",libelle:"Salariés en portage salarial"},{idcc:"1875",libelle:"Salariés des cabinets et cliniques vétérinaires"},{idcc:"0897",libelle:"Services de prévention et de santé au travail interentreprises"},{idcc:"1090",libelle:"Services de l'automobile"},{idcc:"2147",libelle:"Services d'eau et d'assainissement"},{idcc:"2344",libelle:"Sidérurgie (Nord, Moselle, Meurthe-et-Moselle)"},{idcc:"1672",libelle:"Sociétés d'assurances"},{idcc:"1801",libelle:"Sociétés d'assistance"},{idcc:"2150",libelle:"Sociétés anonymes et fondations d'HLM"},{idcc:"3090",libelle:"Spectacle vivant (secteur privé)"},{idcc:"2511",libelle:"Sport"},{idcc:"2728",libelle:"Sucreries, sucreries-distilleries et raffineries de sucre"},{idcc:"2219",libelle:"Taxis parisiens salariés"},{idcc:"2148",libelle:"Télécommunications"},{idcc:"3241",libelle:"Télédiffusion"},{idcc:"1424",libelle:"Transports publics"},{idcc:"0016",libelle:"Transports routiers et activités auxiliaires du transport"},{idcc:"1170",libelle:"Tuiles et briques (obsolète)"},{idcc:"0087",libelle:"Ouvriers des industries de carrières et de matériaux (obsolète)"},{idcc:"1702",libelle:"Ouvriers de travaux publics"},{idcc:"1596",libelle:"Ouvriers des entreprises du bâtiment de moins de 10 salariés"},{idcc:"1597",libelle:"Ouvriers des entreprises du bâtiment de plus de 10 salariés"},{idcc:"2389",libelle:"Ouvriers du bâtiment et des travaux publics région de La Réunion"},{idcc:"2328",libelle:"Ouvriers du bâtiment et des travaux publics de la Guadeloupe et dépendances"},{idcc:"2564",libelle:"Vétérinaires praticiens salariés"},{idcc:"0493",libelle:"Vins, cidres, jus de fruits, sirops, spiritueux et liqueurs de France"}].sort((e,t)=>e.libelle.localeCompare(t.libelle,"fr")),Ce='<option value="">— Choisir une CCN —</option>'+ne.map(e=>`<option value="${e.idcc}">${e.idcc} — ${e.libelle}</option>`).join("");let x=[];window.forgeNav=function(e){["liste","detail","creer"].forEach(t=>{document.getElementById("forge-"+t).style.display=t===e?"block":"none"})};async function Me(){forgeNav("liste");const e=document.getElementById("forge-cards"),t=document.getElementById("forge-subtitle");e.innerHTML='<div style="color:var(--muted);font-size:0.75rem;padding:0.5rem 0">chargement…</div>';try{const l=await fetch("/forge/contributeurs");if(!l.ok){const s=await l.text();throw new Error(`HTTP ${l.status} — ${s||l.statusText}`)}x=await l.json();const i=x.length;t.textContent=i===0?"aucun contributeur pour l'instant":`${i} contributeur${i>1?"s":""} · ${x.reduce((s,n)=>s+n.expertises.length,0)} expertises CCN`,e.innerHTML=i===0?'<div style="color:var(--muted);font-size:0.75rem">Aucun profil encore — sois le premier à rejoindre.</div>':x.map(Ae).join("")}catch(l){e.innerHTML=`<div style="color:var(--red);font-size:0.75rem">Erreur : ${u(M(l))}</div>`}}function Ae(e){const t=e.expertises.slice(0,5).map(i=>`<span class="ccn-badge ${i.niveau==="Maîtrisée"?"m":i.niveau==="Pratiquée"?"p":"c"}" title="${u(i.niveau)}">${u(i.ccn_libelle)}</span>`).join(""),l=e.expertises.length>5?`<span class="ccn-badge c">+${e.expertises.length-5}</span>`:"";return`
    <div class="forge-card" onclick="forgeAfficherProfil('${u(e.pseudo)}')">
      <div class="forge-card-pseudo">${u(e.pseudo)}</div>
      <div class="forge-card-poste">${u(e.poste)} <span style="color:var(--dim);font-size:0.6em">${e.poste_est_actuel?"actuel":"visé"}</span></div>
      <div class="forge-card-ccn">${t}${l}</div>
      <div class="forge-card-stats">
        <span><span class="forge-stat-val">${e.votes_received}</span> votes</span>
        <span><span class="forge-stat-val">${e.topics_count}</span> sujets</span>
        <span><span class="forge-stat-val">${e.posts_count}</span> réponses</span>
      </div>
    </div>`}async function Te(e){forgeNav("detail");const t=document.getElementById("forge-profil-content");t.innerHTML='<div style="color:var(--muted);font-size:0.75rem">chargement…</div>';try{let l=x.find(i=>i.pseudo.toLowerCase()===e.toLowerCase());if(!l){const i=await fetch(`/profil/${encodeURIComponent(e)}`);if(!i.ok)throw new Error(`HTTP ${i.status} — ${await i.text()||i.statusText}`);l=await i.json()}t.innerHTML=Pe(l)}catch(l){t.innerHTML=`<div style="color:var(--red);font-size:0.75rem">Erreur : ${u(M(l))}</div>`}}function Pe(e){const t=e.linkedin_url?`<a class="profil-linkedin" href="${u(e.linkedin_url)}" target="_blank" rel="noopener noreferrer">↗ LinkedIn</a>`:"",i=[{niveau:"Maîtrisée",cls:"m",items:e.expertises.filter(a=>a.niveau==="Maîtrisée")},{niveau:"Pratiquée",cls:"p",items:e.expertises.filter(a=>a.niveau==="Pratiquée")},{niveau:"Connue",cls:"c",items:e.expertises.filter(a=>a.niveau==="Connue")}].filter(a=>a.items.length>0).map(a=>`
    <tr class="profil-ccn-section"><td colspan="3">${u(a.niveau)}</td></tr>
    ${a.items.map(o=>`
    <tr>
      <td class="profil-ccn-idcc">${u(o.ccn_idcc)}</td>
      <td>${u(o.ccn_libelle)}</td>
      <td><span class="ccn-badge ${a.cls}">${u(a.niveau)}</span></td>
    </tr>`).join("")}`).join(""),s=e.expertises.length===0?'<div style="color:var(--muted);font-size:0.72rem">Aucune CCN renseignée.</div>':`<table class="profil-ccn-tbl">${i}</table>`,n=e.created_at?O(e.created_at.slice(0,10)):"—";return`
    <div class="profil-head">
      <div>
        <div class="profil-pseudo">${u(e.pseudo)}</div>
        <div class="profil-poste">${u(e.poste)} <span style="color:var(--dim);font-size:0.85em">(${e.poste_est_actuel?"poste actuel":"poste visé"})</span></div>
        ${t}
      </div>
      <div class="profil-since">membre depuis le ${n}</div>
    </div>

    <div class="profil-body">
      <div class="sect-label">PAIE FRANÇAISE</div>
      ${e.paie_fr_niveau?`<span class="ccn-badge ${e.paie_fr_niveau==="Maîtrisée"?"m":e.paie_fr_niveau==="Pratiquée"?"p":"c"}" style="font-size:0.75rem;padding:0.2rem 0.6rem">${u(e.paie_fr_niveau)}</span>`:'<span style="color:var(--dim);font-size:0.7rem">non renseigné</span>'}

      ${e.pays&&e.pays.length>0?`
      <div class="sect-label" style="margin-top:1rem">PAIE INTERNATIONALE</div>
      <table class="profil-ccn-tbl">
        ${[{niveau:"Maîtrisée",cls:"m",items:e.pays.filter(a=>a.niveau==="Maîtrisée")},{niveau:"Pratiquée",cls:"p",items:e.pays.filter(a=>a.niveau==="Pratiquée")},{niveau:"Connue",cls:"c",items:e.pays.filter(a=>a.niveau==="Connue")}].filter(a=>a.items.length>0).map(a=>`
            <tr class="profil-ccn-section"><td colspan="3">${u(a.niveau)}</td></tr>
            ${a.items.map(o=>`
            <tr>
              <td class="profil-ccn-idcc">${u(o.pays_code)}</td>
              <td>${u(o.pays_libelle)}</td>
              <td><span class="ccn-badge ${a.cls}">${u(a.niveau)}</span></td>
            </tr>`).join("")}`).join("")}
      </table>`:""}

      <div class="sect-label" style="margin-top:1rem">EXPERTISES CCN</div>
      ${s}
    </div>

    <div class="profil-stats">
      <div class="profil-stat">
        <div class="profil-stat-val">${e.votes_received}</div>
        <div class="profil-stat-lbl">votes reçus</div>
      </div>
      <div class="profil-stat">
        <div class="profil-stat-val">${e.votes_given}</div>
        <div class="profil-stat-lbl">votes donnés</div>
      </div>
      <div class="profil-stat">
        <div class="profil-stat-val">${e.topics_count}</div>
        <div class="profil-stat-lbl">sujets</div>
      </div>
      <div class="profil-stat">
        <div class="profil-stat-val">${e.posts_count}</div>
        <div class="profil-stat-lbl">réponses</div>
      </div>
    </div>`}window.setPosteType=function(e){document.getElementById("poste_est_actuel_input").value=e?"1":"0",document.getElementById("ptog-actuel").className="ptog "+(e?"ptog-on":"ptog-off"),document.getElementById("ptog-vise").className="ptog "+(e?"ptog-off":"ptog-on")};const oe=[{code:"BE",libelle:"Belgique"},{code:"LU",libelle:"Luxembourg"},{code:"DE",libelle:"Allemagne"},{code:"CH",libelle:"Suisse"},{code:"IT",libelle:"Italie"},{code:"MC",libelle:"Monaco"},{code:"ES",libelle:"Espagne"},{code:"AD",libelle:"Andorre"},{code:"GB",libelle:"Royaume-Uni"}],Be=oe.map(e=>`<option value="${e.code}">${u(e.libelle)}</option>`).join("");let ce=0;window.forgeAjouterPays=function(){const e=++ce,t=document.createElement("div");t.className="forge-ccn-row",t.id="forge-pays-"+e,t.innerHTML=`
    <select class="forge-pays-select">${Be}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprPays(${e})" title="Supprimer">×</button>`,document.getElementById("forge-pays-list").appendChild(t)};window.forgeSupprPays=function(e){document.getElementById("forge-pays-"+e)?.remove()};let re=0;window.forgeAjouterCcn=function(){const e=++re,t=document.createElement("div");t.className="forge-ccn-row",t.id="forge-ccn-"+e,t.innerHTML=`
    <select class="forge-ccn-select">${Ce}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprCcn(${e})" title="Supprimer">×</button>`,document.getElementById("forge-ccn-list").appendChild(t)};window.forgeSupprCcn=function(e){document.getElementById("forge-ccn-"+e)?.remove()};window.forgeSoumettre=async function(e){e.preventDefault();const t=document.getElementById("forge-form"),l=document.getElementById("forge-form-err"),i=document.getElementById("forge-submit-btn");l.textContent="";const s=[];document.querySelectorAll('[id^="forge-pays-"]').forEach(o=>{const m=o.querySelector(".forge-pays-select")?.value,d=o.querySelector(".forge-ccn-niveau")?.value,c=oe.find(g=>g.code===m);m&&c&&s.push({pays_code:m,pays_libelle:c.libelle,niveau:d})});const n=[];document.querySelectorAll('.forge-ccn-row:not([id^="forge-pays-"])').forEach(o=>{const m=o.querySelector(".forge-ccn-select").value,d=o.querySelector(".forge-ccn-niveau").value,c=ne.find(g=>g.idcc===m);m&&c&&n.push({ccn_idcc:m,ccn_libelle:c.libelle,niveau:d})});const a={email:t.querySelector('[name="email"]').value.trim(),pseudo:t.querySelector('[name="pseudo"]').value.trim(),poste:t.querySelector('[name="poste"]').value.trim(),linkedin_url:t.querySelector('[name="linkedin_url"]').value.trim()||null,poste_est_actuel:t.querySelector('[name="poste_est_actuel"]').value!=="0",paie_fr_niveau:t.querySelector('[name="paie_fr_niveau"]').value||null,pays:s,expertises:n};if(!a.email){l.textContent="Email requis.";return}if(!a.pseudo){l.textContent="Pseudo requis.";return}if(!a.poste){l.textContent="Poste requis.";return}i.disabled=!0,i.textContent="[ envoi… ]";try{const o=await fetch("/forge/profil",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(a)});if(!o.ok)throw new Error(`HTTP ${o.status} — ${await o.text()||o.statusText}`);const m=await o.json();x.unshift(m),t.reset(),document.getElementById("forge-pays-list").innerHTML="",document.getElementById("forge-ccn-list").innerHTML="",ce=0,re=0,Te(m.pseudo)}catch(o){l.textContent=M(o),i.disabled=!1,i.textContent="[ Rejoindre la Forge ]"}};const Fe=[{prenom:"Geralt",nom:"de Riv"},{prenom:"Sam",nom:"Vimes"},{prenom:"Elric",nom:"de Melniboné"},{prenom:"Druss",nom:"la Légende"},{prenom:"Logen",nom:"Neuf-Doigts"},{prenom:"Aragorn",nom:"Grands-Pas"},{prenom:"Jon",nom:"Shannow"},{prenom:"Salim",nom:"Dhibi"},{prenom:"Bayaz",nom:"le Magi"},{prenom:"Merlin",nom:"l'Enchanteur"}],Ne=[{prenom:"Lyra",nom:"Belacqua"},{prenom:"Hermione",nom:"Granger"},{prenom:"Eowyn",nom:"du Rohan"},{prenom:"Ellana",nom:"Caldin"},{prenom:"Ferro",nom:"Maljinn"},{prenom:"Magrat",nom:"Garlick"},{prenom:"Ewilan",nom:"Gil'Sayan"},{prenom:"Sigarni",nom:"la Guerrière"},{prenom:"Rikke",nom:"la Nord"},{prenom:"Tanaquil",nom:"la Magicienne"}],Z=[17,16,16,15,15,15,14,14,14,13,13,11],C=Z[Math.floor(Math.random()*Z.length)]/100;let Q="H",k=!1;function ee(e){return e[Math.floor(Math.random()*e.length)]}function de(e,t){["d-prenom","m-prenom"].forEach(l=>{const i=document.getElementById(l);i&&(i.value=e)}),["d-nom","m-nom"].forEach(l=>{const i=document.getElementById(l);i&&(i.value=t)}),k=!1}function ue(e,t=!1){const l=e==="H";["d-hf-h","m-hf-h"].forEach(i=>{document.getElementById(i)?.classList.toggle("ptog-on",l),document.getElementById(i)?.classList.toggle("ptog-off",!l)}),["d-hf-f","m-hf-f"].forEach(i=>{document.getElementById(i)?.classList.toggle("ptog-on",!l),document.getElementById(i)?.classList.toggle("ptog-off",l)}),t&&document.querySelectorAll(".genre-ecart-hint").forEach(i=>{i.textContent=l?i.dataset.textHf:i.dataset.textFh,i.style.display="inline"})}window.setGenre=function(e){if(e===Q)return;if(!k){const l=e==="F"?window._heroF:window._heroH;de(l.prenom,l.nom)}const t=e==="F"?1-C:1/(1-C);["d-brut","m-brut"].forEach(l=>{const i=document.getElementById(l);i&&(i.value=Math.round(parseFloat(i.value)*t))}),Q=e,ue(e,!0)};const D=document.getElementById("burger-btn"),B=document.getElementById("burger-menu");function qe(){D.classList.add("open"),B.classList.add("open")}window.closeBurger=function(){D.classList.remove("open"),B.classList.remove("open")};D.addEventListener("click",e=>{e.stopPropagation(),B.classList.contains("open")?closeBurger():qe()});document.addEventListener("click",()=>closeBurger());B.addEventListener("click",e=>e.stopPropagation());
