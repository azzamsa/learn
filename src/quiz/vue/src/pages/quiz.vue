<script setup lang="ts">
const quizStore = useQuizStore()

// Reset if User retake the Quiz
quizStore.resetQuiz()

quizStore.fetchQuestions()
</script>

<template>
  <div v-if="quizStore.isLoading" class="mt-40 text-center">
    <i-eos-icons:bubble-loading
      class="block p-5 mx-auto w-20 h-20 rounded-full bg-ghost text-primary"
    />
    <h1 class="mt-4 font-semibold">
      {{ $t('quiz.fetching') }}
    </h1>
  </div>

  <div v-if="!quizStore.isLoading">
    <QuestionStatus />
    <QuestionBox />
  </div>
</template>
