import { computed } from 'vue';

import { cn } from '@/shared/lib/cn';
import { typographyVariants } from '@/shared/ui/typography/lib/variants';
import type { TypographyProps } from '@/shared/ui/typography/model/types';

export const useTypography = (props: TypographyProps) => {
  const className = computed(() => {
    return cn(
      typographyVariants({
        size: props.size,
        weight: props.weight,
        align: props.align,
        transform: props.transform,
      }),
      props.className
    );
  });

  return {
    className,
  };
};
