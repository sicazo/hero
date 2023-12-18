"use client";
import { Button } from "@/components/ui/button";
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

import { useSettingsStore } from "@/lib/stores";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";
import { Checkbox } from "@/components/ui/checkbox";
import { clsx } from "clsx";

const translationFormSchema = z.object({
	translate_new_strings: z.boolean().default(false),
	translate_updated_strings: z.boolean().default(false),
	default_language: z.string().default("en-GB"),
	translation_command: z
		.string()
		.refine((value) => value.startsWith("yarn translate") || value === "", {
			message:
				'The translation command has to be in the format "yarn translate <translation_key>", or an empty string if you don\'t want to use it.',
		})
		.optional(),
});

export default function TranslationForm() {
	const { updateTranslationSettings, translation_settings } =
		useSettingsStore();
	const defaultValues = {
		...translation_settings,
		translate_new_strings: translation_settings.translate_new_strings,
		translate_updated_strings: translation_settings.translate_updated_strings,
	};

	const form = useForm<z.infer<typeof translationFormSchema>>({
		resolver: zodResolver(translationFormSchema),
		...defaultValues,
	});
	function onSubmit(values: z.infer<typeof translationFormSchema>) {
		console.log(values);
	}
	return (
		<Form {...form}>
			<form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
				<div className="space-y-2">
					<FormField
						control={form.control}
						name="translate_new_strings"
						render={({ field }) => (
							<FormItem className="flex flex-row items-start space-x-3 space-y-0">
								<FormControl>
									<Checkbox
										checked={translation_settings.translate_new_strings}
										onCheckedChange={() =>
											updateTranslationSettings({
												...translation_settings,
												translate_new_strings:
													!translation_settings.translate_new_strings,
											})
										}
									/>
								</FormControl>
								<FormLabel>Translate new translation strings</FormLabel>
							</FormItem>
						)}
					/>
					<FormField
						control={form.control}
						name="translate_updated_strings"
						render={({ field }) => (
							<FormItem className="flex flex-row items-start space-x-3 space-y-0">
								<FormControl>
									<Checkbox
										checked={translation_settings.translate_updated_strings}
										onCheckedChange={() =>
											updateTranslationSettings({
												...translation_settings,
												translate_updated_strings:
													!translation_settings.translate_updated_strings,
											})
										}
									/>
								</FormControl>
								<FormLabel>Translate updated translation strings</FormLabel>
							</FormItem>
						)}
					/>
				</div>

				<FormField
					control={form.control}
					name="translation_command"
					disabled={
						!translation_settings.translate_new_strings &&
						!translation_settings.translate_updated_strings
					}
					render={({ field }) => (
						<FormItem
							className={clsx("", {
								"text-gray-500":
									!translation_settings.translate_new_strings &&
									!translation_settings.translate_updated_strings,
							})}
						>
							<FormLabel>Translation Command</FormLabel>
							<FormControl>
								<Input
									placeholder="yarn translate <translation_key>"
									{...field}
								/>
							</FormControl>
							<FormDescription className="-mx-2">
								This is the translation command that will be used to translate
								strings.
							</FormDescription>
							<FormMessage />
						</FormItem>
					)}
				/>

				<Button
					type="submit"
					disabled={
						!translation_settings.translate_updated_strings &&
						!translation_settings.translate_new_strings
					}
				>
					Update Settings
				</Button>
			</form>
		</Form>
	);
}
