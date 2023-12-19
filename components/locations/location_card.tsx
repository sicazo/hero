"use client";

import {
	DotsVerticalIcon,
	StarFilledIcon,
	StarIcon,
} from "@radix-ui/react-icons";

import { Button } from "@/components/ui/button";
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { Location } from "@/lib/bindings";
import { useLocationStore } from "@/lib/stores/location_store";

export function LocationCard({ location }: { location: Location }) {
	const { updateFavorite } = useLocationStore();
	return (
		<Card className="m-5">
			<CardHeader className="grid grid-cols-[1fr_85px] items-start gap-4 space-y-0">
				<div className="space-y-1">
					<CardTitle>{location.name}</CardTitle>
					{/*<CardDescription>{location.description}</CardDescription>*/}
				</div>
				<div className="flex w-[40px] rounded-md bg-secondary items-center text-secondary-foreground space-x-2">
					<Button
						variant="secondary"
						className="px-3 shadow-none"
						onClick={() => updateFavorite(location)}
					>
						{location.is_favourite ? (
							<StarFilledIcon className="h-4 w-4" />
						) : (
							<StarIcon className="h-4 w-4" />
						)}
					</Button>
					<Button variant="ghost" className="">
						<DotsVerticalIcon className="h-4 w-4" />
					</Button>
				</div>
			</CardHeader>
			<CardContent>
				<div className="flex space-x-4 text-sm text-muted-foreground">
					<div>Keys: {location.num_of_keys}</div>
					<div>Untranslated Keys: {location.num_of_untranslated_keys}</div>
					<div>Added: {location.added_at}</div>
				</div>
			</CardContent>
		</Card>
	);
}
