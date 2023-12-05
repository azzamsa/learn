<script lang="ts">
  import { page } from "$app/stores"
  import CoffeeDoodle from "$root/components/icons/CoffeeDoodle.svelte"
</script>

<section class="flex flex-col justify-center items-center py-10 px-4 space-4">
  <h1 class="py-2 font-serif h1 animate-[bounce_4s_ease-in-out_infinite]">{$page.status}</h1>
  <div class="p-5 w-80">
    <CoffeeDoodle />
  </div>
  <div class="text-center">
    <h2 class="font-serif h2">Uh-oh! Is anyone out there?!?</h2>
    <p class="text-lg">
      Don't panic! We're on the hunt for the missing page – it might be hiding in the binary bushes.
    </p>
  </div>
</section>
