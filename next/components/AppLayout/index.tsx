import React from 'react';

export type AppLayoutProps = {
  children?: React.ReactNode;
}

import localFont from 'next/font/local'

const yuyangFont = localFont({
  src: '../../resource/仓耳渔阳体W02.ttf',
  display: 'swap',
})


export default function AppLayout(props: AppLayoutProps) {
  return (
    <div className={`${yuyangFont.className} app-layout`}>
      {props.children}
    </div>
  )
}
