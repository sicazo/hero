"use client";

import { Button } from "@/components/ui/button";
import {
	Command,
	CommandEmpty,
	CommandInput,
	CommandItem,
	CommandList,
} from "@/components/ui/command";
import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from "@/components/ui/popover";
import type { Location } from "@/lib/procedures";
import { rspc } from "@/lib/rspc";
import { useLocationStore } from "@/lib/stores/location_store";
import { useTranslationStore } from "@/lib/stores/translation_store";
import { cn } from "@/lib/utils";
import { CaretSortIcon, CheckIcon } from "@radix-ui/react-icons";
import type React from "react";
import { useEffect, useState } from "react";

type PopoverTriggerProps = React.ComponentPropsWithoutRef<
	typeof PopoverTrigger
>;

type LocationSwitcherProps = PopoverTriggerProps;

export default function LocationSwitcher({ className }: LocationSwitcherProps) {
	const { last_selected_location, setLastSelectedLocation, locations } =
		useLocationStore();
	if (!last_selected_location && locations) {
		setLastSelectedLocation(locations[0]);
	}
	const { setTranslationEntries } = useTranslationStore();
	const [open, setOpen] = useState(false);
	const [selectedLocation, setSelectedLocation] = useState<Location>(
		//@ts-expect-error possible undefined
		(last_selected_location || locations[0]) as Location,
	);
	const [searchTerm, setSearchTerm] = useState("");

	const getData = rspc.useMutation(["translations.get_translations"]);
	useEffect(() => {
		getData
			.mutateAsync(selectedLocation.path as string)
			.then((data) => setTranslationEntries(data));

		//eslint-disable-next-line react-hooks/exhaustive-deps
	}, []);

	return (
		<Popover open={open} onOpenChange={setOpen}>
			<PopoverTrigger asChild>
				<Button
					variant="outline"
					role="combobox"
					aria-expanded={open}
					aria-label="Select a location"
					className={cn("w-[170px] justify-between", className)}
				>
					{selectedLocation?.name}
					<CaretSortIcon className="ml-auto h-4 w-4 shrink-0 opacity-50" />
				</Button>
			</PopoverTrigger>
			<PopoverContent className="w-[170px]">
				<Command>
					<CommandList>
						<CommandInput
							placeholder={"Search"}
							onInput={(e) => setSearchTerm(e.currentTarget.value)}
						/>
						<CommandEmpty>No Location found</CommandEmpty>
						{locations
							?.filter((location) =>
								location.name
									?.toLowerCase()
									.startsWith(searchTerm.toLowerCase()),
							)
							.map((location) => (
								<CommandItem
									key={location.name}
									onSelect={() => {
										setSelectedLocation(location);
										setOpen(false);
										setLastSelectedLocation(location);
										getData
											.mutateAsync(selectedLocation.path as string)
											.then((data) => setTranslationEntries(data));
									}}
									className="text-sm"
								>
									{location.name}
									{/*TODO: make this to id instead of name after db merge	*/}
									<CheckIcon
										className={cn(
											"ml-auto h-4 w-4",
											selectedLocation.name === location.name
												? "opacity-100"
												: "opacity-0",
										)}
									/>
								</CommandItem>
							))}
					</CommandList>
				</Command>
			</PopoverContent>
		</Popover>
	);
}
