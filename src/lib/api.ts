import { invoke } from '@tauri-apps/api/tauri'

export async function loadProto(dirPath: string, filePath: string): Promise<string[]> {
    return await invoke('load_proto', {depsPath: dirPath, filePath: filePath}) as string[];
}

export async function generateDefaultMessageJson(dirPath: string, filePath: string, messageName: string) {
    return await invoke('gen_default_json', {depsPath: dirPath, filePath, messageName}) as string;
}

export async function publishRabbitMessage(dirPath: string, filePath: string, messageName: string, params: RabbitMqParams, json: string) {
    console.log('testing rabbit');
    return await invoke('publish_rabbitmq_message', {
        includesDir: dirPath,
        protoFile: filePath,
        messageName,
        json,
        params,
    });
}

export type RabbitMqParams = {
    target_name: string,
    is_queue: boolean,
    routing_key: string,
    host: string,
    port: number,
    username: string,
    password: string
}