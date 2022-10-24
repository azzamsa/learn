<script lang="ts">
  import { scale } from "svelte/transition";
  import { base } from "$app/paths";
  import { searchTerm as searchTermStore } from "../stores/search";
  import SearchIcon from "./icons/IconSearch.svelte";
  import HamburgerIcon from "./icons/IconHamburger.svelte";

  // Menu
  let showMenu = false;

  // Search
  let showSearchBar = false;
  export let searchTerm = "";
  $: $searchTermStore = searchTerm;
</script>

<div class="navbar rounded-box mb-2 bg-primary shadow-lg">
  <div class="md:hidden">
    <button on:click={() => (showMenu = !showMenu)} class="btn btn-ghost btn-square">
      <HamburgerIcon />
    </button>
  </div>

  <div class="navbar-start mx-2 px-2">
    <span class="text-lg font-bold">
      <a href="{base}/">Pokedex</a>
    </span>
  </div>

  <div class="navbar-center mx-2 px-2">
    <div class="flex hidden items-stretch md:flex">
      <a class="nav-btn" href="{base}/"> Home </a>
      <a class="nav-btn" href="{base}/about"> About </a>
    </div>
  </div>

  <div class="navbar-end">
    {#if showSearchBar}
      <div in:scale out:scale class="form-control text-neutral">
        <input
          type="text"
          placeholder="bulbasur"
          class="input input-bordered"
          bind:value={searchTerm}
        />
      </div>
    {/if}
    <button class="btn btn-ghost btn-square" on:click={() => (showSearchBar = !showSearchBar)}>
      <SearchIcon />
    </button>
  </div>
</div>

{#if showMenu}
  <div class="">
    <ul class="menu rounded-box bg-primary py-3 font-bold">
      <li><a href="{base}/">Home</a></li>
      <li><a href="{base}/about">About</a></li>
    </ul>
  </div>
{/if}

<style lang="postcss">
  .nav-btn {
    @apply btn btn-ghost btn-sm rounded-btn;
  }
</style>
