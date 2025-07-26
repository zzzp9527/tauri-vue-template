<script setup lang="ts">
import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";
import ZButton from "./components/button/button.vue";
import ZSwitch from "./components/switch/switch.vue";
import { ref } from "vue";

const isDarkMode = ref(false);

const fetchSettingInfo = () => {
  const isDark = true;

  isDarkMode.value = isDark;
};

const switchMode = (value: boolean) => {
  document.documentElement.setAttribute("data-theme", value ? "dark" : "light");
};

fetchSettingInfo();

function log() {
  info("Hello, Tauri!");
  debug("This is a debug message");
  trace("This is a trace message");
  warn("This is a warn message");
  error("This is an error message");
}
</script>

<template>
  <main class="h-screen bg-primary text-black flex items-center justify-center">
    <z-button> component </z-button>
    <button
      type="button"
      class="cursor-pointer bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-3 px-6 rounded-lg shadow-lg transform hover:scale-105 transition-transform duration-300"
      @click="log"
    >
      Test Log
    </button>
    <z-switch :value="isDarkMode" @change="switchMode" />
  </main>
</template>

<style>
/* All styles are now handled by Tailwind CSS */
</style>
