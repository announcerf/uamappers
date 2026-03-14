export type FrameAs = keyof HTMLElementTagNameMap;

export type FrameVariant =
  | 'ranked'
  | 'loved'
  | 'nat'
  | 'bn'
  | 'pbn'
  | 'fa'
  | 'gmt'
  | 'default';

export type FrameProps = {
  as?: FrameAs;
  variant?: FrameVariant;
  class?: string;
};
