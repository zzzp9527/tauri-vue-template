import { computed, ref } from "vue"
import { useSettingsStore } from "@/stores/useSettingsStore"
import type { GlobalSettings } from "@/type";
import { THEME_ATTR_KEY } from "@/constants";

type Theme = GlobalSettings['theme']

export function useTheme() {

  const settingsStore = useSettingsStore()
  const updateSettingsField = settingsStore.updateSettingsField

  const theme = ref<Theme>(settingsStore.settings.theme);

  const isDarkMode = computed(() => theme.value === 'dark')

  const initTheme = () => {
    theme.value = settingsStore.settings.theme
    document.documentElement.setAttribute(THEME_ATTR_KEY, theme.value)
  }

  const switchTheme = (checked: boolean) => {
    const newTheme = checked ? 'dark' : 'light'
    theme.value = newTheme
    document.documentElement.setAttribute(THEME_ATTR_KEY, newTheme)
    updateSettingsField('theme', newTheme)
  }

  return {
    theme,
    isDarkMode,
    initTheme,
    switchTheme,
  }
}
