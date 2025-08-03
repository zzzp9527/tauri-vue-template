<template>
  <button :disabled="disabled || loading" :class="computedClass" v-bind="$attrs">
    <span
      v-if="loading"
      class="mr-2 animate-spin border-2 border-current border-r-transparent rounded-full w-4 h-4 inline-block align-middle"
    ></span>
    <slot />
  </button>
</template>

<script setup lang="ts">
defineOptions({ name: "ZButton" });
import { computed } from "vue";

type ButtonVariant = "default" | "text" | "primary" | "danger" | "info";

type ButtonSize = "sm" | "md" | "lg";

interface OButtonProps {
  variant?: ButtonVariant;
  size?: ButtonSize;
  disabled?: boolean;
  loading?: boolean;
}

const props = withDefaults(defineProps<OButtonProps>(), {
  size: "md",
  variant: "default",
  disabled: false,
  loading: false,
});

const sizeClasses = {
  sm: "px-3 py-1 text-sm",
  md: "px-4 py-2 text-base",
  lg: "px-6 py-3 text-lg",
};

const variantClasses = {
  default: "bg-base text-primary-text",
  text: "bg-transparent text-primary-text",
  primary: "bg-primary text-primary-text",
  danger: "bg-danger text-primary-text",
  info: "bg-info text-primary-text",
};

const disabledClasses = "opacity-50 cursor-not-allowed";

const baseClasses = "inline-flex items-center justify-center cursor-pointer";

const computedClass = computed(() => [
  baseClasses,
  sizeClasses[props.size],
  variantClasses[props.variant],
  props.disabled || props.loading ? disabledClasses : "",
  // 允许用户传入自定义 class
]);
</script>
