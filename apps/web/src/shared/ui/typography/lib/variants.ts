import { cva } from 'class-variance-authority';

export const typographyVariants = cva('!leading-none tracking-[-0.04em]', {
  variants: {
    size: {
      xl: 'text-[1.75rem]',
      md: 'text-[1.125rem]',
      sm: 'text-[0.875rem]',
      xs: 'text-[0.75rem]',
    },
    weight: {
      light: 'font-light',
      regular: 'font-normal',
      medium: 'font-medium',
      semibold: 'font-semibold',
      bold: 'font-bold',
      black: 'font-black',
    },
    align: {
      left: 'text-left',
      center: 'text-center',
      right: 'text-right',
      justify: 'text-justify',
    },
    transform: {
      uppercase: 'uppercase',
      lowercase: 'lowercase',
      capitalize: 'capitalize',
      normal: 'normal-case',
    },
  },
  defaultVariants: {
    size: 'md',
    weight: 'regular',
  },
});
