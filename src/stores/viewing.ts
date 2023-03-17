import type { Photo } from "../types/photo";
import { writable } from "svelte/store";

export const viewing = writable<Photo | null>(null);
