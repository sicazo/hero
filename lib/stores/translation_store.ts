import { TranslationStoreState } from "@/lib/bindings";
import storage from "@/lib/stores/local_storage_handler";
import { useMutation } from "@tanstack/react-query";
import axios from "axios";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

interface TranslationStoreActions {
	test: () => void;
}

export const useTranslationStore = create<
	TranslationStoreState & TranslationStoreActions
>()(
	persist(
		immer((set, get) => ({
			languages: [],
			translation_entries: [],
			test: () => {},
		})),
		{
			name: "translation_store",
			storage: createJSONStorage(() => storage),
			version: 0.0,
		},
	),
);
