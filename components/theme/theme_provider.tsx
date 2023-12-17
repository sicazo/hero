"use client";

import { useSettingsStore } from "@/lib/stores";
import { ThemeProvider as NextThemesProvider } from "next-themes";
import { type ThemeProviderProps } from "next-themes/dist/types";
import * as React from "react";

export default function ThemeProvider({
	children,
	...props
}: ThemeProviderProps) {
	return (
		<NextThemesProvider attribute="class" enableSystem={true} {...props}>
			{children}
		</NextThemesProvider>
	);
}
