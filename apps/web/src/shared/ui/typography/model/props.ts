import type { ExtractPropTypes, PropType } from 'vue';

import type {
  TypoSize,
  TypoTransform,
  TypoWeight,
  TypographyAs,
  TypographyProps,
} from '@/shared/ui/typography/model/types';

export const typographyProps = {
  as: {
    type: String as PropType<TypographyAs>,
    default: 'p',
  },
  size: {
    type: String as PropType<TypoSize>,
    default: 'md',
  },
  weight: {
    type: String as PropType<TypoWeight>,
    default: 'light',
  },
  align: {
    type: String as PropType<TypographyProps['align']>,
    default: 'left',
  },
  transform: {
    type: String as PropType<TypoTransform>,
    default: 'normal',
  },
  className: {
    type: String as PropType<TypographyProps['className']>,
    default: undefined,
  },
} as const;

export type TypographyResolvedProps = Readonly<
  ExtractPropTypes<typeof typographyProps>
>;
