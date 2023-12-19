"use client";

import { LocationCard } from "@/components/locations/location_card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export default function Page() {
	return (
		<>
			<div className="flex justify-between pt-5 pl-5 pr-5 pb-2 space-x-5">
				<Input placeholder={"Search"} />
				<Button>Add</Button>
			</div>
			<LocationCard
				name={"test"}
				description={"test description"}
				is_starred={true}
				updated_at={"22.11.2022"}
			/>
		</>
	);
}
