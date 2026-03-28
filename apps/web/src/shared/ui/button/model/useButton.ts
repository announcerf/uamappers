import { computed } from 'vue';

import { cn } from '@/shared/lib/cn';
import {
  buttonIconVariants,
  buttonSeparatorVariants,
  buttonTextVariants,
  buttonVariants,
} from '@/shared/ui/button/lib/variants';
import type { ButtonProps } from '@/shared/ui/button/model/types';

export const useButton = (props: ButtonProps) => {
  const className = computed(() => {
    return cn(
      buttonVariants({ variant: props.variant }),
      props.class,
      '[&>div]:!px-6 h-11'
    );
  });

  const textClassName = computed(() => {
    return buttonTextVariants({ variant: props.variant });
  });

  const iconClassName = computed(() => {
    return buttonIconVariants({ variant: props.variant });
  });

  const separatorClassName = computed(() => {
    return buttonSeparatorVariants({ variant: props.variant });
  });

  const iconComponent = computed(() => {
    if (!props.icon) {
      return undefined;
    }

    return props.icon;
  });

  return {
    className,
    textClassName,
    iconClassName,
    separatorClassName,
    iconComponent,
  };
};
