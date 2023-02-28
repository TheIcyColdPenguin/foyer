<script lang="ts">
    import { fly } from "svelte/transition";
</script>

<div class="container">
    <h1>Welcome to Your Foyer</h1>
    <main>
        {#each Array(100).fill(null) as _}
            <div in:fly={{y:-20}} class="photo">
                <!-- <img src="icons/category.svg" alt="" /> -->
            </div>
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

        gap: 2rem;
    }

    main > div.photo {
        min-height: 10rem;
        border: 2px solid white;
        border-radius: 5px;
        overflow: hidden;
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
</style>
