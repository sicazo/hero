import { LucideIcon } from "lucide-react";
import Link from "next/link";

import { buttonVariants } from "@/components/ui/button";
import {
	Tooltip,
	TooltipContent,
	TooltipTrigger,
} from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";

interface NavProps {
	isCollapsed: boolean;
	links: {
		title: string;
		label?: string;
		icon: LucideIcon;
		variant: "default" | "ghost";
	}[];
}

export function NavTest({ links, isCollapsed }: NavProps): JSX.Element {
	return (
		<div
			data-collapsed={true}
			className="group flex flex-col gap-4 py-2 data-[collapsed=true]:py-2 h-screen"
			style={{ justifyContent: "space-between" }}
		>
			<nav className="grid gap-1 px-2 group-[data-collapsed=true]:justify-center group-[data-collapsed=true]:px-2">
				{links.map((link, index) =>
					isCollapsed ? (
						<Tooltip key={link.label} delayDuration={0}>
							<TooltipTrigger asChild>
								<Link
									href="#"
									className={cn(
										buttonVariants({ variant: link.variant, size: "icon" }),
										"h-9 w-9",
										link.variant === "default" &&
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
									<span className="ml-auto text-muted-foreground">
										{link.label}
									</span>
								)}
							</TooltipContent>
						</Tooltip>
					) : (
						<Link
							key={link.title}
							href="#"
							className={cn(
								buttonVariants({ variant: link.variant, size: "sm" }),
								link.variant === "default" &&
									"dark:bg-muted dark:text-white dark:hover:bg-muted dark:hover:text-white",
								"justify-start",
							)}
						>
							<link.icon className="mr-2 h-4 w-4" />
							{link.title}
							{link.label && (
								<span
									className={cn(
										"ml-auto",
										link.variant === "default" &&
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

			<nav className="grid gap-1 px-2 group-[data-collapsed=true]:justify-center group-[data-collapsed=true]:px-2">
				<Link
					href="#"
					className={cn(buttonVariants({ variant: "ghost", size: "sm" }))}
				>
					Test
				</Link>
			</nav>
		</div>
	);
}
