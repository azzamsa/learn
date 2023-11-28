import { defineStore } from 'pinia'
import type { Question } from '../types/Question'

export const useQuizStore = defineStore({
  id: 'quiz',
  state: () => ({
    questions: [] as Question[] | Question[],
    currentQuestionIndex: 0,
    loading: false,
    correctAnswer: 0,
    // Question limit to Fetch
    questionLimit: 10,
    // Question Level to Fetch
    difficultyLevel: 'Easy',
  }),
  getters: {
    isLoading(state) {
      return state.loading
    },
    getCurrentQuestion(state) {
      return state.questions[state.currentQuestionIndex]
    },
    getTotalQuestions(state) {
      return state.questions.length
    },
    getCorrectAnswer(state) {
      return state.correctAnswer
    },
    getCurrentQuestionIndex(state) {
      return state.currentQuestionIndex
    },
    isFinalQuestion(state) {
      if (state.currentQuestionIndex === state.questions.length - 1)
        return true
      else
        return false
    },
    getDifficultyLevel(state) {
      const level = state.difficultyLevel
      return level.charAt(0).toUpperCase() + level.slice(1)
    },
  },
  actions: {
    incrementCorrectAnswer() {
      this.$state.correctAnswer++
    },
    nextQuestion() {
      this.$state.currentQuestionIndex++
    },
    setQuestionLimit(limit: number) {
      this.$state.questionLimit = limit
    },
    setDifficultyLevel(level: string) {
      this.$state.difficultyLevel = level
    },
    resetQuiz() {
      this.$state.correctAnswer = 0
      this.$state.currentQuestionIndex = 0
    },
    async fetchQuestions() {
      this.$state.loading = true

      const limit = this.$state.questionLimit
      const level = this.$state.difficultyLevel.toLocaleLowerCase()

      const url = `https://opentdb.com/api.php?amount=${limit}&category=9&difficulty=${level}&type=multiple`
      const res = await fetch(url)
      const data = await res.json()
      this.$state.questions = data.results

      this.$state.loading = false
    },
  },
})
