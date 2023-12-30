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
import { TranslationTablePagination } from "@/components/editor/translation_table/pagination";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from "@/components/ui/table";
import { useEffect, useState } from "react";

interface TranslationTableProps<TData, TValue> {
	columns: ColumnDef<TData, TValue>[];
	data: TData[];
}

export default function TranslationTable<TData, TValue>({
	columns,
	data,
}: TranslationTableProps<TData, TValue>) {
	const [sorting, setSorting] = useState<SortingState>([]);
	const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
	const initialStatePageSize = Math.floor(window.innerHeight / 76);
	const [rowSelection, setRowSelection] = useState({});
	const [columnVisibility, setColumnVisibility] = useState<
		Record<string, boolean>
	>({ value: false });

	const [pageSize, setPageSize] = useState(initialStatePageSize);

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
	useEffect(() => {
		const handleResize = () => {
			const newPageSize = Math.floor(window.innerHeight / 76);
			if (newPageSize !== pageSize) {
				table.setPageSize(newPageSize);
				setPageSize(newPageSize);
			}
		};

		window.addEventListener("resize", handleResize);

		return () => window.removeEventListener("resize", handleResize);
	}, [table, pageSize]);

	const rowsSelected = table.getIsSomeRowsSelected();

	return (
		<div>
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
					<Button
						className="h-auto"
						variant={rowsSelected ? "destructive" : "default"}
						onClick={
							rowsSelected
								? () => {
										console.log("delete");
								  }
								: () => {
										console.log("add");
								  }
						}
					>
						{rowsSelected ? "Delete Rows" : "Add"}
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
