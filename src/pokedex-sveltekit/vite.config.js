import { sveltekit } from "@sveltejs/kit/vite";

/** @type {import('vite').UserConfig} */
const config = {
  base: "/pokedex-svelte/",
  plugins: [sveltekit()]
};

export default config;
