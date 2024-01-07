"use client";

import { useLocationStore } from "@/lib/stores/location_store";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { useTranslationStore } from "@/lib/stores/translation_store";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function Home() {
	const router = useRouter();
	useEffect(() => {
		console.log("rehydrate");
		useSettingsStore.persist.rehydrate();
		useLocationStore.persist.rehydrate();
		useTranslationStore.persist.rehydrate();
		router.push("/home");
	}, []);
	useEffect(() => {}, []);
	return null;
}
