<script lang="ts">
  import { getDrawerStore } from "@skeletonlabs/skeleton"
  import { AppBar, LightSwitch } from "@skeletonlabs/skeleton"

  import * as m from "$paraglide/messages"
  import { availableLanguageTags, languageTag } from "$paraglide/runtime"

  import { currentLocale } from "$stores/locale"
  import Hamburger from "$root/components/icons/Hamburger.svelte"
  import LanguageSolid from "virtual:icons/heroicons/language-solid"

  // Drawer
  const drawerStore = getDrawerStore()
  function toggleDrawer() {
    if ($drawerStore.open === true) {
      drawerStore.close()
    } else {
      drawerStore.open()
    }
  }

  // Locale
  function toggleLocale() {
    const locales = availableLanguageTags
    $currentLocale = locales[(locales.indexOf(languageTag()) + 1) % locales.length]
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
      <span class="hidden text-xl font-bold sm:block">Qwiz</span>
    </a>
  </svelte:fragment>

  <nav class="hidden gap-8 font-bold sm:flex">
    <a href="/" class="text-xl btn hover:variant-filled-primary">{m.home()}</a>
    <a href="/about" class="text-xl btn hover:variant-filled-primary">{m.about()}</a>
  </nav>

  <svelte:fragment slot="trail">
    <!-- big screen  -->
    <div class="hidden sm:inline-flex items-center space-x-2">
      <button on:click={toggleLocale} class="rounded-full btn-icon hover:variant-filled-primary">
        <LanguageSolid class="w-6 h-6 hover:stroke-2" />
      </button>
      <LightSwitch class="hover:variant-filled-primary" />
    </div>
    <!-- small screen  -->
    <div class="sm:hidden">
      <button on:click={toggleDrawer} class="btn">
        <div class="w-8 h-8">
          <Hamburger />
        </div>
      </button>
    </div>
    <!-- small screen  -->
  </svelte:fragment>
</AppBar>
