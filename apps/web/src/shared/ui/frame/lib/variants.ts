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
        blue: '[--frame-accent:var(--accent-blue-hover)]',
        green: '[--frame-accent:var(--accent-green-hover)]',
        pink: '[--frame-accent:var(--accent-pink-hover)]',
        orange: '[--frame-accent:var(--accent-orange-hover)]',
        red: '[--frame-accent:var(--accent-red)]',
        purple: '[--frame-accent:var(--accent-purple-hover)]',
        lavander: '[--frame-accent:var(--accent-lavender-hover)]',
        aqua: '[--frame-accent:var(--accent-aqua)]',
        yellow: '[--frame-accent:var(--accent-yellow-hover)]',
        default: '[--frame-accent:var(--outline)]',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  }
);
