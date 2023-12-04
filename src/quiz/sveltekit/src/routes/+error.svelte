<script lang="ts">
  import { page } from "$app/stores"
</script>

<section class="h-screen flex justify-center items-center">
  <div class="text-center space-y-4">
    <h2 class="h2">
      {$page.status}:{#if $page.error}
        {$page.error.message}
      {/if}
    </h2>
    <p>We're sorry, something went wrong.</p>
  </div>
</section>
