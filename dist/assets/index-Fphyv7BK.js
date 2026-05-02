(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))s(i);new MutationObserver(i=>{for(const o of i)if(o.type==="childList")for(const a of o.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&s(a)}).observe(document,{childList:!0,subtree:!0});function l(i){const o={};return i.integrity&&(o.integrity=i.integrity),i.referrerPolicy&&(o.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?o.credentials="include":i.crossOrigin==="anonymous"?o.credentials="omit":o.credentials="same-origin",o}function s(i){if(i.ep)return;i.ep=!0;const o=l(i);fetch(i.href,o)}})();const Ke="modulepreload",Ze=function(e){return"/"+e},he={},et=function(t,l,s){let i=Promise.resolve();if(l&&l.length>0){let a=function(f){return Promise.all(f.map(p=>Promise.resolve(p).then(x=>({status:"fulfilled",value:x}),x=>({status:"rejected",reason:x}))))};document.getElementsByTagName("link");const n=document.querySelector("meta[property=csp-nonce]"),c=n?.nonce||n?.getAttribute("nonce");i=a(l.map(f=>{if(f=Ze(f),f in he)return;he[f]=!0;const p=f.endsWith(".css"),x=p?'[rel="stylesheet"]':"";if(document.querySelector(`link[href="${f}"]${x}`))return;const b=document.createElement("link");if(b.rel=p?"stylesheet":Ke,p||(b.as="script"),b.crossOrigin="",b.href=f,c&&b.setAttribute("nonce",c),document.head.appendChild(b),p)return new Promise((d,v)=>{b.addEventListener("load",d),b.addEventListener("error",()=>v(new Error(`Unable to preload CSS for ${f}`)))})}))}function o(a){const n=new Event("vite:preloadError",{cancelable:!0});if(n.payload=a,window.dispatchEvent(n),!n.defaultPrevented)throw a}return i.then(a=>{for(const n of a||[])n.status==="rejected"&&o(n.reason);return t().catch(o)})};async function Te(e,t={}){if(window.__TAURI_INTERNALS__){const{invoke:s}=await et(async()=>{const{invoke:i}=await import("./core-DV6XEvTN.js");return{invoke:i}},[]);return s(e,t)}const l=await fetch(`/api/${e}`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(t)});if(!l.ok)throw await l.text();return l.json()}function Z(e){if(e==null)return"(erreur nulle — redémarre l'app ou ouvre DevTools Ctrl+Shift+I)";if(typeof e=="string")return e||"(erreur muette — ouvre DevTools Ctrl+Shift+I et consulte la Console)";if(e instanceof Error)return e.message||e.toString();try{return JSON.stringify(e,null,2)}catch{return String(e)}}let de=null;const ue="2026-01-31",Fe=ue;document.addEventListener("DOMContentLoaded",()=>{["d-date","m-date"].forEach(n=>{const c=document.getElementById(n);c&&(c.value=ue,c.max=ue)}),document.addEventListener("keydown",n=>{n.key==="Escape"&&closeFmModal()}),window._heroH=Math.random()<.015?{prenom:"Jean-Noël",nom:"Favari"}:Be(yt),window._heroF=Be(ht),Ue(window._heroH.prenom,window._heroH.nom),je("H"),["d-prenom","m-prenom","d-nom","m-nom"].forEach(n=>{document.getElementById(n)?.addEventListener("input",()=>{ve=!0})});const e=Math.round(J*100),t=Math.round(J/(1-J)*100);document.querySelectorAll(".genre-ecart-hint").forEach(n=>{n.dataset.textFh=`// −${e} % · écart salarial F/H`,n.dataset.textHf=`// +${t} % · écart salarial H/F`});const l=window.matchMedia("(max-width: 680px)"),s=n=>{const c=document.body;!c.classList.contains("is-annuel")&&!c.classList.contains("is-forge")&&!c.classList.contains("is-apropos")&&!c.classList.contains("is-gaabrielle")&&!c.classList.contains("is-hercule")&&!c.classList.contains("is-quizz")&&setView(n.matches?"mobile":"desktop")};l.addEventListener("change",s),s(l),localStorage.getItem("xenna-hv")&&(document.body.classList.add("hv-mode"),document.getElementById("hv-switch")?.classList.add("on")),localStorage.getItem("xenna-zoom")&&(document.body.classList.add("zoom-mode"),document.documentElement.style.zoom="200%",document.getElementById("zoom-switch")?.classList.add("on"),document.getElementById("a11y-magnifier")?.classList.add("active"));const i=localStorage.getItem("xenna-font");i&&setAppFont(i,!0),localStorage.getItem("xenna-hv")&&document.getElementById("a11y-hv-btn")?.classList.add("active"),localStorage.getItem("xenna-bw")&&(document.body.classList.add("bw-mode"),document.getElementById("bw-switch")?.classList.add("on"),document.getElementById("a11y-bw-btn")?.classList.add("active")),localStorage.getItem("xenna-dactylo")&&(N=!0,document.getElementById("dactylo-switch")?.classList.add("on"));const a=[...["#ff6b6b","#ffd93d","#6bcb77","#4d96ff","#ff922b","#cc5de8","#20c997","#f06595"]].sort(()=>Math.random()-.5);document.querySelectorAll(".a11y-float-btn").forEach((n,c)=>{n.style.setProperty("--wakeup-color",a[c%a.length]),n.classList.add("wakeup"),n.addEventListener("animationend",()=>n.classList.remove("wakeup"),{once:!0})}),document.addEventListener("click",n=>{!n.target.closest("#a11y-btn")&&!n.target.closest("#a11y-panel")&&(document.getElementById("a11y-panel")?.classList.remove("open"),document.getElementById("a11y-btn")?.classList.remove("open"))})});let Ee="fr";const re={},Q=new Map;function tt(){const e="script,style,input,select,textarea,.mob-val,.sb-val,.fm-val,.a11y-float,.trad-panel,#a11y-panel",t=document.createTreeWalker(document.body,NodeFilter.SHOW_TEXT,{acceptNode(s){const i=s.textContent.trim();return!i||i.length<2||/^[\d\s,.\-+%€×\/:()[\]]+$/.test(i)||s.parentElement?.closest(e)?NodeFilter.FILTER_REJECT:NodeFilter.FILTER_ACCEPT}}),l=[];for(;t.nextNode();)l.push(t.currentNode);return l}window.toggleTradPanel=function(){const e=document.getElementById("trad-panel"),t=document.getElementById("trad-btn"),l=e.classList.toggle("open");t.classList.toggle("open",l)};window.translateApp=async function(e){if(document.getElementById("trad-panel")?.classList.remove("open"),document.getElementById("trad-btn")?.classList.remove("open"),document.querySelectorAll(".trad-lang-btn").forEach(a=>a.classList.remove("active")),document.querySelector(`.trad-lang-btn[onclick="translateApp('${e}')"]`)?.classList.add("active"),e==="fr"){Q.forEach((a,n)=>{n.isConnected&&(n.textContent=a)}),document.documentElement.lang="fr",Ee="fr";return}const t=document.getElementById("trad-btn");t.classList.add("loading"),t.textContent="🌐 …";const l=tt();l.forEach(a=>{Q.has(a)||Q.set(a,a.textContent)});const s=l.map(a=>Q.get(a));re[e]||(re[e]=new Map);const i=re[e],o=[...new Set(s)].filter(a=>!i.has(a));try{if(o.length>0)for(let n=0;n<o.length;n+=20){const c=o.slice(n,n+20),f=c.join(`

`),p=`https://api.mymemory.translated.net/get?q=${encodeURIComponent(f)}&langpair=fr|${e}`,x=await fetch(p);if(!x.ok)throw new Error("HTTP "+x.status);const d=(await x.json()).responseData.translatedText.split(`

`);c.forEach((v,y)=>i.set(v,d[y]??v))}l.forEach(a=>{const n=Q.get(a);a.isConnected&&i.has(n)&&(a.textContent=i.get(n))}),document.documentElement.lang=e,Ee=e}catch(a){console.error("Traduction échouée :",a),t.textContent="🌐 ✗",setTimeout(()=>{t.textContent="🌐 LANGUE",t.classList.remove("loading")},2e3);return}t.textContent="🌐 LANGUE",t.classList.remove("loading")};document.addEventListener("click",e=>{!e.target.closest("#trad-btn")&&!e.target.closest("#trad-panel")&&(document.getElementById("trad-panel")?.classList.remove("open"),document.getElementById("trad-btn")?.classList.remove("open"))});window.toggleA11yPanel=function(){const e=document.getElementById("a11y-panel"),t=document.getElementById("a11y-btn"),l=e.classList.toggle("open");t.classList.toggle("open",l)};window.toggleHVMode=function(){const e=document.body.classList.toggle("hv-mode");document.getElementById("hv-switch")?.classList.toggle("on",e),document.getElementById("a11y-hv-btn")?.classList.toggle("active",e),localStorage.setItem("xenna-hv",e?"1":"")};window.toggleZoom=function(){const e=document.body.classList.toggle("zoom-mode");document.documentElement.style.zoom=e?"200%":"",document.getElementById("zoom-switch")?.classList.toggle("on",e),document.getElementById("a11y-magnifier")?.classList.toggle("active",e),localStorage.setItem("xenna-zoom",e?"1":"")};const xe=new Set,lt=new Set(["IBM Plex Mono","Fira Code","JetBrains Mono","Source Code Pro","Roboto Mono","Inconsolata"]);window.setAppFont=function(e,t=!1){if(!e){document.body.classList.remove("custom-font"),document.documentElement.style.removeProperty("--app-font"),localStorage.removeItem("xenna-font");const i=document.getElementById("font-picker");i&&(i.value="");return}if(!lt.has(e))return;const l=e.replace(/ /g,"+");if(!xe.has(l)){const i=document.createElement("link");i.rel="stylesheet",i.href=`https://fonts.googleapis.com/css2?family=${l}&display=swap`,document.head.appendChild(i),xe.add(l)}document.documentElement.style.setProperty("--app-font",`'${e}', monospace`),document.body.classList.add("custom-font"),localStorage.setItem("xenna-font",e);const s=document.getElementById("font-picker");s&&t&&(s.value=e)};const k=[];window.scan67=function(){const e=Date.now();for(k.push(e);k.length&&e-k[0]>1500;)k.shift();const t=k.length>=3;t&&(k.length=0);const l=t?/42/:/67/,i=Array.from(document.querySelectorAll(".mob-val, .sb-val, .ascii-tbl td, .fm-val, .fm-result td")).filter(a=>l.test(a.textContent.replace(/[\s ]/g,""))&&a.offsetParent!==null);if(i.length===0)return;const o=document.getElementById("a11y-67-btn");if(o.classList.add("active"),t){const a=["#ff0055","#ff6600","#ffcc00","#00ff88","#00ccff","#aa00ff","#ff00cc","#39ff14","#ff4444","#44ffff","#ff69b4","#7fff00"];i.forEach((n,c)=>{setTimeout(()=>{const f=a[Math.floor(Math.random()*a.length)];Object.assign(n.style,{background:f,color:"#000",outline:`2px solid ${f}`,borderRadius:"2px",transition:"all 0.15s"}),setTimeout(()=>Object.assign(n.style,{background:"",color:"",outline:"",borderRadius:""}),900)},c*250)}),setTimeout(()=>o.classList.remove("active"),i.length*250+1e3)}else i.forEach((a,n)=>{setTimeout(()=>{a.classList.remove("flash-67"),a.offsetWidth,a.classList.add("flash-67"),a.addEventListener("animationend",()=>a.classList.remove("flash-67"),{once:!0})},n*500)}),setTimeout(()=>o.classList.remove("active"),i.length*500+200)};window.toggleBWMode=function(){const e=document.body.classList.toggle("bw-mode");document.getElementById("bw-switch")?.classList.toggle("on",e),document.getElementById("a11y-bw-btn")?.classList.toggle("active",e),localStorage.setItem("xenna-bw",e?"1":"")};let N=!1,Ie=0;const B=e=>new Promise(t=>setTimeout(t,e));window.toggleDactylo=function(){N=!N,document.getElementById("dactylo-switch")?.classList.toggle("on",N),localStorage.setItem("xenna-dactylo",N?"1":"")};async function st(e){const t=++Ie,l=()=>t!==Ie,s=document.getElementById("res-desktop"),i=s.querySelectorAll("tr.data-row");if(!i.length)return;const a=((e?.salarie?.prenom||"")+(e?.salarie?.nom||"")).toLowerCase().includes("ë"),n=a?2:4,c=a?1:2,f=["#ff00ff","#00ffff","#ff0066","#66ff00","#ff6600","#0066ff","#ff00cc","#00ff99","#ffff00","#ff3399"],p=()=>a?f[Math.floor(Math.random()*f.length)]:"#ffe066",x=s.querySelector("tr.tbl-total");let b=null,d=null,v="",y="";if(x){const S=x.querySelectorAll("td");b=S[1],d=S[3],b&&(v=b.textContent,b.textContent=""),d&&(y=d.textContent,d.textContent="")}const $=[];for(const S of i){const E=S.querySelector("td:first-child > span:last-child");if(E){const w=E.textContent;E.textContent="",$.push({target:E,text:w,ms:n})}S.querySelectorAll("td:not(:first-child)").forEach(w=>{const M=[...w.childNodes].find(I=>I.nodeType===3&&I.textContent.trim());if(M){const I=M.textContent;M.textContent="",$.push({target:M,text:I,ms:c})}}),$.push({pause:a?4:8})}for(const S of $){if(l())return;if(S.pause){await B(S.pause);continue}for(const E of S.text){if(l())return;S.target.textContent+=E,await B(S.ms)}}if(!l()){await B(80);for(const S of i){if(l())return;const E=S.querySelectorAll("td")[3];if(E?.classList.contains("c-sal")){const w=p();E.style.background=w,E.style.color="#000",await B(a?65:110),E.style.background="",E.style.color="",await B(a?10:20)}}if(b){a&&(b.style.fontWeight="bold",b.style.fontSize="1.05em");for(const S of v){if(l())return;b.textContent+=S,await B(c)}a&&Ce(b)}if(!l()){await B(80);for(const S of i){if(l())return;const E=S.querySelectorAll("td")[5];if(E?.classList.contains("c-pat")){const w=p();E.style.background=w,E.style.color="#000",await B(a?65:110),E.style.background="",E.style.color="",await B(a?10:20)}}if(d){a&&(d.style.fontWeight="bold",d.style.fontSize="1.05em");for(const S of y){if(l())return;d.textContent+=S,await B(c)}a&&Ce(d)}}}}let Se=!1;function at(){if(Se)return;Se=!0;const e=document.createElement("style");e.textContent="@keyframes flameRise{0%{transform:translateY(0) scale(1);opacity:1}100%{transform:translateY(-42px) scale(0);opacity:0}}",document.head.appendChild(e)}function Ce(e){at();const t=e.getBoundingClientRect(),l=document.createElement("div");l.style.cssText=`position:fixed;pointer-events:none;z-index:9999;overflow:visible;left:${t.left}px;top:${t.top}px;width:${t.width}px;height:${t.height}px`,document.body.appendChild(l);const s=["#ff8800","#ffdd00","#ff5500","#ffaa00","#ff3300","#ffcc00","#ff6600"],i=setInterval(()=>{const o=document.createElement("div"),a=Math.floor(Math.random()*3)+1,n=(.5+Math.random()*1.2).toFixed(2);o.style.cssText=`position:absolute;pointer-events:none;left:${(Math.random()*t.width).toFixed(1)}px;bottom:0;width:${a}px;height:${a}px;background:${s[Math.floor(Math.random()*s.length)]};animation:flameRise ${n}s ease-out forwards`,l.appendChild(o),setTimeout(()=>o.remove(),parseFloat(n)*1e3+50)},35);setTimeout(()=>{clearInterval(i),setTimeout(()=>l.remove(),1400)},3e3)}function u(e){return String(e).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#039;")}window.setView=function(e){["mobile","desktop","annuel","forge","apropos","contact","gaabrielle","hercule","quizz"].forEach(t=>document.body.classList.toggle("is-"+t,e===t)),document.getElementById("btn-desk").classList.toggle("active",e==="desktop"),document.getElementById("btn-mob").classList.toggle("active",e==="mobile"),document.getElementById("btn-ann").classList.toggle("active",e==="annuel"),de&&(e==="desktop"||e==="mobile")&&ke(de),e==="forge"&&pt(),e==="quizz"&&Lt()};let W="EUR";function r(e){const t=parseFloat(e),l=W==="CHF"?" CHF":" €";return t.toLocaleString("fr-FR",{minimumFractionDigits:2,maximumFractionDigits:2})+l}function it(e,t=!1){const l=parseFloat(e),s=W==="CHF"?" CHF":" €",i=l.toLocaleString("fr-FR",{minimumFractionDigits:2,maximumFractionDigits:2})+s;return t&&l>0?"+"+i:i}function ae(e){return(parseFloat(e)*100).toFixed(2)+" %"}function Pe(){const e=document.body.classList.contains("is-mobile")?"m-date":"d-date",t=e==="m-date"?"d-date":"m-date";return document.getElementById(e)?.value||document.getElementById(t)?.value||Fe}function be(e){if(!e)return"—";const[t,l,s]=e.split("-");return`${s}/${l}/${t}`}const nt=[{min:0,max:1620,taux:0},{min:1620,max:1683,taux:.005},{min:1683,max:1791,taux:.013},{min:1791,max:1911,taux:.021},{min:1911,max:2042,taux:.029},{min:2042,max:2151,taux:.035},{min:2151,max:2294,taux:.041},{min:2294,max:2714,taux:.053},{min:2714,max:3107,taux:.075},{min:3107,max:3539,taux:.099},{min:3539,max:3983,taux:.119},{min:3983,max:4648,taux:.138},{min:4648,max:5574,taux:.158},{min:5574,max:6974,taux:.179},{min:6974,max:8711,taux:.2},{min:8711,max:12091,taux:.24},{min:12091,max:16376,taux:.28},{min:16376,max:25706,taux:.33},{min:25706,max:55062,taux:.38},{min:55062,max:1/0,taux:.43}];function ge(e){const t=parseFloat(e);if(isNaN(t)||t<=0)return{total:0,taux_effectif:0,details:[]};let l=0;const s=[];for(const i of nt){if(t<=i.min)break;const a=+((i.max===1/0?t:Math.min(t,i.max))-i.min).toFixed(2),n=a*i.taux;if(s.push({min:i.min,max:i.max===1/0?null:i.max,taux:i.taux,base:a,montant:+n.toFixed(2)}),l+=n,i.max===1/0||t<=i.max)break}return{total:+l.toFixed(2),taux_effectif:t>0?l/t:0,details:s}}const $e={"Sécurité Sociale":"cat-ss","CSG/CRDS":"cat-csg","Retraite complémentaire":"cat-ret",Prévoyance:"cat-prev",Chômage:"cat-cho",Allègement:"cat-alleg","1er pilier":"cat-ss","Assurance chômage":"cat-cho","Assurance accidents":"cat-acc","Prévoyance maladie":"cat-prev","Prévoyance (LPP)":"cat-ret","Assurance pension":"cat-ret","Assurance maladie":"cat-ss","Assurance dépendance":"cat-prev","Mutualité des employeurs":"cat-ss","Previdenza sociale":"cat-ss",Disoccupazione:"cat-cho","Assicurazione infortuni":"cat-acc","Fine rapporto":"cat-prev",Allegement:"cat-alleg","Bonus IRPEF":"cat-alleg",Imposta:"cat-csg","Imposta regionale":"cat-csg","Retraite fédérale":"cat-ret","Retraite Québec":"cat-ret","Chômage fédéral":"cat-cho","Parentalité Québec":"cat-ss","Santé Québec":"cat-ss","Impôt fédéral":"cat-csg","Impôt provincial":"cat-csg",Autres:"cat-prev"},z={},we={SS_VIEILLESSE_PLAF:"min(Salaire brut, Plafond Mensuel Sécurité Sociale — PMSS)",CHOMAGE:"min(Salaire brut, 4 × PMSS)",AGS:"min(Salaire brut, 4 × PMSS)",CSG_DEDUCTIBLE:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels (CSS art. L136-2)",CSG_NON_DEDUCTIBLE:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels",CRDS:"Salaire brut × 98,25 %  — abattement forfaitaire frais professionnels",AGIRC_ARRCO_T1:"min(Salaire brut, PMSS)  — Tranche 1 (entre 0 et 1 PMSS)",AGIRC_ARRCO_CEG_T1:"min(Salaire brut, PMSS)  — Tranche 1",PREVOYANCE_CADRE_MIN:"min(Salaire brut, PMSS)  — Tranche A",AGIRC_ARRCO_T2:"Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2",AGIRC_ARRCO_CEG_T2:"Fraction du salaire entre 1 PMSS et 8 PMSS  — Tranche 2"};function V(e,t=!0){const l=["f","(","x",")"].map((s,i)=>`<span style="animation-delay:${i*45}ms">${s}</span>`).join("");return t?`<span class="formula-star" data-fmkey="${e}" onclick="event.stopPropagation();showFormula('${e}')">${l}</span>`:`<span class="formula-star" aria-hidden="true">${l}</span>`}window.togglePasDetail=function(e){const t=document.getElementById(e);if(!t)return;const l=t.style.display!=="none";t.style.display=l?"none":"block";const s=document.getElementById(e+"-arrow");s&&(s.textContent=l?"▶":"▼")};function Y(e,t){if(e.code==="REDUCTION_FILLON")return`<pre class="fm-fillon">${u(e.explication)}</pre>`;const l=t==="sal",s=l?e.taux_sal:e.taux_pat,i=parseFloat(e.base),o=l?parseFloat(e.montant_sal):Math.abs(parseFloat(e.montant_pat)),a=l?"Taux salarial":"Taux patronal",n=l?"Montant salarial":t==="alleg"?"Montant allègement":"Montant patronal",c=l?"c-sal":t==="alleg"?"c-alleg":"c-pat",f=we[e.code]?`<div class="fm-base-note">Assiette  =  ${u(we[e.code])}</div>`:"";return`
    <div class="fm-generic">Montant  =  Assiette  ×  ${a}</div>
    ${f}
    <table class="fm-calc">
      <tr>
        <td>Assiette</td>
        <td class="fm-op">=</td>
        <td class="fm-val c-base">${r(i)}</td>
      </tr>
      <tr>
        <td>${a}</td>
        <td class="fm-op">×</td>
        <td class="fm-val c-taux">${ae(s)}</td>
      </tr>
      <tr class="fm-result fm-sep">
        <td>${n}</td>
        <td class="fm-op">=</td>
        <td class="fm-val ${c}">${r(o)}</td>
      </tr>
    </table>`}function Re(e){const t=ge(e);return`
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
      <tbody>${t.details.map(s=>{const i=s.min.toLocaleString("fr-FR")+" €",o=s.max===null?"∞":s.max.toLocaleString("fr-FR")+" €",a=s.taux===0;return`
      <tr class="${a?"pas-zero":""}">
        <td>${i} → ${o}</td>
        <td class="r">${r(s.base)}</td>
        <td class="r ${a?"c-dim":""}">${(s.taux*100).toFixed(1).replace(".",",")} %</td>
        <td class="r ${a?"c-dim":"c-purple"}">${a?"—":r(s.montant)}</td>
      </tr>`}).join("")}</tbody>
      <tfoot>
        <tr>
          <td>Net imposable</td>
          <td class="r c-gray">${r(e)}</td>
          <td class="r c-taux">${(t.taux_effectif*100).toFixed(2)} %&nbsp;<span style="color:var(--dim);font-size:0.7em">(taux effectif)</span></td>
          <td class="r c-purple" style="font-weight:bold">${r(t.total)}</td>
        </tr>
      </tfoot>
    </table>`}window.showFormula=function(e){const t=z[e];if(!t)return;const l=document.getElementById("fm-body");if(t.type==="pas"){document.getElementById("fm-title").textContent="Prélèvement à la Source (PAS)",document.getElementById("fm-badge").textContent="── Détail par tranche — barème neutre mensuel DGFIP ─────────",l.className="fm-type-pas",l.innerHTML=Re(t.netImposable),document.getElementById("fm-modal").classList.add("open"),document.querySelectorAll(`[data-fmkey="${e}"]`).forEach(n=>n.classList.add("visited"));return}const{c:s,type:i}=t,o=i==="sal",a=s.code==="REDUCTION_FILLON"?"── Allègement patronal ──────────────────────":o?"── Part salariale ───────────────────────────":"── Part patronale ───────────────────────────";document.getElementById("fm-title").textContent=s.libelle,document.getElementById("fm-badge").textContent=a,l.className=`fm-type-${i}`,l.innerHTML=Y(s,i),document.getElementById("fm-modal").classList.add("open"),document.querySelectorAll(`[data-fmkey="${e}"]`).forEach(n=>n.classList.add("visited"))};window.closeFmModal=function(){document.getElementById("fm-modal").classList.remove("open")};window.toggleExpl=function(e){const t=document.getElementById(`row-${e}`),l=document.getElementById(`expl-${e}`);if(!t||!l)return;const s=l.style.display!=="none";l.style.display=s?"none":"table-row",t.classList.toggle("open",!s)};function ot(e){const t=document.getElementById("res-desktop"),l=e.cotisations,s=["suisse","luxembourg","italia","canada","quebec"].includes(e.salarie?.pays),i=e.salarie?.pays==="italia",o=l.reduce((g,_)=>g+parseFloat(_.montant_sal),0),a=l.reduce((g,_)=>g+parseFloat(_.montant_pat),0),n=s?{total:0,taux_effectif:0}:ge(e.net_imposable),c=parseFloat(e.net_a_payer)-n.total;s||(z.PAS={type:"pas",netImposable:parseFloat(e.net_imposable)});const f=e.salarie?.pays==="suisse"?l.find(g=>g.code==="CH_IS"):null,p=f?parseFloat(f.montant_sal):0,x=f?parseFloat(f.taux_sal):0;f&&(z.CH_IS={c:f,type:"sal"});const b=i?l.find(g=>g.code==="IT_IRPEF"):null,d=b?parseFloat(b.montant_sal):0,v=b?parseFloat(b.taux_sal):0,y=i?l.find(g=>g.code==="IT_BONUS_CUNEO"):null,$=y?parseFloat(y.montant_sal):0,S=o-p,E=o-p-d-$,w=`
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
            <span style="color:var(--red)">− ${r(i?E:S)}</span>
          </div>
          ${f?`<div class="sb-ded-row">
            <span>Impôt à la source (${(x*100).toFixed(1)} %)</span>
            <span class="fm-val" style="color:var(--purple);cursor:pointer" onclick="showFormula('CH_IS')">− ${r(p)}${V("CH_IS")}</span>
          </div>`:""}
          ${b?`<div class="sb-ded-row">
            <span>IRPEF (${(v*100).toFixed(1)} % eff.)</span>
            <span style="color:var(--purple)">− ${r(d)}</span>
          </div>`:""}
          ${y?`<div class="sb-ded-row">
            <span>Bonus cuneo fiscale</span>
            <span style="color:var(--green)">+ ${r(Math.abs($))}</span>
          </div>`:""}
          ${s?"":`<div class="sb-ded-row">
            <span>PAS (${(n.taux_effectif*100).toFixed(1)} %)</span>
            <span class="fm-val" style="color:var(--purple);cursor:pointer" onclick="showFormula('PAS')">− ${r(n.total)}${V("PAS")}</span>
          </div>`}
          <div class="sb-ded-total">
            <span>Total retenues</span>
            <span style="color:var(--red)">− ${r(o+n.total)}</span>
          </div>
        </div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ NET À PAYER</div>
        <div class="sb-val c-green">${r(c)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ CHARGES PAT.</div>
        <div class="sb-val c-orange">${r(a)}</div>
      </div>
      <div class="sb-cell">
        <div class="sb-lbl">▸ SUPER BRUT</div>
        <div class="sb-val c-yellow">${r(parseFloat(e.brut)+a)}</div>
      </div>
    </div>`,M=l.filter(g=>g.categorie!=="Allègement"&&(parseFloat(g.montant_sal)>0||g.taux_sal!=="0"||parseFloat(g.montant_pat)>0)),I=l.filter(g=>g.categorie==="Allègement"),C=M.reduce((g,_)=>g+parseFloat(_.montant_pat),0);function P(g,_){return g.map((h,R)=>{const A=_+R,T=$e[h.categorie]||"cat-ss",oe=parseFloat(h.montant_sal)>0?"c-sal":"c-dim",ce=parseFloat(h.montant_pat)>0?"c-pat":"c-dim",j=`${h.code}_sal`,G=`${h.code}_pat`,le=parseFloat(h.montant_sal)>0,se=parseFloat(h.montant_pat)>0;le&&(z[j]={c:h,type:"sal"}),se&&(z[G]={c:h,type:"pat"});const Xe=V(j,le),We=V(G,se);return`
        <tr class="data-row" id="row-${A}" onclick="toggleExpl(${A})">
          <td>
            <span class="expand-icon">▶</span>
            <span class="cat ${T}">[${h.categorie}]</span>
            <span>${h.libelle}</span>
          </td>
          <td class="r">${r(h.base)}</td>
          <td class="r">${parseFloat(h.taux_sal)>0?"− ":""}${ae(h.taux_sal)}</td>
          <td class="r ${oe}"${le?` onclick="event.stopPropagation();showFormula('${j}')" style="cursor:pointer"`:""}>${le?"− ":""}${r(h.montant_sal)}${Xe}</td>
          <td class="r">${parseFloat(h.taux_pat)>0?"− ":""}${ae(h.taux_pat)}</td>
          <td class="r ${ce}"${se?` onclick="event.stopPropagation();showFormula('${G}')" style="cursor:pointer"`:""}>${se?"− ":""}${r(h.montant_pat)}${We}</td>
        </tr>
        <tr class="expl-row" id="expl-${A}" style="display:none">
          <td colspan="6">
            <div class="expl-box">
              <div class="expl-txt">▸ ${u(h.explication)}</div>
              ${h.loi_ref?`<div class="expl-ref">§ ${u(h.loi_ref)}</div>`:""}
            </div>
          </td>
        </tr>`}).join("")}const ee=`
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
    </thead>`,ne=`
    <div class="tbl-section-head">── COTISATIONS ────────────────────────────────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${ee}
      <tbody>
        ${P(M,0)}
        <tr class="tbl-total">
          <td colspan="3">TOTAUX</td>
          <td class="r c-sal">= − ${r(o)}</td>
          <td></td>
          <td class="r c-pat">= − ${r(C)}</td>
        </tr>
      </tbody>
    </table>`,te=`<div class="sim-period">
    SIMULATION AU <span class="sp-accent">${be(Pe())}</span>
    &nbsp;·&nbsp; PMSS en vigueur calculé depuis la base de données sans le moindre état d'âme
  </div>`,m=I.reduce((g,_)=>g+parseFloat(_.montant_pat),0),L=I.length===0?"":`
    <div class="tbl-section-head">── ALLÈGEMENTS PATRONAUX ───────────────────────────────────────────</div>
    <table class="ascii-tbl">
      ${ee}
      <tbody>
        ${I.map((g,_)=>{const h=M.length+_,R=$e[g.categorie]||"cat-alleg",A=Math.abs(parseFloat(g.montant_pat)),T=`${g.code}_alleg`;return z[T]={c:g,type:"alleg"},`
            <tr class="data-row" id="row-${h}" onclick="toggleExpl(${h})">
              <td>
                <span class="expand-icon">▶</span>
                <span class="cat ${R}">[${g.categorie}]</span>
                <span>${g.libelle}</span>
              </td>
              <td class="r">${r(g.base)}</td>
              <td class="r"></td>
              <td class="r"></td>
              <td class="r c-alleg">${ae(Math.abs(parseFloat(g.taux_pat)))}</td>
              <td class="r c-alleg" onclick="event.stopPropagation();showFormula('${T}')" style="cursor:pointer">− ${r(A)}${V(T)}</td>
            </tr>
            <tr class="expl-row" id="expl-${h}" style="display:none">
              <td colspan="6">
                <div class="expl-box">
                  <div class="expl-txt">▸ ${u(g.explication)}</div>
                  ${g.loi_ref?`<div class="expl-ref">§ ${u(g.loi_ref)}</div>`:""}
                </div>
              </td>
            </tr>`}).join("")}
        <tr class="tbl-total">
          <td colspan="5">TOTAL ALLÈGEMENTS PATRONAUX</td>
          <td class="r c-alleg">− ${r(Math.abs(m))}</td>
        </tr>
      </tbody>
    </table>`;t.innerHTML=te+w+`<div class="tbl-wrap">${ne}${L}</div>`}window.mobToggle=function(e,t){const l=document.getElementById("mob-expand-"+e);if(!l)return;const s=["why","how","sal","pat"],i=l.style.display!=="none",o=l.dataset.panel,a=n=>{const c=document.getElementById(`mob-expand-${e}-${n}`);c&&(c.style.display=n===t?"block":"none")};i?o===t?l.style.display="none":(l.dataset.panel=t,s.forEach(a)):(l.style.display="block",l.dataset.panel=t,s.forEach(a))};function ct(e,t,l,s,i,o=0){const a=e.code==="REDUCTION_FILLON"?`<pre class="fm-fillon">${u(e.explication)}</pre>`:`<div class="fm-type-${i}">${Y(e,i)}</div>`,n=`
    <div class="mob-exp-txt">${u(e.explication)}</div>
    ${e.loi_ref?`<div class="mob-exp-loi">§ ${u(e.loi_ref)}</div>`:""}`;return`
    <div class="${`mob-stripe-${i}-${o%2===0?"a":"b"}`}">
      <div class="mob-row">
        <span class="mob-lbl mob-cot-lbl"
              title="Explication et référence légale"
              onclick="mobToggle('${t}','why')">${u(e.libelle)}</span>
        <span class="mob-val ${s} mob-cot-amt"
              title="Formule de calcul"
              onclick="mobToggle('${t}','how')">${l}</span>
      </div>
      <div class="mob-expand" id="mob-expand-${t}" style="display:none">
        <div id="mob-expand-${t}-why">${n}</div>
        <div id="mob-expand-${t}-how" style="display:none">${a}</div>
      </div>
    </div>`}function rt(e){const t=document.getElementById("res-mobile"),l=document.getElementById("m-nom")?.value||document.getElementById("d-nom")?.value||"",s=document.getElementById("m-prenom")?.value||document.getElementById("d-prenom")?.value||"",i=e.cotisations,o=["suisse","luxembourg","italia","canada","quebec"].includes(e.salarie?.pays),a=e.salarie?.pays==="italia",n=i.reduce((m,L)=>m+parseFloat(L.montant_sal),0),c=i.reduce((m,L)=>m+parseFloat(L.montant_pat),0),f=o?{total:0,taux_effectif:0}:ge(e.net_imposable),p=parseFloat(e.net_a_payer)-f.total,x=parseFloat(e.brut)+c,b=e.salarie?.pays==="suisse"?i.find(m=>m.code==="CH_IS"):null,d=b?parseFloat(b.montant_sal):0,v=b?parseFloat(b.taux_sal):0,y=a?i.find(m=>m.code==="IT_IRPEF"):null,$=y?parseFloat(y.montant_sal):0,S=y?parseFloat(y.taux_sal):0,E=a?i.find(m=>m.code==="IT_BONUS_CUNEO"):null,w=E?parseFloat(E.montant_sal):0,M=n-d-$-w,I=i.filter(m=>m.categorie!=="Allègement"&&m.code!=="CH_IS"&&m.code!=="IT_IRPEF"&&m.code!=="IT_BONUS_CUNEO"&&(parseFloat(m.montant_sal)>0||m.taux_sal!=="0"||parseFloat(m.montant_pat)>0)),C=i.filter(m=>m.categorie==="Allègement"),P=I.reduce((m,L)=>m+parseFloat(L.montant_pat),0),ee=C.reduce((m,L)=>m+parseFloat(L.montant_pat),0),ne=I.map((m,L)=>{const g=parseFloat(m.montant_sal)>0,_=parseFloat(m.montant_pat)>0,h=`${m.code}_u`,R=m.code==="REDUCTION_FILLON",A=g?R?`<pre class="fm-fillon">${u(m.explication)}</pre>`:`<div class="fm-type-sal">${Y(m,"sal")}</div>`:"",T=_?R?`<pre class="fm-fillon">${u(m.explication)}</pre>`:`<div class="fm-type-pat">${Y(m,"pat")}</div>`:"",oe=`
      <div class="mob-exp-txt">${u(m.explication)}</div>
      ${m.loi_ref?`<div class="mob-exp-loi">§ ${u(m.loi_ref)}</div>`:""}`,ce=`mob-stripe-sal-${L%2===0?"a":"b"}`,j=g?`<span class="mob-val mob-cot-amt" style="color:#ffe033" onclick="mobToggle('${h}','sal')">− ${r(m.montant_sal)}</span>`:`<span class="mob-val c-dim">0 ${W==="CHF"?"CHF":"€"}</span>`,G=_?`<span class="mob-val c-orange mob-cot-amt" onclick="mobToggle('${h}','pat')">− ${r(m.montant_pat)}</span>`:`<span class="mob-val c-dim">0 ${W==="CHF"?"CHF":"€"}</span>`;return`
      <div class="${ce}">
        <div class="mob-row">
          <span class="mob-lbl mob-cot-lbl" onclick="mobToggle('${h}','why')">${u(m.libelle)}</span>
          <span style="display:flex;flex-direction:column;align-items:flex-end;gap:0.1rem">${j}${G}</span>
        </div>
        <div class="mob-expand" id="mob-expand-${h}" style="display:none">
          <div id="mob-expand-${h}-why">${oe}</div>
          ${A?`<div id="mob-expand-${h}-sal" style="display:none">${A}</div>`:""}
          ${T?`<div id="mob-expand-${h}-pat" style="display:none">${T}</div>`:""}
        </div>
      </div>`}).join(""),te=C.map((m,L)=>ct(m,`${m.code}_alleg`,`− ${r(Math.abs(parseFloat(m.montant_pat)))}`,"c-alleg","alleg",L)).join("");t.innerHTML=`
    <div class="mob-bulletin">

      <!-- En-tête bulletin -->
      <div class="mob-head">
        <span class="mob-head-title">BULLETIN DE PAYE</span>
        <div style="text-align:right">
          <div class="mob-head-name">${u(s)} ${u(l).toUpperCase()}</div>
          <div class="mob-head-date">simulation au ${be(Pe())}</div>
        </div>
      </div>

      <!-- Brut -->
      <div class="mob-row" style="margin-top:0.15rem">
        <span class="mob-lbl">Salaire de base brut</span>
        <span class="mob-val c-gray">${r(e.brut)}</span>
      </div>

      <!-- Cotisations unifiées (salariales + patronales sur une ligne) -->
      <div class="mob-row section"><span class="mob-lbl">── COTISATIONS ──</span><span style="display:flex;gap:1.5rem;font-size:0.62rem;color:var(--muted)"><span>SAL.</span><span>PAT.</span></span></div>
      ${ne}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL cotisations sociales</span>
        <span class="mob-val c-red">− ${r(M)}</span>
      </div>
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL charges patronales</span>
        <span class="mob-val c-orange">− ${r(P)}</span>
      </div>

      <!-- Impôt à la source suisse — accordéon dédié -->
      ${b?`<div class="mob-row pas-row" style="cursor:pointer" onclick="togglePasDetail('is-detail-mob')">
        <span class="mob-lbl">Impôt à la source (${(v*100).toFixed(1)} %) <span id="is-detail-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-purple">− ${r(d)}</span>
      </div>
      <div id="is-detail-mob" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        <div class="fm-type-sal">${Y(b,"sal")}</div>
        <div class="mob-exp-txt" style="margin-top:0.5rem">${u(b.explication)}</div>
        ${b.loi_ref?`<div class="mob-exp-loi">§ ${u(b.loi_ref)}</div>`:""}
      </div>`:""}

      <!-- Net imposable (France / FPT) -->
      ${o?"":`<div class="mob-row net-row">
        <span class="mob-lbl">NET IMPOSABLE</span>
        <span class="mob-val c-green">${r(e.net_imposable)}</span>
      </div>`}

      <!-- PAS (France / FPT) -->
      ${o?"":`<div class="mob-row pas-row" style="cursor:pointer" onclick="togglePasDetail('pas-detail-mob')">
        <span class="mob-lbl">Prélèvement à la source (${(f.taux_effectif*100).toFixed(1)} %) <span id="pas-detail-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-purple">− ${r(f.total)}</span>
      </div>
      <div id="pas-detail-mob" class="fm-type-pas" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        ${Re(parseFloat(e.net_imposable))}
      </div>`}

      <!-- IRPEF italienne -->
      ${y?`<div class="mob-row pas-row" style="cursor:pointer" onclick="togglePasDetail('irpef-detail-mob')">
        <span class="mob-lbl">IRPEF (${(S*100).toFixed(1)} % eff.) <span id="irpef-detail-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-purple">− ${r($)}</span>
      </div>
      <div id="irpef-detail-mob" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        <div class="mob-exp-txt">${u(y.explication)}</div>
        ${y.loi_ref?`<div class="mob-exp-loi">§ ${u(y.loi_ref)}</div>`:""}
      </div>`:""}

      <!-- Bonus cuneo fiscale -->
      ${E?`<div class="mob-row" style="cursor:pointer" onclick="togglePasDetail('bonus-cuneo-mob')">
        <span class="mob-lbl">Bonus cuneo fiscale <span id="bonus-cuneo-mob-arrow" style="font-size:0.65em">▶</span></span>
        <span class="mob-val c-green">+ ${r(Math.abs(w))}</span>
      </div>
      <div id="bonus-cuneo-mob" style="display:none;padding:0.4rem 0.6rem 0.2rem">
        <div class="mob-exp-txt">${u(E.explication)}</div>
        ${E.loi_ref?`<div class="mob-exp-loi">§ ${u(E.loi_ref)}</div>`:""}
      </div>`:""}

      <!-- Net à payer -->
      <div class="mob-row final-row">
        <span class="mob-lbl">NET À PAYER</span>
        <span class="mob-val c-green">${r(p)}</span>
      </div>

      <!-- Allègements -->
      ${te.length?`
      <div class="mob-row section"><span class="mob-lbl">── ALLÈGEMENTS PATRONAUX ──</span><span></span></div>
      ${te}
      <div class="mob-row subtot">
        <span class="mob-lbl">TOTAL allègements</span>
        <span class="mob-val c-alleg">− ${r(Math.abs(ee))}</span>
      </div>`:""}

      <!-- Super brut -->
      <div class="mob-row superbrut">
        <span class="mob-lbl">SUPER BRUT (coût employeur)</span>
        <span class="mob-val c-blue">${r(x)}</span>
      </div>

    </div>`}function ke(e){W=e.devise||"EUR",ot(e),rt(e),N&&st(e)}function _e(e){const t=`<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">⚠ ${u(e)}</div>`;document.getElementById("res-desktop").innerHTML=t,document.getElementById("res-mobile").innerHTML=t}async function ze(e){const t=e==="mobile",l=document.getElementById(t?"m-brut":"d-brut").value,s=document.getElementById(t?"m-statut":"d-statut").value,i=document.getElementById(t?"m-nom":"d-nom").value||"Dupont",o=document.getElementById(t?"m-prenom":"d-prenom").value||"Marie",a=document.getElementById(t?"m-date":"d-date").value||Fe,n=document.getElementById(t?"m-alsace-moselle":"d-alsace-moselle")?.checked??!1,c=document.getElementById(t?"m-suisse":"d-suisse")?.checked??!1,f=document.getElementById(t?"m-luxembourg":"d-luxembourg")?.checked??!1,p=document.getElementById(t?"m-fpt":"d-fpt")?.checked??!1,x=document.getElementById(t?"m-italie":"d-italie")?.checked??!1,b=document.getElementById(t?"m-canada":"d-canada")?.checked??!1,d=document.getElementById(t?"m-quebec":"d-quebec")?.checked??!1,v=document.getElementById(t?"m-ca-province":"d-ca-province")?.value||"ON",y=document.getElementById(t?"m-assujetti-is":"d-assujetti-is")?.checked??!1,$=document.getElementById(t?"m-canton":"d-canton")?.value||null,S=document.getElementById(t?"m-tarif-is":"d-tarif-is")?.value||null,E=parseFloat(l);if(!l||isNaN(E)||E<=0){_e("Salaire brut invalide — saisir un montant positif.");return}if(!/^\d{4}-\d{2}-\d{2}$/.test(a)){_e(`Date invalide : '${a}' (format attendu : YYYY-MM-DD).`);return}["d-brut","m-brut"].forEach(I=>{const C=document.getElementById(I);C&&(C.value=l)}),["d-statut","m-statut"].forEach(I=>{const C=document.getElementById(I);C&&(C.value=s)}),["d-nom","m-nom"].forEach(I=>{const C=document.getElementById(I);C&&(C.value=i)}),["d-prenom","m-prenom"].forEach(I=>{const C=document.getElementById(I);C&&(C.value=o)}),["d-date","m-date"].forEach(I=>{const C=document.getElementById(I);C&&(C.value=a)});const w=c?"suisse":f?"luxembourg":x?"italia":b?"canada":d?"quebec":null,M=w?"2026-01-01":a;try{const I=await Te("calculer_bulletin",{salarie:{nom:i,prenom:o,salaire_brut:l.toString(),statut:s,alsace_moselle:n,pays:w??(p?"fonction_publique":"france"),assujetti_is:y,canton:c&&y&&$?$:null,tarif_is:c&&y&&S?S:null,regione:null,contratto_termine:!1,province:b?v:null},datePaie:M});de=I,ke(I)}catch(I){console.error("[calculer_bulletin] erreur brute :",I);const C=Z(I),P=`<div style="padding:1.5rem;color:#f87171;font-size:0.8rem">ERREUR : ${u(C)}</div>`;document.getElementById("res-desktop").innerHTML=P,document.getElementById("res-mobile").innerHTML=P}}function dt(e){const t=document.getElementById("res-annuel"),l=e.lignes,s=l.map(p=>p.smic),i=`
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
    </tr></thead>`,o=l.map((p,x)=>{const b=x>0&&p.smic!==s[x-1],d=p.mois_libelle.includes("13e"),v=parseFloat(p.fillon_regularise)-parseFloat(p.fillon_simple),y=Math.abs(v)<.005?'<span style="color:var(--dim)">—</span>':`<span class="delta-nonzero">${v>0?"+":""}${it(v.toFixed(2))}</span>`;return`<tr class="${[b?"smic-change":"",d?"treizieme-mois":""].filter(Boolean).join(" ")}">
      <td>${p.mois_libelle}</td>
      <td>${r(p.smic)}</td>
      <td>${r(p.brut)}</td>
      <td class="c-sal">− ${r(p.total_sal)}</td>
      <td class="c-pat">+ ${r(p.total_pat_brut)}</td>
      <td class="c-alleg">− ${r(p.fillon_regularise)}</td>
      <td>${y}</td>
      <td class="c-green">${r(p.net_a_payer)}</td>
      <td class="c-yellow">${r(p.cout_employeur)}</td>
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
    </tr>`,n=parseFloat(e.total_pat_brut),c=parseFloat(e.total_fillon),f=`
    <div style="display:flex;gap:1rem;flex-wrap:wrap;margin-top:0.75rem;font-size:0.72rem">
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">ÉCONOMIE FILLON (annuelle)</div>
        <div style="color:var(--green);font-size:1.1rem;font-weight:bold">− ${r(e.total_fillon)}</div>
      </div>
      <div style="border:1px solid var(--border);padding:0.5rem 0.9rem;background:var(--bg3)">
        <div style="color:var(--muted)">TAUX FILLON MOYEN</div>
        <div style="color:var(--blue);font-size:1.1rem;font-weight:bold">
          ${n>0?(c/parseFloat(e.total_brut)*100).toFixed(2)+" %":"—"}
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
      ${i}
      <tbody>${o}</tbody>
      ${a}
    </table>
    ${f}`}async function ut(){const e=parseInt(document.getElementById("a-annee").value),t=document.getElementById("a-brut").value,l=document.getElementById("a-statut").value,s=document.getElementById("res-annuel");if(isNaN(e)||e<1900||e>2100){s.innerHTML='<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Année invalide.</div>';return}const i=parseFloat(t);if(!t||isNaN(i)||i<=0){s.innerHTML='<div style="padding:1rem;color:var(--red);font-size:0.8rem">⚠ Salaire brut invalide — saisir un montant positif.</div>';return}s.innerHTML='<div style="color:var(--muted);padding:1rem;font-size:0.78rem">Calcul en cours…</div>';try{const o=await Te("simuler_annee",{annee:e,salaireBrut:t.toString(),statut:l});dt(o)}catch(o){console.error("[simuler_annee] erreur brute :",o),s.innerHTML=`<div style="padding:1rem;color:var(--red);font-size:0.8rem">ERREUR : ${u(Z(o))}</div>`}}window.onTogglePays=function(e,t){const l=["suisse","luxembourg","fpt","italie","canada","quebec"],s=["suisse","luxembourg","italie","canada","quebec"],i=l.filter(d=>d!==e);t&&i.forEach(d=>{["d","m"].forEach(v=>{const y=document.getElementById(`${v}-${d}`);y&&y.checked&&(y.checked=!1)})});const o=s.some(d=>document.getElementById(`d-${d}`)?.checked);["d","m"].forEach(d=>{const v=document.getElementById(`${d}-alsace-moselle-wrap`);v&&(v.style.display=o?"none":"");const y=document.getElementById(`${d}-alsace-moselle`);y&&o&&(y.checked=!1)}),["d-date","m-date"].forEach(d=>{const v=document.getElementById(d);v&&(v.disabled=o,o&&(v.value="2026-01-01"))});const a=document.getElementById("d-suisse")?.checked,n=document.getElementById("d-canada")?.checked||document.getElementById("d-quebec")?.checked,c=a?"SALAIRE BRUT (CHF)":n?"SALAIRE BRUT (CAD)":"SALAIRE BRUT (€)",f=a?"BRUT (CHF)":n?"BRUT (CAD)":"BRUT (€)",p=document.getElementById("d-brut");if(p){const d=p.closest(".field")?.querySelector("label");d&&(d.textContent=c)}const x=document.getElementById("m-brut");if(x){const d=x.closest(".field")?.querySelector("label");d&&(d.textContent=f)}["d","m"].forEach(d=>{const v=document.getElementById(`${d}-ch-is-wrap`);if(v)if(a)v.style.display="";else{v.style.display="none";const y=document.getElementById(`${d}-assujetti-is`);y&&(y.checked=!1);const $=document.getElementById(`${d}-ch-is-detail`);$&&($.style.display="none")}});const b=document.getElementById("d-canada")?.checked;["d","m"].forEach(d=>{const v=document.getElementById(`${d}-ca-province-wrap`);v&&(v.style.display=b?"":"none")})};window.toggleParams=function(e){const t=document.getElementById(`${e}-params`),l=document.getElementById(`${e}-params-toggle`);if(!t)return;const s=t.style.display!=="none";t.style.display=s?"none":"block",l.classList.toggle("open",!s)};window.syncParam=function(e,t){["d","m"].forEach(l=>{const s=document.getElementById(`${l}-${e}`);s&&(s.type==="checkbox"?s.checked!==t&&(s.checked=t):s.value!==t&&(s.value=t))})};window.onToggleAssujetti=function(e){["d","m"].forEach(t=>{const l=document.getElementById(`${t}-ch-is-detail`);l&&(l.style.display=e?"":"none")})};document.getElementById("d-calc").addEventListener("click",()=>ze("desktop"));document.getElementById("m-calc").addEventListener("click",()=>ze("mobile"));document.getElementById("a-calc").addEventListener("click",ut);const Ne=[{idcc:"1261",libelle:"Acteurs du lien social et familial (ALISFA)"},{idcc:"2941",libelle:"Aide, accompagnement, soins et services à domicile"},{idcc:"1747",libelle:"Activités industrielles de boulangerie et de pâtisserie"},{idcc:"2149",libelle:"Activités du déchet"},{idcc:"2335",libelle:"Agences générales d'assurances"},{idcc:"1686",libelle:"Audiovisuel, électronique et équipement ménager"},{idcc:"2120",libelle:"Banque"},{idcc:"3210",libelle:"Banque Populaire"},{idcc:"0567",libelle:"Bijouterie, joaillerie, orfèvrerie (obsolète)"},{idcc:"0158",libelle:"Bois et scieries"},{idcc:"0992",libelle:"Boucherie"},{idcc:"0843",libelle:"Boulangerie-pâtisserie artisanales"},{idcc:"1606",libelle:"Bricolage"},{idcc:"1486",libelle:"Bureaux d'études techniques et sociétés de conseils (Syntec)"},{idcc:"0787",libelle:"Cabinets d'experts-comptables et de commissaires aux comptes"},{idcc:"2332",libelle:"Cabinets d'architectes"},{idcc:"1619",libelle:"Cabinets dentaires"},{idcc:"2420",libelle:"Cadres du bâtiment"},{idcc:"3212",libelle:"Cadres des travaux publics"},{idcc:"1256",libelle:"Cadres des entreprises de gestion d'équipements thermiques et de climatisation"},{idcc:"0211",libelle:"Cadres des industries de carrières et matériaux (obsolète)"},{idcc:"0045",libelle:"Caoutchouc"},{idcc:"2257",libelle:"Casinos"},{idcc:"0783",libelle:"Centres d'hébergement et de réadaptation sociale"},{idcc:"0953",libelle:"Charcuterie de détail"},{idcc:"1580",libelle:"Chaussure"},{idcc:"2060",libelle:"Chaînes de cafétérias"},{idcc:"1557",libelle:"Commerce des articles de sports et d'équipements de loisirs"},{idcc:"2216",libelle:"Commerce de détail et de gros à prédominance alimentaire"},{idcc:"1505",libelle:"Commerce de détail alimentaire non spécialisé"},{idcc:"2198",libelle:"Commerce à distance et E-commerce"},{idcc:"1483",libelle:"Commerce de détail de l'habillement"},{idcc:"1487",libelle:"Commerce de détail de l'horlogerie-bijouterie"},{idcc:"3237",libelle:"Commerce de détail alimentaire spécialisé"},{idcc:"1225",libelle:"Commerce de la Réunion"},{idcc:"0468",libelle:"Commerce succursaliste de la chaussure"},{idcc:"0573",libelle:"Commerces de gros"},{idcc:"1517",libelle:"Commerces de détail non alimentaires (Codena)"},{idcc:"0500",libelle:"Commerces de gros de l'habillement, mercerie, chaussure et jouet"},{idcc:"3243",libelle:"Commerces de quincaillerie, fournitures industrielles, fers, métaux et équipement de la maison"},{idcc:"2596",libelle:"Coiffure"},{idcc:"1611",libelle:"Communication écrite directe"},{idcc:"1286",libelle:"Confiserie, chocolaterie, biscuiterie"},{idcc:"2583",libelle:"Concessionnaires et exploitants d'autoroutes ou d'ouvrages routiers"},{idcc:"3217",libelle:"Convention collective nationale de la branche ferroviaire"},{idcc:"2272",libelle:"Convention collective nationale de l'assainissement et de la maintenance industrielle"},{idcc:"2002",libelle:"Convention collective interrégionale de la blanchisserie, laverie, location de linge, nettoyage à sec, pressing et teinturerie du 17 novembre 1997"},{idcc:"2247",libelle:"Courtage d'assurances et/ou de réassurances"},{idcc:"0303",libelle:"Couture parisienne et autres métiers de la mode"},{idcc:"0733",libelle:"Détaillants en chaussures"},{idcc:"1605",libelle:"Désinfection, désinsectisation, dératisation"},{idcc:"1536",libelle:"Distributeurs conseils hors domicile"},{idcc:"2372",libelle:"Distribution directe"},{idcc:"1408",libelle:"Distribution, Logistique et Services des Energies de Proximité"},{idcc:"2121",libelle:"Édition"},{idcc:"1518",libelle:"Education, culture, loisirs et animation agissant pour l'utilité sociale et environnementale, au service des territoires (ECLAT)"},{idcc:"2609",libelle:"Employés, techniciens et agents de maîtrise du bâtiment"},{idcc:"2614",libelle:"Employés, techniciens et agents de maîtrise des travaux publics"},{idcc:"0135",libelle:"Employés techniciens et agents de maîtrise des industries de carrières et de matériaux (obsolète)"},{idcc:"3218",libelle:"Enseignement privé non lucratif"},{idcc:"2691",libelle:"Enseignement privé hors contrat"},{idcc:"3043",libelle:"Entreprises de propreté"},{idcc:"3127",libelle:"Entreprises de services à la personne"},{idcc:"1285",libelle:"Entreprises artistiques et culturelles"},{idcc:"1539",libelle:"Entreprises du bureau et du numérique - Commerces et services (Eben)"},{idcc:"1412",libelle:"Entreprises d'installation sans fabrication de matériel aéraulique, thermique, frigorifique"},{idcc:"2717",libelle:"Entreprises techniques au service de la création et de l'évènement"},{idcc:"3032",libelle:"Esthétique"},{idcc:"0029",libelle:"Établissements privés d'hospitalisation, de soins, de cure et de garde à but non lucratif (CCN 51 - FEHAP)"},{idcc:"0413",libelle:"Établissements et services pour personnes inadaptées et handicapées (CCN 66)"},{idcc:"0405",libelle:"Établissements médico-sociaux de l'union intersyndicale des secteurs sanitaires et sociaux (CCN 65)"},{idcc:"0478",libelle:"Établissements financiers"},{idcc:"0915",libelle:"Expertises en matière d'évaluations industrielles et commerciales"},{idcc:"1307",libelle:"Exploitation cinématographique"},{idcc:"1405",libelle:"Expédition et exportation de fruits et légumes"},{idcc:"1411",libelle:"Fabrication de l'ameublement"},{idcc:"0669",libelle:"Fabrication mécanique du verre"},{idcc:"1821",libelle:"Fabrication du verre à la main, semi-automatique et mixte"},{idcc:"1031",libelle:"Fédération nationale des associations familiales rurales"},{idcc:"1978",libelle:"Fleuristes, vente et services des animaux familiers"},{idcc:"0200",libelle:"Froid"},{idcc:"1043",libelle:"Gardiens d'immeubles"},{idcc:"2543",libelle:"Géomètres et experts-fonciers"},{idcc:"2021",libelle:"Golf"},{idcc:"2156",libelle:"Grands magasins"},{idcc:"2336",libelle:"Habitat et du Logement Accompagnés"},{idcc:"1631",libelle:"Hôtellerie de plein air"},{idcc:"1979",libelle:"Hôtels, cafés, restaurants (HCR)"},{idcc:"2264",libelle:"Hospitalisation privée (FHP)"},{idcc:"1921",libelle:"Huissiers de justice"},{idcc:"0044",libelle:"Industries chimiques"},{idcc:"1534",libelle:"Industrie et commerces en gros des viandes"},{idcc:"3233",libelle:"Industrie de la fabrication des ciments"},{idcc:"2089",libelle:"Industrie des panneaux à base de bois"},{idcc:"0176",libelle:"Industrie pharmaceutique"},{idcc:"1388",libelle:"Industrie du pétrole"},{idcc:"0112",libelle:"Industrie laitière"},{idcc:"0018",libelle:"Industrie textile"},{idcc:"3236",libelle:"Industrie et services nautiques"},{idcc:"3109",libelle:"Industries alimentaires diverses"},{idcc:"0247",libelle:"Industries de l'habillement"},{idcc:"2542",libelle:"Industries métallurgiques, mécaniques et connexes de l'Aisne (obsolète)"},{idcc:"3209",libelle:"Industries métallurgiques, mécaniques et connexes du Doubs (obsolète)"},{idcc:"2003",libelle:"Industries métallurgiques, électriques et électroniques des Vosges (obsolète)"},{idcc:"2630",libelle:"Industries métallurgiques des Bouches-du-Rhône et Alpes-de-Haute-Provence (obsolète)"},{idcc:"1396",libelle:"Industries de produits alimentaires élaborés"},{idcc:"0489",libelle:"Industries du cartonnage"},{idcc:"0637",libelle:"Industries et commerce de la récupération"},{idcc:"1938",libelle:"Industries de la transformation des volailles"},{idcc:"1586",libelle:"Industries charcutières"},{idcc:"0184",libelle:"Imprimerie de labeur et industries graphiques"},{idcc:"0043",libelle:"Import-export et commerce international"},{idcc:"1527",libelle:"Immobilier"},{idcc:"0650",libelle:"Ingénieurs et cadres de la métallurgie (obsolète)"},{idcc:"1679",libelle:"Inspection d'assurance"},{idcc:"1794",libelle:"Institutions de retraite complémentaire"},{idcc:"1760",libelle:"Jardineries et graineteries"},{idcc:"1480",libelle:"Journalistes"},{idcc:"0959",libelle:"Laboratoires de biologie médicale extra-hospitaliers"},{idcc:"3013",libelle:"Librairie"},{idcc:"1404",libelle:"Machines et matériels agricoles et de travaux publics (SDLM)"},{idcc:"0675",libelle:"Maisons à succursales de vente au détail d'habillement"},{idcc:"0538",libelle:"Manutention ferroviaire"},{idcc:"2528",libelle:"Maroquinerie"},{idcc:"1589",libelle:"Mareyeurs-expéditeurs"},{idcc:"2931",libelle:"Marchés financiers"},{idcc:"3222",libelle:"Menuiseries charpentes et constructions industrialisées et des portes planes"},{idcc:"0822",libelle:"Mensuels de la métallurgie de la Savoie (obsolète)"},{idcc:"1387",libelle:"Mensuels de la métallurgie des Flandres (obsolète)"},{idcc:"0914",libelle:"Mensuels de la métallurgie de l'Ain (obsolète)"},{idcc:"1930",libelle:"Meunerie"},{idcc:"2190",libelle:"Missions locales et PAIO des maisons de l'emploi et PLIE"},{idcc:"1499",libelle:"Miroiterie, transformation et négoce du verre"},{idcc:"0827",libelle:"Métallurgie des Ardennes (obsolète)"},{idcc:"0863",libelle:"Métallurgie d'Ille-et-Vilaine et du Morbihan (obsolète)"},{idcc:"1867",libelle:"Métallurgie de la Drôme et de l'Ardèche (obsolète)"},{idcc:"0984",libelle:"Métallurgie d'Eure-et-Loir (obsolète)"},{idcc:"2992",libelle:"Métallurgie d'Indre-et-Loire (obsolète)"},{idcc:"0898",libelle:"Métallurgie de l'Allier (obsolète)"},{idcc:"1572",libelle:"Métallurgie de la Charente (obsolète)"},{idcc:"1885",libelle:"Métallurgie de la Côte-d'Or (obsolète)"},{idcc:"1635",libelle:"Métallurgie de la Gironde et des Landes (obsolète)"},{idcc:"1578",libelle:"Métallurgie de la Loire et de l'arrondissement d'Yssingeaux (obsolète)"},{idcc:"0828",libelle:"Métallurgie de la Manche (obsolète)"},{idcc:"0899",libelle:"Métallurgie de la Marne (obsolète)"},{idcc:"1813",libelle:"Métallurgie de la région de Maubeuge (obsolète)"},{idcc:"1525",libelle:"Métallurgie de la région dunkerquoise (obsolète)"},{idcc:"0930",libelle:"Métallurgie de la Sarthe (obsolète)"},{idcc:"0920",libelle:"Métallurgie de la Vienne (obsolète)"},{idcc:"3053",libelle:"Métallurgie de Haute-Saône (obsolète)"},{idcc:"1576",libelle:"Métallurgie du Cher (obsolète)"},{idcc:"0943",libelle:"Métallurgie du Calvados (obsolète)"},{idcc:"0860",libelle:"Métallurgie du Finistère (obsolète)"},{idcc:"2126",libelle:"Métallurgie du Gard et de la Lozère (obsolète)"},{idcc:"1912",libelle:"Métallurgie du Haut-Rhin (obsolète)"},{idcc:"0836",libelle:"Métallurgie de la Haute-Savoie (obsolète)"},{idcc:"0937",libelle:"Métallurgie de la Haute-Vienne et de la Creuse (obsolète)"},{idcc:"1577",libelle:"Métallurgie de l'Hérault, de l'Aude et des Pyrénées-Orientales (obsolète)"},{idcc:"2221",libelle:"Métallurgie de l'Isère et des Hautes-Alpes"},{idcc:"1369",libelle:"Métallurgie de Loire-Atlantique (obsolète)"},{idcc:"2579",libelle:"Métallurgie du Loir-et-Cher (obsolète)"},{idcc:"1966",libelle:"Métallurgie du Loiret (obsolète)"},{idcc:"1902",libelle:"Métallurgie du Maine-et-Loire (obsolète)"},{idcc:"2266",libelle:"Métallurgie de la Mayenne (obsolète)"},{idcc:"1365",libelle:"Métallurgie de Meurthe-et-Moselle (obsolète)"},{idcc:"2755",libelle:"Industries de la métallurgie de Belfort/Montbéliard (obsolète)"},{idcc:"1059",libelle:"Métallurgie des Midi-Pyrénées (obsolète)"},{idcc:"0714",libelle:"Métallurgie de la Moselle (obsolète)"},{idcc:"0948",libelle:"Métallurgie de l'Orne (obsolète)"},{idcc:"2700",libelle:"Métallurgie de l'Oise (obsolète)"},{idcc:"1472",libelle:"Métallurgie du Pas-de-Calais (obsolète)"},{idcc:"2615",libelle:"Métallurgie des Pyrénées-Atlantiques et du Seignanx (obsolète)"},{idcc:"0878",libelle:"Métallurgie du Rhône (obsolète)"},{idcc:"1604",libelle:"Métallurgie de Rouen et de Dieppe (obsolète)"},{idcc:"1564",libelle:"Métallurgie de Saône-et-Loire (obsolète)"},{idcc:"0911",libelle:"Métallurgie de Seine-et-Marne (obsolète)"},{idcc:"2980",libelle:"Métallurgie de la Somme (obsolète)"},{idcc:"1592",libelle:"Métallurgie du Valenciennois et du Cambrésis (obsolète)"},{idcc:"2489",libelle:"Métallurgie de la Vendée (obsolète)"},{idcc:"1634",libelle:"Métallurgie des Côtes-d'Armor (obsolète)"},{idcc:"2630",libelle:"Métallurgie des Bouches-du-Rhône (obsolète)"},{idcc:"1315",libelle:"Industries métallurgiques et mécaniques de la Haute-Marne et de la Meuse (obsolète)"},{idcc:"1732",libelle:"Métallurgie de l'Yonne (obsolète)"},{idcc:"1560",libelle:"Métallurgiques des Alpes-Maritimes (obsolète)"},{idcc:"0979",libelle:"Métallurgiques de l'arrondissement du Havre (obsolète)"},{idcc:"2128",libelle:"Mutualité"},{idcc:"1077",libelle:"Négoce et industrie des produits du sol, engrais et produits connexes"},{idcc:"1880",libelle:"Négoce de l'ameublement"},{idcc:"1982",libelle:"Négoce et prestations de services dans les domaines médico-techniques"},{idcc:"1947",libelle:"Négoce de bois d'oeuvre et produits dérivés (obsolète)"},{idcc:"0054",libelle:"Non-cadres des industries métallurgiques et mécaniques de la région parisienne (obsolète)"},{idcc:"0998",libelle:"Non-cadres de l'exploitation d'équipements thermiques et de génie climatique"},{idcc:"2205",libelle:"Notaires"},{idcc:"3220",libelle:"Offices publics de l'habitat"},{idcc:"3245",libelle:"Opérateurs de voyages et guides"},{idcc:"1431",libelle:"Optique-lunetterie de détail"},{idcc:"1316",libelle:"Organismes de tourisme social et familial"},{idcc:"1909",libelle:"Organismes de tourisme"},{idcc:"1516",libelle:"Organismes de formation"},{idcc:"1790",libelle:"Parcs de loisirs et d'attractions"},{idcc:"1267",libelle:"Pâtisserie"},{idcc:"1000",libelle:"Personnel des cabinets d'avocats"},{idcc:"1147",libelle:"Personnel des cabinets médicaux"},{idcc:"0275",libelle:"Personnel au sol du transport aérien"},{idcc:"2046",libelle:"Personnel non médical des centres de lutte contre le cancer"},{idcc:"2972",libelle:"Personnel sédentaire des entreprises de navigation"},{idcc:"1558",libelle:"Personnel des industries céramiques"},{idcc:"1996",libelle:"Pharmacie d'officine"},{idcc:"1504",libelle:"Poissonnerie"},{idcc:"0759",libelle:"Pompes funèbres"},{idcc:"2683",libelle:"Portage de presse"},{idcc:"3017",libelle:"Ports et Manutention"},{idcc:"3230",libelle:"Presse (Information spécialisée [ETAM et cadres])"},{idcc:"3242",libelle:"Presse quotidienne et hebdomadaire en régions"},{idcc:"2098",libelle:"Prestataires de services du secteur tertiaire"},{idcc:"1351",libelle:"Prévention et sécurité"},{idcc:"1512",libelle:"Promotion immobilière"},{idcc:"0292",libelle:"Plasturgie"},{idcc:"3168",libelle:"Professions de la photographie"},{idcc:"3244",libelle:"Professions réglementées auprès des juridictions"},{idcc:"1555",libelle:"Produits à usage pharmaceutique, parapharmaceutique et vétérinaire"},{idcc:"1513",libelle:"Production des eaux embouteillées, des boissons rafraîchissantes sans alcool et de bière"},{idcc:"2642",libelle:"Production audiovisuelle"},{idcc:"3238",libelle:"Production et transformation des papiers et cartons"},{idcc:"0653",libelle:"Producteurs salariés de base des services extérieurs de production des sociétés d'assurances"},{idcc:"0993",libelle:"Prothèse dentaire"},{idcc:"0086",libelle:"Publicité"},{idcc:"1621",libelle:"Répartition pharmaceutique"},{idcc:"0454",libelle:"Remontées mécaniques et domaines skiables"},{idcc:"1266",libelle:"Restauration de collectivités"},{idcc:"1501",libelle:"Restauration rapide"},{idcc:"1413",libelle:"Salariés permanents des entreprises de travail temporaire"},{idcc:"3216",libelle:"Salariés du négoce des matériaux de construction"},{idcc:"3219",libelle:"Salariés en portage salarial"},{idcc:"1875",libelle:"Salariés des cabinets et cliniques vétérinaires"},{idcc:"0897",libelle:"Services de prévention et de santé au travail interentreprises"},{idcc:"1090",libelle:"Services de l'automobile"},{idcc:"2147",libelle:"Services d'eau et d'assainissement"},{idcc:"2344",libelle:"Sidérurgie (Nord, Moselle, Meurthe-et-Moselle)"},{idcc:"1672",libelle:"Sociétés d'assurances"},{idcc:"1801",libelle:"Sociétés d'assistance"},{idcc:"2150",libelle:"Sociétés anonymes et fondations d'HLM"},{idcc:"3090",libelle:"Spectacle vivant (secteur privé)"},{idcc:"2511",libelle:"Sport"},{idcc:"2728",libelle:"Sucreries, sucreries-distilleries et raffineries de sucre"},{idcc:"2219",libelle:"Taxis parisiens salariés"},{idcc:"2148",libelle:"Télécommunications"},{idcc:"3241",libelle:"Télédiffusion"},{idcc:"1424",libelle:"Transports publics"},{idcc:"0016",libelle:"Transports routiers et activités auxiliaires du transport"},{idcc:"1170",libelle:"Tuiles et briques (obsolète)"},{idcc:"0087",libelle:"Ouvriers des industries de carrières et de matériaux (obsolète)"},{idcc:"1702",libelle:"Ouvriers de travaux publics"},{idcc:"1596",libelle:"Ouvriers des entreprises du bâtiment de moins de 10 salariés"},{idcc:"1597",libelle:"Ouvriers des entreprises du bâtiment de plus de 10 salariés"},{idcc:"2389",libelle:"Ouvriers du bâtiment et des travaux publics région de La Réunion"},{idcc:"2328",libelle:"Ouvriers du bâtiment et des travaux publics de la Guadeloupe et dépendances"},{idcc:"2564",libelle:"Vétérinaires praticiens salariés"},{idcc:"0493",libelle:"Vins, cidres, jus de fruits, sirops, spiritueux et liqueurs de France"}].sort((e,t)=>e.libelle.localeCompare(t.libelle,"fr")),mt='<option value="">— Choisir une CCN —</option>'+Ne.map(e=>`<option value="${e.idcc}">${e.idcc} — ${e.libelle}</option>`).join("");let O=[];window.forgeNav=function(e){["liste","detail","creer"].forEach(t=>{document.getElementById("forge-"+t).style.display=t===e?"block":"none"})};async function pt(){forgeNav("liste");const e=document.getElementById("forge-cards"),t=document.getElementById("forge-subtitle");e.innerHTML='<div style="color:var(--muted);font-size:0.75rem;padding:0.5rem 0">chargement…</div>';try{const l=await fetch("/forge/contributeurs");if(!l.ok){const i=await l.text();throw new Error(`HTTP ${l.status} — ${i||l.statusText}`)}O=await l.json();const s=O.length;t.textContent=s===0?"aucun contributeur pour l'instant":`${s} contributeur${s>1?"s":""} · ${O.reduce((i,o)=>i+o.expertises.length,0)} expertises CCN`,e.innerHTML=s===0?'<div style="color:var(--muted);font-size:0.75rem">Aucun profil encore — sois le premier à rejoindre.</div>':O.map(ft).join("")}catch(l){e.innerHTML=`<div style="color:var(--red);font-size:0.75rem">Erreur : ${u(Z(l))}</div>`}}function ft(e){const t=e.expertises.slice(0,5).map(s=>`<span class="ccn-badge ${s.niveau==="Maîtrisée"?"m":s.niveau==="Pratiquée"?"p":"c"}" title="${u(s.niveau)}">${u(s.ccn_libelle)}</span>`).join(""),l=e.expertises.length>5?`<span class="ccn-badge c">+${e.expertises.length-5}</span>`:"";return`
    <div class="forge-card" onclick="forgeAfficherProfil('${u(e.pseudo)}')">
      <div class="forge-card-pseudo">${u(e.pseudo)}</div>
      <div class="forge-card-poste">${u(e.poste)} <span style="color:var(--dim);font-size:0.6em">${e.poste_est_actuel?"actuel":"visé"}</span></div>
      <div class="forge-card-ccn">${t}${l}</div>
      <div class="forge-card-stats">
        <span><span class="forge-stat-val">${e.votes_received}</span> votes</span>
        <span><span class="forge-stat-val">${e.topics_count}</span> sujets</span>
        <span><span class="forge-stat-val">${e.posts_count}</span> réponses</span>
      </div>
    </div>`}async function bt(e){forgeNav("detail");const t=document.getElementById("forge-profil-content");t.innerHTML='<div style="color:var(--muted);font-size:0.75rem">chargement…</div>';try{let l=O.find(s=>s.pseudo.toLowerCase()===e.toLowerCase());if(!l){const s=await fetch(`/profil/${encodeURIComponent(e)}`);if(!s.ok)throw new Error(`HTTP ${s.status} — ${await s.text()||s.statusText}`);l=await s.json()}t.innerHTML=gt(l)}catch(l){t.innerHTML=`<div style="color:var(--red);font-size:0.75rem">Erreur : ${u(Z(l))}</div>`}}function gt(e){const t=e.linkedin_url?`<a class="profil-linkedin" href="${u(e.linkedin_url)}" target="_blank" rel="noopener noreferrer">↗ LinkedIn</a>`:"",s=[{niveau:"Maîtrisée",cls:"m",items:e.expertises.filter(a=>a.niveau==="Maîtrisée")},{niveau:"Pratiquée",cls:"p",items:e.expertises.filter(a=>a.niveau==="Pratiquée")},{niveau:"Connue",cls:"c",items:e.expertises.filter(a=>a.niveau==="Connue")}].filter(a=>a.items.length>0).map(a=>`
    <tr class="profil-ccn-section"><td colspan="3">${u(a.niveau)}</td></tr>
    ${a.items.map(n=>`
    <tr>
      <td class="profil-ccn-idcc">${u(n.ccn_idcc)}</td>
      <td>${u(n.ccn_libelle)}</td>
      <td><span class="ccn-badge ${a.cls}">${u(a.niveau)}</span></td>
    </tr>`).join("")}`).join(""),i=e.expertises.length===0?'<div style="color:var(--muted);font-size:0.72rem">Aucune CCN renseignée.</div>':`<table class="profil-ccn-tbl">${s}</table>`,o=e.created_at?be(e.created_at.slice(0,10)):"—";return`
    <div class="profil-head">
      <div>
        <div class="profil-pseudo">${u(e.pseudo)}</div>
        <div class="profil-poste">${u(e.poste)} <span style="color:var(--dim);font-size:0.85em">(${e.poste_est_actuel?"poste actuel":"poste visé"})</span></div>
        ${t}
      </div>
      <div class="profil-since">membre depuis le ${o}</div>
    </div>

    <div class="profil-body">
      <div class="sect-label">PAIE FRANÇAISE</div>
      ${e.paie_fr_niveau?`<span class="ccn-badge ${e.paie_fr_niveau==="Maîtrisée"?"m":e.paie_fr_niveau==="Pratiquée"?"p":"c"}" style="font-size:0.75rem;padding:0.2rem 0.6rem">${u(e.paie_fr_niveau)}</span>`:'<span style="color:var(--dim);font-size:0.7rem">non renseigné</span>'}

      ${e.pays&&e.pays.length>0?`
      <div class="sect-label" style="margin-top:1rem">PAIE INTERNATIONALE</div>
      <table class="profil-ccn-tbl">
        ${[{niveau:"Maîtrisée",cls:"m",items:e.pays.filter(a=>a.niveau==="Maîtrisée")},{niveau:"Pratiquée",cls:"p",items:e.pays.filter(a=>a.niveau==="Pratiquée")},{niveau:"Connue",cls:"c",items:e.pays.filter(a=>a.niveau==="Connue")}].filter(a=>a.items.length>0).map(a=>`
            <tr class="profil-ccn-section"><td colspan="3">${u(a.niveau)}</td></tr>
            ${a.items.map(n=>`
            <tr>
              <td class="profil-ccn-idcc">${u(n.pays_code)}</td>
              <td>${u(n.pays_libelle)}</td>
              <td><span class="ccn-badge ${a.cls}">${u(a.niveau)}</span></td>
            </tr>`).join("")}`).join("")}
      </table>`:""}

      <div class="sect-label" style="margin-top:1rem">EXPERTISES CCN</div>
      ${i}
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
    </div>`}window.setPosteType=function(e){document.getElementById("poste_est_actuel_input").value=e?"1":"0",document.getElementById("ptog-actuel").className="ptog "+(e?"ptog-on":"ptog-off"),document.getElementById("ptog-vise").className="ptog "+(e?"ptog-off":"ptog-on")};const Oe=[{code:"BE",libelle:"Belgique"},{code:"LU",libelle:"Luxembourg"},{code:"DE",libelle:"Allemagne"},{code:"CH",libelle:"Suisse"},{code:"IT",libelle:"Italie"},{code:"MC",libelle:"Monaco"},{code:"ES",libelle:"Espagne"},{code:"AD",libelle:"Andorre"},{code:"GB",libelle:"Royaume-Uni"}],vt=Oe.map(e=>`<option value="${e.code}">${u(e.libelle)}</option>`).join("");let He=0;window.forgeAjouterPays=function(){const e=++He,t=document.createElement("div");t.className="forge-ccn-row",t.id="forge-pays-"+e,t.innerHTML=`
    <select class="forge-pays-select">${vt}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprPays(${e})" title="Supprimer">×</button>`,document.getElementById("forge-pays-list").appendChild(t)};window.forgeSupprPays=function(e){document.getElementById("forge-pays-"+e)?.remove()};let De=0;window.forgeAjouterCcn=function(){const e=++De,t=document.createElement("div");t.className="forge-ccn-row",t.id="forge-ccn-"+e,t.innerHTML=`
    <select class="forge-ccn-select">${mt}</select>
    <select class="forge-ccn-niveau">
      <option value="Connue">Connue</option>
      <option value="Pratiquée">Pratiquée</option>
      <option value="Maîtrisée" selected>Maîtrisée</option>
    </select>
    <button type="button" class="forge-ccn-del" onclick="forgeSupprCcn(${e})" title="Supprimer">×</button>`,document.getElementById("forge-ccn-list").appendChild(t)};window.forgeSupprCcn=function(e){document.getElementById("forge-ccn-"+e)?.remove()};window.forgeSoumettre=async function(e){e.preventDefault();const t=document.getElementById("forge-form"),l=document.getElementById("forge-form-err"),s=document.getElementById("forge-submit-btn");l.textContent="";const i=[];document.querySelectorAll('[id^="forge-pays-"]').forEach(n=>{const c=n.querySelector(".forge-pays-select")?.value,f=n.querySelector(".forge-ccn-niveau")?.value,p=Oe.find(x=>x.code===c);c&&p&&i.push({pays_code:c,pays_libelle:p.libelle,niveau:f})});const o=[];document.querySelectorAll('.forge-ccn-row:not([id^="forge-pays-"])').forEach(n=>{const c=n.querySelector(".forge-ccn-select").value,f=n.querySelector(".forge-ccn-niveau").value,p=Ne.find(x=>x.idcc===c);c&&p&&o.push({ccn_idcc:c,ccn_libelle:p.libelle,niveau:f})});const a={email:t.querySelector('[name="email"]').value.trim(),pseudo:t.querySelector('[name="pseudo"]').value.trim(),poste:t.querySelector('[name="poste"]').value.trim(),linkedin_url:t.querySelector('[name="linkedin_url"]').value.trim()||null,poste_est_actuel:t.querySelector('[name="poste_est_actuel"]').value!=="0",paie_fr_niveau:t.querySelector('[name="paie_fr_niveau"]').value||null,pays:i,expertises:o};if(!a.email){l.textContent="Email requis.";return}if(!a.pseudo){l.textContent="Pseudo requis.";return}if(!a.poste){l.textContent="Poste requis.";return}s.disabled=!0,s.textContent="[ envoi… ]";try{const n=await fetch("/forge/profil",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(a)});if(!n.ok)throw new Error(`HTTP ${n.status} — ${await n.text()||n.statusText}`);const c=await n.json();O.unshift(c),t.reset(),document.getElementById("forge-pays-list").innerHTML="",document.getElementById("forge-ccn-list").innerHTML="",He=0,De=0,bt(c.pseudo)}catch(n){l.textContent=Z(n),s.disabled=!1,s.textContent="[ Rejoindre la Forge ]"}};const yt=[{prenom:"Geralt",nom:"de Riv"},{prenom:"Sam",nom:"Vimes"},{prenom:"Elric",nom:"de Melniboné"},{prenom:"Druss",nom:"la Légende"},{prenom:"Logen",nom:"Neuf-Doigts"},{prenom:"Aragorn",nom:"Grands-Pas"},{prenom:"Jon",nom:"Shannow"},{prenom:"Salim",nom:"Dhibi"},{prenom:"Bayaz",nom:"le Magi"},{prenom:"Merlin",nom:"l'Enchanteur"}],ht=[{prenom:"Lyra",nom:"Belacqua"},{prenom:"Hermione",nom:"Granger"},{prenom:"Eowyn",nom:"du Rohan"},{prenom:"Ellana",nom:"Caldin"},{prenom:"Ferro",nom:"Maljinn"},{prenom:"Magrat",nom:"Garlick"},{prenom:"Ewilan",nom:"Gil'Sayan"},{prenom:"Sigarni",nom:"la Guerrière"},{prenom:"Rikke",nom:"la Nord"},{prenom:"Tanaquil",nom:"la Magicienne"}],Le=[17,16,16,15,15,15,14,14,14,13,13,11],J=Le[Math.floor(Math.random()*Le.length)]/100;let Me="H",ve=!1;function Be(e){return e[Math.floor(Math.random()*e.length)]}function Ue(e,t){["d-prenom","m-prenom"].forEach(l=>{const s=document.getElementById(l);s&&(s.value=e)}),["d-nom","m-nom"].forEach(l=>{const s=document.getElementById(l);s&&(s.value=t)}),ve=!1}function je(e,t=!1){const l=e==="H";["d-hf-h","m-hf-h"].forEach(s=>{document.getElementById(s)?.classList.toggle("ptog-on",l),document.getElementById(s)?.classList.toggle("ptog-off",!l)}),["d-hf-f","m-hf-f"].forEach(s=>{document.getElementById(s)?.classList.toggle("ptog-on",!l),document.getElementById(s)?.classList.toggle("ptog-off",l)}),t&&document.querySelectorAll(".genre-ecart-hint").forEach(s=>{s.textContent=l?s.dataset.textHf:s.dataset.textFh,s.style.display="inline"})}window.setGenre=function(e){if(e===Me)return;if(!ve){const l=e==="F"?window._heroF:window._heroH;Ue(l.prenom,l.nom)}const t=e==="F"?1-J:1/(1-J);["d-brut","m-brut"].forEach(l=>{const s=document.getElementById(l);s&&(s.value=Math.round(parseFloat(s.value)*t))}),Me=e,je(e,!0)};const ye=document.getElementById("burger-btn"),ie=document.getElementById("burger-menu");function Et(){ye.classList.add("open"),ie.classList.add("open")}window.closeBurger=function(){ye.classList.remove("open"),ie.classList.remove("open")};ye.addEventListener("click",e=>{e.stopPropagation(),ie.classList.contains("open")?closeBurger():Et()});document.addEventListener("click",()=>closeBurger());ie.addEventListener("click",e=>e.stopPropagation());const xt=[{id:1,q:"Quel est le taux global de la CSG sur les revenus d'activité ?",rep:"9,2 %",mr:["9,4 %","9,6 %","9,8 %"],src:"CSS, art. L136-8"},{id:2,q:"Quelle part de la CSG est déductible de l'impôt sur le revenu ?",rep:"6,8 %",mr:["8,4 %","7,4 %","8,2 %"],src:"CGI, art. 154 quinquies"},{id:3,q:"Quelle part de la CSG est non déductible ?",rep:"2,4 %",mr:["3,2 %","2,8 %","3,0 %"],src:"CGI, art. 154 quinquies"},{id:4,q:"Quel est le taux de la CRDS ?",rep:"0,5 %",mr:["0,4 %","0,37 %","0,45 %"],src:"Ordonnance n°96-50 du 24/01/1996"},{id:5,q:"Sur quelle base se calcule la CSG/CRDS ?",rep:"98,25 % du brut",mr:["97,25 % du brut","98,75 % du brut","99,25 % du brut"],src:"CSS, art. L136-2"},{id:6,q:"Quel est le PMSS mensuel 2024 ?",rep:"3 864 €",mr:["3 666 €","3 925 €","3 428 €"],src:"Arrêté du 19/12/2023"},{id:7,q:"Quel est le PMSS annuel 2024 ?",rep:"46 368 €",mr:["43 992 €","46 836 €","47 004 €"],src:"Arrêté du 19/12/2023"},{id:8,q:"Quel est le nombre de jours de carence avant versement des IJSS maladie ?",rep:"3 jours",mr:["1 jour","7 jours","5 jours"],src:"CSS, art. R323-1"},{id:9,q:"Quel est le taux de base des IJSS maladie ?",rep:"50 %",mr:["60 %","66,66 %","45 %"],src:"CSS, art. R323-4"},{id:10,q:"Quelle est la période de référence retenue pour calculer les IJSS ?",rep:"3 mois",mr:["6 mois","12 mois","1 mois"],src:"CSS, art. R323-4"},{id:11,q:"Quel est le plafond des IJSS maladie ?",rep:"1,8 SMIC",mr:["1,5 SMIC","2 SMIC","1,6 SMIC"],src:"CSS, art. R323-4"},{id:12,q:"Quel est le taux de la cotisation vieillesse plafonnée salarié ?",rep:"6,90 %",mr:["6,70 %","7,10 %","6,60 %"],src:"CSS, art. D242-4"},{id:13,q:"Quel est le taux de la cotisation vieillesse déplafonnée salarié ?",rep:"0,40 %",mr:["0,30 %","0,50 %","0,45 %"],src:"CSS, art. D242-4"},{id:14,q:"Quel est le taux de la cotisation vieillesse plafonnée employeur ?",rep:"8,55 %",mr:["8,45 %","8,75 %","8,20 %"],src:"CSS, art. D242-4"},{id:15,q:"Quel est le taux normal des allocations familiales ?",rep:"5,25 %",mr:["5,40 %","4,90 %","5,10 %"],src:"CSS, art. L241-6"},{id:16,q:"En dessous de quel seuil (en SMIC) s'applique le taux réduit des allocations familiales ?",rep:"3,5 SMIC",mr:["3 SMIC","2,5 SMIC","4 SMIC"],src:"CSS, art. L241-6-1"},{id:17,q:"Quel est le taux réduit des allocations familiales ?",rep:"3,45 %",mr:["3,25 %","3,75 %","3,60 %"],src:"CSS, art. D241-3-2"},{id:18,q:"Quel est le taux de la contribution solidarité autonomie (CSA) ?",rep:"0,30 %",mr:["0,10 %","0,50 %","0,25 %"],src:"CASF, art. L14-10-4"},{id:19,q:"Quel est le taux du FNAL pour les entreprises de moins de 50 salariés ?",rep:"0,10 %",mr:["0,30 %","0,20 %","0,50 %"],src:"CSS, art. L834-1"},{id:20,q:"Quel est le taux du FNAL pour les entreprises d'au moins 50 salariés ?",rep:"0,50 %",mr:["0,30 %","0,10 %","0,40 %"],src:"CSS, art. L834-1"},{id:21,q:"Comment est déterminé le taux AT/MP ?",rep:"Variable",mr:["Fixé à 0,70 % pour tous","Forfait de 2 % du brut","Fixé légalement à 1 %"],src:"CSS, art. L242-5"},{id:22,q:"Quel est le SMIC mensuel brut 2024 (base 35h) ?",rep:"1 766,92 €",mr:["1 709,28 €","1 801,80 €","1 747,20 €"],src:"Décret n°2023-1216"},{id:23,q:"Quelle est la durée mensuelle de travail pour 35h hebdomadaires ?",rep:"151,67 h",mr:["152,25 h","150,50 h","153,33 h"],src:"Code du travail, art. L3121-27"},{id:24,q:"Quel est le taux de majoration pour les 8 premières heures supplémentaires ?",rep:"25 %",mr:["10 %","20 %","30 %"],src:"Code du travail, art. L3121-36"},{id:25,q:"Quel est le taux de majoration pour les heures supplémentaires au-delà des 8 premières ?",rep:"50 %",mr:["25 %","40 %","75 %"],src:"Code du travail, art. L3121-36"},{id:26,q:"Quel est le plafond annuel d'exonération fiscale et sociale sur les heures supplémentaires ?",rep:"7 500 €",mr:["5 000 €","7 000 €","8 000 €"],src:"CGI, art. 81 quater"},{id:27,q:"La réduction générale de cotisations patronales est-elle fixe ou variable ?",rep:"Variable",mr:["Fixée à 16 % du brut","Plafonnée à 26 % pour tous","Identique quel que soit l'effectif"],src:"CSS, art. L241-13"},{id:28,q:"Quel est le taux maximum de la réduction Fillon ?",rep:"~32 %",mr:["~26 %","~28 %","~35 %"],src:"CSS, art. D241-7"},{id:29,q:"Quel est le taux de cotisation chômage à la charge du salarié ?",rep:"0 %",mr:["2,40 %","0,95 %","1,20 %"],src:"Loi n°2018-771"},{id:30,q:"Quel est le taux de cotisation chômage à la charge de l'employeur ?",rep:"4,05 %",mr:["3,45 %","4,40 %","3,90 %"],src:"Convention Unédic"}];let q=null,H=null,K=0,me=0,pe=0,qe=-1,F=!1,D=null,fe=!1,U=!1,X=0;function Ae(e){return String(e).toLowerCase().replace(/\s+/g,"").replace(/,/g,".").replace(/[€%°~]/g,"").trim()}function It(e,t){if(!t.trim())return!1;const l=Ae(e),s=Ae(t);if(l===s)return!0;const i=parseFloat(l),o=parseFloat(s);return!isNaN(i)&&!isNaN(o)&&Math.abs(i-o)<.001}function Ge(e){const t=[...e];for(let l=t.length-1;l>0;l--){const s=Math.floor(Math.random()*(l+1));[t[l],t[s]]=[t[s],t[l]]}return t}function Qe(e){return(e/1e3).toFixed(1)+"s"}function St(){return H&&(clearInterval(H),H=null),Date.now()-K}function Ct(e){q=e,F=!1,document.getElementById("qz-num").textContent="Q."+String(e.id).padStart(2,"0"),document.getElementById("qz-q").textContent=e.q,document.getElementById("qz-clock").textContent="0.0s",document.getElementById("qz-input").value="",document.getElementById("qz-input").disabled=!1,document.getElementById("qz-result").style.display="none",document.getElementById("qz-saisie").style.opacity="1";const t=document.getElementById("qz-carre-cb");t.checked=!1,document.getElementById("qz-carre").style.display="none",document.getElementById("qz-carre").style.opacity="1",document.getElementById("qz-fifty").style.display="none",D&&clearTimeout(D),fe=!1,U=!1,D=setTimeout(()=>{fe=!0,!F&&!U&&document.getElementById("qz-carre").style.display!=="none"&&(document.getElementById("qz-fifty").style.display="inline-flex")},8e3);const l=Ge([e.rep,...e.mr]),s=document.getElementById("qz-choix");s.innerHTML="",l.forEach(i=>{const o=document.createElement("button");o.className="qz-choice",o.textContent=i,o.onclick=()=>Ye(i),s.appendChild(o)}),H&&clearInterval(H),K=Date.now(),H=setInterval(()=>{F||(document.getElementById("qz-clock").textContent=Qe(Date.now()-K))},100),document.getElementById("qz-input").focus()}function Ve(e,t,l){F=!0,St(),pe++,e&&me++,D&&(clearTimeout(D),D=null),document.getElementById("qz-fifty").style.display="none";const s=X>0?" · ½ ×"+X:"";document.getElementById("qz-score").textContent=me+" / "+pe+s;const i=document.getElementById("qz-verdict");i.textContent=e?"✓ JUSTE":"✗ FAUX",i.className="qz-verdict "+(e?"ok":"ko");const o=document.getElementById("qz-ans-line");e?o.innerHTML="Bonne réponse : <strong>"+q.rep+"</strong>":o.innerHTML="Réponse correcte : <strong>"+q.rep+"</strong>";const a=U?" · ½":"";document.getElementById("qz-time-line").textContent="⏱ "+Qe(t)+" ("+l+a+")",document.getElementById("qz-src-line").textContent=q.src,document.getElementById("qz-result").style.display="block",document.getElementById("qz-input").disabled=!0,document.getElementById("qz-saisie").style.opacity="0.4",document.querySelectorAll(".qz-choice").forEach(n=>{n.disabled=!0,n.textContent===q.rep&&n.classList.add("qz-correct")})}function $t(e){document.getElementById("qz-carre").style.display=e.checked?"block":"none",e.checked&&fe&&!F&&!U&&(document.getElementById("qz-fifty").style.display="inline-flex")}function wt(){if(U||F)return;U=!0,X++,document.getElementById("qz-fifty").style.display="none";const t=Array.from(document.querySelectorAll(".qz-choice")).filter(s=>s.textContent!==q.rep);Ge(t).slice(0,2).forEach(s=>{s.disabled=!0,s.style.opacity="0.12",s.style.pointerEvents="none"});const l=X>0?" · ½ ×"+X:"";document.getElementById("qz-score").textContent=me+" / "+pe+l}function _t(){if(F)return;const e=document.getElementById("qz-input").value,t=Date.now()-K,l=It(q.rep,e);document.querySelectorAll(".qz-choice").forEach(s=>{s.disabled=!0,s.textContent===q.rep&&s.classList.add("qz-correct")}),Ve(l,t,"saisie")}function Ye(e){if(F)return;const t=Date.now()-K,l=e===q.rep;document.querySelectorAll(".qz-choice").forEach(s=>{s.disabled=!0,s.textContent===q.rep?s.classList.add("qz-correct"):s.textContent===e&&!l&&s.classList.add("qz-wrong")}),Ve(l,t,"carré")}function Je(){const e=xt.filter(l=>l.id!==qe),t=e[Math.floor(Math.random()*e.length)];qe=t.id,Ct(t)}function Lt(){Je()}window.quizzValider=_t;window.quizzChoix=Ye;window.quizzNext=Je;window.quizzToggleCarre=$t;window.quizzFiftyFifty=wt;
