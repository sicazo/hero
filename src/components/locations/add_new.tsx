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

import { rspc } from "@/lib/rspc";
import type { dialog } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import {toast} from "sonner";

interface props {
	setAddNew: (value: boolean) => void;
}

export default function AddNewLocation(props: props) {
	const { locations, addLocation } = useLocationStore();
	const getMyLoc = () => {
		locations?.find((location) => location.path === form.getValues("path"));
	};
	const addLocationFormSchema = z.object({
		name: z
			.string()
			.min(1)
			.max(255)
			.default("")
			.refine(
				(name) => !locations?.some((location) => location.name === name),
				{
					message: "Location with this name already exists",
				},
			),
		path: z
			.string()
			.refine(
				(path) => !locations?.some((location) => location.path === path),
				{
					message: `Location with this path already exists under the name of ${getMyLoc}`,
				},
			),
	});
	type LocationFormValues = z.infer<typeof addLocationFormSchema>;

	const form = useForm<LocationFormValues>({
		resolver: zodResolver(addLocationFormSchema),
		mode: "onChange",
	});

	const rspcLocation = rspc.useMutation(["locations.add_location"]);
	async function onSubmit(data: LocationFormValues) {
		const path = data.path.replace("/messages.ts", "");
		const new_path = path.replace("\\messages.ts", "");

		const mutation = rspcLocation.mutateAsync(new_path);
		mutation.then((response) => {
			addLocation({
				tag: "FE",
				name: data.name,
				path: new_path,
				is_favourite: false,
				num_of_keys: response.keys,
				num_of_untranslated_keys: response.untranslated_keys,
				added_at: new Date().toLocaleString(),
			});
			props.setAddNew(false);
		})
		toast.promise(mutation, {
			loading: "Adding location",
			error: "Error adding location",
			success: "The Location got successfully added"
		})

	}

	const [tauriOpen, setTauriOpen] = useState<typeof dialog>();

	const setup = async () => {
		const open = await import("@tauri-apps/api/dialog");

		setTauriOpen(open);
	};

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	useEffect(() => {
		setup();
	}, []);

	async function SelectPath() {
		let path = await tauriOpen?.open({
			multiple: false,
			defaultPath: ".",
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
			const cleaned = path.replace("\\messages.ts", ""); // windows specific path shit :(
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
										<Input
											placeholder="My Location"
											{...field}
											autoComplete="off"
											autoCapitalize="off"
											spellCheck={false}
										/>
									</FormControl>
									<FormDescription>
										The Name the location gets saved as.
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
