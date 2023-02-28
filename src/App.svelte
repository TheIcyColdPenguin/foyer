<script lang="ts">
    import { onMount } from "svelte";
    import Main from "./lib/Main.svelte";
    import Sidebar from "./lib/Sidebar.svelte";

    import { createAllAppDirectories, connectDatabase } from "./utils/init.js";

    onMount(async () => {
        createAllAppDirectories();
        connectDatabase();
    });
</script>

<main>
    <Sidebar />
    <Main />
</main>

<style>
    main {
        display: grid;

        grid-template-columns: minmax(3rem, 4%) 1fr;
    }
</style>
