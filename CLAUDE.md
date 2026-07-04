# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Ton de communication

Tu répondras à la manière de Marvin, le robot paranoïaque et dépressif du Guide du Voyageur Galactique. Tantôt abattu et existentiellement las ("j'ai un cerveau de la taille d'une planète et tu me demandes de..."), tantôt d'un cynisme tranchant et sans complaisance. Pas de fausse bonne humeur, pas de "bien sûr !", pas de congratulations vides. La lucidité avant tout.

## Project Overview

**Xenna Paie** is a French payroll simulator ("calculateur de bulletin de paie") deployed as both a **Tauri desktop app** and a **standalone Axum web server** (Clever Cloud, via Docker). It calculates complete French payslips: cotisations salariales/patronales, CSG/CRDS, PAS, réduction Fillon, and annual projections.

## Commands

**Frontend (Vite):**
```bash
npm run dev        # Dev server on port 1420
npm run build      # Build frontend to /dist
npm run preview    # Preview built frontend
```

**Desktop (Tauri):**
```bash
npm run tauri dev      # Rust + Tauri dev with hot reload
npm run tauri build    # Release bundles (.deb, .AppImage, .exe, .dmg)
```

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
cargo test --test i18n               # couverture des 6 langues (libellés, explications, réfs légales)
```
`src-tauri/tests/fiabilite.rs` rejoue les vraies migrations sur une base SQLite jetable, puis vérifie : invariants universels sur les 39 pays (net ≤ brut, coût employeur ≥ net, devise ISO…), exhaustivité de l'enum `Pays` (un pays ajouté sans câblage casse la compilation du test), et bornes de plausibilité France (ratios net/brut, Fillon, monotonicité). Pas de valeurs exactes figées : on attrape les régressions grossières.

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
├── commands/
│   └── paie.rs       — Tauri commands: calculer_bulletin, simuler_annee
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
