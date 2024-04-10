"use client";

import {Link, useRouterState} from "@tanstack/react-router";

import { buttonVariants } from "@/components/ui/button";
import { cn } from "@/lib/utils";

interface SidebarNavProps extends React.HTMLAttributes<HTMLElement> {
	items: {
		href: string;
		title: string;
	}[];
}

export function TopNav({ className, items, ...props }: SidebarNavProps) {
	const router = useRouterState()
	const pathname = router.location.pathname

	return (
		<nav className={cn("flex space-x-2 ", className)} {...props}>
			{items.map((item) => (
				<Link
					key={item.href}
					to={item.href}
					className={cn(
						buttonVariants({ variant: "ghost" }),
						pathname === item.href
							? "bg-muted hover:bg-muted"
							: "hover:bg-transparent hover:underline",
						"justify-start cursor-pointer",
					)}
				>
					{item.title}
				</Link>
			))}
		</nav>
	);
}
