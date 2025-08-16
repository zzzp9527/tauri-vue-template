<script setup lang="ts">
defineOptions({ name: "IndexPage" });

import { ref } from "vue";
import ZButton from "@/components/button/z-button.vue";
import ZSwitch from "@/components/switch/z-switch.vue";
import ZConfig from "@/components/config/z-config.vue";
import { useSettingsStore } from "@/stores/useSettingsStore";
import { useLogger } from "@/hooks/useLogger";
import { useTheme } from "@/hooks/useTheme";

const logger = useLogger()
const { isDarkMode, switchTheme } = useTheme()

const settingsStore = useSettingsStore()

const settings = settingsStore.settings
const updateSettingsField = settingsStore.updateSettingsField

const isAutoStart = ref(settings?.auto_start);

const switchAutoStart = (value: boolean) => {
  isAutoStart.value = value;
  updateSettingsField("auto_start", value);
};

const triggerLog = () => {
  logger.info("Hello, Tauri!");
  logger.debug("This is a debug message");
  logger.trace("This is a trace message");
  logger.warn("This is a warn message");
  logger.error("This is an error message");
};

</script>

<template>
  <div class="bg-info h-screen w-screen p-4">
    <header class="text-primary-text text-2xl font-bold p-2">设置</header>
    <div class="flex flex-col gap-4">
      <div class="bg-info p-3 rounded-sm shadow-lg flex flex-col gap-2">
        <z-config label="夜间模式">
          <z-switch :value="isDarkMode" @change="switchTheme" />
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
