"use client";

import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { TranslationEntry } from "@/lib/bindings";
import { useLocationStore } from "@/lib/stores/location_store";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { ColumnDef } from "@tanstack/react-table";
import { ArrowUpDown, Pencil, Trash2 } from "lucide-react";

export const columns: ColumnDef<TranslationEntry>[] = [
	{
		id: "select",
		header: ({ table }) => (
			<Checkbox
				checked={
					table.getIsAllPageRowsSelected() ||
					(table.getIsSomePageRowsSelected() && "indeterminate")
				}
				onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
				aria-label="Select all"
			/>
		),
		cell: ({ row }) => (
			<Checkbox
				checked={row.getIsSelected()}
				onCheckedChange={(value) => row.toggleSelected(!!value)}
				aria-label="Select row"
			/>
		),
		enableSorting: false,
		enableHiding: false,
	},
	// {
	// 	accessorKey: "in_use",
	// 	header: "In Use",
	// },
	{
		accessorKey: "key",
		header: ({ column }) => {
			return (
				<Button
					variant="ghost"
					onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
				>
					Typescript Key
					<ArrowUpDown className="ml-2 h-4 w-4" />
				</Button>
			);
		},
		cell: ({ row }) => <div className="ml-4">{row.original.key}</div>,
	},
	{
		accessorKey: "value",
		header: ({ column }) => (
			<Button
				variant="ghost"
				className="m-0 p-0"
				onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
			>
				Json Key
				<ArrowUpDown className="ml-2 h-4 w-4" />
			</Button>
		),
		cell: ({ row }) => <div className="ml-4">{row.original.key}</div>,
		enableHiding: true,
		enableSorting: true,
	},
	{
		accessorKey: "translations",
		header: ({ column }) => {
			const default_language = useSettingsStore(
				(state) => state.translation_settings.default_language,
			);
			return <div>{default_language}</div>;
		},
		cell: ({ row }) => {
			const default_language = useSettingsStore(
				(state) => state.translation_settings.default_language,
			);
			return <div>{row.original.translations[default_language]}</div>;
		},
	},
	{
		accessorKey: "actions",
		header: "",
		cell: ({ row }) => (
			<div className="flex w-[40px]">
				<Button variant="ghost" className="">
					<Pencil className=" w-4" />
				</Button>
				<Button variant="ghost" className="">
					<Trash2 className=" w-4" />
				</Button>
			</div>
		),
		enableSorting: false,
		enableHiding: false,
		enableResizing: false,
		size: 100,
	},
];
