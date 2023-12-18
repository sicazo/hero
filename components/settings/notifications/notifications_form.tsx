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
import { cn } from "@/lib/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import { clsx } from "clsx";
import { useForm } from "react-hook-form";
import * as z from "zod";

const notificationsFormSchema = z.object({
	file_changes: z.boolean().default(false).optional(),
	finished_translation: z.boolean().default(false).optional(),
	finished_scan: z.boolean().default(false).optional(),
});

type NotificationsFormValues = z.infer<typeof notificationsFormSchema>;

export function NotificationsForm() {
	const {
		notifications_enabled,
		enabled_notification_types,
		updateNotificationTypes,
	} = useSettingsStore();

	const defaultValues: Partial<NotificationsFormValues> = useSettingsStore(
		(state) => state.enabled_notification_types,
	);
	const form = useForm<NotificationsFormValues>({
		resolver: zodResolver(notificationsFormSchema),
		defaultValues,
	});

	function onSubmit(data: NotificationsFormValues) {}

	return (
		<Form {...form}>
			<form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
				<div>
					<h3
						className={clsx("mb-4 text-lg font-medium", {
							"text-gray-400": !notifications_enabled,
						})}
					>
						Notify me about...
					</h3>
					<div className="space-y-4">
						<FormField
							control={form.control}
							name="file_changes"
							render={({ field }) => (
								<FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
									<div className="space-y-0.5">
										<FormLabel
											className={clsx("text-base", {
												"text-gray-400": !notifications_enabled,
											})}
										/>
										<FormDescription
											className={clsx("", {
												"text-gray-400": !notifications_enabled,
											})}
										>
											Receive notifications when translation files in your
											watched locations change.
										</FormDescription>
									</div>
									<FormControl>
										<Switch
											checked={enabled_notification_types.file_changes}
											onCheckedChange={() =>
												updateNotificationTypes({
													...enabled_notification_types,
													file_changes:
														!enabled_notification_types.file_changes,
												})
											}
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
										<FormLabel className="text-base" />
										<FormDescription
											className={clsx("", {
												"text-gray-400": !notifications_enabled,
											})}
										>
											Receive notifications when a translation process finishes.
										</FormDescription>
									</div>
									<FormControl>
										<Switch
											checked={enabled_notification_types.finished_translation}
											onCheckedChange={() =>
												updateNotificationTypes({
													...enabled_notification_types,
													finished_translation:
														!enabled_notification_types.finished_translation,
												})
											}
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
										<FormLabel className="text-base" />
										<FormDescription
											className={clsx("", {
												"text-gray-400": !notifications_enabled,
											})}
										>
											Receive notifications when a directory scan finishes.
										</FormDescription>
									</div>
									<FormControl>
										<Switch
											checked={enabled_notification_types.finished_scan}
											onCheckedChange={() =>
												updateNotificationTypes({
													...enabled_notification_types,
													finished_scan:
														!enabled_notification_types.finished_scan,
												})
											}
											disabled={!notifications_enabled}
										/>
									</FormControl>
								</FormItem>
							)}
						/>
					</div>
				</div>
			</form>
		</Form>
	);
}
