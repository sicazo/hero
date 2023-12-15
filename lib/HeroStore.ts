'use client'
import {commands} from "@/lib/bindings"
import {create} from 'zustand'
import {createJSONStorage, persist, PersistStorage, StateStorage} from "zustand/middleware";
import { immer } from "zustand/middleware/immer";


//TODO: create local settings.json file
const storage:  StateStorage = {
    getItem: async (name,): Promise<string> => {
        return await commands.getStore(name)

    },
    setItem: async (name, value): Promise<void> => {
        await commands.updateStore(name, value)
    },
    removeItem: async (name): Promise<void> => {await commands.removeStore(name)},
}

interface HeroStoreState {
test: number
}

interface HeroStoreActions {
    updateTest:(x: number) => void
}

export const useHeroStore = create<HeroStoreState & HeroStoreActions>()(
   persist(immer((set, get) => ({
        test: 0,
       updateTest: (x: number) => set((state) => {state.test = x})
        })), {name: 'hero_store', storage: createJSONStorage(() => storage), skipHydration: true})
)


