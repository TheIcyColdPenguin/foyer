import { invoke } from "@tauri-apps/api";
import type { Photo, RawPhoto } from "src/types/photo";
export async function fetchPhotos(offset?: number): Promise<Photo[]> {
    const photos = await invoke<RawPhoto[]>("fetch_photos_after", { offset: offset ?? 0 });

    return Promise.all(
        photos.map(async (photo) => ({
            ...photo,
            img_url: await getImageUrl(photo.id),
        }))
    );
}

export async function getImageUrl(id: number) {
    return navigator.userAgent.includes("Windows") ? `https://foyerimg.test/?id=${id}` : `foyerimg://test/?id=${id}`;
}
