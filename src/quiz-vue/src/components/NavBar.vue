<script setup lang="ts">
const { availableLocales, locale } = useI18n()

// Menu
const showMenu = ref(false)

// Locale
const toggleLocales = () => {
  const locales = availableLocales
  locale.value = locales[(locales.indexOf(locale.value) + 1) % locales.length]
}
</script>

<template>
  <header>
    <div class="navbar rounded-box mb-2 bg-primary shadow-lg">
      <div class="md:hidden">
        <button class="btn btn-ghost btn-square" @click="showMenu = !showMenu">
          <i-tabler:menu-2 v-if="!showMenu" class="h-8 w-8" />
          <i-tabler:x v-else class="h-8 w-8" />
        </button>
      </div>

      <div class="navbar-start mx-2 px-2">
        <span class="text-lg font-bold">
          <RouterLink class="nav-btn" to="/"> Qüiz </RouterLink>
        </span>
      </div>

      <div class="navbar-center mx-2 px-2">
        <div class="hidden items-stretch md:flex">
          <RouterLink class="nav-btn" to="/">
            {{ $t('nav.home') }}
          </RouterLink>
          <RouterLink class="nav-btn" to="/about">
            {{ $t('nav.about') }}
          </RouterLink>
        </div>
      </div>

      <div class="navbar-end">
        <button class="btn btn-ghost btn-square" @click="toggleLocales">
          <i-tabler:language class="h-8 w-8" />
        </button>
      </div>
    </div>

    <ul
      v-if="showMenu"
      class="menu rounded-box bg-primary py-3 font-bold md:hidden"
    >
      <li>
        <RouterLink class="nav-btn" to="/" @click="showMenu = !showMenu">
          {{ $t('nav.home') }}
        </RouterLink>
      </li>
      <div class="divider mt-0 mb-0"></div>
      <li>
        <RouterLink class="nav-btn" to="/about" @click="showMenu = !showMenu">
          {{ $t('nav.about') }}
        </RouterLink>
      </li>
    </ul>
  </header>
</template>

<style scoped>
.nav-btn {
  @apply btn btn-ghost btn-sm rounded-btn;
}
</style>
