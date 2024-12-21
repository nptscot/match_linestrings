import { defineConfig } from "vite";
import { resolve } from "path";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig({
  base: "/match_linestrings/",
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
      },
    },
  },
  plugins: [svelte()]
})
