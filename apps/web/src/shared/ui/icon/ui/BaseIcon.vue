<script setup lang="ts">
import { iconProps } from '@/shared/ui/icon/model/props';
import { useIcon } from '@/shared/ui/icon/model/useIcon';

defineOptions({
  inheritAttrs: false,
});

const props = defineProps(iconProps);
const { size, viewBox, title } = props;
const { attrs, resolvedTitleId } = useIcon(props, 'base-icon');
</script>

<template>
  <svg
    v-bind="attrs"
    :width="size"
    :height="size"
    :viewBox="viewBox"
    fill="currentColor"
    stroke="currentColor"
    xmlns="http://www.w3.org/2000/svg"
    :role="title ? 'img' : undefined"
    :aria-hidden="title ? undefined : 'true'"
    :aria-labelledby="title ? resolvedTitleId : undefined"
  >
    <title
      v-if="title"
      :id="resolvedTitleId"
    >
      {{ title }}
    </title>
    <slot />
  </svg>
</template>
