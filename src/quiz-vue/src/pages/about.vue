<script setup lang="ts">
useHead({
  title: 'Quiz - About',
})
</script>

<template>
  <div class="mx-8">
    <p class="mt-4 text-justify" v-html="$t('about.intro')"></p>
  </div>
</template>

<style scoped>
a {
  @apply link link-primary;
}
</style>
