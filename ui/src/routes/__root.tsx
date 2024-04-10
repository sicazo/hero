import {createRootRoute, Link, Outlet} from "@tanstack/react-router"
import {TanStackRouterDevtools} from "@tanstack/router-devtools"
import {useState} from "react";
import {Database, Home, PencilRuler} from "lucide-react";
import {useSettingsStore} from "../../lib/stores/settings_store.ts";
import {ResizableHandle, ResizablePanel, ResizablePanelGroup} from "../../components/ui/resizable.tsx";
import {cn} from "../../lib/utils.ts";
import {Nav} from "../../components/nav/main_nav.tsx";
import {Toaster} from "../../components/ui/sonner.tsx";
import {TooltipProvider} from "../../components/ui/tooltip.tsx";


export const Route = createRootRoute({

    component: () => {
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


        return (
            <>
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
                            //@ts-ignore
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
                                        link: "/application/home",
                                        icon: Home,
                                    },
                                    {
                                        title: "Editor",
                                        label: "",
                                        link: "/application/editor",
                                        icon: PencilRuler,
                                    },
                                    {
                                        title: "Locations",
                                        label: "",
                                        link: "/application/locations",
                                        icon: Database,
                                    },
                                ]}
                            />
                        </ResizablePanel>
                        <ResizableHandle />
                        <ResizablePanel
                            //@ts-ignore
                            defaultSize={home_default_sizes[1]}
                            minSize={30}
                            className="z-10"
                        >
                            <Outlet />

                        </ResizablePanel>
                    </ResizablePanelGroup>
                    <Toaster richColors={toast_rich_colors} />
                <TanStackRouterDevtools />
</TooltipProvider>
            </>

    )}
})