<div align="center">
<h1>Qüiz.</h1><img src='docs/logo.png' width=150px/>

Test your general knowledge 🎲.

<a href="https://github.com/azzamsa/learn/actions/workflows/quiz_vue.yml">
  <img src="https://github.com/azzamsa/learn/actions/workflows/quiz_vue.yml/badge.svg" alt="Build status" />
</a>

<br/>

</div>

---

Simple Qüiz application.

## 🔥 Features

- Vue 3
  - 🚦 Vue router
  - 🗂 [File based routing](./src/pages)
  - 📦 [Components auto importing](./src/components)
  - 🍍 [State Management via Pinia](https://pinia.vuejs.org/)
  - 📑 [Layout system](./src/layouts)
  - 😃 [Use icons from any iconify sets](https://github.com/antfu/unplugin-icons)
  - 🌍 [I18n ready](./locales)
  - 🔥 Use the [new `<script setup>` syntax](https://github.com/vuejs/rfcs/pull/227)
  - 🤙🏻 [Reactivity Transform](https://vuejs.org/guide/extras/reactivity-transform.html) enabled
  - 📥 [APIs auto importing](https://github.com/antfu/unplugin-auto-import) - use Composition API and others directly
  - ☁️ Deploy on Netlify, zero-config
  - ⚙️ Unit Testing with [Vitest](https://github.com/vitest-dev/vitest), E2E Testing with [Cypress](https://cypress.io/).
- 🎛️ [VueUse](https://github.com/antfu/vueuse) - collection of useful composition APIs
  - [`@vueuse/head`](https://github.com/vueuse/head) - manipulate document head reactively
- 🦾 TypeScript, of course
- 🍃 TailwindCSS + 🌼 daisyUI
  - Sort TailwindCSS class automatically.
- Strict ESLint, Git hooks, Makefile, and more.

## Navigating the Code

All the features can be viewed in the [CHANGELOG](CHANGELOG.md) file tagged with `feat`.
The file only contains user-facing changes, so you won't get lost bisecting the features.

## Credits

- Inspired by Gwendolyn Faraday's [demo quiz app](https://github.com/gwenf/vue-quiz)
- [Open Trivia DB](https://opentdb.com/) for questions & answers.
