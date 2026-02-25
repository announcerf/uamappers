import type { HTMLAttributes, JSX, ReactNode, Ref } from 'react';
import type { VariantProps } from 'class-variance-authority';
import { typographyVariants } from '../lib/variants';

export type TypoSize = NonNullable<
  VariantProps<typeof typographyVariants>['size']
>;
export type TypoWeight = NonNullable<
  VariantProps<typeof typographyVariants>['weight']
>;
export type TypoTransform = NonNullable<
  VariantProps<typeof typographyVariants>['transform']
>;

export type TypographyAs = keyof JSX.IntrinsicElements;

export interface TypographyProps
  extends
    Omit<HTMLAttributes<HTMLElement>, 'size'>,
    VariantProps<typeof typographyVariants> {
  as?: TypographyAs;
  children?: ReactNode;
  ref?: Ref<HTMLElement>;
}
