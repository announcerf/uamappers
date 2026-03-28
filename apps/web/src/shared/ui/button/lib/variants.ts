import { cva } from 'class-variance-authority';

const buttonVariantsMap = {
  blue: 'bg-(--accent-blue)',
  green: 'bg-(--accent-green)',
  pink: 'bg-(--accent-pink)',
  orange: 'bg-(--accent-orange)',
  purple: 'bg-(--accent-purple)',
  lavander: 'bg-(--accent-lavender)',
  default: 'bg-(--background)',
} as const;

const buttonTextVariantsMap = {
  blue: 'text-(--accent-blue-muted)',
  green: 'text-(--accent-green-muted)',
  pink: 'text-(--accent-pink-muted)',
  orange: 'text-(--background)',
  purple: 'text-(--accent-purple-muted)',
  lavander: 'text-(--accent-lavender-muted)',
  default: 'text-(--muted)',
} as const;

const buttonIconVariantsMap = {
  blue: 'text-(--accent-blue-dim)',
  green: 'text-(--accent-green-dim)',
  pink: 'text-(--accent-pink-dim)',
  orange: 'text-(--accent-orange-dim)',
  purple: 'text-(--accent-purple-dim)',
  lavander: 'text-(--accent-lavender-dim)',
  default: 'text-(--muted)',
} as const;

const buttonSeparatorVariantsMap = {
  blue: 'bg-(--accent-blue-dim)',
  green: 'bg-(--accent-green-dim)',
  pink: 'bg-(--accent-pink-dim)',
  orange: 'bg-(--accent-orange-dim)',
  purple: 'bg-(--accent-purple-dim)',
  lavander: 'bg-(--accent-lavender-dim)',
  default: 'bg-(--muted)',
} as const;

export const buttonVariants = cva([], {
  variants: {
    variant: buttonVariantsMap,
  },
  defaultVariants: {
    variant: 'default',
  },
});

export const buttonTextVariants = cva([], {
  variants: {
    variant: buttonTextVariantsMap,
  },
  defaultVariants: {
    variant: 'default',
  },
});

export const buttonIconVariants = cva([], {
  variants: {
    variant: buttonIconVariantsMap,
  },
  defaultVariants: {
    variant: 'default',
  },
});

export const buttonSeparatorVariants = cva([], {
  variants: {
    variant: buttonSeparatorVariantsMap,
  },
  defaultVariants: {
    variant: 'default',
  },
});
