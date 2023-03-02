import { createDir, BaseDirectory } from "@tauri-apps/api/fs";
import { appDataDir, join } from "@tauri-apps/api/path";

import { invoke } from "@tauri-apps/api/tauri";

export async function init() {
    await createDir("databases", { dir: BaseDirectory.AppData, recursive: true });
    connectDatabase();
}

export async function connectDatabase() {
    if (!(await invoke("is_connected_db"))) {
        await invoke("connect_db", { dbPath: await join(await appDataDir(), "databases", "data.db") });
    }
}
