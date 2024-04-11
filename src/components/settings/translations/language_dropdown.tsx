import {Button} from "@/components/ui/button";
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem
} from "@/components/ui/command";
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover";
import {useSettingsStore} from "@/lib/stores/settings_store";
import {useTranslationStore} from "@/lib/stores/translation_store";
import {cn} from "@/lib/utils";
import {ChevronUpDownIcon} from "@heroicons/react/24/outline";
import {CheckIcon} from "@radix-ui/react-icons";
import {useState} from "react";

export default function TranslationLanguageDropdown() {
    const {default_language} = useSettingsStore(
        (state) => state.translation_settings,
    );
    const setDefaultLanguage = useSettingsStore(
        (state) => state.setDefaultLanguage,
    );
    const {languages} = useTranslationStore();
    const [open, setOpen] = useState(false);
    const [value, setValue] = useState(default_language);
    console.log(languages)

    return (
        <Popover open={open} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
                <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={open}
                    className="w-[200px] justify-between"
                >
                    {value}
                    <ChevronUpDownIcon className="ml-2 h-4 w-4 shrink-0 opacity-50"/>
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-[200px] p-0">
                <Command>
                        <CommandInput placeholder="Search language.."/>
                        <CommandEmpty>No Language found</CommandEmpty>
                        <CommandGroup className="max-h-[150px] overflow-y-scroll">
                            {languages.map((language) => {
                                return (
                                    <>
                                        <CommandItem
                                            key={language}
                                            value={language}
                                            onSelect={() => {
                                                setValue(language);
                                                setDefaultLanguage(language);
                                                setOpen(false);
                                            }}
                                            disabled={false}
                                        >
                                            <CheckIcon
                                                className={cn(
                                                    "mr-2 h-4 w-4",
                                                    value === language ? "opacity-100" : "opacity-0",
                                                )}
                                            />
                                            {language}
                                        </CommandItem>
                                    </>
                                )
                            })}

                        </CommandGroup>
                </Command>
            </PopoverContent>
        </Popover>
    );
}
