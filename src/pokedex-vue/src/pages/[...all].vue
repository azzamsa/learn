<script setup lang="ts">
const { t } = useI18n()
</script>

<template>
  <div class="mt-40 text-center">
    <i-tabler:mood-sad
      class="mx-auto block h-20 w-20 rounded-full bg-secondary p-5"
    />
    <h1 class="mt-4 font-semibold">
      {{ t('not-found') }}
    </h1>
  </div>
</template>
