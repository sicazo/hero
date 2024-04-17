
import {
	DotsVerticalIcon,
	StarFilledIcon,
	StarIcon,
} from "@radix-ui/react-icons";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import type { Location, LocationStore } from "@/lib/procedures";
import { client, rspc } from "@/lib/rspc";
import { useLocationStore } from "@/lib/stores/location_store";
import { toast } from "sonner";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip.tsx";

export function LocationCard({ location }: { location: Location }) {
	const { updateFavorite, removeLocation, setLocations } = useLocationStore();

	const check = rspc.useMutation(["locations.rescan_location"]);

	const rescanLocation = () => {
		const checkPromise = check.mutateAsync({ path: location.path as string, tag: location.tag });
		checkPromise.then(async () => {
			await client.mutation(["stores.getStore", "location_store"]).then((data) => {
				const store = data as LocationStore;
				setLocations(store.state.locations as Location[])
			})
		})
		toast.promise(checkPromise, {
			loading: "Scanning...",
			success: "Location rescanned",
			error: "There was an error rescanning the location",
		});
	};

	const removeLocationFromList = () => {
		//TODO: remove location in backend
		removeLocation(location);
		toast.success("Removed location");
	};

	return (
		<Card className="m-5 my-2">
			<CardHeader className="grid grid-cols-[1fr_125px] items-start gap-2 space-y-0">
				<div className="space-y-1">
					<CardTitle className="mb-2">{location.name}</CardTitle>
					<CardDescription>{location.path}</CardDescription>
				</div>
				<div className="flex w-[40px] rounded-md bg-secondary items-center text-secondary-foreground space-x-2">
					<Tooltip>
						<TooltipTrigger>
							<Button
								variant="secondary"
								className="px-3 shadow-none cursor-default"
								disabled
							>
								{location.tag}
							</Button>
						</TooltipTrigger>
						<TooltipContent side={"bottom"}>
							The type of the Location. <br />
							FE for Frontend and BE for Backend
						</TooltipContent>

					</Tooltip>

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
							<DropdownMenuItem onClick={rescanLocation}>
								Rescan
							</DropdownMenuItem>
							<DropdownMenuItem onClick={removeLocationFromList}>
								Delete
							</DropdownMenuItem>
						</DropdownMenuContent>
					</DropdownMenu>
				</div>
			</CardHeader>
			<CardContent className="-mt-2 flex justify-between">
				<div className="flex space-x-4 text-sm text-muted-foreground">
					<div>Keys: {location.num_of_keys}</div>
					<div>Untranslated Keys: {location.num_of_untranslated_keys}</div>
					<div>Added: {location.added_at?.split(" ")[0]}</div>
				</div>

			</CardContent>
		</Card>
	);
}
