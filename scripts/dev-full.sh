#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# Dev « tout-en-un » : lance le backend Axum (:8080) PUIS le front, ensemble.
#
# Pourquoi : en dev, Vite (:1420) proxifie /api, /admin, /forge, /quizz… vers le
# serveur Axum :8080 (cf. vite.config.js). Sans ce serveur, les pages À propos /
# Carnet / Admin / Quizz renvoient ECONNREFUSED. Ce script démarre les deux.
#
# Usage :
#   npm run dev:full     → backend :8080 + app Tauri (desktop)   [défaut]
#   npm run dev:web      → backend :8080 + Vite seul (navigateur)
#
# Variables optionnelles :
#   FRONT=web            → front navigateur au lieu de Tauri
#   ADMIN_JWT_SECRET=…   → secret JWT admin (défaut : "dev", insécurisé, local only)
#   DATABASE_PATH=…      → base du backend (défaut : src-tauri/xenna.db)
# ─────────────────────────────────────────────────────────────────────────────
set -euo pipefail

# Racine du projet (le script vit dans scripts/).
cd "$(dirname "$0")/.."

DB_PATH="${DATABASE_PATH:-src-tauri/xenna.db}"
JWT="${ADMIN_JWT_SECRET:-dev}"

echo "▶ Compilation du backend Axum (web)…"
( cd src-tauri && cargo build --bin web )

echo "▶ Backend sur http://localhost:8080  (base : $DB_PATH)"
ADMIN_JWT_SECRET="$JWT" \
DATABASE_PATH="$DB_PATH" \
DIST_DIR="dist" \
  ./src-tauri/target/debug/web &
WEB_PID=$!

# Arrêt propre du backend quand le front se ferme (Ctrl+C ou fin de process).
cleanup() {
  echo ""
  echo "■ Arrêt du backend (pid $WEB_PID)…"
  kill "$WEB_PID" 2>/dev/null || true
  wait "$WEB_PID" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

# Petit répit pour que le port 8080 soit à l'écoute avant le premier fetch.
for _ in $(seq 1 30); do
  if curl -s -o /dev/null -m 1 http://127.0.0.1:8080/api/apropos/posts; then break; fi
  sleep 0.3
done

if [ "${FRONT:-tauri}" = "web" ]; then
  echo "▶ Front navigateur : http://localhost:1420  (Ctrl+C pour tout arrêter)"
  npm run dev
else
  echo "▶ App Tauri (desktop) + Vite  (Ctrl+C pour tout arrêter)"
  npm run tauri dev
fi
