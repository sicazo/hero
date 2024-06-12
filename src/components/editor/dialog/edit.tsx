
import { Button } from "@/components/ui/button";
import {
	CardContent,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { DialogTrigger } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import type {
	TranslationEntry,
	UpdateKeysBody,
	UpdatedKeyValues,
} from "@/lib/procedures";
import { rspc } from "@/lib/rspc";
import { useLocationStore } from "@/lib/stores/location_store";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import {useTranslationStore} from "@/lib/stores/translation_store.ts";

interface EditTranslationDialogProps {
	translation: TranslationEntry;
}

export default function EditTranslationDialog({
	translation,
}: EditTranslationDialogProps) {
	const [translationsJson, setTranslationsJson] = useState("");
	const { last_selected_location } = useLocationStore();
	const {languages} = useTranslationStore();

	useEffect(() => {
		// @ts-expect-error reasons
		const orderedTranslations = Object.keys(translation.translations)
			.sort()
			.reduce((obj, key) => {
				// @ts-expect-error reasons
				obj[key] = translation.translations[key];
				return obj;
			}, {});

		setTranslationsJson(JSON.stringify(orderedTranslations, null, 2));
	}, [translation]);
	const updateMutation = rspc.useMutation(["translations.update_keys"]);

	const update = () => {
		const newTranslationsJson = JSON.parse(translationsJson);
        const filteredTranslationsJson : {[key: string]: string} = {};
		const newChangedValues: {[key: string]: string} = {};

        languages.forEach((language) => {
            if(language in newTranslationsJson){
                filteredTranslationsJson[language] = newTranslationsJson[language];
            }
        });

        Object.keys(filteredTranslationsJson).forEach((key) => {
            if (filteredTranslationsJson[key] !== translation.translations![key]) {
                newChangedValues[key] = filteredTranslationsJson[key];
            }
        });

		const key: UpdatedKeyValues = {
			json_key: translation.value!,
			translation_values: newChangedValues,
			ts_key: translation.key!,
		};
		const body: UpdateKeysBody = {
			key,
			path: last_selected_location?.path as string,
		};
		toast.promise(updateMutation.mutateAsync(body), {
			loading: "Updating...",
			success: "Entry updated",
			error: "There was an error updating the Entry",
		});
	};
	return (
		<>
			<CardHeader>
				<CardTitle>
					Edit <code>{translation.key}</code>
				</CardTitle>
			</CardHeader>
			<CardContent className="grid gap-6">
				<div className="flex gap-2">
					<div className="grid gap-2">
						<Label htmlFor="subject">TS Key</Label>
						<Input
							value={translation.key}
							placeholder="I need help with..."
							autoComplete="off"
							autoCapitalize="off"
							spellCheck={false}
							disabled
						/>
					</div>
					<div className="grid gap-2">
						<Label htmlFor="subject">Json Key</Label>
						<Input
							value={translation.value}
							placeholder="I need help with..."
							autoComplete="off"
							autoCapitalize="off"
							spellCheck={false}
							disabled
						/>
					</div>
				</div>

				<div className="grid h-full w-full gap-2">
					<Label htmlFor="description">
						Here you can change the translations
					</Label>
					<Textarea
						id="description"
						placeholder="Please include all information relevant to your issue."
						value={translationsJson}
						onChange={(e) => setTranslationsJson(e.target.value)}
						className="h-[300px] resize-none"
						spellCheck={false}
					/>
				</div>
			</CardContent>
			<CardFooter className="justify-between space-x-2">
				<DialogTrigger>
					<Button variant="ghost">Cancel</Button>
				</DialogTrigger>
				<DialogTrigger>
					<Button onClick={update}>Submit</Button>
				</DialogTrigger>
			</CardFooter>
		</>
	);
}
