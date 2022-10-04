<script setup lang="ts">
import type { Pokemon } from '../types/Pokemon'

defineProps<{
  pokemon: Pokemon
  isDetailPage: boolean
}>()

function toCapitalCase(word: string) {
  return word.charAt(0).toUpperCase() + word.slice(1)
}
</script>

<template>
  <Head v-if="isDetailPage">
    <title>Pokédex - {{ toCapitalCase(pokemon.name) }}</title>
  </Head>

  <div class="card text-center shadow-2xl">
    <figure class="px-10 pt-10">
      <img
        loading="lazy"
        :src="pokemon.image"
        :alt="toCapitalCase(pokemon.name)"
      />
    </figure>
    <div class="card-body items-center text-center">
      <h2 class="card-title">
        {{ toCapitalCase(pokemon.name) }}
      </h2>
      <div v-if="isDetailPage">
        <p>Height: {{ pokemon.height }} Dm</p>
        <p>Weight: {{ pokemon.weight }} Hg</p>
      </div>
      <div v-if="!isDetailPage" class="card-actions">
        <RouterLink
          class="btn btn-outline btn-accent"
          :to="`/pokemon/${pokemon.id}`"
        >
          {{ $t('More Info') }}
        </RouterLink>
      </div>
    </div>
  </div>
</template>
