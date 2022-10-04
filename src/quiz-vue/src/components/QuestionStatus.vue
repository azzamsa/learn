<script setup lang="ts">
const quizStore = useQuizStore()

let correctAnswer = $ref(0)
let totalQuestions = $ref(0)
let currentQuestionIndex = $ref(0)

totalQuestions = quizStore.getTotalQuestions
correctAnswer = quizStore.getCorrectAnswer
currentQuestionIndex = quizStore.getCurrentQuestionIndex

watch(quizStore, () => {
  correctAnswer = quizStore.getCorrectAnswer
  totalQuestions = quizStore.getTotalQuestions
  currentQuestionIndex = quizStore.getCurrentQuestionIndex
})
</script>

<template>
  <div
    class="mx-4 mt-4 rounded-lg border border-4 border-primary bg-base-100 py-2 text-center font-bold shadow-xl"
  >
    <h1>
      {{ $t('status.current') }}: {{ currentQuestionIndex }} /
      {{ totalQuestions }}
    </h1>
    <h1>
      {{ $t('status.correct') }}: {{ correctAnswer }} / {{ totalQuestions }}
    </h1>
  </div>
</template>
