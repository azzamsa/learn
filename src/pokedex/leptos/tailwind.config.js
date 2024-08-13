/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  daisyui: {
    themes: ["cupcake"],
  },
  plugins: [require("daisyui")],
};
