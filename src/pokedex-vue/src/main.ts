import { createHead } from '@vueuse/head'
import { createPinia } from 'pinia'
import { setupLayouts } from 'virtual:generated-layouts'
import { createApp } from 'vue'
import { createI18n } from 'vue-i18n'
import { createRouter, createWebHistory } from 'vue-router'

import App from './App.vue'
import './assets/main.css'
import { messages } from './messages'
import generatedRoutes from '~pages'

const routes = setupLayouts(generatedRoutes)
const router = createRouter({
  history: createWebHistory(),
  routes,
})
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages,
})
const head = createHead()
const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(i18n)
app.use(head)

app.mount('#app')
