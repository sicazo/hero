import { NotificationsForm } from "@/components/settings/notifications/notifications_form";
import { Separator } from "@/components/ui/separator";
import { Switch } from "@/components/ui/switch";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { createFileRoute } from "@tanstack/react-router";
export const Route = createFileRoute("/settings/notifications")({
	component: Notifications,
});

function Notifications() {
	const { notifications_enabled, setNotifications } = useSettingsStore();
	return (
		<div className="space-y-6 w-full">
			<div className="flex w-full justify-between">
				<div>
					<h3 className="text-lg font-medium">Notifications</h3>
					<p className="text-sm text-muted-foreground">
						Configure how you receive notifications.
					</p>
				</div>
				<div className="flex items-center">
					<Switch
						checked={notifications_enabled}
						onCheckedChange={() => setNotifications(!notifications_enabled)}
						disabled
					/>
				</div>
			</div>
			<Separator />
			<NotificationsForm />
		</div>
	);
}
