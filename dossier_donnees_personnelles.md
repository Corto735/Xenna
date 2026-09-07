# Données personnelles — inventaire et position

**Établi le 15 août 2026.** Document de constat : rien n'a été retiré ni modifié
à cette date. Il sert de référence pour toute décision ultérieure.

## Position retenue

**Xenna Paie ne collecte aucune donnée utilisateur, par choix.** Principe posé
le 15 août 2026, applicable aux développements à venir.

Le code existant ne s'y conforme que partiellement. L'inventaire ci-dessous
décrit l'écart, sans le corriger. Toute nouvelle fonctionnalité doit se mesurer
à ce principe avant d'être écrite ; c'est là son intérêt principal, puisque
retirer une collecte déjà en place coûte toujours plus cher que ne pas
l'introduire.

## Ce qui est déjà conforme : le simulateur de paie

Le cœur du produit ne persiste rien. Vérifié par recherche exhaustive :
**aucun `INSERT`, `UPDATE` ou `DELETE` dans `src-tauri/src/calculs/` ni dans
`src-tauri/src/commands/`.**

Traversent le serveur sans jamais être écrits :

- nom, prénom, statut, salaire brut, ETP, ancienneté ;
- absences (dates, type d'arrêt, méthode de valorisation) ;
- net cible en paye inversée ;
- l'intégralité du bulletin calculé.

Il n'y a pas non plus de journal d'accès applicatif : aucun `TraceLayer` de
`tower-http` n'est monté dans `src-tauri/src/bin/web.rs`. Les logs du reverse
proxy Clever Cloud restent hors de portée du code.

La base SQLite est en lecture seule sur ce chemin : `ContextPaie` y lit des
barèmes (SMIC, PMSS, taux), jamais des données de personnes.

## Ce qui collecte aujourd'hui

Onze tables alimentées par des visiteurs. (`admin_users`, `apropos_posts` et
`ccn_reglementations` sont exclues : contenu d'exploitation, pas d'utilisateurs.)

| Table | Contenu | Origine |
|---|---|---|
| `users` | email chiffré AES-GCM + HMAC de recherche, hash argon2id du mot de passe, jeton de vérification | `forge/routes.rs` — `POST /forge/profil` |
| `contributor_profiles` | pseudo, poste, URL LinkedIn, statut de modération | idem |
| `contributor_ccn_expertises` | IDCC et niveau déclarés | idem + `membre/routes.rs` |
| `contributor_pays_expertises` | pays et niveau déclarés | idem |
| `forum_topics` | titre, contenu, auteur | `membre/routes.rs` — `POST /la_forge/forum/topic` |
| `forum_replies` | contenu, auteur | idem |
| `forge_votes` | votant, auteur voté, sujet | idem |
| `quizz_scores` | pseudo libre, pays, score | `quizz/routes.rs` — `POST /quizz/score` |
| `quizz_suggestions` | pseudo libre, question, réponses, source | `POST /quizz/suggestion` |
| `quizz_suggestion_votes` | `voter_id` pseudonyme persistant | `POST /quizz/vote/{id}` |
| `meliinda_sequences` | **frappes clavier horodatées à la milliseconde** | `meliinda/src/routes.rs` — `POST /api/meliinda/record` |

### Le cas Meliinda

C'est la collecte la plus intrusive de l'application, et la plus éloignée de son
objet. Une séquence Meliinda enregistre la dynamique de frappe : rythme entre
touches, hésitations, corrections. Cela relève de la **biométrie
comportementale** — une donnée identifiante en elle-même, indépendamment de son
contenu.

Elle est collectée sans compte, sans consentement explicite, depuis un panneau
accessible à tout visiteur par le menu burger (`index.html:2332`), et la
bibliothèque des séquences est publiquement consultable
(`GET /api/meliinda/sequences`).

Si un seul élément contredit frontalement la position retenue, c'est celui-ci.

## Sorties vers des tiers

### API de traduction MyMemory

`translateApp()` (`src/main.js:353-437`) parcourt tous les nœuds texte de
`document.body` et envoie ceux absents du dictionnaire statique à
`https://api.mymemory.translated.net` — API gratuite, sans clé, opérée par un
tiers. Autorisée explicitement dans le CSP (`connect-src`, `web.rs`).

Le filtre `_getTranslatableNodes()` (`main.js:326-344`) exclut les champs de
saisie, les valeurs numériques et les libellés de cotisation. Il **n'exclut pas**
le texte rendu contenant des données saisies : en vue Hercule,
`main.js:4444` affiche « Bulletin de *Prénom Nom* » dans un nœud texte ordinaire.

**Conséquence :** un utilisateur qui saisit son vrai nom puis change de langue
transmet ce nom à un serveur tiers. Le serveur Xenna n'en conserve rien, mais le
principe « aucune collecte » ne couvre pas « transit chez un tiers ».

*Correctif possible sans perte majeure :* se limiter au dictionnaire statique
(`trStatic`) et aux libellés déjà traduits par le backend (`crate::i18n`, six
langues), en supprimant l'appel réseau. Les chaînes hors dictionnaire resteraient
en français.

### Journal d'IP du limiteur de débit

`web.rs:191` — le middleware `limite_par_ip`, ajouté le 15 août 2026 avec les
correctifs de sécurité, journalise l'adresse IP lors d'un dépassement de quota :

```rust
tracing::warn!("Quota dépassé — {ip} sur {}", req.uri().path());
```

Pratique courante en anti-abus, mais c'est une adresse IP écrite dans les logs
Clever Cloud. Le compteur lui-même vit en mémoire et n'a pas besoin de cette
trace pour fonctionner : la ligne peut disparaître sans affaiblir la protection.

*Note :* le limiteur **manipule** des IP en mémoire (table `ratelimit`, purgée à
chaque fenêtre). C'est un traitement transitoire à finalité anti-abus, sans
persistance ni recoupement. Distinct d'une collecte, mais à mentionner pour être
complet.

## Le contrat de travail (module Gaabrielle) — ajouté le 7 septembre 2026

C'est la première fonctionnalité de Xenna qui manipule des données nominatives
réelles : nom, date et lieu de naissance, NIR, adresse, nationalité, titre de
séjour, IBAN, rémunération. Elle a donc été mesurée au principe ci-dessus avant
d'être écrite.

**Ce qui a été retenu.**

| Point | Décision |
|---|---|
| Saisie | reste dans le navigateur ; brouillon en `localStorage`, clé `xenna.contrat.brouillon` |
| Base de données | **aucune écriture** — aucune table n'a été créée pour ce module |
| Disque serveur | **aucune écriture** — le PDF est fabriqué en mémoire et rendu en base64 |
| Journalisation | le corps de requête n'est jamais journalisé, y compris en cas d'échec : seule la cause technique l'est (`src-tauri/src/bin/web.rs`, `handle_contrat_pdf`) |
| Tiers | aucun |

**L'écart assumé.** En version **web**, la fabrication du PDF est faite par le
back Rust : les informations saisies traversent donc le réseau et transitent en
mémoire serveur le temps de composer le document. C'est un choix explicite du
porteur du projet, pris en connaissance de cette tension. En version **Tauri**,
la question ne se pose pas : tout reste sur le poste.

Ce que ce choix coûte exactement : les données existent en mémoire du processus
serveur pendant la durée d'une requête, et n'en sortent par aucun chemin — ni
base, ni fichier, ni log, ni service tiers. Elles ne sont ni recoupables ni
retrouvables après la réponse.

Le chemin conforme, si la question se repose : composer le PDF côté client
(`window.print()` et une feuille `@media print`), au prix d'une maîtrise
typographique moindre et d'une sortie qui dépend du navigateur.

## Identifiants stockés côté client

Sans transmission au serveur, sauf mention contraire :

| Clé | Portée | Nature |
|---|---|---|
| `xenna-hv`, `xenna-zoom`, `xenna-dyslexia`, `xenna-bw`, `xenna-dactylo` | `localStorage` | préférences d'accessibilité, purement locales |
| `xenna.contrat.brouillon` | `localStorage` | **brouillon du contrat de travail en cours** — nominatif (nom, NIR, adresse, salaire), jamais transmis au serveur, effaçable par le bouton « Vider le brouillon » |
| `QZ_SESSION_KEY` | `sessionStorage` | progression du quizz en cours |
| `QZ_VOTER_KEY` | `localStorage` | **identifiant pseudonyme persistant**, transmis au serveur à chaque vote et stocké dans `quizz_suggestion_votes` |
| `QZ_VOTED_KEY` | `localStorage` | suggestions déjà votées |
| `forge_member_token` | `localStorage` | JWT membre, 7 jours |
| `xenna_admin_token` | `sessionStorage` | JWT admin, 8 heures |

## Options chiffrées, si la question se repose

1. **Nettoyage sans perte de fonction** — retirer le log d'IP, rendre la
   traduction locale. Deux fichiers touchés, aucune table supprimée, aucune
   fonctionnalité perdue hormis la traduction des chaînes hors dictionnaire.
2. **Retrait de Meliinda** — le point 1, plus la suppression de la route, de la
   table et du panneau. Supprime la collecte biométrique. Le crate `meliinda`
   et sa vue disparaissent ; La Forge, le forum et le quizz survivent.
3. **Zéro compte, zéro table** — retrait de La Forge, du forum, du quizz, de
   Meliinda, de la table `users`, du SMTP et du captcha. Il reste le simulateur,
   Le Chakrram et les pages éditoriales, tous sans état. La majorité des failles
   du diagnostic de sécurité du 15 août 2026 disparaîtraient avec le code
   correspondant — c'est la propriété la plus intéressante de cette option, et
   accessoirement la seule manière d'être exact quand on affiche « aucune
   donnée collectée ».

Aucune n'a été engagée à ce jour.
