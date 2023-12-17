"use client";

import { Button } from "@/components/ui/button";
import {
	Form,
	FormControl,
	FormDescription,
	FormField,
	FormItem,
	FormLabel,
} from "@/components/ui/form";
import { Switch } from "@/components/ui/switch";
import { toast } from "@/components/ui/use-toast";
import { useSettingsStore } from "@/lib/stores";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";

const notificationsFormSchema = z.object({
	file_changes: z.boolean().default(false).optional(),
	finished_translation: z.boolean().default(false).optional(),
	finished_scan: z.boolean().default(false).optional(),
});

type NotificationsFormValues = z.infer<typeof notificationsFormSchema>;

export function NotificationsForm() {
	const { notifications_enabled } = useSettingsStore();

	const defaultValues: Partial<NotificationsFormValues> = useSettingsStore(
		(state) => state.enabled_notification_types,
	);
	const form = useForm<NotificationsFormValues>({
		resolver: zodResolver(notificationsFormSchema),
		defaultValues,
	});

	function onSubmit(data: NotificationsFormValues) {
		toast({
			title: "You submitted the following values:",
			description: (
				<pre className="mt-2 w-[340px] rounded-md bg-slate-950 p-4">
					<code className="text-white">{JSON.stringify(data, null, 2)}</code>
				</pre>
			),
		});
	}

	return (
		<Form {...form}>
			<form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
				<div>
					<h3 className="mb-4 text-lg font-medium">Notify me about...</h3>
					<div className="space-y-4">
						<FormField
							control={form.control}
							name="file_changes"
							render={({ field }) => (
								<FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
									<div className="space-y-0.5">
										<FormLabel className="text-base"></FormLabel>
										<FormDescription>
											Receive notifications when translation files in your
											watched locations change.
										</FormDescription>
									</div>
									<FormControl>
										<Switch
											checked={field.value}
											onCheckedChange={field.onChange}
											disabled={!notifications_enabled}
										/>
									</FormControl>
								</FormItem>
							)}
						/>
						<FormField
							control={form.control}
							name="finished_translation"
							render={({ field }) => (
								<FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
									<div className="space-y-0.5">
										<FormLabel className="text-base"></FormLabel>
										<FormDescription>
											Receive notifications when a translation process finishes.
										</FormDescription>
									</div>
									<FormControl>
										<Switch
											checked={field.value}
											onCheckedChange={field.onChange}
											disabled={!notifications_enabled}
										/>
									</FormControl>
								</FormItem>
							)}
						/>
						<FormField
							control={form.control}
							name="finished_scan"
							render={({ field }) => (
								<FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
									<div className="space-y-0.5">
										<FormLabel className="text-base"></FormLabel>
										<FormDescription>
											Receive notifications when a directory scan finishes.
										</FormDescription>
									</div>
									<FormControl>
										<Switch
											checked={field.value}
											onCheckedChange={field.onChange}
											disabled={!notifications_enabled}
										/>
									</FormControl>
								</FormItem>
							)}
						/>
					</div>
				</div>

				<Button type="submit" disabled={!notifications_enabled}>
					Update notifications
				</Button>
			</form>
		</Form>
	);
}
