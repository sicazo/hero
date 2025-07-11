import { Location, LocationStoreState } from "@/lib/procedures";
import { create } from "zustand";
import {  persist } from "zustand/middleware";
import { immer } from "zustand/middleware/immer";
import rspc_storage_handler from "@/lib/stores/rspc_handler";

interface LocationStoreActions {
	addLocation: (x: Location) => void;
	removeLocation: (x: Location) => void;
	updateFavorite: (x: Location) => void;
	setLastSelectedLocation: (x: Location) => void;
	updateLocation: (x: Location) => void;
	setLocations: (x: Location[]) => void;
}
export const useLocationStore = create<
	LocationStoreState & LocationStoreActions
>()(
	persist(
		immer((set) => ({
			last_selected_location: null,
			locations: [],
			setLocations: (x: Location[]) => {
				set((state) => {
					state.locations = x
				})
			},
			addLocation: (x: Location) => {
				set((state) => {
					state.locations?.push(x);
				});
			},
			removeLocation: (x: Location) => {
				set((state) => {
					state.locations = state.locations?.filter((y) => y.name !== x.name);
				});
			},
			updateFavorite: (x: Location) => {
				set((state) => {
					state.locations = state.locations?.map((y) => {
						if (y.name === x.name) {
							y.is_favourite = !y.is_favourite;
						}
						return y;
					});
				});
			},
			setLastSelectedLocation: (x: Location) => {
				set((state) => {
					state.last_selected_location = x;
				});
			},
			updateLocation: (x: Location) => {
				set((state) => {
					state.locations = state.locations?.map((y) => {
						if (y.name === x.name) {
							return x;
						}
						return y;
					});
				});
			},
		})),
		{
			name: "location_store",
			// storage: createJSONStorage(() => storage),
			storage: rspc_storage_handler,
		},
	),
);
