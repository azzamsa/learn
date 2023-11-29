<script setup lang="ts">
import { ref } from 'vue'
import type { Ref } from 'vue'

import type { Pokemon } from '../../types/Pokemon'

// Auto register for types is WIP
// <https://github.com/antfu/unplugin-auto-import/issues/61>

// `id` passed from vite-plugin-pages
// props passed has `string` type
const props = defineProps<{ id: string }>()

const pokemon_: Ref<Pokemon> = ref({
  id: 0,
  name: '',
  image: '',
  height: 0,
  weight: 0,
})

onMounted(async () => {
  const id = props.id
  const url = `https://pokeapi.co/api/v2/pokemon/${id}`
  const res = await fetch(url)
  const pokemonItem = await res.json()

  const pokemonResult = await {
    id: pokemonItem.id,
    name: pokemonItem.name,
    image: `https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/${
      pokemonItem.id + 1
    }.png`,
    height: pokemonItem.height,
    weight: pokemonItem.weight,
  }
  pokemon_.value = pokemonResult
})
</script>

<template>
  <PokemonCard :pokemon="pokemon_" :is-detail-page="true" />
</template>
