import { computed } from 'vue';

import { cn } from '@/shared/lib/cn';
import type { NavbarProps } from '@/shared/ui/navbar/model/types';

export const useNavbar = (props: NavbarProps) => {
  const className = computed(() => {
    return cn(
      'fixed flex flex-row max-h-[64px] h-full w-full px-[260px] items-center justify-between ',
      props.className
    );
  });

  const sideClassName = computed(() => {
    return cn('flex flex-row');
  });

  const actionClassname = computed(() => {
    return cn('text-muted hover:text-muted-foreground active:text-foreground');
  });
  return { className, sideClassName, actionClassname };
};
