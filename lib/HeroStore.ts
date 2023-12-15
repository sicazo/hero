import {create} from 'zustand'
import {persist, PersistStorage} from "zustand/middleware";
import superjson from 'superjson'
import {immer} from "zustand/middleware/immer"

//TODO: create local settings.json file
const storage: PersistStorage<HeroStoreState> = {
    getItem: (name) => {
        const str = localStorage.getItem(name)
        if (!str) return null
        return superjson.parse(str)
    },
    setItem: (name, value) => {
        localStorage.setItem(name, superjson.stringify(value))
    },
    removeItem: (name) => localStorage.removeItem(name),
}
interface HeroStoreState {
test: number
}

interface HeroStoreActions {

}

export const useHeroStore = create<HeroStoreState & HeroStoreActions>()(
   immer(persist((set, get) => ({
        test: 0
        }), {
       name: "HeroStore",
       storage
       }
    ))
)


