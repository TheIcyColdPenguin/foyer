export interface Tag {
    id: number;
    name: string;
    colour: string;
}

export interface TagGroup {
    photo_id: number;
    tags: Tag[];
}
