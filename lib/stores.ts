"use client";
import {
	Notifications,
	SettingsStoreState,
	TranslationStoreState,
	commands, TranslationSettings,
} from "@/lib/bindings";
import { create } from "zustand";
import { StateStorage, createJSONStorage, persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

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
	toggleNav: () => void;
	setTheme: (theme: "light" | "dark") => void;
	setNotifications: (x: boolean) => void;
	updateNotificationTypes: (x: Notifications) => void;
	updateTranslationSettings: (x: TranslationSettings) => void;
	setDefaultLanguage: (x: string) => void;
}
interface TranslationStoreActions {
	test:()=>void
}

export const useSettingsStore = create<
	SettingsStoreState & SettingsStoreActions
>()(
	persist(
		immer((set, get) => ({
			nav_open: true,
			theme: "light",
			notifications_enabled: false,
			enabled_notification_types: {
				file_changes: false,
				finished_translation: false,
				finished_scan: false,
			},
			translation_settings: {
				translate_new_strings: false,
				translate_updated_strings: false,
				default_language: "en-GB",
				translation_command: ""
			},

			// actions
			toggleNav: () =>
				set((state) => {
					state.nav_open = !state.nav_open;
				}),
			setTheme: (theme: "light" | "dark") =>
				set((state) => {
					state.theme = theme;
				}),
			setNotifications: (x) =>
				set((state) => {
					state.notifications_enabled = x;
				}),
			updateNotificationTypes: (x) =>
				set((state) => {
					state.enabled_notification_types = x;
				}),
			updateTranslationSettings: (x) =>
				set((state) => {
					state.translation_settings = x;
				}),
			setDefaultLanguage: (x) =>
				set((state) => {
					state.translation_settings.default_language = x;
				}),
		})),
		{
			name: "settings_store",
			storage: createJSONStorage(() => storage),
			skipHydration: true,
			version: 0.0,
		},
	),
);

export const useTranslationStore = create<
	TranslationStoreState & TranslationStoreActions
>()(
	persist(
		immer((set, get) => ({
			languages: [],
			translation_entries: [],
			test: () => {}
		})),
		{
			name: "translation_store",
			storage: createJSONStorage(() => storage),
			skipHydration: true,
			version: 0.0,
		},
	),
);


export const useLocationStore = create()(immer((set, get) => ({})));
