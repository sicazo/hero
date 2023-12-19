'use client'

import {LocationCard} from "@/components/locations/location_card";

export default function Page() {
	return (
		<>
		<LocationCard name={"test"} description={"test description"} is_starred={true} updated_at={"22.11.2022"} />
		</>
	);
}
