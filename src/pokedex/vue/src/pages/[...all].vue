<script setup lang="ts">
const { t } = useI18n()
</script>

<template>
  <div class="mt-40 text-center">
    <i-tabler:mood-sad
      class="block p-5 mx-auto w-20 h-20 rounded-full bg-secondary"
    />
    <h1 class="mt-4 font-semibold">
      {{ t('not-found') }}
    </h1>
  </div>
</template>
