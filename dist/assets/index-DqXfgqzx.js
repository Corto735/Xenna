(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const n of document.querySelectorAll('link[rel="modulepreload"]'))s(n);new MutationObserver(n=>{for(const o of n)if(o.type==="childList")for(const i of o.addedNodes)i.tagName==="LINK"&&i.rel==="modulepreload"&&s(i)}).observe(document,{childList:!0,subtree:!0});function l(n){const o={};return n.integrity&&(o.integrity=n.integrity),n.referrerPolicy&&(o.referrerPolicy=n.referrerPolicy),n.crossOrigin==="use-credentials"?o.credentials="include":n.crossOrigin==="anonymous"?o.credentials="omit":o.credentials="same-origin",o}function s(n){if(n.ep)return;n.ep=!0;const o=l(n);fetch(n.href,o)}})();const Ge="modulepreload",Qe=function(e){return"/"+e},me={},Ve=function(t,l,s){let n=Promise.resolve();if(l&&l.length>0){let i=function(v){return Promise.all(v.map(b=>Promise.resolve(b).then(r=>({status:"fulfilled",value:r}),r=>({status:"rejected",reason:r}))))};document.getElementsByTagName("link");const a=document.querySelector("meta[property=csp-nonce]"),c=a?.nonce||a?.getAttribute("nonce");n=i(l.map(v=>{if(v=Qe(v),v in me)return;me[v]=!0;const b=v.endsWith(".css"),r=b?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${v}"]${r}`))return;const u=document.createElement("link");if(u.rel=b?"stylesheet":Ge,b||(u.as="script"),u.crossOrigin="",u.href=v,c&&u.setAttribute("nonce",c),document.head.appendChild(u),b)return new Promise((y,S)=>{u.addEventListener("load",y),u.addEventListener("error",()=>S(new Error(`Unable to preload CSS for ${v}`)))})}))}function o(i){const a=new Event("vite:preloadError",{cancelable:!0});if(a.payload=i,window.dispatchEvent(a),!a.defaultPrevented)throw i}return n.then(i=>{for(const a of i||[])a.status==="rejected"&&o(a.reason);return t().catch(o)})};async function we(e,t={}){if(window.__TAURI_INTERNALS__){const{invoke:s}=await Ve(async()=>{const{invoke:n}=await import("./core-DV6XEvTN.js");return{invoke:n}},[]);return s(e,t)}const l=await fetch(`/api/${e}`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(t)});if(!l.ok)throw await l.text();return l.json()}function X(e){if(e==null)return"(erreur nulle — redémarre l'app ou ouvre DevTools Ctrl+Shift+I)";if(typeof e=="string")return e||"(erreur muette — ouvre DevTools Ctrl+Shift+I et consulte la Console)";if(e instanceof Error)return e.message||e.toString();try{return JSON.stringify(e,null,2)}catch{return String(e)}}let se=null;const ie="2026-01-31",_e=ie;document.addEventListener("DOMContentLoaded",()=>{["d-date","m-date"].forEach(a=>{const c=document.getElementById(a);c&&(c.value=ie,c.max=ie)}),document.addEventListener("keydown",a=>{a.key==="Escape"&&closeFmModal()}),window._heroH=Math.random()<.015?{prenom:"Jean-Noël",nom:"Favari"}:Ie(ut),window._heroF=Ie(mt),ze(window._heroH.prenom,window._heroH.nom),ke("H"),["d-prenom","m-prenom","d-nom","m-nom"].forEach(a=>{document.getElementById(a)?.addEventListener("input",()=>{de=!0})});const e=Math.round(Q*100),t=Math.round(Q/(1-Q)*100);document.querySelectorAll(".genre-ecart-hint").forEach(a=>{a.dataset.textFh=`// −${e} % · écart salarial F/H`,a.dataset.textHf=`// +${t} % · écart salarial H/F`});const l=window.matchMedia("(max-width: 680px)"),s=a=>{const c=document.body;!c.classList.contains("is-annuel")&&!c.classList.contains("is-forge")&&!c.classList.contains("is-apropos")&&!c.classList.contains("is-gaabrielle")&&!c.classList.contains("is-hercule")&&!c.classList.contains("is-quizz")&&setView(a.matches?"mobile":"desktop")};l.addEventListener("change",s),s(l),localStorage.getItem("xenna-hv")&&(document.body.classList.add("hv-mode"),document.getElementById("hv-switch")?.classList.add("on")),localStorage.getItem("xenna-zoom")&&(document.body.classList.add("zoom-mode"),document.documentElement.style.zoom="200%",document.getElementById("zoom-switch")?.classList.add("on"),document.getElementById("a11y-magnifier")?.classList.add("active"));const n=localStorage.getItem("xenna-font");n&&setAppFont(n,!0),localStorage.getItem("xenna-hv")&&document.getElementById("a11y-hv-btn")?.classList.add("active"),localStorage.getItem("xenna-bw")&&(document.body.classList.add("bw-mode"),document.getElementById("bw-switch")?.classList.add("on"),document.getElementById("a11y-bw-btn")?.classList.add("active")),localStorage.getItem("xenna-dactylo")&&(z=!0,document.getElementById("dactylo-switch")?.classList.add("on"));const i=[...["#ff6b6b","#ffd93d","#6bcb77","#4d96ff","#ff922b","#cc5de8","#20c997","#f06595"]].sort(()=>Math.random()-.5);document.querySelectorAll(".a11y-float-btn").forEach((a,c)=>{a.style.setProperty("--wakeup-color",i[c%i.length]),a.classList.add("wakeup"),a.addEventListener("animationend",()=>a.classList.remove("wakeup"),{once:!0})}),document.addEventListener("click",a=>{!a.target.closest("#a11y-btn")&&!a.target.closest("#a11y-panel")&&(document.getElementById("a11y-panel")?.classList.remove("open"),document.getElementById("a11y-btn")?.classList.remove("open"))})});let pe="fr";const le={},U=new Map;function Ye(){const e="script,style,input,select,textarea,.mob-val,.sb-val,.fm-val,.a11y-float,.trad-panel,#a11y-panel",t=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT,{acceptNode(s){const n=s.textContent.trim();return!n||n.length<2||/^[\d\s,.\-+%€×\/:()[\]]+$/.test(n)||s.parentElement?.closest(e)?NodeFilter.FILTER_REJECT:NodeFilter.FILTER_ACCEPT}}),l=[];for(;t.nextNode();)l.push(t.currentNode);return l}window.toggleTradPanel=function(){const e=document.getElementById("trad-panel"),t=document.getElementById("trad-btn"),l=e.classList.toggle("open");t.classList.toggle("open",l)};window.translateApp=async function(e){if(document.getElementById("trad-panel")?.classList.remove("open"),document.getElementById("trad-btn")?.classList.remove("open"),document.querySelectorAll(".trad-lang-btn").forEach(i=>i.classList.remove("active")),document.querySelector(`.trad-lang-btn[onclick="translateApp('${e}')"]`)?.classList.add("active"),e==="fr"){U.forEach((i,a)=>{a.isConnected&&(a.textContent=i)}),document.documentElement.lang="fr",pe="fr";return}const t=document.getElementById("trad-btn");t.classList.add("loading"),t.textContent="🌐 …";const l=Ye();l.forEach(i=>{U.has(i)||U.set(i,i.textContent)});const s=l.map(i=>U.get(i));le[e]||(le[e]=new Map);const n=le[e],o=[...new Set(s)].filter(i=>!n.has(i));try{if(o.length>0)for(let a=0;a<o.length;a+=20){const c=o.slice(a,a+20),v=c.join(`

`),b=`https://api.mymemory.translated.net/get?q=${encodeURIComponent(v)}&langpair=fr|${e}`,r=await fetch(b);if(!r.ok)throw new Error("HTTP "+r.status);const y=(await r.json()).responseData.translatedText.split(`

`);c.forEach((S,I)=>n.set(S,y[I]??S))}l.forEach(i=>{const a=U.get(i);i.isConnected&&n.has(a)&&(i.textContent=n.get(a))}),document.documentElement.lang=e,pe=e}catch(i){console.error("Traduction échouée :",i),t.textContent="🌐 ✗",setTimeout(()=>{t.textContent="🌐 LANGUE",t.classList.remove("loading")},2e3);return}t.textContent="🌐 LANGUE",t.classList.remove("loading")};document.addEventListener("click",e=>{!e.target.closest("#trad-btn")&&!e.target.closest("#trad-panel")&&(document.getElementById("trad-panel")?.classList.remove("open"),document.getElementById("trad-btn")?.classList.remove("open"))});window.toggleA11yPanel=function(){const e=document.getElementById("a11y-panel"),t=document.getElementById("a11y-btn"),l=e.classList.toggle("open");t.classList.toggle("open",l)};window.toggleHVMode=function(){const e=document.body.classList.toggle("hv-mode");document.getElementById("hv-switch")?.classList.toggle("on",e),document.getElementById("a11y-hv-btn")?.classList.toggle("active",e),localStorage.setItem("xenna-hv",e?"1":"")};window.toggleZoom=function(){const e=document.body.classList.toggle("zoom-mode");document.documentElement.style.zoom=e?"200%":"",document.getElementById("zoom-switch")?.classList.toggle("on",e),document.getElementById("a11y-magnifier")?.classList.toggle("active",e),localStorage.setItem("xenna-zoom",e?"1":"")};const fe=new Set,Je=new Set(["IBM Plex Mono","Fira Code","JetBrains Mono","Source Code Pro","Roboto Mono","Inconsolata"]);window.setAppFont=function(e,t=!1){if(!e){document.body.classList.remove("custom-font"),document.documentElement.style.removeProperty("--app-font"),localStorage.removeItem("xenna-font");const n=document.getElementById("font-picker");n&&(n.value="");return}if(!Je.has(e))return;const l=e.replace(/ /g,"+");if(!fe.has(l)){const n=document.createElement("link");n.rel="stylesheet",n.href=`https://fonts.googleapis.com/css2?family=${l}&display=swap`,document.head.appendChild(n),fe.add(l)}document.documentElement.style.setProperty("--app-font",`'${e}', monospace`),document.body.classList.add("custom-font"),localStorage.setItem("xenna-font",e);const s=document.getElementById("font-picker");s&&t&&(s.value=e)};const P=[];window.scan67=function(){const e=Date.now();for(P.push(e);P.length&&e-P[0]>1500;)P.shift();const t=P.length>=3;t&&(P.length=0);const l=t?/42/:/67/,n=Array.from(document.querySelectorAll(".mob-val, .sb-val, .ascii-tbl td, .fm-val, .fm-result td")).filter(i=>l.test(i.textContent.replace(/[\s ]/g,""))&&i.offsetParent!==null);if(n.length===0)return;const o=document.getElementById("a11y-67-btn");if(o.classList.add("active"),t){const i=["#ff0055","#ff6600","#ffcc00","#00ff88","#00ccff","#aa00ff","#ff00cc","#39ff14","#ff4444","#44ffff","#ff69b4","#7fff00"];n.forEach((a,c)=>{setTimeout(()=>{const v=i[Math.floor(Math.random()*i.length)];Object.assign(a.style,{background:v,color:"#000",outline:`2px solid ${v}`,borderRadius:"2px",transition:"all 0.15s"}),setTimeout(()=>Object.assign(a.style,{background:"",color:"",outline:"",borderRadius:""}),900)},c*250)}),setTimeout(()=>o.classList.remove("active"),n.length*250+1e3)}else n.forEach((i,a)=>{setTimeout(()=>{i.classList.remove("flash-67"),i.offsetWidth,i.classList.add("flash-67"),i.addEventListener("animationend",()=>i.classList.remove("flash-67"),{once:!0})},a*500)}),setTimeout(()=>o.classList.remove("active"),n.length*500+200)};window.toggleBWMode=function(){const e=document.body.classList.toggle("bw-mode");document.getElementById("bw-switch")?.classList.toggle("on",e),document.getElementById("a11y-bw-btn")?.classList.toggle("active",e),localStorage.setItem("xenna-bw",e?"1":"")};let z=!1,be=0;const L=e=>new Promise(t=>setTimeout(t,e));window.toggleDactylo=function(){z=!z,document.getElementById("dactylo-switch")?.classList.toggle("on",z),localStorage.setItem("xenna-dactylo",z?"1":"")};async function Xe(e){const t=++be,l=()=>t!==be,s=document.getElementById("res-desktop"),n=s.querySelectorAll("tr.data-row");if(!n.length)return;const i=((e?.salarie?.prenom||"")+(e?.salarie?.nom||"")).toLowerCase().includes("ë"),a=i?2:4,c=i?1:2,v=["#ff00ff","#00ffff","#ff0066","#66ff00","#ff6600","#0066ff","#ff00cc","#00ff99","#ffff00","#ff3399"],b=()=>i?v[Math.floor(Math.random()*v.length)]:"#ffe066",r=s.querySelector("tr.tbl-total");let u=null,y=null,S="",I="";if(r){const x=r.querySelectorAll("td");u=x[1],y=x[3],u&&(S=u.textContent,u.textContent=""),y&&(I=y.textContent,y.textContent="")}const $=[];for(const x of n){const g=x.querySelector("td:first-child > span:last-child");if(g){const h=g.textContent;g.textContent="",$.push({target:g,text:h,ms:a})}x.querySelectorAll("td:not(:first-child)").forEach(h=>{const w=[...h.childNodes].find(m=>m.nodeType===3&&m.textContent.trim());if(w){const m=w.textContent;w.textContent="",$.push({target:w,text:m,ms:c})}}),$.push({pause:i?4:8})}for(const x of $){if(l())return;if(x.pause){await L(x.pause);continue}for(const g of x.text){if(l())return;x.target.textContent+=g,await L(x.ms)}}if(!l()){await L(80);for(const x of n){if(l())return;const g=x.querySelectorAll("td")[3];if(g?.classList.contains("c-sal")){const h=b();g.style.background=h,g.style.color="#000",await L(i?65:110),g.style.background="",g.style.color="",await L(i?10:20)}}if(u){i&&(u.style.fontWeight="bold",u.style.fontSize="1.05em");for(const x of S){if(l())return;u.textContent+=x,await L(c)}i&&ve(u)}if(!l()){await L(80);for(const x of n){if(l())return;const g=x.querySelectorAll("td")[5];if(g?.classList.contains("c-pat")){const h=b();g.style.background=h,g.style.color="#000",await L(i?65:110),g.style.background="",g.style.color="",await L(i?10:20)}}if(y){i&&(y.style.fontWeight="bold",y.style.fontSize="1.05em");for(const x of I){if(l())return;y.textContent+=x,await L(c)}i&&ve(y)}}}}let ge=!1;function We(){if(ge)return;ge=!0;const e=document.createElement("style");e.textContent="@keyframes flameRise{0%{transform:translateY(0) scale(1);opacity:1}100%{transform:translateY(-42px) scale(0);opacity:0}}",document.head.appendChild(e)}function ve(e){We();const t=e.getBoundingClientRect(),l=document.createElement("div");l.style.cssText=`position:fixed;pointer-events:none;z-index:9999;overflow:visible;left:${t.left}px;top:${t.top}px;width:${t.width}px;height:${t.height}px`,document.body.appendChild(l);const s=["#ff8800","#ffdd00","#ff5500","#ffaa00","#ff3300","#ffcc00","#ff6600"],n=setInterval(()=>{const o=document.createElement("div"),i=Math.floor(Math.random()*3)+1,a=(.5+Math.random()*1.2).toFixed(2);o.style.cssText=`position:absolute;pointer-events:none;left:${(Math.random()*t.width).toFixed(1)}px;bottom:0;width:${i}px;height:${i}px;background:${s[Math.floor(Math.random()*s.length)]};animation:flameRise ${a}s ease-out forwards`,l.appendChild(o),setTimeout(()=>o.remove(),parseFloat(a)*1e3+50)},35);setTimeout(()=>{clearInterval(n),setTimeout(()=>l.remove(),1400)},3e3)}function f(e){return String(e).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#039;")}window.setView=function(e){["mobile","desktop","annuel","forge","apropos","contact","gaabrielle","hercule","quizz"].forEach(t=>document.body.classList.toggle("is-"+t,e===t)),document.getElementById("btn-desk").classList.toggle("active",e==="desktop"),document.getElementById("btn-mob").classList.toggle("active",e==="mobile"),document.getElementById("btn-ann").classList.toggle("active",e==="annuel"),se&&(e==="desktop"||e==="mobile")&&qe(se),e==="forge"&&at(),e==="quizz"&&xt()};let Y="EUR";function d(e){const t=parseFloat(e),l=Y==="CHF"?" CHF":" €";return t.toLocaleString("fr-FR",{minimumFractionDigits:2,maximumFractionDigits:2})+l}function Ke(e,t=!1){const l=parseFloat(e),s=Y==="CHF"?" CHF":" €",n=l.toLocaleString("fr-FR",{minimumFractionDigits:2,maximumFractionDigits:2})+s;return t&&l>0?"+"+n:n}function Z(e){return(parseFloat(e)*100).toFixed(2)+" %"}function Le(){const e=document.body.classList.contains("is-mobile")?"m-date":"d-date",t=e==="m-date"?"d-date":"m-date";return document.getElementById(e)?.value||document.getElementById(t)?.value||_e}function ce(e){if(!e)return"—";const[t,l,s]=e.split("-");return`${s}/${l}/${t}`}const Ze=[{min:0,max:1620,taux:0},{min:1620,max:1683,taux:.005},{min:1683,max:1791,taux:.013},{min:1791,max:1911,taux:.021},{min:1911,max:2042,taux:.029},{min:2042,max:2151,taux:.035},{min:2151,max:2294,taux:.041},{min:2294,max:2714,taux:.053},{min:2714,max:3107,taux:.075},{min:3107,max:3539,taux:.099},{min:3539,max:3983,taux:.119},{min:3983,max:4648,taux:.138},{min:4648,max:5574,taux:.158},{min:5574,max:6974,taux:.179},{min:6974,max:8711,taux:.2},{min:8711,max:12091,taux:.24},{min:12091,max:16376,taux:.28},{min:16376,max:25706,taux:.33},{min:25706,max:55062,taux:.38},{min:55062,max:1/0,taux:.43}];function re(e){const t=parseFloat(e);if(isNaN(t)||t<=0)return{total:0,taux_effectif:0,details:[]};let l=0;const s=[];for(const n of Ze){if(t<=n.min)break;const i=+((n.max===1/0?t:Math.min(t,n.max))-n.min).toFixed(2),a=i*n.taux;if(s.push({min:n.min,max:n.max===1/0?null:n.max,taux:n.taux,base:i,montant:+a.toFixed(2)}),l+=a,n.max===1/0||t<=n.max)break}return{total:+l.toFixed(2),taux_effectif:t>0?l/t:0,details:s}}const ye={"Sécurité Sociale":"cat-ss","CSG/CRDS":"cat-csg","Retraite complémentaire":"cat-ret",Prévoyance:"cat-prev",Chômage:"cat-cho",Allègement:"cat-alleg","1er pilier":"cat-ss","Assurance chômage":"cat-cho","Assurance accidents":"cat-acc","Prévoyance maladie":"cat-prev","Prévoyance (LPP)":"cat-ret","Assurance pension":"cat-ret","Assurance maladie":"cat-ss","Assurance dépendance":"cat-prev","Mutualité des employeurs":"cat-ss","Previdenza sociale":"cat-ss",Disoccupazione:"cat-cho","Assicurazione infortuni":"cat-acc","Fine rapporto":"cat-prev",Allegement:"cat-alleg","Bonus IRPEF":"cat-alleg",Imposta:"cat-csg","Imposta regionale":"cat-csg"},F={},he={SS_VIEILLESSE_PLAF:"min(Salaire brut, Plafond Mensuel Sécurité Sociale — PMSS)",CHOMAGE:"min(Salaire brut, 4 × PMSS)",AGS:"min(Salaire brut, 4 × PMSS)",CSG_DEDUCTIBLE:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels (CSS art. L136-2)",CSG_NON_DEDUCTIBLE:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels",CRDS:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels",AGIRC_ARRCO_T1:"min(Salaire brut, PMSS)  — Tranche 1 (entre 0 et 1 PMSS)",AGIRC_ARRCO_CEG_T1:"min(Salaire brut, PMSS)  — Tranche 1",PREVOYANCE_CADRE_MIN:"min(Salaire brut, PMSS)  — Tranche A",AGIRC_ARRCO_T2:"Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2",AGIRC_ARRCO_CEG_T2:"Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2"};function j(e,t=!0){const l=["f","(","x",")"].map((s,n)=>`<span style="animation-delay:${n*45}ms">${s}</span>`).join("");return t?`<span class="formula-star" data-fmkey="${e}" onclick="event.stopPropagation();showFormula('${e}')">${l}</span>`:`<span class="formula-star" aria-hidden="true">${l}</span>`}window.togglePasDetail=function(e){const t=document.getElementById(e);if(!t)return;const l=t.style.display!=="none";t.style.display=l?"none":"block";const s=document.getElementById(e+"-arrow");s&&(s.textContent=l?"▶":"▼")};function G(e,t){if(e.code==="REDUCTION_FILLON")return`<pre class="fm-fillon">${f(e.explication)}</pre>`;const l=t==="sal",s=l?e.taux_sal:e.taux_pat,n=parseFloat(e.base),o=l?parseFloat(e.montant_sal):Math.abs(parseFloat(e.montant_pat)),i=l?"Taux salarial":"Taux patronal",a=l?"Montant salarial":t==="alleg"?"Montant allègement":"Montant patronal",c=l?"c-sal":t==="alleg"?"c-alleg":"c-pat",v=he[e.code]?`<div class="fm-base-note">Assiette  =  ${f(he[e.code])}</div>`:"";return`
    <div class="fm-generic">Montant  =  Assiette  ×  ${i}</div>
    ${v}
    <table class="fm-calc">
      <tr>
        <td>Assiette</td>
        <td class="fm-op">=</td>
        <td class="fm-val c-base">${d(n)}</td>
      </tr>
      <tr>
        <td>${i}</td>
        <td class="fm-op">×</td>
        <td class="fm-val c-taux">${Z(s)}</td>
      </tr>
      <tr class="fm-result fm-sep">
        <td>${a}</td>
        <td class="fm-op">=</td>
        <td class="fm-val ${c}">${d(o)}</td>
      </tr>
    </table>`}function Me(e){const t=re(e);return`
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
      <tbody>${t.details.map(s=>{const n=s.min.toLocaleString("fr-FR")+" €",o=s.max===null?"∞":s.max.toLocaleString("fr-FR")+" €",i=s.taux===0;return`
      <tr class="${i?"pas-zero":""}">
        <td>${n} → ${o}</td>
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
    </table>`}window.showFormula=function(e){const t=F[e];if(!t)return;const l=document.getElementById("fm-body");if(t.type==="pas"){document.getElementById("fm-title").textContent="Prélèvement à la Source (PAS)",document.getElementById("fm-badge").textContent="── Détail par tranche — barème neutre mensuel DGFIP ─────────",l.className="fm-type-pas",l.innerHTML=Me(t.netImposable),document.getElementById("fm-modal").classList.add("open"),document.querySelectorAll(`[data-fmkey="${e}"]`).forEach(a=>a.classList.add("visited"));return}const{c:s,type:n}=t,o=n==="sal",i=s.code==="REDUCTION_FILLON"?"── Allègement patronal ──────────────────────":o?"── Part salariale ───────────────────────────":"── Part patronale ───────────────────────────";document.getElementById("fm-title").textContent=s.libelle,document.getElementById("fm-badge").textContent=i,l.className=`fm-type-${n}`,l.innerHTML=G(s,n),document.getElementById("fm-modal").classList.add("open"),document.querySelectorAll(`[data-fmkey="${e}"]`).forEach(a=>a.classList.add("visited"))};window.closeFmModal=function(){document.getElementById("fm-modal").classList.remove("open")};window.toggleExpl=function(e){const t=document.getElementById(`row-${e}`),l=document.getElementById(`expl-${e}`);if(!t||!l)return;const s=l.style.display!=="none";l.style.display=s?"none":"table-row",t.classList.toggle("open",!s)};function et(e){const t=document.getElementById("res-desktop"),l=e.cotisations,s=["suisse","luxembourg"].includes(e.salarie?.pays),n=l.reduce((p,C)=>p+parseFloat(C.montant_sal),0),o=l.reduce((p,C)=>p+parseFloat(C.montant_pat),0),i=s?{total:0,taux_effectif:0}:re(e.net_imposable),a=parseFloat(e.net_a_payer)-i.total;s||(F.PAS={type:"pas",netImposable:parseFloat(e.net_imposable)});const c=e.salarie?.pays==="suisse"?l.find(p=>p.code==="CH_IS"):null,v=c?parseFloat(c.montant_sal):0,b=c?parseFloat(c.taux_sal):0;c&&(F.CH_IS={c,type:"sal"});const r=n-v,u=`
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
            <span style="color:var(--red)">− ${d(r)}</span>
          </div>
          ${c?`<div class="sb-ded-row">
            <span>Impôt à la source (${(b*100).toFixed(1)} %)</span>
            <span class="fm-val" style="color:var(--purple);cursor:pointer" onclick="showFormula('CH_IS')">− ${d(v)}${j("CH_IS")}</span>
          </div>`:""}
          ${s?"":`<div class="sb-ded-row">
            <span>PAS (${(i.taux_effectif*100).toFixed(1)} %)</span>
            <span class="fm-val" style="color:var(--purple);cursor:pointer" onclick="showFormula('PAS')">− ${d(i.total)}${j("PAS")}</span>
          </div>`}
          <div class="sb-ded-total">
            <span>Total retenues</span>
            <span style="color:var(--red)">− ${d(n+i.total)}</span>
          </div>
        </div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ NET À PAYER</div>
        <div class="sb-val c-green">${d(a)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ CHARGES PAT.</div>
        <div class="sb-val c-orange">${d(o)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ SUPER BRUT</div>
        <div class="sb-val c-yellow">${d(parseFloat(e.brut)+o)}</div>
      </div>
    </div>`,y=l.filter(p=>p.categorie!=="Allègement"&&(parseFloat(p.montant_sal)>0||p.taux_sal!=="0"||parseFloat(p.montant_pat)>0)),S=l.filter(p=>p.categorie==="Allègement"),I=y.reduce((p,C)=>p+parseFloat(C.montant_pat),0);function $(p,C){return p.map((E,_)=>{const q=C+_,B=ye[E.categorie]||"cat-ss",W=parseFloat(E.montant_sal)>0?"c-sal":"c-dim",te=parseFloat(E.montant_pat)>0?"c-pat":"c-dim",H=`${E.code}_sal`,D=`${E.code}_pat`,T=parseFloat(E.montant_sal)>0,K=parseFloat(E.montant_pat)>0;T&&(F[H]={c:E,type:"sal"}),K&&(F[D]={c:E,type:"pat"});const Ue=j(H,T),je=j(D,K);return`
        <tr class="data-row" id="row-${q}" onclick="toggleExpl(${q})">
          <td>
            <span class="expand-icon">▶</span>
            <span class="cat ${B}">[${E.categorie}]</span>
            <span>${E.libelle}</span>
          </td>
          <td class="r">${d(E.base)}</td>
          <td class="r">${parseFloat(E.taux_sal)>0?"− ":""}${Z(E.taux_sal)}</td>
          <td class="r ${W}"${T?` onclick="event.stopPropagation();showFormula('${H}')" style="cursor:pointer"`:""}>${T?"− ":""}${d(E.montant_sal)}${Ue}</td>
          <td class="r">${parseFloat(E.taux_pat)>0?"− ":""}${Z(E.taux_pat)}</td>
          <td class="r ${te}"${K?` onclick="event.stopPropagation();showFormula('${D}')" style="cursor:pointer"`:""}>${K?"− ":""}${d(E.montant_pat)}${je}</td>
        </tr>
        <tr class="expl-row" id="expl-${q}" style="display:none">
          <td colspan="6">
            <div class="expl-box">
              <div class="expl-txt">▸ ${f(E.explication)}</div>
              ${E.loi_ref?`<div class="expl-ref">§ ${f(E.loi_ref)}</div>`:""}
            </div>
          </td>
        </tr>`}).join("")}const x=`
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
      ${x}
      <tbody>
        ${$(y,0)}
        <tr class="tbl-total">
          <td colspan="3">TOTAUX</td>
          <td class="r c-sal">= − ${d(n)}</td>
          <td></td>
          <td class="r c-pat">= − ${d(I)}</td>
        </tr>
      </tbody>
    </table>`,h=`<div class="sim-period">
    SIMULATION AU <span class="sp-accent">${ce(Le())}</span>
    &nbsp;·&nbsp; PMSS en vigueur calculé depuis la base de données sans le moindre état d'âme
  </div>`,w=S.reduce((p,C)=>p+parseFloat(C.montant_pat),0),m=S.length===0?"":`
    <div class="tbl-section-head">── ALLÈGEMENTS PATRONAUX ───────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${x}
      <tbody>
        ${S.map((p,C)=>{const E=y.length+C,_=ye[p.categorie]||"cat-alleg",q=Math.abs(parseFloat(p.montant_pat)),B=`${p.code}_alleg`;return F[B]={c:p,type:"alleg"},`
            <tr class="data-row" id="row-${E}" onclick="toggleExpl(${E})">
              <td>
                <span class="expand-icon">▶</span>
                <span class="cat ${_}">[${p.categorie}]</span>
                <span>${p.libelle}</span>
              </td>
              <td class="r">${d(p.base)}</td>
              <td class="r"></td>
              <td class="r"></td>
              <td class="r c-alleg">${Z(Math.abs(parseFloat(p.taux_pat)))}</td>
              <td class="r c-alleg" onclick="event.stopPropagation();showFormula('${B}')" style="cursor:pointer">− ${d(q)}${j(B)}</td>
            </tr>
            <tr class="expl-row" id="expl-${E}" style="display:none">
              <td colspan="6">
                <div class="expl-box">
                  <div class="expl-txt">▸ ${f(p.explication)}</div>
                  ${p.loi_ref?`<div class="expl-ref">§ ${f(p.loi_ref)}</div>`:""}
                </div>
              </td>
            </tr>`}).join("")}
        <tr class="tbl-total">
          <td colspan="5">TOTAL ALLÈGEMENTS PATRONAUX</td>
          <td class="r c-alleg">− ${d(Math.abs(w))}</td>
        </tr>
      </tbody>
    </table>`;t.innerHTML=h+u+`<div class="tbl-wrap">${g}${m}</div>`}window.mobToggle=function(e,t){const l=document.getElementById("mob-expand-"+e);if(!l)return;const s=["why","how","sal","pat"],n=l.style.display!=="none",o=l.dataset.panel,i=a=>{const c=document.getElementById(`mob-expand-${e}-${a}`);c&&(c.style.display=a===t?"block":"none")};n?o===t?l.style.display="none":(l.dataset.panel=t,s.forEach(i)):(l.style.display="block",l.dataset.panel=t,s.forEach(i))};function tt(e,t,l,s,n,o=0){const i=e.code==="REDUCTION_FILLON"?`<pre class="fm-fillon">${f(e.explication)}</pre>`:`<div class="fm-type-${n}">${G(e,n)}</div>`,a=`
    <div class="mob-exp-txt">${f(e.explication)}</div>
    ${e.loi_ref?`<div class="mob-exp-loi">§ ${f(e.loi_ref)}</div>`:""}`;return`
    <div class="${`mob-stripe-${n}-${o%2===0?"a":"b"}`}">
      <div class="mob-row">
        <span class="mob-lbl mob-cot-lbl"
              title="Explication et référence légale"
              onclick="mobToggle('${t}','why')">${f(e.libelle)}</span>
        <span class="mob-val ${s} mob-cot-amt"
              title="Formule de calcul"
              onclick="mobToggle('${t}','how')">${l}</span>
      </div>
      <div class="mob-expand" id="mob-expand-${t}" style="display:none">
        <div id="mob-expand-${t}-why">${a}</div>
        <div id="mob-expand-${t}-how" style="display:none">${i}</div>
      </div>
    </div>`}function lt(e){const t=document.getElementById("res-mobile"),l=document.getElementById("m-nom")?.value||document.getElementById("d-nom")?.value||"",s=document.getElementById("m-prenom")?.value||document.getElementById("d-prenom")?.value||"",n=e.cotisations,o=["suisse","luxembourg"].includes(e.salarie?.pays),i=n.reduce((m,p)=>m+parseFloat(p.montant_sal),0),a=n.reduce((m,p)=>m+parseFloat(p.montant_pat),0),c=o?{total:0,taux_effectif:0}:re(e.net_imposable),v=parseFloat(e.net_a_payer)-c.total,b=parseFloat(e.brut)+a,r=e.salarie?.pays==="suisse"?n.find(m=>m.code==="CH_IS"):null,u=r?parseFloat(r.montant_sal):0,y=r?parseFloat(r.taux_sal):0,S=i-u,I=n.filter(m=>m.categorie!=="Allègement"&&m.code!=="CH_IS"&&(parseFloat(m.montant_sal)>0||m.taux_sal!=="0"||parseFloat(m.montant_pat)>0)),$=n.filter(m=>m.categorie==="Allègement"),x=I.reduce((m,p)=>m+parseFloat(p.montant_pat),0),g=$.reduce((m,p)=>m+parseFloat(p.montant_pat),0),h=I.map((m,p)=>{const C=parseFloat(m.montant_sal)>0,E=parseFloat(m.montant_pat)>0,_=`${m.code}_u`,q=m.code==="REDUCTION_FILLON",B=C?q?`<pre class="fm-fillon">${f(m.explication)}</pre>`:`<div class="fm-type-sal">${G(m,"sal")}</div>`:"",W=E?q?`<pre class="fm-fillon">${f(m.explication)}</pre>`:`<div class="fm-type-pat">${G(m,"pat")}</div>`:"",te=`
      <div class="mob-exp-txt">${f(m.explication)}</div>
      ${m.loi_ref?`<div class="mob-exp-loi">§ ${f(m.loi_ref)}</div>`:""}`,H=`mob-stripe-sal-${p%2===0?"a":"b"}`,D=C?`<span class="mob-val mob-cot-amt" style="color:#ffe033" onclick="mobToggle('${_}','sal')">− ${d(m.montant_sal)}</span>`:`<span class="mob-val c-dim">0 ${Y==="CHF"?"CHF":"€"}</span>`,T=E?`<span class="mob-val c-orange mob-cot-amt" onclick="mobToggle('${_}','pat')">− ${d(m.montant_pat)}</span>`:`<span class="mob-val c-dim">0 ${Y==="CHF"?"CHF":"€"}</span>`;return`
      <div class="${H}">
        <div class="mob-row">
          <span class="mob-lbl mob-cot-lbl" onclick="mobToggle('${_}','why')">${f(m.libelle)}</span>
          <span style="display:flex;flex-direction:column;align-items:flex-end;gap:0.1rem">${D}${T}</span>
        </div>
        <div class="mob-expand" id="mob-expand-${_}" style="display:none">
          <div id="mob-expand-${_}-why">${te}</div>
          ${B?`<div id="mob-expand-${_}-sal" style="display:none">${B}</div>`:""}
          ${W?`<div id="mob-expand-${_}-pat" style="display:none">${W}</div>`:""}
        </div>
      </div>`}).join(""),w=$.map((m,p)=>tt(m,`${m.code}_alleg`,`− ${d(Math.abs(parseFloat(m.montant_pat)))}`,"c-alleg","alleg",p)).join("");t.innerHTML=`
    <div class="mob-bulletin">

      <!-- En-tête bulletin -->
      <div class="mob-head">
        <span class="mob-head-title">BULLETIN DE PAYE</span>
        <div style="text-align:right">
          <div class="mob-head-name">${f(s)} ${f(l).toUpperCase()}</div>
          <div class="mob-head-date">simulation au ${ce(Le())}</div>
        </div>
      </div>

      <!-- Brut -->
      <div class="mob-row" style="margin-top:0.15rem">
        <span class="mob-lbl">Salaire de base brut</span>
        <span class="mob-val c-gray">${d(e.brut)}</span>
      </div>

      <!-- Cotisations unifiées (salariales + patronales sur une ligne) -->
      <div class="mob-row section"><span class="mob-lbl">── COTISATIONS ──</span><span style="display:flex;gap:1.5rem;font-size:0.62rem;color:var(--muted)"><span>SAL.</span><span>PAT.</span></span></div>
      ${h}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL cotisations sociales</span>
        <span class="mob-val c-red">− ${d(S)}</span>
      </div>
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL charges patronales</span>
        <span class="mob-val c-orange">− ${d(x)}</span>
      </div>

      <!-- Impôt à la source suisse — accordéon dédié -->
      ${r?`<div class="mob-row pas-row" style="cursor:pointer" onclick="togglePasDetail('is-detail-mob')">
        <span class="mob-lbl">Impôt à la source (${(y*100).toFixed(1)} %) <span id="is-detail-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-purple">− ${d(u)}</span>
      </div>
      <div id="is-detail-mob" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        <div class="fm-type-sal">${G(r,"sal")}</div>
        <div class="mob-exp-txt" style="margin-top:0.5rem">${f(r.explication)}</div>
        ${r.loi_ref?`<div class="mob-exp-loi">§ ${f(r.loi_ref)}</div>`:""}
      </div>`:""}

      <!-- Net imposable (France / FPT) -->
      ${o?"":`<div class="mob-row net-row">
        <span class="mob-lbl">NET IMPOSABLE</span>
        <span class="mob-val c-green">${d(e.net_imposable)}</span>
      </div>`}

      <!-- PAS (France / FPT) -->
      ${o?"":`<div class="mob-row pas-row" style="cursor:pointer" onclick="togglePasDetail('pas-detail-mob')">
        <span class="mob-lbl">Prélèvement à la source (${(c.taux_effectif*100).toFixed(1)} %) <span id="pas-detail-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-purple">− ${d(c.total)}</span>
      </div>
      <div id="pas-detail-mob" class="fm-type-pas" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        ${Me(parseFloat(e.net_imposable))}
      </div>`}

      <!-- Net à payer -->
      <div class="mob-row final-row">
        <span class="mob-lbl">NET À PAYER</span>
        <span class="mob-val c-green">${d(v)}</span>
      </div>

      <!-- Allègements -->
      ${w.length?`
      <div class="mob-row section"><span class="mob-lbl">── ALLÈGEMENTS PATRONAUX ──</span><span></span></div>
      ${w}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL allègements</span>
        <span class="mob-val c-alleg">− ${d(Math.abs(g))}</span>
      </div>`:""}

      <!-- Super brut -->
      <div class="mob-row superbrut">
        <span class="mob-lbl">SUPER BRUT (coût employeur)</span>
        <span class="mob-val c-blue">${d(b)}</span>
      </div>

    </div>`}function qe(e){Y=e.devise||"EUR",et(e),lt(e),z&&Xe(e)}function Ee(e){const t=`<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">⚠ ${f(e)}</div>`;document.getElementById("res-desktop").innerHTML=t,document.getElementById("res-mobile").innerHTML=t}async function Be(e){const t=e==="mobile",l=document.getElementById(t?"m-brut":"d-brut").value,s=document.getElementById(t?"m-statut":"d-statut").value,n=document.getElementById(t?"m-nom":"d-nom").value||"Dupont",o=document.getElementById(t?"m-prenom":"d-prenom").value||"Marie",i=document.getElementById(t?"m-date":"d-date").value||_e,a=document.getElementById(t?"m-alsace-moselle":"d-alsace-moselle")?.checked??!1,c=document.getElementById(t?"m-suisse":"d-suisse")?.checked??!1,v=document.getElementById(t?"m-luxembourg":"d-luxembourg")?.checked??!1,b=document.getElementById(t?"m-fpt":"d-fpt")?.checked??!1,r=document.getElementById(t?"m-italie":"d-italie")?.checked??!1,u=document.getElementById(t?"m-assujetti-is":"d-assujetti-is")?.checked??!1,y=document.getElementById(t?"m-canton":"d-canton")?.value||null,S=document.getElementById(t?"m-tarif-is":"d-tarif-is")?.value||null,I=parseFloat(l);if(!l||isNaN(I)||I<=0){Ee("Salaire brut invalide — saisir un montant positif.");return}if(!/^\d{4}-\d{2}-\d{2}$/.test(i)){Ee(`Date invalide : '${i}' (format attendu : YYYY-MM-DD).`);return}["d-brut","m-brut"].forEach(g=>{const h=document.getElementById(g);h&&(h.value=l)}),["d-statut","m-statut"].forEach(g=>{const h=document.getElementById(g);h&&(h.value=s)}),["d-nom","m-nom"].forEach(g=>{const h=document.getElementById(g);h&&(h.value=n)}),["d-prenom","m-prenom"].forEach(g=>{const h=document.getElementById(g);h&&(h.value=o)}),["d-date","m-date"].forEach(g=>{const h=document.getElementById(g);h&&(h.value=i)});const $=c?"suisse":v?"luxembourg":r?"italia":null,x=c||v||r?"2026-01-01":i;try{const g=await we("calculer_bulletin",{salarie:{nom:n,prenom:o,salaire_brut:l.toString(),statut:s,alsace_moselle:a,pays:$??(b?"fonction_publique":"france"),assujetti_is:u,canton:c&&u&&y?y:null,tarif_is:c&&u&&S?S:null,regione:null,contratto_termine:!1},datePaie:x});se=g,qe(g)}catch(g){console.error("[calculer_bulletin] erreur brute :",g);const h=X(g),w=`<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">ERREUR : ${f(h)}</div>`;document.getElementById("res-desktop").innerHTML=w,document.getElementById("res-mobile").innerHTML=w}}function st(e){const t=document.getElementById("res-annuel"),l=e.lignes,s=l.map(b=>b.smic),n=`
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
    </tr></thead>`,o=l.map((b,r)=>{const u=r>0&&b.smic!==s[r-1],y=b.mois_libelle.includes("13e"),S=parseFloat(b.fillon_regularise)-parseFloat(b.fillon_simple),I=Math.abs(S)<.005?'<span style="color:var(--dim)">—</span>':`<span class="delta-nonzero">${S>0?"+":""}${Ke(S.toFixed(2))}</span>`;return`<tr class="${[u?"smic-change":"",y?"treizieme-mois":""].filter(Boolean).join(" ")}">
      <td>${b.mois_libelle}</td>
      <td>${d(b.smic)}</td>
      <td>${d(b.brut)}</td>
      <td class="c-sal">− ${d(b.total_sal)}</td>
      <td class="c-pat">+ ${d(b.total_pat_brut)}</td>
      <td class="c-alleg">− ${d(b.fillon_regularise)}</td>
      <td>${I}</td>
      <td class="c-green">${d(b.net_a_payer)}</td>
      <td class="c-yellow">${d(b.cout_employeur)}</td>
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
    </tr>`,a=parseFloat(e.total_pat_brut),c=parseFloat(e.total_fillon),v=`
    <div style="display:flex;gap:1rem;flex-wrap:wrap;margin-top:0.75rem;font-size:0.72rem">
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">ÉCONOMIE FILLON (annuelle)</div>
        <div style="color:var(--green);font-size:1.1rem;font-weight:bold">− ${d(e.total_fillon)}</div>
      </div>
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">TAUX FILLON MOYEN</div>
        <div style="color:var(--blue);font-size:1.1rem;font-weight:bold">
          ${a>0?(c/parseFloat(e.total_brut)*100).toFixed(2)+" %":"—"}
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
      ${n}
      <tbody>${o}</tbody>
      ${i}
    </table>
    ${v}`}async function it(){const e=parseInt(document.getElementById("a-annee").value),t=document.getElementById("a-brut").value,l=document.getElementById("a-statut").value,s=document.getElementById("res-annuel");if(isNaN(e)||e<1900||e>2100){s.innerHTML='<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Année invalide.</div>';return}const n=parseFloat(t);if(!t||isNaN(n)||n<=0){s.innerHTML='<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Salaire brut invalide — saisir un montant positif.</div>';return}s.innerHTML='<div style="color:var(--muted);padding:1rem;font-size:0.78rem">Calcul en cours…</div>';try{const o=await we("simuler_annee",{annee:e,salaireBrut:t.toString(),statut:l});st(o)}catch(o){console.error("[simuler_annee] erreur brute :",o),s.innerHTML=`<div style="padding:1rem;color:var(--red);font-size:0.8rem">ERREUR : ${f(X(o))}</div>`}}window.onTogglePays=function(e,t){const l=["suisse","luxembourg","fpt","italie"],s=["suisse","luxembourg","italie"],n=l.filter(r=>r!==e);t&&n.forEach(r=>{["d","m"].forEach(u=>{const y=document.getElementById(`${u}-${r}`);y&&y.checked&&(y.checked=!1)})});const o=s.some(r=>document.getElementById(`d-${r}`)?.checked);["d","m"].forEach(r=>{const u=document.getElementById(`${r}-alsace-moselle-wrap`);u&&(u.style.display=o?"none":"");const y=document.getElementById(`${r}-alsace-moselle`);y&&o&&(y.checked=!1)}),["d-date","m-date"].forEach(r=>{const u=document.getElementById(r);u&&(u.disabled=o,o&&(u.value="2026-01-01"))});const i=document.getElementById("d-suisse")?.checked,a=i?"SALAIRE BRUT (CHF)":"SALAIRE BRUT (€)",c=i?"BRUT (CHF)":"BRUT (€)",v=document.getElementById("d-brut");if(v){const r=v.closest(".field")?.querySelector("label");r&&(r.textContent=a)}const b=document.getElementById("m-brut");if(b){const r=b.closest(".field")?.querySelector("label");r&&(r.textContent=c)}["d","m"].forEach(r=>{const u=document.getElementById(`${r}-ch-is-wrap`);if(u)if(i)u.style.display="";else{u.style.display="none";const y=document.getElementById(`${r}-assujetti-is`);y&&(y.checked=!1);const S=document.getElementById(`${r}-ch-is-detail`);S&&(S.style.display="none")}})};window.toggleParams=function(e){const t=document.getElementById(`${e}-params`),l=document.getElementById(`${e}-params-toggle`);if(!t)return;const s=t.style.display!=="none";t.style.display=s?"none":"block",l.classList.toggle("open",!s)};window.syncParam=function(e,t){["d","m"].forEach(l=>{const s=document.getElementById(`${l}-${e}`);s&&(s.type==="checkbox"?s.checked!==t&&(s.checked=t):s.value!==t&&(s.value=t))})};window.onToggleAssujetti=function(e){["d","m"].forEach(t=>{const l=document.getElementById(`${t}-ch-is-detail`);l&&(l.style.display=e?"":"none")})};document.getElementById("d-calc").addEventListener("click",()=>Be("desktop"));document.getElementById("m-calc").addEventListener("click",()=>Be("mobile"));document.getElementById("a-calc").addEventListener("click",it);const Ae=[{idcc:"1261",libelle:"Acteurs du lien social et familial (ALISFA)"},{idcc:"2941",libelle:"Aide, accompagnement, soins et services à domicile"},{idcc:"1747",libelle:"Activités industrielles de boulangerie et de pâtisserie"},{idcc:"2149",libelle:"Activités du déchet"},{idcc:"2335",libelle:"Agences générales d'assurances"},{idcc:"1686",libelle:"Audiovisuel, électronique et équipement ménager"},{idcc:"2120",libelle:"Banque"},{idcc:"3210",libelle:"Banque Populaire"},{idcc:"0567",libelle:"Bijouterie, joaillerie, orfèvrerie (obsolète)"},{idcc:"0158",libelle:"Bois et scieries"},{idcc:"0992",libelle:"Boucherie"},{idcc:"0843",libelle:"Boulangerie-pâtisserie artisanales"},{idcc:"1606",libelle:"Bricolage"},{idcc:"1486",libelle:"Bureaux d'études techniques et sociétés de conseils (Syntec)"},{idcc:"0787",libelle:"Cabinets d'experts-comptables et de commissaires aux comptes"},{idcc:"2332",libelle:"Cabinets d'architectes"},{idcc:"1619",libelle:"Cabinets dentaires"},{idcc:"2420",libelle:"Cadres du bâtiment"},{idcc:"3212",libelle:"Cadres des travaux publics"},{idcc:"1256",libelle:"Cadres des entreprises de gestion d'équipements thermiques et de climatisation"},{idcc:"0211",libelle:"Cadres des industries de carrières et matériaux (obsolète)"},{idcc:"0045",libelle:"Caoutchouc"},{idcc:"2257",libelle:"Casinos"},{idcc:"0783",libelle:"Centres d'hébergement et de réadaptation sociale"},{idcc:"0953",libelle:"Charcuterie de détail"},{idcc:"1580",libelle:"Chaussure"},{idcc:"2060",libelle:"Chaînes de cafétérias"},{idcc:"1557",libelle:"Commerce des articles de sports et d'équipements de loisirs"},{idcc:"2216",libelle:"Commerce de détail et de gros à prédominance alimentaire"},{idcc:"1505",libelle:"Commerce de détail alimentaire non spécialisé"},{idcc:"2198",libelle:"Commerce à distance et E-commerce"},{idcc:"1483",libelle:"Commerce de détail de l'habillement"},{idcc:"1487",libelle:"Commerce de détail de l'horlogerie-bijouterie"},{idcc:"3237",libelle:"Commerce de détail alimentaire spécialisé"},{idcc:"1225",libelle:"Commerce de la Réunion"},{idcc:"0468",libelle:"Commerce succursaliste de la chaussure"},{idcc:"0573",libelle:"Commerces de gros"},{idcc:"1517",libelle:"Commerces de détail non alimentaires (Codena)"},{idcc:"0500",libelle:"Commerces de gros de l'habillement, mercerie, chaussure et jouet"},{idcc:"3243",libelle:"Commerces de quincaillerie, fournitures industrielles, fers, métaux et équipement de la maison"},{idcc:"2596",libelle:"Coiffure"},{idcc:"1611",libelle:"Communication écrite directe"},{idcc:"1286",libelle:"Confiserie, chocolaterie, biscuiterie"},{idcc:"2583",libelle:"Concessionnaires et exploitants d'autoroutes ou d'ouvrages routiers"},{idcc:"3217",libelle:"Convention collective nationale de la branche ferroviaire"},{idcc:"2272",libelle:"Convention collective nationale de l'assainissement et de la maintenance industrielle"},{idcc:"2002",libelle:"Convention collective interrégionale de la blanchisserie, laverie, location de linge, nettoyage à sec, pressing et teinturerie du 17 novembre 1997"},{idcc:"2247",libelle:"Courtage d'assurances et/ou de réassurances"},{idcc:"0303",libelle:"Couture parisienne et autres métiers de la mode"},{idcc:"0733",libelle:"Détaillants en chaussures"},{idcc:"1605",libelle:"Désinfection, désinsectisation, dératisation"},{idcc:"1536",libelle:"Distributeurs conseils hors domicile"},{idcc:"2372",libelle:"Distribution directe"},{idcc:"1408",libelle:"Distribution, Logistique et Services des Energies de Proximité"},{idcc:"2121",libelle:"Édition"},{idcc:"1518",libelle:"Education, culture, loisirs et animation agissant pour l'utilité sociale et environnementale, au service des territoires (ECLAT)"},{idcc:"2609",libelle:"Employés, techniciens et agents de maîtrise du bâtiment"},{idcc:"2614",libelle:"Employés, techniciens et agents de maîtrise des travaux publics"},{idcc:"0135",libelle:"Employés techniciens et agents de maîtrise des industries de carrières et de matériaux (obsolète)"},{idcc:"3218",libelle:"Enseignement privé non lucratif"},{idcc:"2691",libelle:"Enseignement privé hors contrat"},{idcc:"3043",libelle:"Entreprises de propreté"},{idcc:"3127",libelle:"Entreprises de services à la personne"},{idcc:"1285",libelle:"Entreprises artistiques et culturelles"},{idcc:"1539",libelle:"Entreprises du bureau et du numérique - Commerces et services (Eben)"},{idcc:"1412",libelle:"Entreprises d'installation sans fabrication de matériel aéraulique, thermique, frigorifique"},{idcc:"2717",libelle:"Entreprises techniques au service de la création et de l'évènement"},{idcc:"3032",libelle:"Esthétique"},{idcc:"0029",libelle:"Établissements privés d'hospitalisation, de soins, de cure et de garde à but non lucratif (CCN 51 - FEHAP)"},{idcc:"0413",libelle:"Établissements et services pour personnes inadaptées et handicapées (CCN 66)"},{idcc:"0405",libelle:"Établissements médico-sociaux de l'union intersyndicale des secteurs sanitaires et sociaux (CCN 65)"},{idcc:"0478",libelle:"Établissements financiers"},{idcc:"0915",libelle:"Expertises en matière d'évaluations industrielles et commerciales"},{idcc:"1307",libelle:"Exploitation cinématographique"},{idcc:"1405",libelle:"Expédition et exportation de fruits et légumes"},{idcc:"1411",libelle:"Fabrication de l'ameublement"},{idcc:"0669",libelle:"Fabrication mécanique du verre"},{idcc:"1821",libelle:"Fabrication du verre à la main, semi-automatique et mixte"},{idcc:"1031",libelle:"Fédération nationale des associations familiales rurales"},{idcc:"1978",libelle:"Fleuristes, vente et services des animaux familiers"},{idcc:"0200",libelle:"Froid"},{idcc:"1043",libelle:"Gardiens d'immeubles"},{idcc:"2543",libelle:"Géomètres et experts-fonciers"},{idcc:"2021",libelle:"Golf"},{idcc:"2156",libelle:"Grands magasins"},{idcc:"2336",libelle:"Habitat et du Logement Accompagnés"},{idcc:"1631",libelle:"Hôtellerie de plein air"},{idcc:"1979",libelle:"Hôtels, cafés, restaurants (HCR)"},{idcc:"2264",libelle:"Hospitalisation privée (FHP)"},{idcc:"1921",libelle:"Huissiers de justice"},{idcc:"0044",libelle:"Industries chimiques"},{idcc:"1534",libelle:"Industrie et commerces en gros des viandes"},{idcc:"3233",libelle:"Industrie de la fabrication des ciments"},{idcc:"2089",libelle:"Industrie des panneaux à base de bois"},{idcc:"0176",libelle:"Industrie pharmaceutique"},{idcc:"1388",libelle:"Industrie du pétrole"},{idcc:"0112",libelle:"Industrie laitière"},{idcc:"0018",libelle:"Industrie textile"},{idcc:"3236",libelle:"Industrie et services nautiques"},{idcc:"3109",libelle:"Industries alimentaires diverses"},{idcc:"0247",libelle:"Industries de l'habillement"},{idcc:"2542",libelle:"Industries métallurgiques, mécaniques et connexes de l'Aisne (obsolète)"},{idcc:"3209",libelle:"Industries métallurgiques, mécaniques et connexes du Doubs (obsolète)"},{idcc:"2003",libelle:"Industries métallurgiques, électriques et électroniques des Vosges (obsolète)"},{idcc:"2630",libelle:"Industries métallurgiques des Bouches-du-Rhône et Alpes-de-Haute-Provence (obsolète)"},{idcc:"1396",libelle:"Industries de produits alimentaires élaborés"},{idcc:"0489",libelle:"Industries du cartonnage"},{idcc:"0637",libelle:"Industries et commerce de la récupération"},{idcc:"1938",libelle:"Industries de la transformation des volailles"},{idcc:"1586",libelle:"Industries charcutières"},{idcc:"0184",libelle:"Imprimerie de labeur et industries graphiques"},{idcc:"0043",libelle:"Import-export et commerce international"},{idcc:"1527",libelle:"Immobilier"},{idcc:"0650",libelle:"Ingénieurs et cadres de la métallurgie (obsolète)"},{idcc:"1679",libelle:"Inspection d'assurance"},{idcc:"1794",libelle:"Institutions de retraite complémentaire"},{idcc:"1760",libelle:"Jardineries et graineteries"},{idcc:"1480",libelle:"Journalistes"},{idcc:"0959",libelle:"Laboratoires de biologie médicale extra-hospitaliers"},{idcc:"3013",libelle:"Librairie"},{idcc:"1404",libelle:"Machines et matériels agricoles et de travaux publics (SDLM)"},{idcc:"0675",libelle:"Maisons à succursales de vente au détail d'habillement"},{idcc:"0538",libelle:"Manutention ferroviaire"},{idcc:"2528",libelle:"Maroquinerie"},{idcc:"1589",libelle:"Mareyeurs-expéditeurs"},{idcc:"2931",libelle:"Marchés financiers"},{idcc:"3222",libelle:"Menuiseries charpentes et constructions industrialisées et des portes planes"},{idcc:"0822",libelle:"Mensuels de la métallurgie de la Savoie (obsolète)"},{idcc:"1387",libelle:"Mensuels de la métallurgie des Flandres (obsolète)"},{idcc:"0914",libelle:"Mensuels de la métallurgie de l'Ain (obsolète)"},{idcc:"1930",libelle:"Meunerie"},{idcc:"2190",libelle:"Missions locales et PAIO des maisons de l'emploi et PLIE"},{idcc:"1499",libelle:"Miroiterie, transformation et négoce du verre"},{idcc:"0827",libelle:"Métallurgie des Ardennes (obsolète)"},{idcc:"0863",libelle:"Métallurgie d'Ille-et-Vilaine et du Morbihan (obsolète)"},{idcc:"1867",libelle:"Métallurgie de la Drôme et de l'Ardèche (obsolète)"},{idcc:"0984",libelle:"Métallurgie d'Eure-et-Loir (obsolète)"},{idcc:"2992",libelle:"Métallurgie d'Indre-et-Loire (obsolète)"},{idcc:"0898",libelle:"Métallurgie de l'Allier (obsolète)"},{idcc:"1572",libelle:"Métallurgie de la Charente (obsolète)"},{idcc:"1885",libelle:"Métallurgie de la Côte-d'Or (obsolète)"},{idcc:"1635",libelle:"Métallurgie de la Gironde et des Landes (obsolète)"},{idcc:"1578",libelle:"Métallurgie de la Loire et de l'arrondissement d'Yssingeaux (obsolète)"},{idcc:"0828",libelle:"Métallurgie de la Manche (obsolète)"},{idcc:"0899",libelle:"Métallurgie de la Marne (obsolète)"},{idcc:"1813",libelle:"Métallurgie de la région de Maubeuge (obsolète)"},{idcc:"1525",libelle:"Métallurgie de la région dunkerquoise (obsolète)"},{idcc:"0930",libelle:"Métallurgie de la Sarthe (obsolète)"},{idcc:"0920",libelle:"Métallurgie de la Vienne (obsolète)"},{idcc:"3053",libelle:"Métallurgie de Haute-Saône (obsolète)"},{idcc:"1576",libelle:"Métallurgie du Cher (obsolète)"},{idcc:"0943",libelle:"Métallurgie du Calvados (obsolète)"},{idcc:"0860",libelle:"Métallurgie du Finistère (obsolète)"},{idcc:"2126",libelle:"Métallurgie du Gard et de la Lozère (obsolète)"},{idcc:"1912",libelle:"Métallurgie du Haut-Rhin (obsolète)"},{idcc:"0836",libelle:"Métallurgie de la Haute-Savoie (obsolète)"},{idcc:"0937",libelle:"Métallurgie de la Haute-Vienne et de la Creuse (obsolète)"},{idcc:"1577",libelle:"Métallurgie de l'Hérault, de l'Aude et des Pyrénées-Orientales (obsolète)"},{idcc:"2221",libelle:"Métallurgie de l'Isère et des Hautes-Alpes"},{idcc:"1369",libelle:"Métallurgie de Loire-Atlantique (obsolète)"},{idcc:"2579",libelle:"Métallurgie du Loir-et-Cher (obsolète)"},{idcc:"1966",libelle:"Métallurgie du Loiret (obsolète)"},{idcc:"1902",libelle:"Métallurgie du Maine-et-Loire (obsolète)"},{idcc:"2266",libelle:"Métallurgie de la Mayenne (obsolète)"},{idcc:"1365",libelle:"Métallurgie de Meurthe-et-Moselle (obsolète)"},{idcc:"2755",libelle:"Industries de la métallurgie de Belfort/Montbéliard (obsolète)"},{idcc:"1059",libelle:"Métallurgie des Midi-Pyrénées (obsolète)"},{idcc:"0714",libelle:"Métallurgie de la Moselle (obsolète)"},{idcc:"0948",libelle:"Métallurgie de l'Orne (obsolète)"},{idcc:"2700",libelle:"Métallurgie de l'Oise (obsolète)"},{idcc:"1472",libelle:"Métallurgie du Pas-de-Calais (obsolète)"},{idcc:"2615",libelle:"Métallurgie des Pyrénées-Atlantiques et du Seignanx (obsolète)"},{idcc:"0878",libelle:"Métallurgie du Rhône (obsolète)"},{idcc:"1604",libelle:"Métallurgie de Rouen et de Dieppe (obsolète)"},{idcc:"1564",libelle:"Métallurgie de Saône-et-Loire (obsolète)"},{idcc:"0911",libelle:"Métallurgie de Seine-et-Marne (obsolète)"},{idcc:"2980",libelle:"Métallurgie de la Somme (obsolète)"},{idcc:"1592",libelle:"Métallurgie du Valenciennois et du Cambrésis (obsolète)"},{idcc:"2489",libelle:"Métallurgie de la Vendée (obsolète)"},{idcc:"1634",libelle:"Métallurgie des Côtes-d'Armor (obsolète)"},{idcc:"2630",libelle:"Métallurgie des Bouches-du-Rhône (obsolète)"},{idcc:"1315",libelle:"Industries métallurgiques et mécaniques de la Haute-Marne et de la Meuse (obsolète)"},{idcc:"1732",libelle:"Métallurgie de l'Yonne (obsolète)"},{idcc:"1560",libelle:"Métallurgiques des Alpes-Maritimes (obsolète)"},{idcc:"0979",libelle:"Métallurgiques de l'arrondissement du Havre (obsolète)"},{idcc:"2128",libelle:"Mutualité"},{idcc:"1077",libelle:"Négoce et industrie des produits du sol, engrais et produits connexes"},{idcc:"1880",libelle:"Négoce de l'ameublement"},{idcc:"1982",libelle:"Négoce et prestations de services dans les domaines médico-techniques"},{idcc:"1947",libelle:"Négoce de bois d'oeuvre et produits dérivés (obsolète)"},{idcc:"0054",libelle:"Non-cadres des industries métallurgiques et mécaniques de la région parisienne (obsolète)"},{idcc:"0998",libelle:"Non-cadres de l'exploitation d'équipements thermiques et de génie climatique"},{idcc:"2205",libelle:"Notaires"},{idcc:"3220",libelle:"Offices publics de l'habitat"},{idcc:"3245",libelle:"Opérateurs de voyages et guides"},{idcc:"1431",libelle:"Optique-lunetterie de détail"},{idcc:"1316",libelle:"Organismes de tourisme social et familial"},{idcc:"1909",libelle:"Organismes de tourisme"},{idcc:"1516",libelle:"Organismes de formation"},{idcc:"1790",libelle:"Parcs de loisirs et d'attractions"},{idcc:"1267",libelle:"Pâtisserie"},{idcc:"1000",libelle:"Personnel des cabinets d'avocats"},{idcc:"1147",libelle:"Personnel des cabinets médicaux"},{idcc:"0275",libelle:"Personnel au sol du transport aérien"},{idcc:"2046",libelle:"Personnel non médical des centres de lutte contre le cancer"},{idcc:"2972",libelle:"Personnel sédentaire des entreprises de navigation"},{idcc:"1558",libelle:"Personnel des industries céramiques"},{idcc:"1996",libelle:"Pharmacie d'officine"},{idcc:"1504",libelle:"Poissonnerie"},{idcc:"0759",libelle:"Pompes funèbres"},{idcc:"2683",libelle:"Portage de presse"},{idcc:"3017",libelle:"Ports et Manutention"},{idcc:"3230",libelle:"Presse (Information spécialisée [ETAM et cadres])"},{idcc:"3242",libelle:"Presse quotidienne et hebdomadaire en régions"},{idcc:"2098",libelle:"Prestataires de services du secteur tertiaire"},{idcc:"1351",libelle:"Prévention et sécurité"},{idcc:"1512",libelle:"Promotion immobilière"},{idcc:"0292",libelle:"Plasturgie"},{idcc:"3168",libelle:"Professions de la photographie"},{idcc:"3244",libelle:"Professions réglementées auprès des juridictions"},{idcc:"1555",libelle:"Produits à usage pharmaceutique, parapharmaceutique et vétérinaire"},{idcc:"1513",libelle:"Production des eaux embouteillées, des boissons rafraîchissantes sans alcool et de bière"},{idcc:"2642",libelle:"Production audiovisuelle"},{idcc:"3238",libelle:"Production et transformation des papiers et cartons"},{idcc:"0653",libelle:"Producteurs salariés de base des services extérieurs de production des sociétés d'assurances"},{idcc:"0993",libelle:"Prothèse dentaire"},{idcc:"0086",libelle:"Publicité"},{idcc:"1621",libelle:"Répartition pharmaceutique"},{idcc:"0454",libelle:"Remontées mécaniques et domaines skiables"},{idcc:"1266",libelle:"Restauration de collectivités"},{idcc:"1501",libelle:"Restauration rapide"},{idcc:"1413",libelle:"Salariés permanents des entreprises de travail temporaire"},{idcc:"3216",libelle:"Salariés du négoce des matériaux de construction"},{idcc:"3219",libelle:"Salariés en portage salarial"},{idcc:"1875",libelle:"Salariés des cabinets et cliniques vétérinaires"},{idcc:"0897",libelle:"Services de prévention et de santé au travail interentreprises"},{idcc:"1090",libelle:"Services de l'automobile"},{idcc:"2147",libelle:"Services d'eau et d'assainissement"},{idcc:"2344",libelle:"Sidérurgie (Nord, Moselle, Meurthe-et-Moselle)"},{idcc:"1672",libelle:"Sociétés d'assurances"},{idcc:"1801",libelle:"Sociétés d'assistance"},{idcc:"2150",libelle:"Sociétés anonymes et fondations d'HLM"},{idcc:"3090",libelle:"Spectacle vivant (secteur privé)"},{idcc:"2511",libelle:"Sport"},{idcc:"2728",libelle:"Sucreries, sucreries-distilleries et raffineries de sucre"},{idcc:"2219",libelle:"Taxis parisiens salariés"},{idcc:"2148",libelle:"Télécommunications"},{idcc:"3241",libelle:"Télédiffusion"},{idcc:"1424",libelle:"Transports publics"},{idcc:"0016",libelle:"Transports routiers et activités auxiliaires du transport"},{idcc:"1170",libelle:"Tuiles et briques (obsolète)"},{idcc:"0087",libelle:"Ouvriers des industries de carrières et de matériaux (obsolète)"},{idcc:"1702",libelle:"Ouvriers de travaux publics"},{idcc:"1596",libelle:"Ouvriers des entreprises du bâtiment de moins de 10 salariés"},{idcc:"1597",libelle:"Ouvriers des entreprises du bâtiment de plus de 10 salariés"},{idcc:"2389",libelle:"Ouvriers du bâtiment et des travaux publics région de La Réunion"},{idcc:"2328",libelle:"Ouvriers du bâtiment et des travaux publics de la Guadeloupe et dépendances"},{idcc:"2564",libelle:"Vétérinaires praticiens salariés"},{idcc:"0493",libelle:"Vins, cidres, jus de fruits, sirops, spiritueux et liqueurs de France"}].sort((e,t)=>e.libelle.localeCompare(t.libelle,"fr")),nt='<option value="">— Choisir une CCN —</option>'+Ae.map(e=>`<option value="${e.idcc}">${e.idcc} — ${e.libelle}</option>`).join("");let k=[];window.forgeNav=function(e){["liste","detail","creer"].forEach(t=>{document.getElementById("forge-"+t).style.display=t===e?"block":"none"})};async function at(){forgeNav("liste");const e=document.getElementById("forge-cards"),t=document.getElementById("forge-subtitle");e.innerHTML='<div style="color:var(--muted);font-size:0.75rem;padding:0.5rem 0">chargement…</div>';try{const l=await fetch("/forge/contributeurs");if(!l.ok){const n=await l.text();throw new Error(`HTTP ${l.status} — ${n||l.statusText}`)}k=await l.json();const s=k.length;t.textContent=s===0?"aucun contributeur pour l'instant":`${s} contributeur${s>1?"s":""} · ${k.reduce((n,o)=>n+o.expertises.length,0)} expertises CCN`,e.innerHTML=s===0?'<div style="color:var(--muted);font-size:0.75rem">Aucun profil encore — sois le premier à rejoindre.</div>':k.map(ot).join("")}catch(l){e.innerHTML=`<div style="color:var(--red);font-size:0.75rem">Erreur : ${f(X(l))}</div>`}}function ot(e){const t=e.expertises.slice(0,5).map(s=>`<span class="ccn-badge ${s.niveau==="Maîtrisée"?"m":s.niveau==="Pratiquée"?"p":"c"}" title="${f(s.niveau)}">${f(s.ccn_libelle)}</span>`).join(""),l=e.expertises.length>5?`<span class="ccn-badge c">+${e.expertises.length-5}</span>`:"";return`
    <div class="forge-card" onclick="forgeAfficherProfil('${f(e.pseudo)}')">
      <div class="forge-card-pseudo">${f(e.pseudo)}</div>
      <div class="forge-card-poste">${f(e.poste)} <span style="color:var(--dim);font-size:0.6em">${e.poste_est_actuel?"actuel":"visé"}</span></div>
      <div class="forge-card-ccn">${t}${l}</div>
      <div class="forge-card-stats">
        <span><span class="forge-stat-val">${e.votes_received}</span> votes</span>
        <span><span class="forge-stat-val">${e.topics_count}</span> sujets</span>
        <span><span class="forge-stat-val">${e.posts_count}</span> réponses</span>
      </div>
    </div>`}async function ct(e){forgeNav("detail");const t=document.getElementById("forge-profil-content");t.innerHTML='<div style="color:var(--muted);font-size:0.75rem">chargement…</div>';try{let l=k.find(s=>s.pseudo.toLowerCase()===e.toLowerCase());if(!l){const s=await fetch(`/profil/${encodeURIComponent(e)}`);if(!s.ok)throw new Error(`HTTP ${s.status} — ${await s.text()||s.statusText}`);l=await s.json()}t.innerHTML=rt(l)}catch(l){t.innerHTML=`<div style="color:var(--red);font-size:0.75rem">Erreur : ${f(X(l))}</div>`}}function rt(e){const t=e.linkedin_url?`<a class="profil-linkedin" href="${f(e.linkedin_url)}" target="_blank" rel="noopener noreferrer">↗ LinkedIn</a>`:"",s=[{niveau:"Maîtrisée",cls:"m",items:e.expertises.filter(i=>i.niveau==="Maîtrisée")},{niveau:"Pratiquée",cls:"p",items:e.expertises.filter(i=>i.niveau==="Pratiquée")},{niveau:"Connue",cls:"c",items:e.expertises.filter(i=>i.niveau==="Connue")}].filter(i=>i.items.length>0).map(i=>`
    <tr class="profil-ccn-section"><td colspan="3">${f(i.niveau)}</td></tr>
    ${i.items.map(a=>`
    <tr>
      <td class="profil-ccn-idcc">${f(a.ccn_idcc)}</td>
      <td>${f(a.ccn_libelle)}</td>
      <td><span class="ccn-badge ${i.cls}">${f(i.niveau)}</span></td>
    </tr>`).join("")}`).join(""),n=e.expertises.length===0?'<div style="color:var(--muted);font-size:0.72rem">Aucune CCN renseignée.</div>':`<table class="profil-ccn-tbl">${s}</table>`,o=e.created_at?ce(e.created_at.slice(0,10)):"—";return`
    <div class="profil-head">
      <div>
        <div class="profil-pseudo">${f(e.pseudo)}</div>
        <div class="profil-poste">${f(e.poste)} <span style="color:var(--dim);font-size:0.85em">(${e.poste_est_actuel?"poste actuel":"poste visé"})</span></div>
        ${t}
      </div>
      <div class="profil-since">membre depuis le ${o}</div>
    </div>

    <div class="profil-body">
      <div class="sect-label">PAIE FRANÇAISE</div>
      ${e.paie_fr_niveau?`<span class="ccn-badge ${e.paie_fr_niveau==="Maîtrisée"?"m":e.paie_fr_niveau==="Pratiquée"?"p":"c"}" style="font-size:0.75rem;padding:0.2rem 0.6rem">${f(e.paie_fr_niveau)}</span>`:'<span style="color:var(--dim);font-size:0.7rem">non renseigné</span>'}

      ${e.pays&&e.pays.length>0?`
      <div class="sect-label" style="margin-top:1rem">PAIE INTERNATIONALE</div>
      <table class="profil-ccn-tbl">
        ${[{niveau:"Maîtrisée",cls:"m",items:e.pays.filter(i=>i.niveau==="Maîtrisée")},{niveau:"Pratiquée",cls:"p",items:e.pays.filter(i=>i.niveau==="Pratiquée")},{niveau:"Connue",cls:"c",items:e.pays.filter(i=>i.niveau==="Connue")}].filter(i=>i.items.length>0).map(i=>`
            <tr class="profil-ccn-section"><td colspan="3">${f(i.niveau)}</td></tr>
            ${i.items.map(a=>`
            <tr>
              <td class="profil-ccn-idcc">${f(a.pays_code)}</td>
              <td>${f(a.pays_libelle)}</td>
              <td><span class="ccn-badge ${i.cls}">${f(i.niveau)}</span></td>
            </tr>`).join("")}`).join("")}
      </table>`:""}

      <div class="sect-label" style="margin-top:1rem">EXPERTISES CCN</div>
      ${n}
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
    </div>`}window.setPosteType=function(e){document.getElementById("poste_est_actuel_input").value=e?"1":"0",document.getElementById("ptog-actuel").className="ptog "+(e?"ptog-on":"ptog-off"),document.getElementById("ptog-vise").className="ptog "+(e?"ptog-off":"ptog-on")};const Te=[{code:"BE",libelle:"Belgique"},{code:"LU",libelle:"Luxembourg"},{code:"DE",libelle:"Allemagne"},{code:"CH",libelle:"Suisse"},{code:"IT",libelle:"Italie"},{code:"MC",libelle:"Monaco"},{code:"ES",libelle:"Espagne"},{code:"AD",libelle:"Andorre"},{code:"GB",libelle:"Royaume-Uni"}],dt=Te.map(e=>`<option value="${e.code}">${f(e.libelle)}</option>`).join("");let Pe=0;window.forgeAjouterPays=function(){const e=++Pe,t=document.createElement("div");t.className="forge-ccn-row",t.id="forge-pays-"+e,t.innerHTML=`
    <select class="forge-pays-select">${dt}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprPays(${e})" title="Supprimer">×</button>`,document.getElementById("forge-pays-list").appendChild(t)};window.forgeSupprPays=function(e){document.getElementById("forge-pays-"+e)?.remove()};let Fe=0;window.forgeAjouterCcn=function(){const e=++Fe,t=document.createElement("div");t.className="forge-ccn-row",t.id="forge-ccn-"+e,t.innerHTML=`
    <select class="forge-ccn-select">${nt}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprCcn(${e})" title="Supprimer">×</button>`,document.getElementById("forge-ccn-list").appendChild(t)};window.forgeSupprCcn=function(e){document.getElementById("forge-ccn-"+e)?.remove()};window.forgeSoumettre=async function(e){e.preventDefault();const t=document.getElementById("forge-form"),l=document.getElementById("forge-form-err"),s=document.getElementById("forge-submit-btn");l.textContent="";const n=[];document.querySelectorAll('[id^="forge-pays-"]').forEach(a=>{const c=a.querySelector(".forge-pays-select")?.value,v=a.querySelector(".forge-ccn-niveau")?.value,b=Te.find(r=>r.code===c);c&&b&&n.push({pays_code:c,pays_libelle:b.libelle,niveau:v})});const o=[];document.querySelectorAll('.forge-ccn-row:not([id^="forge-pays-"])').forEach(a=>{const c=a.querySelector(".forge-ccn-select").value,v=a.querySelector(".forge-ccn-niveau").value,b=Ae.find(r=>r.idcc===c);c&&b&&o.push({ccn_idcc:c,ccn_libelle:b.libelle,niveau:v})});const i={email:t.querySelector('[name="email"]').value.trim(),pseudo:t.querySelector('[name="pseudo"]').value.trim(),poste:t.querySelector('[name="poste"]').value.trim(),linkedin_url:t.querySelector('[name="linkedin_url"]').value.trim()||null,poste_est_actuel:t.querySelector('[name="poste_est_actuel"]').value!=="0",paie_fr_niveau:t.querySelector('[name="paie_fr_niveau"]').value||null,pays:n,expertises:o};if(!i.email){l.textContent="Email requis.";return}if(!i.pseudo){l.textContent="Pseudo requis.";return}if(!i.poste){l.textContent="Poste requis.";return}s.disabled=!0,s.textContent="[ envoi… ]";try{const a=await fetch("/forge/profil",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(i)});if(!a.ok)throw new Error(`HTTP ${a.status} — ${await a.text()||a.statusText}`);const c=await a.json();k.unshift(c),t.reset(),document.getElementById("forge-pays-list").innerHTML="",document.getElementById("forge-ccn-list").innerHTML="",Pe=0,Fe=0,ct(c.pseudo)}catch(a){l.textContent=X(a),s.disabled=!1,s.textContent="[ Rejoindre la Forge ]"}};const ut=[{prenom:"Geralt",nom:"de Riv"},{prenom:"Sam",nom:"Vimes"},{prenom:"Elric",nom:"de Melniboné"},{prenom:"Druss",nom:"la Légende"},{prenom:"Logen",nom:"Neuf-Doigts"},{prenom:"Aragorn",nom:"Grands-Pas"},{prenom:"Jon",nom:"Shannow"},{prenom:"Salim",nom:"Dhibi"},{prenom:"Bayaz",nom:"le Magi"},{prenom:"Merlin",nom:"l'Enchanteur"}],mt=[{prenom:"Lyra",nom:"Belacqua"},{prenom:"Hermione",nom:"Granger"},{prenom:"Eowyn",nom:"du Rohan"},{prenom:"Ellana",nom:"Caldin"},{prenom:"Ferro",nom:"Maljinn"},{prenom:"Magrat",nom:"Garlick"},{prenom:"Ewilan",nom:"Gil'Sayan"},{prenom:"Sigarni",nom:"la Guerrière"},{prenom:"Rikke",nom:"la Nord"},{prenom:"Tanaquil",nom:"la Magicienne"}],xe=[17,16,16,15,15,15,14,14,14,13,13,11],Q=xe[Math.floor(Math.random()*xe.length)]/100;let Se="H",de=!1;function Ie(e){return e[Math.floor(Math.random()*e.length)]}function ze(e,t){["d-prenom","m-prenom"].forEach(l=>{const s=document.getElementById(l);s&&(s.value=e)}),["d-nom","m-nom"].forEach(l=>{const s=document.getElementById(l);s&&(s.value=t)}),de=!1}function ke(e,t=!1){const l=e==="H";["d-hf-h","m-hf-h"].forEach(s=>{document.getElementById(s)?.classList.toggle("ptog-on",l),document.getElementById(s)?.classList.toggle("ptog-off",!l)}),["d-hf-f","m-hf-f"].forEach(s=>{document.getElementById(s)?.classList.toggle("ptog-on",!l),document.getElementById(s)?.classList.toggle("ptog-off",l)}),t&&document.querySelectorAll(".genre-ecart-hint").forEach(s=>{s.textContent=l?s.dataset.textHf:s.dataset.textFh,s.style.display="inline"})}window.setGenre=function(e){if(e===Se)return;if(!de){const l=e==="F"?window._heroF:window._heroH;ze(l.prenom,l.nom)}const t=e==="F"?1-Q:1/(1-Q);["d-brut","m-brut"].forEach(l=>{const s=document.getElementById(l);s&&(s.value=Math.round(parseFloat(s.value)*t))}),Se=e,ke(e,!0)};const ue=document.getElementById("burger-btn"),ee=document.getElementById("burger-menu");function pt(){ue.classList.add("open"),ee.classList.add("open")}window.closeBurger=function(){ue.classList.remove("open"),ee.classList.remove("open")};ue.addEventListener("click",e=>{e.stopPropagation(),ee.classList.contains("open")?closeBurger():pt()});document.addEventListener("click",()=>closeBurger());ee.addEventListener("click",e=>e.stopPropagation());const ft=[{id:1,q:"Quel est le taux global de la CSG sur les revenus d'activité ?",rep:"9,2 %",mr:["9,4 %","9,6 %","9,8 %"],src:"CSS, art. L136-8"},{id:2,q:"Quelle part de la CSG est déductible de l'impôt sur le revenu ?",rep:"6,8 %",mr:["8,4 %","7,4 %","8,2 %"],src:"CGI, art. 154 quinquies"},{id:3,q:"Quelle part de la CSG est non déductible ?",rep:"2,4 %",mr:["3,2 %","2,8 %","3,0 %"],src:"CGI, art. 154 quinquies"},{id:4,q:"Quel est le taux de la CRDS ?",rep:"0,5 %",mr:["0,4 %","0,37 %","0,45 %"],src:"Ordonnance n°96-50 du 24/01/1996"},{id:5,q:"Sur quelle base se calcule la CSG/CRDS ?",rep:"98,25 % du brut",mr:["97,25 % du brut","98,75 % du brut","99,25 % du brut"],src:"CSS, art. L136-2"},{id:6,q:"Quel est le PMSS mensuel 2024 ?",rep:"3 864 €",mr:["3 666 €","3 925 €","3 428 €"],src:"Arrêté du 19/12/2023"},{id:7,q:"Quel est le PMSS annuel 2024 ?",rep:"46 368 €",mr:["43 992 €","46 836 €","47 004 €"],src:"Arrêté du 19/12/2023"},{id:8,q:"Quel est le nombre de jours de carence avant versement des IJSS maladie ?",rep:"3 jours",mr:["1 jour","7 jours","5 jours"],src:"CSS, art. R323-1"},{id:9,q:"Quel est le taux de base des IJSS maladie ?",rep:"50 %",mr:["60 %","66,66 %","45 %"],src:"CSS, art. R323-4"},{id:10,q:"Quelle est la période de référence retenue pour calculer les IJSS ?",rep:"3 mois",mr:["6 mois","12 mois","1 mois"],src:"CSS, art. R323-4"},{id:11,q:"Quel est le plafond des IJSS maladie ?",rep:"1,8 SMIC",mr:["1,5 SMIC","2 SMIC","1,6 SMIC"],src:"CSS, art. R323-4"},{id:12,q:"Quel est le taux de la cotisation vieillesse plafonnée salarié ?",rep:"6,90 %",mr:["6,70 %","7,10 %","6,60 %"],src:"CSS, art. D242-4"},{id:13,q:"Quel est le taux de la cotisation vieillesse déplafonnée salarié ?",rep:"0,40 %",mr:["0,30 %","0,50 %","0,45 %"],src:"CSS, art. D242-4"},{id:14,q:"Quel est le taux de la cotisation vieillesse plafonnée employeur ?",rep:"8,55 %",mr:["8,45 %","8,75 %","8,20 %"],src:"CSS, art. D242-4"},{id:15,q:"Quel est le taux normal des allocations familiales ?",rep:"5,25 %",mr:["5,40 %","4,90 %","5,10 %"],src:"CSS, art. L241-6"},{id:16,q:"En dessous de quel seuil (en SMIC) s'applique le taux réduit des allocations familiales ?",rep:"3,5 SMIC",mr:["3 SMIC","2,5 SMIC","4 SMIC"],src:"CSS, art. L241-6-1"},{id:17,q:"Quel est le taux réduit des allocations familiales ?",rep:"3,45 %",mr:["3,25 %","3,75 %","3,60 %"],src:"CSS, art. D241-3-2"},{id:18,q:"Quel est le taux de la contribution solidarité autonomie (CSA) ?",rep:"0,30 %",mr:["0,10 %","0,50 %","0,25 %"],src:"CASF, art. L14-10-4"},{id:19,q:"Quel est le taux du FNAL pour les entreprises de moins de 50 salariés ?",rep:"0,10 %",mr:["0,30 %","0,20 %","0,50 %"],src:"CSS, art. L834-1"},{id:20,q:"Quel est le taux du FNAL pour les entreprises d'au moins 50 salariés ?",rep:"0,50 %",mr:["0,30 %","0,10 %","0,40 %"],src:"CSS, art. L834-1"},{id:21,q:"Comment est déterminé le taux AT/MP ?",rep:"Variable",mr:["Fixé à 0,70 % pour tous","Forfait de 2 % du brut","Fixé légalement à 1 %"],src:"CSS, art. L242-5"},{id:22,q:"Quel est le SMIC mensuel brut 2024 (base 35h) ?",rep:"1 766,92 €",mr:["1 709,28 €","1 801,80 €","1 747,20 €"],src:"Décret n°2023-1216"},{id:23,q:"Quelle est la durée mensuelle de travail pour 35h hebdomadaires ?",rep:"151,67 h",mr:["152,25 h","150,50 h","153,33 h"],src:"Code du travail, art. L3121-27"},{id:24,q:"Quel est le taux de majoration pour les 8 premières heures supplémentaires ?",rep:"25 %",mr:["10 %","20 %","30 %"],src:"Code du travail, art. L3121-36"},{id:25,q:"Quel est le taux de majoration pour les heures supplémentaires au-delà des 8 premières ?",rep:"50 %",mr:["25 %","40 %","75 %"],src:"Code du travail, art. L3121-36"},{id:26,q:"Quel est le plafond annuel d'exonération fiscale et sociale sur les heures supplémentaires ?",rep:"7 500 €",mr:["5 000 €","7 000 €","8 000 €"],src:"CGI, art. 81 quater"},{id:27,q:"La réduction générale de cotisations patronales est-elle fixe ou variable ?",rep:"Variable",mr:["Fixée à 16 % du brut","Plafonnée à 26 % pour tous","Identique quel que soit l'effectif"],src:"CSS, art. L241-13"},{id:28,q:"Quel est le taux maximum de la réduction Fillon ?",rep:"~32 %",mr:["~26 %","~28 %","~35 %"],src:"CSS, art. D241-7"},{id:29,q:"Quel est le taux de cotisation chômage à la charge du salarié ?",rep:"0 %",mr:["2,40 %","0,95 %","1,20 %"],src:"Loi n°2018-771"},{id:30,q:"Quel est le taux de cotisation chômage à la charge de l'employeur ?",rep:"4,05 %",mr:["3,45 %","4,40 %","3,90 %"],src:"Convention Unédic"}];let M=null,N=null,J=0,ne=0,ae=0,Ce=-1,A=!1,R=null,oe=!1,O=!1,V=0;function $e(e){return String(e).toLowerCase().replace(/\s+/g,"").replace(/,/g,".").replace(/[€%°~]/g,"").trim()}function bt(e,t){if(!t.trim())return!1;const l=$e(e),s=$e(t);if(l===s)return!0;const n=parseFloat(l),o=parseFloat(s);return!isNaN(n)&&!isNaN(o)&&Math.abs(n-o)<.001}function Ne(e){const t=[...e];for(let l=t.length-1;l>0;l--){const s=Math.floor(Math.random()*(l+1));[t[l],t[s]]=[t[s],t[l]]}return t}function Re(e){return(e/1e3).toFixed(1)+"s"}function gt(){return N&&(clearInterval(N),N=null),Date.now()-J}function vt(e){M=e,A=!1,document.getElementById("qz-num").textContent="Q."+String(e.id).padStart(2,"0"),document.getElementById("qz-q").textContent=e.q,document.getElementById("qz-clock").textContent="0.0s",document.getElementById("qz-input").value="",document.getElementById("qz-input").disabled=!1,document.getElementById("qz-result").style.display="none",document.getElementById("qz-saisie").style.opacity="1";const t=document.getElementById("qz-carre-cb");t.checked=!1,document.getElementById("qz-carre").style.display="none",document.getElementById("qz-carre").style.opacity="1",document.getElementById("qz-fifty").style.display="none",R&&clearTimeout(R),oe=!1,O=!1,R=setTimeout(()=>{oe=!0,!A&&!O&&document.getElementById("qz-carre").style.display!=="none"&&(document.getElementById("qz-fifty").style.display="inline-flex")},8e3);const l=Ne([e.rep,...e.mr]),s=document.getElementById("qz-choix");s.innerHTML="",l.forEach(n=>{const o=document.createElement("button");o.className="qz-choice",o.textContent=n,o.onclick=()=>He(n),s.appendChild(o)}),N&&clearInterval(N),J=Date.now(),N=setInterval(()=>{A||(document.getElementById("qz-clock").textContent=Re(Date.now()-J))},100),document.getElementById("qz-input").focus()}function Oe(e,t,l){A=!0,gt(),ae++,e&&ne++,R&&(clearTimeout(R),R=null),document.getElementById("qz-fifty").style.display="none";const s=V>0?" · ½ ×"+V:"";document.getElementById("qz-score").textContent=ne+" / "+ae+s;const n=document.getElementById("qz-verdict");n.textContent=e?"✓ JUSTE":"✗ FAUX",n.className="qz-verdict "+(e?"ok":"ko");const o=document.getElementById("qz-ans-line");e?o.innerHTML="Bonne réponse : <strong>"+M.rep+"</strong>":o.innerHTML="Réponse correcte : <strong>"+M.rep+"</strong>";const i=O?" · ½":"";document.getElementById("qz-time-line").textContent="⏱ "+Re(t)+" ("+l+i+")",document.getElementById("qz-src-line").textContent=M.src,document.getElementById("qz-result").style.display="block",document.getElementById("qz-input").disabled=!0,document.getElementById("qz-saisie").style.opacity="0.4",document.querySelectorAll(".qz-choice").forEach(a=>{a.disabled=!0,a.textContent===M.rep&&a.classList.add("qz-correct")})}function yt(e){document.getElementById("qz-carre").style.display=e.checked?"block":"none",e.checked&&oe&&!A&&!O&&(document.getElementById("qz-fifty").style.display="inline-flex")}function ht(){if(O||A)return;O=!0,V++,document.getElementById("qz-fifty").style.display="none";const t=Array.from(document.querySelectorAll(".qz-choice")).filter(s=>s.textContent!==M.rep);Ne(t).slice(0,2).forEach(s=>{s.disabled=!0,s.style.opacity="0.12",s.style.pointerEvents="none"});const l=V>0?" · ½ ×"+V:"";document.getElementById("qz-score").textContent=ne+" / "+ae+l}function Et(){if(A)return;const e=document.getElementById("qz-input").value,t=Date.now()-J,l=bt(M.rep,e);document.querySelectorAll(".qz-choice").forEach(s=>{s.disabled=!0,s.textContent===M.rep&&s.classList.add("qz-correct")}),Oe(l,t,"saisie")}function He(e){if(A)return;const t=Date.now()-J,l=e===M.rep;document.querySelectorAll(".qz-choice").forEach(s=>{s.disabled=!0,s.textContent===M.rep?s.classList.add("qz-correct"):s.textContent===e&&!l&&s.classList.add("qz-wrong")}),Oe(l,t,"carré")}function De(){const e=ft.filter(l=>l.id!==Ce),t=e[Math.floor(Math.random()*e.length)];Ce=t.id,vt(t)}function xt(){De()}window.quizzValider=Et;window.quizzChoix=He;window.quizzNext=De;window.quizzToggleCarre=yt;window.quizzFiftyFifty=ht;
