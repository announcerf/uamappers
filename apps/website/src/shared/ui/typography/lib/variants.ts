import { cva } from 'class-variance-authority';

/**
 * Typography scale
 * (values converted to vw from 1920px desktop width)
 *   sm  |  md  | lg
 *  14px | 18px | 28px
 */
export const typographyVariants = cva('tracking-normal', {
  variants: {
    size: {
      lg: 'text-[1.458vw]',
      md: 'text-[0.938vw]',
      sm: 'text-[0.729vw]',
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
