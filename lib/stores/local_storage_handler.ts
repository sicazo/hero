import { commands } from "@/lib/bindings";
import { StateStorage } from "zustand/middleware";

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

export default storage;
