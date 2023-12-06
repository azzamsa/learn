<script lang="ts">
  import { Drawer } from "@skeletonlabs/skeleton"
  import { autoModeWatcher } from "@skeletonlabs/skeleton"
  import { AppBar, LightSwitch } from "@skeletonlabs/skeleton"
  import { initializeStores } from "@skeletonlabs/skeleton"
  import "../app.postcss"

  import * as m from "$paraglide/messages"
  import { page } from "$app/stores"
  import { setLanguageTag } from "$paraglide/runtime"
  import { currentLocale } from "$stores/locale"

  import NavBar from "$components/NavBar.svelte"

  // Drawer
  initializeStores()

  // Locale
  $: {
    setLanguageTag($currentLocale)
  }
</script>

<svelte:head>
  {@html `<script>${autoModeWatcher.toString()} autoModeWatcher();</script>`}
</svelte:head>

{#key $currentLocale}
  <Drawer class="w-3/4 h-1/2 sm:hidden">
    <div class="p-4">
      <!-- Menu -->
      <h2 class="h2 font-serif">Menu</h2>
      <ul class="list font-serif">
        <li><a href="/" class="text-xl btn hover:variant-filled-primary">{m.home()}</a></li>
        <li><a href="/about" class="text-xl btn hover:variant-filled-primary">{m.about()}</a></li>
      </ul>
      <!-- Settings -->
      <h2 class="h2 pt-2 font-serif">Settings</h2>
      <ul class="list font-serif mt-4">
        <li class="px-4"><LightSwitch class="hover:variant-filled-primary" /></li>
      </ul>
    </div>
  </Drawer>

  <div>
    <NavBar />
    <slot />
  </div>
{/key}
