'use client';

import { createElement } from 'react';

import { cn } from '@/src/shared/lib';

import type { TypographyProps } from '../model/types';
import { typographyVariants } from '../lib/variants';

export const Typography = ({
  as: Tag = 'p',
  className,
  size,
  weight,
  align,
  transform,
  children,
  ref,
  ...rest
}: TypographyProps) => {
  return createElement(
    Tag,
    {
      ref,
      className: cn(
        typographyVariants({ size, weight, align, transform }),
        className,
      ),
      ...rest,
    },
    children,
  );
};
