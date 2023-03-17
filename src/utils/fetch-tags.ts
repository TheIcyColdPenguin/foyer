import { invoke } from "@tauri-apps/api";
import type { Tag, TagGroup } from "../types/tags";
import { connectDatabase } from "./init";
export async function fetchTags(photoId: number): Promise<TagGroup> {
    await connectDatabase();

    return {
        photo_id: photoId,
        tags: await invoke("fetch_tags", { photoId }),
    };
}
export async function fetchAllTags(): Promise<Tag[]> {
    await connectDatabase();

    return await invoke("fetch_all_tags");
}
