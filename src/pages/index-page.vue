<script setup lang="ts">
defineOptions({ name: "IndexPage" });

import { onMounted, ref } from "vue";
import ZButton from "@/components/button/z-button.vue";
import ZSwitch from "@/components/switch/z-switch.vue";
import ZConfig from "@/components/config/z-config.vue";
import { useSettingsStore } from "@/stores/useSettingsStore";
import { useLogger } from "@/hooks/useLogger";
import { useTheme } from "@/hooks/useTheme";
import { getVersion } from '@tauri-apps/api/app'

const logger = useLogger();
const { isDarkMode, switchTheme } = useTheme();

const settingsStore = useSettingsStore();

const getAutostartStatus = settingsStore.getAutostartStatus;
const setAutostartStatus = settingsStore.setAutostartStatus;

const version = ref('');
const isAutoStart = ref(false);

const initAutoStart = async () => {
  isAutoStart.value = await getAutostartStatus();
};

const switchAutoStart = (value: boolean) => {
  isAutoStart.value = value;
  setAutostartStatus(value)
};

const triggerLog = () => {
  logger.info("Hello, Tauri!");
  logger.debug("This is a debug message");
  logger.trace("This is a trace message");
  logger.warn("This is a warn message");
  logger.error("This is an error message");
};

onMounted(async () => {
  version.value = await getVersion();
  logger.info(`当前版本：${version.value}`);
})

const init = () => {
  initAutoStart();
};

init();
</script>

<template>
  <div class="bg-info w-full h-full p-4">
    <header class="text-primary-text text-2xl font-bold p-2">设置</header>
    <div class="flex flex-col gap-4">
      <div class="config-card">
        <z-config label="夜间模式">
          <z-switch :value="isDarkMode" @change="switchTheme" />
        </z-config>
        <z-config label="开机自启">
          <z-switch :value="isAutoStart" @change="switchAutoStart" />
        </z-config>
      </div>
      <div class="config-card">
        <z-config>
          <z-button variant="text" @click="triggerLog"> 触发日志 </z-button>
        </z-config>
      </div>
      <div class="config-card">
        <z-config>
          <div>当前版本：<span class=" text-purple-400">{{ version }}</span></div>
        </z-config>
      </div>
    </div>
  </div>
</template>
