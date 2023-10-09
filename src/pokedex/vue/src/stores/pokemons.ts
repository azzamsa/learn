import { defineStore } from 'pinia'

import type { Pokemon } from '../types/Pokemon'

export const usePokemonsStore = defineStore({
  id: 'pokemons',
  state: () => ({
    pokemons: [] as Pokemon[] | Pokemon[],
    loading: false,
  }),
  getters: {
    getPokemons(state) {
      return state.pokemons
    },
    isLoading(state) {
      return state.loading
    },
  },
  actions: {
    async fetchPokemons() {
      this.$state.loading = true

      const url = 'https://pokeapi.co/api/v2/pokemon?limit=894'
      const res = await fetch(url)
      const data = await res.json()
      const pokemons_ = data.results.map((pokemon: Pokemon, index: number) => {
        return {
          name: pokemon.name,
          id: index + 1,
          image: `https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/${
            index + 1
          }.png`,
        }
      })
      this.$state.pokemons = pokemons_

      this.$state.loading = false
    },
  },
})
