<script lang="ts">
  import "../app.css";
  import Nav from "../components/NavBar.svelte";

  export let searchTerm = "";
</script>

<div class="mx-auto my-5 max-w-6xl p-2">
  <Nav {searchTerm} />
  <slot />
</div>
