import { cva } from 'class-variance-authority';

export const frameVariants = cva(
  [
    'relative',
    'box-border',
    'block',
    'min-h-10',
    'min-w-10',
    'border-4',
    'border-card',
    'bg-background',
    '[--frame-accent:var(--outline)]',
  ],
  {
    variants: {
      variant: {
        ranked: '[--frame-accent:var(--accent-green)]',
        loved: '[--frame-accent:var(--accent-pink)]',
        nat: '[--frame-accent:var(--accent-red)]',
        bn: '[--frame-accent:var(--accent-purple)]',
        pbn: '[--frame-accent:var(--accent-lavender)]',
        fa: '[--frame-accent:var(--accent-aqua)]',
        gmt: '[--frame-accent:var(--accent-yellow)]',
        default: '[--frame-accent:var(--outline)]',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  }
);
