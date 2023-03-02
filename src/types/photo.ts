export interface RawPhoto {
    id: number;
    ext: number;
    timestamp: string;
}

export interface Photo extends RawPhoto {
    img_url: string;
}
