// place files you want to import through the `$lib` alias in this folder.
import { writable } from "svelte/store";

export const dependenciesPath = writable<string>();
export const currentProtoFilePath = writable<string>();
export const currentSelectedMessageName = writable<string>();
export const messageTypesNames = writable<string[]>();

export type EndpointConfig = {
    host: string,
    type: string,
    target: string,
    loop: boolean,
    ratePerSec: number,
    routingKey: string,
    username: string,
    password: string
}