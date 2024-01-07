"use client";

import { Button } from "@/components/ui/button";
import {
	CardContent,
	CardFooter,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { DialogTrigger } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import z from "zod";
import {useForm} from "react-hook-form";
import {zodResolver} from "@hookform/resolvers/zod";
import {Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage} from "@/components/ui/form";
const formSchema = z.object({
	ts_key: z.string().min(1).max(255),
	json_key: z.string().min(1).max(255),
	translation: z.string().min(1).max(255),
})
export default function AddNewKeyDialog() {

	const form = useForm<z.infer<typeof formSchema>>({
		resolver: zodResolver(formSchema),
		defaultValues: {
			ts_key: "",
			json_key: "",
			translation: "",
		}
	})

	function onSubmit(values: z.infer<typeof formSchema>) {
		console.log(values)
	}

	return (
		<>
			<CardHeader>
				<CardTitle>Create a new Translation Key</CardTitle>
			</CardHeader>
			<CardContent className="grid gap-6">
				<Form {...form}>
					<form onSubmit={form.handleSubmit(onSubmit)}>
						<FormField control={form.control} name="ts_key" render={(field) => (
							<FormItem>
								<FormLabel>TS Key</FormLabel>
								<FormControl>
									<Input placeholder="TS Key" {...field} />
								</FormControl>
								<FormDescription>This is the key added to the messages.ts</FormDescription>
								<FormMessage />
							</FormItem>
						)}/>
						<FormField control={form.control} name="json_key" render={(field) => (
							<FormItem>
								<FormLabel>Json Key</FormLabel>
								<FormControl>
									<Input placeholder="Json Key" {...field} />
								</FormControl>
								<FormDescription>This is the key added to the json files</FormDescription>
								<FormMessage />
							</FormItem>
						)}/>
						<FormField control={form.control} name="translation" render={(field) => (
							<FormItem>
								<FormLabel>TS Key</FormLabel>
								<FormControl>
									<Input placeholder="Translation" {...field} />
								</FormControl>
								<FormDescription>This is the translation added to the en-GB.json</FormDescription>
								<FormMessage />
							</FormItem>
						)}/>
						<CardFooter className="justify-between space-x-2">
				<DialogTrigger>
					<Button variant="ghost" type="button">Cancel</Button>
				</DialogTrigger>
					<Button type="submit">Submit</Button>
			</CardFooter>
					</form>
				</Form>
			</CardContent>
		</>
	);
}
