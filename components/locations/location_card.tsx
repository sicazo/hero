"use client";

import { StarFilledIcon, StarIcon } from "@radix-ui/react-icons";

import { Button } from "@/components/ui/button";
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";

interface LocationCardProps {
	name: string;
	description: string;
	is_starred: boolean;
	updated_at: string;
}

export function LocationCard({
	name,
	description,
	is_starred,
	updated_at,
}: LocationCardProps) {
	return (
		<Card className="m-5">
			<CardHeader className="grid grid-cols-[1fr_110px] items-start gap-4 space-y-0">
				<div className="space-y-1">
					<CardTitle>{name}</CardTitle>
					<CardDescription>{description}</CardDescription>
				</div>
				<div className="flex items-center rounded-md bg-secondary text-secondary-foreground">
					<Button variant="secondary" className="px-3 shadow-none">
						{is_starred ? (
							<StarFilledIcon className="mr-2 h-4 w-4" />
						) : (
							<StarIcon className="mr-2 h-4 w-4" />
						)}
						Favourite
					</Button>
				</div>
			</CardHeader>
			<CardContent>
				<div className="flex space-x-4 text-sm text-muted-foreground">
					<div>{updated_at}</div>
				</div>
			</CardContent>
		</Card>
	);
}
