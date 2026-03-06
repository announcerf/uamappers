import { cva } from 'class-variance-authority';

export const separatorVariants = cva('block shrink-0 bg-current', {
  variants: {
    orientation: {
      horizontal: 'h-px min-h-px w-full',
      vertical: 'h-full w-px min-w-px',
    },
  },
  defaultVariants: {
    orientation: 'horizontal',
  },
});
