<script setup lang="ts">
import { type ButtonProps, Frame, Typography } from '@/shared/ui';
import { useButton } from '@/shared/ui/button/model/useButton';

const props = withDefaults(defineProps<ButtonProps>(), {
  variant: 'default',
});
const {
  className,
  textClassName,
  iconClassName,
  separatorClassName,
  iconComponent,
} = useButton(props);
</script>

<template>
  <Frame as="button" :variant="props.variant" :class="className">
    <div class="flex items-center gap-3">
      <template v-if="iconComponent">
        <div class="flex items-center gap-1">
          <component
            :is="iconComponent"
            :size="16"
            :class="iconClassName"
            class="shrink-0"
            aria-hidden="true"
          />
          <span
            aria-hidden="true"
            :class="separatorClassName"
            class="h-4 w-px shrink-0"
          />
        </div>
      </template>
      <Typography size="xs" weight="medium" :class-name="textClassName">
        <slot />
      </Typography>
    </div>
  </Frame>
</template>
