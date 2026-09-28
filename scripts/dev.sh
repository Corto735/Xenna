#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# Front de dev (Vite :1420) + backend Axum (:8080), lancés ensemble.
#
# Pourquoi : en dev, Vite proxifie /api, /admin, /forge, /quizz… vers Axum :8080
# (cf. vite.config.js) — Meliinda, la Forge, l'admin, le quizz, « À propos »
# n'existent que là. Sans ce serveur : ECONNREFUSED dans le terminal.
#
# C'est la commande `npm run dev`, que Tauri lance lui-même avant l'appli
# (`beforeDevCommand`) : `npm run tauri dev` démarre donc tout, comme avant.
#   npm run tauri dev   → appli desktop + Vite + backend
#   npm run dev         → Vite + backend (navigateur : http://localhost:1420)
#   npm run dev:vite    → Vite seul
#
# Le backend compile en arrière-plan dans son propre dossier (target/web-dev) :
# Vite répond aussitôt et la compilation de l'appli Tauri n'attend pas le verrou
# de Cargo. Journal : src-tauri/target/web-dev.log. Un backend déjà à l'écoute
# sur :8080 est réutilisé tel quel.
#
# XENNA_DEV_MODE=1 lève l'exigence des quatre secrets (debug uniquement, jamais
# en release). Base : DATABASE_PATH, par défaut src-tauri/xenna.db.
# ─────────────────────────────────────────────────────────────────────────────
set -uo pipefail
cd "$(dirname "$0")/.."

WEB_PID=""
if curl -s -o /dev/null -m 1 http://127.0.0.1:8080/; then
  echo "▶ Backend déjà à l'écoute sur :8080 — réutilisé"
else
  mkdir -p src-tauri/target
  echo "▶ Backend Axum :8080 en préparation (journal : src-tauri/target/web-dev.log)"
  (
    cd src-tauri
    CARGO_TARGET_DIR=target/web-dev \
    XENNA_DEV_MODE=1 \
    DATABASE_PATH="${DATABASE_PATH:-xenna.db}" \
    DIST_DIR=../dist \
      exec cargo run --bin web
  ) > src-tauri/target/web-dev.log 2>&1 &
  WEB_PID=$!

  # Tauri peut tuer ce script sans lui laisser exécuter de trap : un veilleur
  # arrête le backend dès que le script disparaît.
  PARENT=$$
  ( while kill -0 "$PARENT" 2>/dev/null; do sleep 2; done
    kill "$WEB_PID" 2>/dev/null ) &

  # Annonce la disponibilité du backend sans bloquer Vite.
  ( for _ in $(seq 1 600); do
      if curl -s -o /dev/null -m 1 http://127.0.0.1:8080/; then
        echo "✔ Backend prêt sur http://localhost:8080"; exit 0
      fi
      kill -0 "$WEB_PID" 2>/dev/null || { echo "✘ Le backend s'est arrêté — voir src-tauri/target/web-dev.log"; exit 1; }
      sleep 1
    done ) &
fi

cleanup() { [ -n "$WEB_PID" ] && kill "$WEB_PID" 2>/dev/null; }
trap cleanup EXIT INT TERM

./node_modules/.bin/vite
