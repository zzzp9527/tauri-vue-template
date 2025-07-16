<script setup lang="ts">
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log'

function log() {
  info('Hello, Tauri!')
  debug('This is a debug message')
  trace('This is a trace message')
  warn('This is a warn message')
  error('This is an error message')
}
</script>

<template>
  <main class="h-screen bg-gray-900 flex items-center justify-center">
    <button
      type="button"
      class=" cursor-pointer bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-3 px-6 rounded-lg shadow-lg transform hover:scale-105 transition-transform duration-300"
      @click="log"
    >
      Test Log
    </button>
  </main>
</template>

<style>
/* All styles are now handled by Tailwind CSS */
</style>
