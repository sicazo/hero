"use client";
import AddNewKeyDialog from "@/components/editor/dialog/add_new_key";
import LocationSwitcher from "@/components/editor/location_switcher";
import { columns } from "@/components/editor/translation_table/columns";
import TranslationTable from "@/components/editor/translation_table/table";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Separator } from "@/components/ui/separator";
import { useTranslationStore } from "@/lib/stores/translation_store";

export default function Page() {
	const { translation_entries } = useTranslationStore();
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
