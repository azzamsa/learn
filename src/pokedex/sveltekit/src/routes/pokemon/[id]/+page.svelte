<script lang="ts">
  import PokemonCard from "../../../components/PokemonCard.svelte";
  import type { Pokemon } from "../../../types/Pokemon";

  export let data;
  export let pokemon: Pokemon = data.pokemon;
</script>

<PokemonCard {pokemon} endPage={true} />
