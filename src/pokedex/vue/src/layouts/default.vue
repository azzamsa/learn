<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<template>
  <div class="mx-auto mx-4 mt-4">
    <header>
      <NavBar />
    </header>

    <RouterView />
  </div>
</template>
