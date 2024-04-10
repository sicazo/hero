"use client";

import {
	Form,
	FormControl,
	FormDescription,
	FormField,
	FormItem,
	FormLabel,
	FormMessage,
} from "@/components/ui/form";
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Switch } from "@/components/ui/switch";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { zodResolver } from "@hookform/resolvers/zod";
import { useTheme } from "next-themes";
import { useForm } from "react-hook-form";
import * as z from "zod";

const appearanceFormSchema = z.object({
	toast_rich_colors: z.boolean(),
	theme: z.enum(["light", "dark"], {
		required_error: "Please select a theme.",
	}),
});

type AppearanceFormValues = z.infer<typeof appearanceFormSchema>;

export default function AppearanceForm() {
	const settings = useSettingsStore();
	const { setTheme } = useTheme();
	const defaultValues: Partial<AppearanceFormValues> = {
		toast_rich_colors: settings.toast_rich_colors,
		theme: settings.theme,
	};
	const form = useForm<AppearanceFormValues>({
		resolver: zodResolver(appearanceFormSchema),
		defaultValues,
	});

	function onSubmit(data: AppearanceFormValues) {
		settings.setTheme(data.theme);
		settings.setToastRichColors(data.toast_rich_colors);
		setTheme(data.theme);
	}

	return (
		<Form {...form}>
			<form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
				<FormField
					control={form.control}
					name="toast_rich_colors"
					render={(field) => (
						<FormItem>
							<FormLabel>Toast Rich Colors</FormLabel>
							<FormDescription>
								Toggles if the inApp toasts are shown with rich colors.
							</FormDescription>
							<FormControl>
								<div className="flex space-x-2 mt-2 align-middle h-auto items-center">
									<Label htmlFor="toast_rich_color">Monochrome</Label>
									<Switch
										id="toast_rich_color"
										checked={settings.toast_rich_colors}
										{...field}
										onCheckedChange={(e) => {
											settings.setToastRichColors(e);
										}}
									/>
									<Label htmlFor="toast_rich_color">Rich Color</Label>
								</div>
							</FormControl>

							{/*<Button onClick={() => {toast.success("test")}}>Test</Button>*/}
						</FormItem>
					)}
				/>
				<FormField
					control={form.control}
					name="theme"
					render={( ) => (
						<FormItem className="space-y-1">
							<FormLabel>Theme</FormLabel>
							<FormDescription>
								Select the theme for the dashboard.
							</FormDescription>
							<FormMessage />
							<RadioGroup
								onValueChange={(theme) => {
									setTheme(theme);
									// @ts-expect-error reasons
									settings.setTheme(theme);
								}}
								defaultValue={settings.theme}
								className="grid max-w-md grid-cols-2 gap-8 pt-2"
							>
								<FormItem>
									<FormLabel className="[&:has([data-state=checked])>div]:border-primary">
										<FormControl>
											<RadioGroupItem value="light" className="sr-only" />
										</FormControl>
										<div className="items-center rounded-md border-2 border-muted p-1 hover:border-accent">
											<div className="space-y-2 rounded-sm bg-[#ecedef] p-2">
												<div className="space-y-2 rounded-md bg-white p-2 shadow-sm">
													<div className="h-2 w-[80px] rounded-lg bg-[#ecedef]" />
													<div className="h-2 w-[100px] rounded-lg bg-[#ecedef]" />
												</div>
												<div className="flex items-center space-x-2 rounded-md bg-white p-2 shadow-sm">
													<div className="h-4 w-4 rounded-full bg-[#ecedef]" />
													<div className="h-2 w-[100px] rounded-lg bg-[#ecedef]" />
												</div>
												<div className="flex items-center space-x-2 rounded-md bg-white p-2 shadow-sm">
													<div className="h-4 w-4 rounded-full bg-[#ecedef]" />
													<div className="h-2 w-[100px] rounded-lg bg-[#ecedef]" />
												</div>
											</div>
										</div>
										<span className="block w-full p-2 text-center font-normal">
											Light
										</span>
									</FormLabel>
								</FormItem>
								<FormItem>
									<FormLabel className="[&:has([data-state=checked])>div]:border-primary">
										<FormControl>
											<RadioGroupItem value="dark" className="sr-only" />
										</FormControl>
										<div className="items-center rounded-md border-2 border-muted bg-popover p-1 hover:bg-accent hover:text-accent-foreground">
											<div className="space-y-2 rounded-sm bg-slate-950 p-2">
												<div className="space-y-2 rounded-md bg-slate-800 p-2 shadow-sm">
													<div className="h-2 w-[80px] rounded-lg bg-slate-400" />
													<div className="h-2 w-[100px] rounded-lg bg-slate-400" />
												</div>
												<div className="flex items-center space-x-2 rounded-md bg-slate-800 p-2 shadow-sm">
													<div className="h-4 w-4 rounded-full bg-slate-400" />
													<div className="h-2 w-[100px] rounded-lg bg-slate-400" />
												</div>
												<div className="flex items-center space-x-2 rounded-md bg-slate-800 p-2 shadow-sm">
													<div className="h-4 w-4 rounded-full bg-slate-400" />
													<div className="h-2 w-[100px] rounded-lg bg-slate-400" />
												</div>
											</div>
										</div>
										<span className="block w-full p-2 text-center font-normal">
											Dark
										</span>
									</FormLabel>
								</FormItem>
							</RadioGroup>
						</FormItem>
					)}
				/>
			</form>
		</Form>
	);
}
