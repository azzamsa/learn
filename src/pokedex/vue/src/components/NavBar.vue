<script setup lang="ts">
const { availableLocales, locale } = useI18n()

// Menu
const showMenu = ref(false)

// Search
const searchStore = useSearchStore()
const showSearchBar = ref(false)
const updateSearchTerm = debounce((e) => {
  searchStore.setSearchTerm(e.target.value)
}, 100)

// Locale
function toggleLocales() {
  const locales = availableLocales
  locale.value = locales[(locales.indexOf(locale.value) + 1) % locales.length]
}
</script>

<template>
  <div class="mb-2 shadow-lg navbar rounded-box bg-primary">
    <div class="md:hidden">
      <button class="btn btn-ghost btn-square" @click="showMenu = !showMenu">
        <i-tabler:menu-2 class="w-8 h-8" />
      </button>
    </div>

    <div class="px-2 mx-2 navbar-start">
      <span class="text-lg font-bold">
        <RouterLink class="nav-btn" to="/"> Pokédex </RouterLink>
      </span>
    </div>

    <div class="px-2 mx-2 navbar-center">
      <div class="hidden items-stretch md:flex">
        <RouterLink class="nav-btn" to="/">
          {{ $t('Home') }}
        </RouterLink>
        <RouterLink class="nav-btn" to="/about">
          {{ $t('About') }}
        </RouterLink>
      </div>
    </div>

    <div class="navbar-end">
      <div v-if="showSearchBar" class="form-control">
        <input
          type="text"
          placeholder="bulbasur"
          class="input input-bordered"
          @input="updateSearchTerm"
        >
      </div>
      <button
        class="btn btn-primary btn-square"
        @click="showSearchBar = !showSearchBar"
      >
        <i-tabler:search class="w-8 h-8" />
      </button>
      <button class="btn btn-primary btn-square" @click="toggleLocales">
        <i-tabler:language class="w-8 h-8" />
      </button>
    </div>
  </div>

  <ul
    v-if="showMenu"
    class="py-3 font-bold md:hidden menu rounded-box bg-primary"
  >
    <li>
      <RouterLink class="nav-btn" to="/">
        {{ $t('Home') }}
      </RouterLink>
    </li>
    <li>
      <RouterLink class="nav-btn" to="/about">
        {{ $t('About') }}
      </RouterLink>
    </li>
  </ul>
</template>

<style scoped>
.nav-btn {
  @apply btn-primary rounded-btn btn-sm btn;
}
</style>
