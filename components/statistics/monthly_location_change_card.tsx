'use client';

import {Card, CardContent, CardDescription, CardHeader, CardTitle} from "@/components/ui/card";
import {Line, LineChart, ResponsiveContainer, Tooltip} from "recharts";
import {useLocationStore} from "@/lib/stores/location_store";
import { useEffect, useState } from "react";

interface MonthlyLocationChange {
    total: number;
    date: string
}
export default function MonthlyLocationChangeCard() {
    const {locations} = useLocationStore();
    const [monthlyChanges, setMonthlyChanges] = useState<MonthlyLocationChange[]>([]);

    useEffect(() => {
        const changes: Partial<Record<string, number>> = {};
        let total = 0;
        for (const location of locations) {
            // convert date string to Date object
            const dateParts = location.added_at.split("/");
            const date = new Date(+dateParts[2], Number(dateParts[1]) - 1, +dateParts[0]);
            const monthKey = `${date.getFullYear()}-${date.getMonth() + 1}`;
            total = total + 1;
            changes[monthKey] = total;
        }
        console.log(changes)
        const result = Object.entries(changes)
            .map(([date, total]) => ({date, total}))
        setMonthlyChanges(result as MonthlyLocationChange[]);
    }, [locations]);

    let changeInTotal = 0;
    let changeInTotalPercent = "";

    if (monthlyChanges.length > 1) {
        const previousTotal = monthlyChanges[monthlyChanges.length - 2].total;
        changeInTotal = monthlyChanges[monthlyChanges.length - 1].total - previousTotal;
        changeInTotalPercent = ((changeInTotal / previousTotal) * 100).toFixed(2);
    }

   const sign = changeInTotal > 0 ? "+" : changeInTotal < 0 ? "-" : "";

    return (
        <Card className="m-5">
            <CardHeader>
                <CardTitle>Monthly Location Change</CardTitle>
                <CardDescription>
                    Total number of locations at the end of this month
                </CardDescription>
            </CardHeader>
            <CardContent>
                <div className="text-2xl font-bold">{sign}{changeInTotal}</div>
                <p className="text-xs text-muted-foreground">
                    {sign} {changeInTotalPercent}% from last month
                </p>
                <div className="h-[200px]">
                    <ResponsiveContainer width="100%" height="100%">
                        <LineChart
                            data={monthlyChanges}
                            margin={{
                                top: 5,
                                right: 10,
                                left: 10,
                                bottom: 0,
                            }}
                        >
                            <Tooltip
                                content={({ active, payload }) => {
                                    if (active && payload && payload.length) {
                                        return (
                                            <div className="rounded-lg border bg-background p-2 shadow-sm mb-1">
                                                {payload[0].payload.date}
                                                <div className="grid grid-cols-2 gap-2">
                                                    <div className="flex flex-col">
														<span className="text-[0.70rem] uppercase text-muted-foreground">
															Num of Locations
														</span>
                                                        <span className="font-bold text-muted-foreground">
															{payload[0].value}
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
                                dataKey="total"
                                strokeWidth={2}
                                activeDot={{
                                    r: 6,
                                    style: {fill: "var(--theme-primary)", opacity: 0.25},
                                }}
                            />
                        </LineChart>
                    </ResponsiveContainer>
                </div>
            </CardContent>
        </Card>
)
}