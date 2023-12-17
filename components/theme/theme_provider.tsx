'use client'

import * as React from 'react'
import {ThemeProvider as NextThemesProvider} from "next-themes";
import {type ThemeProviderProps} from "next-themes/dist/types";
import {useSettingsStore} from "@/lib/stores";

export default function ThemeProvider({children, ...props}: ThemeProviderProps) {
    return (
        <NextThemesProvider
            attribute="class"
            enableSystem={true}
            {...props}
        >
            {children}
        </NextThemesProvider>
    )
}