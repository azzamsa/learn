<script lang="ts">
  import "../app.postcss"
  import { autoModeWatcher } from "@skeletonlabs/skeleton"
  import NavBar from "$components/NavBar.svelte"
</script>

<svelte:head>
  {@html `<script>${autoModeWatcher.toString()} autoModeWatcher();</script>`}
</svelte:head>

<div>
  <NavBar />
  <slot />
</div>
