import type { PageLoad } from "./$types";

export const load = (async ({ fetch, params }) => {
  const id = params.id;
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
    pokemon
  };
}) satisfies PageLoad;
