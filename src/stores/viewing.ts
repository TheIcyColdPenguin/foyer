import type { Photo } from "src/types/photo";
import { writable } from "svelte/store";

export const viewing = writable<Photo | null>(null);
