"use client";

import KeyStatisticsCard from "@/components/statistics/key_statistics_card";
import { ScrollArea } from "@/components/ui/scroll-area";

export default function Page() {
	return (
		<ScrollArea className="h-screen grid grid-cols-2">
			<KeyStatisticsCard />
		</ScrollArea>
	);
}
