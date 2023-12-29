"use client";
import {
	Location,
	LocationStoreState,
	Notifications,
	SettingsStoreState,
	TranslationSettings,
	TranslationStoreState,
	commands,
} from "@/lib/bindings";
import storage from "@/lib/stores/local_storage_handler";
import { create } from "zustand";
import { StateStorage, createJSONStorage, persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

interface StatisticsStoreActions {
	test: () => void;
}

export const useStatisticsStore = create<StatisticsStoreActions>()(
	persist(
		immer((set, get) => ({
			// actions
			test: () => {},
		})),
		{
			name: "statistics_store",
			storage: createJSONStorage(() => localStorage),
			skipHydration: true,
			version: 0.0,
		},
	),
);
