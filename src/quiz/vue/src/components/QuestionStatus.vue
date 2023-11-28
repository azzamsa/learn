<script setup lang="ts">
import { ref } from 'vue'

const quizStore = useQuizStore()

const correctAnswer = ref(0)
const totalQuestions = ref(0)
const currentQuestionIndex = ref(0)

totalQuestions.value = quizStore.getTotalQuestions
correctAnswer.value = quizStore.getCorrectAnswer
currentQuestionIndex.value = quizStore.getCurrentQuestionIndex

watch(quizStore, () => {
  correctAnswer.value = quizStore.getCorrectAnswer
  totalQuestions.value = quizStore.getTotalQuestions
  currentQuestionIndex.value = quizStore.getCurrentQuestionIndex
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
