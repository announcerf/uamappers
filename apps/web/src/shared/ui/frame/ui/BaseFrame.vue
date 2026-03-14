<script setup lang="ts">
import { useAttrs } from 'vue';

import { frameProps } from '@/shared/ui/frame/model/props';
import { useFrame } from '@/shared/ui/frame/model/useFrame';

defineOptions({
  inheritAttrs: false,
});

const props = defineProps(frameProps);
const attrs = useAttrs();

const { className, contentClassName, accentLineClassName } = useFrame(props);
</script>

<template>
  <component
    :is="props.as"
    v-bind="attrs"
    :class="className"
  >
    <span
      aria-hidden="true"
      :class="accentLineClassName"
    />
    <div :class="contentClassName">
      <slot />
    </div>
  </component>
</template>
