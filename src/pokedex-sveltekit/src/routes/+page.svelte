<script lang="ts">
  import { paginate, LightPaginationNav } from "svelte-paginate";

  import type { Pokemon } from "../types/Pokemon";
  import { pokemons } from "../stores/pokemons";
  import PokemonCard from "../components/PokemonCard.svelte";
  import { searchTerm } from "../stores/search";

  let filteredPokemon: Pokemon[] = [];
  let paginatedItems: Pokemon[] = [];

  $: {
    if ($searchTerm) {
      filteredPokemon = $pokemons.filter((pokemon: Pokemon) =>
        pokemon.name.includes($searchTerm.toLowerCase())
      );
    } else {
      filteredPokemon = [...$pokemons];
    }
  }

  let currentPage = 1;
  let pageSize = 6;
  let items = [];
  $: {
    items = filteredPokemon;
    currentPage = currentPage;
    paginatedItems = paginate({ items, pageSize, currentPage });
  }
</script>

<svelte:head>
  <title>Pokedex - Home</title>
</svelte:head>

<div class="grid grid-cols-2 gap-2">
  {#each paginatedItems as pokemon}
    <PokemonCard {pokemon} />
  {/each}
</div>

<div class="mt-4">
  <LightPaginationNav
    totalItems={items.length}
    {pageSize}
    {currentPage}
    limit={1}
    showStepOptions={true}
    on:setPage={(e) => (currentPage = e.detail.page)}
  />
</div>
