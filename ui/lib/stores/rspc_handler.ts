'use client'


import {StateStorage} from "zustand/middleware";
import axios from "axios";
import {PersistStorage} from "zustand/middleware";
import {StorageValue} from "zustand/middleware";


const rspc_storage: PersistStorage<any> = {
    getItem: async (name: string) : Promise<StorageValue<any> | null> => {
        return null
    },
    setItem: (name: string, value: StorageValue<any>): void | Promise<void> => {
        console.log(value)
    },
    removeItem: (name: string) : void | Promise<void> => {}
};

export default rspc_storage;
