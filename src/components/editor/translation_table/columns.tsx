import EditTranslationDialog from "@/components/editor/dialog/edit";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Dialog, DialogContent, DialogTrigger, DialogHeader, DialogTitle, DialogFooter } from "@/components/ui/dialog";
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
import type { TranslationEntry } from "@/lib/procedures";
import { rspc } from "@/lib/rspc";
import { useLocationStore } from "@/lib/stores/location_store";
import { useSettingsStore } from "@/lib/stores/settings_store";
import { useTranslationStore } from "@/lib/stores/translation_store";
import type { ColumnDef } from "@tanstack/react-table";
import { ArrowUpDown, Info, MoreVertical } from "lucide-react";
import { toast } from "sonner";
import { useState } from "react";

export const frontend_columns: ColumnDef<TranslationEntry>[] = [
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
		header: () => {
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
			const default_language = useSettingsStore(
				(state) => state.translation_settings.default_language,
			);
			return (
				//@ts-expect-error reason
				<div className="">{row.original.translations[default_language]}</div>
			);
		},
	},
	{
		accessorKey: "actions",
		header: "",
		cell: ({ row }) => {
			const { last_selected_location } = useLocationStore();
			const { removeKeysFromTranslationEntries } = useTranslationStore();
			const deleteMutation = rspc.useMutation("translations.remove_keys");
			const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);

			const deleteKeys = () => {
				const mutation = deleteMutation.mutateAsync({
					path: last_selected_location?.path as string,
					ts_key: [row.original.value as string],
					json_key: [row.original.value as string],
				});

				toast.promise(mutation, {
					loading: "Removing key...",
					success: "The Entry got successfully removed",
					error: "Failed to delete the translation",
				});
				mutation.then(() => {
					removeKeysFromTranslationEntries([row.original.key as string]);
					setIsDeleteDialogOpen(false);
				});
			};

			return (
				<div className="flex w-auto">
					<Dialog open={isDeleteDialogOpen} onOpenChange={setIsDeleteDialogOpen}>
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
								<DialogTrigger asChild>
									<DropdownMenuItem>
										Delete
									</DropdownMenuItem>
								</DialogTrigger>
							</DropdownMenuContent>
						</DropdownMenu>
						<DialogContent className="w-[80vw]">
							{isDeleteDialogOpen ? (
								<>
									<DialogHeader>
										<DialogTitle>Confirm Deletion</DialogTitle>
									</DialogHeader>
									<div className="py-4">
										<p>Are you sure you want to delete this translation?</p>
									</div>
									<DialogFooter>
										<Button
											variant="outline"
											onClick={() => setIsDeleteDialogOpen(false)}
										>
											Cancel
										</Button>
										<Button variant="destructive" onClick={deleteKeys}>
											Delete
										</Button>
									</DialogFooter>
								</>
							) : (
								<EditTranslationDialog translation={row.original} />
							)}
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

export const backend_columns: ColumnDef<TranslationEntry>[] = [
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
	{
		accessorKey: "key",
		header: ({ column }) => {
			return (
				<Button
					variant="ghost"
					onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
				>
					Key
					<ArrowUpDown className="ml-2 h-4 w-4" />
				</Button>
			);
		},
		cell: ({ row }) => <div className="ml-4">{row.original.key}</div>,
	},
	{
		accessorKey: "default",
		header: ({ column }) => (
			<Button
				variant="ghost"
				className="m-0 p-0"
				onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
			>
				Default Value
				<ArrowUpDown className="ml-2 h-4 w-4" />
			</Button>
		),
		cell: ({ row }) => <div className="ml-4">{row.original.key}</div>,
		enableSorting: true,
		sortDescFirst: false,
	},
	{
		accessorKey: "actions",
		header: "",
		cell: ({ row }) => {
			const { last_selected_location } = useLocationStore();
			const { removeKeysFromTranslationEntries } = useTranslationStore();
			const deleteMutation = rspc.useMutation("translations.remove_keys");
			const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);

			const deleteKeys = () => {
				const mutation = deleteMutation.mutateAsync({
					path: last_selected_location?.path as string,
					ts_key: [row.original.value as string],
					json_key: [row.original.value as string],
				});

				toast.promise(mutation, {
					loading: "Removing key...",
					success: "The Entry got successfully removed",
					error: "Failed to delete the translation",
				});
				mutation.then(() => {
					removeKeysFromTranslationEntries([row.original.key as string]);
					setIsDeleteDialogOpen(false);
				});
			};

			return (
				<div className="flex w-auto">
					<Dialog open={isDeleteDialogOpen} onOpenChange={setIsDeleteDialogOpen}>
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
								<DialogTrigger asChild>
									<DropdownMenuItem>
										Delete
									</DropdownMenuItem>
								</DialogTrigger>
							</DropdownMenuContent>
						</DropdownMenu>
						<DialogContent className="w-[80vw]">
							{isDeleteDialogOpen ? (
								<>
									<DialogHeader>
										<DialogTitle>Confirm Deletion</DialogTitle>
									</DialogHeader>
									<div className="py-4">
										<p>Are you sure you want to delete this translation?</p>
									</div>
									<DialogFooter>
										<Button
											variant="outline"
											onClick={() => setIsDeleteDialogOpen(false)}
										>
											Cancel
										</Button>
										<Button variant="destructive" onClick={deleteKeys}>
											Delete
										</Button>
									</DialogFooter>
								</>
							) : (
								<EditTranslationDialog translation={row.original} />
							)}
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