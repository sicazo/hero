import { Button } from "@/components/ui/button";
import { useLocationStore } from "@/lib/stores/location_store.ts";
import { createFileRoute } from "@tanstack/react-router";
import { Link } from "@tanstack/react-router";
import { AlertTriangle } from "lucide-react";
import TranslationManager from "@/components/translation-manager/translation-manager";

export const Route = createFileRoute("/editor/")({
	component: Editor,
});

function Editor() {
	const { locations } = useLocationStore();

	if (locations?.length === 0) {
		return (
			<div className="w-full h-full flex flex-col space-y-5 items-center justify-center">
				<div className="flex space-x-3 items-center">
					<AlertTriangle />
					<h1 className="font-bold text-xl">No locations added yet</h1>
				</div>
				<Link to="/locations" params={{ add: "true" }}>
					<Button>Add one!</Button>
				</Link>
			</div>
		);
	}
	
	return <TranslationManager />;
}
