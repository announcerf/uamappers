import localFont from 'next/font/local';

export const kyivType = localFont({
  src: [
    {
      path: './assets/kyiv-type/KyivTypeSans-Thin.woff2',
      style: 'normal',
      weight: '100',
    },
    {
      path: './assets/kyiv-type/KyivTypeSans-Light.woff2',
      style: 'normal',
      weight: '300',
    },
    {
      path: './assets/kyiv-type/KyivTypeSans-Regular.woff2',
      style: 'normal',
      weight: '400',
    },
    {
      path: './assets/kyiv-type/KyivTypeSans-Medium.woff2',
      style: 'normal',
      weight: '500',
    },
    {
      path: './assets/kyiv-type/KyivTypeSans-Bold.woff2',
      style: 'normal',
      weight: '700',
    },
    {
      path: './assets/kyiv-type/KyivTypeSans-Heavy.woff2',
      style: 'normal',
      weight: '800',
    },
    {
      path: './assets/kyiv-type/KyivTypeSans-Black.woff2',
      style: 'normal',
      weight: '900',
    },
  ],
  variable: '--font-kyiv-type',
  display: 'swap',
});
