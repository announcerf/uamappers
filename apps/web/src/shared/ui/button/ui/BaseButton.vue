<script setup lang="ts">
import { type ButtonProps, Frame, Typography } from '@/shared/ui';
import { useButton } from '@/shared/ui/button/model/useButton';

const props = withDefaults(defineProps<ButtonProps>(), {
  variant: 'default',
  hasIcon: false,
});
const { className, iconComponent } = useButton(props);
</script>

<template>
  <Frame as="button" :variant="props.variant" :class="className">
    <div class="flex items-center gap-1">
      <template v-if="iconComponent">
        <component
          :is="iconComponent"
          :size="16"
          class="shrink-0"
          aria-hidden="true"
        />
        <span aria-hidden="true" class="h-4 w-px shrink-0"></span>
      </template>
    </div>
    <Typography size="xs" weight="medium">
      <slot />
    </Typography>
  </Frame>
</template>
