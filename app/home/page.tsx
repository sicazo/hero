"use client";

import KeyStatisticsCard from "@/components/statistics/key_statistics_card";
import { ScrollArea } from "@/components/ui/scroll-area";
import MonthlyLocationChangeCard from "@/components/statistics/monthly_location_change_card";

export default function Page() {
	return (
		<div className=" w-full grid grid-cols-2">
			<KeyStatisticsCard />
			<MonthlyLocationChangeCard />
		</div>
	);
}
