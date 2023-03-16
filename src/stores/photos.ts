import type { Photo } from "src/types/photo";
import { derived, writable, type Writable } from "svelte/store";

export const photos = writable<Photo[]>([]);

export const groupedPhotos = derived<Writable<Photo[]>, Photo[][]>(photos, ($photos) => {
    let grouped = [[]];

    for (let i = 0; i < $photos.length; i++) {
        grouped[grouped.length - 1].push($photos[i]);

        if (isStartOfNewDay($photos, i)) {
            grouped.push([]);
        }
    }

    return grouped.filter((i) => i.length != 0);
});

function isStartOfNewDay($photos: Photo[], i: number) {
    if (i === $photos.length - 1) {
        // end of photos, show day banner
        return false;
    }
    let currPhotoDay = new Date($photos[i].timestamp).toDateString();
    let nextPhotoDay = new Date($photos[i + 1].timestamp).toDateString();

    return currPhotoDay != nextPhotoDay;
}
