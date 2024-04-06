"use client";
import AddNewKeyDialog from "@/components/editor/dialog/add_new_key";
import LocationSwitcher from "@/components/editor/location_switcher";
import { columns } from "@/components/editor/translation_table/columns";
import TranslationTable from "@/components/editor/translation_table/table";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Separator } from "@/components/ui/separator";
import { useLocationStore } from "@/lib/stores/location_store";
import { useTranslationStore } from "@/lib/stores/translation_store";
import { AlertTriangle } from "lucide-react";
import Link from "next/link";

export default function Page() {
	const { translation_entries } = useTranslationStore();
	const { locations } = useLocationStore();
	if (locations?.length === 0) {
		return (
			<div className="w-full h-full flex flex-col space-y-5 items-center justify-center">
				<div className="flex space-x-3 items-center">
					<AlertTriangle />
					<h1 className="font-bold text-xl">No locations added yet</h1>
				</div>
				<Link
					href={{
						pathname: "/locations",
						query: { add: "true" },
					}}
				>
					<Button>Add one!</Button>
				</Link>
			</div>
		);
	}
	return (
		<div className="p-5 overflow-hidden">
			<LocationSwitcher />
			<Separator className="my-2" />
			<Dialog>
				<TranslationTable columns={columns} data={translation_entries} />
				<DialogContent>
					<AddNewKeyDialog />
				</DialogContent>
			</Dialog>
		</div>
	);
}
