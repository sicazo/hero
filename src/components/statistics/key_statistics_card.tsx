"use client";

import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { useLocationStore } from "@/lib/stores/location_store";
import { Line, LineChart, ResponsiveContainer, Tooltip } from "recharts";

export default function KeyStatisticsCard() {
	const { locations } = useLocationStore();
	return (
		<Card className="m-5">
			<CardHeader>
				<CardTitle>Location Statistics</CardTitle>
				<CardDescription>
					The keys and untranslated keys of all locations
				</CardDescription>
			</CardHeader>
			<CardContent className="">
				<div className="h-[150px]">
					<ResponsiveContainer width="100%" height="100%">
						<LineChart
							data={locations}
							margin={{
								top: 10,
								right: 10,
								left: 10,
								bottom: 10,
							}}
						>
							<Tooltip
								content={({ active, payload }) => {
									if (active && payload && payload.length) {
										return (
											<div className="rounded-lg border bg-background p-2 shadow-sm mb-1">
												{payload[0].payload.name}
												<div className="grid grid-cols-2 gap-2">
													<div className="flex flex-col">
														<span className="text-[0.70rem] uppercase text-muted-foreground">
															Num of Keys
														</span>
														<span className="font-bold text-muted-foreground">
															{payload[0].value}
														</span>
													</div>
													<div className="flex flex-col">
														<span className="text-[0.70rem] uppercase text-muted-foreground">
															Num of Untranslated Keys
														</span>
														<span className="font-bold">
															{payload[1].value}
														</span>
													</div>
												</div>
											</div>
										);
									}

									return null;
								}}
							/>
							<Line
								type="monotone"
								strokeWidth={2}
								dataKey="num_of_keys"
								activeDot={{
									r: 6,
									style: { fill: "var(--theme-primary)", opacity: 0.25 },
								}}
							/>
							<Line
								type="monotone"
								dataKey="num_of_untranslated_keys"
								strokeWidth={2}
								stroke="#ff8080"
								activeDot={{
									r: 8,
									style: { fill: "var(--theme-primary)" },
								}}
							/>
						</LineChart>
					</ResponsiveContainer>
				</div>
			</CardContent>
		</Card>
	);
}
