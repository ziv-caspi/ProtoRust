// place files you want to import through the `$lib` alias in this folder.
import { writable } from "svelte/store";

export const protoConfig = writable<ProtoConfig>({
    dependenciesPath: '',
    currentSelectedMessage: '',
    protoFilePath: '',
    messageTypes: []
});

export type ProtoConfig = {
    dependenciesPath: string,
    protoFilePath: string,
    messageTypes: string[],
    currentSelectedMessage: string
}

export type EndpointConfig = {
    host: string,
    port: number,
    type: string,
    target: string,
    loop: boolean,
    ratePerSec: number,
    routingKey: string,
    username: string,
    password: string,
    speed: number,
    quantity: number
}