<script lang="ts">
    import { fly, fade } from "svelte/transition";
    import { onMount } from "svelte";

    import { photos } from "../stores/photos";
    import { fetchPhotos } from "../utils/fetch-photos";

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
</script>

<div class="container">
    <h1>Welcome to Your Foyer</h1>
    <main on:scroll={(e) => handleScroll(e)}>
        {#each $photos as photo (photo.id)}
            <div in:fly={{ y: 20 }} class="photo">
                <img loading="lazy" in:fade src={photo.img_url} alt="" />
            </div>
        {:else}
            <h3>Add some memories</h3>
        {/each}
    </main>
</div>

<style>
    div.container {
        height: 100vh;
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

    main > div.photo {
        min-height: 10rem;
        border-radius: 5px;
        overflow: hidden;

        transition: all 0.05s ease-in-out;
    }

    div.photo > img {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }

    main > div.photo:nth-of-type(7n) {
        grid-column: span 2;
        grid-row: span 2;
    }
    main > div.photo:nth-of-type(9n) {
        grid-column: span 2;
    }
    main > div.photo:nth-of-type(13n) {
        grid-row: span 2;
    }

    div.photo:hover {
        z-index: 10;
        transform: scale(1.01);
    }
</style>
