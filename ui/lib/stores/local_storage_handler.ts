"use client";
import axios from "axios";
import { StateStorage } from "zustand/middleware";

const storage: StateStorage = {
	getItem: async (name): Promise<string> => {
		return (
			await axios.post("http://localhost:3001/store/get", {
				name: name,
			})
		).data;
	},
	setItem: async (name, value): Promise<void> => {
		return await axios.post("http://localhost:3001/store/set", {
			name: name,
			value: value,
		});
	},
	removeItem: async (name): Promise<void> => {
		return await axios.post("http://localhost:3001/store/remove", {
			name: name,
		});
	},
};

export default storage;
