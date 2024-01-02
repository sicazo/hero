"use client";
import { Nav } from "@/components/nav/main_nav";
import ThemeProvider from "@/components/theme/theme_provider";
import {
	ResizableHandle,
	ResizablePanel,
	ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useLocationStore } from "@/lib/stores/location_store";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { useTranslationStore } from "@/lib/stores/translation_store";
import { cn } from "@/lib/utils";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { isPermissionGranted } from "@tauri-apps/api/notification";
import { clsx } from "clsx";
import { Database, Home, PencilRuler } from "lucide-react";
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
	useEffect(() => {
		useSettingsStore.persist.rehydrate();
		useLocationStore.persist.rehydrate()
		useTranslationStore.persist.rehydrate()
	}, []);
	const theme = useSettingsStore((state) => state.theme);
	const { toast_rich_colors } = useSettingsStore();
	const { home_default_sizes, home_nav_collapsed, home_collapsed_size } =
		useSettingsStore((state) => state.resizable_panel_state);
	const setHomePanelSizes = useSettingsStore(
		(state) => state.setHomePanelSizes,
	);
	const updateNavCollapsed = useSettingsStore(
		(state) => state.updateNavCollapsed,
	);
	const [isCollapsed, setIsCollapsed] = useState(home_nav_collapsed);
	const queryClient = new QueryClient();
	return (
		<html lang="en">
			<body className={clsx(inter.className, "flex h-screen w-screen")}>
				<QueryClientProvider client={queryClient}>
					<ThemeProvider defaultTheme={theme}>
						<TooltipProvider delayDuration={0}>
							<ResizablePanelGroup
								direction="horizontal"
								onLayout={(sizes: number[]) => {
									setHomePanelSizes(sizes);
								}}
								className="h-full items-stretch"
							>
								<ResizablePanel
									defaultSize={home_default_sizes[0]}
									collapsedSize={home_collapsed_size}
									collapsible={true}
									minSize={15}
									maxSize={20}
									onCollapse={(state: boolean) => {
										setIsCollapsed(state);
										updateNavCollapsed(state);
									}}
									className={cn(
										isCollapsed &&
											"min-w-[50px] transition-all duration-300 ease-in-out z-50",
									)}
								>
									<Nav
										isCollapsed={isCollapsed}
										links={[
											{
												title: "Home",
												label: "",
												link: "/home",
												icon: Home,
											},
											{
												title: "Editor",
												label: "",
												link: "/editor",
												icon: PencilRuler,
											},
											{
												title: "Locations",
												label: "",
												link: "/locations",
												icon: Database,
											},
										]}
									/>
								</ResizablePanel>
								<ResizableHandle />
								<ResizablePanel
									defaultSize={home_default_sizes[1]}
									minSize={30}
									className="z-10"
								>
									{children}
								</ResizablePanel>
							</ResizablePanelGroup>
						</TooltipProvider>
						<Toaster richColors={toast_rich_colors} />
					</ThemeProvider>
					{/*<ReactQueryDevtools />*/}
				</QueryClientProvider>
			</body>
		</html>
	);
}
