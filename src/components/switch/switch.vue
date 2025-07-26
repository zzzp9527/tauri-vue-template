<template>
  <label
    :class="[
      'relative inline-flex items-center',
      props.disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer',
      $attrs.class || '',
    ]"
  >
    <!-- 1. 隐藏的原生 checkbox，它负责所有状态逻辑 -->
    <input
      class="sr-only peer"
      v-bind="$attrs"
      type="checkbox"
      :checked="props.value"
      :disabled="props.disabled"
      @change="handleChange"
    />

    <!-- 2. 开关的轨道 (背景) -->
    <div
      :class="[
        'rounded-full transition-colors duration-200',
        'peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-offset-2 peer-focus:ring-blue-500',
        'peer-checked:bg-blue-600 bg-gray-300',
        sizes[props.size].track,
      ]"
    ></div>

    <!-- 3. 开关的滑块 (圆形按钮) -->
    <span
      :class="[
        'absolute bg-white rounded-full shadow transition-transform duration-200 ease-in-out',
        'peer-checked:translate-x-full',
        sizes[props.size].thumb,
        sizes[props.size].position,
      ]"
    ></span>

    <!-- 可选的标签文本 -->
    <span v-if="$slots.default" class="ml-3 select-none">
      <slot />
    </span>
  </label>
</template>

<script setup lang="ts">
defineOptions({
  name: "ZSwitch",
  inheritAttrs: false,
});

type SwitchSize = "sm" | "md" | "lg";

// Props 接口已更新
interface OSwitchProps {
  value?: boolean; // 从 modelValue 变为 value，并设为可选以使用默认值
  disabled?: boolean;
  size?: SwitchSize;
}

const props = withDefaults(defineProps<OSwitchProps>(), {
  value: false, // 为 value 提供默认值
  disabled: false,
  size: "md",
});

// Emits 定义已更新，使用更类型安全的方式
const emit = defineEmits<{
  (e: "change", value: boolean): void; // 定义 change 事件和其负载类型
}>();

// 用于处理 input 变化的函数
function handleChange(event: Event) {
  // 确保事件目标是 HTMLInputElement
  const target = event.target as HTMLInputElement;
  // 触发 change 事件并传递新的布尔值
  emit("change", target.checked);
}

const sizes = {
  sm: {
    track: "w-9 h-5",
    thumb: "w-4 h-4",
    position: "left-[2px] top-[2px]",
  },
  md: {
    track: "w-11 h-6",
    thumb: "w-5 h-5",
    position: "left-0.5 top-0.5",
  },
  lg: {
    track: "w-14 h-8",
    thumb: "w-7 h-7",
    position: "left-0.5 top-0.5",
  },
};
</script>
