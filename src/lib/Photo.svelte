<script lang="ts">
    import type { Photo } from "../types/photo";

    import { fade, fly } from "svelte/transition";

    import { viewing } from "../stores/viewing";

    export let photo: Photo;

    function handleClick() {
        $viewing = photo;
    }
</script>

<div in:fly={{ y: 20 }} class="photo" on:click={handleClick} on:keypress={handleClick}>
    <img in:fade src={photo.img_url} alt="" />
</div>

<style>
    div.photo {
        min-height: 10rem;
        border-radius: 5px;
        background-color: #fff3;
        overflow: hidden;

        transition: all 0.05s ease-in-out;
    }

    div.photo > img {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }

    div.photo:nth-of-type(7n) {
        grid-column: span 2;
        grid-row: span 2;
    }
    div.photo:nth-of-type(9n) {
        grid-column: span 2;
    }
    div.photo:nth-of-type(13n) {
        grid-row: span 2;
    }

    div.photo:hover {
        z-index: 10;
        transform: scale(1.01);
    }
</style>
