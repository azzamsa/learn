/* eslint-env node */
require('@rushstack/eslint-patch/modern-module-resolution')

module.exports = {
  root: true,
  ignorePatterns: ['auto-imports.d.ts'],
  // `@sxzz/eslint-config` plays well with TailwindCSS
  // and `@anthony` doesn't
  extends: ['@sxzz/eslint-config'],
  overrides: [
    {
      files: ['cypress/e2e/**.{cy,spec}.{js,ts,jsx,tsx}'],
      extends: ['plugin:cypress/recommended'],
    },
  ],
}
