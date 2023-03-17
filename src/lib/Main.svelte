<script lang="ts">
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";

    import { photos, groupedPhotos } from "../stores/photos";
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
        {#each $groupedPhotos as group}
            <div class="photo-group">
                <div class="day-banner">
                    <span>
                        {new Date(group[0].timestamp).toLocaleDateString(undefined, { dateStyle: "full" })}
                    </span>
                </div>

                {#each { length: group.length } as _, i (group[i].id)}
                    <Photo photo={group[i]} />
                {/each}
            </div>
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

        display: flex;
        flex-direction: column;
    }

    div.photo-group {
        width: 100%;
        min-width: 100%;
        height: 100%;

        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr));
        grid-template-rows: repeat(auto-fit, 10rem);
        grid-auto-flow: row dense;

        gap: 0.5rem;
    }

    div.day-banner {
        min-height: 10rem;
        max-height: 10rem;
        grid-column: 1/-1;
        grid-row: span 1;

        display: flex;
        justify-content: flex-start;
        align-items: center;
        text-align: left;
        font-size: xx-large;
        transform: translateY(20%);
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
