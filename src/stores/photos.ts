import type { Photo } from "src/types/photo";
import { writable } from "svelte/store";

export const photos = writable<Photo[]>([]);
