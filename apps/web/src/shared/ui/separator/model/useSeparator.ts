import { computed } from 'vue';
import type { CSSProperties } from 'vue';

import { cn } from '@/shared/lib/cn';
import type {
  SeparatorProps,
  SeparatorSize,
} from '@/shared/ui/separator/model/types';

const toCssSize = (value: SeparatorSize | undefined): string | undefined => {
  if (value === undefined) {
    return undefined;
  }

  return typeof value === 'number' ? `${value}px` : value;
};

export const useSeparator = (props: SeparatorProps) => {
  const className = computed(() => {
    return cn(props.className);
  });

  const style = computed<CSSProperties>(() => {
    const separatorStyle: CSSProperties = {
      width: toCssSize(props.width),
    };

    const maxWidth = toCssSize(props.maxWidth);
    if (maxWidth) {
      separatorStyle.maxWidth = maxWidth;
    }

    return separatorStyle;
  });

  return {
    className,
    style,
  };
};
