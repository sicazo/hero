"use client";
import { commands } from "@/lib/bindings";
import { useSettingsStore } from "@/lib/stores";
import { Inter } from "next/font/google";
import { useEffect } from "react";
import { StateStorage } from "zustand/middleware";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	useEffect(() => {
		useSettingsStore.persist.rehydrate();
	}, []);

	return (
		<html lang="en">
			<body className={inter.className}>{children}</body>
		</html>
	);
}
