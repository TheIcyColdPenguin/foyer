import type { Tag, TagGroup } from "../types/tags";
import { writable } from "svelte/store";

export const tags = writable<TagGroup>();
export const allTags = writable<Tag[]>([]);
