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
const toggleLocales = () => {
  const locales = availableLocales
  locale.value = locales[(locales.indexOf(locale.value) + 1) % locales.length]
}
</script>

<template>
  <div class="navbar rounded-box mb-2 bg-primary shadow-lg">
    <div class="md:hidden">
      <button class="btn btn-ghost btn-square" @click="showMenu = !showMenu">
        <i-tabler:menu-2 class="h-8 w-8" />
      </button>
    </div>

    <div class="navbar-start mx-2 px-2">
      <span class="text-lg font-bold">
        <RouterLink class="nav-btn" to="/"> Pokédex </RouterLink>
      </span>
    </div>

    <div class="navbar-center mx-2 px-2">
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
        />
      </div>
      <button
        class="btn btn-ghost btn-square"
        @click="showSearchBar = !showSearchBar"
      >
        <i-tabler:search class="h-8 w-8" />
      </button>
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
  @apply btn btn-ghost btn-sm rounded-btn;
}
</style>
