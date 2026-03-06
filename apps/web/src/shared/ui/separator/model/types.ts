export type SeparatorOrientation = 'horizontal' | 'vertical';
export type SeparatorSize = number | string;

export type SeparatorProps = {
  orientation?: SeparatorOrientation;
  className?: string;
  width?: SeparatorSize;
  maxWidth?: SeparatorSize;
  height?: SeparatorSize;
};
