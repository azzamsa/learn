<script setup lang="ts">
import HeartIcon from "./icons/IconHeart.vue";
</script>

<template>
  <section class="mx-auto flex max-w-lg justify-center font-medium text-white">
    <a
      class="transform duration-500 hover:-translate-y-1 hover:scale-125 hover:underline"
      href="https://azzamsa.com/support/"
      target="_blank"
    >
      Support Me
      <i class="mx-1 inline-block h-4 w-4 pt-0.5"> <HeartIcon /> </i>
    </a>

    <span class="mx-3">•</span>

    <a
      class="hover:underline"
      href="https://github.com/azzamsa/caesars"
      target="_blank"
    >
      Meta
    </a>
  </section>
</template>
