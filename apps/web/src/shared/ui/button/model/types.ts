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
  title?: string;
  class?: string;
};
