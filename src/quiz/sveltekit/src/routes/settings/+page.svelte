<script lang="ts">
  import { difficulty, numberOfQuestions } from "$stores/quiz"
  import Arrow from "virtual:icons/heroicons/arrow-small-right-20-solid"

  function updateDifficulty(value: string) {
    $difficulty = value
  }
  function updateNumberOfQuestions(value: string) {
    $numberOfQuestions = value
  }
</script>

<svelte:head>
  <title>Qwiz - Settings</title>
</svelte:head>

<section class="font-sans p-20 space-y-8">
  <!-- Header -->
  <header class="space-y-2">
    <h2 class="font-serif font-bold h2">Settings</h2>
    <p class="font-sans text-xl">Configure the Qwiz</p>
  </header>
  <!-- Body -->
  <div class="flex flex-col gap-8 p-20 text-center rounded-xl bg-surface-100 px-40">
    <!-- Difficulty Levels -->
    <div>
      <label class="label">
        <span>Difficulty Levels</span>

        <select
          bind:value={$difficulty}
          on:change={(e) => updateDifficulty(e.currentTarget.value)}
          class="text-center select"
        >
          <option value="easy">Easy</option>
          <option value="medium">Medium</option>
          <option value="hard">Hard</option>
        </select>
      </label>
    </div>
    <!-- Number of Questions -->
    <div>
      <label class="label">
        <span>Number of Questions</span>
        <select
          bind:value={$numberOfQuestions}
          on:change={(e) => updateNumberOfQuestions(e.currentTarget.value)}
          class="text-center select"
        >
          <option value="10">10</option>
          <option value="20">20</option>
          <option value="30">30</option>
        </select>
      </label>
    </div>
  </div>
  <!-- butons -->
  <a href="/quiz" class="btn variant-filled-primary">
    <span>Start</span>
    <i><Arrow /></i>
  </a>
</section>
