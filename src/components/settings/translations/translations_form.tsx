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

import TranslationLanguageDropdown from "@/components/settings/translations/language_dropdown";
import { Checkbox } from "@/components/ui/checkbox";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { zodResolver } from "@hookform/resolvers/zod";
import { clsx } from "clsx";
import { useForm } from "react-hook-form";
import * as z from "zod";
import {toast} from "sonner";

const translationFormSchema = z.object({
	translate_new_strings: z.boolean().default(false),
	translate_updated_strings: z.boolean().default(false),
	default_language: z.string().default("en-GB"),
	translation_command: z
		.string()
		.refine((value) => value.startsWith("yarn run translate") || value === "", {
			message:
				'The translation command has to be in the format "yarn run translate <translation_key>", or an empty string if you don\'t want to use it.',
		})
		.optional(),
});

export default function TranslationForm() {
	const { updateTranslationSettings, translation_settings } =
		useSettingsStore();

	const form = useForm<z.infer<typeof translationFormSchema>>({
		resolver: zodResolver(translationFormSchema),
		defaultValues: {
			translation_command: translation_settings.translation_command,
			translate_new_strings: translation_settings.translate_new_strings,
			translate_updated_strings: translation_settings.translate_updated_strings,
			default_language: translation_settings.default_language,
		},
	});
	function onSubmit(values: z.infer<typeof translationFormSchema>) {
		updateTranslationSettings({
			...translation_settings,
			translation_command: values.translation_command,
		});
		toast.success("Settings updated successfully")
	}
	return (
		<Form {...form}>
			<form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
				<div className="space-y-2">
					<FormField
						control={form.control}
						name="translate_new_strings"
						render={() => (
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
						render={() => (
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
										disabled
									/>
								</FormControl>
								<FormLabel>Translate updated translation strings</FormLabel>
							</FormItem>
						)}
					/>
				</div>
				<div className="flex grow justify-between w-full space-x-5">
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
								<FormDescription className="">
									This is the translation command that will be used to translate
									strings.
								</FormDescription>
								<FormMessage />
							</FormItem>
						)}
					/>
					<FormField
						control={form.control}
						name="default_language"
						render={() => (
							<FormItem className="w-[250px]">
								<FormLabel>Default Language</FormLabel>
								<FormControl className="w-full">
									<TranslationLanguageDropdown />
								</FormControl>
								<FormDescription className="">
									This is the default language that the translations will be
									shown in.
								</FormDescription>
								<FormMessage />
							</FormItem>
						)}
					/>
				</div>

				<Button type="submit">Update Settings</Button>
			</form>
		</Form>
	);
}
