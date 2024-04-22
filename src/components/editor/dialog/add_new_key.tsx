
import { Button } from "@/components/ui/button";
import { CardContent, CardFooter } from "@/components/ui/card";
import { DialogTrigger } from "@/components/ui/dialog";
import {
	Form,
	FormControl,
	FormField,
	FormItem,
	FormLabel,
	FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { type AddNewKeyBody } from "@/lib/procedures";
import { rspc } from "@/lib/rspc";
import { useLocationStore } from "@/lib/stores/location_store";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { useTranslationStore } from "@/lib/stores/translation_store";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";

export const AddNewFrontendKeyDialog = ()=> {
	const { translation_entries, setTranslationEntries } = useTranslationStore();
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
						//@ts-expect-error may be undefined
						(entry) => entry.translations[default_language] === translation,
					),
				"A Translation with that value is already existing.",
			),
	});
	const form = useForm<z.infer<typeof formSchema>>({
		resolver: zodResolver(formSchema),
		mode: "onChange",
	});

	const addNewMutation = rspc.useMutation(["translations.add_key"]);

	function onSubmit(values: z.infer<typeof formSchema>) {
		const body: AddNewKeyBody = {
			path: last_selected_location?.path as string,
			value: values.translation,
			json_key: values.json_key,
			ts_key: values.ts_key,
		};

		const mutation = addNewMutation.mutateAsync(body);
		toast.promise(mutation, {
			loading: "Adding translation....",
			success: () => {
				return `${values.ts_key} has been added`;
			},
			error: "There was an error",
		});
		mutation.then((data) => setTranslationEntries(data));
	}

	return (
		<>
			<h1>Create a new Frontend Translation Key</h1>
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
											placeholder="Translation"
											{...field}
											autoComplete="off"
											autoCapitalize="off"
										/>
									</FormControl>

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


export const AddNewBackendKeyDialog = ()=> {
	const { translation_entries, setTranslationEntries } = useTranslationStore();
	const { default_language } = useSettingsStore(
		(state) => state.translation_settings,
	);
	const { last_selected_location } = useLocationStore();

	const formSchema = z.object({
		key: z
			.string()
			.min(1)
			.max(255)
			.default("")
			.refine(
				(key) => !translation_entries.some((entry) => entry.key === key),
				"A TS Key with that value is already existing.",
			),
		default_value: z
			.string()
			.min(1)
			.max(255)
			.default("")
			.refine(
				(key) => !translation_entries.some((entry) => entry.value === key),
				"A Json Key with that value is already existing.",
			),
		// translation: z
		// 	.string()
		// 	.min(1)
		// 	.max(255)
		// 	.default("")
		// 	.refine(
		// 		(translation) =>
		// 			!translation_entries.some(
		// 				//@ts-expect-error may be undefined
		// 				(entry) => entry.translations[default_language] === translation,
		// 			),
		// 		"A Translation with that value is already existing.",
		// 	),
	});
	const form = useForm<z.infer<typeof formSchema>>({
		resolver: zodResolver(formSchema),
		mode: "onChange",
	});

	const addNewMutation = rspc.useMutation(["translations.add_key"]);
	//
	function onSubmit(values: z.infer<typeof formSchema>) {
	// 	const body: AddNewKeyBody = {
	// 		path: last_selected_location?.path as string,
	// 		value: values.translation,
	// 		json_key: values.json_key,
	// 		ts_key: values.ts_key,
	// 	};
	//
	// 	const mutation = addNewMutation.mutateAsync(body);
	// 	toast.promise(mutation, {
	// 		loading: "Adding translation....",
	// 		success: () => {
	// 			return `${values.ts_key} has been added`;
	// 		},
	// 		error: "There was an error",
	// 	});
	// 	mutation.then((data) => setTranslationEntries(data));
	}

	return (
		<>
			<h1>Create a new Backend Translation Key</h1>
			<Form {...form}>
				<form onSubmit={form.handleSubmit(onSubmit)}>
					<CardContent className="grid gap-6">
						<FormField
							control={form.control}
							name="key"
							render={({ field }) => (
								<FormItem className="grid gap-2">
									<FormLabel>Key</FormLabel>
									<FormControl>
										<Input
											placeholder="Key"
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
							name="default_value"
							render={({ field }) => (
								<FormItem className="grid gap-2">
									<FormLabel>Default Value</FormLabel>
									<FormControl>
										<Input
											placeholder="Default Value"
											{...field}
											autoComplete="off"
											autoCapitalize="off"
										/>
									</FormControl>
									<FormMessage />
								</FormItem>
							)}
						/>
						{/*<FormField*/}
						{/*	control={form.control}*/}
						{/*	name="translation"*/}
						{/*	render={({ field }) => (*/}
						{/*		<FormItem className="grid gap-2">*/}
						{/*			<FormLabel>Translation</FormLabel>*/}
						{/*			<FormControl>*/}
						{/*				<Input*/}
						{/*					placeholder="Translation"*/}
						{/*					{...field}*/}
						{/*					autoComplete="off"*/}
						{/*					autoCapitalize="off"*/}
						{/*				/>*/}
						{/*			</FormControl>*/}

						{/*			<FormMessage />*/}
						{/*		</FormItem>*/}
						{/*	)}*/}
						{/*/>*/}
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
