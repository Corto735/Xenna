import { defineConfig } from "vite";

export default defineConfig({
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    // En dev, le moteur de paie et la Forge tournent sur Axum :8080.
    // Vite proxifie ces préfixes pour éviter les erreurs CORS / 404 HTML.
    proxy: {
      '/forge':  { target: 'http://localhost:8080', changeOrigin: true },
      '/profil': { target: 'http://localhost:8080', changeOrigin: true },
      '/api':    { target: 'http://localhost:8080', changeOrigin: true },
    },
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "chrome105",
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
