"use client";
import {
	SettingsStoreState,
	TranslationStoreState,
	commands,
} from "@/lib/bindings";
import { create } from "zustand";
import { StateStorage, createJSONStorage, persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";
import {useTheme} from "next-themes";

const storage: StateStorage = {
	getItem: async (name): Promise<string> => {
		return await commands.getStore(name);
	},
	setItem: async (name, value): Promise<void> => {
		await commands.updateStore(name, value);
	},
	removeItem: async (name): Promise<void> => {
		await commands.removeStore(name);
	},
};

interface SettingsStoreActions {
	toggleNav: () => void,
	setTheme: (theme: "light" | "dark") => void,
}
interface TranslationStoreActions {
	updateTest: (x: number) => void;
}

export const useSettingsStore = create<
	SettingsStoreState & SettingsStoreActions
>()(
	persist(
		immer((set, get) => ({
			nav_open: true,
			theme: 'light',
			translation_command: "",
			run_translation_on_change: false,
			watch_directories: false,

			// actions
			toggleNav: () => set((state) => {state.nav_open = !state.nav_open}),
			setTheme: (theme: "light" | "dark") => set((state) => {
				state.theme = theme
			}),
		})),
		{
			name: "settings_store",
			storage: createJSONStorage(() => storage),
			skipHydration: true,
			version: 0.0
		},
	),
);

export const useTranslationStore = create<
	TranslationStoreState & TranslationStoreActions
>()(
	persist(
		immer((set, get) => ({
			test: 0,
			updateTest: (x: number) =>
				set((state) => {
					state.test = x;
				}),
		})),
		{
			name: "translation_store",
			storage: createJSONStorage(() => storage),
			skipHydration: true,
			version: 0.0
		},
	),
);
