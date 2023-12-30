"use client";

import { Button } from "@/components/ui/button";
import {
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { DialogTrigger } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { TranslationEntry } from "@/lib/bindings";
import { useEffect, useState } from "react";

interface EditTranslationDialogProps {
	translation: TranslationEntry;
}

export default function EditTranslationDialog({
	translation,
}: EditTranslationDialogProps) {
	const [translationsJson, setTranslationsJson] = useState("");

	useEffect(() => {
		const orderedTranslations = Object.keys(translation.translations)
			.sort()
			.reduce((obj, key) => {
				obj[key] = translation.translations[key];
				return obj;
			}, {});

		setTranslationsJson(JSON.stringify(orderedTranslations, null, 2));
	}, [translation]);

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
						<Input value={translation.key} placeholder="I need help with..." />
					</div>
					<div className="grid gap-2">
						<Label htmlFor="subject">Json Key</Label>
						<Input
							value={translation.value}
							placeholder="I need help with..."
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
						className="h-[300px]"
					/>
				</div>
			</CardContent>
			<CardFooter className="justify-between space-x-2">
				<DialogTrigger>
					<Button variant="ghost">Cancel</Button>
				</DialogTrigger>

				<Button>Submit</Button>
			</CardFooter>
		</>
	);
}
