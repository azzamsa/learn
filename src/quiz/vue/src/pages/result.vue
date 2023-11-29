<script setup lang="ts">
import { ref } from 'vue'

const quizStore = useQuizStore()

const totalQuestions = ref(0)
const correctAnswer = ref(0)
const difficultyLevel = ref('')

totalQuestions.value = quizStore.getTotalQuestions
correctAnswer.value = quizStore.getCorrectAnswer
difficultyLevel.value = quizStore.getDifficultyLevel

const resultLevel = computed(() => {
  const middle = Math.ceil((totalQuestions.value + 1) / 2)
  if (correctAnswer.value > middle)
    return 'great'
  else if (correctAnswer.value === middle)
    return 'soso'
  else
    return 'bad'
})

watch(quizStore, () => {
  totalQuestions.value = quizStore.getTotalQuestions
  correctAnswer.value = quizStore.getCorrectAnswer
  difficultyLevel.value = quizStore.getDifficultyLevel
})
</script>

<template>
  <div class="mt-20 hero">
    <div class="text-center hero-content">
      <div class="max-w-md">
        <div />
        <h1 class="text-5xl font-bold">
          {{ correctAnswer }}/{{ totalQuestions }}
          <i-noto:party-popper
            v-if="resultLevel === 'great'"
            class="inline-block w-10 h-10"
          />
          <i-noto:mechanical-arm
            v-else-if="resultLevel === 'soso'"
            class="inline-block w-10 h-10"
          />
          <i-noto:sad-but-relieved-face v-else class="inline-block w-10 h-10" />
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

        <RouterLink class="btn-primary btn" to="/quiz">
          {{ $t('result.retake') }}
        </RouterLink>
      </div>
    </div>
  </div>
</template>
