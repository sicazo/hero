import { type LucideIcon, Settings2 } from "lucide-react";

import { Link, useRouterState } from "@tanstack/react-router";
import { cn } from "../../lib/utils.ts";
import { buttonVariants } from "../ui/button.tsx";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip.tsx";

interface NavProps {
	isCollapsed: boolean;
	links: {
		title: string;
		link: string;
		label?: string;
		icon: LucideIcon;
	}[];
}

export function Nav({ links, isCollapsed }: NavProps): JSX.Element {
	const params = useRouterState();
	const pathName = params.location.pathname;
	const isCurrentPath = (link: string) => {
		if (pathName === link) {
			return "default";
		}
		return "ghost";
	};
	const isSettingsPath = (link: string) => {
		if (pathName.startsWith(link)) {
			return "default";
		}
		return "ghost";
	};
	return (
		<div
			data-collapsed={true}
			className="group flex h-full flex-col gap-4 py-2 data-[collapsed=true]:py-2"
			style={{ justifyContent: "space-between" }}
		>
			<nav className="grid gap-1 px-2 group-[data-collapsed=true]:justify-center group-[data-collapsed=true]:px-2">
				{links.map((link, index) =>
					isCollapsed ? (
						// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
						<Tooltip key={index} delayDuration={0}>
							<TooltipTrigger asChild>
								<Link
									to={link.link}
									className={cn(
										buttonVariants({
											variant: isCurrentPath(link.link),
											size: "icon",
										}),
										"h-9 w-9",
										isCurrentPath(link.link) === "default" &&
											"dark:bg-muted dark:text-muted-foreground dark:hover:bg-muted dark:hover:text-white",
									)}
								>
									<link.icon className="h-4 w-4" />
									<span className="sr-only">{link.title}</span>
								</Link>
							</TooltipTrigger>
							<TooltipContent side="right" className="flex items-center gap-4">
								{link.title}
								{link.label && (
									<span className="text-muted-foreground ml-auto">
										{link.label}
									</span>
								)}
							</TooltipContent>
						</Tooltip>
					) : (
						<Link
							// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
							key={index}
							to={link.link}
							className={cn(
								buttonVariants({
									variant: isCurrentPath(link.link),
									size: "sm",
								}),
								isCurrentPath(link.link) === "default" &&
									"dark:bg-muted dark:hover:bg-muted dark:text-white dark:hover:text-white",
								"justify-start",
							)}
						>
							<link.icon className="mr-2 h-4 w-4" />
							{link.title}
							{link.label && (
								<span
									className={cn(
										"ml-auto",
										isCurrentPath(link.link) === "default" &&
											"text-background dark:text-white",
									)}
								>
									{link.label}
								</span>
							)}
						</Link>
					),
				)}
			</nav>

			{/* Spacer div */}
			<div style={{ flexGrow: 1 }} />
			<div className="flex w-full gap-1 items-center justify-center px-2">
				{/*<Pulse />*/}
			</div>

			<div className="flex w-full gap-1 px-2 pb-2">
				{isCollapsed ? (
					<Tooltip delayDuration={0}>
						<TooltipTrigger asChild>
							<Link
								to="/settings"
								className={cn(
									buttonVariants({
										variant: isSettingsPath("/settings"),
										size: "icon",
									}),
									"h-9 w-9",
									isSettingsPath("/settings") === "default" &&
										"dark:bg-muted dark:text-muted-foreground dark:hover:bg-muted dark:hover:text-white",
								)}
							>
								<Settings2 className="h-4 w-4" />
								<span className="sr-only">Settings</span>
							</Link>
						</TooltipTrigger>
						<TooltipContent side="right" className="flex items-center gap-4">
							Settings
						</TooltipContent>
					</Tooltip>
				) : (
					<Link
						to="/settings"
						className={cn(
							buttonVariants({
								variant: isSettingsPath("/settings"),
							}),
							isSettingsPath("/settings") === "default" &&
								"dark:bg-muted dark:hover:bg-muted dark:text-white dark:hover:text-white",
							"justify-start",
							"w-full",
						)}
					>
						<Settings2 className="h-4 w-4" />
						Settings
					</Link>
				)}
			</div>
		</div>
	);
}
