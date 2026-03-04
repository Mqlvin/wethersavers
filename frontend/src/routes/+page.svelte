<script lang="ts">
    import { DirectRequest } from "$types/api";
    import { API_URL } from "$lib/api";

    import Title from "$components/Title.svelte";
    import VenueSearch from "$components/VenueSearch.svelte";
</script>

<div class="center-container container">
    <Title />

    <br>

    <VenueSearch />
</div>

<style>
    .container {
        margin-top: 2em;
    }

</style>
