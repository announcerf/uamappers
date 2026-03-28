import { computed } from 'vue';

import { cn } from '@/shared/lib/cn';
import { buttonVariants } from '@/shared/ui/button/lib/variants';
import type { ButtonProps } from '@/shared/ui/button/model/types';

export const useButton = (props: ButtonProps) => {
  const className = computed(() => {
    return cn(buttonVariants({ variant: props.variant }), props.class);
  });

  return { className };
};
