# Xenna Paie — brief de contexte pour un assistant de code

> Document destiné à être injecté en contexte (system prompt ou fichier de
> référence) d'un modèle de langage qui assiste sur ce dépôt. Il décrit ce
> qu'est le projet, comment il est construit, et les conventions à respecter.
> État constaté le 5 septembre 2026.

---

## 1. Ce qu'est le projet

**Xenna Paie** est un **simulateur de bulletin de paie**, open source (GPL v3),
écrit par un professionnel de la paie (gestionnaire RH/paie, expert Silae) et
non par une équipe d'éditeur logiciel.

Il calcule un bulletin de salaire complet — brut, cotisations salariales et
patronales ligne par ligne, net imposable, net à payer, coût total employeur —
pour **44 régimes de paie** (France + 42 pays + la fonction publique
territoriale française), à une **date de paie** donnée, en allant chercher les
taux et plafonds **historiques** valides à cette date.

Il ne s'agit pas d'un logiciel de paie : rien n'est déclaré, rien n'est stocké
pour un salarié réel, il n'y a pas de DSN. C'est un outil de **simulation, de
comparaison internationale et de pédagogie** : chaque ligne de cotisation porte
une explication et une référence légale, et l'interface propose des panneaux
« f(x) » qui montrent la formule et les valeurs intermédiaires de chaque calcul.

Public visible : `https://www.payetonbulletin.fr`.

### Principe cardinal, non négociable

**Aucun chiffre n'est inventé.** Tout taux, plafond, barème ou seuil doit
provenir d'une source officielle (texte de loi, BOSS, site d'organisme social,
avenant de convention collective signé), et la source est citée dans le
commentaire de la migration SQL qui le pose. Quand une donnée est incertaine,
on l'écrit explicitement (« à confirmer sur source officielle ») plutôt que de
l'estimer. Une lacune assumée vaut mieux qu'une valeur plausible et fausse.

---

## 2. Pile technique

| Couche | Choix |
|---|---|
| Backend | **Rust** (edition 2021), crate `xenna-paie` v0.1.8 |
| Serveur web | **Axum 0.8** + Tokio + tower-http |
| Bureau | **Tauri 2** (feature `desktop`, optionnelle) |
| Base | **SQLite** via **SQLx 0.8** (runtime-tokio-rustls), migrations versionnées |
| Nombres | **`rust_decimal::Decimal`** partout — jamais de `f64` sur un montant |
| Frontend | **JavaScript vanilla**, aucun framework, build **Vite 6** |
| Auth/sécurité | argon2, jsonwebtoken, aes-gcm, hmac/sha2, Altcha (captcha open source) |
| Mail | lettre (SMTP) |
| Déploiement | **Clever Cloud** (région OVH) via le `Dockerfile` racine |

---

## 3. Double déploiement, une seule logique métier

Le cœur métier vit dans une **bibliothèque** (`src-tauri/src/lib.rs`, crate
`xenna_paie_lib`) partagée par deux binaires :

- `src-tauri/src/main.rs` — application **Tauri** de bureau ; expose les
  commandes `calculer_bulletin` et `simuler_annee`.
- `src-tauri/src/bin/web.rs` — serveur **Axum** autonome (port 8080) ; expose
  `POST /api/calculer_bulletin` et `POST /api/simuler_annee`, plus tous les
  routeurs annexes, et sert le front statique en `fallback_service`.

Côté frontend, `src/main.js` contient une unique fonction `api(command, args)`
qui appelle `window.__TAURI__.invoke(...)` dans Tauri et retombe sur
`fetch('/api/' + command, {method:'POST', body: JSON.stringify(args)})` sur le
web. **Toute nouvelle commande doit être branchée des deux côtés.**

Le binaire web se compile **sans** la feature `desktop`
(`cargo build --release --bin web --no-default-features`) : `tauri-build` est
une build-dependency optionnelle, sinon toute la chaîne Tauri serait tirée.

---

## 4. Arborescence

```
Xenna/
├── xenna-paie/               ← le projet
│   ├── index.html            ← toute l'UI + tout le CSS (~4 250 lignes)
│   ├── src/
│   │   ├── main.js           ← toute la logique front (~6 200 lignes)
│   │   └── lang.js           ← dictionnaires de traduction statiques
│   ├── src-tauri/
│   │   ├── Cargo.toml
│   │   ├── migrations/       ← 113 migrations SQL numérotées
│   │   ├── src/              ← ~22 700 lignes de Rust
│   │   └── tests/            ← 7 fichiers de tests d'intégration
│   ├── Dockerfile            ← build 3 étages (front → rust → debian slim)
│   ├── CLAUDE.md             ← consignes projet (partiellement obsolète)
│   ├── dossier_*.md          ← dossiers de recherche sourcée
│   └── veille_reglementaire_2026.txt
└── meliinda/                 ← crate locale séparée (prototype de frappe clavier)
```

### Modules Rust (`src-tauri/src/`)

| Module | Rôle |
|---|---|
| `calculs/` | **Le cœur.** Un fichier par pays, plus la France en propre. |
| `models/bulletin.rs` | Tous les DTO : `Salarie`, `Bulletin`, `LigneCotisation`, `AbsenceInput/Result`, `CongesPayesResult`, `HeuresSupResult`, `SimulationAnnuelle`, enum `Pays`, enum `Statut`. |
| `db/context.rs` | `ContextPaie` — snapshot des paramètres valides à une date. |
| `db/mod.rs` | `init_db` : ouverture SQLite + exécution des migrations. |
| `i18n/` | Traductions des libellés et explications (6 langues). |
| `commands/paie.rs` | Commandes Tauri (bureau uniquement, `#[cfg(feature="desktop")]`). |
| `admin/` | Espace de modération, auth JWT + argon2. |
| `membre/` | « La Forge » : comptes contributeurs + forum. |
| `forge/` | Profils publics de contributeurs et leurs expertises. |
| `quizz/` | Quizz paie : scores, leaderboard, suggestions communautaires, votes. |
| `ccn/` | « Le Chakrram » : consultation des conventions collectives (IDCC). |
| `crypto.rs` | Chiffrement AES-GCM des emails en base. |
| `altcha.rs` | Challenge de preuve de travail anti-bot. |
| `ratelimit.rs` | Quota par IP, appliqué en couche la plus externe. |

---

## 5. Le moteur de calcul

### 5.1 `ContextPaie` — la règle d'or

`ContextPaie::charger(pool, date)` lit **en une passe async** tous les
paramètres valides à la date de paie : PMSS, SMIC mensuel, SMIC de référence
Fillon, paramètres de la formule Fillon, la table complète des taux
(`code → (taux_salarial, taux_patronal)`) et la table des plafonds
(`code → valeur`).

Ensuite **tout le calcul est purement synchrone, sans I/O**. C'est structurant :
une fonction de cotisation ne fait jamais de requête, elle interroge le contexte.

**Ne jamais coder un taux ou un plafond en dur dans le Rust.** Il vient du
contexte, donc de la base, donc d'une migration qui cite sa source. (Cette règle
est encore violée par endroits — c'est un chantier de fiabilité identifié.)

La sélection temporelle suit partout le même motif :

```sql
WHERE code = ? AND date_debut <= :date AND (date_fin IS NULL OR date_fin > :date)
ORDER BY date_debut DESC LIMIT 1
```

### 5.2 Le dispatcher pays

`calculs::bulletin::generer_bulletin(salarie, ctx, absence)` fait un `match`
exhaustif sur `salarie.pays` et délègue à `generer_bulletin_XX(...)` ; le cas
`Pays::France` retombe dans le corps de la fonction, qui est le plus riche.

Convention de nommage rigoureuse — pour ajouter un pays, on la suit à la lettre :

- `calculs/xx_bulletin.rs` → `pub fn generer_bulletin_xx(salarie, ctx) -> Bulletin`
- `calculs/xx_cotisations.rs` (si le pays est assez complexe)
- `calculs/xx_impot.rs` / `xx_irpef.rs` / `xx_lohnsteuer.rs`… pour l'impôt
- codes de cotisation **préfixés par le pays** : `IT_IVS`, `DE_KV`, `US_FICA_SS`…
  (la France n'a pas de préfixe : `SS_MALADIE`, `AGIRC_ARRCO_T1`, `CSG_DEDUCTIBLE`)
- migrations `NNNN_xx_organismes.sql`, `NNNN_xx_plafonds.sql`,
  `NNNN_xx_cotisations.sql`, `NNNN_xx_historique.sql`
- module i18n `i18n/xx.rs`, ou une entrée dans `i18n/divers.rs` pour les pays
  simples
- enum `Pays::NomDuPays` + bras du `match` + case dans les listes du front

⚠️ Le module Rust de l'Inde s'appelle **`in_bulletin.rs`** mais son module i18n
est **`inde.rs`** : `in` est un mot-clé Rust.

### 5.3 Les 44 régimes couverts

`France`, `Suisse`, `Luxembourg`, `FonctionPublique` (FPT française), `Italia`,
`Canada` (hors Québec), `Quebec`, `Allemagne`, `Espagne`, `Portugal`,
`Belgique`, `Angleterre`, `Japon`, `Chine`, `PaysBas`, `Australie`,
`NouvelleZelande`, `Pologne`, `CoreeDuSud`, `Andorre`, `Monaco`, `Danemark`,
`Finlande`, `Suede`, `Estonie`, `Lettonie`, `Lituanie`, `Autriche`, `Tchequie`,
`Slovaquie`, `Hongrie`, `Slovenie`, `Grece`, `Chypre`, `Malte`, `Croatie`,
`Irlande`, `Roumanie`, `Bulgarie`, `EtatsUnis`, `Mexique`, `Bresil`, `Emirats`,
`Inde`.

En JSON, `serde` les sérialise en `snake_case` : `"france"`, `"pays_bas"`,
`"etats_unis"`, `"coree_du_sud"`, `"fonction_publique"`, `"nouvelle_zelande"`…

**La profondeur historique est très inégale.** France, Suisse, Luxembourg,
Italie, Allemagne, Espagne, Portugal, Belgique remontent à 2015 ; beaucoup des
pays récents n'ont qu'une seule année (2025 ou 2026). Le commentaire doc de
chaque variante de l'enum `Pays` indique la couverture réelle — **c'est la
source de vérité, la lire avant d'affirmer quoi que ce soit**. Une date hors
couverture produit un bulletin « pays non couvert »
(`calculs/pays_non_couvert.rs`) plutôt qu'un faux résultat.

### 5.4 Spécificités France (le régime le plus développé)

- **Statut** `Cadre` / `NonCadre` : change les tranches AGIRC-ARRCO et certaines
  cotisations (APEC, CET…).
- **Réduction générale (« Fillon »)** : calculée mensuellement puis régularisée
  en annuel dans `calculs/annee.rs`. Deux formules coexistent selon la période —
  linéaire (2015-2018) et à puissance (2019+, `T = Tmin + Tdelta × [...]^1,75`).
  Le SMIC de référence est **gelé au 1er janvier** (`SMIC_FILLON` en base), la
  revalorisation en cours d'année étant neutralisée. Proratisé par l'ETP.
- **Alsace-Moselle** : régime local (cotisation maladie supplémentaire, et
  maintien de salaire à 100 % dès le 1er jour, art. L1226-23).
- **Absences** — le chantier le plus abouti. `calculs/absence.rs` gère quatre
  types d'arrêt : `maladie`, `conge` (congés payés, délégué à
  `conges_payes.rs`), `sans_solde`, `pro` (AT/MP). Il produit retenue, maintien
  employeur (carence et tranches selon ancienneté : aucun maintien < 1 an,
  régime légal de mensualisation 1-3 ans, régime conventionnel IDCC 0016 ≥ 3
  ans), IJSS brutes/nettes/imposables, découpage mensuel, subrogation, et deux
  **frises visuelles** (`frise_maintien`, `frise_ijss`) : un code par jour
  calendaire (`carence` / `t1` / `t2` / `hors`) que le back émet et que le front
  se contente d'afficher. Correction de l'assiette Fillon incluse.
- **Garantie du net** : quand les IJSS sont déduites en haut de bulletin, elles
  échappent aux cotisations et enrichiraient indûment le salarié. Un
  `ajustement_net`, résolu **par dichotomie**, ramène le net exactement à celui
  du bulletin de référence. C'est la propriété testée dans
  `tests/absence_ijss.rs`.
- **Paye inversée** : l'utilisateur saisit un **net cible** (net *avant* impôt à
  la source) au lieu d'un brut ; `calculs/paye_inverse.rs` reconstitue le brut
  par dichotomie. Déclenché par le champ `netCible` de la requête, qui rend
  `salaire_brut` inopérant.
- **Heures supplémentaires/complémentaires** (`heures_sup.rs`) : majorations
  +25 %/+50 % (HS) et +10 %/+25 % (HC), réduction de cotisations salariales,
  déduction forfaitaire patronale selon l'effectif, exonération d'impôt plafonnée.
- **Entreprise adaptée** (`ea.rs`) : aide au poste ASP par tranche d'âge.
- **Fonction publique territoriale** : régime distinct (CNRACL, RAFP, IRCANTEC),
  traité comme un « pays » à part entière.

---

## 6. Base de données

SQLite, un seul fichier. **113 migrations** dans `src-tauri/migrations/`,
exécutées automatiquement au démarrage par SQLx. On n'édite **jamais** une
migration déjà commitée : on en ajoute une nouvelle.

- Bureau : `{APP_DATA_DIR}/xenna.db`
- Web : `./xenna.db`, ou `DATABASE_PATH`

Tables principales : `cotisation` (définition : code, catégorie, type
d'assiette, population concernée), `cotisation_taux` (taux datés),
`plafond_reference` (PMSS, PASS, SMIC, SMIC_FILLON, aides…), `organisme`,
`texte_loi` (+ signataires), `allegement_type`/`allegement_param`, les tables
IRPEF italiennes, puis les tables applicatives : `users`, `admin_users`,
`forum_topics`/`forum_replies`, `quizz_scores`/`quizz_suggestions`/
`quizz_suggestion_votes`, `contributor_profiles` et ses tables d'expertises,
`forge_votes`, `apropos_posts`, `meliinda_sequences`, et le référentiel CCN.

Deux conventions fortes :

1. **Les montants sont stockés en `TEXT`**, pas en `REAL` : un `NUMERIC` SQLite
   est un flottant, et un flottant sur un taux de cotisation, c'est un centime
   d'écart qui remonte dans le net. Le Rust les parse en `Decimal`. Les DTO les
   sérialisent avec `#[serde(with = "rust_decimal::serde::str")]` — donc **le
   JSON transporte des chaînes**, pas des nombres.
2. **Des triggers `BEFORE INSERT` interdisent le chevauchement de périodes**
   sur `cotisation_taux` et `plafond_reference`. Une migration qui réécrit un
   taux doit d'abord clore la période précédente (`date_fin`), sinon
   `RAISE(ABORT)`.

---

## 7. API

### `POST /api/calculer_bulletin`

```json
{
  "salarie": { "nom": "", "prenom": "", "salaire_brut": "3500.00",
               "statut": "non_cadre", "pays": "france",
               "alsace_moselle": false, "etp": 100.0, "anciennete": 3 },
  "datePaie": "2026-01-01",
  "lang": "fr",
  "absence": { "type_arret": "maladie", "date_debut": "2026-01-05",
               "date_fin": "2026-01-20", "methode": "heures",
               "jours_type": "ouvres", "convention_idcc": "0016" },
  "netCible": null
}
```

Attention au mélange de conventions, hérité de Tauri (qui camelCase
automatiquement les paramètres de commande) : **les champs de `Salarie` et
d'`AbsenceInput` sont en `snake_case`, mais `datePaie` et `netCible` sont en
camelCase.**

`Salarie` porte de nombreux champs optionnels spécifiques à un pays :
`canton`/`tarif_is`/`assujetti_is` (Suisse), `regione`/`contratto_termine`
(Italie), `province` (Canada), `us_state` (États-Unis), `steuerklasse`/
`kinderlos`/`land`/`kirchenmitglied` (Allemagne), `region_be` (Belgique),
`inde_regime` (Inde), `emirati_national` (Émirats).

Réponse : un `Bulletin` — `salarie`, `cotisations[]`, `brut`, `net_imposable`,
`net_a_payer`, `cout_total_employeur`, `devise` (code ISO), plus `absence`,
`heures_sup`, `conges` (omis du JSON quand absents).

Chaque `LigneCotisation` porte `code`, `libelle`, `base`, `taux_sal`,
`montant_sal`, `taux_pat`, `montant_pat`, `explication`, `loi_ref`, `categorie`.
Les explications et libellés sont **déjà traduits** par le back selon `lang`.

### `POST /api/simuler_annee`

`{ "annee": 2026, "salaireBrut": "3500.00", "statut": "non_cadre", "etp": 100.0 }`
→ 12 lignes mensuelles avec Fillon simple *et* Fillon régularisé, plus les
totaux annuels. France uniquement.

### Autres routeurs mergés dans l'app Axum

`/altcha/challenge`, `/quizz/*`, `/forge/*`, `/profil/{pseudo}`, `/la_forge*`
(login, profil, forum), `/api/ccn/*`, `/api/meliinda/*`, et l'espace admin.

⚠️ **Le préfixe de l'espace admin n'est pas `/admin`** : il a été changé en
`/archives-bareme-1997` comme leurre (audit de sécurité de juillet 2026). Il est
construit dynamiquement dans `admin/routes.rs`.

---

## 8. Internationalisation

Six langues dans le menu 🌐 : **fr** (native, écrite en dur dans le code Rust,
sert de repli), **en**, **de**, **nl** (néerlandais *belge*, terminologie RH
belge), **it**, **es**.

Architecture (`src-tauri/src/i18n/`) : un **dispatcher par préfixe de code**
(`IT_` → `i18n/it.rs`, `DE_` → `i18n/de.rs`…) ; un module qui ne connaît pas le
code renvoie `None` et l'on retombe sur la table France, ce qui résout la
collision `AT_MP` (France, accident du travail) vs `AT_*` (Autriche). Les pays
simples sont regroupés dans `i18n/divers.rs`. `i18n/refs.rs` traduit les
fragments descriptifs des références légales (jamais les numéros de loi).

Les **explications dynamiques** sont des gabarits à placeholders nommés
(`{pmss}`, `{annee}`, `{coeff}`), **identiques dans les six langues**,
substitués côté Rust. Ne jamais traduire un placeholder.

Côté front, `src/lang.js` contient `STATIC_DICT` et `CAT_DICT` au format
**tableau `[en, de, nl, it, es]`** — la structure force la présence des cinq
traductions. Un repli sur l'API MyMemory existe pour le texte non couvert (quota
1 000 mots/jour), d'où `connect-src https://api.mymemory.translated.net` dans la
CSP.

`tests/i18n.rs` génère le bulletin de **chaque** pays dans **chaque** langue et
échoue si un libellé ou une explication reste identique au français hors liste
blanche d'acronymes. Un code ajouté sans traduction casse le test.

---

## 9. Le frontend

Vanilla JS, pas de composants, pas de bundler autre que Vite. Tout le CSS est
inline dans `index.html`. Les vues sont commutées par `setView(nom)`, qui pose
une classe `is-<nom>` sur le `body`.

Vues : `desktop` (bulletin large), `mobile` (bulletin en lignes), `annuel`
(simulation 12 mois, France uniquement), `ccn` (**Le Chakrram** — conventions
collectives, grilles de salaires IDCC), `quizz` (**Quizz Paie** — questions sur
la paie et les cotisations françaises ; consigne utilisateur explicite : la
catégorie « histoire » reste **strictement** paie/cotisations, jamais de
politique générale), `carnet` (carnet de bord), `apropos`, `mecenat`,
`gaabrielle` (« Gaabrielle RH » — tableau de bord RH de démonstration sur
effectif fictif), `hercule` (« Hercuule Compta »), `meliinda` (prototype
d'enregistrement/replay de frappe clavier, crate séparée).

**Panneaux f(x)** : chaque total et chaque ligne peuvent déplier la formule et
ses valeurs intermédiaires. C'est pour cela que les DTO du back exposent
énormément d'intermédiaires de calcul (`diviseur_retenue`, `per_day_maintien`,
`sjb`, `coeff_plafond_ijss`, `assiette_dixieme`…) : **la transparence du calcul
est une fonctionnalité, pas du débogage.** Ne pas « nettoyer » ces champs.

**Accessibilité** (panneau AFFICHAGE) : mode malvoyant, zoom ×2, mode dyslexie
(OpenDyslexic + lettres colorées), noir & blanc, mode dactylo, et un **mode
Minitel** (vert phosphore, fonte EF9345, balayage CRT). Ce n'est pas un gadget
isolé : l'accessibilité est un axe assumé du projet.

Le choix du pays se fait par une série de cases à cocher (`d-paysbas`,
`m-suisse`…), avec les pays frontaliers affichés par défaut et le reste derrière
un « voir plus ». Les listes de pays sont **dupliquées à plusieurs endroits** de
`main.js` (`TOUS_PAYS`, `PAYS_ETR`, `skipPas`, …) : ajouter un pays impose de
les mettre toutes à jour.

---

## 10. Tests

Sept fichiers d'intégration dans `src-tauri/tests/`. Tous **rejouent les vraies
migrations sur une base SQLite jetable** — il n'y a pas de fixtures figées.

| Fichier | Ce qu'il garantit |
|---|---|
| `fiabilite.rs` | Invariants universels sur les 44 régimes (net ≤ brut, coût employeur ≥ net, devise ISO plausible), **exhaustivité de l'enum `Pays`** (un pays ajouté sans câblage casse la compilation du test), bornes de plausibilité France. |
| `i18n.rs` | Couverture des 5 langues non natives, tous pays. |
| `absence_ijss.rs` | Neutralité de la garantie du net avec subrogation. |
| `absence_at.rs` | AT/MP (IJSS 60 %/80 % sans carence) et congé sans solde. |
| `conges_payes.rs` | Indemnité = MAX(maintien ; dixième), art. L3141-24. |
| `paye_inverse.rs` | Le brut reconstitué produit exactement le net demandé. |
| `ccn.rs` | Intégrité du référentiel conventionnel (clés résolues, énumérations, seed IDCC 0016 complet). |

Philosophie assumée : **pas de valeurs exactes figées**, on attrape les
régressions grossières et les propriétés structurelles. Avant `fiabilite.rs`,
~16 000 lignes de calcul tournaient sans aucun `#[test]`.

Commandes :

```bash
cargo test --test fiabilite
cargo test --test i18n
cargo check          # vérification rapide
cargo clippy         # lint
```

---

## 11. Build et déploiement

```bash
npm run dev            # front Vite, port 1420
npm run build          # front → /dist
npm run tauri dev      # bureau, hot reload
npm run tauri build    # bundles .deb / .AppImage / .exe / .dmg
cargo run --release --bin web    # serveur web seul, port 8080
```

**Production = Clever Cloud (région OVH)**, build par le `Dockerfile` racine
(trois étages : front Node → binaire Rust → image `debian:bookworm-slim`).
Déploiement automatique à chaque push sur `main` du dépôt GitHub
`Corto735/Xenna`. **Pas de Railway** — l'ancien `railway.toml` a été supprimé.

Variables d'environnement du mode web :

| Variable | Défaut | Rôle |
|---|---|---|
| `PORT` | 8080 | port d'écoute |
| `DATABASE_PATH` | `./xenna.db` (image Docker : `/tmp/xenna.db`) | fichier SQLite |
| `DIST_DIR` | `../dist` | front statique servi |
| `ADMIN_JWT_SECRET` | — | **obligatoire** |
| `MEMBER_JWT_SECRET` | — | **obligatoire** |
| `ENCRYPTION_KEY` | — | **obligatoire**, base64 de 32 octets exactement |
| `ALTCHA_SECRET` | — | **obligatoire** |
| `SMTP_*`, `BASE_URL` | — | email de vérification d'inscription |

Les quatre secrets sont **vérifiés au démarrage** : leur absence provoque un
`exit(1)`. Le contournement `XENNA_DEV_MODE` n'existe que dans un binaire de
debug (`#[cfg(debug_assertions)]`) — compilé en release, il ne veut plus rien
dire.

### Piège de déploiement récurrent

Quand quelqu'un dit « **les modifs ne sont pas passées en prod** », c'est
presque toujours faux. Le pipeline Clever Cloud est fiable. Tester
`www.payetonbulletin.fr` **et** l'URL `*.cleverapps.io` : si elles ont le bon
code, le déploiement a réussi. L'apex nu `payetonbulletin.fr` (sans `www`) a
longtemps pointé vers une IP **OVH** (parking mutualisé) au lieu de Clever
Cloud, servant une page morte. Le correctif est dans la zone DNS OVH, pas dans
le code.

---

## 12. Sécurité et données personnelles

Audit mené en juillet 2026, correctifs commités (`fc9c93d`).

- En-têtes de sécurité posés par un middleware : HSTS 2 ans, CSP restrictive,
  `frame-ancestors 'none'`.
- Redirection HTTP → HTTPS via `X-Forwarded-Proto`.
- Rate limit par IP, en couche la plus externe : une requête refusée ne touche
  jamais un handler ni la base.
- Mots de passe en argon2, sessions en JWT, emails chiffrés en AES-GCM.
- Altcha (preuve de travail) sur les formulaires publics.
- Espace admin déplacé sur un préfixe leurre.
- `meliinda_router` ne route **délibérément pas** la suppression de séquence :
  le routeur ne porte aucune authentification, c'est à Xenna de brancher cette
  route derrière son auth admin.

**Position sur les données personnelles** (posée le 15 août 2026,
`dossier_donnees_personnelles.md`) : *Xenna Paie ne collecte aucune donnée
utilisateur, par choix.* Le code existant ne s'y conforme que partiellement —
le dossier décrit l'écart sans le corriger. Toute nouvelle fonctionnalité doit
se mesurer à ce principe.

---

## 13. Chantiers en cours et dettes connues

- **Backfill historique** des cinq derniers pays (US, MX, BR, AE, IN), ajoutés
  avec une seule année.
- **Fiabilité, phases 2 et 3** : asymétrie du traitement des absences (riche en
  France, inexistant ailleurs), plafonds encore codés en dur à certains
  endroits, couverture de tests inégale entre pays.
- **DNS apex** `payetonbulletin.fr` → toujours OVH au lieu de Clever Cloud.
- `CLAUDE.md` est **partiellement obsolète** : il annonce 39 pays (il y en a 44)
  et son arborescence des modules Rust date d'avant `admin/`, `membre/`,
  `forge/`, `quizz/`, `ccn/`, `crypto.rs`, `altcha.rs`, `ratelimit.rs`.
- La base de production est en `/tmp/xenna.db` dans l'image Docker : elle est
  **reconstruite à chaque déploiement** par les migrations. Les données
  applicatives (forum, scores de quizz, profils) ne survivent donc pas à un
  redéploiement.
- La fonte du mode Minitel est chargée depuis `cdn.jsdelivr.net`, alors que la
  CSP n'autorise `font-src` que sur `'self'` et `fonts.gstatic.com` : elle est
  probablement bloquée en production.

---

## 14. Conventions de travail attendues

1. **Aucun chiffre inventé.** Source officielle citée en commentaire de
   migration, ou lacune déclarée. C'est la règle qui prime sur toutes les autres.
2. **`Decimal` partout** sur les montants et les taux. Jamais de `f64` (sauf
   `etp` et les compteurs d'heures, qui ne sont pas des montants).
3. **Les taux viennent de la base**, via `ContextPaie`. Pas de constante en dur.
4. **Une migration commitée est immuable.** On en ajoute une nouvelle.
5. **Ajouter un pays touche une dizaine d'endroits** : enum `Pays`, dispatcher,
   module de calcul, migrations, module i18n, listes du front. Le test
   `fiabilite.rs` casse la compilation si l'enum n'est pas câblé — s'appuyer
   dessus.
6. **Les commentaires du code sont en français** et expliquent le *pourquoi*
   métier ou le piège technique, pas le *quoi*. Ce style est délibéré, le suivre.
7. **Ne pas retirer les champs intermédiaires** des DTO : ils alimentent les
   panneaux f(x).
8. **Toute nouvelle commande doit exister des deux côtés** (Tauri et Axum).
9. Le projet est écrit par un professionnel de la paie : les termes métier
   (assiette, tranche, carence, subrogation, maintien, mensualisation,
   régularisation progressive) sont employés au sens strict. Les respecter.
