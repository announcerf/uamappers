<script setup lang="ts">
import type { TypographyProps } from '@/shared/ui/typography/model/types';
import { useTypography } from '@/shared/ui/typography/model/use-typography';

const props = withDefaults(defineProps<TypographyProps>(), {
  as: 'p',
  size: 'md',
  weight: 'light',
  align: 'left',
  transform: 'normal',
});
const { className } = useTypography(props);
</script>

<template>
  <component :is="props.as" :class="className">
    <slot />
  </component>
</template>
