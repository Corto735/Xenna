# ── Stage 1 : build frontend ──────────────────────────────────────────────────
FROM node:22-slim AS frontend
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY index.html vite.config.js ./
COPY src ./src
RUN npm run build

# ── Stage 2 : build Rust binary ───────────────────────────────────────────────
FROM rust:1.88-slim AS backend
WORKDIR /app

# Dépendances système pour SQLx (sqlite3)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./
COPY src-tauri/src ./src
COPY src-tauri/migrations ./migrations
COPY src-tauri/build.rs ./build.rs
COPY src-tauri/capabilities ./capabilities

RUN cargo build --release --bin web

# ── Stage 3 : image finale ────────────────────────────────────────────────────
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    libsqlite3-0 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend /app/target/release/web ./web
COPY --from=frontend /app/dist ./dist

ENV DIST_DIR=/app/dist
ENV DATABASE_PATH=/data/xenna.db
EXPOSE 8080

CMD ["./web"]
