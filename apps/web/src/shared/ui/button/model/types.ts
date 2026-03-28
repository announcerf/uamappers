import type { Component } from 'vue';

export type ButtonVariant =
  | 'blue'
  | 'green'
  | 'pink'
  | 'orange'
  | 'lavander'
  | 'default';

export type ButtonProps = {
  variant?: ButtonVariant;
  hasIcon?: boolean;
  icon?: Component;
  title?: string;
  class?: string;
};
