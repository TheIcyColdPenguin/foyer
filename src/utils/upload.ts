import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
export async function uploadPhotos() {
    const folder = await open({
        directory: true,
        multiple: false,
        recursive: true,
        title: "Choose a folder containing photos to be added",
    });

    if (!folder) {
        return;
    }

    let folders: string[];

    if (typeof folder == "string") {
        folders = [folder];
    } else {
        folders = folder;
    }

    console.log("upload", { folders });
}
