<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<template>
  <div class="mx-4 mx-auto mt-4">
    <header>
      <NavBar />
    </header>

    <RouterView />
  </div>
</template>
