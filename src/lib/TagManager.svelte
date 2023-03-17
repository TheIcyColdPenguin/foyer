<script lang="ts">
    import { fetchAllTags, fetchTags } from "../utils/fetch-tags";
    import { onMount } from "svelte";
    import { tags, allTags } from "../stores/tags";
    import type { Photo } from "../types/photo";

    import ContextMenu from "./ContextMenu.svelte";
    import { addTag, createTag } from "../utils/create-tag";
    import { search } from "../utils/search";

    export let click: MouseEvent;
    export let photo: Photo;

    onMount(async () => {
        if (!$allTags.length) {
            $allTags = await fetchAllTags();
        }
    });

    async function handleSubmit({ currentTarget: formElem }: { currentTarget: HTMLFormElement }) {
        const input = formElem.children[0] as HTMLInputElement;

        const tagIndex = search(
            input.value,
            $allTags.map((i) => i.name)
        );

        if (tagIndex === -1) {
            const tagId = await createTag(input.value);
            await addTag(photo.id, tagId);
        } else {
            // the tag might already exist
            try {
                await addTag(photo.id, $allTags[tagIndex].id);
            } catch {}
        }

        $allTags = await fetchAllTags();
        $tags = await fetchTags(photo.id);
        input.value = "";
        input.blur();
        formElem.blur();
    }
</script>

{#key click}
    {#if $tags && photo.id == $tags.photo_id}
        <ContextMenu {click}>
            <form on:click|stopPropagation on:keypress|stopPropagation on:submit|preventDefault={handleSubmit}>
                <!-- svelte-ignore a11y-autofocus -->
                <input list="tag-list" placeholder="Find or create a tag" autofocus />
                <datalist id="tag-list">
                    {#each $allTags as eachTag (eachTag.id)}
                        <option value={eachTag.name} />
                    {/each}
                </datalist>
            </form>
            <ul>
                {#each $tags.tags as tag (tag.id)}
                    <li class="tag">
                        <span>{tag.name}</span>
                    </li>
                {:else}
                    <li class="no-tags">
                        <span> No tags found</span>
                    </li>
                {/each}
            </ul>
        </ContextMenu>
    {/if}
{/key}

<style>
    ul {
        margin: 0;
        padding: 0.5rem;
        background-color: #fffc;
    }

    ul li {
        display: block;
        list-style-type: none;
        width: 1fr;
    }
</style>
