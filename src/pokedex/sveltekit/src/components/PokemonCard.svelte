<script lang="ts">
  import { scale } from "svelte/transition";
  import { base } from "$app/paths";
  import type { Pokemon } from "../types/Pokemon";

  function toCapitalCase(word: string) {
    return word.charAt(0).toUpperCase() + word.slice(1);
  }

  export let pokemon: Pokemon;
  export let endPage = false;
</script>

<div in:scale out:scale class="card text-center shadow-2xl">
  <figure class="px-10 pt-10">
    <img loading="lazy" src={pokemon.image} alt={toCapitalCase(pokemon.name)} />
  </figure>
  <div class="card-body items-center">
    <h2 class="card-title">{toCapitalCase(pokemon.name)}</h2>
    {#if endPage}
      <p>Height: {pokemon.height} Dm</p>
      <p>Weight: {pokemon.weight} Hg</p>
    {:else}
      <div class="card-actions">
        <a class="btn-accent btn-outline btn" href="{base}/pokemon/{pokemon.id}">More info</a>
      </div>
    {/if}
  </div>
</div>
