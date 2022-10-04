<script setup lang="ts">
const quizStore = useQuizStore()

const showCustomize = ref(false)
const difficultyLevelChoice = ref([
  { id: 'easy', desc: 'index.level.easy' },
  { id: 'medium', desc: 'index.level.medium' },
  { id: 'hard', desc: 'index.level.hard' },
])
const questionLimitChoice = ref([10, 20, 30, 40, 50])

function setDifficulty(event: any) {
  quizStore.setDifficultyLevel(event.target.value)
}

function setLimit(event: any) {
  quizStore.setQuestionLimit(event.target.value)
}
</script>

<template>
  <div class="hero mt-20">
    <div class="hero-content text-center">
      <div class="max-w-md">
        <h1 class="text-5xl font-bold">
          {{ $t('index.hi') }}
          <i-ph:hand-waving class="inline-block h-10 w-10 animate-bounce" />
        </h1>
        <p class="py-6">
          {{ $t('index.intro') }}
        </p>

        <div class="flex justify-center space-x-4">
          <button
            class="btn btn-secondary"
            @click="showCustomize = !showCustomize"
          >
            {{ $t('index.customize') }}
          </button>

          <RouterLink class="btn btn-primary" to="/quiz">
            {{ $t('index.start') }}
          </RouterLink>
        </div>

        <div v-if="showCustomize">
          <div class="divider mt-2 mb-2"></div>

          <div class="mt-4 flex flex-col items-center space-x-4 space-y-4">
            <select class="select select-secondary" @change="setDifficulty">
              <option disabled selected>
                {{ $t('index.difficultyLevel') }}:
              </option>
              <option
                v-for="level in difficultyLevelChoice"
                :key="level.id"
                :value="level.id"
              >
                {{ $t(level.desc) }}
              </option>
            </select>

            <select class="select select-secondary" @change="setLimit">
              <option disabled selected>
                {{ $t('index.questionLimit') }}:
              </option>
              <option v-for="limit in questionLimitChoice" :key="limit">
                {{ limit }}
              </option>
            </select>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
