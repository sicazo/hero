"use client";

import { Separator } from "@/components/ui/separator";

export default function SettingsProfilePage() {
	return (
		<div className="space-y-6">
			<div>
				<h3 className="text-lg font-medium">Translations</h3>
				<p className="text-sm text-muted-foreground">
					Here you can edit the translation specific settings for the tool.
				</p>
			</div>
			<Separator />
		</div>
	);
}
