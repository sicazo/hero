
import { LocationCard } from "@/components/locations/location_card";
import { ScrollArea } from "@/components/ui/scroll-area";
import type { Location } from "@/lib/procedures.ts";

interface LocationListProps {
	locations: Location[];
}
export default function LocationList({ locations }: LocationListProps) {
	return (
		<ScrollArea className="h-[82vh] w-full">
			<div className="flex flex-col grow">
				{locations.length > 0 ? (
					locations.map((location) => (
						<LocationCard key={location.name} location={location} />
					))
				) : (
					<div className="flex grow items-center justify-center mt-5">
						<h1 className="font-bold">No locations found</h1>
					</div>
				)}
			</div>
		</ScrollArea>
	);
}
