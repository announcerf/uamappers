import { cva } from 'class-variance-authority';

export const buttonVariants = cva([], {
  variants: {
    variant: {
      blue: 'bg-(--accent-blue) text-(--accent-blue-muted)',
      green: 'bg-(--accent-green text-(--accent-green-muted)',
      pink: 'bg-(--accent-pink) text-(--accent-pink-muted)',
      orange: 'bg-(--accent-orange) text-(--background)',
      lavander: 'bg-(--accent-lavender) text-(--accent-lavender-muted)',
      default: 'bg-(--background) text-(--muted)',
    },
  },
  defaultVariants: {
    variant: 'default',
  },
});
