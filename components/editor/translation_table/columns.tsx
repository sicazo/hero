"use client";

import EditTranslationDialog from "@/components/editor/dialog/edit";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuItem,
	DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
	Tooltip,
	TooltipContent,
	TooltipTrigger,
} from "@/components/ui/tooltip";
import { TranslationEntry } from "@/lib/bindings";
import { useLocationStore } from "@/lib/stores/location_store";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { useTranslationStore } from "@/lib/stores/translation_store";
import { DialogTrigger } from "@radix-ui/react-dialog";
import { useMutation } from "@tanstack/react-query";
import { ColumnDef } from "@tanstack/react-table";
import axios from "axios";
import { ArrowUpDown, Info, MoreVertical } from "lucide-react";
import { toast } from "sonner";

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
		sortDescFirst: false,
	},
	{
		accessorKey: "translations",
		header: ({ column }) => {
			// eslint-disable-next-line react-hooks/rules-of-hooks
			const default_language = useSettingsStore(
				(state) => state.translation_settings.default_language,
			);
			return (
				<Tooltip>
					<div className="flex space-x-2 items-center w-[70px]">
						<div>{default_language}</div>
						<TooltipTrigger asChild>
							<Info className="h-4 w-4" />
						</TooltipTrigger>
					</div>
					<TooltipContent side="bottom">
						If you want to change the default language, go to settings -{">"}{" "}
						translations.
					</TooltipContent>
				</Tooltip>
			);
		},
		cell: ({ row }) => {
			// eslint-disable-next-line react-hooks/rules-of-hooks
			const default_language = useSettingsStore(
				(state) => state.translation_settings.default_language,
			);
			return (
				//@ts-expect-error
				<div className="">{row.original.translations[default_language]}</div>
			);
		},
	},
	{
		accessorKey: "actions",
		header: "",
		cell: ({ row }) => {
			const { last_selected_location } = useLocationStore();
			const { removeKeysFromTranslationEntries } =
				useTranslationStore();
			const deleteMutation = useMutation({
				mutationKey: ["delete_translation", row.original.key],
				mutationFn: async () => {
					await axios.post("http://localhost:3001/translation/delete", {
						translations: [
							{
								path: last_selected_location?.path,
								ts_key: [row.original.key],
								json_key:[row.original.value],
							},
						],
					});
				},
				onSuccess: () => {
					toast.success("The Entry got successfully removed")
					removeKeysFromTranslationEntries([row.original.key as string])
				},
				onError: (e) => {
					toast.error(
						<p>
							Failed to delete translation <br />
							<code>{e.message}</code>
						</p>,
					);
				},
			});


			return (
				<div className="flex w-auto">
					<Dialog>
						<DropdownMenu>
							<DropdownMenuTrigger asChild>
								<Button variant="ghost" className="mx-2">
									<MoreVertical className=" w-4" />
								</Button>
							</DropdownMenuTrigger>
							<DropdownMenuContent className="mx-2">
								<DialogTrigger asChild>
									<DropdownMenuItem>Edit</DropdownMenuItem>
								</DialogTrigger>
								<DropdownMenuItem
									onClick={() =>
										toast.warning(
											"Are you sure you want to delete this translation?",
											{
												action: {
													label: "Yes",
													onClick: () => deleteMutation.mutate(),
												},
												cancel: {
													label: "No",
												},
												duration: 15000,
											},
										)
									}
								>
									Delete
								</DropdownMenuItem>
							</DropdownMenuContent>
						</DropdownMenu>
						<DialogContent className="w-[80vw]">
							<EditTranslationDialog translation={row.original} />
						</DialogContent>
					</Dialog>
				</div>
			);
		},
		enableSorting: false,
		enableHiding: false,
		enableResizing: false,
		size: 100,
	},
];
