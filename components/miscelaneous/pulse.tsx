import {
	Popover,
	PopoverContent,
	PopoverTrigger,
} from "@/components/ui/popover";

export default function Pulse() {
	return (
		<Popover>
			<PopoverTrigger asChild>
				<div className="w-2 h-2 bg-blue-400 rounded-full -mb-2">
					<div className="w-2 h-2 bg-blue-500 rounded-full pulse-animation" />
				</div>
			</PopoverTrigger>
			<PopoverContent side="top" className="m-2">
				{/*TODO: show recent jobs here*/}
				Tst
			</PopoverContent>
		</Popover>
	);
}
