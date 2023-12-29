"use client";
import { Button } from "@/components/ui/button";
import {
	CardContent,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import {
	Form,
	FormControl,
	FormDescription,
	FormField,
	FormItem,
	FormLabel,
	FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useLocationStore } from "@/lib/stores/location_store";
import { zodResolver } from "@hookform/resolvers/zod";
import { open } from "@tauri-apps/api/dialog";
import axios from "axios";
import { useForm } from "react-hook-form";
import { z } from "zod";

interface props {
	setAddNew: (value: boolean) => void;
}

export default function AddNewLocation(props: props) {
	const { locations, addLocation } = useLocationStore();
	const getMyLoc = () => {
		locations.find((location) => location.path === form.getValues("path"));
	};
	const addLocationFormSchema = z.object({
		name: z
			.string()
			.min(1)
			.max(255)
			.default("")
			.refine((name) => !locations.some((location) => location.name === name), {
				message: "Location with this name already exists",
			}),
		path: z
			.string()
			.refine((path) => !locations.some((location) => location.path === path), {
				message: `Location with this path already exists under the name of ${getMyLoc}`,
			}),
	});
	type LocationFormValues = z.infer<typeof addLocationFormSchema>;

	const form = useForm<LocationFormValues>({
		resolver: zodResolver(addLocationFormSchema),
		mode: "onChange",
	});
	async function onSubmit(data: LocationFormValues) {
		const response = await axios.post(
			"http://localhost:3001/translation/keys",
			{ path: data.path },
		);
		addLocation({
			name: data.name,
			path: data.path,
			is_favourite: false,
			num_of_keys: response.data.num_of_keys,
			num_of_untranslated_keys: 0,
			added_at: new Date().toLocaleDateString(),
		});
		props.setAddNew(false);
	}

	async function SelectPath() {
		let path = await open({
			multiple: false,
			directory: false,
			filters: [
				{
					name: "Typescript Files",
					extensions: ["ts"],
				},
			],
		});
		if (path !== null) {
			path = path as string;
			const cleaned = path.replace("/messages.ts", "");
			form.setValue("path", cleaned);
		}
	}

	return (
		<div className="h-full w-full items-center justify-center flex-grow flex">
			<Form {...form}>
				<form
					onSubmit={form.handleSubmit(onSubmit)}
					className="w-full flex-col flex-grow"
				>
					<CardHeader>
						<CardTitle>Add a new Location</CardTitle>
					</CardHeader>
					<CardContent className="grid gap-6">
						<FormField
							control={form.control}
							name="name"
							render={({ field }) => (
								<FormItem className="grid gap-2">
									<FormLabel>Name</FormLabel>
									<FormControl>
										<Input placeholder="My Location" {...field} />
									</FormControl>
									<FormDescription>
										The Name the location ges saved as.
									</FormDescription>
									<FormMessage />
								</FormItem>
							)}
						/>
						<FormField
							control={form.control}
							name="path"
							render={({ field }) => (
								<FormItem className="grid gap-2">
									<FormLabel>Path</FormLabel>
									<FormControl>
										<div className="flex w-full  items-center space-x-2">
											<Input
												type="text"
												placeholder="Path"
												disabled
												value={field.value}
											/>
											<Button type="button" onClick={SelectPath}>
												Select Path
											</Button>
										</div>
									</FormControl>
									<FormDescription>
										The path to the message.ts of the location.
									</FormDescription>
									<FormMessage />
								</FormItem>
							)}
						/>
					</CardContent>
					<CardFooter className="justify-between space-x-2">
						<Button variant="ghost" onClick={() => props.setAddNew(false)}>
							Cancel
						</Button>
						<Button>Submit</Button>
					</CardFooter>
				</form>
			</Form>
		</div>
	);
}
