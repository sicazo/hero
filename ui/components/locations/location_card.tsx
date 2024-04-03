"use client";

import {
	DotsVerticalIcon,
	StarFilledIcon,
	StarIcon,
} from "@radix-ui/react-icons";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Location } from "@/lib/bindings";
import { useLocationStore } from "@/lib/stores/location_store";
import { useMutation } from "@tanstack/react-query";
import axios from "axios";
import { toast } from "sonner";

export function LocationCard({ location }: { location: Location }) {
	const { updateFavorite, removeLocation, updateLocation } = useLocationStore();

	const check = useMutation({
		mutationKey: ["rescan"],
		mutationFn: async () => {
			const response = await axios.post(
				"http://localhost:3001/translation/scan",
				{ path: location.path },
			);

			return response.data

		},
		onSuccess: (data) => {
			updateLocation({
				...location,
				num_of_keys: data.keys,
				num_of_untranslated_keys: data.untranslated_keys,
			});
		},

	});

	const rescanLocation = () => {
		toast.promise(check.mutateAsync(), {
			loading: "scanning...",
			success: "Location rescanned",
			error: "There was an error rescanning the location"
		})
	}

	const removeLocationFromList = () => {
		removeLocation(location);
		toast.success("Removed location");
	};

	return (
		<Card className="m-5 my-2">
			<CardHeader className="grid grid-cols-[1fr_85px] items-start gap-4 space-y-0">
				<div className="space-y-1">
					<CardTitle>{location.name}</CardTitle>
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
					<DropdownMenu>
						<DropdownMenuTrigger asChild>
							<Button variant="ghost" className="">
								<DotsVerticalIcon className="h-4 w-4" />
							</Button>
						</DropdownMenuTrigger>
						<DropdownMenuContent className="w-42 mx-5">
							{/*TODO: make work*/}
							<DropdownMenuItem disabled>Edit</DropdownMenuItem>
							<DropdownMenuItem onClick={rescanLocation}>Rescan</DropdownMenuItem>
							<DropdownMenuItem onClick={removeLocationFromList}>
								Delete
							</DropdownMenuItem>
						</DropdownMenuContent>
					</DropdownMenu>
				</div>
			</CardHeader>
			<CardContent>
				<div className="flex space-x-4 text-sm text-muted-foreground">
					<div>Keys: {location.num_of_keys}</div>
					<div>Untranslated Keys: {location.num_of_untranslated_keys}</div>
					<div>Added: {location.added_at?.split(",")[0]}</div>
				</div>
			</CardContent>
		</Card>
	);
}
