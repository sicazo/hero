import { createFileRoute } from "@tanstack/react-router";

import KeyStatisticsCard from "@/components/statistics/key_statistics_card";
import MonthlyLocationChangeCard from "@/components/statistics/monthly_location_change_card";
import { Separator } from "@/components/ui/separator";

export const Route = createFileRoute("/")({
	component: Home,
});

export default function Home() {
	return (
		<div className="p-5">
			<h2 className="text-2xl font-semibold ml-5 mb-2">Statistics</h2>
			<Separator className="ml-5 w-[95%]" />
			<div className="w-full grid grid-cols-2">
				<KeyStatisticsCard />
				<MonthlyLocationChangeCard />
			</div>
		</div>
	);
}
