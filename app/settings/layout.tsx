"use client";
import { TopNav } from "@/components/settings/top_nav";
import { Separator } from "@/components/ui/separator";

const sidebarNavItems = [
	{
		title: "Translations",
		href: "/settings",
	},
	// {
	//     title: "Account",
	//     href: "/settings/account",
	// },
	{
		title: "Appearance",
		href: "/settings/appearance",
	},
	{
		title: "Notifications",
		href: "/settings/notifications",
	},
	{
		title: "Display",
		href: "/settings/display",
	},
];

interface SettingsLayoutProps {
	children: React.ReactNode;
}

export default function SettingsLayout({ children }: SettingsLayoutProps) {
	return (
		<>
			<div
				id="header"
				className="sticky top-0 z-50 space-y-6 p-10 pb-16 h-full w-full"
			>
				<div className="space-y-0.5">
					<h2 className="text-2xl font-bold tracking-tight">Settings</h2>
					<p className="text-muted-foreground">
						Manage your settings and set notification preferences.
					</p>
				</div>
				<Separator className="my-6" />
				<div className="flex flex-col space-y-8p-0 bg-white h-full w-full overflow-x-hidden overflow-y-scroll">
					<aside className="sticky top-0 -mx-4 p-6 ">
						<TopNav items={sidebarNavItems} />
					</aside>
					<div id="content" className="flex-1  p-2 w-full">
						{children}
					</div>
				</div>
			</div>
		</>
	);
}
