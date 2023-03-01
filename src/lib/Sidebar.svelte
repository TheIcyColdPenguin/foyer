<script>
    import { uploadPhotos } from "../utils/upload";
    import { photos } from "../stores/photos";
    import { fetchPhotos } from "../utils/fetch-photos";

    async function handleUpload() {
        await uploadPhotos();
        $photos = await fetchPhotos();
    }
</script>

<section>
    <span />
    <div>
        <button on:click={handleUpload}>
            <img src="/icons/upload.svg" alt="icon" width="80%" />
        </button>
        <button>
            <img src="/icons/all-categories.svg" alt="icon" width="80%" />
        </button>
        <button>
            <img src="/icons/category.svg" alt="icon" width="80%" />
        </button>
        <button>
            <img src="/icons/category.svg" alt="icon" width="80%" />
        </button>
        <button>
            <img src="/icons/category.svg" alt="icon" width="80%" />
        </button>
    </div>
    <button>
        <img src="/icons/settings.svg" alt="icon" width="100%" />
    </button>
</section>

<style>
    section {
        height: 100vh;
        background-color: #ffffff22;

        display: grid;

        grid-template-columns: 1fr;
        grid-template-rows: 2fr 6fr 2fr;

        text-align: center;
        place-items: center;
    }

    div {
        width: 80%;
        height: 80%;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-evenly;
    }

    button {
        display: inline-block;
        padding: 0;
        margin: 0;
        background-color: transparent;
    }

    img {
        width: 100%;
        height: 100%;
        /* object-fit: cover; */
        filter: brightness(0%) saturate(100%) invert(94%) sepia(2%) saturate(5448%) hue-rotate(194deg) brightness(118%)
            contrast(73%);
    }
</style>
