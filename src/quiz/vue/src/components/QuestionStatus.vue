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
    class="py-2 mx-4 mt-4 font-bold text-center rounded-lg border-4 border shadow-xl border-primary bg-base-100"
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
