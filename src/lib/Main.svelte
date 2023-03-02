<script lang="ts">
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";

    import { photos } from "../stores/photos";
    import { viewing } from "../stores/viewing";
    import { fetchPhotos } from "../utils/fetch-photos";
    import Photo from "./Photo.svelte";

    let lastScrollTop = 0;

    onMount(async () => {
        if (!$photos.length) {
            $photos = await fetchPhotos();
        }
    });

    async function handleScroll({ currentTarget: elem }: { currentTarget: HTMLElement }) {
        if (elem.scrollTop < lastScrollTop) {
            return; // upscroll
        }
        lastScrollTop = elem.scrollTop <= 0 ? 0 : elem.scrollTop;
        if (elem.scrollTop + elem.offsetHeight >= elem.scrollHeight) {
            $photos = $photos.concat(await fetchPhotos($photos.length));
        }
    }

    function escapeViewing() {
        if ($viewing) {
            $viewing = null;
        }
    }
</script>

<div
    class="container"
    tabindex="-1"
    on:keyup={(e) => {
        if (e.key == "Escape") {
            escapeViewing();
        }
    }}
>
    <h1>Welcome to Your Foyer</h1>
    <main class={$viewing ? "viewing" : ""} on:scroll={(e) => handleScroll(e)}>
        {#each $photos as photo (photo.id)}
            <Photo {photo} />
        {:else}
            <h3>Add some memories</h3>
        {/each}
    </main>
    {#if $viewing}
        <div transition:fade={{ duration: 125 }} class="viewing" on:click={escapeViewing} on:keypress={escapeViewing}>
            <img src={$viewing.img_url} alt="" />
        </div>
    {/if}
</div>

<style>
    div.container {
        height: 100vh;
        outline: none;
    }

    h1 {
        height: 4rem;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    h3 {
        height: calc(100vh - 10rem);
        display: flex;
        justify-content: center;
        align-items: center;
        color: #ccc;
    }

    main {
        min-width: 0;
        max-width: 95%;
        width: 95%;
        max-height: calc(100vh - 10rem);
        overflow-y: visible;
        padding: 1rem 1rem;
        margin: 1rem 1rem;

        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr));
        grid-template-rows: repeat(auto-fit, minmax(10rem, 1fr));
        grid-auto-flow: row dense;

        gap: 0.5rem;
    }

    div.viewing {
        position: absolute;
        top: 0;
        bottom: 0;

        width: 100vw;
        height: 100vh;

        z-index: 100;
        border-radius: 5px;

        background-color: #000c;
    }

    div.viewing > img {
        position: absolute;

        width: 95%;
        height: 95%;

        top: 50%;
        left: 50%;

        transform: translate(-50%, -50%);

        object-fit: contain;
    }
</style>
