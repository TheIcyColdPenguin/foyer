<script lang="ts">
    import type { Photo } from "../types/photo";

    import { fade, fly } from "svelte/transition";

    import { viewing } from "../stores/viewing";
    import { fetchTags } from "../utils/fetch-tags";
    import { tags } from "../stores/tags";

    import TagManager from "./TagManager.svelte";

    export let photo: Photo;

    let click: MouseEvent;

    function handleClick() {
        if ($tags) {
            $tags = undefined;
            return;
        }

        $viewing = photo;
    }

    async function handleRightClick(e: MouseEvent) {
        $tags = await fetchTags(photo.id);
        click = e;
    }
</script>

<div
    in:fly={{ y: 20 }}
    class="photo"
    on:click={handleClick}
    on:keypress={handleClick}
    on:contextmenu|preventDefault={handleRightClick}
>
    <img in:fade src={photo.img_url} alt="" />

    <TagManager {click} {photo} />
</div>

<style>
    div.photo {
        border-radius: 5px;
        margin-block: 1rem;
        background-color: #000;
        overflow: hidden;
    }

    div.photo > img {
        width: 100%;
        height: 100%;
        object-fit: cover;
        vertical-align: middle;
        transition: all 0.05s ease-in-out;
    }

    div.photo:hover > img {
        opacity: 0.7;
        transform: scale(1.05);
    }
</style>
