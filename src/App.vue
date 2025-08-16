<script setup lang="ts">
import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";
import ZButton from "./components/button/z-button.vue";
import ZSwitch from "./components/switch/z-switch.vue";
import ZConfig from "./components/config/z-config.vue";
import { ref, inject } from "vue";
import { GlobalSettings } from "./type";

import { invoke } from "@tauri-apps/api/core";

// TODO 使用 pinia 管理状态
const settings = inject<GlobalSettings>("settings");

const isDarkMode = ref(settings?.theme === "dark");
const isAutoStart = ref(settings?.auto_start);

const updateSetting = async (key: string, value: any) => {
  await invoke("set_global_settings_command", { key, value });
};

const fetchSettingInfo = () => {
  switchMode(isDarkMode.value);
};

const switchMode = (value: boolean) => {
  document.documentElement.setAttribute("data-theme", value ? "dark" : "light");
  isDarkMode.value = value;
  updateSetting("theme", value ? "dark" : "light");
};

const switchAutoStart = (value: boolean) => {
  isAutoStart.value = value;
  updateSetting("auto_start", value);
};

const triggerLog = () => {
  info("Hello, Tauri!");
  debug("This is a debug message");
  trace("This is a trace message");
  warn("This is a warn message");
  error("This is an error message");
};

fetchSettingInfo();
</script>

<template>
  <div class="bg-info h-screen w-screen p-4">
    <header class="text-primary-text text-2xl font-bold p-2">设置</header>
    <div class="flex flex-col gap-4">
      <div class="bg-info p-3 rounded-sm shadow-lg flex flex-col gap-2">
        <z-config label="夜间模式">
          <z-switch :value="isDarkMode" @change="switchMode" />
        </z-config>
        <z-config label="开机自启">
          <z-switch :value="isAutoStart" @change="switchAutoStart" />
        </z-config>
      </div>
      <div class="bg-info p-3 rounded-sm shadow-lg flex flex-col">
        <z-config>
          <z-button variant="text" @click="triggerLog"> 触发日志 </z-button>
        </z-config>
      </div>
    </div>
  </div>
</template>

<style>
/* All styles are now handled by Tailwind CSS */
</style>
