import { join } from "path"
import type { Config } from "tailwindcss"
import typography from "@tailwindcss/typography"
import { skeleton } from "@skeletonlabs/tw-plugin"

export default {
  darkMode: "class",
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    join(require.resolve("@skeletonlabs/skeleton"), "../**/*.{html,js,svelte,ts}")
  ],
  theme: {
    extend: {
      fontFamily: {
        heading: ["Cinzel Decorative", "sans-serif"]
      }
    }
  },
  plugins: [
    "prettier-plugin-tailwindcss",
    typography,
    skeleton({
      themes: {
        preset: [
          {
            name: "skeleton",
            enhancements: true
          }
        ]
      }
    })
  ]
} satisfies Config
