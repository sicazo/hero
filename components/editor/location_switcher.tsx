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
import {Location, TranslationEntry} from "@/lib/bindings";
import { useLocationStore } from "@/lib/stores/location_store";
import { cn } from "@/lib/utils";
import { CaretSortIcon, CheckIcon } from "@radix-ui/react-icons";
import { useMutation } from "@tanstack/react-query";
import axios from "axios";
import React, { useState } from "react";

type PopoverTriggerProps = React.ComponentPropsWithoutRef<
	typeof PopoverTrigger
>;

type LocationSwitcherProps = PopoverTriggerProps;

export default function LocationSwitcher({ className }: LocationSwitcherProps) {
	const { last_selected_location, setLastSelectedLocation, locations } =
		useLocationStore();
	const [open, setOpen] = useState(false);
	const [selectedLocation, setSelectedLocation] = useState<Location | null>(
		last_selected_location,
	);
	const [searchTerm, setSearchTerm] = useState("");
	const getData = useMutation<{keys:TranslationEntry[]}>({
		mutationKey: [`get_location${selectedLocation?.name}`],
		mutationFn: async () => {
			const response = await axios.post(
				"http://localhost:3001/translation/translations",
				{ path: selectedLocation?.path },
			);
			return response.data
		},
		onSuccess: (data) => {
			console.log(data.keys);
		},
	});


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
							.filter((location) =>
								location.name
									.toLowerCase()
									.startsWith(searchTerm.toLowerCase()),
							)
							.map((location) => (
								<CommandItem
									key={location.name}
									onSelect={() => {
										setSelectedLocation(location);
										setOpen(false);
										setLastSelectedLocation(location);
										getData.mutate();
									}}
									className="text-sm"
								>
									{location.name}
									<CheckIcon
										className={cn(
											"ml-auto h-4 w-4",
											selectedLocation === location
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
