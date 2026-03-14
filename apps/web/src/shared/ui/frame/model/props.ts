import type { ExtractPropTypes, PropType } from 'vue';

import type {
  FrameAs,
  FrameProps,
  FrameVariant,
} from '@/shared/ui/frame/model/types';

export const frameProps = {
  as: {
    type: String as PropType<FrameAs>,
    default: 'div',
  },
  variant: {
    type: String as PropType<FrameVariant>,
    default: 'default',
  },
  class: {
    type: String as PropType<FrameProps['class']>,
    default: undefined,
  },
} as const;

export type FrameResolvedProps = Readonly<ExtractPropTypes<typeof frameProps>>;
