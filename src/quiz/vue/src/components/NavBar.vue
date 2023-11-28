<script setup lang="ts">
const { availableLocales, locale } = useI18n()

// Menu
const showMenu = ref(false)

// Locale
function toggleLocales() {
  const locales = availableLocales
  locale.value = locales[(locales.indexOf(locale.value) + 1) % locales.length]
}
</script>

<template>
  <header>
    <div class="mb-2 shadow-lg navbar rounded-box bg-primary">
      <div class="md:hidden">
        <button class="btn-ghost btn-square btn" @click="showMenu = !showMenu">
          <i-tabler:menu-2 v-if="!showMenu" class="w-8 h-8" />
          <i-tabler:x v-else class="w-8 h-8" />
        </button>
      </div>

      <div class="px-2 mx-2 navbar-start">
        <span class="text-lg font-bold">
          <RouterLink class="nav-btn" to="/"> Qüiz </RouterLink>
        </span>
      </div>

      <div class="px-2 mx-2 navbar-center">
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
        <button class="btn-ghost btn-square btn" @click="toggleLocales">
          <i-tabler:language class="w-8 h-8" />
        </button>
      </div>
    </div>

    <ul
      v-if="showMenu"
      class="py-3 font-bold md:hidden menu rounded-box bg-primary"
    >
      <li>
        <RouterLink class="nav-btn" to="/" @click="showMenu = !showMenu">
          {{ $t('nav.home') }}
        </RouterLink>
      </li>
      <div class="mt-0 mb-0 divider" />
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
  @apply btn-ghost rounded-btn btn-sm btn;
}
</style>
