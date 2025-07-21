import { defineConfig } from "vite";
import { resolve } from "path";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasmPack from "vite-plugin-wasm-pack";

export default defineConfig({
  base: "/match_linestrings/",
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        review: resolve(__dirname, "review.html"),
      },
    },
  },
  plugins: [svelte(), wasmPack(["../backend"], [])]
})
