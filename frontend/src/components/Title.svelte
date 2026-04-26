<script lang="ts">
    import { API_URL, IS_DEV } from "$lib/api";
    import { onMount } from "svelte";
    import { DirectRequest } from "$types/api";

    let totalSearches = $state(null);
    let totalUniqueVenues = $state(null);

    onMount(async () => {
        let response = await fetch(API_URL + "/stats");
        let responseObj = new DirectRequest<string>(await response.text());

        if(responseObj.isOk()) {
            totalSearches = responseObj.data.total_searches;
            totalUniqueVenues = responseObj.data.total_unique_venues;
        }
    });
</script>

<h1 id="title" class="thick-shadow">wethersavers{IS_DEV ? " [dev.env]" : ""}</h1>
<a id="subtext" class:shown={totalSearches != null} class="hidden">Helping buy <b>{totalSearches}</b> pints across <b>{totalUniqueVenues}</b> pubs!</a>

<style>
    #title {
        font-size: 2.4em;
    }

    #subtext {
        font-size: 0.9em;
        font-weight: 500;
    }

    .hidden {
        opacity: 0;
        transition: 0.2s;
        transform: translateY(10px);
    }

    .shown {
        opacity: 1;
        transform: none;
    }
</style>
