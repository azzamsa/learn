<script lang="ts">
  import SwingingDoodle from "$components/icons/SwingingDoodle.svelte"
  import Arrow from "virtual:icons/heroicons/arrow-small-right-20-solid"
</script>

<svelte:head>
  <title>Qwiz - Home</title>
</svelte:head>

<section class="flex flex-col gap-8 justify-center items-center font-sans">
  <!-- Hero -->
  <div class="flex flex-col gap-2 pt-20">
    <!-- `duration-*` doesn't work, custom attribute does -->
    <img
      src="logo.png"
      alt="Qwiz logo"
      class="w-10 h-10 animate-[bounce_4s_ease-in-out_infinite]"
    />
    <h2 class="h2">
      <span class="font-serif font-bold">Qwiz: Where Learning Meets Play.</span>
    </h2>
    <p class="pt-2 font-sans text-xl">Navigate your knowledge journey with interactive quizzes.</p>
  </div>
  <!-- butons -->
  <div class="flex gap-4">
    <a href="/quiz" class="btn variant-filled-primary">
      <span>Start</span>
      <i><Arrow /></i>
    </a>
    <a href="/settings" class="btn variant-soft-primary">Configure</a>
  </div>
  <div class="p-5 w-80">
    <SwingingDoodle />
  </div>
</section>
