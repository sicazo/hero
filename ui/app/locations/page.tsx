"use client";

import AddNewLocation from "@/components/locations/add_new";
import LocationList from "@/components/locations/location_list";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useLocationStore } from "@/lib/stores/location_store";
import { Search } from "lucide-react";
import { useEffect, useState } from "react";

export default function Page() {
	const [addNew, setAddNew] = useState(false);
	const preLocations = useLocationStore((state) => state.locations);
	const [locations, setLocations] = useState(preLocations);
	const [shownLocations, setShownLocations] = useState(locations);

	useEffect(() => {
		setLocations(preLocations);
		setShownLocations(preLocations);
	}, [preLocations]);

	return (
		<div className="flex-grow max-h-screen p-2">
			<Tabs defaultValue="all">
				<div className="flex items-center mx-5 my-2">
					<h1 className="text-xl font-bold">Locations</h1>
					<TabsList className="ml-auto">
						<TabsTrigger
							value="all"
							className="text-zinc-600 dark:text-zinc-200"
							disabled={addNew}
						>
							All Locations
						</TabsTrigger>
						<TabsTrigger
							value="favorites"
							className="text-zinc-600 dark:text-zinc-200"
							disabled={addNew}
						>
							Favorite
						</TabsTrigger>
					</TabsList>
				</div>
				<Separator />
				{addNew ? (
					<AddNewLocation setAddNew={setAddNew} />
				) : (
					<>
						<div className="bg-background/95 py-3 backdrop-blur supports-[backdrop-filter]:bg-background/60 mx-5">
							<form className="flex gap-2">
								<div className="relative w-[90%]">
									<Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
									<Input
										placeholder="Search"
										className="pl-8"
										onChange={(event) =>
											setShownLocations(
												locations?.filter((loc) =>
													loc?.name
														?.toLowerCase()
														.includes(event.target.value.toLowerCase()),
												),
											)
										}
									/>
								</div>
								<Button onClick={() => setAddNew(true)}>Add</Button>
							</form>
						</div>

						<TabsContent value="all" className="m-0 h-full">
							<LocationList locations={shownLocations ?? []} />
						</TabsContent>
						<TabsContent value="favorites" className="m-0 h-full">
							<LocationList
								locations={
									shownLocations?.filter((item) => item.is_favourite) ?? []
								}
							/>
						</TabsContent>
					</>
				)}
			</Tabs>
		</div>
	);
}
