export type TypoSize = 'xs' | 'sm' | 'md' | 'xl';
export type TypoWeight =
  | 'light'
  | 'regular'
  | 'medium'
  | 'semibold'
  | 'bold'
  | 'black';
export type TypoAlign = 'left' | 'center' | 'right' | 'justify';
export type TypoTransform = 'uppercase' | 'lowercase' | 'capitalize' | 'normal';
export type TypographyAs = keyof HTMLElementTagNameMap;

export type TypographyProps = {
  as?: TypographyAs;
  size?: TypoSize;
  weight?: TypoWeight;
  align?: TypoAlign;
  transform?: TypoTransform;
  className?: string;
};
