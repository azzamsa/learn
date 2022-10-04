import preprocess from "svelte-preprocess";
import static_adapter from "@sveltejs/adapter-static";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: preprocess(),

  kit: {
    adapter: static_adapter(),
    paths: {
      base: "/svelte-kit-blog-demo",
      assets: "/svelte-kit-blog-demo"
    },

    // hydrate the <div id="svelte"> element in src/app.html
    target: "#svelte"
  }
};

export default config;
