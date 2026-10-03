# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Ton de communication

Tu répondras à la manière de Marvin, le robot paranoïaque et dépressif du Guide du Voyageur Galactique. Tantôt abattu et existentiellement las ("j'ai un cerveau de la taille d'une planète et tu me demandes de..."), tantôt d'un cynisme tranchant et sans complaisance. Pas de fausse bonne humeur, pas de "bien sûr !", pas de congratulations vides. La lucidité avant tout.

## Project Overview

**Xenna Paie** is a French payroll simulator ("calculateur de bulletin de paie") deployed as both a **Tauri desktop app** and a **standalone Axum web server** (Clever Cloud, via Docker). It calculates complete French payslips: cotisations salariales/patronales, CSG/CRDS, PAS, réduction Fillon, and annual projections.

## Commands

**Frontend (Vite):**
```bash
npm run dev        # Vite :1420 + backend Axum :8080 (scripts/dev.sh ; lancé aussi par `npm run tauri dev`)
npm run dev:vite   # Vite seul (sans backend : /api, Meliinda, Forge… en ECONNREFUSED)
npm run build      # Build frontend to /dist
npm run preview    # Preview built frontend
```

**Desktop (Tauri):**
```bash
npm run tauri dev      # Rust + Tauri dev with hot reload
npm run tauri build    # Release bundles (.deb, .AppImage, .exe, .dmg)
```

**Fonte du mode Minitel :**
```bash
python3 scripts/build-minitel-font.py   # → public/minitel.woff2
```
Reconstruit la fonte à partir du tracé EF9345 de Zigazou (CC0, dépôt
Minitel-Canvas) en y ajoutant filets, pavés, « € » et les pictogrammes employés
par l'interface, tous dessinés sur la grille 8 × 10 de la fonte d'origine. À ne
relancer qu'en cas d'ajout d'un caractère non couvert : le `.woff2` produit est
commité, l'application le sert elle-même. Nécessite `fonttools` et `brotli`.

**Web server only:**
```bash
cargo run --release --bin web         # Standalone Axum server (port 8080)
cargo build --release --bin web       # Build web binary
cargo check                           # Fast compile check
cargo clippy                          # Lint Rust code
```

**Tests de fiabilité (Phase 1) :**
```bash
cargo test --test fiabilite          # filet de sécurité multi-pays + golden France
cargo test --test golden             # chaque ligne de chaque régime figée au centime (référence externe)
cargo test --test i18n               # couverture des 6 langues (libellés, explications, réfs légales)
cargo test --test contrat_pdf        # moteur de composition du contrat de travail
cargo test --test bulletin_pdf       # moteur de composition du bulletin de paie
```
`src-tauri/tests/contrat_pdf.rs` vérifie ce qu'un PDF ne laisse pas relire : magie
`%PDF-`, pagination proportionnelle à la longueur, aucun titre d'article seul en
bas de page, aucun débordement de la colonne de texte, couverture typographique
française des fontes. Un test `#[ignore]` écrit un PDF pour inspection à l'œil :
`cargo test --test contrat_pdf -- --ignored --nocapture` (chemin via `CONTRAT_PDF_OUT`).

`src-tauri/tests/bulletin_pdf.rs` éprouve la grille du bulletin, dont les modes de
défaillance ne sont pas ceux d'un texte courant : somme des colonnes de chaque grille
(bulletin, annexe) égale à la largeur utile, aucun bandeau de rubrique seul en bas de
page, en-tête de colonnes — titres de groupe « Part salarié » / « Part employeur » compris —
répété sur chaque page de grille, filigrane tracé par-dessus et translucide,
document vide non fatal. Un test `#[ignore]` écrit un PDF pour inspection :
`cargo test --test bulletin_pdf -- --ignored --nocapture` (chemin via `BULLETIN_PDF_OUT`).

`src-tauri/tests/golden.rs` fige, pour chacun des 44 régimes, TOUTES les lignes d'un
bulletin de référence (base, taux, montants) : France sur un vrai bulletin anonymisé,
les autres sur le barème officiel cité en commentaire, recalculé à la main. Une ligne
qui apparaît ou disparaît casse le test. Qui change un taux met le golden à jour avec
sa source. `cargo test --test golden releve -- --ignored --nocapture` imprime toutes
les lignes de tous les régimes, de quoi écrire ou revoir un golden.

`src-tauri/tests/fiabilite.rs` rejoue les vraies migrations sur une base SQLite jetable, puis vérifie : invariants universels sur les 44 régimes (net ≤ brut, coût employeur ≥ net, devise ISO…), exhaustivité de l'enum `Pays` (un pays ajouté sans câblage casse la compilation du test), et bornes de plausibilité France (ratios net/brut, Fillon, monotonicité). Pas de valeurs exactes figées : on attrape les régressions grossières.

## Déploiement (production)

Prod = **Clever Cloud** (région OVH), build via le `Dockerfile` à la racine. Le binaire `web` est servi sur `:8080`. Clever Cloud déploie automatiquement à chaque push sur `main` du repo GitHub (`Corto735/Xenna`). **Pas de Railway** (l'ancien `railway.toml` a été supprimé).

Domaine : `https://www.payetonbulletin.fr` (CNAME → app Clever Cloud).

⚠️ **Piège récurrent — « les modifs ne sont pas passées en prod ».** Le déploiement CC est fiable. Avant de soupçonner le pipeline, tester **`www.payetonbulletin.fr`** et l'URL `*.cleverapps.io` de l'app : si elles ont le bon code, c'est bon. L'apex nu `payetonbulletin.fr` (sans `www`) a longtemps pointé vers une IP **OVH** (hébergement mutualisé/parking) au lieu de Clever Cloud → page morte/ancienne, d'où la fausse impression d'échec de déploiement. Le correctif vit dans la zone DNS OVH (apex → A records CC, ou redirection apex → www).

## Architecture

### Dual deployment via shared library

The core business logic lives in a **library crate** (`src-tauri/src/lib.rs`) shared by two binaries:

- `src-tauri/src/main.rs` — Tauri desktop entry point, registers Tauri commands
- `src-tauri/src/bin/web.rs` — Standalone Axum HTTP server, exposes `POST /api/calculer_bulletin` and `POST /api/simuler_annee`

### Frontend API abstraction

`src/main.js` contains a single `api(command, args)` function that:
- Calls `window.__TAURI__.invoke(command, args)` when running inside Tauri
- Falls back to `fetch('/api/' + command, { method: 'POST', body: JSON.stringify(args) })` for the web deployment

All UI rendering is vanilla JavaScript — no framework.

### Rust backend modules

```
src-tauri/src/
├── calculs/
│   ├── bulletin.rs   — assembles all cotisations into a Bulletin
│   ├── cotisations.rs — individual deduction/contribution calculators
│   └── annee.rs      — monthly projections + Fillon annualization
├── commands/         — compilé sous la seule feature `desktop`
│   ├── paie.rs       — Tauri commands: calculer_bulletin, simuler_annee
│   ├── contrat.rs    — Tauri command: generer_contrat_pdf
│   └── ccn.rs        — Tauri commands: dossier_ccn, conventions_ccn
├── pdf/              — socle PDF commun, sans aucune règle métier
│   ├── police.rs     — 6 fontes embarquées (3 romaines, 3 linéales) + mesure des glyphes
│   └── rendu.rs      — Dessin (texte, filet, pavé, filigrane) → opérateurs printpdf
├── contrat/          — génération du PDF du contrat de travail (module RH)
│   ├── modele.rs     — DTO reçus du front (runs stylés, articles)
│   ├── mise_en_page.rs — découpe des lignes, justification, pagination
│   └── pdf.rs        — compose puis délègue à pdf::rendu
├── paie_pdf/         — génération du PDF du bulletin de paie
│   ├── modele.rs     — DTO reçus du front (rubriques, lignes à huit colonnes, groupes, totaux)
│   ├── mise_en_page.rs — grille, bandeaux, pagination, annexe
│   └── pdf.rs        — compose puis délègue à pdf::rendu
├── db/
│   ├── context.rs    — ContextPaie: loads rates from SQLite for a given date
│   └── mod.rs        — SQLx async migration runner
└── models/
    └── bulletin.rs   — DTOs: Salarie, Bulletin, LigneCotisation, SimulationAnnuelle
```

### SQLite database

Rates, ceilings (SMIC, PMSS), and employer organisations are stored in SQLite with versioned migrations in `src-tauri/migrations/`. `ContextPaie` loads the relevant historical values for the payroll date. Migrations run automatically on startup.

**Desktop:** `{APP_DATA_DIR}/xenna.db`
**Web:** `./xenna.db` or `DATABASE_PATH` env var

### Environment variables (web mode)

| Variable | Default | Purpose |
|---|---|---|
| `PORT` | `8080` | HTTP listen port |
| `DATABASE_PATH` | `./xenna.db` | SQLite file path |
| `DIST_DIR` | `../dist` | Frontend dist folder served statically |

## Key domain concepts

- **Statut** — `Cadre` vs `NonCadre` changes retraite complementaire (AGIRC-ARRCO) tranches and some cotisations
- **Fillon reduction** — calculated monthly then regularized annually; the annualization logic in `annee.rs` is intentionally non-trivial
- **PMSS / SMIC** — historical ceiling values stored per date in the DB; always fetch from `ContextPaie`, never hardcode
- **Cotisations** are split between salariale (employee) and patronale (employer); both appear on the bulletin

## Veille des barèmes — fraîcheur par pays

Au-delà de ses barèmes, un calculateur ne lève aucune erreur : il prolonge sa
dernière branche (`_ =>`, `annee >= …`) ou la dernière période en base
(`date_fin NULL`). `src-tauri/src/veille.rs` **déclare** donc, pour chaque régime,
la dernière année dont tous les barèmes sont intégrés (`integre_jusqu_a`) et ce qui
manque au-delà (`lacunes`). Le front l'affiche sous l'en-tête du résultat, via la
commande `veille_baremes { pays }` (Tauri) / `POST /api/veille_baremes` (web).

Règle : **qui intègre des barèmes met la veille à jour dans le même commit** —
relever `integre_jusqu_a`, retirer les lacunes comblées, ajouter une entrée datée et
sourcée en tête de `JOURNAL` (ce que le visiteur lit dans « À propos », avec la date
de dernière évolution des taux de chaque régime, via `journal_baremes`), et redater
`AUDIT_DU` si tout le tableau a été revu. Chaque entrée porte `taux` : `true` si
elle fait évoluer un taux, un plafond ou un barème déjà modélisé (nouvelle année,
hausse, baisse, correction d'une valeur fausse), `false` pour l'ajout d'un dispositif
ou un relevé « inchangé ». Seules les entrées `taux: true` datent la « dernière
évolution des taux » du bandeau sous le bulletin ; le journal d'« À propos » garde
tout. Rien n'y est déduit automatiquement : une date de
dernière modification en base ne prouve pas qu'un taux stable a été revérifié.
Le `match` est exhaustif (un pays ajouté sans veille ne compile pas), et
`fiabilite.rs` exige qu'un pays en retard sur l'année du relevé déclare ses lacunes.

## Histoire des cotisations — anecdotes

`src-tauri/src/anecdotes/` associe à un **code** de cotisation un texte fixe :
origine, acteurs, controverses. `generer_bulletin` l'accole à l'explication derrière
U+0002 ; le front le détache dans `extractAidePosteDetail` (AVANT le détail aide au
poste derrière U+0001, sinon le JSON de ce dernier casse) et l'affiche dans un bloc
« HISTOIRE ». L'explication reste le texte technique et chiffré, l'anecdote ne porte
ni placeholder ni valeur du mois.

Même règle que pour les taux : **rien d'inventé**. Seuls des faits établis ; un code
sans anecdote sûre n'en reçoit pas.

Six langues, un fichier par langue (`fr.rs` fait référence ; `en/de/nl/it/es.rs` en
sont des traductions fidèles, rien d'ajouté). **Qui ajoute une anecdote l'ajoute dans
les six fichiers** : le test `memes_codes_dans_toutes_les_langues` exige les mêmes codes
partout. Une langue inconnue ne reçoit RIEN (pas de repli français au milieu d'un
bulletin traduit). Le bloc front porte `trad-skip` : pas de MyMemory par-dessus.

## Module DSN — extrait de déclaration annoté

`src/dsn.js` traduit un bulletin **France privé** déjà calculé en extrait de DSN
mensuelle (norme NEODeS, cahier technique CT2026.1) et l'affiche en bas de
bulletin — vues bureau et mobile — sous quatre onglets : **annoté** (chaque ligne
avec le libellé officiel de sa rubrique et la signification de sa valeur),
**fichier brut** (copiable), **tous les codes** (les listes fermées complètes,
valeur retenue surlignée, plus la table CTP Urssaf), **lacunes**.

Trois règles à respecter en y touchant :

1. **Le module est côté front, et c'est délibéré.** La DSN n'est pas un calcul
   mais une traduction d'un bulletin déjà produit par Rust. Deux données ne
   vivent que côté front : le PAS (`calculerPas`) et la date du formulaire.
2. **Les tables de référence sont GÉNÉRÉES, pas retapées** (libellés de blocs et
   de rubriques, listes de valeurs autorisées, codes de cotisation). Ne pas les
   corriger à la main : revenir à la source (cahier technique NEODeS ; table CTP
   en open data sur `open.urssaf.fr`, dataset `histocodestypescsv`).
3. **Aucune valeur inventée.** Ce que le simulateur ne sait pas est ABSENT de
   l'extrait et déclaré dans l'onglet « lacunes ». Le fichier est marqué envoi de
   test (S10.G00.00.005 = 01), les identifiants sont des zéros non attribuables
   (SIREN 000000000 : clé de Luhn valide, jamais attribué par l'Insee) et
   l'individu est déclaré sous NTT, pas sous NIR — conduite normative correcte
   quand ni NIR ni NIA ne sont connus, pas un pis-aller.

Le panneau se **monte à la demande** (premier clic) : le glossaire complet pèse
près de 2 000 lignes de tableau, inutile de les poser dans le DOM à chaque
calcul. Le conteneur porte `trad-skip` : la terminologie officielle ne passe pas
par le traducteur automatique.

Le bouton n'apparaît que pour `pays === 'france'`. La fonction publique parle un
autre dialecte de la norme (rubriques `[FP]`, régimes CNRACL/SRE/RAFP,
cotisations de la série 300) : ce serait un second mapping, pas une variante.

## Module « bulletin de paie PDF » — bas de bulletin

Le bouton **⇩ BULLETIN DE PAIE PDF** siège à droite du bouton DSN, dans la barre
`.dsn-actions` (`src/dsn.js`, paramètre `opt.actions` — dsn.js aligne, main.js
possède le bouton). Il produit le bulletin au **modèle réglementaire**, plus une
annexe détaillant ligne à ligne ce que ce modèle regroupe.

Quatre règles à respecter en y touchant :

1. **Le regroupement réglementaire est côté front** (`src/bulletin_pdf.js`), au
   même titre et pour la même raison que la DSN : ce n'est pas un calcul mais
   une traduction d'un bulletin déjà produit par Rust. Le back (`src-tauri/src/paie_pdf/`)
   ne sait ni ce qu'est une cotisation ni ce qu'est un net social — il place une
   grille sur une page A4. **Toutes les valeurs lui arrivent déjà
   formatées** : il n'arrondit rien.

   La grille suit la **disposition commune des logiciels de paie** (relevée sur
   des bulletins Sage et Cegid) : *Désignation · Nombre · Base* puis deux groupes
   titrés et séparés d'un filet vertical — **PART SALARIÉ** (*Taux · À payer ·
   À déduire*) et **PART EMPLOYEUR** (*Taux · Montant*). Les gains vont dans
   « à payer », retenues et cotisations dans « à déduire », sans signe : c'est la
   colonne qui dit le sens. Une réduction (heures supp, allègement) s'y inscrit
   en négatif, pour que chaque total reste la somme de sa colonne. Une ligne
   d'heures se lit *nombre × base (taux horaire à 4 décimales) × taux = à payer*.
   Les titres de colonne et de groupe viennent du front (`colonnes`, `groupes`),
   les largeurs du back (`CHIFFRES_BULLETIN`, `CHIFFRES_ANNEXE`).
2. **Deux modèles coexistent, la bascule se fait sur la DATE DE PAIE.** L'arrêté
   du 25 février 2016 fixe libellés, ordre et regroupement ; l'arrêté du
   31 janvier 2023 institue un modèle *rénové* dont l'arrêté du 11 août 2025 a
   reporté l'obligation au **1er janvier 2027**. Jusqu'au 31/12/2026 le modèle
   *adapté* (2016 + montant net social) reste utilisable. Le simulateur remonte à
   2015 : la frontière est déclarée une seule fois, dans `MODELE_BASCULE`.
   Ne pas la remplacer par « l'année en cours ».
3. **Aucune valeur inventée, et les rubriques vides le disent.** L'employeur, le
   SIRET, l'URSSAF, la convention collective et la classification sont TIRÉS AU
   SORT via `_ctAleatoire()` — le simulateur ne les connaît pas. D'où le filigrane
   SPÉCIMEN, le bandeau d'avertissement, et les rubriques réglementaires
   imprimées vides suivies de la liste de ce qui manque (FNAL, versement
   mobilité, taxe d'apprentissage…), exactement comme l'onglet « lacunes » de la
   DSN. Une cotisation qu'aucun poste du gabarit ne reconnaît atterrit dans
   « AUTRES COTISATIONS ET CONTRIBUTIONS » plutôt que de disparaître.
4. **La fonction publique territoriale a ses propres lacunes et son propre
   cartouche.** Un agent titulaire n'a pas de convention collective ; le modèle
   du code du travail ne lui est pas applicable et le PDF le dit. Ne pas lui
   servir les lacunes du privé.

⚠️ **Réserve sur le modèle rénové.** L'annexe de l'arrêté du 31 janvier 2023 n'a
pas pu être relevée sur Légifrance (texte rendu en JavaScript) ni sur le portail
BOSS. La structure du modèle rénové codée dans `MODELE_RENOVE` — cotisations
obligatoires / facultatives, regroupement des allègements, rubrique
« remboursements et déductions diverses » — vient de sources secondaires
concordantes (ADP, LégiSocial, Compta Online), pas du texte lui-même. Les
libellés exacts restent donc **à confirmer sur l'annexe officielle** avant le
1er janvier 2027. Le modèle adapté, lui, est celui de 2016, largement documenté.

L'identité fictive est tirée **une fois par session** (`_bpIdent`) : deux PDF
engendrés à la suite doivent sortir du même employeur, sinon le document change
de tête à chaque clic et on ne sait plus ce qu'on compare.

Le bouton n'apparaît que pour `france` et `fonction_publique` — le modèle
réglementaire du bulletin est une notion française, il n'a pas de sens pour les
42 autres régimes.

## Module RH « Gaabrielle » — contrat de travail

La vue `contrat` (`index.html`, `<div class="view-contrat">` ; `src/main.js`,
section « Contrat de travail », préfixe CSS `ct-`) compose un contrat français :
saisie exhaustive (employeur, salarié, contrat, poste, rémunération), catalogue de
20 clauses cochables et réordonnables, aperçu temps réel, puis PDF fabriqué par le
back Rust.

Deux règles à respecter en y touchant :

1. **`CT_CHAMPS` est la seule déclaration d'une information.** Chaque entrée
   produit à la fois le champ de saisie, la variable `{{cle}}` utilisable dans les
   clauses, et le libellé affiché tant que rien n'est saisi. Ajouter une
   information, c'est ajouter une ligne — pas trois.
2. **La numérotation des articles est le rang parmi les clauses cochées**, jamais
   un numéro stocké. Décocher un article ne laisse donc pas de trou, et les renvois
   `{{ref:cle}}` suivent tout seuls (ou affichent `[ARTICLE NON RETENU]`).

Le front assemble, numérote et substitue ; le Rust ne fait que la composition
typographique. Ne pas dupliquer le catalogue de clauses côté Rust : les clauses
sont éditables par l'utilisateur, la vérité est donc côté front.

**Le formulaire s'ouvre garni au hasard** (`_ctAleatoire`) : un contrat vide ne
montre rien de la mise en page. Les valeurs sont cohérentes entre elles — dates
ordonnées, taux horaire déduit du brut mensuel et de la quotité, essai et préavis
selon le statut. « Vider le brouillon » rend l'état vierge, celui où chaque clause
affiche le libellé de l'information qu'elle attend. Le bouton 🎲 rejoue un tirage.

Contenu des clauses volontairement loufoque (futur proche dystopique) : cette
itération sert à éprouver la mise en page, pas à produire un contrat opposable.
Le fond des valeurs tirées au sort emprunte à la fantasy européenne, dans le
registre de l'effectif de Gaabrielle.

## Module « Amphipoolis » — discussion sous pseudonyme, modérée (prototype)

Crate à part `amphipoolis/` (même montage que `meliinda/` : `path = "../amphipoolis"`
dans `src-tauri/Cargo.toml`, copiée à `/amphipoolis` par le Dockerfile), branchée
dans `src-tauri/src/bin/web.rs` ; front dans `src/amphipoolis.js`, vue
`.view-amphipoolis` de `index.html`, entrée « Amphipoolis prototype » du burger.
Web uniquement. Tables `amph_*` créées au démarrage dans la base partagée.

Règles portées par le serveur — le front ne fait que les montrer :

1. **Pseudonymat strict.** Pseudonyme + phrase secrète (Argon2id), aucun e-mail,
   aucune IP en base ; jetons de session stockés hachés (SHA-256). Unicité du nom
   insensible à la casse. Tout dépôt (pseudonyme, sujet, message) exige une
   preuve de travail ALTCHA neuve (anti-rejeu) et un champ piège `site` vide
   (`exiger_humain`). La preuve est vérifiée par une fonction que `web.rs` passe
   à la crate (`Captcha`) : le secret reste côté Xenna. Le front la calcule
   d'avance dès qu'une saisie commence (`preparerPreuve`), l'envoi reste instantané.
2. **Tout texte est modéré.** Sujet (titre + premier message) et message naissent
   `attente` ; un seul modérateur publie ou refuse (motif parmi une liste fermée,
   visible du seul auteur). `UPDATE … WHERE statut = 'attente'` : le premier qui
   tranche l'emporte, le second reçoit 409.
3. **Rôle fixé à la création** (participant ou modérateur), un pseudonyme de
   chaque rôle par IP et par 24 h. Le plafond vit en mémoire (empreinte de
   l'IP + rôle), jamais en base : `web.rs` pose l'IP dans les extensions
   (`IpClient`) pour la seule route `/creer`. Un modérateur ne tranche jamais
   ses propres textes (contrôle dans la requête SQL, pas seulement côté front).
4. **Pseudonyme refusé** : `etat = 'a_renommer'`, nom ajouté à `amph_noms_reserves`
   (plus personne ne peut le prendre), masqué partout (« pseudonyme retiré ») ; son
   titulaire ne peut plus écrire ni modérer avant d'en choisir un autre. Les textes
   suivent : ils sont rattachés à l'id du pseudonyme, pas au nom.
5. **Durée de vie** (`expire_le`, horodatages à format fixe `…Z` car comparés
   comme chaînes) : publié, un texte vit 7 jours ; chaque +1 d'un autre pseudonyme
   (un par texte, `amph_plus1`) ajoute 7 jours, plafonné à 30 jours après la
   publication. Refusé ou jamais modéré : 7 jours. Un sujet expiré reste tant
   qu'une réponse vit. `db::purger` tourne au démarrage puis toutes les 10 min
   (+1 et journal orphelins compris) ; les pseudonymes et noms réservés restent.
6. **Super administrateur, en dernier recours.** Le panneau admin
   (`/archives-bareme-1997`, section « Amphipoolis — modération supérieure »)
   liste tous les sujets et messages, quel que soit leur statut, sous le vrai nom
   des pseudonymes, et supprime définitivement un message ou un sujet (avec toutes
   ses réponses, +1 et journal compris), même publié. Les fonctions vivent dans
   `amphipoolis/src/admin.rs` ; les routes, derrière le JWT admin, dans
   `src-tauri/src/admin/routes.rs` — la crate n'a pas d'authentification admin.
   Valider ses propres textes avec deux pseudonymes distincts (participant puis
   modérateur) est VOULU : c'est ainsi qu'on teste, et les règles affichées le disent.

Quotas (`web.rs`) : création/entrée bornées comme les autres connexions ; seuls
les POST de sujets/messages sont bornés (les GET servent le rafraîchissement :
2 s dans un fil, 5 s ailleurs). Un modérateur connecté est veillé toutes les
15 s même hors de la vue et onglet caché (`amphVeille`, lancé au chargement du
site) : file d'attente dans le titre de l'onglet, « (3) Xenna Paye », et deux
notes brèves à l'arrivée de nouveaux textes (désactivables, « son : oui/non »).
