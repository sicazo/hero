"use client";
import ThemeProvider from "@/components/theme/theme_provider";
import { TooltipProvider } from "@/components/ui/tooltip";
import { client, queryClient, rspc } from "@/lib/rspc";
import { useLocationStore } from "@/lib/stores/location_store";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { useTranslationStore } from "@/lib/stores/translation_store";
import { isPermissionGranted } from "@tauri-apps/api/notification";
import { clsx } from "clsx";
import { Inter } from "next/font/google";
import { useEffect, useState } from "react";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	const { notifications_enabled, setNotifications } = useSettingsStore();
	useEffect(() => {
		if (notifications_enabled) {
			isPermissionGranted().then((permission) => {
				setNotifications(permission);
			});
		}
	}, [notifications_enabled, setNotifications]);
	const theme = useSettingsStore((state) => state.theme);

	useEffect(() => {
		console.log("rehydrate");
		useSettingsStore.persist.rehydrate();
		useLocationStore.persist.rehydrate();
		useTranslationStore.persist.rehydrate();
	}, []);

	return (
		<html lang="en">
			<body className={clsx(inter.className, "flex h-screen w-screen")}>
				<rspc.Provider client={client} queryClient={queryClient}>
					<ThemeProvider defaultTheme={theme}>
						<TooltipProvider delayDuration={0}>{children}</TooltipProvider>
					</ThemeProvider>
				</rspc.Provider>
			</body>
		</html>
	);
}
