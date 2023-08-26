import { invoke } from '@tauri-apps/api/tauri'

export async function loadProto(dirPath: string, filePath: string): Promise<string[]> {
    return await invoke('load_proto', {depsPath: dirPath, filePath: filePath}) as string[];
}

export async function generateDefaultMessageJson(dirPath: string, filePath: string, messageName: string) {
    return await invoke('gen_default_json', {depsPath: dirPath, filePath, messageName}) as string;
}