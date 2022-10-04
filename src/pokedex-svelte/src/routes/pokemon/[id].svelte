<script context="module" lang="ts">
  import type { Pokemon } from "../../types/Pokemon";

  export async function load({ url }) {
    const id = url.params.id;
    const url_ = `https://pokeapi.co/api/v2/pokemon/${id}`;
    const res = await fetch(url_);
    const pokemon_ = await res.json();

    const pokemon = {
      id: pokemon_.id,
      name: pokemon_.name,
      image: `https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/${
        pokemon_.id + 1
      }.png`,
      height: pokemon_.height,
      weight: pokemon_.weight
    };
    return {
      props: {
        pokemon
      }
    };
  }
</script>

<script lang="ts">
  import PokemonCard from "../../components/PokemonCard.svelte";

  export let pokemon: Pokemon;
</script>

<PokemonCard {pokemon} endPage={true} />
