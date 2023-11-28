<script setup lang="ts">
import { ref } from 'vue'

import { debounce, shuffle } from 'lodash-es'
import type { Question } from '../types/Question'

const quizStore = useQuizStore()

// Current Question
const question: any | Question = ref('')
// Selected index of questions
const selectedIndex = ref(0)
// Correct index of questions
const correctIndex = ref(0)
// Randomize question order
const shuffledAnswers: string[] = ref([])
// is Final Question
const isFinalQuestion = ref(false)
// is Final Question
const isCountdown = ref(false)

function shuffleAnswer() {
  const answers = [...question.value.incorrect_answers, question.value.correct_answer]
  shuffledAnswers.value = shuffle(answers)
  correctIndex.value = shuffledAnswers.value.indexOf(question.value.correct_answer)
}

question.value = quizStore.getCurrentQuestion
shuffleAnswer()
isFinalQuestion.value = quizStore.isFinalQuestion

watch(quizStore, () => {
  question.value = quizStore.getCurrentQuestion
  shuffleAnswer()
  isFinalQuestion.value = quizStore.isFinalQuestion
})

// Add check mark to correct answwer
function showAnswer() {
  const answers = []
  for (const shuffledAnswer of shuffledAnswers.value) {
    if (shuffledAnswer === question.value.correct_answer)
      answers.push(`${shuffledAnswer}&nbsp;&nbsp;&nbsp;` + `✅`)
    else
      answers.push(shuffledAnswer)
  }
  shuffledAnswers.value = answers
}

function submit_() {
  if (selectedIndex.value === correctIndex.value)
    quizStore.incrementCorrectAnswer()

  quizStore.nextQuestion()
  isCountdown.value = false
}

const submitDebounce = debounce(() => {
  submit_()
}, 800)

function submit() {
  isCountdown.value = true
  showAnswer()
  submitDebounce()
}
</script>

<template>
  <div
    class="py-2 mx-4 mt-4 font-bold text-center rounded-lg border-4 border shadow-xl card border-primary bg-base-100"
  >
    <div class="items-center text-center card-body">
      <h2 class="card-title" v-html="question.question" />
      <div class="mt-1 mb-1 divider" />

      <div class="form-control">
        <div
          v-for="(answer, index) in shuffledAnswers"
          :key="index"
          class="flex items-center p-1 space-x-4 rounded"
        >
          <label class="cursor-pointer label">
            <input
              :key="answer"
              type="radio"
              name="radio-6"
              class="radio checked:bg-accent"
              @click="selectedIndex = index"
            >
          </label>
          <p class="text-left label-text" v-html="answer" />
        </div>
      </div>

      <div class="mt-1 mb-1 divider" />
      <div class="card-actions">
        <RouterLink v-if="isFinalQuestion" class="btn-primary btn" to="/result">
          {{ $t('box.finish') }}
        </RouterLink>
        <button
          v-else
          class="btn-primary btn"
          :disabled="isCountdown"
          @click="submit"
        >
          <p v-if="!isCountdown">
            {{ $t('box.submit') }}
          </p>
          <p v-else>
            {{ $t('box.showAnswer') }}
          </p>
        </button>
      </div>
    </div>
  </div>
</template>
