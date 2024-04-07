"use client";

import {
	ColumnDef,
	ColumnFiltersState,
	SortingState,
	flexRender,
	getCoreRowModel,
	getFilteredRowModel,
	getPaginationRowModel,
	getSortedRowModel,
	useReactTable,
} from "@tanstack/react-table";

import { TranslationTableViewOptions } from "@/components/editor/translation_table/column_toggle";
import TranslationTablePagination from "@/components/editor/translation_table/pagination";
import { Button } from "@/components/ui/button";
import { DialogTrigger } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from "@/components/ui/table";
import { TranslationEntry } from "@/lib/procedures";
import { useSize } from "@/lib/hooks/useSize";
import { useLocationStore } from "@/lib/stores/location_store";
import { useTranslationStore } from "@/lib/stores/translation_store";
import { useMutation } from "@tanstack/react-query";
import axios from "axios";
import { useEffect, useState } from "react";
import { toast } from "sonner";

interface TranslationTableProps<TData, TValue> {
	columns: ColumnDef<TData, TValue>[];
	data: TData[];
}

export default function TranslationTable<TData, TValue>({
	columns,
	data,
}: TranslationTableProps<TData, TValue>) {
	const { ref, width, height } = useSize();
	// States
	const [sorting, setSorting] = useState<SortingState>([]);
	const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
	const initialStatePageSize = Math.floor(720 / 80);
	const [rowSelection, setRowSelection] = useState({});
	const [columnVisibility, setColumnVisibility] = useState<
		Record<string, boolean>
	>({ value: false });
	const [pageSize, setPageSize] = useState(initialStatePageSize);
	const { last_selected_location } = useLocationStore();
	const {
		removeKeysFromTranslationEntries,
		translation_entries,
		setTranslationEntries,
	} = useTranslationStore();

	// Requests
	const removeKeyMutation = useMutation({
		mutationKey: ["remove_keys"],
		mutationFn: async ({
			ts_keys,
			json_keys,
		}: { ts_keys: string[]; json_keys: string[] }) => {
			const response = await axios.post(
				"http://localhost:3001/translation/remove",
				{
					path: last_selected_location?.path,
					ts_key: ts_keys,
					json_key: json_keys,
				},
			);
			return response.data;
		},
		onSuccess: (_, variables) => {
			toast.success("The Entries got successfully removed");
			removeKeysFromTranslationEntries(variables.ts_keys);
		},
	});

	const checkLocationMutation = useMutation<{ keys: TranslationEntry[] }>({
		mutationKey: ["check-location"],
		mutationFn: async () => {
			const response = await axios.post(
				"http://localhost:3001/translation/check",
				{
					path: last_selected_location?.path,
				},
			);
			return response.data;
		},
		onSuccess: (data) => {
			updatePossibleLocationChanges(data.keys);
		},
	});

	// Functions

	const updatePossibleLocationChanges = (keys: TranslationEntry[]) => {
		const newTranslationEntries = [...translation_entries];
		for (const key of keys) {
			const existingEntryIndex = newTranslationEntries.findIndex(
				(entry) => entry.key === key.key,
			);
			if (existingEntryIndex !== -1) {
				if (
					JSON.stringify(
						newTranslationEntries[existingEntryIndex].translations,
					) !== JSON.stringify(key.translations)
				) {
					console.info("The translations inside of the keys differ");
				}
				newTranslationEntries[existingEntryIndex] = key;
			}
		}
		setTranslationEntries(newTranslationEntries);
	};
	const removeSelectedKeys = () => {
		const ts_keys: string[] = [];
		const json_keys: string[] = [];
		const selectedRowData = table.getSelectedRowModel();
		for (const row of selectedRowData.rows) {
			//@ts-expect-error
			ts_keys.push(row.original.key);
			//@ts-expect-error
			json_keys.push(row.original.value);
		}
		removeKeyMutation.mutate({ ts_keys, json_keys });
	};

	const table = useReactTable({
		data,
		columns,
		getPaginationRowModel: getPaginationRowModel(),
		getCoreRowModel: getCoreRowModel(),
		onSortingChange: setSorting,
		getSortedRowModel: getSortedRowModel(),
		onColumnFiltersChange: setColumnFilters,
		getFilteredRowModel: getFilteredRowModel(),
		onRowSelectionChange: setRowSelection,
		onColumnVisibilityChange: setColumnVisibility,
		state: {
			sorting,
			columnFilters,
			rowSelection,
			columnVisibility,
		},
		initialState: {
			pagination: {
				pageIndex: 0,
				pageSize: pageSize,
			},
		},
	});

	// Setup
	useEffect(() => {
		const handleResize = () => {
			const newPageSize = Math.floor(height / 80);
			if (newPageSize !== pageSize) {
				table.setPageSize(newPageSize);
				setPageSize(newPageSize);
			}
		};

		window.addEventListener("resize", handleResize);

		return () => window.removeEventListener("resize", handleResize);
	}, [table, pageSize]);

	useEffect(() => {}, []);

	// Misc

	const rowsSelected = table.getIsSomeRowsSelected();

	return (
		<div ref={ref} className="h-screen">
			<div className="flex items-center py-2 justify-between">
				<Input
					placeholder="Filter keys..."
					value={(table.getColumn("key")?.getFilterValue() as string) ?? ""}
					onChange={(event) =>
						table.getColumn("key")?.setFilterValue(event.target.value)
					}
					className="max-w-sm"
				/>
				<div className="flex space-x-2 items-center">
					<div>
						<DialogTrigger>
							<Button className="h-10" variant="default">
								Add
							</Button>
						</DialogTrigger>
					</div>
					<Button
						disabled={!rowsSelected}
						className="h-10"
						variant="destructive"
						onClick={removeSelectedKeys}
					>
						{removeKeyMutation.isPending ? "..." : "Delete Selected Keys"}
					</Button>

					<TranslationTableViewOptions table={table} />
				</div>
			</div>
			<div className="rounded-md border">
				<Table className="overflow-hidden">
					<TableHeader>
						{table.getHeaderGroups().map((headerGroup) => (
							<TableRow key={headerGroup.id}>
								{headerGroup.headers.map((header) => {
									return (
										<TableHead key={header.id}>
											{header.isPlaceholder
												? null
												: flexRender(
														header.column.columnDef.header,
														header.getContext(),
												  )}
										</TableHead>
									);
								})}
							</TableRow>
						))}
					</TableHeader>
					<TableBody>
						{table.getRowModel().rows?.length ? (
							table.getRowModel().rows.map((row) => (
								<TableRow
									key={row.id}
									data-state={row.getIsSelected() && "selected"}
								>
									{row.getVisibleCells().map((cell) => (
										<TableCell key={cell.id}>
											{flexRender(
												cell.column.columnDef.cell,
												cell.getContext(),
											)}
										</TableCell>
									))}
								</TableRow>
							))
						) : (
							<TableRow>
								<TableCell
									colSpan={columns.length}
									className="h-24 text-center"
								>
									No results.
								</TableCell>
							</TableRow>
						)}
					</TableBody>
				</Table>
			</div>
			<div className="p-2">
				<TranslationTablePagination table={table} />
			</div>
		</div>
	);
}
