<script lang="ts">
  import { AppBar, LightSwitch } from "@skeletonlabs/skeleton"
  import LanguageSolid from "virtual:icons/heroicons/language-solid"
</script>

<AppBar
  class="font-heading"
  gridColumns="grid-cols-3"
  slotDefault="place-self-center"
  slotTrail="place-content-end"
>
  <svelte:fragment slot="lead">
    <a href="/" class="flex gap-2 items-center">
      <img src="logo.png" alt="logo" class="w-9" />
      <span class="font-heading font-bold">Qwiz</span>
    </a>
  </svelte:fragment>

  <nav class="flex gap-8 font-bold">
    <a href="/" class="btn hover:variant-filled-primary">Home</a>
    <a href="/about" class="btn hover:variant-filled-primary">About</a>
  </nav>

  <svelte:fragment slot="trail">
    <button class="btn-icon rounded-full hover:variant-filled-primary">
      <LanguageSolid class=" hover:stroke-2 " />
    </button>
    <LightSwitch class="hover:variant-filled-primary" />
  </svelte:fragment>
</AppBar>
