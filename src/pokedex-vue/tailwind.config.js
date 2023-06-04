/* eslint-env node */
module.exports = {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: [
      {
        cupcake: {
          ...require('daisyui/src/theming/themes')['[data-theme=cupcake]'],
          '--btn-text-case': 'none',
        },
        dark: {
          ...require('daisyui/src/theming/themes')['[data-theme=dark]'],
          '--btn-text-case': 'none',
        },
      },
    ],
  },
  plugins: [require('daisyui')],
}
