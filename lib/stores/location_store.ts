import { LocationStoreState } from "@/lib/bindings";
import storage from "@/lib/stores/local_storage_handler";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

interface LocationStoreActions {
	test: () => void;
}
export const useLocationStore = create<
	LocationStoreState & LocationStoreActions
>()(
	persist(
		immer((set, get) => ({
			locations: [],
			test: () => {},
		})),
		{
			name: "location_store",
			storage: createJSONStorage(() => storage),
			skipHydration: true,
			version: 0.0,
		},
	),
);
