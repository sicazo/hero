

import {PersistStorage} from "zustand/middleware";
import {StorageValue} from "zustand/middleware";
import {client} from "@/lib/rspc";
import {LocationStore, SettingsStore, TranslationStore} from "@/lib/procedures";


const rspc_storage_handler: PersistStorage<any> = {
    getItem: async (name: string) : Promise<StorageValue<any> | null> => {
        const data = await client.mutation(["stores.getStore", name])
        return data
    },
    setItem: async (name: string, value: StorageValue<any>): Promise<void> => {
        if (name === "settings_store") {
            await client.mutation(["stores.setStore", value as SettingsStore])
        } else if (name === "translation_store") {
            await client.mutation(["stores.setStore", value as TranslationStore])
        } else if (name === "location_store") {
            await client.mutation(["stores.setStore", value as LocationStore])
        }
    },
    removeItem: async (name: string) : Promise<void> => {
        await client.mutation(["stores.removeStore", name])
    }
};

export default rspc_storage_handler;