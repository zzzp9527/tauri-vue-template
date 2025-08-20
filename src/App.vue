<script setup lang="ts">
import { useTheme } from "./hooks/useTheme";
import IndexPage from "./pages/index-page.vue";

const { initTheme } = useTheme()

initTheme()

</script>

<template>
  <div class="h-screen w-screen">
    <index-page />
  </div>
</template>
