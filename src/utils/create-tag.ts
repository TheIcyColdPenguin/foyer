import { invoke } from "@tauri-apps/api";
import type { Tag } from "../types/tags";
import { connectDatabase } from "./init";
export async function createTag(tagName: string): Promise<number> {
    await connectDatabase();

    return await invoke("create_tag", { tagName });
}

export async function addTag(photoId: number, tagId: number) {
    await connectDatabase();

    await invoke("add_tag", { photoId, tagId });
}
