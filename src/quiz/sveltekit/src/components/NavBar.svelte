<script lang="ts">
  import * as m from "$paraglide/messages"
  import { availableLanguageTags, languageTag, setLanguageTag } from "$paraglide/runtime"
  import { currentLocale } from "$stores/locale"
  import { AppBar, LightSwitch } from "@skeletonlabs/skeleton"
  import LanguageSolid from "virtual:icons/heroicons/language-solid"

  // Locale
  function toggleLocale() {
    const locales = availableLanguageTags
    $currentLocale = locales[(locales.indexOf(languageTag()) + 1) % locales.length]
  }

  let home = m.home()
  let about = m.about()
  $: {
    setLanguageTag($currentLocale)
    home = m.home()
    about = m.about()
  }
</script>

<AppBar
  class="font-serif"
  gridColumns="grid-cols-3"
  slotDefault="place-self-center"
  slotTrail="place-content-end"
>
  <svelte:fragment slot="lead">
    <a href="/" class="flex gap-2 items-center">
      <img src="logo.png" alt="logo" class="w-9" />
      <span class="text-xl font-bold">Qwiz</span>
    </a>
  </svelte:fragment>

  <nav class="flex gap-8 font-bold">
    <a href="/" class="text-xl btn hover:variant-filled-primary">{home}</a>
    <a href="/about" class="text-xl btn hover:variant-filled-primary">{about}</a>
  </nav>

  <svelte:fragment slot="trail">
    <button on:click={toggleLocale} class="rounded-full btn-icon hover:variant-filled-primary">
      <LanguageSolid class="hover:stroke-2 w-6 h-6" />
    </button>
    <LightSwitch class="hover:variant-filled-primary" />
  </svelte:fragment>
</AppBar>
