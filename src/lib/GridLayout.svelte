<script lang="ts">
    import type { Photo as PhotoType } from "../types/photo";
    import Photo from "./Photo.svelte";

    export let group: PhotoType[];

    let numColumns = 4;
    let cols = Array(numColumns)
        .fill(null)
        .map((_, i) => group.filter((_, j) => j % numColumns === i));
</script>

<div class="grid">
    {#each cols as col, i (i)}
        <div class="col">
            {#each col as photo (photo.id)}
                <Photo {photo} />
            {/each}
        </div>
    {/each}
</div>

<style>
    div.grid {
        display: flex;
        flex-wrap: wrap;
        justify-content: space-evenly;
        gap: 1rem;
    }

    div.col {
        flex: 1;
        max-width: 50%;
    }
</style>
