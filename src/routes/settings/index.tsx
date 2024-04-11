import TranslationForm from "@/components/settings/translations/translations_form";
import { Separator } from "@/components/ui/separator";
import { createFileRoute } from "@tanstack/react-router";
export const Route = createFileRoute("/settings/")({
	component: RootSettings,
});

function RootSettings() {
	return (
		<div className=" dark:bg-gray-950 space-y-6">
			<div>
				<h3 className="text-lg font-medium">Translations</h3>
				<p className="text-sm text-muted-foreground">
					Here you can edit the translation specific settings for the tool.
				</p>
			</div>
			<Separator />
			<TranslationForm />
		</div>
	);
}
