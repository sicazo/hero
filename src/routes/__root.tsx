import { Nav } from "@/components/nav/main_nav";
import {
	ResizableHandle,
	ResizablePanel,
	ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip.tsx";
import { useLocationStore } from "@/lib/stores/location_store.ts";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { cn } from "@/lib/utils";
import { Outlet, createRootRoute } from "@tanstack/react-router";
import { Database, Home, PencilRuler } from "lucide-react";
import { useEffect, useState } from "react";
import {useTheme} from "@/components/theme/theme_provider.tsx";
import {Theme} from "@/lib/procedures.ts";

export const Route = createRootRoute({
	component: Layout,
});

function Layout() {
	const { toast_rich_colors , theme} = useSettingsStore();
	const { home_default_sizes, home_nav_collapsed, home_collapsed_size } =
		useSettingsStore((state) => state.resizable_panel_state);

	const updateNavCollapsed = useSettingsStore(
		(state) => state.updateNavCollapsed,
	);
	const [isCollapsed, setIsCollapsed] = useState(home_nav_collapsed);
	const {setTheme} = useTheme()

	useEffect(() => {
		useLocationStore.persist.rehydrate();
		useSettingsStore.persist.rehydrate();
	}, []);

	useEffect(() => {
		setTheme(theme as Theme)
	},[theme])

	return (
		// <ThemeProvider defaultTheme={"dark"}>
		<div className="h-screen w-screen flex">
			<TooltipProvider>
				<ResizablePanelGroup
					direction="horizontal"
					//TODO: why is this breaking with rerenders?
					// onLayout={(sizes: number[]) => {
					// 	setHomePanelSizes(sizes);
					// }}
					className="h-full items-stretch"
				>
					<ResizablePanel
						//@ts-expect-error idk throws error
						defaultSize={home_default_sizes[0]}
						collapsedSize={home_collapsed_size}
						collapsible={true}
						minSize={15}
						maxSize={20}
						onCollapse={() => {
							setIsCollapsed(!isCollapsed);
							updateNavCollapsed(!isCollapsed);
						}}
						className={cn(
							isCollapsed &&
								"min-w-[50px] transition-all duration-300 ease-in-out z-50",
						)}
					>
						<Nav
							//@ts-expect-error somehow throws error
							isCollapsed={isCollapsed}
							links={[
								{
									title: "Home",
									label: "",
									link: "/",
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
						//@ts-expect-error throws error aswell
						defaultSize={home_default_sizes[1]}
						minSize={30}
						className="z-10"
					>
						<Outlet />
					</ResizablePanel>
				</ResizablePanelGroup>
				<Toaster richColors={toast_rich_colors} />
			</TooltipProvider>
		</div>
		// </ThemeProvider>
	);
}
