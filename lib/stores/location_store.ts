import { Location, LocationStoreState } from "@/lib/bindings";
import storage from "@/lib/stores/local_storage_handler";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";

interface LocationStoreActions {
	addLocation: (x: Location) => void;
	removeLocation: (x: Location) => void;
	updateFavorite: (x: Location) => void;
}
export const useLocationStore = create<
	LocationStoreState & LocationStoreActions
>()(
	persist(
		immer((set, get) => ({
			locations: [],
			addLocation: (x: Location) => {
				set((state) => {
					state.locations.push(x);
				});
			},
			removeLocation: (x: Location) => {
				set((state) => {
					state.locations = state.locations.filter((y) => y.name !== x.name);
				});
			},
			updateFavorite: (x: Location) => {
				set((state) => {
					state.locations = state.locations.map((y) => {
						if (y.name === x.name) {
							y.is_favourite = !y.is_favourite;
						}
						return y;
					});
				});
			},
		})),
		{
			name: "location_store",
			storage: createJSONStorage(() => storage),
			skipHydration: true,
			version: 0.0,
		},
	),
);
