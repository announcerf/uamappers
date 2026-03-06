<script setup lang="ts">
import type { SeparatorProps } from '@/shared/ui/separator/model/types';
import { useSeparator } from '@/shared/ui/separator/model/useSeparator';

const props = withDefaults(defineProps<SeparatorProps>(), {
  orientation: 'horizontal',
  width: '100%',
  height: '100%',
});

const { className, style } = useSeparator(props);
</script>

<template>
  <span
    :class="className"
    :style="style"
    role="separator"
    :aria-orientation="props.orientation"
  />
</template>
