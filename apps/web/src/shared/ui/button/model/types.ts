import type { Component } from 'vue';

import type { FrameVariant } from '@/shared/ui';

export type ButtonVariant = Exclude<FrameVariant, 'red' | 'aqua' | 'yellow'>;

export type ButtonProps = {
  variant?: ButtonVariant;
  hasIcon?: boolean;
  icon?: Component;
  title?: string;
  class?: string;
};
