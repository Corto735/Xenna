(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const a of document.querySelectorAll('link[rel="modulepreload"]'))s(a);new MutationObserver(a=>{for(const n of a)if(n.type==="childList")for(const i of n.addedNodes)i.tagName==="LINK"&&i.rel==="modulepreload"&&s(i)}).observe(document,{childList:!0,subtree:!0});function l(a){const n={};return a.integrity&&(n.integrity=a.integrity),a.referrerPolicy&&(n.referrerPolicy=a.referrerPolicy),a.crossOrigin==="use-credentials"?n.credentials="include":a.crossOrigin==="anonymous"?n.credentials="omit":n.credentials="same-origin",n}function s(a){if(a.ep)return;a.ep=!0;const n=l(a);fetch(a.href,n)}})();const ye="modulepreload",he=function(e){return"/"+e},K={},Ee=function(t,l,s){let a=Promise.resolve();if(l&&l.length>0){let i=function(c){return Promise.all(c.map(r=>Promise.resolve(r).then(f=>({status:"fulfilled",value:f}),f=>({status:"rejected",reason:f}))))};document.getElementsByTagName("link");const o=document.querySelector("meta[property=csp-nonce]"),p=o?.nonce||o?.getAttribute("nonce");a=i(l.map(c=>{if(c=he(c),c in K)return;K[c]=!0;const r=c.endsWith(".css"),f=r?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${c}"]${f}`))return;const h=document.createElement("link");if(h.rel=r?"stylesheet":ye,r||(h.as="script"),h.crossOrigin="",h.href=c,p&&h.setAttribute("nonce",p),document.head.appendChild(h),r)return new Promise((v,g)=>{h.addEventListener("load",v),h.addEventListener("error",()=>g(new Error(`Unable to preload CSS for ${c}`)))})}))}function n(i){const o=new Event("vite:preloadError",{cancelable:!0});if(o.payload=i,window.dispatchEvent(o),!o.defaultPrevented)throw i}return a.then(i=>{for(const o of i||[])o.status==="rejected"&&n(o.reason);return t().catch(n)})};async function ae(e,t={}){if(window.__TAURI_INTERNALS__){const{invoke:s}=await Ee(async()=>{const{invoke:a}=await import("./core-DV6XEvTN.js");return{invoke:a}},[]);return s(e,t)}const l=await fetch(`/api/${e}`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(t)});if(!l.ok)throw await l.text();return l.json()}function T(e){if(e==null)return"(erreur nulle — redémarre l'app ou ouvre DevTools Ctrl+Shift+I)";if(typeof e=="string")return e||"(erreur muette — ouvre DevTools Ctrl+Shift+I et consulte la Console)";if(e instanceof Error)return e.message||e.toString();try{return JSON.stringify(e,null,2)}catch{return String(e)}}let H=null;const U="2026-01-31",ne=U;document.addEventListener("DOMContentLoaded",()=>{["d-date","m-date"].forEach(n=>{const i=document.getElementById(n);i&&(i.value=U,i.max=U)}),document.addEventListener("keydown",n=>{n.key==="Escape"&&closeFmModal()}),window._heroH=ie(Re),window._heroF=ie(qe),be(window._heroH.prenom,window._heroH.nom),fe("H"),["d-prenom","m-prenom","d-nom","m-nom"].forEach(n=>{document.getElementById(n)?.addEventListener("input",()=>{G=!0})});const e=Math.round(P*100),t=Math.round(P/(1-P)*100);document.querySelectorAll(".genre-ecart-hint").forEach(n=>{n.dataset.textFh=`// −${e} % · écart salarial F/H`,n.dataset.textHf=`// +${t} % · écart salarial H/F`});const l=window.matchMedia("(max-width: 680px)"),s=n=>{const i=document.body;!i.classList.contains("is-annuel")&&!i.classList.contains("is-forge")&&!i.classList.contains("is-apropos")&&!i.classList.contains("is-gaabrielle")&&!i.classList.contains("is-hercule")&&setView(n.matches?"mobile":"desktop")};l.addEventListener("change",s),s(l),localStorage.getItem("xenna-hv")&&(document.body.classList.add("hv-mode"),document.getElementById("hv-switch")?.classList.add("on")),localStorage.getItem("xenna-zoom")&&(document.body.classList.add("zoom-mode"),document.documentElement.style.zoom="200%",document.getElementById("zoom-switch")?.classList.add("on"),document.getElementById("a11y-magnifier")?.classList.add("active"));const a=localStorage.getItem("xenna-font");a&&setAppFont(a,!0),localStorage.getItem("xenna-hv")&&document.getElementById("a11y-hv-btn")?.classList.add("active"),localStorage.getItem("xenna-bw")&&(document.body.classList.add("bw-mode"),document.getElementById("bw-switch")?.classList.add("on"),document.getElementById("a11y-bw-btn")?.classList.add("active")),document.addEventListener("click",n=>{!n.target.closest("#a11y-btn")&&!n.target.closest("#a11y-panel")&&(document.getElementById("a11y-panel")?.classList.remove("open"),document.getElementById("a11y-btn")?.classList.remove("open"))})});let W="fr";const k={},C=new Map;function $e(){const e="script,style,input,select,textarea,.mob-val,.sb-val,.fm-val,.a11y-float,.trad-panel,#a11y-panel",t=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT,{acceptNode(s){const a=s.textContent.trim();return!a||a.length<2||/^[\d\s,.\-+%€×\/:()[\]]+$/.test(a)||s.parentElement?.closest(e)?NodeFilter.FILTER_REJECT:NodeFilter.FILTER_ACCEPT}}),l=[];for(;t.nextNode();)l.push(t.currentNode);return l}window.toggleTradPanel=function(){const e=document.getElementById("trad-panel"),t=document.getElementById("trad-btn"),l=e.classList.toggle("open");t.classList.toggle("open",l)};window.translateApp=async function(e){if(document.getElementById("trad-panel")?.classList.remove("open"),document.getElementById("trad-btn")?.classList.remove("open"),document.querySelectorAll(".trad-lang-btn").forEach(i=>i.classList.remove("active")),document.querySelector(`.trad-lang-btn[onclick="translateApp('${e}')"]`)?.classList.add("active"),e==="fr"){C.forEach((i,o)=>{o.isConnected&&(o.textContent=i)}),document.documentElement.lang="fr",W="fr";return}const t=document.getElementById("trad-btn");t.classList.add("loading"),t.textContent="🌐 …";const l=$e();l.forEach(i=>{C.has(i)||C.set(i,i.textContent)});const s=l.map(i=>C.get(i));k[e]||(k[e]=new Map);const a=k[e],n=[...new Set(s)].filter(i=>!a.has(i));try{if(n.length>0)for(let o=0;o<n.length;o+=20){const p=n.slice(o,o+20),c=p.join(`

`),r=`https://api.mymemory.translated.net/get?q=${encodeURIComponent(c)}&langpair=fr|${e}`,f=await fetch(r);if(!f.ok)throw new Error("HTTP "+f.status);const v=(await f.json()).responseData.translatedText.split(`

`);p.forEach((g,$)=>a.set(g,v[$]??g))}l.forEach(i=>{const o=C.get(i);i.isConnected&&a.has(o)&&(i.textContent=a.get(o))}),document.documentElement.lang=e,W=e}catch(i){console.error("Traduction échouée :",i),t.textContent="🌐 ✗",setTimeout(()=>{t.textContent="🌐 LANGUE",t.classList.remove("loading")},2e3);return}t.textContent="🌐 LANGUE",t.classList.remove("loading")};document.addEventListener("click",e=>{!e.target.closest("#trad-btn")&&!e.target.closest("#trad-panel")&&(document.getElementById("trad-panel")?.classList.remove("open"),document.getElementById("trad-btn")?.classList.remove("open"))});window.toggleA11yPanel=function(){const e=document.getElementById("a11y-panel"),t=document.getElementById("a11y-btn"),l=e.classList.toggle("open");t.classList.toggle("open",l)};window.toggleHVMode=function(){const e=document.body.classList.toggle("hv-mode");document.getElementById("hv-switch")?.classList.toggle("on",e),document.getElementById("a11y-hv-btn")?.classList.toggle("active",e),localStorage.setItem("xenna-hv",e?"1":"")};window.toggleZoom=function(){const e=document.body.classList.toggle("zoom-mode");document.documentElement.style.zoom=e?"200%":"",document.getElementById("zoom-switch")?.classList.toggle("on",e),document.getElementById("a11y-magnifier")?.classList.toggle("active",e),localStorage.setItem("xenna-zoom",e?"1":"")};const Z=new Set;window.setAppFont=function(e,t=!1){if(!e){document.body.classList.remove("custom-font"),document.documentElement.style.removeProperty("--app-font"),localStorage.removeItem("xenna-font");const a=document.getElementById("font-picker");a&&(a.value="");return}const l=e.replace(/ /g,"+");if(!Z.has(l)){const a=document.createElement("link");a.rel="stylesheet",a.href=`https://fonts.googleapis.com/css2?family=${l}&display=swap`,document.head.appendChild(a),Z.add(l)}document.documentElement.style.setProperty("--app-font",`'${e}', monospace`),document.body.classList.add("custom-font"),localStorage.setItem("xenna-font",e);const s=document.getElementById("font-picker");s&&t&&(s.value=e)};const L=[];window.scan67=function(){const e=Date.now();for(L.push(e);L.length&&e-L[0]>1500;)L.shift();const t=L.length>=3;t&&(L.length=0);const l=t?/42/:/67/,a=Array.from(document.querySelectorAll(".mob-val, .sb-val, .ascii-tbl td, .fm-val, .fm-result td")).filter(i=>l.test(i.textContent.replace(/[\s ]/g,""))&&i.offsetParent!==null);if(a.length===0)return;const n=document.getElementById("a11y-67-btn");if(n.classList.add("active"),t){const i=["#ff0055","#ff6600","#ffcc00","#00ff88","#00ccff","#aa00ff","#ff00cc","#39ff14","#ff4444","#44ffff","#ff69b4","#7fff00"];a.forEach((o,p)=>{setTimeout(()=>{const c=i[Math.floor(Math.random()*i.length)];Object.assign(o.style,{background:c,color:"#000",outline:`2px solid ${c}`,borderRadius:"2px",transition:"all 0.15s"}),setTimeout(()=>Object.assign(o.style,{background:"",color:"",outline:"",borderRadius:""}),900)},p*250)}),setTimeout(()=>n.classList.remove("active"),a.length*250+1e3)}else a.forEach((i,o)=>{setTimeout(()=>{i.classList.remove("flash-67"),i.offsetWidth,i.classList.add("flash-67"),i.addEventListener("animationend",()=>i.classList.remove("flash-67"),{once:!0})},o*500)}),setTimeout(()=>n.classList.remove("active"),a.length*500+200)};window.toggleBWMode=function(){const e=document.body.classList.toggle("bw-mode");document.getElementById("bw-switch")?.classList.toggle("on",e),document.getElementById("a11y-bw-btn")?.classList.toggle("active",e),localStorage.setItem("xenna-bw",e?"1":"")};function m(e){return String(e).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#039;")}window.setView=function(e){["mobile","desktop","annuel","forge","apropos","gaabrielle","hercule"].forEach(t=>document.body.classList.toggle("is-"+t,e===t)),document.getElementById("btn-desk").classList.toggle("active",e==="desktop"),document.getElementById("btn-mob").classList.toggle("active",e==="mobile"),document.getElementById("btn-ann").classList.toggle("active",e==="annuel"),H&&(e==="desktop"||e==="mobile")&&ce(H),e==="forge"&&Pe()};let D="EUR";function d(e){const t=parseFloat(e),l=D==="CHF"?" CHF":" €";return t.toLocaleString("fr-FR",{minimumFractionDigits:2,maximumFractionDigits:2})+l}function xe(e,t=!1){const l=parseFloat(e),s=D==="CHF"?" CHF":" €",a=l.toLocaleString("fr-FR",{minimumFractionDigits:2,maximumFractionDigits:2})+s;return t&&l>0?"+"+a:a}function F(e){return(parseFloat(e)*100).toFixed(2)+" %"}function oe(){const e=document.body.classList.contains("is-mobile")?"m-date":"d-date",t=e==="m-date"?"d-date":"m-date";return document.getElementById(e)?.value||document.getElementById(t)?.value||ne}function j(e){if(!e)return"—";const[t,l,s]=e.split("-");return`${s}/${l}/${t}`}const Ie=[{min:0,max:1620,taux:0},{min:1620,max:1683,taux:.005},{min:1683,max:1791,taux:.013},{min:1791,max:1911,taux:.021},{min:1911,max:2042,taux:.029},{min:2042,max:2151,taux:.035},{min:2151,max:2294,taux:.041},{min:2294,max:2714,taux:.053},{min:2714,max:3107,taux:.075},{min:3107,max:3539,taux:.099},{min:3539,max:3983,taux:.119},{min:3983,max:4648,taux:.138},{min:4648,max:5574,taux:.158},{min:5574,max:6974,taux:.179},{min:6974,max:8711,taux:.2},{min:8711,max:12091,taux:.24},{min:12091,max:16376,taux:.28},{min:16376,max:25706,taux:.33},{min:25706,max:55062,taux:.38},{min:55062,max:1/0,taux:.43}];function z(e){const t=parseFloat(e);if(isNaN(t)||t<=0)return{total:0,taux_effectif:0,details:[]};let l=0;const s=[];for(const a of Ie){if(t<=a.min)break;const i=+((a.max===1/0?t:Math.min(t,a.max))-a.min).toFixed(2),o=i*a.taux;if(s.push({min:a.min,max:a.max===1/0?null:a.max,taux:a.taux,base:i,montant:+o.toFixed(2)}),l+=o,a.max===1/0||t<=a.max)break}return{total:+l.toFixed(2),taux_effectif:t>0?l/t:0,details:s}}const Q={"Sécurité Sociale":"cat-ss","CSG/CRDS":"cat-csg","Retraite complémentaire":"cat-ret",Prévoyance:"cat-prev",Chômage:"cat-cho",Allègement:"cat-alleg","1er pilier":"cat-ss","Assurance chômage":"cat-cho","Assurance accidents":"cat-acc","Prévoyance maladie":"cat-prev","Prévoyance (LPP)":"cat-ret","Assurance pension":"cat-ret","Assurance maladie":"cat-ss","Assurance dépendance":"cat-prev","Mutualité des employeurs":"cat-ss"},M={},ee={SS_VIEILLESSE_PLAF:"min(Salaire brut, Plafond Mensuel Sécurité Sociale — PMSS)",CHOMAGE:"min(Salaire brut, 4 × PMSS)",AGS:"min(Salaire brut, 4 × PMSS)",CSG_DEDUCTIBLE:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels (CSS art. L136-2)",CSG_NON_DEDUCTIBLE:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels",CRDS:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels",AGIRC_ARRCO_T1:"min(Salaire brut, PMSS)  — Tranche 1 (entre 0 et 1 PMSS)",AGIRC_ARRCO_CEG_T1:"min(Salaire brut, PMSS)  — Tranche 1",PREVOYANCE_CADRE_MIN:"min(Salaire brut, PMSS)  — Tranche A",AGIRC_ARRCO_T2:"Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2",AGIRC_ARRCO_CEG_T2:"Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2"};function A(e,t=!0){const l=["f","(","x",")"].map((s,a)=>`<span style="animation-delay:${a*45}ms">${s}</span>`).join("");return t?`<span class="formula-star" data-fmkey="${e}" onclick="event.stopPropagation();showFormula('${e}')">${l}</span>`:`<span class="formula-star" aria-hidden="true">${l}</span>`}function N(e,t){if(e.code==="REDUCTION_FILLON")return`<pre class="fm-fillon">${m(e.explication)}</pre>`;const l=t==="sal",s=l?e.taux_sal:e.taux_pat,a=parseFloat(e.base),n=l?parseFloat(e.montant_sal):Math.abs(parseFloat(e.montant_pat)),i=l?"Taux salarial":"Taux patronal",o=l?"Montant salarial":t==="alleg"?"Montant allègement":"Montant patronal",p=l?"c-sal":t==="alleg"?"c-alleg":"c-pat",c=ee[e.code]?`<div class="fm-base-note">Assiette  =  ${m(ee[e.code])}</div>`:"";return`
    <div class="fm-generic">Montant  =  Assiette  ×  ${i}</div>
    ${c}
    <table class="fm-calc">
      <tr>
        <td>Assiette</td>
        <td class="fm-op">=</td>
        <td class="fm-val c-base">${d(a)}</td>
      </tr>
      <tr>
        <td>${i}</td>
        <td class="fm-op">×</td>
        <td class="fm-val c-taux">${F(s)}</td>
      </tr>
      <tr class="fm-result fm-sep">
        <td>${o}</td>
        <td class="fm-op">=</td>
        <td class="fm-val ${p}">${d(n)}</td>
      </tr>
    </table>`}function Se(e){const t=z(e);return`
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
      <tbody>${t.details.map(s=>{const a=s.min.toLocaleString("fr-FR")+" €",n=s.max===null?"∞":s.max.toLocaleString("fr-FR")+" €",i=s.taux===0;return`
      <tr class="${i?"pas-zero":""}">
        <td>${a} → ${n}</td>
        <td class="r">${d(s.base)}</td>
        <td class="r ${i?"c-dim":""}">${(s.taux*100).toFixed(1).replace(".",",")} %</td>
        <td class="r ${i?"c-dim":"c-purple"}">${i?"—":d(s.montant)}</td>
      </tr>`}).join("")}</tbody>
      <tfoot>
        <tr>
          <td>Net imposable</td>
          <td class="r c-gray">${d(e)}</td>
          <td class="r c-taux">${(t.taux_effectif*100).toFixed(2)} %&nbsp;<span style="color:var(--dim);font-size:0.7em">(taux effectif)</span></td>
          <td class="r c-purple" style="font-weight:bold">${d(t.total)}</td>
        </tr>
      </tfoot>
    </table>`}window.showFormula=function(e){const t=M[e];if(!t)return;const l=document.getElementById("fm-body");if(t.type==="pas"){document.getElementById("fm-title").textContent="Prélèvement à la Source (PAS)",document.getElementById("fm-badge").textContent="── Détail par tranche — barème neutre mensuel DGFIP ─────────",l.className="fm-type-pas",l.innerHTML=Se(t.netImposable),document.getElementById("fm-modal").classList.add("open"),document.querySelectorAll(`[data-fmkey="${e}"]`).forEach(o=>o.classList.add("visited"));return}const{c:s,type:a}=t,n=a==="sal",i=s.code==="REDUCTION_FILLON"?"── Allègement patronal ──────────────────────":n?"── Part salariale ───────────────────────────":"── Part patronale ───────────────────────────";document.getElementById("fm-title").textContent=s.libelle,document.getElementById("fm-badge").textContent=i,l.className=`fm-type-${a}`,l.innerHTML=N(s,a),document.getElementById("fm-modal").classList.add("open"),document.querySelectorAll(`[data-fmkey="${e}"]`).forEach(o=>o.classList.add("visited"))};window.closeFmModal=function(){document.getElementById("fm-modal").classList.remove("open")};window.toggleExpl=function(e){const t=document.getElementById(`row-${e}`),l=document.getElementById(`expl-${e}`);if(!t||!l)return;const s=l.style.display!=="none";l.style.display=s?"none":"table-row",t.classList.toggle("open",!s)};function Le(e){const t=document.getElementById("res-desktop"),l=e.cotisations,s=e.salarie?.pays&&e.salarie.pays!=="france",a=l.reduce((u,E)=>u+parseFloat(E.montant_sal),0),n=l.reduce((u,E)=>u+parseFloat(E.montant_pat),0),i=s?{total:0,taux_effectif:0}:z(e.net_imposable),o=parseFloat(e.net_a_payer)-i.total;s||(M.PAS={type:"pas",netImposable:parseFloat(e.net_imposable)});const p=`
    <div class="summary-bar">
      <div class="sb-cell">
        <div class="sb-lbl">▸ SALAIRE BRUT</div>
        <div class="sb-val c-gray">${d(e.brut)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ RETENUES</div>
        <div class="sb-ded">
          <div class="sb-ded-row">
            <span>Cot. salariales</span>
            <span style="color:var(--red)">− ${d(a)}</span>
          </div>
          ${s?"":`<div class="sb-ded-row">
            <span>PAS (${(i.taux_effectif*100).toFixed(1)} %)</span>
            <span style="color:var(--purple)">− ${d(i.total)}${A("PAS")}</span>
          </div>`}
          <div class="sb-ded-total">
            <span>Total retenues</span>
            <span style="color:var(--red)">− ${d(a+i.total)}</span>
          </div>
        </div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ NET À PAYER</div>
        <div class="sb-val c-green">${d(o)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ CHARGES PAT.</div>
        <div class="sb-val c-orange">${d(n)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ SUPER BRUT</div>
        <div class="sb-val c-yellow">${d(parseFloat(e.brut)+n)}</div>
      </div>
    </div>`,c=l.filter(u=>u.categorie!=="Allègement"&&(parseFloat(u.montant_sal)>0||u.taux_sal!=="0"||parseFloat(u.montant_pat)>0)),r=l.filter(u=>u.categorie==="Allègement"),f=c.reduce((u,E)=>u+parseFloat(E.montant_pat),0);function h(u,E){return u.map((y,x)=>{const I=E+x,S=Q[y.categorie]||"cat-ss",q=parseFloat(y.montant_sal)>0?"c-sal":"c-dim",O=parseFloat(y.montant_pat)>0?"c-pat":"c-dim",B=`${y.code}_sal`,Y=`${y.code}_pat`,X=parseFloat(y.montant_sal)>0,J=parseFloat(y.montant_pat)>0;X&&(M[B]={c:y,type:"sal"}),J&&(M[Y]={c:y,type:"pat"});const ge=A(B,X),ve=A(Y,J);return`
        <tr class="data-row" id="row-${I}" onclick="toggleExpl(${I})">
          <td>
            <span class="expand-icon">▶</span>
            <span class="cat ${S}">[${y.categorie}]</span>
            <span>${y.libelle}</span>
          </td>
          <td class="r">${d(y.base)}</td>
          <td class="r">${F(y.taux_sal)}</td>
          <td class="r ${q}">${d(y.montant_sal)}${ge}</td>
          <td class="r">${F(y.taux_pat)}</td>
          <td class="r ${O}">${d(y.montant_pat)}${ve}</td>
        </tr>
        <tr class="expl-row" id="expl-${I}" style="display:none">
          <td colspan="6">
            <div class="expl-box">
              <div class="expl-txt">▸ ${y.explication}</div>
              ${y.loi_ref?`<div class="expl-ref">§ ${y.loi_ref}</div>`:""}
            </div>
          </td>
        </tr>`}).join("")}const v=`
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
    </thead>`,g=`
    <div class="tbl-section-head">── COTISATIONS ────────────────────────────────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${v}
      <tbody>
        ${h(c,0)}
        <tr class="tbl-total">
          <td colspan="3">TOTAUX</td>
          <td class="r c-sal">− ${d(a)}</td>
          <td></td>
          <td class="r c-pat">+ ${d(f)}</td>
        </tr>
      </tbody>
    </table>`,$=`<div class="sim-period">
    SIMULATION AU <span class="sp-accent">${j(oe())}</span>
    &nbsp;·&nbsp; PMSS en vigueur calculé depuis la base de données
  </div>`,_=r.reduce((u,E)=>u+parseFloat(E.montant_pat),0),b=r.length===0?"":`
    <div class="tbl-section-head">── ALLÈGEMENTS PATRONAUX ───────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${v}
      <tbody>
        ${r.map((u,E)=>{const y=c.length+E,x=Q[u.categorie]||"cat-alleg",I=Math.abs(parseFloat(u.montant_pat)),S=`${u.code}_alleg`;return M[S]={c:u,type:"alleg"},`
            <tr class="data-row" id="row-${y}" onclick="toggleExpl(${y})">
              <td>
                <span class="expand-icon">▶</span>
                <span class="cat ${x}">[${u.categorie}]</span>
                <span>${u.libelle}</span>
              </td>
              <td class="r">${d(u.base)}</td>
              <td class="r"></td>
              <td class="r"></td>
              <td class="r c-alleg">${F(Math.abs(parseFloat(u.taux_pat)))}</td>
              <td class="r c-alleg">− ${d(I)}${A(S)}</td>
            </tr>
            <tr class="expl-row" id="expl-${y}" style="display:none">
              <td colspan="6">
                <div class="expl-box">
                  <div class="expl-txt">▸ ${u.explication}</div>
                  ${u.loi_ref?`<div class="expl-ref">§ ${u.loi_ref}</div>`:""}
                </div>
              </td>
            </tr>`}).join("")}
        <tr class="tbl-total">
          <td colspan="5">TOTAL ALLÈGEMENTS PATRONAUX</td>
          <td class="r c-alleg">− ${d(Math.abs(_))}</td>
        </tr>
      </tbody>
    </table>`;t.innerHTML=$+p+`<div class="tbl-wrap">${g}${b}</div>`}window.mobToggle=function(e,t){const l=document.getElementById("mob-expand-"+e),s=document.getElementById("mob-expand-"+e+"-why"),a=document.getElementById("mob-expand-"+e+"-how");if(!l)return;const n=l.style.display!=="none",i=l.dataset.panel;n?i===t?l.style.display="none":(l.dataset.panel=t,s.style.display=t==="why"?"block":"none",a.style.display=t==="how"?"block":"none"):(l.style.display="block",l.dataset.panel=t,s.style.display=t==="why"?"block":"none",a.style.display=t==="how"?"block":"none")};function we(e,t,l,s,a,n=0){const i=e.code==="REDUCTION_FILLON"?`<pre class="fm-fillon">${m(e.explication)}</pre>`:`<div class="fm-type-${a}">${N(e,a)}</div>`,o=`
    <div class="mob-exp-txt">${m(e.explication)}</div>
    ${e.loi_ref?`<div class="mob-exp-loi">§ ${m(e.loi_ref)}</div>`:""}`;return`
    <div class="${`mob-stripe-${a}-${n%2===0?"a":"b"}`}">
      <div class="mob-row">
        <span class="mob-lbl mob-cot-lbl"
              title="Explication et référence légale"
              onclick="mobToggle('${t}','why')">${m(e.libelle)}</span>
        <span class="mob-val ${s} mob-cot-amt"
              title="Formule de calcul"
              onclick="mobToggle('${t}','how')">${l}</span>
      </div>
      <div class="mob-expand" id="mob-expand-${t}" style="display:none">
        <div id="mob-expand-${t}-why">${o}</div>
        <div id="mob-expand-${t}-how" style="display:none">${i}</div>
      </div>
    </div>`}function _e(e){const t=document.getElementById("res-mobile"),l=document.getElementById("m-nom")?.value||document.getElementById("d-nom")?.value||"",s=document.getElementById("m-prenom")?.value||document.getElementById("d-prenom")?.value||"",a=e.cotisations,n=e.salarie?.pays&&e.salarie.pays!=="france",i=a.reduce((b,u)=>b+parseFloat(u.montant_sal),0),o=a.reduce((b,u)=>b+parseFloat(u.montant_pat),0),p=n?{total:0,taux_effectif:0}:z(e.net_imposable),c=parseFloat(e.net_a_payer)-p.total,r=parseFloat(e.brut)+o,f=a.filter(b=>b.categorie!=="Allègement"&&(parseFloat(b.montant_sal)>0||b.taux_sal!=="0"||parseFloat(b.montant_pat)>0)),h=a.filter(b=>b.categorie==="Allègement"),v=f.reduce((b,u)=>b+parseFloat(u.montant_pat),0),g=h.reduce((b,u)=>b+parseFloat(u.montant_pat),0),$=f.map((b,u)=>{const E=parseFloat(b.montant_sal)>0,y=parseFloat(b.montant_pat)>0,x=`${b.code}_u`,I=b.code==="REDUCTION_FILLON"?`<pre class="fm-fillon">${m(b.explication)}</pre>`:[E?`<div class="fm-type-sal">${N(b,"sal")}</div>`:"",y?`<div class="fm-type-pat">${N(b,"pat")}</div>`:""].join(""),S=`
      <div class="mob-exp-txt">${m(b.explication)}</div>
      ${b.loi_ref?`<div class="mob-exp-loi">§ ${m(b.loi_ref)}</div>`:""}`,q=`mob-stripe-sal-${u%2===0?"a":"b"}`,O=E?`<span class="mob-val c-red mob-cot-amt" onclick="mobToggle('${x}','how')">− ${d(b.montant_sal)}</span>`:"",B=y?`<span class="mob-val c-orange mob-cot-amt" onclick="mobToggle('${x}','how')">+ ${d(b.montant_pat)}</span>`:"";return`
      <div class="${q}">
        <div class="mob-row">
          <span class="mob-lbl mob-cot-lbl" onclick="mobToggle('${x}','why')">${m(b.libelle)}</span>
          <span style="display:flex;flex-direction:column;align-items:flex-end;gap:0.1rem">${O}${B}</span>
        </div>
        <div class="mob-expand" id="mob-expand-${x}" style="display:none">
          <div id="mob-expand-${x}-why">${S}</div>
          <div id="mob-expand-${x}-how" style="display:none">${I}</div>
        </div>
      </div>`}).join(""),_=h.map((b,u)=>we(b,`${b.code}_alleg`,`− ${d(Math.abs(parseFloat(b.montant_pat)))}`,"c-alleg","alleg",u)).join("");t.innerHTML=`
    <div class="mob-bulletin">

      <!-- En-tête bulletin -->
      <div class="mob-head">
        <span class="mob-head-title">BULLETIN DE PAYE</span>
        <div style="text-align:right">
          <div class="mob-head-name">${m(s)} ${m(l).toUpperCase()}</div>
          <div class="mob-head-date">simulation au ${j(oe())}</div>
        </div>
      </div>

      <!-- Brut -->
      <div class="mob-row" style="margin-top:0.15rem">
        <span class="mob-lbl">Salaire de base brut</span>
        <span class="mob-val c-gray">${d(e.brut)}</span>
      </div>

      <!-- Cotisations unifiées (salariales + patronales sur une ligne) -->
      <div class="mob-row section"><span class="mob-lbl">── COTISATIONS ──</span><span style="display:flex;gap:1.5rem;font-size:0.62rem;color:var(--muted)"><span>SAL.</span><span>PAT.</span></span></div>
      ${$}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL retenues salariales</span>
        <span class="mob-val c-red">− ${d(i)}</span>
      </div>
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL charges patronales</span>
        <span class="mob-val c-orange">+ ${d(v)}</span>
      </div>

      <!-- Net imposable (France seulement) -->
      ${n?"":`<div class="mob-row net-row">
        <span class="mob-lbl">NET IMPOSABLE</span>
        <span class="mob-val c-green">${d(e.net_imposable)}</span>
      </div>`}

      <!-- PAS (France seulement) -->
      ${n?"":`<div class="mob-row pas-row">
        <span class="mob-lbl">Prélèvement à la source (${(p.taux_effectif*100).toFixed(1)} %)</span>
        <span class="mob-val c-purple">− ${d(p.total)}${A("PAS")}</span>
      </div>`}

      <!-- Net à payer -->
      <div class="mob-row final-row">
        <span class="mob-lbl">NET À PAYER</span>
        <span class="mob-val c-green">${d(c)}</span>
      </div>

      <!-- Allègements -->
      ${_.length?`
      <div class="mob-row section"><span class="mob-lbl">── ALLÈGEMENTS PATRONAUX ──</span><span></span></div>
      ${_}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL allègements</span>
        <span class="mob-val c-alleg">− ${d(Math.abs(g))}</span>
      </div>`:""}

      <!-- Super brut -->
      <div class="mob-row superbrut">
        <span class="mob-lbl">SUPER BRUT (coût employeur)</span>
        <span class="mob-val c-blue">${d(r)}</span>
      </div>

    </div>`}function ce(e){D=e.devise||"EUR",Le(e),_e(e)}function te(e){const t=`<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">⚠ ${m(e)}</div>`;document.getElementById("res-desktop").innerHTML=t,document.getElementById("res-mobile").innerHTML=t}async function re(e){const t=e==="mobile",l=document.getElementById(t?"m-brut":"d-brut").value,s=document.getElementById(t?"m-statut":"d-statut").value,a=document.getElementById(t?"m-nom":"d-nom").value||"Dupont",n=document.getElementById(t?"m-prenom":"d-prenom").value||"Marie",i=document.getElementById(t?"m-date":"d-date").value||ne,o=document.getElementById(t?"m-alsace-moselle":"d-alsace-moselle")?.checked??!1,p=document.getElementById(t?"m-suisse":"d-suisse")?.checked??!1,c=document.getElementById(t?"m-luxembourg":"d-luxembourg")?.checked??!1,r=parseFloat(l);if(!l||isNaN(r)||r<=0){te("Salaire brut invalide — saisir un montant positif.");return}if(!/^\d{4}-\d{2}-\d{2}$/.test(i)){te(`Date invalide : '${i}' (format attendu : YYYY-MM-DD).`);return}["d-brut","m-brut"].forEach(v=>{const g=document.getElementById(v);g&&(g.value=l)}),["d-statut","m-statut"].forEach(v=>{const g=document.getElementById(v);g&&(g.value=s)}),["d-nom","m-nom"].forEach(v=>{const g=document.getElementById(v);g&&(g.value=a)}),["d-prenom","m-prenom"].forEach(v=>{const g=document.getElementById(v);g&&(g.value=n)}),["d-date","m-date"].forEach(v=>{const g=document.getElementById(v);g&&(g.value=i)});const f=p?"suisse":c?"luxembourg":null,h=f?"2026-01-01":i;try{const v=await ae("calculer_bulletin",{salarie:{nom:a,prenom:n,salaire_brut:l.toString(),statut:s,alsace_moselle:o,pays:f??"france"},datePaie:h});H=v,ce(v)}catch(v){console.error("[calculer_bulletin] erreur brute :",v);const g=T(v),$=`<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">ERREUR : ${m(g)}</div>`;document.getElementById("res-desktop").innerHTML=$,document.getElementById("res-mobile").innerHTML=$}}function Ce(e){const t=document.getElementById("res-annuel"),l=e.lignes,s=l.map(r=>r.smic),a=`
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
    </tr></thead>`,n=l.map((r,f)=>{const h=f>0&&r.smic!==s[f-1],v=r.mois_libelle.includes("13e"),g=parseFloat(r.fillon_regularise)-parseFloat(r.fillon_simple),$=Math.abs(g)<.005?'<span style="color:var(--dim)">—</span>':`<span class="delta-nonzero">${g>0?"+":""}${xe(g.toFixed(2))}</span>`;return`<tr class="${[h?"smic-change":"",v?"treizieme-mois":""].filter(Boolean).join(" ")}">
      <td>${r.mois_libelle}</td>
      <td>${d(r.smic)}</td>
      <td>${d(r.brut)}</td>
      <td class="c-sal">− ${d(r.total_sal)}</td>
      <td class="c-pat">+ ${d(r.total_pat_brut)}</td>
      <td class="c-alleg">− ${d(r.fillon_regularise)}</td>
      <td>${$}</td>
      <td class="c-green">${d(r.net_a_payer)}</td>
      <td class="c-yellow">${d(r.cout_employeur)}</td>
    </tr>`}).join(""),i=`
    <tr class="ann-total">
      <td>TOTAL ${e.annee}</td>
      <td></td>
      <td>${d(e.total_brut)}</td>
      <td class="c-sal">− ${d(e.total_sal)}</td>
      <td class="c-pat">+ ${d(e.total_pat_brut)}</td>
      <td class="c-alleg">− ${d(e.total_fillon)}</td>
      <td></td>
      <td class="c-green">${d(e.total_net)}</td>
      <td class="c-yellow">${d(e.total_cout)}</td>
    </tr>`,o=parseFloat(e.total_pat_brut),p=parseFloat(e.total_fillon),c=`
    <div style="display:flex;gap:1rem;flex-wrap:wrap;margin-top:0.75rem;font-size:0.72rem">
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">ÉCONOMIE FILLON (annuelle)</div>
        <div style="color:var(--green);font-size:1.1rem;font-weight:bold">− ${d(e.total_fillon)}</div>
      </div>
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">TAUX FILLON MOYEN</div>
        <div style="color:var(--blue);font-size:1.1rem;font-weight:bold">
          ${o>0?(p/parseFloat(e.total_brut)*100).toFixed(2)+" %":"—"}
        </div>
      </div>
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">COÛT EMPLOYEUR ANNUEL</div>
        <div style="color:var(--yellow);font-size:1.1rem;font-weight:bold">${d(e.total_cout)}</div>
      </div>
    </div>`;t.innerHTML=`
    <div class="tbl-section-head">── SIMULATION ANNUELLE ${e.annee} ────────────────────────────────────</div>
    <div style="font-size:0.70rem;color:var(--muted);margin-bottom:0.4rem">
      Décembre inclut un 13e mois (salaire doublé). Brut total = 13 mois. Fillon régularisé sur rémunération annuelle réelle.
    </div>
    <table class="ann-tbl">
      ${a}
      <tbody>${n}</tbody>
      ${i}
    </table>
    ${c}`}async function Me(){const e=parseInt(document.getElementById("a-annee").value),t=document.getElementById("a-brut").value,l=document.getElementById("a-statut").value,s=document.getElementById("res-annuel");if(isNaN(e)||e<1900||e>2100){s.innerHTML='<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Année invalide.</div>';return}const a=parseFloat(t);if(!t||isNaN(a)||a<=0){s.innerHTML='<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Salaire brut invalide — saisir un montant positif.</div>';return}s.innerHTML='<div style="color:var(--muted);padding:1rem;font-size:0.78rem">Calcul en cours…</div>';try{const n=await ae("simuler_annee",{annee:e,salaireBrut:t.toString(),statut:l});Ce(n)}catch(n){console.error("[simuler_annee] erreur brute :",n),s.innerHTML=`<div style="padding:1rem;color:var(--red);font-size:0.8rem">ERREUR : ${m(T(n))}</div>`}}window.onTogglePays=function(e,t){const l=["suisse","luxembourg"].filter(c=>c!==e);t&&l.forEach(c=>{["d","m"].forEach(r=>{const f=document.getElementById(`${r}-${c}`);f&&f.checked&&(f.checked=!1)})});const s=["suisse","luxembourg"].some(c=>document.getElementById(`d-${c}`)?.checked);["d","m"].forEach(c=>{const r=document.getElementById(`${c}-alsace-moselle-wrap`);r&&(r.style.display=s?"none":"");const f=document.getElementById(`${c}-alsace-moselle`);f&&s&&(f.checked=!1)}),["d-date","m-date"].forEach(c=>{const r=document.getElementById(c);r&&(r.disabled=s,s&&(r.value="2026-01-01"))});const a=document.getElementById("d-suisse")?.checked,n=a?"SALAIRE BRUT (CHF)":"SALAIRE BRUT (€)",i=a?"BRUT (CHF)":"BRUT (€)",o=document.getElementById("d-brut");if(o){const c=o.closest(".field")?.querySelector("label");c&&(c.textContent=n)}const p=document.getElementById("m-brut");if(p){const c=p.closest(".field")?.querySelector("label");c&&(c.textContent=i)}};window.toggleParams=function(e){const t=document.getElementById(`${e}-params`),l=document.getElementById(`${e}-params-toggle`);if(!t)return;const s=t.style.display!=="none";t.style.display=s?"none":"block",l.classList.toggle("open",!s)};window.syncParam=function(e,t){["d","m"].forEach(l=>{const s=document.getElementById(`${l}-${e}`);s&&s.checked!==t&&(s.checked=t)})};document.getElementById("d-calc").addEventListener("click",()=>re("desktop"));document.getElementById("m-calc").addEventListener("click",()=>re("mobile"));document.getElementById("a-calc").addEventListener("click",Me);const de=[{idcc:"1261",libelle:"Acteurs du lien social et familial (ALISFA)"},{idcc:"2941",libelle:"Aide, accompagnement, soins et services à domicile"},{idcc:"1747",libelle:"Activités industrielles de boulangerie et de pâtisserie"},{idcc:"2149",libelle:"Activités du déchet"},{idcc:"2335",libelle:"Agences générales d'assurances"},{idcc:"1686",libelle:"Audiovisuel, électronique et équipement ménager"},{idcc:"2120",libelle:"Banque"},{idcc:"3210",libelle:"Banque Populaire"},{idcc:"0567",libelle:"Bijouterie, joaillerie, orfèvrerie (obsolète)"},{idcc:"0158",libelle:"Bois et scieries"},{idcc:"0992",libelle:"Boucherie"},{idcc:"0843",libelle:"Boulangerie-pâtisserie artisanales"},{idcc:"1606",libelle:"Bricolage"},{idcc:"1486",libelle:"Bureaux d'études techniques et sociétés de conseils (Syntec)"},{idcc:"0787",libelle:"Cabinets d'experts-comptables et de commissaires aux comptes"},{idcc:"2332",libelle:"Cabinets d'architectes"},{idcc:"1619",libelle:"Cabinets dentaires"},{idcc:"2420",libelle:"Cadres du bâtiment"},{idcc:"3212",libelle:"Cadres des travaux publics"},{idcc:"1256",libelle:"Cadres des entreprises de gestion d'équipements thermiques et de climatisation"},{idcc:"0211",libelle:"Cadres des industries de carrières et matériaux (obsolète)"},{idcc:"0045",libelle:"Caoutchouc"},{idcc:"2257",libelle:"Casinos"},{idcc:"0783",libelle:"Centres d'hébergement et de réadaptation sociale"},{idcc:"0953",libelle:"Charcuterie de détail"},{idcc:"1580",libelle:"Chaussure"},{idcc:"2060",libelle:"Chaînes de cafétérias"},{idcc:"1557",libelle:"Commerce des articles de sports et d'équipements de loisirs"},{idcc:"2216",libelle:"Commerce de détail et de gros à prédominance alimentaire"},{idcc:"1505",libelle:"Commerce de détail alimentaire non spécialisé"},{idcc:"2198",libelle:"Commerce à distance et E-commerce"},{idcc:"1483",libelle:"Commerce de détail de l'habillement"},{idcc:"1487",libelle:"Commerce de détail de l'horlogerie-bijouterie"},{idcc:"3237",libelle:"Commerce de détail alimentaire spécialisé"},{idcc:"1225",libelle:"Commerce de la Réunion"},{idcc:"0468",libelle:"Commerce succursaliste de la chaussure"},{idcc:"0573",libelle:"Commerces de gros"},{idcc:"1517",libelle:"Commerces de détail non alimentaires (Codena)"},{idcc:"0500",libelle:"Commerces de gros de l'habillement, mercerie, chaussure et jouet"},{idcc:"3243",libelle:"Commerces de quincaillerie, fournitures industrielles, fers, métaux et équipement de la maison"},{idcc:"2596",libelle:"Coiffure"},{idcc:"1611",libelle:"Communication écrite directe"},{idcc:"1286",libelle:"Confiserie, chocolaterie, biscuiterie"},{idcc:"2583",libelle:"Concessionnaires et exploitants d'autoroutes ou d'ouvrages routiers"},{idcc:"3217",libelle:"Convention collective nationale de la branche ferroviaire"},{idcc:"2272",libelle:"Convention collective nationale de l'assainissement et de la maintenance industrielle"},{idcc:"2002",libelle:"Convention collective interrégionale de la blanchisserie, laverie, location de linge, nettoyage à sec, pressing et teinturerie du 17 novembre 1997"},{idcc:"2247",libelle:"Courtage d'assurances et/ou de réassurances"},{idcc:"0303",libelle:"Couture parisienne et autres métiers de la mode"},{idcc:"0733",libelle:"Détaillants en chaussures"},{idcc:"1605",libelle:"Désinfection, désinsectisation, dératisation"},{idcc:"1536",libelle:"Distributeurs conseils hors domicile"},{idcc:"2372",libelle:"Distribution directe"},{idcc:"1408",libelle:"Distribution, Logistique et Services des Energies de Proximité"},{idcc:"2121",libelle:"Édition"},{idcc:"1518",libelle:"Education, culture, loisirs et animation agissant pour l'utilité sociale et environnementale, au service des territoires (ECLAT)"},{idcc:"2609",libelle:"Employés, techniciens et agents de maîtrise du bâtiment"},{idcc:"2614",libelle:"Employés, techniciens et agents de maîtrise des travaux publics"},{idcc:"0135",libelle:"Employés techniciens et agents de maîtrise des industries de carrières et de matériaux (obsolète)"},{idcc:"3218",libelle:"Enseignement privé non lucratif"},{idcc:"2691",libelle:"Enseignement privé hors contrat"},{idcc:"3043",libelle:"Entreprises de propreté"},{idcc:"3127",libelle:"Entreprises de services à la personne"},{idcc:"1285",libelle:"Entreprises artistiques et culturelles"},{idcc:"1539",libelle:"Entreprises du bureau et du numérique - Commerces et services (Eben)"},{idcc:"1412",libelle:"Entreprises d'installation sans fabrication de matériel aéraulique, thermique, frigorifique"},{idcc:"2717",libelle:"Entreprises techniques au service de la création et de l'évènement"},{idcc:"3032",libelle:"Esthétique"},{idcc:"0029",libelle:"Établissements privés d'hospitalisation, de soins, de cure et de garde à but non lucratif (CCN 51 - FEHAP)"},{idcc:"0413",libelle:"Établissements et services pour personnes inadaptées et handicapées (CCN 66)"},{idcc:"0405",libelle:"Établissements médico-sociaux de l'union intersyndicale des secteurs sanitaires et sociaux (CCN 65)"},{idcc:"0478",libelle:"Établissements financiers"},{idcc:"0915",libelle:"Expertises en matière d'évaluations industrielles et commerciales"},{idcc:"1307",libelle:"Exploitation cinématographique"},{idcc:"1405",libelle:"Expédition et exportation de fruits et légumes"},{idcc:"1411",libelle:"Fabrication de l'ameublement"},{idcc:"0669",libelle:"Fabrication mécanique du verre"},{idcc:"1821",libelle:"Fabrication du verre à la main, semi-automatique et mixte"},{idcc:"1031",libelle:"Fédération nationale des associations familiales rurales"},{idcc:"1978",libelle:"Fleuristes, vente et services des animaux familiers"},{idcc:"0200",libelle:"Froid"},{idcc:"1043",libelle:"Gardiens d'immeubles"},{idcc:"2543",libelle:"Géomètres et experts-fonciers"},{idcc:"2021",libelle:"Golf"},{idcc:"2156",libelle:"Grands magasins"},{idcc:"2336",libelle:"Habitat et du Logement Accompagnés"},{idcc:"1631",libelle:"Hôtellerie de plein air"},{idcc:"1979",libelle:"Hôtels, cafés, restaurants (HCR)"},{idcc:"2264",libelle:"Hospitalisation privée (FHP)"},{idcc:"1921",libelle:"Huissiers de justice"},{idcc:"0044",libelle:"Industries chimiques"},{idcc:"1534",libelle:"Industrie et commerces en gros des viandes"},{idcc:"3233",libelle:"Industrie de la fabrication des ciments"},{idcc:"2089",libelle:"Industrie des panneaux à base de bois"},{idcc:"0176",libelle:"Industrie pharmaceutique"},{idcc:"1388",libelle:"Industrie du pétrole"},{idcc:"0112",libelle:"Industrie laitière"},{idcc:"0018",libelle:"Industrie textile"},{idcc:"3236",libelle:"Industrie et services nautiques"},{idcc:"3109",libelle:"Industries alimentaires diverses"},{idcc:"0247",libelle:"Industries de l'habillement"},{idcc:"2542",libelle:"Industries métallurgiques, mécaniques et connexes de l'Aisne (obsolète)"},{idcc:"3209",libelle:"Industries métallurgiques, mécaniques et connexes du Doubs (obsolète)"},{idcc:"2003",libelle:"Industries métallurgiques, électriques et électroniques des Vosges (obsolète)"},{idcc:"2630",libelle:"Industries métallurgiques des Bouches-du-Rhône et Alpes-de-Haute-Provence (obsolète)"},{idcc:"1396",libelle:"Industries de produits alimentaires élaborés"},{idcc:"0489",libelle:"Industries du cartonnage"},{idcc:"0637",libelle:"Industries et commerce de la récupération"},{idcc:"1938",libelle:"Industries de la transformation des volailles"},{idcc:"1586",libelle:"Industries charcutières"},{idcc:"0184",libelle:"Imprimerie de labeur et industries graphiques"},{idcc:"0043",libelle:"Import-export et commerce international"},{idcc:"1527",libelle:"Immobilier"},{idcc:"0650",libelle:"Ingénieurs et cadres de la métallurgie (obsolète)"},{idcc:"1679",libelle:"Inspection d'assurance"},{idcc:"1794",libelle:"Institutions de retraite complémentaire"},{idcc:"1760",libelle:"Jardineries et graineteries"},{idcc:"1480",libelle:"Journalistes"},{idcc:"0959",libelle:"Laboratoires de biologie médicale extra-hospitaliers"},{idcc:"3013",libelle:"Librairie"},{idcc:"1404",libelle:"Machines et matériels agricoles et de travaux publics (SDLM)"},{idcc:"0675",libelle:"Maisons à succursales de vente au détail d'habillement"},{idcc:"0538",libelle:"Manutention ferroviaire"},{idcc:"2528",libelle:"Maroquinerie"},{idcc:"1589",libelle:"Mareyeurs-expéditeurs"},{idcc:"2931",libelle:"Marchés financiers"},{idcc:"3222",libelle:"Menuiseries charpentes et constructions industrialisées et des portes planes"},{idcc:"0822",libelle:"Mensuels de la métallurgie de la Savoie (obsolète)"},{idcc:"1387",libelle:"Mensuels de la métallurgie des Flandres (obsolète)"},{idcc:"0914",libelle:"Mensuels de la métallurgie de l'Ain (obsolète)"},{idcc:"1930",libelle:"Meunerie"},{idcc:"2190",libelle:"Missions locales et PAIO des maisons de l'emploi et PLIE"},{idcc:"1499",libelle:"Miroiterie, transformation et négoce du verre"},{idcc:"0827",libelle:"Métallurgie des Ardennes (obsolète)"},{idcc:"0863",libelle:"Métallurgie d'Ille-et-Vilaine et du Morbihan (obsolète)"},{idcc:"1867",libelle:"Métallurgie de la Drôme et de l'Ardèche (obsolète)"},{idcc:"0984",libelle:"Métallurgie d'Eure-et-Loir (obsolète)"},{idcc:"2992",libelle:"Métallurgie d'Indre-et-Loire (obsolète)"},{idcc:"0898",libelle:"Métallurgie de l'Allier (obsolète)"},{idcc:"1572",libelle:"Métallurgie de la Charente (obsolète)"},{idcc:"1885",libelle:"Métallurgie de la Côte-d'Or (obsolète)"},{idcc:"1635",libelle:"Métallurgie de la Gironde et des Landes (obsolète)"},{idcc:"1578",libelle:"Métallurgie de la Loire et de l'arrondissement d'Yssingeaux (obsolète)"},{idcc:"0828",libelle:"Métallurgie de la Manche (obsolète)"},{idcc:"0899",libelle:"Métallurgie de la Marne (obsolète)"},{idcc:"1813",libelle:"Métallurgie de la région de Maubeuge (obsolète)"},{idcc:"1525",libelle:"Métallurgie de la région dunkerquoise (obsolète)"},{idcc:"0930",libelle:"Métallurgie de la Sarthe (obsolète)"},{idcc:"0920",libelle:"Métallurgie de la Vienne (obsolète)"},{idcc:"3053",libelle:"Métallurgie de Haute-Saône (obsolète)"},{idcc:"1576",libelle:"Métallurgie du Cher (obsolète)"},{idcc:"0943",libelle:"Métallurgie du Calvados (obsolète)"},{idcc:"0860",libelle:"Métallurgie du Finistère (obsolète)"},{idcc:"2126",libelle:"Métallurgie du Gard et de la Lozère (obsolète)"},{idcc:"1912",libelle:"Métallurgie du Haut-Rhin (obsolète)"},{idcc:"0836",libelle:"Métallurgie de la Haute-Savoie (obsolète)"},{idcc:"0937",libelle:"Métallurgie de la Haute-Vienne et de la Creuse (obsolète)"},{idcc:"1577",libelle:"Métallurgie de l'Hérault, de l'Aude et des Pyrénées-Orientales (obsolète)"},{idcc:"2221",libelle:"Métallurgie de l'Isère et des Hautes-Alpes"},{idcc:"1369",libelle:"Métallurgie de Loire-Atlantique (obsolète)"},{idcc:"2579",libelle:"Métallurgie du Loir-et-Cher (obsolète)"},{idcc:"1966",libelle:"Métallurgie du Loiret (obsolète)"},{idcc:"1902",libelle:"Métallurgie du Maine-et-Loire (obsolète)"},{idcc:"2266",libelle:"Métallurgie de la Mayenne (obsolète)"},{idcc:"1365",libelle:"Métallurgie de Meurthe-et-Moselle (obsolète)"},{idcc:"2755",libelle:"Industries de la métallurgie de Belfort/Montbéliard (obsolète)"},{idcc:"1059",libelle:"Métallurgie des Midi-Pyrénées (obsolète)"},{idcc:"0714",libelle:"Métallurgie de la Moselle (obsolète)"},{idcc:"0948",libelle:"Métallurgie de l'Orne (obsolète)"},{idcc:"2700",libelle:"Métallurgie de l'Oise (obsolète)"},{idcc:"1472",libelle:"Métallurgie du Pas-de-Calais (obsolète)"},{idcc:"2615",libelle:"Métallurgie des Pyrénées-Atlantiques et du Seignanx (obsolète)"},{idcc:"0878",libelle:"Métallurgie du Rhône (obsolète)"},{idcc:"1604",libelle:"Métallurgie de Rouen et de Dieppe (obsolète)"},{idcc:"1564",libelle:"Métallurgie de Saône-et-Loire (obsolète)"},{idcc:"0911",libelle:"Métallurgie de Seine-et-Marne (obsolète)"},{idcc:"2980",libelle:"Métallurgie de la Somme (obsolète)"},{idcc:"1592",libelle:"Métallurgie du Valenciennois et du Cambrésis (obsolète)"},{idcc:"2489",libelle:"Métallurgie de la Vendée (obsolète)"},{idcc:"1634",libelle:"Métallurgie des Côtes-d'Armor (obsolète)"},{idcc:"2630",libelle:"Métallurgie des Bouches-du-Rhône (obsolète)"},{idcc:"1315",libelle:"Industries métallurgiques et mécaniques de la Haute-Marne et de la Meuse (obsolète)"},{idcc:"1732",libelle:"Métallurgie de l'Yonne (obsolète)"},{idcc:"1560",libelle:"Métallurgiques des Alpes-Maritimes (obsolète)"},{idcc:"0979",libelle:"Métallurgiques de l'arrondissement du Havre (obsolète)"},{idcc:"2128",libelle:"Mutualité"},{idcc:"1077",libelle:"Négoce et industrie des produits du sol, engrais et produits connexes"},{idcc:"1880",libelle:"Négoce de l'ameublement"},{idcc:"1982",libelle:"Négoce et prestations de services dans les domaines médico-techniques"},{idcc:"1947",libelle:"Négoce de bois d'oeuvre et produits dérivés (obsolète)"},{idcc:"0054",libelle:"Non-cadres des industries métallurgiques et mécaniques de la région parisienne (obsolète)"},{idcc:"0998",libelle:"Non-cadres de l'exploitation d'équipements thermiques et de génie climatique"},{idcc:"2205",libelle:"Notaires"},{idcc:"3220",libelle:"Offices publics de l'habitat"},{idcc:"3245",libelle:"Opérateurs de voyages et guides"},{idcc:"1431",libelle:"Optique-lunetterie de détail"},{idcc:"1316",libelle:"Organismes de tourisme social et familial"},{idcc:"1909",libelle:"Organismes de tourisme"},{idcc:"1516",libelle:"Organismes de formation"},{idcc:"1790",libelle:"Parcs de loisirs et d'attractions"},{idcc:"1267",libelle:"Pâtisserie"},{idcc:"1000",libelle:"Personnel des cabinets d'avocats"},{idcc:"1147",libelle:"Personnel des cabinets médicaux"},{idcc:"0275",libelle:"Personnel au sol du transport aérien"},{idcc:"2046",libelle:"Personnel non médical des centres de lutte contre le cancer"},{idcc:"2972",libelle:"Personnel sédentaire des entreprises de navigation"},{idcc:"1558",libelle:"Personnel des industries céramiques"},{idcc:"1996",libelle:"Pharmacie d'officine"},{idcc:"1504",libelle:"Poissonnerie"},{idcc:"0759",libelle:"Pompes funèbres"},{idcc:"2683",libelle:"Portage de presse"},{idcc:"3017",libelle:"Ports et Manutention"},{idcc:"3230",libelle:"Presse (Information spécialisée [ETAM et cadres])"},{idcc:"3242",libelle:"Presse quotidienne et hebdomadaire en régions"},{idcc:"2098",libelle:"Prestataires de services du secteur tertiaire"},{idcc:"1351",libelle:"Prévention et sécurité"},{idcc:"1512",libelle:"Promotion immobilière"},{idcc:"0292",libelle:"Plasturgie"},{idcc:"3168",libelle:"Professions de la photographie"},{idcc:"3244",libelle:"Professions réglementées auprès des juridictions"},{idcc:"1555",libelle:"Produits à usage pharmaceutique, parapharmaceutique et vétérinaire"},{idcc:"1513",libelle:"Production des eaux embouteillées, des boissons rafraîchissantes sans alcool et de bière"},{idcc:"2642",libelle:"Production audiovisuelle"},{idcc:"3238",libelle:"Production et transformation des papiers et cartons"},{idcc:"0653",libelle:"Producteurs salariés de base des services extérieurs de production des sociétés d'assurances"},{idcc:"0993",libelle:"Prothèse dentaire"},{idcc:"0086",libelle:"Publicité"},{idcc:"1621",libelle:"Répartition pharmaceutique"},{idcc:"0454",libelle:"Remontées mécaniques et domaines skiables"},{idcc:"1266",libelle:"Restauration de collectivités"},{idcc:"1501",libelle:"Restauration rapide"},{idcc:"1413",libelle:"Salariés permanents des entreprises de travail temporaire"},{idcc:"3216",libelle:"Salariés du négoce des matériaux de construction"},{idcc:"3219",libelle:"Salariés en portage salarial"},{idcc:"1875",libelle:"Salariés des cabinets et cliniques vétérinaires"},{idcc:"0897",libelle:"Services de prévention et de santé au travail interentreprises"},{idcc:"1090",libelle:"Services de l'automobile"},{idcc:"2147",libelle:"Services d'eau et d'assainissement"},{idcc:"2344",libelle:"Sidérurgie (Nord, Moselle, Meurthe-et-Moselle)"},{idcc:"1672",libelle:"Sociétés d'assurances"},{idcc:"1801",libelle:"Sociétés d'assistance"},{idcc:"2150",libelle:"Sociétés anonymes et fondations d'HLM"},{idcc:"3090",libelle:"Spectacle vivant (secteur privé)"},{idcc:"2511",libelle:"Sport"},{idcc:"2728",libelle:"Sucreries, sucreries-distilleries et raffineries de sucre"},{idcc:"2219",libelle:"Taxis parisiens salariés"},{idcc:"2148",libelle:"Télécommunications"},{idcc:"3241",libelle:"Télédiffusion"},{idcc:"1424",libelle:"Transports publics"},{idcc:"0016",libelle:"Transports routiers et activités auxiliaires du transport"},{idcc:"1170",libelle:"Tuiles et briques (obsolète)"},{idcc:"0087",libelle:"Ouvriers des industries de carrières et de matériaux (obsolète)"},{idcc:"1702",libelle:"Ouvriers de travaux publics"},{idcc:"1596",libelle:"Ouvriers des entreprises du bâtiment de moins de 10 salariés"},{idcc:"1597",libelle:"Ouvriers des entreprises du bâtiment de plus de 10 salariés"},{idcc:"2389",libelle:"Ouvriers du bâtiment et des travaux publics région de La Réunion"},{idcc:"2328",libelle:"Ouvriers du bâtiment et des travaux publics de la Guadeloupe et dépendances"},{idcc:"2564",libelle:"Vétérinaires praticiens salariés"},{idcc:"0493",libelle:"Vins, cidres, jus de fruits, sirops, spiritueux et liqueurs de France"}].sort((e,t)=>e.libelle.localeCompare(t.libelle,"fr")),Ae='<option value="">— Choisir une CCN —</option>'+de.map(e=>`<option value="${e.idcc}">${e.idcc} — ${e.libelle}</option>`).join("");let w=[];window.forgeNav=function(e){["liste","detail","creer"].forEach(t=>{document.getElementById("forge-"+t).style.display=t===e?"block":"none"})};async function Pe(){forgeNav("liste");const e=document.getElementById("forge-cards"),t=document.getElementById("forge-subtitle");e.innerHTML='<div style="color:var(--muted);font-size:0.75rem;padding:0.5rem 0">chargement…</div>';try{const l=await fetch("/forge/contributeurs");if(!l.ok){const a=await l.text();throw new Error(`HTTP ${l.status} — ${a||l.statusText}`)}w=await l.json();const s=w.length;t.textContent=s===0?"aucun contributeur pour l'instant":`${s} contributeur${s>1?"s":""} · ${w.reduce((a,n)=>a+n.expertises.length,0)} expertises CCN`,e.innerHTML=s===0?'<div style="color:var(--muted);font-size:0.75rem">Aucun profil encore — sois le premier à rejoindre.</div>':w.map(Te).join("")}catch(l){e.innerHTML=`<div style="color:var(--red);font-size:0.75rem">Erreur : ${m(T(l))}</div>`}}function Te(e){const t=e.expertises.slice(0,5).map(s=>`<span class="ccn-badge ${s.niveau==="Maîtrisée"?"m":s.niveau==="Pratiquée"?"p":"c"}" title="${m(s.niveau)}">${m(s.ccn_libelle)}</span>`).join(""),l=e.expertises.length>5?`<span class="ccn-badge c">+${e.expertises.length-5}</span>`:"";return`
    <div class="forge-card" onclick="forgeAfficherProfil('${m(e.pseudo)}')">
      <div class="forge-card-pseudo">${m(e.pseudo)}</div>
      <div class="forge-card-poste">${m(e.poste)} <span style="color:var(--dim);font-size:0.6em">${e.poste_est_actuel?"actuel":"visé"}</span></div>
      <div class="forge-card-ccn">${t}${l}</div>
      <div class="forge-card-stats">
        <span><span class="forge-stat-val">${e.votes_received}</span> votes</span>
        <span><span class="forge-stat-val">${e.topics_count}</span> sujets</span>
        <span><span class="forge-stat-val">${e.posts_count}</span> réponses</span>
      </div>
    </div>`}async function Be(e){forgeNav("detail");const t=document.getElementById("forge-profil-content");t.innerHTML='<div style="color:var(--muted);font-size:0.75rem">chargement…</div>';try{let l=w.find(s=>s.pseudo.toLowerCase()===e.toLowerCase());if(!l){const s=await fetch(`/profil/${encodeURIComponent(e)}`);if(!s.ok)throw new Error(`HTTP ${s.status} — ${await s.text()||s.statusText}`);l=await s.json()}t.innerHTML=Fe(l)}catch(l){t.innerHTML=`<div style="color:var(--red);font-size:0.75rem">Erreur : ${m(T(l))}</div>`}}function Fe(e){const t=e.linkedin_url?`<a class="profil-linkedin" href="${m(e.linkedin_url)}" target="_blank" rel="noopener noreferrer">↗ LinkedIn</a>`:"",s=[{niveau:"Maîtrisée",cls:"m",items:e.expertises.filter(i=>i.niveau==="Maîtrisée")},{niveau:"Pratiquée",cls:"p",items:e.expertises.filter(i=>i.niveau==="Pratiquée")},{niveau:"Connue",cls:"c",items:e.expertises.filter(i=>i.niveau==="Connue")}].filter(i=>i.items.length>0).map(i=>`
    <tr class="profil-ccn-section"><td colspan="3">${m(i.niveau)}</td></tr>
    ${i.items.map(o=>`
    <tr>
      <td class="profil-ccn-idcc">${m(o.ccn_idcc)}</td>
      <td>${m(o.ccn_libelle)}</td>
      <td><span class="ccn-badge ${i.cls}">${m(i.niveau)}</span></td>
    </tr>`).join("")}`).join(""),a=e.expertises.length===0?'<div style="color:var(--muted);font-size:0.72rem">Aucune CCN renseignée.</div>':`<table class="profil-ccn-tbl">${s}</table>`,n=e.created_at?j(e.created_at.slice(0,10)):"—";return`
    <div class="profil-head">
      <div>
        <div class="profil-pseudo">${m(e.pseudo)}</div>
        <div class="profil-poste">${m(e.poste)} <span style="color:var(--dim);font-size:0.85em">(${e.poste_est_actuel?"poste actuel":"poste visé"})</span></div>
        ${t}
      </div>
      <div class="profil-since">membre depuis le ${n}</div>
    </div>

    <div class="profil-body">
      <div class="sect-label">PAIE FRANÇAISE</div>
      ${e.paie_fr_niveau?`<span class="ccn-badge ${e.paie_fr_niveau==="Maîtrisée"?"m":e.paie_fr_niveau==="Pratiquée"?"p":"c"}" style="font-size:0.75rem;padding:0.2rem 0.6rem">${m(e.paie_fr_niveau)}</span>`:'<span style="color:var(--dim);font-size:0.7rem">non renseigné</span>'}

      ${e.pays&&e.pays.length>0?`
      <div class="sect-label" style="margin-top:1rem">PAIE INTERNATIONALE</div>
      <table class="profil-ccn-tbl">
        ${[{niveau:"Maîtrisée",cls:"m",items:e.pays.filter(i=>i.niveau==="Maîtrisée")},{niveau:"Pratiquée",cls:"p",items:e.pays.filter(i=>i.niveau==="Pratiquée")},{niveau:"Connue",cls:"c",items:e.pays.filter(i=>i.niveau==="Connue")}].filter(i=>i.items.length>0).map(i=>`
            <tr class="profil-ccn-section"><td colspan="3">${m(i.niveau)}</td></tr>
            ${i.items.map(o=>`
            <tr>
              <td class="profil-ccn-idcc">${m(o.pays_code)}</td>
              <td>${m(o.pays_libelle)}</td>
              <td><span class="ccn-badge ${i.cls}">${m(i.niveau)}</span></td>
            </tr>`).join("")}`).join("")}
      </table>`:""}

      <div class="sect-label" style="margin-top:1rem">EXPERTISES CCN</div>
      ${a}
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
    </div>`}window.setPosteType=function(e){document.getElementById("poste_est_actuel_input").value=e?"1":"0",document.getElementById("ptog-actuel").className="ptog "+(e?"ptog-on":"ptog-off"),document.getElementById("ptog-vise").className="ptog "+(e?"ptog-off":"ptog-on")};const ue=[{code:"BE",libelle:"Belgique"},{code:"LU",libelle:"Luxembourg"},{code:"DE",libelle:"Allemagne"},{code:"CH",libelle:"Suisse"},{code:"IT",libelle:"Italie"},{code:"MC",libelle:"Monaco"},{code:"ES",libelle:"Espagne"},{code:"AD",libelle:"Andorre"},{code:"GB",libelle:"Royaume-Uni"}],Ne=ue.map(e=>`<option value="${e.code}">${m(e.libelle)}</option>`).join("");let me=0;window.forgeAjouterPays=function(){const e=++me,t=document.createElement("div");t.className="forge-ccn-row",t.id="forge-pays-"+e,t.innerHTML=`
    <select class="forge-pays-select">${Ne}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprPays(${e})" title="Supprimer">×</button>`,document.getElementById("forge-pays-list").appendChild(t)};window.forgeSupprPays=function(e){document.getElementById("forge-pays-"+e)?.remove()};let pe=0;window.forgeAjouterCcn=function(){const e=++pe,t=document.createElement("div");t.className="forge-ccn-row",t.id="forge-ccn-"+e,t.innerHTML=`
    <select class="forge-ccn-select">${Ae}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprCcn(${e})" title="Supprimer">×</button>`,document.getElementById("forge-ccn-list").appendChild(t)};window.forgeSupprCcn=function(e){document.getElementById("forge-ccn-"+e)?.remove()};window.forgeSoumettre=async function(e){e.preventDefault();const t=document.getElementById("forge-form"),l=document.getElementById("forge-form-err"),s=document.getElementById("forge-submit-btn");l.textContent="";const a=[];document.querySelectorAll('[id^="forge-pays-"]').forEach(o=>{const p=o.querySelector(".forge-pays-select")?.value,c=o.querySelector(".forge-ccn-niveau")?.value,r=ue.find(f=>f.code===p);p&&r&&a.push({pays_code:p,pays_libelle:r.libelle,niveau:c})});const n=[];document.querySelectorAll('.forge-ccn-row:not([id^="forge-pays-"])').forEach(o=>{const p=o.querySelector(".forge-ccn-select").value,c=o.querySelector(".forge-ccn-niveau").value,r=de.find(f=>f.idcc===p);p&&r&&n.push({ccn_idcc:p,ccn_libelle:r.libelle,niveau:c})});const i={email:t.querySelector('[name="email"]').value.trim(),pseudo:t.querySelector('[name="pseudo"]').value.trim(),poste:t.querySelector('[name="poste"]').value.trim(),linkedin_url:t.querySelector('[name="linkedin_url"]').value.trim()||null,poste_est_actuel:t.querySelector('[name="poste_est_actuel"]').value!=="0",paie_fr_niveau:t.querySelector('[name="paie_fr_niveau"]').value||null,pays:a,expertises:n};if(!i.email){l.textContent="Email requis.";return}if(!i.pseudo){l.textContent="Pseudo requis.";return}if(!i.poste){l.textContent="Poste requis.";return}s.disabled=!0,s.textContent="[ envoi… ]";try{const o=await fetch("/forge/profil",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(i)});if(!o.ok)throw new Error(`HTTP ${o.status} — ${await o.text()||o.statusText}`);const p=await o.json();w.unshift(p),t.reset(),document.getElementById("forge-pays-list").innerHTML="",document.getElementById("forge-ccn-list").innerHTML="",me=0,pe=0,Be(p.pseudo)}catch(o){l.textContent=T(o),s.disabled=!1,s.textContent="[ Rejoindre la Forge ]"}};const Re=[{prenom:"Geralt",nom:"de Riv"},{prenom:"Sam",nom:"Vimes"},{prenom:"Elric",nom:"de Melniboné"},{prenom:"Druss",nom:"la Légende"},{prenom:"Logen",nom:"Neuf-Doigts"},{prenom:"Aragorn",nom:"Grands-Pas"},{prenom:"Jon",nom:"Shannow"},{prenom:"Salim",nom:"Dhibi"},{prenom:"Bayaz",nom:"le Magi"},{prenom:"Merlin",nom:"l'Enchanteur"}],qe=[{prenom:"Lyra",nom:"Belacqua"},{prenom:"Hermione",nom:"Granger"},{prenom:"Eowyn",nom:"du Rohan"},{prenom:"Ellana",nom:"Caldin"},{prenom:"Ferro",nom:"Maljinn"},{prenom:"Magrat",nom:"Garlick"},{prenom:"Ewilan",nom:"Gil'Sayan"},{prenom:"Sigarni",nom:"la Guerrière"},{prenom:"Rikke",nom:"la Nord"},{prenom:"Tanaquil",nom:"la Magicienne"}],le=[17,16,16,15,15,15,14,14,14,13,13,11],P=le[Math.floor(Math.random()*le.length)]/100;let se="H",G=!1;function ie(e){return e[Math.floor(Math.random()*e.length)]}function be(e,t){["d-prenom","m-prenom"].forEach(l=>{const s=document.getElementById(l);s&&(s.value=e)}),["d-nom","m-nom"].forEach(l=>{const s=document.getElementById(l);s&&(s.value=t)}),G=!1}function fe(e,t=!1){const l=e==="H";["d-hf-h","m-hf-h"].forEach(s=>{document.getElementById(s)?.classList.toggle("ptog-on",l),document.getElementById(s)?.classList.toggle("ptog-off",!l)}),["d-hf-f","m-hf-f"].forEach(s=>{document.getElementById(s)?.classList.toggle("ptog-on",!l),document.getElementById(s)?.classList.toggle("ptog-off",l)}),t&&document.querySelectorAll(".genre-ecart-hint").forEach(s=>{s.textContent=l?s.dataset.textHf:s.dataset.textFh,s.style.display="inline"})}window.setGenre=function(e){if(e===se)return;if(!G){const l=e==="F"?window._heroF:window._heroH;be(l.prenom,l.nom)}const t=e==="F"?1-P:1/(1-P);["d-brut","m-brut"].forEach(l=>{const s=document.getElementById(l);s&&(s.value=Math.round(parseFloat(s.value)*t))}),se=e,fe(e,!0)};const V=document.getElementById("burger-btn"),R=document.getElementById("burger-menu");function Oe(){V.classList.add("open"),R.classList.add("open")}window.closeBurger=function(){V.classList.remove("open"),R.classList.remove("open")};V.addEventListener("click",e=>{e.stopPropagation(),R.classList.contains("open")?closeBurger():Oe()});document.addEventListener("click",()=>closeBurger());R.addEventListener("click",e=>e.stopPropagation());
