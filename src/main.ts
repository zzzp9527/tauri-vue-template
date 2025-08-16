import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "./index.css";
import { useSettingsStore } from "./stores/useSettingsStore";

async function init() {
  const app = createApp(App);
  app.use(createPinia());

  // 初始化配置
  const settingsStore = useSettingsStore();
  await settingsStore.initSettings();

  app.mount("#app");
}

init();
