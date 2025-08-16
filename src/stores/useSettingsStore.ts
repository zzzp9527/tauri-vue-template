import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { GlobalSettings } from '@/type'
import { DEFAULT_SETTINGS } from '@/constants/config'

interface SettingsState {
  settings: GlobalSettings
}

export const useSettingsStore = defineStore('settings', {
  state: (): SettingsState => {
    return {
      settings: DEFAULT_SETTINGS,
    }
  },
  getters: {

  },
  actions: {
    async initSettings() {
      const settings = await invoke<GlobalSettings>('get_global_settings_command')
      this.settings = { ...DEFAULT_SETTINGS, ...settings }
    },
    async updateSettingsField<K extends keyof GlobalSettings>(key: K, value: GlobalSettings[K]) {
      this.settings[key] = value
      await invoke('set_global_settings_command', { key, value })
    }
  }
})
