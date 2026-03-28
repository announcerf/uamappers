<script setup lang="ts">
import { type ButtonProps, Frame, Typography } from '@/shared/ui';
import { useButton } from '@/shared/ui/button/model/useButton';

const props = withDefaults(defineProps<ButtonProps>(), {
  variant: 'default',
});
const { className } = useButton(props);
</script>

<template>
  <Frame as="button" :variant="props.variant" :class="className">
    <Typography size="xs" weight="medium">
      <slot />
    </Typography>
  </Frame>
</template>
