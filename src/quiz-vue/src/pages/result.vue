<script setup lang="ts">
const quizStore = useQuizStore()
quizStore.fetchQuestions()

let totalQuestions = $ref(0)
let correctAnswer = $ref(0)
let difficultyLevel = $ref('')

totalQuestions = quizStore.getTotalQuestions
correctAnswer = quizStore.getCorrectAnswer
difficultyLevel = quizStore.getDifficultyLevel

const resultLevel = computed(() => {
  const middle = Math.ceil((totalQuestions + 1) / 2)
  if (correctAnswer > middle) {
    return 'great'
  } else if (correctAnswer === middle) {
    return 'soso'
  } else {
    return 'bad'
  }
})

watch(quizStore, () => {
  totalQuestions = quizStore.getTotalQuestions
  correctAnswer = quizStore.getCorrectAnswer
  difficultyLevel = quizStore.getDifficultyLevel
})
</script>

<template>
  <div class="hero mt-20">
    <div class="hero-content text-center">
      <div class="max-w-md">
        <div></div>
        <h1 class="text-5xl font-bold">
          {{ correctAnswer }}/{{ totalQuestions }}
          <i-noto:party-popper
            v-if="resultLevel === 'great'"
            class="inline-block h-10 w-10"
          />
          <i-noto:mechanical-arm
            v-else-if="resultLevel === 'soso'"
            class="inline-block h-10 w-10"
          />
          <i-noto:sad-but-relieved-face v-else class="inline-block h-10 w-10" />
        </h1>

        <div class="mt-2">
          <p>{{ $t('index.difficultyLevel') }}: {{ difficultyLevel }}</p>
          <p>{{ $t('result.totalQuestion') }}: {{ totalQuestions }}</p>
        </div>

        <div class="py-6 font-bold">
          <p v-if="resultLevel === 'great'">
            {{ $t('result.desc.great') }}
          </p>
          <p v-else-if="resultLevel === 'soso'">
            {{ $t('result.desc.soso') }}
          </p>
          <p v-else>
            {{ $t('result.desc.bad') }}
          </p>
        </div>

        <RouterLink class="btn btn-primary" to="/quiz">
          {{ $t('result.retake') }}
        </RouterLink>
      </div>
    </div>
  </div>
</template>
