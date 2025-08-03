<template>
  <div class="flex justify-between">
    <label v-if="label" :style="{ ...labelStyle }" class="text-primary-text font-normal">{{ label }}</label>
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
defineOptions({ name: "ZConfig" });

import type { CSSProperties } from "vue";

interface OConfigProps {
  label?: string;
  labelStyle?: CSSProperties;
}

withDefaults(defineProps<OConfigProps>(), {
  label: "",
  labelStyle: () => ({}),
});
</script>
