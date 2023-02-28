import { createDir, BaseDirectory } from "@tauri-apps/api/fs";
import { appDataDir, join } from "@tauri-apps/api/path";

import { invoke } from "@tauri-apps/api/tauri";

export async function createAllAppDirectories() {
    await createDir("databases", { dir: BaseDirectory.AppData, recursive: true });
}
export async function connectDatabase() {
    await invoke("connect_db", { dbPath: await join(await appDataDir(), "databases", "data.db") });
}
