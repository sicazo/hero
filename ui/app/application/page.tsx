"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function Home() {
	const router = useRouter();

	// biome-ignore lint/correctness/useExhaustiveDependencies: redirect to /home as / has no content
	useEffect(() => {
		router.push("/application/home");
	}, []);
}
