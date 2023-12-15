'use client'

import { Inter } from 'next/font/google'
import './globals.css'
import {useHeroStore} from "@/lib/HeroStore";

const inter = Inter({ subsets: ['latin'] })


export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const {test} = useHeroStore()
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  )
}
