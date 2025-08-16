<script setup lang="ts">
import { useTheme } from "./hooks/useTheme";
import IndexPage from "./pages/index-page.vue";

const { initTheme } = useTheme()

initTheme()

</script>

<template>
  <div class="bg-info h-screen w-screen p-4">
    <index-page />
  </div>
</template>
