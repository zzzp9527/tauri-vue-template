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

type ButtonSize = "sm" | "md" | "lg";

type ButtonVariant = "primary" | "secondary" | "danger" | "outline";

interface OButtonProps {
  size?: ButtonSize;
  variant?: ButtonVariant;
  disabled?: boolean;
  loading?: boolean;
}

const props = withDefaults(defineProps<OButtonProps>(), {
  size: "md",
  variant: "primary",
  disabled: false,
  loading: false,
});

const sizeClasses = {
  sm: "px-3 py-1 text-sm",
  md: "px-4 py-2 text-base",
  lg: "px-6 py-3 text-lg",
};

const variantClasses = {
  primary: "bg-blue-600 text-grey-500 hover:bg-blue-700",
  secondary: "bg-gray-200 text-gray-800 hover:bg-gray-300",
  danger: "bg-red-600 text-white hover:bg-red-700",
  outline: "bg-transparent border border-blue-600 text-blue-600 hover:bg-blue-50",
};

const disabledClasses = "opacity-50 cursor-not-allowed";

const computedClass = computed(() => [
  "inline-flex items-center justify-center rounded transition-colors duration-150 font-medium focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2",
  sizeClasses[props.size],
  variantClasses[props.variant],
  props.disabled || props.loading ? disabledClasses : "",
  // 允许用户传入自定义 class
  // (会自动合并，因为 $attrs.class 也会挂到根节点)
]);
</script>
