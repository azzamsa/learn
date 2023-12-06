import { purgeCss } from "vite-plugin-tailwind-purgecss"
import { sveltekit } from "@sveltejs/kit/vite"
import { defineConfig } from "vite"
import { paraglide } from "@inlang/paraglide-js-adapter-vite"
import Icons from "unplugin-icons/vite"

export default defineConfig({
    plugins: [
        sveltekit(),
        purgeCss(),
        Icons({
            compiler: "svelte",
            // experimental
            autoInstall: true
        }),
        paraglide({
            project: "./project.inlang",
            outdir: "./src/paraglide"
        })
    ]
})
