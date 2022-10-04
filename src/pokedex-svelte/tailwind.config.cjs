/* eslint-env node */
module.exports = {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {}
  },
  daisyui: {
    themes: ["cupcake"]
  },
  plugins: [require("daisyui")]
};
