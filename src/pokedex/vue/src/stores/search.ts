import { defineStore } from 'pinia'

export const useSearchStore = defineStore({
  id: 'search',
  state: () => ({
    searchTerm: '',
  }),
  getters: {
    getSearchTerm(state) {
      return state.searchTerm
    },
    isSearching(state) {
      if (state.searchTerm === '')
        return false
      else
        return true
    },
  },
  actions: {
    setSearchTerm(term: string) {
      this.$state.searchTerm = term
    },
  },
})
