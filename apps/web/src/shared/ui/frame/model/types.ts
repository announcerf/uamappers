export type FrameAs = keyof HTMLElementTagNameMap;

export type FrameVariant =
  | 'blue'
  | 'green'
  | 'pink'
  | 'orange'
  | 'red'
  | 'purple'
  | 'lavander'
  | 'aqua'
  | 'yellow'
  | 'default';

export type FrameProps = {
  as?: FrameAs;
  variant?: FrameVariant;
  class?: string;
};
