"use client";
import Nav from "@/components/nav/main_nav";
import ThemeProvider from "@/components/theme/theme_provider";
import { Toaster } from "@/components/ui/toaster";
import { useSettingsStore } from "@/lib/stores";
import { isPermissionGranted } from "@tauri-apps/api/notification";
import { Inter } from "next/font/google";
import { useEffect } from "react";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	const { notifications_enabled, setNotifications } = useSettingsStore();
	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useEffect(() => {
		useSettingsStore.persist.rehydrate();

		if (notifications_enabled) {
			isPermissionGranted().then((permission) => {
				setNotifications(permission);
			});
		}
	}, []);
	const theme = useSettingsStore((state) => state.theme);

	return (
		<html lang="en">
			<body className={inter.className}>
				<ThemeProvider defaultTheme={theme}>
					<div className="h-screen w-screen overflow-hidden flex">
						<aside className=" w-1/5">
							<Nav />
						</aside>
						<div className="flex-1">{children}</div>
					</div>
					<Toaster />
				</ThemeProvider>
			</body>
		</html>
	);
}
