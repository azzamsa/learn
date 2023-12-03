<script lang="ts">
  import { AppBar, LightSwitch } from "@skeletonlabs/skeleton"
  import LanguageSolid from "virtual:icons/heroicons/language-solid"
  import * as m from "$paraglide/messages"
  import { setLanguageTag, availableLanguageTags, languageTag } from "$paraglide/runtime"
  import { currentLocale as currentLocaleStore } from "$stores/locale"

  // Locale
  function toggleLocale() {
    const locales = availableLanguageTags
    $currentLocaleStore = locales[(locales.indexOf(languageTag()) + 1) % locales.length]
  }

  let home = m.home()
  let about = m.about()
  $: {
    setLanguageTag($currentLocaleStore)
    home = m.home()
    about = m.about()
  }
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
    <a href="/" class="btn hover:variant-filled-primary">{home}</a>
    <a href="/about" class="btn hover:variant-filled-primary">{about}</a>
  </nav>

  <svelte:fragment slot="trail">
    <button on:click={toggleLocale} class="btn-icon rounded-full hover:variant-filled-primary">
      <LanguageSolid class=" hover:stroke-2 " />
    </button>
    <LightSwitch class="hover:variant-filled-primary" />
  </svelte:fragment>
</AppBar>
