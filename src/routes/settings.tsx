import { TopNav } from "@/components/settings/top_nav.tsx";
import { Separator } from "@/components/ui/separator.tsx";
import { Outlet, createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/settings")({
	component: Layout,
});

const sidebarNavItems = [
	{
		title: "Translations",
		href: "/settings",
	},
	{
		title: "Appearance",
		href: "/settings/appearance",
	},
	{
		title: "Notifications",
		href: "/settings/notifications",
	},
	// {
	// 	title: "Display",
	// 	href: "/settings/display",
	// },
];

export default function Layout() {
	return (
		<>
			<div
				id="header"
				className="dark:bg-gray-950 sticky top-0 z-50  p-5  pb-16 h-full w-full"
			>
				<div className="space-y-8 px-3">
					<h2 className="text-2xl font-bold tracking-tight">Settings</h2>
				</div>
				<Separator className="my-3 ml-3" />
				<div className="dark:bg-gray-950 flex flex-col space-y-8 p-0 bg-white h-full w-full overflow-x-hidden overflow-y-scroll">
					<aside className="sticky top-0 -mx-4 px-6 ">
						<TopNav items={sidebarNavItems} />
					</aside>
					<div id="content" className="flex-1  pl-5 w-full">
						<Outlet />
					</div>
				</div>
			</div>
		</>
	);
}
