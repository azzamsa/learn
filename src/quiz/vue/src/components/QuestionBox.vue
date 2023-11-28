<script setup lang="ts">
import { debounce, shuffle } from 'lodash-es'
import type { Question } from '../types/Question'

const quizStore = useQuizStore()

// Current Question
let question: any | Question = $ref('')
// Selected index of questions
const selectedIndex = $ref(0)
// Correct index of questions
let correctIndex = $ref(0)
// Randomize question order
let shuffledAnswers: string[] = $ref([])
// is Final Question
let isFinalQuestion = $ref(false)
// is Final Question
let isCountdown = $ref(false)

function shuffleAnswer() {
  const answers = [...question.incorrect_answers, question.correct_answer]
  shuffledAnswers = shuffle(answers)
  correctIndex = shuffledAnswers.indexOf(question.correct_answer)
}

question = quizStore.getCurrentQuestion
shuffleAnswer()
isFinalQuestion = quizStore.isFinalQuestion

watch(quizStore, () => {
  question = quizStore.getCurrentQuestion
  shuffleAnswer()
  isFinalQuestion = quizStore.isFinalQuestion
})

// Add check mark to correct answwer
function showAnswer() {
  const answers = []
  for (const shuffledAnswer of shuffledAnswers) {
    if (shuffledAnswer === question.correct_answer)
      answers.push(`${shuffledAnswer}&nbsp;&nbsp;&nbsp;` + `✅`)
    else
      answers.push(shuffledAnswer)
  }
  shuffledAnswers = answers
}

function submit_() {
  if (selectedIndex === correctIndex)
    quizStore.incrementCorrectAnswer()

  quizStore.nextQuestion()
  isCountdown = false
}

const submitDebounce = debounce(() => {
  submit_()
}, 800)

function submit() {
  isCountdown = true
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
