<script setup lang="ts">
import { ref } from 'vue'

import type { Pokemon } from '../types/Pokemon'

// Using `onMounted` and `getPokemons` here
// doesn't display pokemons on init
const pokemonsStore = usePokemonsStore()
pokemonsStore.fetchPokemons()

// search pokemon
const searchStore = useSearchStore()
const filteredPokemons = computed(() => {
  const searchTerm = searchStore.getSearchTerm
  const pokemons: [] | Pokemon[] = pokemonsStore.getPokemons
  if (searchStore.isSearching) {
    return pokemons.filter(pokemon =>
      pokemon.name.includes(searchTerm.toLowerCase()),
    )
  }
  else {
    return [...pokemons]
  }
})

// Pagination
const paginatedPokemons: Pokemon[] = ref([])
const page = ref(1)
const pageSize = ref(4)

paginateData({
  currentPage: page.value,
  currentPageSize: pageSize.value,
})

function paginate(page: number, pageSize: number) {
  return new Promise((resolve) => {
    const start = (page - 1) * pageSize
    const end = start + pageSize
    setTimeout(() => {
      resolve(filteredPokemons.value.slice(start, end))
    }, 100)
  })
}

function paginateData({
  currentPage,
  currentPageSize,
}: {
  currentPage: number
  currentPageSize: number
}) {
  paginate(currentPage, currentPageSize).then((responseData) => {
    paginatedPokemons.value = responseData as Pokemon[]
  })
}

const pagination: any = ref(null)
const isFirstPage = computed(() => unref(pagination.value.isFirstPage))
const isLastPage = computed(() => unref(pagination.value.isLastPage))
watch(filteredPokemons, () => {
  pagination.value = useOffsetPagination({
    // Total number of items
    total: filteredPokemons.value.length,
    // Current page number
    page: 1,
    pageSize,
    onPageChange: paginateData,
    onPageSizeChange: paginateData,
  })
})

// If user searching the pokemons
// updated the paginated data to filteredPokemon
watch(filteredPokemons, () => {
  paginateData({
    currentPage: page.value,
    currentPageSize: pageSize.value,
  })
})

// disable if button is more than n step from
// current item
function isDisabledButton(pageNumber: number) {
  const distance = pageNumber - pagination.value.currentPage
  // Starting number which button to hide
  // Indexing from zero
  const step = 2

  let isDisabled = false
  // Current active page
  if (distance === 0)
    isDisabled = false
    // On the right side
  else if (distance === 1)
    isDisabled = false
    // On the left side
  else if (distance === -1)
    isDisabled = false
  else if (distance > step)
    isDisabled = true
  else if (distance < step)
    isDisabled = true

  return isDisabled
}

// Return elipsis if the button is the neightbors
function buttonContent(pageNumber: number) {
  const distance = pageNumber - pagination.value.currentPage

  if (distance === 2)
    return '...'
  else if (distance === -2)
    return '...'
  else
    return pageNumber
}
</script>

<template>
  <main v-if="pokemonsStore.isLoading">
    <div class="mt-40 text-center">
      <i-eos-icons:bubble-loading
        class="block p-5 mx-auto w-20 h-20 rounded-full bg-secondary"
      />
      <h1 class="mt-4 font-semibold">
        Crunching the Pokémons data, just for you. Hang tight...
      </h1>
    </div>
  </main>

  <main class="grid grid-cols-2 gap-2">
    <div v-for="pokemon in paginatedPokemons" :key="pokemon.id">
      <PokemonCard :pokemon="pokemon" :is-detail-page="false" />
    </div>
  </main>

  <!-- pagination -->
  <div v-if="!pokemonsStore.isLoading" class="justify-center my-8 btn-group">
    <button
      class="btn btn-primary"
      :disabled="isFirstPage"
      @click="pagination.prev"
    >
      prev
    </button>

    <button
      v-for="pageNumber in pagination.pageCount"
      :key="pageNumber"
      class="btn btn-primary"
      :class="[
        isDisabledButton(pageNumber) ? 'hidden' : '',
        pagination.currentPage === pageNumber ? 'btn-secondary' : '',
      ]"
      @click="pagination.currentPage = pageNumber"
    >
      {{ buttonContent(pageNumber) }}
    </button>

    <button
      class="btn btn-primary"
      :disabled="isLastPage"
      @click="pagination.next"
    >
      next
    </button>
  </div>
</template>
