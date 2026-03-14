import type { PropType } from 'vue';

import type { BaseIconProps } from '@/shared/ui/icon/model/types';

export const iconProps = {
  size: {
    type: [Number, String] as PropType<BaseIconProps['size']>,
    default: 24,
  },
  title: {
    type: String as PropType<BaseIconProps['title']>,
    default: undefined,
  },
  titleId: {
    type: String as PropType<BaseIconProps['titleId']>,
    default: undefined,
  },
  viewBox: {
    type: String as PropType<BaseIconProps['viewBox']>,
    default: '0 0 24 24',
  },
} as const;
