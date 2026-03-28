import type { Component, ExtractPropTypes, PropType } from 'vue';

import type {
  ButtonProps,
  ButtonVariant,
} from '@/shared/ui/button/model/types';

export const buttonProps = {
  variant: {
    type: String as PropType<ButtonVariant>,
    default: 'default',
  },
  class: {
    type: String as PropType<ButtonProps['class']>,
    default: undefined,
  },
  hasIcon: {
    type: Boolean,
    default: false,
  },
  icon: {
    type: [Object, Function] as PropType<Component>,
    default: undefined,
  },
} as const;

export type ButtonResolvedProps = Readonly<
  ExtractPropTypes<typeof buttonProps>
>;
