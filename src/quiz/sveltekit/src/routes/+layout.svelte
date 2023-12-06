<script lang="ts">
  import { Drawer } from "@skeletonlabs/skeleton"

  import NavBar from "$components/NavBar.svelte"
  import { autoModeWatcher } from "@skeletonlabs/skeleton"

  import { AppBar, LightSwitch } from "@skeletonlabs/skeleton"
  import "../app.postcss"

  import { initializeStores } from "@skeletonlabs/skeleton"
  initializeStores()

  import * as m from "$paraglide/messages"

  let home = m.home()
  let about = m.about()
  $: {
    home = m.home()
    about = m.about()
  }
</script>

<svelte:head>
  {@html `<script>${autoModeWatcher.toString()} autoModeWatcher();</script>`}
</svelte:head>

<Drawer class="w-3/4 h-1/2 sm:hidden">
  <div class="p-4">
    <!-- Menu -->
    <h2 class="h2 font-serif">Menu</h2>
    <ul class="list font-serif">
      <li><a href="/" class="text-xl btn hover:variant-filled-primary">{home}</a></li>
      <li><a href="/about" class="text-xl btn hover:variant-filled-primary">{about}</a></li>
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
