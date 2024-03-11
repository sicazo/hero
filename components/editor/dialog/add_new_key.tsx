"use client";

import { Button } from "@/components/ui/button";
import { CardContent, CardFooter } from "@/components/ui/card";
import { DialogTrigger } from "@/components/ui/dialog";
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
import { useSettingsStore } from "@/lib/stores/settings_store";
import { useTranslationStore } from "@/lib/stores/translation_store";
import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation } from "@tanstack/react-query";
import axios from "axios";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import z from "zod";

export default function AddNewKeyDialog() {
	const { translation_entries } = useTranslationStore();
	const { default_language } = useSettingsStore(
		(state) => state.translation_settings,
	);
	const { last_selected_location } = useLocationStore();

	const formSchema = z.object({
		ts_key: z
			.string()
			.min(1)
			.max(255)
			.default("")
			.refine(
				(key) => !translation_entries.some((entry) => entry.key === key),
				"A TS Key with that value is already existing.",
			),
		json_key: z
			.string()
			.min(1)
			.max(255)
			.default("")
			.refine(
				(key) => !translation_entries.some((entry) => entry.value === key),
				"A Json Key with that value is already existing.",
			),
		translation: z
			.string()
			.min(1)
			.max(255)
			.default("")
			.refine(
				(translation) =>
					!translation_entries.some(
						//@ts-ignore
						(entry) => entry.translations[default_language] === translation,
					),
				"A Translation with that value is already existing.",
			),
	});
	const form = useForm<z.infer<typeof formSchema>>({
		resolver: zodResolver(formSchema),
		mode: "onChange",
	});

	const addNewMutation = useMutation({
		mutationKey: ["Add New Key"],
		mutationFn: async (values: z.infer<typeof formSchema>) => {
			const result = await axios.post("http://localhost:3001/translation/add", {
				path: last_selected_location?.path,
				ts_key: values.ts_key,
				json_key: values.json_key,
				value: values.translation,
			});
			return result.data;
		},
		onSuccess: (data) => {
			toast.success(data);
		},
	});
	function onSubmit(values: z.infer<typeof formSchema>) {
		addNewMutation.mutate(values);
	}

	return (
		<>
			<h1>Create a new Translation Key</h1>
			<Form {...form}>
				<form onSubmit={form.handleSubmit(onSubmit)}>
					<CardContent className="grid gap-6">
						<FormField
							control={form.control}
							name="ts_key"
							render={({ field }) => (
								<FormItem className="grid gap-2">
									<FormLabel>TS_Key</FormLabel>
									<FormControl>
										<Input
											placeholder="ts key"
											{...field}
											autoComplete="off"
											autoCapitalize="off"
										/>
									</FormControl>
									<FormMessage />
								</FormItem>
							)}
						/>

						<FormField
							control={form.control}
							name="json_key"
							render={({ field }) => (
								<FormItem className="grid gap-2">
									<FormLabel>Json_Key</FormLabel>
									<FormControl>
										<Input
											placeholder="json key"
											{...field}
											autoComplete="off"
											autoCapitalize="off"
										/>
									</FormControl>
									<FormMessage />
								</FormItem>
							)}
						/>
						<FormField
							control={form.control}
							name="translation"
							render={({ field }) => (
								<FormItem className="grid gap-2">
									<FormLabel>Translation</FormLabel>
									<FormControl>
										<Input
											placeholder="translation.."
											{...field}
											autoComplete="off"
											autoCapitalize="off"
										/>
									</FormControl>
									<FormDescription>
										The Name the location gets saved as.
									</FormDescription>
									<FormMessage />
								</FormItem>
							)}
						/>
					</CardContent>
					<CardFooter className="justify-between space-x-2">
						<DialogTrigger>
							<Button variant="ghost" type="button">
								Cancel
							</Button>
						</DialogTrigger>

						<DialogTrigger>
							<Button type="submit">Submit</Button>
						</DialogTrigger>
					</CardFooter>
				</form>
			</Form>
		</>
	);
}
