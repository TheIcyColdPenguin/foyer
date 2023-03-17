<!-- original from https://dev.to/dukenmarga/application-menu-and-context-right-click-menu-using-svelte-46am -->
<script lang="ts">
    import { onMount } from "svelte";
    import { quintInOut } from "svelte/easing";
    import { scale } from "svelte/transition";

    let pos = { x: 0, y: 0 };
    let menu = { h: 0, w: 0 };
    let browser = { h: 0, w: 0 };
    let showMenu = false;

    export let click: MouseEvent;

    onMount(() => {
        showMenu = true;
        browser = {
            w: window.innerWidth,
            h: window.innerHeight,
        };
        pos = {
            x: click.clientX,
            y: click.clientY,
        };
        if (browser.h - pos.y < menu.h) pos.y = pos.y - menu.h;
        if (browser.w - pos.x < menu.w) pos.x = pos.x - menu.w;
    });

    function getContextMenuDimension(node: HTMLElement) {
        menu = {
            h: node.offsetHeight,
            w: node.offsetWidth,
        };
    }
</script>

{#if showMenu}
    <nav
        on:focusout={() => (showMenu = false)}
        transition:scale={{ duration: 150, easing: quintInOut }}
        use:getContextMenuDimension
        style="top:{pos.y}px; left:{pos.x}px"
    >
        <div class="navbar" id="navbar">
            <slot />
        </div>
    </nav>
{/if}

<svelte:window
    on:click={() => {
        showMenu = false;
    }}
    on:contextmenu|preventDefault={() => {
        showMenu = false;
    }}
/>

<style>
    nav {
        position: absolute;
        z-index: 200;
    }

    .navbar {
        overflow: hidden;

        display: inline-flex;
        flex-direction: column;

        border: none;
        width: 14rem;
        border-radius: 5px;
        margin: 0;
        padding: 0;

        z-index: 200;

        color: black;
        background-color: #2f2f2f;
    }
</style>
