(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const a of document.querySelectorAll('link[rel="modulepreload"]'))i(a);new MutationObserver(a=>{for(const n of a)if(n.type==="childList")for(const s of n.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&i(s)}).observe(document,{childList:!0,subtree:!0});function l(a){const n={};return a.integrity&&(n.integrity=a.integrity),a.referrerPolicy&&(n.referrerPolicy=a.referrerPolicy),a.crossOrigin==="use-credentials"?n.credentials="include":a.crossOrigin==="anonymous"?n.credentials="omit":n.credentials="same-origin",n}function i(a){if(a.ep)return;a.ep=!0;const n=l(a);fetch(a.href,n)}})();const Le="modulepreload",Me=function(e){return"/"+e},le={},Te=function(t,l,i){let a=Promise.resolve();if(l&&l.length>0){let s=function(y){return Promise.all(y.map(f=>Promise.resolve(f).then(c=>({status:"fulfilled",value:c}),c=>({status:"rejected",reason:c}))))};document.getElementsByTagName("link");const o=document.querySelector("meta[property=csp-nonce]"),d=o?.nonce||o?.getAttribute("nonce");a=s(l.map(y=>{if(y=Me(y),y in le)return;le[y]=!0;const f=y.endsWith(".css"),c=f?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${y}"]${c}`))return;const m=document.createElement("link");if(m.rel=f?"stylesheet":Le,f||(m.as="script"),m.crossOrigin="",m.href=y,d&&m.setAttribute("nonce",d),document.head.appendChild(m),f)return new Promise((h,x)=>{m.addEventListener("load",h),m.addEventListener("error",()=>x(new Error(`Unable to preload CSS for ${y}`)))})}))}function n(s){const o=new Event("vite:preloadError",{cancelable:!0});if(o.payload=s,window.dispatchEvent(o),!o.defaultPrevented)throw s}return a.then(s=>{for(const o of s||[])o.status==="rejected"&&n(o.reason);return t().catch(n)})};async function be(e,t={}){if(window.__TAURI_INTERNALS__){const{invoke:i}=await Te(async()=>{const{invoke:a}=await import("./core-DV6XEvTN.js");return{invoke:a}},[]);return i(e,t)}const l=await fetch(`/api/${e}`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(t)});if(!l.ok)throw await l.text();return l.json()}function j(e){if(e==null)return"(erreur nulle — redémarre l'app ou ouvre DevTools Ctrl+Shift+I)";if(typeof e=="string")return e||"(erreur muette — ouvre DevTools Ctrl+Shift+I et consulte la Console)";if(e instanceof Error)return e.message||e.toString();try{return JSON.stringify(e,null,2)}catch{return String(e)}}let W=null;const K="2026-01-31",fe=K;document.addEventListener("DOMContentLoaded",()=>{["d-date","m-date"].forEach(n=>{const s=document.getElementById(n);s&&(s.value=K,s.max=K)}),document.addEventListener("keydown",n=>{n.key==="Escape"&&closeFmModal()}),window._heroH=Math.random()<.015?{prenom:"Jean-Noël",nom:"Favari"}:pe(Je),window._heroF=pe(Xe),Se(window._heroH.prenom,window._heroH.nom),we("H"),["d-prenom","m-prenom","d-nom","m-nom"].forEach(n=>{document.getElementById(n)?.addEventListener("input",()=>{ee=!0})});const e=Math.round(D*100),t=Math.round(D/(1-D)*100);document.querySelectorAll(".genre-ecart-hint").forEach(n=>{n.dataset.textFh=`// −${e} % · écart salarial F/H`,n.dataset.textHf=`// +${t} % · écart salarial H/F`});const l=window.matchMedia("(max-width: 680px)"),i=n=>{const s=document.body;!s.classList.contains("is-annuel")&&!s.classList.contains("is-forge")&&!s.classList.contains("is-apropos")&&!s.classList.contains("is-gaabrielle")&&!s.classList.contains("is-hercule")&&setView(n.matches?"mobile":"desktop")};l.addEventListener("change",i),i(l),localStorage.getItem("xenna-hv")&&(document.body.classList.add("hv-mode"),document.getElementById("hv-switch")?.classList.add("on")),localStorage.getItem("xenna-zoom")&&(document.body.classList.add("zoom-mode"),document.documentElement.style.zoom="200%",document.getElementById("zoom-switch")?.classList.add("on"),document.getElementById("a11y-magnifier")?.classList.add("active"));const a=localStorage.getItem("xenna-font");a&&setAppFont(a,!0),localStorage.getItem("xenna-hv")&&document.getElementById("a11y-hv-btn")?.classList.add("active"),localStorage.getItem("xenna-bw")&&(document.body.classList.add("bw-mode"),document.getElementById("bw-switch")?.classList.add("on"),document.getElementById("a11y-bw-btn")?.classList.add("active")),localStorage.getItem("xenna-dactylo")&&(F=!0,document.getElementById("dactylo-switch")?.classList.add("on")),document.addEventListener("click",n=>{!n.target.closest("#a11y-btn")&&!n.target.closest("#a11y-panel")&&(document.getElementById("a11y-panel")?.classList.remove("open"),document.getElementById("a11y-btn")?.classList.remove("open"))})});let se="fr";const X={},k=new Map;function Ae(){const e="script,style,input,select,textarea,.mob-val,.sb-val,.fm-val,.a11y-float,.trad-panel,#a11y-panel",t=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT,{acceptNode(i){const a=i.textContent.trim();return!a||a.length<2||/^[\d\s,.\-+%€×\/:()[\]]+$/.test(a)||i.parentElement?.closest(e)?NodeFilter.FILTER_REJECT:NodeFilter.FILTER_ACCEPT}}),l=[];for(;t.nextNode();)l.push(t.currentNode);return l}window.toggleTradPanel=function(){const e=document.getElementById("trad-panel"),t=document.getElementById("trad-btn"),l=e.classList.toggle("open");t.classList.toggle("open",l)};window.translateApp=async function(e){if(document.getElementById("trad-panel")?.classList.remove("open"),document.getElementById("trad-btn")?.classList.remove("open"),document.querySelectorAll(".trad-lang-btn").forEach(s=>s.classList.remove("active")),document.querySelector(`.trad-lang-btn[onclick="translateApp('${e}')"]`)?.classList.add("active"),e==="fr"){k.forEach((s,o)=>{o.isConnected&&(o.textContent=s)}),document.documentElement.lang="fr",se="fr";return}const t=document.getElementById("trad-btn");t.classList.add("loading"),t.textContent="🌐 …";const l=Ae();l.forEach(s=>{k.has(s)||k.set(s,s.textContent)});const i=l.map(s=>k.get(s));X[e]||(X[e]=new Map);const a=X[e],n=[...new Set(i)].filter(s=>!a.has(s));try{if(n.length>0)for(let o=0;o<n.length;o+=20){const d=n.slice(o,o+20),y=d.join(`

`),f=`https://api.mymemory.translated.net/get?q=${encodeURIComponent(y)}&langpair=fr|${e}`,c=await fetch(f);if(!c.ok)throw new Error("HTTP "+c.status);const h=(await c.json()).responseData.translatedText.split(`

`);d.forEach((x,$)=>a.set(x,h[$]??x))}l.forEach(s=>{const o=k.get(s);s.isConnected&&a.has(o)&&(s.textContent=a.get(o))}),document.documentElement.lang=e,se=e}catch(s){console.error("Traduction échouée :",s),t.textContent="🌐 ✗",setTimeout(()=>{t.textContent="🌐 LANGUE",t.classList.remove("loading")},2e3);return}t.textContent="🌐 LANGUE",t.classList.remove("loading")};document.addEventListener("click",e=>{!e.target.closest("#trad-btn")&&!e.target.closest("#trad-panel")&&(document.getElementById("trad-panel")?.classList.remove("open"),document.getElementById("trad-btn")?.classList.remove("open"))});window.toggleA11yPanel=function(){const e=document.getElementById("a11y-panel"),t=document.getElementById("a11y-btn"),l=e.classList.toggle("open");t.classList.toggle("open",l)};window.toggleHVMode=function(){const e=document.body.classList.toggle("hv-mode");document.getElementById("hv-switch")?.classList.toggle("on",e),document.getElementById("a11y-hv-btn")?.classList.toggle("active",e),localStorage.setItem("xenna-hv",e?"1":"")};window.toggleZoom=function(){const e=document.body.classList.toggle("zoom-mode");document.documentElement.style.zoom=e?"200%":"",document.getElementById("zoom-switch")?.classList.toggle("on",e),document.getElementById("a11y-magnifier")?.classList.toggle("active",e),localStorage.setItem("xenna-zoom",e?"1":"")};const ie=new Set,Pe=new Set(["IBM Plex Mono","Fira Code","JetBrains Mono","Source Code Pro","Roboto Mono","Inconsolata"]);window.setAppFont=function(e,t=!1){if(!e){document.body.classList.remove("custom-font"),document.documentElement.style.removeProperty("--app-font"),localStorage.removeItem("xenna-font");const a=document.getElementById("font-picker");a&&(a.value="");return}if(!Pe.has(e))return;const l=e.replace(/ /g,"+");if(!ie.has(l)){const a=document.createElement("link");a.rel="stylesheet",a.href=`https://fonts.googleapis.com/css2?family=${l}&display=swap`,document.head.appendChild(a),ie.add(l)}document.documentElement.style.setProperty("--app-font",`'${e}', monospace`),document.body.classList.add("custom-font"),localStorage.setItem("xenna-font",e);const i=document.getElementById("font-picker");i&&t&&(i.value=e)};const P=[];window.scan67=function(){const e=Date.now();for(P.push(e);P.length&&e-P[0]>1500;)P.shift();const t=P.length>=3;t&&(P.length=0);const l=t?/42/:/67/,a=Array.from(document.querySelectorAll(".mob-val, .sb-val, .ascii-tbl td, .fm-val, .fm-result td")).filter(s=>l.test(s.textContent.replace(/[\s ]/g,""))&&s.offsetParent!==null);if(a.length===0)return;const n=document.getElementById("a11y-67-btn");if(n.classList.add("active"),t){const s=["#ff0055","#ff6600","#ffcc00","#00ff88","#00ccff","#aa00ff","#ff00cc","#39ff14","#ff4444","#44ffff","#ff69b4","#7fff00"];a.forEach((o,d)=>{setTimeout(()=>{const y=s[Math.floor(Math.random()*s.length)];Object.assign(o.style,{background:y,color:"#000",outline:`2px solid ${y}`,borderRadius:"2px",transition:"all 0.15s"}),setTimeout(()=>Object.assign(o.style,{background:"",color:"",outline:"",borderRadius:""}),900)},d*250)}),setTimeout(()=>n.classList.remove("active"),a.length*250+1e3)}else a.forEach((s,o)=>{setTimeout(()=>{s.classList.remove("flash-67"),s.offsetWidth,s.classList.add("flash-67"),s.addEventListener("animationend",()=>s.classList.remove("flash-67"),{once:!0})},o*500)}),setTimeout(()=>n.classList.remove("active"),a.length*500+200)};window.toggleBWMode=function(){const e=document.body.classList.toggle("bw-mode");document.getElementById("bw-switch")?.classList.toggle("on",e),document.getElementById("a11y-bw-btn")?.classList.toggle("active",e),localStorage.setItem("xenna-bw",e?"1":"")};let F=!1,ae=0;const _=e=>new Promise(t=>setTimeout(t,e));window.toggleDactylo=function(){F=!F,document.getElementById("dactylo-switch")?.classList.toggle("on",F),localStorage.setItem("xenna-dactylo",F?"1":"")};async function Be(e){const t=++ae,l=()=>t!==ae,i=document.getElementById("res-desktop"),a=i.querySelectorAll("tr.data-row");if(!a.length)return;const s=((e?.salarie?.prenom||"")+(e?.salarie?.nom||"")).toLowerCase().includes("ë"),o=s?2:4,d=s?1:2,y=["#ff00ff","#00ffff","#ff0066","#66ff00","#ff6600","#0066ff","#ff00cc","#00ff99","#ffff00","#ff3399"],f=()=>s?y[Math.floor(Math.random()*y.length)]:"#ffe066",c=i.querySelector("tr.tbl-total");let m=null,h=null,x="",$="";if(c){const g=c.querySelectorAll("td");m=g[1],h=g[3],m&&(x=m.textContent,m.textContent=""),h&&($=h.textContent,h.textContent="")}const w=[];for(const g of a){const v=g.querySelector("td:first-child > span:last-child");if(v){const I=v.textContent;v.textContent="",w.push({target:v,text:I,ms:o})}g.querySelectorAll("td:not(:first-child)").forEach(I=>{const L=[...I.childNodes].find(u=>u.nodeType===3&&u.textContent.trim());if(L){const u=L.textContent;L.textContent="",w.push({target:L,text:u,ms:d})}}),w.push({pause:s?4:8})}for(const g of w){if(l())return;if(g.pause){await _(g.pause);continue}for(const v of g.text){if(l())return;g.target.textContent+=v,await _(g.ms)}}if(!l()){await _(80);for(const g of a){if(l())return;const v=g.querySelectorAll("td")[3];if(v?.classList.contains("c-sal")){const I=f();v.style.background=I,v.style.color="#000",await _(s?65:110),v.style.background="",v.style.color="",await _(s?10:20)}}if(m){s&&(m.style.fontWeight="bold",m.style.fontSize="1.05em");for(const g of x){if(l())return;m.textContent+=g,await _(d)}s&&oe(m)}if(!l()){await _(80);for(const g of a){if(l())return;const v=g.querySelectorAll("td")[5];if(v?.classList.contains("c-pat")){const I=f();v.style.background=I,v.style.color="#000",await _(s?65:110),v.style.background="",v.style.color="",await _(s?10:20)}}if(h){s&&(h.style.fontWeight="bold",h.style.fontSize="1.05em");for(const g of $){if(l())return;h.textContent+=g,await _(d)}s&&oe(h)}}}}let ne=!1;function Fe(){if(ne)return;ne=!0;const e=document.createElement("style");e.textContent="@keyframes flameRise{0%{transform:translateY(0) scale(1);opacity:1}100%{transform:translateY(-42px) scale(0);opacity:0}}",document.head.appendChild(e)}function oe(e){Fe();const t=e.getBoundingClientRect(),l=document.createElement("div");l.style.cssText=`position:fixed;pointer-events:none;z-index:9999;overflow:visible;left:${t.left}px;top:${t.top}px;width:${t.width}px;height:${t.height}px`,document.body.appendChild(l);const i=["#ff8800","#ffdd00","#ff5500","#ffaa00","#ff3300","#ffcc00","#ff6600"],a=setInterval(()=>{const n=document.createElement("div"),s=Math.floor(Math.random()*3)+1,o=(.5+Math.random()*1.2).toFixed(2);n.style.cssText=`position:absolute;pointer-events:none;left:${(Math.random()*t.width).toFixed(1)}px;bottom:0;width:${s}px;height:${s}px;background:${i[Math.floor(Math.random()*i.length)]};animation:flameRise ${o}s ease-out forwards`,l.appendChild(n),setTimeout(()=>n.remove(),parseFloat(o)*1e3+50)},35);setTimeout(()=>{clearInterval(a),setTimeout(()=>l.remove(),1400)},3e3)}function b(e){return String(e).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#039;")}window.setView=function(e){["mobile","desktop","annuel","forge","apropos","contact","gaabrielle","hercule"].forEach(t=>document.body.classList.toggle("is-"+t,e===t)),document.getElementById("btn-desk").classList.toggle("active",e==="desktop"),document.getElementById("btn-mob").classList.toggle("active",e==="mobile"),document.getElementById("btn-ann").classList.toggle("active",e==="annuel"),W&&(e==="desktop"||e==="mobile")&&ye(W),e==="forge"&&je()};let U="EUR";function r(e){const t=parseFloat(e),l=U==="CHF"?" CHF":" €";return t.toLocaleString("fr-FR",{minimumFractionDigits:2,maximumFractionDigits:2})+l}function Re(e,t=!1){const l=parseFloat(e),i=U==="CHF"?" CHF":" €",a=l.toLocaleString("fr-FR",{minimumFractionDigits:2,maximumFractionDigits:2})+i;return t&&l>0?"+"+a:a}function V(e){return(parseFloat(e)*100).toFixed(2)+" %"}function ge(){const e=document.body.classList.contains("is-mobile")?"m-date":"d-date",t=e==="m-date"?"d-date":"m-date";return document.getElementById(e)?.value||document.getElementById(t)?.value||fe}function Z(e){if(!e)return"—";const[t,l,i]=e.split("-");return`${i}/${l}/${t}`}const Ne=[{min:0,max:1620,taux:0},{min:1620,max:1683,taux:.005},{min:1683,max:1791,taux:.013},{min:1791,max:1911,taux:.021},{min:1911,max:2042,taux:.029},{min:2042,max:2151,taux:.035},{min:2151,max:2294,taux:.041},{min:2294,max:2714,taux:.053},{min:2714,max:3107,taux:.075},{min:3107,max:3539,taux:.099},{min:3539,max:3983,taux:.119},{min:3983,max:4648,taux:.138},{min:4648,max:5574,taux:.158},{min:5574,max:6974,taux:.179},{min:6974,max:8711,taux:.2},{min:8711,max:12091,taux:.24},{min:12091,max:16376,taux:.28},{min:16376,max:25706,taux:.33},{min:25706,max:55062,taux:.38},{min:55062,max:1/0,taux:.43}];function Q(e){const t=parseFloat(e);if(isNaN(t)||t<=0)return{total:0,taux_effectif:0,details:[]};let l=0;const i=[];for(const a of Ne){if(t<=a.min)break;const s=+((a.max===1/0?t:Math.min(t,a.max))-a.min).toFixed(2),o=s*a.taux;if(i.push({min:a.min,max:a.max===1/0?null:a.max,taux:a.taux,base:s,montant:+o.toFixed(2)}),l+=o,a.max===1/0||t<=a.max)break}return{total:+l.toFixed(2),taux_effectif:t>0?l/t:0,details:i}}const ce={"Sécurité Sociale":"cat-ss","CSG/CRDS":"cat-csg","Retraite complémentaire":"cat-ret",Prévoyance:"cat-prev",Chômage:"cat-cho",Allègement:"cat-alleg","1er pilier":"cat-ss","Assurance chômage":"cat-cho","Assurance accidents":"cat-acc","Prévoyance maladie":"cat-prev","Prévoyance (LPP)":"cat-ret","Assurance pension":"cat-ret","Assurance maladie":"cat-ss","Assurance dépendance":"cat-prev","Mutualité des employeurs":"cat-ss"},B={},re={SS_VIEILLESSE_PLAF:"min(Salaire brut, Plafond Mensuel Sécurité Sociale — PMSS)",CHOMAGE:"min(Salaire brut, 4 × PMSS)",AGS:"min(Salaire brut, 4 × PMSS)",CSG_DEDUCTIBLE:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels (CSS art. L136-2)",CSG_NON_DEDUCTIBLE:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels",CRDS:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels",AGIRC_ARRCO_T1:"min(Salaire brut, PMSS)  — Tranche 1 (entre 0 et 1 PMSS)",AGIRC_ARRCO_CEG_T1:"min(Salaire brut, PMSS)  — Tranche 1",PREVOYANCE_CADRE_MIN:"min(Salaire brut, PMSS)  — Tranche A",AGIRC_ARRCO_T2:"Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2",AGIRC_ARRCO_CEG_T2:"Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2"};function O(e,t=!0){const l=["f","(","x",")"].map((i,a)=>`<span style="animation-delay:${a*45}ms">${i}</span>`).join("");return t?`<span class="formula-star" data-fmkey="${e}" onclick="event.stopPropagation();showFormula('${e}')">${l}</span>`:`<span class="formula-star" aria-hidden="true">${l}</span>`}window.togglePasDetail=function(e){const t=document.getElementById(e);if(!t)return;const l=t.style.display!=="none";t.style.display=l?"none":"block";const i=document.getElementById(e+"-arrow");i&&(i.textContent=l?"▶":"▼")};function H(e,t){if(e.code==="REDUCTION_FILLON")return`<pre class="fm-fillon">${b(e.explication)}</pre>`;const l=t==="sal",i=l?e.taux_sal:e.taux_pat,a=parseFloat(e.base),n=l?parseFloat(e.montant_sal):Math.abs(parseFloat(e.montant_pat)),s=l?"Taux salarial":"Taux patronal",o=l?"Montant salarial":t==="alleg"?"Montant allègement":"Montant patronal",d=l?"c-sal":t==="alleg"?"c-alleg":"c-pat",y=re[e.code]?`<div class="fm-base-note">Assiette  =  ${b(re[e.code])}</div>`:"";return`
    <div class="fm-generic">Montant  =  Assiette  ×  ${s}</div>
    ${y}
    <table class="fm-calc">
      <tr>
        <td>Assiette</td>
        <td class="fm-op">=</td>
        <td class="fm-val c-base">${r(a)}</td>
      </tr>
      <tr>
        <td>${s}</td>
        <td class="fm-op">×</td>
        <td class="fm-val c-taux">${V(i)}</td>
      </tr>
      <tr class="fm-result fm-sep">
        <td>${o}</td>
        <td class="fm-op">=</td>
        <td class="fm-val ${d}">${r(n)}</td>
      </tr>
    </table>`}function ve(e){const t=Q(e);return`
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
      <tbody>${t.details.map(i=>{const a=i.min.toLocaleString("fr-FR")+" €",n=i.max===null?"∞":i.max.toLocaleString("fr-FR")+" €",s=i.taux===0;return`
      <tr class="${s?"pas-zero":""}">
        <td>${a} → ${n}</td>
        <td class="r">${r(i.base)}</td>
        <td class="r ${s?"c-dim":""}">${(i.taux*100).toFixed(1).replace(".",",")} %</td>
        <td class="r ${s?"c-dim":"c-purple"}">${s?"—":r(i.montant)}</td>
      </tr>`}).join("")}</tbody>
      <tfoot>
        <tr>
          <td>Net imposable</td>
          <td class="r c-gray">${r(e)}</td>
          <td class="r c-taux">${(t.taux_effectif*100).toFixed(2)} %&nbsp;<span style="color:var(--dim);font-size:0.7em">(taux effectif)</span></td>
          <td class="r c-purple" style="font-weight:bold">${r(t.total)}</td>
        </tr>
      </tfoot>
    </table>`}window.showFormula=function(e){const t=B[e];if(!t)return;const l=document.getElementById("fm-body");if(t.type==="pas"){document.getElementById("fm-title").textContent="Prélèvement à la Source (PAS)",document.getElementById("fm-badge").textContent="── Détail par tranche — barème neutre mensuel DGFIP ─────────",l.className="fm-type-pas",l.innerHTML=ve(t.netImposable),document.getElementById("fm-modal").classList.add("open"),document.querySelectorAll(`[data-fmkey="${e}"]`).forEach(o=>o.classList.add("visited"));return}const{c:i,type:a}=t,n=a==="sal",s=i.code==="REDUCTION_FILLON"?"── Allègement patronal ──────────────────────":n?"── Part salariale ───────────────────────────":"── Part patronale ───────────────────────────";document.getElementById("fm-title").textContent=i.libelle,document.getElementById("fm-badge").textContent=s,l.className=`fm-type-${a}`,l.innerHTML=H(i,a),document.getElementById("fm-modal").classList.add("open"),document.querySelectorAll(`[data-fmkey="${e}"]`).forEach(o=>o.classList.add("visited"))};window.closeFmModal=function(){document.getElementById("fm-modal").classList.remove("open")};window.toggleExpl=function(e){const t=document.getElementById(`row-${e}`),l=document.getElementById(`expl-${e}`);if(!t||!l)return;const i=l.style.display!=="none";l.style.display=i?"none":"table-row",t.classList.toggle("open",!i)};function qe(e){const t=document.getElementById("res-desktop"),l=e.cotisations,i=["suisse","luxembourg"].includes(e.salarie?.pays),a=l.reduce((p,S)=>p+parseFloat(S.montant_sal),0),n=l.reduce((p,S)=>p+parseFloat(S.montant_pat),0),s=i?{total:0,taux_effectif:0}:Q(e.net_imposable),o=parseFloat(e.net_a_payer)-s.total;i||(B.PAS={type:"pas",netImposable:parseFloat(e.net_imposable)});const d=e.salarie?.pays==="suisse"?l.find(p=>p.code==="CH_IS"):null,y=d?parseFloat(d.montant_sal):0,f=d?parseFloat(d.taux_sal):0;d&&(B.CH_IS={c:d,type:"sal"});const c=a-y,m=`
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
            <span style="color:var(--red)">− ${r(c)}</span>
          </div>
          ${d?`<div class="sb-ded-row">
            <span>Impôt à la source (${(f*100).toFixed(1)} %)</span>
            <span class="fm-val" style="color:var(--purple);cursor:pointer" onclick="showFormula('CH_IS')">− ${r(y)}${O("CH_IS")}</span>
          </div>`:""}
          ${i?"":`<div class="sb-ded-row">
            <span>PAS (${(s.taux_effectif*100).toFixed(1)} %)</span>
            <span class="fm-val" style="color:var(--purple);cursor:pointer" onclick="showFormula('PAS')">− ${r(s.total)}${O("PAS")}</span>
          </div>`}
          <div class="sb-ded-total">
            <span>Total retenues</span>
            <span style="color:var(--red)">− ${r(a+s.total)}</span>
          </div>
        </div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ NET À PAYER</div>
        <div class="sb-val c-green">${r(o)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ CHARGES PAT.</div>
        <div class="sb-val c-orange">${r(n)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ SUPER BRUT</div>
        <div class="sb-val c-yellow">${r(parseFloat(e.brut)+n)}</div>
      </div>
    </div>`,h=l.filter(p=>p.categorie!=="Allègement"&&(parseFloat(p.montant_sal)>0||p.taux_sal!=="0"||parseFloat(p.montant_pat)>0)),x=l.filter(p=>p.categorie==="Allègement"),$=h.reduce((p,S)=>p+parseFloat(S.montant_pat),0);function w(p,S){return p.map((E,C)=>{const M=S+C,T=ce[E.categorie]||"cat-ss",z=parseFloat(E.montant_sal)>0?"c-sal":"c-dim",J=parseFloat(E.montant_pat)>0?"c-pat":"c-dim",N=`${E.code}_sal`,q=`${E.code}_pat`,A=parseFloat(E.montant_sal)>0,G=parseFloat(E.montant_pat)>0;A&&(B[N]={c:E,type:"sal"}),G&&(B[q]={c:E,type:"pat"});const Ce=O(N,A),_e=O(q,G);return`
        <tr class="data-row" id="row-${M}" onclick="toggleExpl(${M})">
          <td>
            <span class="expand-icon">▶</span>
            <span class="cat ${T}">[${E.categorie}]</span>
            <span>${E.libelle}</span>
          </td>
          <td class="r">${r(E.base)}</td>
          <td class="r">${parseFloat(E.taux_sal)>0?"− ":""}${V(E.taux_sal)}</td>
          <td class="r ${z}"${A?` onclick="event.stopPropagation();showFormula('${N}')" style="cursor:pointer"`:""}>${A?"− ":""}${r(E.montant_sal)}${Ce}</td>
          <td class="r">${parseFloat(E.taux_pat)>0?"− ":""}${V(E.taux_pat)}</td>
          <td class="r ${J}"${G?` onclick="event.stopPropagation();showFormula('${q}')" style="cursor:pointer"`:""}>${G?"− ":""}${r(E.montant_pat)}${_e}</td>
        </tr>
        <tr class="expl-row" id="expl-${M}" style="display:none">
          <td colspan="6">
            <div class="expl-box">
              <div class="expl-txt">▸ ${b(E.explication)}</div>
              ${E.loi_ref?`<div class="expl-ref">§ ${b(E.loi_ref)}</div>`:""}
            </div>
          </td>
        </tr>`}).join("")}const g=`
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
    </thead>`,v=`
    <div class="tbl-section-head">── COTISATIONS ────────────────────────────────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${g}
      <tbody>
        ${w(h,0)}
        <tr class="tbl-total">
          <td colspan="3">TOTAUX</td>
          <td class="r c-sal">= − ${r(a)}</td>
          <td></td>
          <td class="r c-pat">= − ${r($)}</td>
        </tr>
      </tbody>
    </table>`,I=`<div class="sim-period">
    SIMULATION AU <span class="sp-accent">${Z(ge())}</span>
    &nbsp;·&nbsp; PMSS en vigueur calculé depuis la base de données sans le moindre état d'âme
  </div>`,L=x.reduce((p,S)=>p+parseFloat(S.montant_pat),0),u=x.length===0?"":`
    <div class="tbl-section-head">── ALLÈGEMENTS PATRONAUX ───────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${g}
      <tbody>
        ${x.map((p,S)=>{const E=h.length+S,C=ce[p.categorie]||"cat-alleg",M=Math.abs(parseFloat(p.montant_pat)),T=`${p.code}_alleg`;return B[T]={c:p,type:"alleg"},`
            <tr class="data-row" id="row-${E}" onclick="toggleExpl(${E})">
              <td>
                <span class="expand-icon">▶</span>
                <span class="cat ${C}">[${p.categorie}]</span>
                <span>${p.libelle}</span>
              </td>
              <td class="r">${r(p.base)}</td>
              <td class="r"></td>
              <td class="r"></td>
              <td class="r c-alleg">${V(Math.abs(parseFloat(p.taux_pat)))}</td>
              <td class="r c-alleg" onclick="event.stopPropagation();showFormula('${T}')" style="cursor:pointer">− ${r(M)}${O(T)}</td>
            </tr>
            <tr class="expl-row" id="expl-${E}" style="display:none">
              <td colspan="6">
                <div class="expl-box">
                  <div class="expl-txt">▸ ${b(p.explication)}</div>
                  ${p.loi_ref?`<div class="expl-ref">§ ${b(p.loi_ref)}</div>`:""}
                </div>
              </td>
            </tr>`}).join("")}
        <tr class="tbl-total">
          <td colspan="5">TOTAL ALLÈGEMENTS PATRONAUX</td>
          <td class="r c-alleg">− ${r(Math.abs(L))}</td>
        </tr>
      </tbody>
    </table>`;t.innerHTML=I+m+`<div class="tbl-wrap">${v}${u}</div>`}window.mobToggle=function(e,t){const l=document.getElementById("mob-expand-"+e);if(!l)return;const i=["why","how","sal","pat"],a=l.style.display!=="none",n=l.dataset.panel,s=o=>{const d=document.getElementById(`mob-expand-${e}-${o}`);d&&(d.style.display=o===t?"block":"none")};a?n===t?l.style.display="none":(l.dataset.panel=t,i.forEach(s)):(l.style.display="block",l.dataset.panel=t,i.forEach(s))};function ke(e,t,l,i,a,n=0){const s=e.code==="REDUCTION_FILLON"?`<pre class="fm-fillon">${b(e.explication)}</pre>`:`<div class="fm-type-${a}">${H(e,a)}</div>`,o=`
    <div class="mob-exp-txt">${b(e.explication)}</div>
    ${e.loi_ref?`<div class="mob-exp-loi">§ ${b(e.loi_ref)}</div>`:""}`;return`
    <div class="${`mob-stripe-${a}-${n%2===0?"a":"b"}`}">
      <div class="mob-row">
        <span class="mob-lbl mob-cot-lbl"
              title="Explication et référence légale"
              onclick="mobToggle('${t}','why')">${b(e.libelle)}</span>
        <span class="mob-val ${i} mob-cot-amt"
              title="Formule de calcul"
              onclick="mobToggle('${t}','how')">${l}</span>
      </div>
      <div class="mob-expand" id="mob-expand-${t}" style="display:none">
        <div id="mob-expand-${t}-why">${o}</div>
        <div id="mob-expand-${t}-how" style="display:none">${s}</div>
      </div>
    </div>`}function Oe(e){const t=document.getElementById("res-mobile"),l=document.getElementById("m-nom")?.value||document.getElementById("d-nom")?.value||"",i=document.getElementById("m-prenom")?.value||document.getElementById("d-prenom")?.value||"",a=e.cotisations,n=["suisse","luxembourg"].includes(e.salarie?.pays),s=a.reduce((u,p)=>u+parseFloat(p.montant_sal),0),o=a.reduce((u,p)=>u+parseFloat(p.montant_pat),0),d=n?{total:0,taux_effectif:0}:Q(e.net_imposable),y=parseFloat(e.net_a_payer)-d.total,f=parseFloat(e.brut)+o,c=e.salarie?.pays==="suisse"?a.find(u=>u.code==="CH_IS"):null,m=c?parseFloat(c.montant_sal):0,h=c?parseFloat(c.taux_sal):0,x=s-m,$=a.filter(u=>u.categorie!=="Allègement"&&u.code!=="CH_IS"&&(parseFloat(u.montant_sal)>0||u.taux_sal!=="0"||parseFloat(u.montant_pat)>0)),w=a.filter(u=>u.categorie==="Allègement"),g=$.reduce((u,p)=>u+parseFloat(p.montant_pat),0),v=w.reduce((u,p)=>u+parseFloat(p.montant_pat),0),I=$.map((u,p)=>{const S=parseFloat(u.montant_sal)>0,E=parseFloat(u.montant_pat)>0,C=`${u.code}_u`,M=u.code==="REDUCTION_FILLON",T=S?M?`<pre class="fm-fillon">${b(u.explication)}</pre>`:`<div class="fm-type-sal">${H(u,"sal")}</div>`:"",z=E?M?`<pre class="fm-fillon">${b(u.explication)}</pre>`:`<div class="fm-type-pat">${H(u,"pat")}</div>`:"",J=`
      <div class="mob-exp-txt">${b(u.explication)}</div>
      ${u.loi_ref?`<div class="mob-exp-loi">§ ${b(u.loi_ref)}</div>`:""}`,N=`mob-stripe-sal-${p%2===0?"a":"b"}`,q=S?`<span class="mob-val mob-cot-amt" style="color:#ffe033" onclick="mobToggle('${C}','sal')">− ${r(u.montant_sal)}</span>`:`<span class="mob-val c-dim">0 ${U==="CHF"?"CHF":"€"}</span>`,A=E?`<span class="mob-val c-orange mob-cot-amt" onclick="mobToggle('${C}','pat')">− ${r(u.montant_pat)}</span>`:`<span class="mob-val c-dim">0 ${U==="CHF"?"CHF":"€"}</span>`;return`
      <div class="${N}">
        <div class="mob-row">
          <span class="mob-lbl mob-cot-lbl" onclick="mobToggle('${C}','why')">${b(u.libelle)}</span>
          <span style="display:flex;flex-direction:column;align-items:flex-end;gap:0.1rem">${q}${A}</span>
        </div>
        <div class="mob-expand" id="mob-expand-${C}" style="display:none">
          <div id="mob-expand-${C}-why">${J}</div>
          ${T?`<div id="mob-expand-${C}-sal" style="display:none">${T}</div>`:""}
          ${z?`<div id="mob-expand-${C}-pat" style="display:none">${z}</div>`:""}
        </div>
      </div>`}).join(""),L=w.map((u,p)=>ke(u,`${u.code}_alleg`,`− ${r(Math.abs(parseFloat(u.montant_pat)))}`,"c-alleg","alleg",p)).join("");t.innerHTML=`
    <div class="mob-bulletin">

      <!-- En-tête bulletin -->
      <div class="mob-head">
        <span class="mob-head-title">BULLETIN DE PAYE</span>
        <div style="text-align:right">
          <div class="mob-head-name">${b(i)} ${b(l).toUpperCase()}</div>
          <div class="mob-head-date">simulation au ${Z(ge())}</div>
        </div>
      </div>

      <!-- Brut -->
      <div class="mob-row" style="margin-top:0.15rem">
        <span class="mob-lbl">Salaire de base brut</span>
        <span class="mob-val c-gray">${r(e.brut)}</span>
      </div>

      <!-- Cotisations unifiées (salariales + patronales sur une ligne) -->
      <div class="mob-row section"><span class="mob-lbl">── COTISATIONS ──</span><span style="display:flex;gap:1.5rem;font-size:0.62rem;color:var(--muted)"><span>SAL.</span><span>PAT.</span></span></div>
      ${I}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL cotisations sociales</span>
        <span class="mob-val c-red">− ${r(x)}</span>
      </div>
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL charges patronales</span>
        <span class="mob-val c-orange">− ${r(g)}</span>
      </div>

      <!-- Impôt à la source suisse — accordéon dédié -->
      ${c?`<div class="mob-row pas-row" style="cursor:pointer" onclick="togglePasDetail('is-detail-mob')">
        <span class="mob-lbl">Impôt à la source (${(h*100).toFixed(1)} %) <span id="is-detail-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-purple">− ${r(m)}</span>
      </div>
      <div id="is-detail-mob" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        <div class="fm-type-sal">${H(c,"sal")}</div>
        <div class="mob-exp-txt" style="margin-top:0.5rem">${b(c.explication)}</div>
        ${c.loi_ref?`<div class="mob-exp-loi">§ ${b(c.loi_ref)}</div>`:""}
      </div>`:""}

      <!-- Net imposable (France / FPT) -->
      ${n?"":`<div class="mob-row net-row">
        <span class="mob-lbl">NET IMPOSABLE</span>
        <span class="mob-val c-green">${r(e.net_imposable)}</span>
      </div>`}

      <!-- PAS (France / FPT) -->
      ${n?"":`<div class="mob-row pas-row" style="cursor:pointer" onclick="togglePasDetail('pas-detail-mob')">
        <span class="mob-lbl">Prélèvement à la source (${(d.taux_effectif*100).toFixed(1)} %) <span id="pas-detail-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-purple">− ${r(d.total)}</span>
      </div>
      <div id="pas-detail-mob" class="fm-type-pas" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        ${ve(parseFloat(e.net_imposable))}
      </div>`}

      <!-- Net à payer -->
      <div class="mob-row final-row">
        <span class="mob-lbl">NET À PAYER</span>
        <span class="mob-val c-green">${r(y)}</span>
      </div>

      <!-- Allègements -->
      ${L.length?`
      <div class="mob-row section"><span class="mob-lbl">── ALLÈGEMENTS PATRONAUX ──</span><span></span></div>
      ${L}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL allègements</span>
        <span class="mob-val c-alleg">− ${r(Math.abs(v))}</span>
      </div>`:""}

      <!-- Super brut -->
      <div class="mob-row superbrut">
        <span class="mob-lbl">SUPER BRUT (coût employeur)</span>
        <span class="mob-val c-blue">${r(f)}</span>
      </div>

    </div>`}function ye(e){U=e.devise||"EUR",qe(e),Oe(e),F&&Be(e)}function de(e){const t=`<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">⚠ ${b(e)}</div>`;document.getElementById("res-desktop").innerHTML=t,document.getElementById("res-mobile").innerHTML=t}async function he(e){const t=e==="mobile",l=document.getElementById(t?"m-brut":"d-brut").value,i=document.getElementById(t?"m-statut":"d-statut").value,a=document.getElementById(t?"m-nom":"d-nom").value||"Dupont",n=document.getElementById(t?"m-prenom":"d-prenom").value||"Marie",s=document.getElementById(t?"m-date":"d-date").value||fe,o=document.getElementById(t?"m-alsace-moselle":"d-alsace-moselle")?.checked??!1,d=document.getElementById(t?"m-suisse":"d-suisse")?.checked??!1,y=document.getElementById(t?"m-luxembourg":"d-luxembourg")?.checked??!1,f=document.getElementById(t?"m-fpt":"d-fpt")?.checked??!1,c=document.getElementById(t?"m-assujetti-is":"d-assujetti-is")?.checked??!1,m=document.getElementById(t?"m-canton":"d-canton")?.value||null,h=document.getElementById(t?"m-tarif-is":"d-tarif-is")?.value||null,x=parseFloat(l);if(!l||isNaN(x)||x<=0){de("Salaire brut invalide — saisir un montant positif.");return}if(!/^\d{4}-\d{2}-\d{2}$/.test(s)){de(`Date invalide : '${s}' (format attendu : YYYY-MM-DD).`);return}["d-brut","m-brut"].forEach(g=>{const v=document.getElementById(g);v&&(v.value=l)}),["d-statut","m-statut"].forEach(g=>{const v=document.getElementById(g);v&&(v.value=i)}),["d-nom","m-nom"].forEach(g=>{const v=document.getElementById(g);v&&(v.value=a)}),["d-prenom","m-prenom"].forEach(g=>{const v=document.getElementById(g);v&&(v.value=n)}),["d-date","m-date"].forEach(g=>{const v=document.getElementById(g);v&&(v.value=s)});const $=d?"suisse":y?"luxembourg":null,w=$?"2026-01-01":s;try{const g=await be("calculer_bulletin",{salarie:{nom:a,prenom:n,salaire_brut:l.toString(),statut:i,alsace_moselle:o,pays:$??(f?"fonction_publique":"france"),assujetti_is:c,canton:d&&c&&m?m:null,tarif_is:d&&c&&h?h:null},datePaie:w});W=g,ye(g)}catch(g){console.error("[calculer_bulletin] erreur brute :",g);const v=j(g),I=`<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">ERREUR : ${b(v)}</div>`;document.getElementById("res-desktop").innerHTML=I,document.getElementById("res-mobile").innerHTML=I}}function He(e){const t=document.getElementById("res-annuel"),l=e.lignes,i=l.map(f=>f.smic),a=`
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
    </tr></thead>`,n=l.map((f,c)=>{const m=c>0&&f.smic!==i[c-1],h=f.mois_libelle.includes("13e"),x=parseFloat(f.fillon_regularise)-parseFloat(f.fillon_simple),$=Math.abs(x)<.005?'<span style="color:var(--dim)">—</span>':`<span class="delta-nonzero">${x>0?"+":""}${Re(x.toFixed(2))}</span>`;return`<tr class="${[m?"smic-change":"",h?"treizieme-mois":""].filter(Boolean).join(" ")}">
      <td>${f.mois_libelle}</td>
      <td>${r(f.smic)}</td>
      <td>${r(f.brut)}</td>
      <td class="c-sal">− ${r(f.total_sal)}</td>
      <td class="c-pat">+ ${r(f.total_pat_brut)}</td>
      <td class="c-alleg">− ${r(f.fillon_regularise)}</td>
      <td>${$}</td>
      <td class="c-green">${r(f.net_a_payer)}</td>
      <td class="c-yellow">${r(f.cout_employeur)}</td>
    </tr>`}).join(""),s=`
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
    </tr>`,o=parseFloat(e.total_pat_brut),d=parseFloat(e.total_fillon),y=`
    <div style="display:flex;gap:1rem;flex-wrap:wrap;margin-top:0.75rem;font-size:0.72rem">
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">ÉCONOMIE FILLON (annuelle)</div>
        <div style="color:var(--green);font-size:1.1rem;font-weight:bold">− ${r(e.total_fillon)}</div>
      </div>
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">TAUX FILLON MOYEN</div>
        <div style="color:var(--blue);font-size:1.1rem;font-weight:bold">
          ${o>0?(d/parseFloat(e.total_brut)*100).toFixed(2)+" %":"—"}
        </div>
      </div>
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">COÛT EMPLOYEUR ANNUEL</div>
        <div style="color:var(--yellow);font-size:1.1rem;font-weight:bold">${r(e.total_cout)}</div>
      </div>
    </div>`;t.innerHTML=`
    <div class="tbl-section-head">── SIMULATION ANNUELLE ${e.annee} ────────────────────────────────────</div>
    <div style="font-size:0.70rem;color:var(--muted);margin-bottom:0.4rem">
      Décembre inclut un 13e mois (salaire doublé). Brut total = 13 mois. Fillon régularisé sur rémunération annuelle réelle.
    </div>
    <table class="ann-tbl">
      ${a}
      <tbody>${n}</tbody>
      ${s}
    </table>
    ${y}`}async function De(){const e=parseInt(document.getElementById("a-annee").value),t=document.getElementById("a-brut").value,l=document.getElementById("a-statut").value,i=document.getElementById("res-annuel");if(isNaN(e)||e<1900||e>2100){i.innerHTML='<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Année invalide.</div>';return}const a=parseFloat(t);if(!t||isNaN(a)||a<=0){i.innerHTML='<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Salaire brut invalide — saisir un montant positif.</div>';return}i.innerHTML='<div style="color:var(--muted);padding:1rem;font-size:0.78rem">Calcul en cours…</div>';try{const n=await be("simuler_annee",{annee:e,salaireBrut:t.toString(),statut:l});He(n)}catch(n){console.error("[simuler_annee] erreur brute :",n),i.innerHTML=`<div style="padding:1rem;color:var(--red);font-size:0.8rem">ERREUR : ${b(j(n))}</div>`}}window.onTogglePays=function(e,t){const l=["suisse","luxembourg","fpt"],i=["suisse","luxembourg"],a=l.filter(c=>c!==e);t&&a.forEach(c=>{["d","m"].forEach(m=>{const h=document.getElementById(`${m}-${c}`);h&&h.checked&&(h.checked=!1)})});const n=i.some(c=>document.getElementById(`d-${c}`)?.checked);["d","m"].forEach(c=>{const m=document.getElementById(`${c}-alsace-moselle-wrap`);m&&(m.style.display=n?"none":"");const h=document.getElementById(`${c}-alsace-moselle`);h&&n&&(h.checked=!1)}),["d-date","m-date"].forEach(c=>{const m=document.getElementById(c);m&&(m.disabled=n,n&&(m.value="2026-01-01"))});const s=document.getElementById("d-suisse")?.checked,o=s?"SALAIRE BRUT (CHF)":"SALAIRE BRUT (€)",d=s?"BRUT (CHF)":"BRUT (€)",y=document.getElementById("d-brut");if(y){const c=y.closest(".field")?.querySelector("label");c&&(c.textContent=o)}const f=document.getElementById("m-brut");if(f){const c=f.closest(".field")?.querySelector("label");c&&(c.textContent=d)}["d","m"].forEach(c=>{const m=document.getElementById(`${c}-ch-is-wrap`);if(m)if(s)m.style.display="";else{m.style.display="none";const h=document.getElementById(`${c}-assujetti-is`);h&&(h.checked=!1);const x=document.getElementById(`${c}-ch-is-detail`);x&&(x.style.display="none")}})};window.toggleParams=function(e){const t=document.getElementById(`${e}-params`),l=document.getElementById(`${e}-params-toggle`);if(!t)return;const i=t.style.display!=="none";t.style.display=i?"none":"block",l.classList.toggle("open",!i)};window.syncParam=function(e,t){["d","m"].forEach(l=>{const i=document.getElementById(`${l}-${e}`);i&&(i.type==="checkbox"?i.checked!==t&&(i.checked=t):i.value!==t&&(i.value=t))})};window.onToggleAssujetti=function(e){["d","m"].forEach(t=>{const l=document.getElementById(`${t}-ch-is-detail`);l&&(l.style.display=e?"":"none")})};document.getElementById("d-calc").addEventListener("click",()=>he("desktop"));document.getElementById("m-calc").addEventListener("click",()=>he("mobile"));document.getElementById("a-calc").addEventListener("click",De);const Ee=[{idcc:"1261",libelle:"Acteurs du lien social et familial (ALISFA)"},{idcc:"2941",libelle:"Aide, accompagnement, soins et services à domicile"},{idcc:"1747",libelle:"Activités industrielles de boulangerie et de pâtisserie"},{idcc:"2149",libelle:"Activités du déchet"},{idcc:"2335",libelle:"Agences générales d'assurances"},{idcc:"1686",libelle:"Audiovisuel, électronique et équipement ménager"},{idcc:"2120",libelle:"Banque"},{idcc:"3210",libelle:"Banque Populaire"},{idcc:"0567",libelle:"Bijouterie, joaillerie, orfèvrerie (obsolète)"},{idcc:"0158",libelle:"Bois et scieries"},{idcc:"0992",libelle:"Boucherie"},{idcc:"0843",libelle:"Boulangerie-pâtisserie artisanales"},{idcc:"1606",libelle:"Bricolage"},{idcc:"1486",libelle:"Bureaux d'études techniques et sociétés de conseils (Syntec)"},{idcc:"0787",libelle:"Cabinets d'experts-comptables et de commissaires aux comptes"},{idcc:"2332",libelle:"Cabinets d'architectes"},{idcc:"1619",libelle:"Cabinets dentaires"},{idcc:"2420",libelle:"Cadres du bâtiment"},{idcc:"3212",libelle:"Cadres des travaux publics"},{idcc:"1256",libelle:"Cadres des entreprises de gestion d'équipements thermiques et de climatisation"},{idcc:"0211",libelle:"Cadres des industries de carrières et matériaux (obsolète)"},{idcc:"0045",libelle:"Caoutchouc"},{idcc:"2257",libelle:"Casinos"},{idcc:"0783",libelle:"Centres d'hébergement et de réadaptation sociale"},{idcc:"0953",libelle:"Charcuterie de détail"},{idcc:"1580",libelle:"Chaussure"},{idcc:"2060",libelle:"Chaînes de cafétérias"},{idcc:"1557",libelle:"Commerce des articles de sports et d'équipements de loisirs"},{idcc:"2216",libelle:"Commerce de détail et de gros à prédominance alimentaire"},{idcc:"1505",libelle:"Commerce de détail alimentaire non spécialisé"},{idcc:"2198",libelle:"Commerce à distance et E-commerce"},{idcc:"1483",libelle:"Commerce de détail de l'habillement"},{idcc:"1487",libelle:"Commerce de détail de l'horlogerie-bijouterie"},{idcc:"3237",libelle:"Commerce de détail alimentaire spécialisé"},{idcc:"1225",libelle:"Commerce de la Réunion"},{idcc:"0468",libelle:"Commerce succursaliste de la chaussure"},{idcc:"0573",libelle:"Commerces de gros"},{idcc:"1517",libelle:"Commerces de détail non alimentaires (Codena)"},{idcc:"0500",libelle:"Commerces de gros de l'habillement, mercerie, chaussure et jouet"},{idcc:"3243",libelle:"Commerces de quincaillerie, fournitures industrielles, fers, métaux et équipement de la maison"},{idcc:"2596",libelle:"Coiffure"},{idcc:"1611",libelle:"Communication écrite directe"},{idcc:"1286",libelle:"Confiserie, chocolaterie, biscuiterie"},{idcc:"2583",libelle:"Concessionnaires et exploitants d'autoroutes ou d'ouvrages routiers"},{idcc:"3217",libelle:"Convention collective nationale de la branche ferroviaire"},{idcc:"2272",libelle:"Convention collective nationale de l'assainissement et de la maintenance industrielle"},{idcc:"2002",libelle:"Convention collective interrégionale de la blanchisserie, laverie, location de linge, nettoyage à sec, pressing et teinturerie du 17 novembre 1997"},{idcc:"2247",libelle:"Courtage d'assurances et/ou de réassurances"},{idcc:"0303",libelle:"Couture parisienne et autres métiers de la mode"},{idcc:"0733",libelle:"Détaillants en chaussures"},{idcc:"1605",libelle:"Désinfection, désinsectisation, dératisation"},{idcc:"1536",libelle:"Distributeurs conseils hors domicile"},{idcc:"2372",libelle:"Distribution directe"},{idcc:"1408",libelle:"Distribution, Logistique et Services des Energies de Proximité"},{idcc:"2121",libelle:"Édition"},{idcc:"1518",libelle:"Education, culture, loisirs et animation agissant pour l'utilité sociale et environnementale, au service des territoires (ECLAT)"},{idcc:"2609",libelle:"Employés, techniciens et agents de maîtrise du bâtiment"},{idcc:"2614",libelle:"Employés, techniciens et agents de maîtrise des travaux publics"},{idcc:"0135",libelle:"Employés techniciens et agents de maîtrise des industries de carrières et de matériaux (obsolète)"},{idcc:"3218",libelle:"Enseignement privé non lucratif"},{idcc:"2691",libelle:"Enseignement privé hors contrat"},{idcc:"3043",libelle:"Entreprises de propreté"},{idcc:"3127",libelle:"Entreprises de services à la personne"},{idcc:"1285",libelle:"Entreprises artistiques et culturelles"},{idcc:"1539",libelle:"Entreprises du bureau et du numérique - Commerces et services (Eben)"},{idcc:"1412",libelle:"Entreprises d'installation sans fabrication de matériel aéraulique, thermique, frigorifique"},{idcc:"2717",libelle:"Entreprises techniques au service de la création et de l'évènement"},{idcc:"3032",libelle:"Esthétique"},{idcc:"0029",libelle:"Établissements privés d'hospitalisation, de soins, de cure et de garde à but non lucratif (CCN 51 - FEHAP)"},{idcc:"0413",libelle:"Établissements et services pour personnes inadaptées et handicapées (CCN 66)"},{idcc:"0405",libelle:"Établissements médico-sociaux de l'union intersyndicale des secteurs sanitaires et sociaux (CCN 65)"},{idcc:"0478",libelle:"Établissements financiers"},{idcc:"0915",libelle:"Expertises en matière d'évaluations industrielles et commerciales"},{idcc:"1307",libelle:"Exploitation cinématographique"},{idcc:"1405",libelle:"Expédition et exportation de fruits et légumes"},{idcc:"1411",libelle:"Fabrication de l'ameublement"},{idcc:"0669",libelle:"Fabrication mécanique du verre"},{idcc:"1821",libelle:"Fabrication du verre à la main, semi-automatique et mixte"},{idcc:"1031",libelle:"Fédération nationale des associations familiales rurales"},{idcc:"1978",libelle:"Fleuristes, vente et services des animaux familiers"},{idcc:"0200",libelle:"Froid"},{idcc:"1043",libelle:"Gardiens d'immeubles"},{idcc:"2543",libelle:"Géomètres et experts-fonciers"},{idcc:"2021",libelle:"Golf"},{idcc:"2156",libelle:"Grands magasins"},{idcc:"2336",libelle:"Habitat et du Logement Accompagnés"},{idcc:"1631",libelle:"Hôtellerie de plein air"},{idcc:"1979",libelle:"Hôtels, cafés, restaurants (HCR)"},{idcc:"2264",libelle:"Hospitalisation privée (FHP)"},{idcc:"1921",libelle:"Huissiers de justice"},{idcc:"0044",libelle:"Industries chimiques"},{idcc:"1534",libelle:"Industrie et commerces en gros des viandes"},{idcc:"3233",libelle:"Industrie de la fabrication des ciments"},{idcc:"2089",libelle:"Industrie des panneaux à base de bois"},{idcc:"0176",libelle:"Industrie pharmaceutique"},{idcc:"1388",libelle:"Industrie du pétrole"},{idcc:"0112",libelle:"Industrie laitière"},{idcc:"0018",libelle:"Industrie textile"},{idcc:"3236",libelle:"Industrie et services nautiques"},{idcc:"3109",libelle:"Industries alimentaires diverses"},{idcc:"0247",libelle:"Industries de l'habillement"},{idcc:"2542",libelle:"Industries métallurgiques, mécaniques et connexes de l'Aisne (obsolète)"},{idcc:"3209",libelle:"Industries métallurgiques, mécaniques et connexes du Doubs (obsolète)"},{idcc:"2003",libelle:"Industries métallurgiques, électriques et électroniques des Vosges (obsolète)"},{idcc:"2630",libelle:"Industries métallurgiques des Bouches-du-Rhône et Alpes-de-Haute-Provence (obsolète)"},{idcc:"1396",libelle:"Industries de produits alimentaires élaborés"},{idcc:"0489",libelle:"Industries du cartonnage"},{idcc:"0637",libelle:"Industries et commerce de la récupération"},{idcc:"1938",libelle:"Industries de la transformation des volailles"},{idcc:"1586",libelle:"Industries charcutières"},{idcc:"0184",libelle:"Imprimerie de labeur et industries graphiques"},{idcc:"0043",libelle:"Import-export et commerce international"},{idcc:"1527",libelle:"Immobilier"},{idcc:"0650",libelle:"Ingénieurs et cadres de la métallurgie (obsolète)"},{idcc:"1679",libelle:"Inspection d'assurance"},{idcc:"1794",libelle:"Institutions de retraite complémentaire"},{idcc:"1760",libelle:"Jardineries et graineteries"},{idcc:"1480",libelle:"Journalistes"},{idcc:"0959",libelle:"Laboratoires de biologie médicale extra-hospitaliers"},{idcc:"3013",libelle:"Librairie"},{idcc:"1404",libelle:"Machines et matériels agricoles et de travaux publics (SDLM)"},{idcc:"0675",libelle:"Maisons à succursales de vente au détail d'habillement"},{idcc:"0538",libelle:"Manutention ferroviaire"},{idcc:"2528",libelle:"Maroquinerie"},{idcc:"1589",libelle:"Mareyeurs-expéditeurs"},{idcc:"2931",libelle:"Marchés financiers"},{idcc:"3222",libelle:"Menuiseries charpentes et constructions industrialisées et des portes planes"},{idcc:"0822",libelle:"Mensuels de la métallurgie de la Savoie (obsolète)"},{idcc:"1387",libelle:"Mensuels de la métallurgie des Flandres (obsolète)"},{idcc:"0914",libelle:"Mensuels de la métallurgie de l'Ain (obsolète)"},{idcc:"1930",libelle:"Meunerie"},{idcc:"2190",libelle:"Missions locales et PAIO des maisons de l'emploi et PLIE"},{idcc:"1499",libelle:"Miroiterie, transformation et négoce du verre"},{idcc:"0827",libelle:"Métallurgie des Ardennes (obsolète)"},{idcc:"0863",libelle:"Métallurgie d'Ille-et-Vilaine et du Morbihan (obsolète)"},{idcc:"1867",libelle:"Métallurgie de la Drôme et de l'Ardèche (obsolète)"},{idcc:"0984",libelle:"Métallurgie d'Eure-et-Loir (obsolète)"},{idcc:"2992",libelle:"Métallurgie d'Indre-et-Loire (obsolète)"},{idcc:"0898",libelle:"Métallurgie de l'Allier (obsolète)"},{idcc:"1572",libelle:"Métallurgie de la Charente (obsolète)"},{idcc:"1885",libelle:"Métallurgie de la Côte-d'Or (obsolète)"},{idcc:"1635",libelle:"Métallurgie de la Gironde et des Landes (obsolète)"},{idcc:"1578",libelle:"Métallurgie de la Loire et de l'arrondissement d'Yssingeaux (obsolète)"},{idcc:"0828",libelle:"Métallurgie de la Manche (obsolète)"},{idcc:"0899",libelle:"Métallurgie de la Marne (obsolète)"},{idcc:"1813",libelle:"Métallurgie de la région de Maubeuge (obsolète)"},{idcc:"1525",libelle:"Métallurgie de la région dunkerquoise (obsolète)"},{idcc:"0930",libelle:"Métallurgie de la Sarthe (obsolète)"},{idcc:"0920",libelle:"Métallurgie de la Vienne (obsolète)"},{idcc:"3053",libelle:"Métallurgie de Haute-Saône (obsolète)"},{idcc:"1576",libelle:"Métallurgie du Cher (obsolète)"},{idcc:"0943",libelle:"Métallurgie du Calvados (obsolète)"},{idcc:"0860",libelle:"Métallurgie du Finistère (obsolète)"},{idcc:"2126",libelle:"Métallurgie du Gard et de la Lozère (obsolète)"},{idcc:"1912",libelle:"Métallurgie du Haut-Rhin (obsolète)"},{idcc:"0836",libelle:"Métallurgie de la Haute-Savoie (obsolète)"},{idcc:"0937",libelle:"Métallurgie de la Haute-Vienne et de la Creuse (obsolète)"},{idcc:"1577",libelle:"Métallurgie de l'Hérault, de l'Aude et des Pyrénées-Orientales (obsolète)"},{idcc:"2221",libelle:"Métallurgie de l'Isère et des Hautes-Alpes"},{idcc:"1369",libelle:"Métallurgie de Loire-Atlantique (obsolète)"},{idcc:"2579",libelle:"Métallurgie du Loir-et-Cher (obsolète)"},{idcc:"1966",libelle:"Métallurgie du Loiret (obsolète)"},{idcc:"1902",libelle:"Métallurgie du Maine-et-Loire (obsolète)"},{idcc:"2266",libelle:"Métallurgie de la Mayenne (obsolète)"},{idcc:"1365",libelle:"Métallurgie de Meurthe-et-Moselle (obsolète)"},{idcc:"2755",libelle:"Industries de la métallurgie de Belfort/Montbéliard (obsolète)"},{idcc:"1059",libelle:"Métallurgie des Midi-Pyrénées (obsolète)"},{idcc:"0714",libelle:"Métallurgie de la Moselle (obsolète)"},{idcc:"0948",libelle:"Métallurgie de l'Orne (obsolète)"},{idcc:"2700",libelle:"Métallurgie de l'Oise (obsolète)"},{idcc:"1472",libelle:"Métallurgie du Pas-de-Calais (obsolète)"},{idcc:"2615",libelle:"Métallurgie des Pyrénées-Atlantiques et du Seignanx (obsolète)"},{idcc:"0878",libelle:"Métallurgie du Rhône (obsolète)"},{idcc:"1604",libelle:"Métallurgie de Rouen et de Dieppe (obsolète)"},{idcc:"1564",libelle:"Métallurgie de Saône-et-Loire (obsolète)"},{idcc:"0911",libelle:"Métallurgie de Seine-et-Marne (obsolète)"},{idcc:"2980",libelle:"Métallurgie de la Somme (obsolète)"},{idcc:"1592",libelle:"Métallurgie du Valenciennois et du Cambrésis (obsolète)"},{idcc:"2489",libelle:"Métallurgie de la Vendée (obsolète)"},{idcc:"1634",libelle:"Métallurgie des Côtes-d'Armor (obsolète)"},{idcc:"2630",libelle:"Métallurgie des Bouches-du-Rhône (obsolète)"},{idcc:"1315",libelle:"Industries métallurgiques et mécaniques de la Haute-Marne et de la Meuse (obsolète)"},{idcc:"1732",libelle:"Métallurgie de l'Yonne (obsolète)"},{idcc:"1560",libelle:"Métallurgiques des Alpes-Maritimes (obsolète)"},{idcc:"0979",libelle:"Métallurgiques de l'arrondissement du Havre (obsolète)"},{idcc:"2128",libelle:"Mutualité"},{idcc:"1077",libelle:"Négoce et industrie des produits du sol, engrais et produits connexes"},{idcc:"1880",libelle:"Négoce de l'ameublement"},{idcc:"1982",libelle:"Négoce et prestations de services dans les domaines médico-techniques"},{idcc:"1947",libelle:"Négoce de bois d'oeuvre et produits dérivés (obsolète)"},{idcc:"0054",libelle:"Non-cadres des industries métallurgiques et mécaniques de la région parisienne (obsolète)"},{idcc:"0998",libelle:"Non-cadres de l'exploitation d'équipements thermiques et de génie climatique"},{idcc:"2205",libelle:"Notaires"},{idcc:"3220",libelle:"Offices publics de l'habitat"},{idcc:"3245",libelle:"Opérateurs de voyages et guides"},{idcc:"1431",libelle:"Optique-lunetterie de détail"},{idcc:"1316",libelle:"Organismes de tourisme social et familial"},{idcc:"1909",libelle:"Organismes de tourisme"},{idcc:"1516",libelle:"Organismes de formation"},{idcc:"1790",libelle:"Parcs de loisirs et d'attractions"},{idcc:"1267",libelle:"Pâtisserie"},{idcc:"1000",libelle:"Personnel des cabinets d'avocats"},{idcc:"1147",libelle:"Personnel des cabinets médicaux"},{idcc:"0275",libelle:"Personnel au sol du transport aérien"},{idcc:"2046",libelle:"Personnel non médical des centres de lutte contre le cancer"},{idcc:"2972",libelle:"Personnel sédentaire des entreprises de navigation"},{idcc:"1558",libelle:"Personnel des industries céramiques"},{idcc:"1996",libelle:"Pharmacie d'officine"},{idcc:"1504",libelle:"Poissonnerie"},{idcc:"0759",libelle:"Pompes funèbres"},{idcc:"2683",libelle:"Portage de presse"},{idcc:"3017",libelle:"Ports et Manutention"},{idcc:"3230",libelle:"Presse (Information spécialisée [ETAM et cadres])"},{idcc:"3242",libelle:"Presse quotidienne et hebdomadaire en régions"},{idcc:"2098",libelle:"Prestataires de services du secteur tertiaire"},{idcc:"1351",libelle:"Prévention et sécurité"},{idcc:"1512",libelle:"Promotion immobilière"},{idcc:"0292",libelle:"Plasturgie"},{idcc:"3168",libelle:"Professions de la photographie"},{idcc:"3244",libelle:"Professions réglementées auprès des juridictions"},{idcc:"1555",libelle:"Produits à usage pharmaceutique, parapharmaceutique et vétérinaire"},{idcc:"1513",libelle:"Production des eaux embouteillées, des boissons rafraîchissantes sans alcool et de bière"},{idcc:"2642",libelle:"Production audiovisuelle"},{idcc:"3238",libelle:"Production et transformation des papiers et cartons"},{idcc:"0653",libelle:"Producteurs salariés de base des services extérieurs de production des sociétés d'assurances"},{idcc:"0993",libelle:"Prothèse dentaire"},{idcc:"0086",libelle:"Publicité"},{idcc:"1621",libelle:"Répartition pharmaceutique"},{idcc:"0454",libelle:"Remontées mécaniques et domaines skiables"},{idcc:"1266",libelle:"Restauration de collectivités"},{idcc:"1501",libelle:"Restauration rapide"},{idcc:"1413",libelle:"Salariés permanents des entreprises de travail temporaire"},{idcc:"3216",libelle:"Salariés du négoce des matériaux de construction"},{idcc:"3219",libelle:"Salariés en portage salarial"},{idcc:"1875",libelle:"Salariés des cabinets et cliniques vétérinaires"},{idcc:"0897",libelle:"Services de prévention et de santé au travail interentreprises"},{idcc:"1090",libelle:"Services de l'automobile"},{idcc:"2147",libelle:"Services d'eau et d'assainissement"},{idcc:"2344",libelle:"Sidérurgie (Nord, Moselle, Meurthe-et-Moselle)"},{idcc:"1672",libelle:"Sociétés d'assurances"},{idcc:"1801",libelle:"Sociétés d'assistance"},{idcc:"2150",libelle:"Sociétés anonymes et fondations d'HLM"},{idcc:"3090",libelle:"Spectacle vivant (secteur privé)"},{idcc:"2511",libelle:"Sport"},{idcc:"2728",libelle:"Sucreries, sucreries-distilleries et raffineries de sucre"},{idcc:"2219",libelle:"Taxis parisiens salariés"},{idcc:"2148",libelle:"Télécommunications"},{idcc:"3241",libelle:"Télédiffusion"},{idcc:"1424",libelle:"Transports publics"},{idcc:"0016",libelle:"Transports routiers et activités auxiliaires du transport"},{idcc:"1170",libelle:"Tuiles et briques (obsolète)"},{idcc:"0087",libelle:"Ouvriers des industries de carrières et de matériaux (obsolète)"},{idcc:"1702",libelle:"Ouvriers de travaux publics"},{idcc:"1596",libelle:"Ouvriers des entreprises du bâtiment de moins de 10 salariés"},{idcc:"1597",libelle:"Ouvriers des entreprises du bâtiment de plus de 10 salariés"},{idcc:"2389",libelle:"Ouvriers du bâtiment et des travaux publics région de La Réunion"},{idcc:"2328",libelle:"Ouvriers du bâtiment et des travaux publics de la Guadeloupe et dépendances"},{idcc:"2564",libelle:"Vétérinaires praticiens salariés"},{idcc:"0493",libelle:"Vins, cidres, jus de fruits, sirops, spiritueux et liqueurs de France"}].sort((e,t)=>e.libelle.localeCompare(t.libelle,"fr")),Ue='<option value="">— Choisir une CCN —</option>'+Ee.map(e=>`<option value="${e.idcc}">${e.idcc} — ${e.libelle}</option>`).join("");let R=[];window.forgeNav=function(e){["liste","detail","creer"].forEach(t=>{document.getElementById("forge-"+t).style.display=t===e?"block":"none"})};async function je(){forgeNav("liste");const e=document.getElementById("forge-cards"),t=document.getElementById("forge-subtitle");e.innerHTML='<div style="color:var(--muted);font-size:0.75rem;padding:0.5rem 0">chargement…</div>';try{const l=await fetch("/forge/contributeurs");if(!l.ok){const a=await l.text();throw new Error(`HTTP ${l.status} — ${a||l.statusText}`)}R=await l.json();const i=R.length;t.textContent=i===0?"aucun contributeur pour l'instant":`${i} contributeur${i>1?"s":""} · ${R.reduce((a,n)=>a+n.expertises.length,0)} expertises CCN`,e.innerHTML=i===0?'<div style="color:var(--muted);font-size:0.75rem">Aucun profil encore — sois le premier à rejoindre.</div>':R.map(ze).join("")}catch(l){e.innerHTML=`<div style="color:var(--red);font-size:0.75rem">Erreur : ${b(j(l))}</div>`}}function ze(e){const t=e.expertises.slice(0,5).map(i=>`<span class="ccn-badge ${i.niveau==="Maîtrisée"?"m":i.niveau==="Pratiquée"?"p":"c"}" title="${b(i.niveau)}">${b(i.ccn_libelle)}</span>`).join(""),l=e.expertises.length>5?`<span class="ccn-badge c">+${e.expertises.length-5}</span>`:"";return`
    <div class="forge-card" onclick="forgeAfficherProfil('${b(e.pseudo)}')">
      <div class="forge-card-pseudo">${b(e.pseudo)}</div>
      <div class="forge-card-poste">${b(e.poste)} <span style="color:var(--dim);font-size:0.6em">${e.poste_est_actuel?"actuel":"visé"}</span></div>
      <div class="forge-card-ccn">${t}${l}</div>
      <div class="forge-card-stats">
        <span><span class="forge-stat-val">${e.votes_received}</span> votes</span>
        <span><span class="forge-stat-val">${e.topics_count}</span> sujets</span>
        <span><span class="forge-stat-val">${e.posts_count}</span> réponses</span>
      </div>
    </div>`}async function Ge(e){forgeNav("detail");const t=document.getElementById("forge-profil-content");t.innerHTML='<div style="color:var(--muted);font-size:0.75rem">chargement…</div>';try{let l=R.find(i=>i.pseudo.toLowerCase()===e.toLowerCase());if(!l){const i=await fetch(`/profil/${encodeURIComponent(e)}`);if(!i.ok)throw new Error(`HTTP ${i.status} — ${await i.text()||i.statusText}`);l=await i.json()}t.innerHTML=Ve(l)}catch(l){t.innerHTML=`<div style="color:var(--red);font-size:0.75rem">Erreur : ${b(j(l))}</div>`}}function Ve(e){const t=e.linkedin_url?`<a class="profil-linkedin" href="${b(e.linkedin_url)}" target="_blank" rel="noopener noreferrer">↗ LinkedIn</a>`:"",i=[{niveau:"Maîtrisée",cls:"m",items:e.expertises.filter(s=>s.niveau==="Maîtrisée")},{niveau:"Pratiquée",cls:"p",items:e.expertises.filter(s=>s.niveau==="Pratiquée")},{niveau:"Connue",cls:"c",items:e.expertises.filter(s=>s.niveau==="Connue")}].filter(s=>s.items.length>0).map(s=>`
    <tr class="profil-ccn-section"><td colspan="3">${b(s.niveau)}</td></tr>
    ${s.items.map(o=>`
    <tr>
      <td class="profil-ccn-idcc">${b(o.ccn_idcc)}</td>
      <td>${b(o.ccn_libelle)}</td>
      <td><span class="ccn-badge ${s.cls}">${b(s.niveau)}</span></td>
    </tr>`).join("")}`).join(""),a=e.expertises.length===0?'<div style="color:var(--muted);font-size:0.72rem">Aucune CCN renseignée.</div>':`<table class="profil-ccn-tbl">${i}</table>`,n=e.created_at?Z(e.created_at.slice(0,10)):"—";return`
    <div class="profil-head">
      <div>
        <div class="profil-pseudo">${b(e.pseudo)}</div>
        <div class="profil-poste">${b(e.poste)} <span style="color:var(--dim);font-size:0.85em">(${e.poste_est_actuel?"poste actuel":"poste visé"})</span></div>
        ${t}
      </div>
      <div class="profil-since">membre depuis le ${n}</div>
    </div>

    <div class="profil-body">
      <div class="sect-label">PAIE FRANÇAISE</div>
      ${e.paie_fr_niveau?`<span class="ccn-badge ${e.paie_fr_niveau==="Maîtrisée"?"m":e.paie_fr_niveau==="Pratiquée"?"p":"c"}" style="font-size:0.75rem;padding:0.2rem 0.6rem">${b(e.paie_fr_niveau)}</span>`:'<span style="color:var(--dim);font-size:0.7rem">non renseigné</span>'}

      ${e.pays&&e.pays.length>0?`
      <div class="sect-label" style="margin-top:1rem">PAIE INTERNATIONALE</div>
      <table class="profil-ccn-tbl">
        ${[{niveau:"Maîtrisée",cls:"m",items:e.pays.filter(s=>s.niveau==="Maîtrisée")},{niveau:"Pratiquée",cls:"p",items:e.pays.filter(s=>s.niveau==="Pratiquée")},{niveau:"Connue",cls:"c",items:e.pays.filter(s=>s.niveau==="Connue")}].filter(s=>s.items.length>0).map(s=>`
            <tr class="profil-ccn-section"><td colspan="3">${b(s.niveau)}</td></tr>
            ${s.items.map(o=>`
            <tr>
              <td class="profil-ccn-idcc">${b(o.pays_code)}</td>
              <td>${b(o.pays_libelle)}</td>
              <td><span class="ccn-badge ${s.cls}">${b(s.niveau)}</span></td>
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
    </div>`}window.setPosteType=function(e){document.getElementById("poste_est_actuel_input").value=e?"1":"0",document.getElementById("ptog-actuel").className="ptog "+(e?"ptog-on":"ptog-off"),document.getElementById("ptog-vise").className="ptog "+(e?"ptog-off":"ptog-on")};const xe=[{code:"BE",libelle:"Belgique"},{code:"LU",libelle:"Luxembourg"},{code:"DE",libelle:"Allemagne"},{code:"CH",libelle:"Suisse"},{code:"IT",libelle:"Italie"},{code:"MC",libelle:"Monaco"},{code:"ES",libelle:"Espagne"},{code:"AD",libelle:"Andorre"},{code:"GB",libelle:"Royaume-Uni"}],Ye=xe.map(e=>`<option value="${e.code}">${b(e.libelle)}</option>`).join("");let $e=0;window.forgeAjouterPays=function(){const e=++$e,t=document.createElement("div");t.className="forge-ccn-row",t.id="forge-pays-"+e,t.innerHTML=`
    <select class="forge-pays-select">${Ye}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprPays(${e})" title="Supprimer">×</button>`,document.getElementById("forge-pays-list").appendChild(t)};window.forgeSupprPays=function(e){document.getElementById("forge-pays-"+e)?.remove()};let Ie=0;window.forgeAjouterCcn=function(){const e=++Ie,t=document.createElement("div");t.className="forge-ccn-row",t.id="forge-ccn-"+e,t.innerHTML=`
    <select class="forge-ccn-select">${Ue}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprCcn(${e})" title="Supprimer">×</button>`,document.getElementById("forge-ccn-list").appendChild(t)};window.forgeSupprCcn=function(e){document.getElementById("forge-ccn-"+e)?.remove()};window.forgeSoumettre=async function(e){e.preventDefault();const t=document.getElementById("forge-form"),l=document.getElementById("forge-form-err"),i=document.getElementById("forge-submit-btn");l.textContent="";const a=[];document.querySelectorAll('[id^="forge-pays-"]').forEach(o=>{const d=o.querySelector(".forge-pays-select")?.value,y=o.querySelector(".forge-ccn-niveau")?.value,f=xe.find(c=>c.code===d);d&&f&&a.push({pays_code:d,pays_libelle:f.libelle,niveau:y})});const n=[];document.querySelectorAll('.forge-ccn-row:not([id^="forge-pays-"])').forEach(o=>{const d=o.querySelector(".forge-ccn-select").value,y=o.querySelector(".forge-ccn-niveau").value,f=Ee.find(c=>c.idcc===d);d&&f&&n.push({ccn_idcc:d,ccn_libelle:f.libelle,niveau:y})});const s={email:t.querySelector('[name="email"]').value.trim(),pseudo:t.querySelector('[name="pseudo"]').value.trim(),poste:t.querySelector('[name="poste"]').value.trim(),linkedin_url:t.querySelector('[name="linkedin_url"]').value.trim()||null,poste_est_actuel:t.querySelector('[name="poste_est_actuel"]').value!=="0",paie_fr_niveau:t.querySelector('[name="paie_fr_niveau"]').value||null,pays:a,expertises:n};if(!s.email){l.textContent="Email requis.";return}if(!s.pseudo){l.textContent="Pseudo requis.";return}if(!s.poste){l.textContent="Poste requis.";return}i.disabled=!0,i.textContent="[ envoi… ]";try{const o=await fetch("/forge/profil",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(s)});if(!o.ok)throw new Error(`HTTP ${o.status} — ${await o.text()||o.statusText}`);const d=await o.json();R.unshift(d),t.reset(),document.getElementById("forge-pays-list").innerHTML="",document.getElementById("forge-ccn-list").innerHTML="",$e=0,Ie=0,Ge(d.pseudo)}catch(o){l.textContent=j(o),i.disabled=!1,i.textContent="[ Rejoindre la Forge ]"}};const Je=[{prenom:"Geralt",nom:"de Riv"},{prenom:"Sam",nom:"Vimes"},{prenom:"Elric",nom:"de Melniboné"},{prenom:"Druss",nom:"la Légende"},{prenom:"Logen",nom:"Neuf-Doigts"},{prenom:"Aragorn",nom:"Grands-Pas"},{prenom:"Jon",nom:"Shannow"},{prenom:"Salim",nom:"Dhibi"},{prenom:"Bayaz",nom:"le Magi"},{prenom:"Merlin",nom:"l'Enchanteur"}],Xe=[{prenom:"Lyra",nom:"Belacqua"},{prenom:"Hermione",nom:"Granger"},{prenom:"Eowyn",nom:"du Rohan"},{prenom:"Ellana",nom:"Caldin"},{prenom:"Ferro",nom:"Maljinn"},{prenom:"Magrat",nom:"Garlick"},{prenom:"Ewilan",nom:"Gil'Sayan"},{prenom:"Sigarni",nom:"la Guerrière"},{prenom:"Rikke",nom:"la Nord"},{prenom:"Tanaquil",nom:"la Magicienne"}],ue=[17,16,16,15,15,15,14,14,14,13,13,11],D=ue[Math.floor(Math.random()*ue.length)]/100;let me="H",ee=!1;function pe(e){return e[Math.floor(Math.random()*e.length)]}function Se(e,t){["d-prenom","m-prenom"].forEach(l=>{const i=document.getElementById(l);i&&(i.value=e)}),["d-nom","m-nom"].forEach(l=>{const i=document.getElementById(l);i&&(i.value=t)}),ee=!1}function we(e,t=!1){const l=e==="H";["d-hf-h","m-hf-h"].forEach(i=>{document.getElementById(i)?.classList.toggle("ptog-on",l),document.getElementById(i)?.classList.toggle("ptog-off",!l)}),["d-hf-f","m-hf-f"].forEach(i=>{document.getElementById(i)?.classList.toggle("ptog-on",!l),document.getElementById(i)?.classList.toggle("ptog-off",l)}),t&&document.querySelectorAll(".genre-ecart-hint").forEach(i=>{i.textContent=l?i.dataset.textHf:i.dataset.textFh,i.style.display="inline"})}window.setGenre=function(e){if(e===me)return;if(!ee){const l=e==="F"?window._heroF:window._heroH;Se(l.prenom,l.nom)}const t=e==="F"?1-D:1/(1-D);["d-brut","m-brut"].forEach(l=>{const i=document.getElementById(l);i&&(i.value=Math.round(parseFloat(i.value)*t))}),me=e,we(e,!0)};const te=document.getElementById("burger-btn"),Y=document.getElementById("burger-menu");function We(){te.classList.add("open"),Y.classList.add("open")}window.closeBurger=function(){te.classList.remove("open"),Y.classList.remove("open")};te.addEventListener("click",e=>{e.stopPropagation(),Y.classList.contains("open")?closeBurger():We()});document.addEventListener("click",()=>closeBurger());Y.addEventListener("click",e=>e.stopPropagation());
