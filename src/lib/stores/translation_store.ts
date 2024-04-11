import { TranslationEntry  } from "@/lib/procedures";
import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

interface TranslationStoreActions {
	setTranslationEntries: (x: TranslationEntry[]) => void;
	removeKeysFromTranslationEntries: (x: string[]) => void;
}

interface State {
	languages: string[]; translation_entries: TranslationEntry[]
}

export const useTranslationStore = create<
	State & TranslationStoreActions
>()(
		immer((set) => ({
			languages: [
				"de-DE", "de-AT", "de-CH", "de-LU", "nl-NL", "nl-BE", "en-GB", "en-US", "es-ES", "fr-FR",
				"fr-BE", "fr-CH", "it-IT", "it-CH", "pl-PL", "pt-PT", "hu-HU", "hr-HR", "sr-La", "sl-SI",
				"el-GR", "bg-BG", "ro-RO", "tr-TR", "da-DK", "fi-FI", "nb-NO", "sv-SE", "sk-SK", "cs-CZ",
				"uk-UA", "et-EE", "lt-LT", "lv-LV",
			] as string[],
			translation_entries: [] as TranslationEntry[],
			setTranslationEntries: (x) => set({ translation_entries: x }),
			removeKeysFromTranslationEntries: (x) =>
				set((state) => {
					const new_entries: TranslationEntry[] = [];
					for (const entry of state.translation_entries) {
						if (!x.includes(entry.key as string)) {
							new_entries.push(entry);
						}
					}
					state.translation_entries = new_entries;
				}),
		})

	),
);
