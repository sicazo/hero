"use client";

import AddNewLocation from "@/components/locations/add_new";
import { LocationCard } from "@/components/locations/location_card";
import NoLocations from "@/components/locations/no_locations";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Location as TranslationLocation } from "@/lib/bindings";
import { useLocationStore } from "@/lib/stores/location_store";
import { useState } from "react";

export default function Page() {
	const [addNew, setAddNew] = useState(false);
	const locations = useLocationStore((state) => state.locations);
	return addNew ? (
		<AddNewLocation setAddNew={setAddNew} />
	) : (
		<div className="flex-grow">
			<div className="flex justify-between pt-5 pl-5 pr-5 pb-2 space-x-5">
				<Input placeholder={"Search"} />
				<Button onClick={() => setAddNew(true)}>Add</Button>
			</div>
			<div className="overflow-hidden h-full w-full">
				<div className="flex-grow overflow-y-scroll">
					{locations.length > 0 ? (
						locations.map((location) => <LocationCard location={location} />)
					) : (
						<NoLocations />
					)}
				</div>
			</div>
		</div>
	);
}
