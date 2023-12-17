"use client";
import { commands } from "@/lib/bindings";
import { useSettingsStore } from "@/lib/stores";
import { Inter } from "next/font/google";
import { useEffect } from "react";
import "./globals.css";
import ThemeProvider from "@/components/theme/theme_provider";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	useEffect(() => {
		useSettingsStore.persist.rehydrate();
	}, []);
	const theme = useSettingsStore((state) => state.theme);

	return (
		<html lang="en">
			<body className={inter.className}>
			<ThemeProvider
				defaultTheme={theme}
			>
				{children}
			</ThemeProvider>
			</body>
		</html>
	);
}
