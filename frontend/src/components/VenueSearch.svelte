<script lang="ts">
    import { DirectRequest } from "$types/api";
    import { API_URL, decodeGzippedBase64 } from "$lib/api";
    import { selectedVenue } from "$lib/venueStore";
    import { goto } from '$app/navigation';


    let venueData = $state<any | null>(null);
    let venueFetchError = $state<string | null>(null);
    let isFetchingData = $state(false);

    let searchQuery = $state<string>("");
    let searchResults = $state<any[] | null>(null);
    // searchResults = [
    //     {"identifier":5362, "name":"The Draper's Arms", "town": "Peterborough", "county": "Cambridgeshire", "is_closed": 0, "post_code":"PE1 1LZ"},
    //     {"identifier":5363, "name":"The Dragon Inn", "town": "Peterborough", "county": "Cambridgeshire", "is_closed": 0, "post_code":"PE1 1LZ"},
    // ];
    // venueData = "hi";
    // searchQuery = "hi";

    async function fetchVenues() {
        if(!isFetchingData && venueData == null && venueFetchError == null) {
            isFetchingData = true;
            try {
                let response = await fetch(API_URL + "/venues");
                let responseObj = new DirectRequest<string>(await response.text());

                if (responseObj.isOk()) {
                    let str = await decodeGzippedBase64(responseObj.data);
                    let json = JSON.parse(str);
                    venueData = json;
                    isFetchingData = false;
                    queryChanged(); // if the user's already typed something, just check and fill results if so
                } else {
                    venueFetchError = responseObj.error_reason;
                    isFetchingData = false;
                }
            } catch (e) {
                venueFetchError = "Failed to fetch venues: " + e;
                isFetchingData = false;
            } finally {
                isFetchingData = false;
            }
        }
    }


    async function queryChanged() {
        fetchVenues();

        if(searchQuery.length < 3 || searchQuery.trim().toLowerCase() == "the") {
            searchResults = null;
            return;
        }

        if(venueData == null || isFetchingData) {
            searchResults = null;
            return;
        }

        searchResults = venueData.filter(
            o => (o.name != undefined && o.name.toLowerCase().includes(searchQuery.toLowerCase()))
            || (o.town != undefined && o.town.toLowerCase().includes(searchQuery.toLowerCase()))
        ).slice(0, 10);
    }

    async function handleVenueClick(venueObj: any) {
        selectedVenue.set(venueObj);
        goto("/drinks");
    }
</script>

<div id="container" class="center-container">
    <input
        placeholder="Find a venue..."
        type="text"
        maxlength="50"
        class="box venue-search {searchQuery != "" ? "no-round-bottom" : ""}"
        bind:value={searchQuery}
        on:input={queryChanged}
        autofocus
    >

    {#if searchQuery != ""}
        <div id="result-container">
            {#if venueFetchError != null}
                <p class="result-error">Error: {venueFetchError}</p>
            {:else if searchResults != null}

                {#if searchResults.length == 0}
                    <p class="result-error">No results</p>
                {:else}
                    
                    {#each searchResults as venue}
                        <a class="search-result" on:click|preventDefault={() => { handleVenueClick(venue)}}>
                            <div class="result-content">
                                <p style="font-weight: 600;">{venue.name}</p>
                                <p style="font-weight: 400; opacity: 0.7; font-size: 0.8em;">{venue.town}</p>
                            </div>
                            <div class="result-content" style="text-align: right;">
                                <p style="font-weight: 400; opacity: 0.7; font-size: 0.8em;">{!venue.is_closed ? "Open" : "Closed"}</p>
                                <p style="font-weight: 400; opacity: 0.7; font-size: 0.8em;">{venue.post_code}</p>
                            </div>
                        </a>
                    {/each}

                {/if}

            {/if}
        </div>
    {/if}
</div>

<style>
    .result-error {
        color: black;
        text-align: center;

        margin: 10px 5px 10px 5px;
        font-weight: 600;
        font-size: 1.1em;
    }

    #container {
        width: 600px;
        max-width: 75%;
    }

    .venue-search {
        width: 100%;
        height: 38px;

        padding: 2px 10px 2px 10px;
        box-sizing: border-box;

        font-size: 0.85em;
        font-weight: 500;
        outline: none;
    }

    .no-round-bottom {
        border-bottom-left-radius: 0px;
        border-bottom-right-radius: 0px;
    }

    #result-container {
        border-top: 1px solid #dadada;
        border-bottom-left-radius: 6px;
        border-bottom-right-radius: 6px;
        overflow: hidden;

        width: 100%;
        min-height: 10px;

        background-color: white;

    }

    .search-result {
        background-color: #eee;
        border-bottom: 1px solid #ddd;

        width: 100%;
        height: 60px;
        box-sizing: border-box;
        padding: 10px 14px 10px 14px;


        display: flex;
        justify-content: space-between;

        color: black;
        font-weight: 500;
    }

    .search-result:hover, .search-result:focus {
        background-color: #cacaca;
        cursor: pointer;
    }

    .result-content {
        display: flex;
        flex-direction: column;
        justify-content: center;
    }
</style>
