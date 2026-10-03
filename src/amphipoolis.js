// ── Amphipoolis (prototype) ──────────────────────────────────────────────────
//
// Discussion sous pseudonyme, modérée message par message. Le serveur
// (crate amphipoolis/) porte toutes les règles ; ce module ne fait que les
// montrer :
//   - on entre avec un pseudonyme et une phrase secrète (aucun e-mail) ;
//   - le rôle (participant ou modérateur) se choisit à la création, un
//     pseudonyme de chaque par jour ;
//   - un texte publié vit 7 jours ; chaque +1 d'un autre les rallonge de 7,
//     jusqu'à 30 jours après sa publication ;
//   - un sujet ou un message n'est visible des autres qu'une fois publié par
//     un modérateur — jamais par le pseudonyme qui l'a écrit ; une même
//     personne peut en revanche publier avec son pseudonyme modérateur ce
//     qu'elle a écrit en participant (voulu : c'est ainsi qu'on teste) ;
//   - un refus porte un motif, que seul l'auteur voit ;
//   - un pseudonyme refusé doit être remplacé, l'ancien nom est réservé.
//
// Tout texte venu du serveur passe par esc() : les messages sont du texte
// brut, affiché tel quel (white-space: pre-wrap), jamais interprété.

const API = '/api/amphipoolis';
const CLE_JETON = 'amph.jeton';
// Rafraîchissement : rapide dans un fil (la conversation), plus lent ailleurs.
const RAFRAICHIR_FIL_MS = 2000;
const RAFRAICHIR_MS = 5000;
// Veille des modérateurs : tourne même hors de la vue et onglet en arrière-plan,
// pour signaler la file d'attente (titre de l'onglet, son).
const VEILLE_MS = 15000;
const CLE_SON = 'amph.son';
const TITRE_BASE = document.title;
const TEXTE_MAX = 2000;
const TITRE_MAX = 120;

const MOTIFS = {
  hors_sujet:           'Hors sujet',
  discourtois:          'Discourtois',
  donnees_personnelles: 'Données personnelles',
  illicite:             'Contenu illicite',
  autre:                'Autre',
};

const etat = {
  jeton:    null,
  moi:      null,    // { nom, moderateur, a_renommer }
  aModerer: 0,
  vue:      'sujets', // 'sujets' | 'nouveau' | 'fil' | 'moderation'
  sujetId:  null,
  sujetPublie: false,
  ongletEntree: 'entrer', // 'entrer' | 'creer'
  roleCreation: false,    // rôle choisi à la création (true = modérateur)
  motifs:   {},      // choix de motif en cours, par « objet:id » (survit au rafraîchissement)
  minuteur: null,
  veille:   null,
  actif:    false,   // vue Amphipoolis affichée
  ecouteur: false,
};

// ── Utilitaires ──────────────────────────────────────────────────────────────

function esc(str) {
  return String(str ?? '')
    .replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;').replace(/'/g, '&#039;');
}

function sonActif() {
  try { return localStorage.getItem(CLE_SON) !== 'non'; } catch { return true; }
}
function ecrireSon(oui) {
  try { localStorage.setItem(CLE_SON, oui ? 'oui' : 'non'); } catch { /* navigation privée */ }
}

// Deux notes brèves et douces. Le navigateur n'autorise le son qu'après une
// interaction avec la page : avant, l'appel échoue en silence.
let _audio = null;
function bip() {
  if (!sonActif()) return;
  try {
    _audio = _audio || new (window.AudioContext || window.webkitAudioContext)();
    if (_audio.state === 'suspended') _audio.resume();
    const t0 = _audio.currentTime;
    [[660, 0], [880, 0.14]].forEach(([f, d]) => {
      const o = _audio.createOscillator(), g = _audio.createGain();
      o.type = 'sine'; o.frequency.value = f;
      g.gain.setValueAtTime(0.0001, t0 + d);
      g.gain.exponentialRampToValueAtTime(0.08, t0 + d + 0.02);
      g.gain.exponentialRampToValueAtTime(0.0001, t0 + d + 0.18);
      o.connect(g).connect(_audio.destination);
      o.start(t0 + d); o.stop(t0 + d + 0.2);
    });
  } catch { /* pas d'audio : le titre suffit */ }
}

// File d'attente des modérateurs : compteur de l'onglet, titre de la page,
// et un son quand de nouveaux textes arrivent.
function signaler(n) {
  const avant = etat.aModerer;
  etat.aModerer = n;
  majCompteur();
  document.title = etat.moi?.moderateur && n > 0 ? `(${n}) ${TITRE_BASE}` : TITRE_BASE;
  if (etat.moi?.moderateur && n > avant) bip();
}

function lireJeton() {
  try { return localStorage.getItem(CLE_JETON); } catch { return null; }
}
function ecrireJeton(j) {
  try { j ? localStorage.setItem(CLE_JETON, j) : localStorage.removeItem(CLE_JETON); } catch { /* navigation privée */ }
}

function quand(iso) {
  const d = new Date(iso);
  if (isNaN(d)) return '';
  const p = n => String(n).padStart(2, '0');
  return `${p(d.getDate())}/${p(d.getMonth() + 1)}/${d.getFullYear()} ${p(d.getHours())}:${p(d.getMinutes())}`;
}

// « encore 6 j », « encore 5 h » : ce qu'il reste à vivre à un texte.
function resteAVivre(iso) {
  const ms = new Date(iso) - Date.now();
  if (!iso || isNaN(ms)) return '';
  if (ms <= 0) return 'expire';
  const h = Math.ceil(ms / 3600000);
  return h > 24 ? `encore ${Math.ceil(h / 24)} j` : `encore ${h} h`;
}

// Bouton +1 : actif seulement sur le texte publié d'un autre, une fois.
function plus1Html(objet, t) {
  if (t.statut !== 'publie') return '';
  const off = t.a_moi || t.a_plus1;
  const titre = t.a_moi ? 'Pas de +1 à son propre texte'
    : t.a_plus1 ? 'Vous avez déjà donné un +1'
    : 'Chaque +1 rallonge la vie de ce texte de 7 jours (30 jours au plus)';
  return `<button class="am-plus1${t.a_plus1 ? ' donne' : ''}" data-a="plus1" data-objet="${objet}" data-id="${t.id}"${off ? ' disabled' : ''} title="${titre}">+1 <b>${t.plus1}</b></button>
    <span class="am-vie" title="Disparaît le ${quand(t.expire_le)}">${resteAVivre(t.expire_le)}</span>`;
}

function auteurHtml(nom) {
  return nom
    ? `<span class="am-auteur trad-skip">${esc(nom)}</span>`
    : '<span class="am-auteur retire">pseudonyme retiré</span>';
}

const racine = () => document.getElementById('am-racine');
const $ = id => document.getElementById(id);

class ErreurApi extends Error {
  constructor(message, code) { super(message); this.code = code; }
}

async function api(methode, chemin, corps) {
  const headers = { 'content-type': 'application/json' };
  if (etat.jeton) headers.authorization = `Bearer ${etat.jeton}`;
  let res;
  try {
    res = await fetch(API + chemin, {
      method: methode, headers,
      body: corps === undefined ? undefined : JSON.stringify(corps),
    });
  } catch {
    throw new ErreurApi('Serveur injoignable.', 0);
  }
  if (res.status === 204) return null;
  let data = null;
  const brut = await res.text();
  try { data = brut ? JSON.parse(brut) : null; } catch { /* quota : texte brut */ }
  if (!res.ok) {
    if (res.status === 401 && etat.jeton) {
      // Session expirée ou fermée ailleurs : retour à l'entrée.
      etat.jeton = null; etat.moi = null; ecrireJeton(null);
      rendre();
    }
    throw new ErreurApi(data?.erreur || brut || `Erreur ${res.status}`, res.status);
  }
  return data;
}

function statut(id, texte, classe = '') {
  const el = $(id);
  if (!el) return;
  el.textContent = texte;
  el.className = 'am-statut' + (classe ? ' ' + classe : '');
}

// Preuve de travail ALTCHA (celle du serveur, /altcha/challenge) : on cherche
// le nombre n tel que SHA-256(sel:n) = challenge. Quelques secondes au plus.
async function resoudreAltcha() {
  const r = await fetch('/altcha/challenge');
  if (!r.ok) throw new ErreurApi('Vérification anti-robot indisponible.', r.status);
  const c = await r.json();
  const enc = new TextEncoder();
  for (let n = 0; n <= c.maxNumber; n++) {
    const h = await crypto.subtle.digest('SHA-256', enc.encode(`${c.salt}:${n}`));
    const hex = [...new Uint8Array(h)].map(b => b.toString(16).padStart(2, '0')).join('');
    if (hex === c.challenge) {
      return btoa(JSON.stringify({
        algorithm: c.algorithm, challenge: c.challenge, number: n, salt: c.salt, signature: c.signature,
      }));
    }
  }
  throw new ErreurApi('Vérification anti-robot impossible.', 0);
}

// Chaque dépôt (pseudonyme, sujet, message) exige une preuve neuve. On la
// calcule d'avance, dès qu'une saisie commence : à l'envoi, elle est prête.
// Le serveur la refuse au-delà de 10 minutes ; on jette la nôtre à 8.
const PREUVE_VALIDITE_MS = 8 * 60 * 1000;
let _preuve = null; // { promesse, le }
function preparerPreuve() {
  if (_preuve && Date.now() - _preuve.le > PREUVE_VALIDITE_MS) _preuve = null;
  if (!_preuve) {
    const p = { le: Date.now(), promesse: null };
    p.promesse = resoudreAltcha().catch(e => { if (_preuve === p) _preuve = null; throw e; });
    _preuve = p;
  }
  return _preuve.promesse;
}
async function prendrePreuve() {
  const preuve = await preparerPreuve();
  _preuve = null; // à usage unique (anti-rejeu côté serveur)
  return preuve;
}

// Champ piège : hors écran, ignoré des lecteurs d'écran et du clavier. Un
// humain le laisse vide ; un robot remplisseur de formulaires le renseigne.
const PIEGE = `<div class="am-piege" aria-hidden="true"><label>Site web <input name="site" tabindex="-1" autocomplete="off"></label></div>`;
const piege = form => form?.querySelector('[name="site"]')?.value ?? '';

// ── Écrans ───────────────────────────────────────────────────────────────────

const REGLES = `
  <div class="am-card">
    <div class="am-lbl">// Règles de la taverne</div>
    <ul class="am-regles">
      <li><b>Pseudonyme strictement respecté.</b> On entre avec un pseudonyme et une phrase secrète : ni e-mail, ni adresse IP conservée. Personne d'autre ne peut parler sous votre nom.</li>
      <li><b>Révéler son identité est permis</b>, en votre âme et conscience — c'est votre choix, jamais une obligation. Mais rien ici ne vérifie qui est qui : n'importe qui peut prétendre être n'importe qui. Prenez toute identité annoncée pour ce qu'elle est, une simple affirmation.</li>
      <li><b>Chaque texte est lu avant d'être publié.</b> Un modérateur publie ou refuse ; en cas de refus, vous seul voyez le motif.</li>
      <li><b>Participant ou modérateur</b> : le rôle se choisit à la création du pseudonyme, un de chaque par jour. Vous pouvez valider vos propres textes : écrivez avec votre pseudonyme participant, puis publiez-les avec votre pseudonyme modérateur. C'est voulu, pour pouvoir tester tout le circuit sur ce prototype.</li>
      <li><b>En dernier recours</b>, un super administrateur peut également supprimer des messages, même déjà publiés.</li>
      <li><b>Rien n'est éternel.</b> Un texte publié vit 7 jours ; chaque +1 d'un autre pseudonyme lui ajoute 7 jours, jusqu'à 30 jours après sa publication. Un texte refusé ou jamais modéré disparaît au bout de 7 jours.</li>
      <li><b>Un pseudonyme discourtois</b> peut être refusé : vous en choisissez alors un autre, vos textes vous suivent.</li>
      <li class="am-avert"><b>Prototype.</b> Si vous voulez rester anonyme, n'écrivez rien qui permette de vous reconnaître (nom, poste, lieu, dates précises).</li>
    </ul>
  </div>`;

function ecranEntree() {
  const creer = etat.ongletEntree === 'creer';
  return `
    <div class="am-card">
      <div class="am-onglets">
        <button class="am-onglet ${creer ? '' : 'actif'}" data-a="onglet-entree" data-v="entrer">Entrer</button>
        <button class="am-onglet ${creer ? 'actif' : ''}" data-a="onglet-entree" data-v="creer">Créer un pseudonyme</button>
      </div>
      <form id="am-form-entree" autocomplete="off">${creer ? PIEGE : ''}
        <div class="am-champ">
          <label for="am-pseudo">Pseudonyme</label>
          <input id="am-pseudo" maxlength="30" required autocomplete="username">
          ${creer ? '<span class="am-aide">3 à 30 caractères : lettres, chiffres, espace, - _ . \'</span>' : ''}
        </div>
        <div class="am-champ">
          <label for="am-phrase">Phrase secrète</label>
          <input id="am-phrase" type="password" maxlength="200" required autocomplete="${creer ? 'new-password' : 'current-password'}">
          ${creer ? '<span class="am-aide">8 caractères au moins. Une courte phrase se retient mieux qu\'un mot de passe.</span>' : ''}
        </div>
        ${creer ? `
        <div class="am-champ">
          <label for="am-phrase2">Phrase secrète, encore</label>
          <input id="am-phrase2" type="password" maxlength="200" required autocomplete="new-password">
        </div>
        <div class="am-champ">
          <label>Rôle — définitif pour ce pseudonyme</label>
          <span class="am-mode" role="radiogroup" aria-label="Rôle" style="align-self:flex-start">
            <button type="button" class="${etat.roleCreation ? '' : 'actif'}" data-a="role" data-v="0" aria-checked="${!etat.roleCreation}" role="radio">Participant</button>
            <button type="button" class="${etat.roleCreation ? 'actif' : ''}" data-a="role" data-v="1" aria-checked="${etat.roleCreation}" role="radio">Modérateur</button>
          </span>
          <span class="am-aide">${etat.roleCreation
            ? 'Un modérateur écrit aussi, et lit la file d\'attente pour publier ou refuser les textes des autres.'
            : 'Un participant ouvre des sujets, répond, et donne des +1.'} Un pseudonyme de chaque rôle par jour.</span>
        </div>
        <label class="am-aide" style="display:flex;gap:0.4rem;align-items:flex-start;margin-bottom:0.7rem">
          <input type="checkbox" id="am-compris" required>
          <span>J'ai compris : sans e-mail, une phrase secrète perdue ne se récupère pas — le pseudonyme est perdu avec elle.</span>
        </label>` : ''}
        <div class="am-ligne am-ligne-fin">
          <button class="am-btn" type="submit" id="am-btn-entree">[ ${creer ? 'Créer et entrer' : 'Entrer'} ]</button>
        </div>
        <div class="am-statut" id="am-statut-entree"></div>
      </form>
    </div>
    ${REGLES}`;
}

function ecranRenommer() {
  return `
    <div class="am-card">
      <div class="am-lbl">// Pseudonyme refusé</div>
      <p class="am-regles" style="margin-bottom:0.8rem">
        Un modérateur a jugé votre pseudonyme contraire aux règles de courtoisie. Il ne s'affiche plus nulle part
        et ne pourra plus être repris. Choisissez-en un autre : vos sujets et messages passeront sous ce nouveau nom.
      </p>
      <form id="am-form-renommer" autocomplete="off">
        <div class="am-champ">
          <label for="am-nouveau">Nouveau pseudonyme</label>
          <input id="am-nouveau" maxlength="30" required>
        </div>
        <div class="am-ligne am-ligne-fin">
          <button class="am-btn discret" type="button" data-a="sortir">[ Sortir ]</button>
          <button class="am-btn" type="submit">[ Valider ]</button>
        </div>
        <div class="am-statut" id="am-statut-renommer"></div>
      </form>
    </div>`;
}

function barre() {
  const m = etat.moi;
  return `
    <div class="am-barre">
      <span class="am-qui">Vous êtes <b class="trad-skip">${esc(m.nom)}</b></span>
      <span class="am-badge ${m.moderateur ? 'objet' : 'role'}" style="margin-left:0">${m.moderateur ? 'modérateur' : 'participant'}</span>
      <span class="am-espace"></span>
      ${m.moderateur ? `<button class="am-lien" data-a="son" title="Son à l'arrivée de textes à modérer">son : ${sonActif() ? 'oui' : 'non'}</button>` : ''}
      <button class="am-lien" data-a="sortir">sortir</button>
    </div>
    <div class="am-onglets">
      <button class="am-onglet ${etat.vue === 'moderation' ? '' : 'actif'}" data-a="vue" data-v="sujets">Sujets</button>
      ${m.moderateur ? `<button class="am-onglet ${etat.vue === 'moderation' ? 'actif' : ''}" data-a="vue" data-v="moderation">Modération<span class="am-compte" id="am-compte"${etat.aModerer ? '' : ' hidden'}>${etat.aModerer}</span></button>` : ''}
    </div>`;
}

function ecranSujets() {
  return `
    <div class="am-card">
      <div class="am-ligne" style="margin-bottom:0.6rem">
        <div class="am-lbl" style="margin:0;flex:1">// Sujets</div>
        <button class="am-btn" data-a="vue" data-v="nouveau">[ + Ouvrir un sujet ]</button>
      </div>
      <div id="am-liste"><div class="am-vide">Chargement…</div></div>
    </div>`;
}

function ecranNouveau() {
  return `
    <div class="am-card">
      <div class="am-lbl">// Ouvrir un sujet</div>
      <form id="am-form-sujet" autocomplete="off">${PIEGE}
        <div class="am-champ">
          <label for="am-titre">Titre</label>
          <input id="am-titre" maxlength="${TITRE_MAX}" required>
        </div>
        <div class="am-champ">
          <label for="am-texte-sujet">Premier message</label>
          <textarea id="am-texte-sujet" class="am-saisie" maxlength="${TEXTE_MAX}" required></textarea>
          <div class="am-compteur" data-compteur="am-texte-sujet">0 / ${TEXTE_MAX}</div>
        </div>
        <div class="am-ligne am-ligne-fin">
          <button class="am-btn discret" type="button" data-a="vue" data-v="sujets">[ Annuler ]</button>
          <button class="am-btn" type="submit">[ Déposer ]</button>
        </div>
        <div class="am-statut" id="am-statut-sujet"></div>
        <div class="am-aide">Le sujet sera visible des autres une fois publié par un modérateur.</div>
      </form>
    </div>`;
}

function ecranFil() {
  return `
    <div class="am-card">
      <button class="am-lien" data-a="vue" data-v="sujets">← tous les sujets</button>
      <div id="am-fil" style="margin-top:0.7rem"><div class="am-vide">Chargement…</div></div>
    </div>
    <div class="am-card" id="am-repondre" hidden>
      <div class="am-lbl">// Répondre</div>
      <form id="am-form-message" autocomplete="off">${PIEGE}
        <textarea id="am-texte-message" class="am-saisie" maxlength="${TEXTE_MAX}" required style="width:100%"></textarea>
        <div class="am-compteur" data-compteur="am-texte-message">0 / ${TEXTE_MAX}</div>
        <div class="am-ligne am-ligne-fin">
          <button class="am-btn" type="submit">[ Envoyer ]</button>
        </div>
        <div class="am-statut" id="am-statut-message"></div>
      </form>
    </div>`;
}

function ecranModeration() {
  return `
    <div class="am-card">
      <div class="am-lbl">// File de modération — du plus ancien au plus récent</div>
      <div id="am-file"><div class="am-vide">Chargement…</div></div>
    </div>`;
}

// Écran complet (structure + formulaires). Les données sont posées ensuite par
// rafraichir(), qui ne touche qu'aux conteneurs de données : une saisie en
// cours n'est jamais effacée par le rafraîchissement périodique.
function rendre() {
  const r = racine();
  if (!r) return;
  if (!etat.jeton || !etat.moi) {
    r.innerHTML = ecranEntree();
    return;
  }
  if (etat.moi.a_renommer) {
    r.innerHTML = ecranRenommer();
    return;
  }
  if (etat.vue === 'moderation' && !etat.moi.moderateur) etat.vue = 'sujets';
  const corps = {
    sujets: ecranSujets, nouveau: ecranNouveau, fil: ecranFil, moderation: ecranModeration,
  }[etat.vue]();
  r.innerHTML = barre() + corps + REGLES;
  rafraichir();
  planifier(); // le rythme suit la vue (2 s dans un fil)
}

// ── Données ──────────────────────────────────────────────────────────────────

function badgeStatut(statut, motif) {
  if (statut === 'attente') return '<span class="am-badge attente">en attente de modération</span>';
  if (statut === 'refuse')  return '<span class="am-badge refuse">refusé</span>';
  return '';
}
function motifHtml(statut, motif) {
  return statut === 'refuse' && motif
    ? `<div class="am-motif-txt">Motif du refus : ${esc(MOTIFS[motif] || motif)} — vous seul voyez ce message.</div>`
    : '';
}

async function chargerListe() {
  const el = $('am-liste');
  if (!el) return;
  const { sujets } = await api('GET', '/sujets');
  if (!$('am-liste')) return; // vue changée pendant la requête
  el.innerHTML = sujets.length ? sujets.map(s => `
    <button class="am-sujet" data-a="ouvrir" data-id="${s.id}">
      <div class="am-sujet-titre"><span class="trad-skip">${esc(s.titre)}</span>${s.a_moi ? badgeStatut(s.statut) : ''}</div>
      <div class="am-meta">${auteurHtml(s.auteur)} · ${s.nb_messages} réponse${s.nb_messages > 1 ? 's' : ''} · dernière activité <b>${quand(s.derniere)}</b>${s.statut === 'publie' ? ` · +1 <b>${s.plus1}</b> · ${resteAVivre(s.expire_le)}` : ''}</div>
      ${s.a_moi ? motifHtml(s.statut, s.motif) : ''}
    </button>`).join('')
    : '<div class="am-vide">Aucun sujet publié pour l\'instant. Ouvrez le premier.</div>';
}

async function chargerFil() {
  const el = $('am-fil');
  if (!el) return;
  const { sujet, messages } = await api('GET', `/sujets/${etat.sujetId}`);
  if (!$('am-fil')) return;
  etat.sujetPublie = sujet.statut === 'publie';
  el.innerHTML = `
    <div class="am-msg ${sujet.statut}">
      <div class="am-sujet-titre"><span class="trad-skip">${esc(sujet.titre)}</span>${sujet.a_moi ? badgeStatut(sujet.statut) : ''}</div>
      <div class="am-meta">${auteurHtml(sujet.auteur)} · ${quand(sujet.cree_le)}</div>
      <div class="am-msg-texte trad-skip">${esc(sujet.texte)}</div>
      ${sujet.a_moi ? motifHtml(sujet.statut, sujet.motif) : ''}
      <div class="am-actions">${plus1Html('sujet', sujet)}</div>
    </div>
    ${messages.map(m => `
    <div class="am-msg ${m.statut}">
      <div class="am-meta">${auteurHtml(m.auteur)} · ${quand(m.cree_le)}${m.a_moi ? badgeStatut(m.statut) : ''}</div>
      <div class="am-msg-texte trad-skip">${esc(m.texte)}</div>
      ${m.a_moi ? motifHtml(m.statut, m.motif) : ''}
      <div class="am-actions">${plus1Html('message', m)}</div>
    </div>`).join('')}
    ${!etat.sujetPublie ? '<div class="am-aide" style="margin-top:0.6rem">On pourra répondre une fois le sujet publié.</div>' : ''}`;
  const rep = $('am-repondre');
  if (rep) rep.hidden = !etat.sujetPublie;
}

async function chargerFile() {
  const el = $('am-file');
  if (!el) return;
  // Un modérateur en train de choisir un motif ne doit pas voir la liste se
  // reconstruire sous son curseur : on attend qu'il ait quitté la file.
  if (el.contains(document.activeElement) && document.activeElement.tagName === 'SELECT') return;
  const { file } = await api('GET', '/moderation');
  if (!$('am-file')) return;
  el.innerHTML = file.length ? file.map(f => {
    const cle = `${f.objet}:${f.id}`;
    const motif = etat.motifs[cle] || '';
    return `
    <div class="am-msg" data-objet="${f.objet}" data-id="${f.id}">
      <div class="am-meta">
        <span class="am-badge objet" style="margin-left:0">${f.objet === 'sujet' ? 'nouveau sujet' : 'message'}</span>
        ${f.objet === 'message' ? `dans « <span class="trad-skip">${esc(f.sujet_titre)}</span> »` : ''}
        · ${auteurHtml(f.auteur)} · ${quand(f.cree_le)}
      </div>
      ${f.titre ? `<div class="am-sujet-titre trad-skip" style="margin-top:0.35rem">${esc(f.titre)}</div>` : ''}
      <div class="am-msg-texte trad-skip">${esc(f.texte)}</div>
      <div class="am-mod-actions">
        <button class="am-btn ok" data-a="publier">[ Publier ]</button>
        <select class="am-motif" data-a="motif" aria-label="Motif du refus">
          <option value="">— motif du refus —</option>
          ${Object.entries(MOTIFS).map(([k, l]) => `<option value="${k}"${motif === k ? ' selected' : ''}>${l}</option>`).join('')}
        </select>
        <button class="am-btn ko" data-a="refuser"${motif ? '' : ' disabled'}>[ Refuser ]</button>
        ${f.auteur ? '<button class="am-lien" data-a="refuser-pseudo" style="margin-left:auto">refuser le pseudonyme…</button>' : ''}
      </div>
      <div class="am-statut" data-statut></div>
    </div>`;
  }).join('')
    : '<div class="am-vide">Rien à modérer. La taverne est calme.</div>';
}

function majCompteur() {
  const c = $('am-compte');
  if (!c) return;
  c.textContent = etat.aModerer;
  c.hidden = !etat.aModerer;
}

async function rafraichir() {
  if (!etat.jeton) return;
  try {
    if (etat.vue === 'sujets') await chargerListe();
    else if (etat.vue === 'fil') await chargerFil();
    else if (etat.vue === 'moderation') await chargerFile();
    if (etat.moi) {
      const r = await api('GET', '/moi');
      const avant = etat.moi;
      etat.moi = r.moi;
      // Pseudonyme refusé entre-temps : on bascule sur l'écran de renommage.
      if (r.moi.a_renommer !== avant.a_renommer) { signaler(r.a_moderer); return rendre(); }
      signaler(r.a_moderer);
    }
  } catch (e) {
    if (e.code === 404 && etat.vue === 'fil') { etat.vue = 'sujets'; rendre(); }
    else console.warn('[amphipoolis]', e.message);
  }
}

// ── Actions ──────────────────────────────────────────────────────────────────

// Le choix du rôle reconstruit le formulaire : on y reporte la saisie en cours.
function rendreEntreeEnGardant() {
  const garde = ['am-pseudo', 'am-phrase', 'am-phrase2'].map(id => [id, $(id)?.value ?? '']);
  const compris = $('am-compris')?.checked;
  rendre();
  garde.forEach(([id, v]) => { if ($(id)) $(id).value = v; });
  if ($('am-compris')) $('am-compris').checked = !!compris;
}

async function soumettreEntree(ev) {
  ev.preventDefault();
  const creer = etat.ongletEntree === 'creer';
  const pseudo = $('am-pseudo').value.trim();
  const phrase = $('am-phrase').value;
  if (creer && phrase !== $('am-phrase2').value) {
    return statut('am-statut-entree', 'Les deux phrases secrètes diffèrent.', 'err');
  }
  const btn = $('am-btn-entree');
  btn.disabled = true;
  try {
    let r;
    if (creer) {
      statut('am-statut-entree', 'Vérification anti-robot…');
      const altcha = await prendrePreuve();
      statut('am-statut-entree', 'Création du pseudonyme…');
      r = await api('POST', '/creer', { pseudo, phrase, altcha, moderateur: etat.roleCreation, site: piege(ev.target) });
    } else {
      statut('am-statut-entree', 'Vérification…');
      r = await api('POST', '/entrer', { pseudo, phrase });
    }
    etat.jeton = r.jeton;
    etat.moi = r.moi;
    etat.vue = 'sujets';
    ecrireJeton(r.jeton);
    rendre();
    demarrerVeille();
  } catch (e) {
    statut('am-statut-entree', e.message, 'err');
    btn.disabled = false;
  }
}

async function soumettreRenommer(ev) {
  ev.preventDefault();
  try {
    const r = await api('POST', '/renommer', { nouveau: $('am-nouveau').value.trim() });
    etat.moi = r.moi;
    rendre();
  } catch (e) {
    statut('am-statut-renommer', e.message, 'err');
  }
}

async function soumettreSujet(ev) {
  ev.preventDefault();
  const btn = ev.target.querySelector('button[type=submit]');
  btn.disabled = true;
  try {
    statut('am-statut-sujet', 'Vérification anti-robot…');
    const altcha = await prendrePreuve();
    await api('POST', '/sujets', { titre: $('am-titre').value, texte: $('am-texte-sujet').value, altcha, site: piege(ev.target) });
    etat.vue = 'sujets';
    rendre();
  } catch (e) {
    statut('am-statut-sujet', e.message, 'err');
    btn.disabled = false;
  }
}

async function soumettreMessage(ev) {
  ev.preventDefault();
  const zone = $('am-texte-message');
  const btn = ev.target.querySelector('button[type=submit]');
  btn.disabled = true;
  try {
    statut('am-statut-message', 'Vérification anti-robot…');
    const altcha = await prendrePreuve();
    await api('POST', `/sujets/${etat.sujetId}/messages`, { texte: zone.value, altcha, site: piege(ev.target) });
    btn.disabled = false;
    zone.value = '';
    majCompteurs();
    statut('am-statut-message', 'Déposé : visible des autres après validation par un modérateur.', 'ok');
    await chargerFil();
  } catch (e) {
    statut('am-statut-message', e.message, 'err');
    btn.disabled = false;
  }
}

async function moderer(carte, action) {
  const objet = carte.dataset.objet;
  const id = Number(carte.dataset.id);
  const st = carte.querySelector('[data-statut]');
  const cle = `${objet}:${id}`;
  try {
    if (action === 'refuser-pseudo') {
      // Confirmation dans la carte, sans boîte de dialogue du navigateur.
      if (carte.dataset.confirme !== '1') {
        carte.dataset.confirme = '1';
        st.className = 'am-statut err';
        st.textContent = 'Refuser ce pseudonyme ? Il disparaîtra partout et ne pourra plus être repris. Cliquez à nouveau pour confirmer.';
        return;
      }
      await api('POST', '/moderation/pseudo', { objet, id });
      st.className = 'am-statut ok';
      st.textContent = 'Pseudonyme refusé : son titulaire devra en choisir un autre. Le texte reste à trancher.';
      return;
    }
    const corps = { objet, id, decision: action };
    if (action === 'refuser') corps.motif = etat.motifs[cle];
    await api('POST', '/moderation/decision', corps);
    delete etat.motifs[cle];
    signaler(Math.max(0, etat.aModerer - 1));
    carte.remove();
    if (!$('am-file')?.children.length) await chargerFile();
  } catch (e) {
    st.className = 'am-statut err';
    st.textContent = e.message;
    // Tranché par un autre modérateur : la carte n'a plus lieu d'être.
    if (e.code === 409) setTimeout(() => { carte.remove(); }, 1500);
  }
}

function majCompteurs() {
  document.querySelectorAll('#am-racine [data-compteur]').forEach(c => {
    const zone = $(c.dataset.compteur);
    if (!zone) return;
    const n = [...zone.value].length;
    c.textContent = `${n} / ${TEXTE_MAX}`;
    c.classList.toggle('trop', n > TEXTE_MAX);
  });
}

async function donnerPlus1(b) {
  b.disabled = true;
  try {
    const r = await api('POST', '/plus1', { objet: b.dataset.objet, id: Number(b.dataset.id) });
    b.classList.add('donne');
    b.querySelector('b').textContent = r.plus1;
    b.title = 'Vous avez déjà donné un +1';
    const vie = b.nextElementSibling;
    if (vie) { vie.textContent = resteAVivre(r.expire_le); vie.title = `Disparaît le ${quand(r.expire_le)}`; }
  } catch (e) {
    b.title = e.message;
  }
}

async function sortir() {
  try { await api('POST', '/sortir'); } catch { /* session déjà morte : rien à fermer */ }
  etat.jeton = null; etat.moi = null; etat.vue = 'sujets';
  ecrireJeton(null);
  arreterVeille();
  rendre();
}

function brancherEcouteurs() {
  if (etat.ecouteur) return;
  const r = racine();
  if (!r) return;
  etat.ecouteur = true;

  r.addEventListener('click', ev => {
    const b = ev.target.closest('[data-a]');
    if (!b || !r.contains(b) || b.tagName === 'SELECT') return;
    const a = b.dataset.a;
    if (a === 'onglet-entree') { etat.ongletEntree = b.dataset.v; rendre(); }
    else if (a === 'vue')      { etat.vue = b.dataset.v; rendre(); }
    else if (a === 'ouvrir')   { etat.sujetId = Number(b.dataset.id); etat.vue = 'fil'; rendre(); }
    else if (a === 'role')     { etat.roleCreation = b.dataset.v === '1'; rendreEntreeEnGardant(); }
    else if (a === 'plus1')    { donnerPlus1(b); }
    else if (a === 'sortir')   { sortir(); }
    else if (a === 'son')      { ecrireSon(!sonActif()); b.textContent = `son : ${sonActif() ? 'oui' : 'non'}`; if (sonActif()) bip(); }
    else if (['publier', 'refuser', 'refuser-pseudo'].includes(a)) {
      const carte = b.closest('[data-objet]');
      if (carte) moderer(carte, a);
    }
  });

  r.addEventListener('change', ev => {
    const sel = ev.target.closest('select[data-a="motif"]');
    if (!sel) return;
    const carte = sel.closest('[data-objet]');
    etat.motifs[`${carte.dataset.objet}:${carte.dataset.id}`] = sel.value;
    carte.querySelector('[data-a="refuser"]').disabled = !sel.value;
  });

  r.addEventListener('input', majCompteurs);

  // Une saisie commence dans un formulaire de dépôt : on prépare la preuve.
  r.addEventListener('focusin', ev => {
    const f = ev.target.closest('form');
    if (!f || f.id === 'am-form-renommer') return;
    if (f.id === 'am-form-entree' && etat.ongletEntree !== 'creer') return;
    preparerPreuve().catch(() => { /* réessayé à l'envoi */ });
  });

  r.addEventListener('submit', ev => {
    const id = ev.target.id;
    if (id === 'am-form-entree')   soumettreEntree(ev);
    if (id === 'am-form-renommer') soumettreRenommer(ev);
    if (id === 'am-form-sujet')    soumettreSujet(ev);
    if (id === 'am-form-message')  soumettreMessage(ev);
  });
}

// ── Cycle de vie de la vue ───────────────────────────────────────────────────

// Boucle de rafraîchissement de la vue : 2 s dans un fil, 5 s ailleurs, en
// pause quand l'onglet est caché (la veille des modérateurs prend le relais).
function planifier() {
  clearTimeout(etat.minuteur);
  if (!etat.actif) return;
  etat.minuteur = setTimeout(async () => {
    if (document.visibilityState === 'visible') await rafraichir();
    planifier();
  }, etat.vue === 'fil' ? RAFRAICHIR_FIL_MS : RAFRAICHIR_MS);
}

async function veiller() {
  if (!etat.jeton) return arreterVeille();
  // Vue affichée et onglet visible : rafraichir() s'en charge déjà.
  if (etat.actif && document.visibilityState === 'visible') return;
  try {
    const r = await api('GET', '/moi');
    etat.moi = r.moi;
    if (!r.moi.moderateur) return arreterVeille();
    signaler(r.a_moderer);
  } catch { /* 401 : api() a déjà tout remis à zéro */ }
}

function demarrerVeille() {
  if (!etat.jeton || !etat.moi?.moderateur || etat.veille) return;
  etat.veille = setInterval(veiller, VEILLE_MS);
  signaler(etat.aModerer);
}

function arreterVeille() {
  clearInterval(etat.veille);
  etat.veille = null;
  document.title = TITRE_BASE;
}

/// Au chargement du site : un modérateur déjà connecté est prévenu de la file
/// d'attente même s'il n'ouvre pas Amphipoolis.
export async function amphVeille() {
  etat.jeton = etat.jeton || lireJeton();
  if (!etat.jeton) return;
  try {
    const r = await api('GET', '/moi');
    etat.moi = r.moi;
    etat.aModerer = 0;
    demarrerVeille();
    signaler(r.a_moderer);
  } catch { /* session expirée : rien à surveiller */ }
}

export async function amphInit() {
  brancherEcouteurs();
  etat.actif = true;
  etat.jeton = etat.jeton || lireJeton();
  if (etat.jeton && !etat.moi) {
    try {
      const r = await api('GET', '/moi');
      etat.moi = r.moi;
      etat.aModerer = r.a_moderer;
    } catch {
      etat.jeton = null; ecrireJeton(null);
    }
  }
  rendre();
  demarrerVeille();
  planifier();
}

export function amphQuitter() {
  etat.actif = false;
  clearTimeout(etat.minuteur);
  etat.minuteur = null;
}
