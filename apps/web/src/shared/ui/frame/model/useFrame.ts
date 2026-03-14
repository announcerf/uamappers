import { computed } from 'vue';

import { cn } from '@/shared/lib/cn';
import { frameVariants } from '@/shared/ui/frame/lib/variants';
import type { FrameProps } from '@/shared/ui/frame/model/types';

export const useFrame = (props: FrameProps) => {
  const className = computed(() => {
    return cn(frameVariants({ variant: props.variant }), props.class);
  });

  const contentClassName = computed(() => {
    return cn('relative min-h-8 min-w-8 px-3 py-4');
  });

  const accentLineClassName = computed(() => {
    return cn(
      'pointer-events-none absolute top-1 right-1 left-1 h-0.5 bg-[var(--frame-accent)]'
    );
  });

  return {
    className,
    contentClassName,
    accentLineClassName,
  };
};
