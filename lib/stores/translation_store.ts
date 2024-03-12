"use client";
import { TranslationEntry, TranslationStoreState } from "@/lib/bindings";
import storage from "@/lib/stores/local_storage_handler";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

interface TranslationStoreActions {
	setTranslationEntries: (x: TranslationEntry[]) => void;
	removeKeysFromTranslationEntries: (x: string[]) => void;
}

export const useTranslationStore = create<
	TranslationStoreState & TranslationStoreActions
>()(
	persist(
		immer((set, get) => ({
			languages: [],
			translation_entries: [],
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
		})),
		{
			name: "translation_store",
			storage: createJSONStorage(() => storage),
			skipHydration: true,
		},
	),
);
