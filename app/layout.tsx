'use client'
import { Inter } from 'next/font/google'
import './globals.css'
import {useHeroStore} from "@/lib/HeroStore";
import {useEffect} from "react";
import {StateStorage} from "zustand/middleware";
import {commands} from "@/lib/bindings";

const inter = Inter({ subsets: ['latin'] })


export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {

  useEffect(() => {
    useHeroStore.persist.rehydrate();
  }, [])

  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  )
}
