"use client";

import {
	DocumentDuplicateIcon,
	HomeIcon,
	UserGroupIcon,
} from "@heroicons/react/24/outline";
import { GearIcon } from "@radix-ui/react-icons";
import clsx from "clsx";
import Link from "next/link";
import { usePathname } from "next/navigation";
import {Button} from "@/components/ui/button";

const links = [
	{
		title: "Home",
		href: "/",
		icon: HomeIcon,
	},
	{
		title: "Editor",
		href: "/editor",
		icon: HomeIcon,
	},
	{
		title: "Locations",
		href: "/locations",
		icon: HomeIcon,
	},
	{
		title: "Settings",
		href: "/settings",
		icon: GearIcon,
	},
];

function NavLinks() {
	const pathName = usePathname();
	return (
		<>
			{links.map((link) => {
				const LinkIcon = link.icon;
				return (
					<Link
						key={link.title}
						href={link.href}
						className={clsx(
							"dark:bg-gray-800 flex h-[48px] items-center  gap-2 rounded-md bg-gray-50  text-sm font-medium hover:bg-sky-100 hover:text-blue-600 flex-none justify-start p-2 px-3 pl-5",
							{
								"bg-sky-100 text-blue-600 dark:bg-gray-700 dark:text-blue-400": pathName.includes(
									link.title.toLowerCase(),
								),
							},
						)}
					>
						<LinkIcon className="w-6" />
						<p className="block">{link.title}</p>
					</Link>
				);
			})}
		</>
	);
}

export default function Nav() {
	return (
		<div className="flex h-full flex-col -mt-1 -ml-2">
			<Link
				className="mb-2 flex  items-end justify-start rounded-md bg-blue-600 p-4 h-40"
				href="/"
			>
				<div className="w-32 text-white md:w-40">Logo goes here</div>
			</Link>
			<div className="flex grow  justify-between flex-col space-x-0 space-y-2">
				<NavLinks />
				<div className="dark:bg-gray-950 h-auto w-full grow rounded-md bg-gray-50 block" />
			</div>
		</div>
	);
}
