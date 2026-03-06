import type { ExtractPropTypes, PropType } from 'vue';

import type {
  SeparatorOrientation,
  SeparatorProps,
} from '@/shared/ui/separator/model/types';

export const separatorProps = {
  orientation: {
    type: String as PropType<SeparatorOrientation>,
    default: 'horizontal',
  },
  width: {
    type: [Number, String] as PropType<SeparatorProps['width']>,
    default: '100%',
  },
  maxWidth: {
    type: [Number, String] as PropType<SeparatorProps['maxWidth']>,
    default: undefined,
  },
  height: {
    type: [Number, String] as PropType<SeparatorProps['height']>,
    default: '100%',
  },
  className: {
    type: String as PropType<SeparatorProps['className']>,
    default: undefined,
  },
} as const;

export type SeparatorResolvedProps = Readonly<
  ExtractPropTypes<typeof separatorProps>
>;
