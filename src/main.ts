import { createApp } from "vue";
import App from "./App.vue";
import "./index.css";
import { invoke } from "@tauri-apps/api/core";
import { GlobalSettings } from "./type";

async function init() {
  let settings: GlobalSettings = {
    theme: "light",
    auto_start: false,
  };

  try {
    settings = await invoke<GlobalSettings>("get_settings_command");
  } catch (error) {
    console.error("Failed to initialize app:", error);
  } finally {
    const app = createApp(App);

    app.provide("settings", settings);

    app.mount("#app");
  }
}

init();
