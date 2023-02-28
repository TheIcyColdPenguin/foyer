<script>

    import { uploadPhotos } from "../utils/upload";

</script>
<section>
    <span />
    <div>
        <img src="/icons/upload.svg" alt="icon" width="80%" on:click={uploadPhotos}/>
        <img src="/icons/all-categories.svg" alt="icon" width="80%" />
        <img src="/icons/category.svg" alt="icon" width="80%" />
        <img src="/icons/category.svg" alt="icon" width="80%" />
        <img src="/icons/category.svg" alt="icon" width="80%" />
    </div>
    <img src="/icons/settings.svg" alt="icon" width="100%" />
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

    img {
        filter: brightness(0%) saturate(100%) invert(94%) sepia(2%) saturate(5448%) hue-rotate(194deg) brightness(118%)
            contrast(73%);
    }
</style>
