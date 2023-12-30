"use client";

import KeyStatisticsCard from "@/components/statistics/key_statistics_card";
import MonthlyLocationChangeCard from "@/components/statistics/monthly_location_change_card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";

export default function Page() {
	return (
		<div className="p-5">
			<h2 className="text-2xl font-semibold">Statistics</h2>
			<Separator />
			<div className="w-full grid grid-cols-2">
				<KeyStatisticsCard />
				<MonthlyLocationChangeCard />
			</div>
		</div>
	);
}
