import { invoke } from "@tauri-apps/api";
import type { Photo, RawPhoto } from "src/types/photo";
import { connectDatabase } from "./init";
export async function fetchPhotos(offset?: number): Promise<Photo[]> {
    await connectDatabase();

    const photos = await invoke<RawPhoto[]>("fetch_photos_after", { offset: offset ?? 0 });

    return photos.map((photo) => ({
        ...photo,
        img_url: getImageUrl(photo.id),
    }));
}

export function getImageUrl(id: number) {
    return navigator.userAgent.includes("Windows") ? `https://foyerimg.test/?id=${id}` : `foyerimg://test/?id=${id}`;
}
