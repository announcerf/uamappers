<script setup lang="ts">
import type { SeparatorProps } from '@/shared/ui/separator/model/types';
import { useSeparator } from '@/shared/ui/separator/model/useSeparator';

const props = withDefaults(defineProps<SeparatorProps>(), {
  width: '100%',
});

const { className, style } = useSeparator(props);
</script>

<template>
  <div
    :class="className"
    class="'block shrink-0' w-full"
    :style="style"
    role="separator"
  >
    <span class="bg-card mb-0.5 block h-0.75" />
    <span class="bg-outline block h-px" />
  </div>
</template>
