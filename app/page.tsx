"use client";

import { Button } from "@/components/ui/button";
import {
	isPermissionGranted,
	requestPermission,
	sendNotification,
} from "@tauri-apps/api/notification";
import Link from "next/link";

export default function Home() {
	return (
		<>
			<Link href={"/settings"}>Settings</Link>
		</>
	);
}
