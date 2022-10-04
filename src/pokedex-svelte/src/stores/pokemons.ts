import { writable } from "svelte/store";
import type { Pokemon } from "../types/Pokemon";

export const pokemons = writable([]);

const fetchPokemons = async () => {
  // use maximum limit. we have pagination.
  // 894 is the last pokemon with image.
  const url = "https://pokeapi.co/api/v2/pokemon?limit=894";
  const res = await fetch(url);
  const data = await res.json();
  const pokemons_ = data.results.map((pokemon: Pokemon, index: number) => {
    return {
      name: pokemon.name,
      id: index + 1,
      image: `https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/${
        index + 1
      }.png`
    };
  });
  pokemons.set(pokemons_);
};
fetchPokemons();
