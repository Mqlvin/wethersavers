<script lang="ts">
    import { onMount } from "svelte";
    import { API_URL } from "$lib/api";
    import { DirectRequest } from "$types/api";
    import { selectedVenue } from '$lib/venueStore.js';
    const venue = $selectedVenue;

    let drinksData = $state<any | null>(null);
    let drinksFetchError = $state<any | null>(null);

    if(venue == null) {
        drinksFetchError = "Error, please re-search your venue"; 
    }

    async function getDrinks() {
        console.log("getting drinks")
        if(drinksData == null && drinksFetchError == null) {
            try {
                let response = await fetch(API_URL + "/drinks/" + venue.identifier);
                let responseObj = new DirectRequest<string>(await response.text());

                if (responseObj.isOk()) {
                    let str = await decodeGzippedBase64(responseObj.data);
                    let json = JSON.parse(str);
                    drinksData = json;
                } else {
                    drinksFetchError = responseObj.error_reason;
                }
            } catch (e) {
                drinksFetchError = "Failed to fetch venues: " + e;
            }
        }
    }

    async function decodeGzippedBase64(b64: string): Promise<string> {
        let binary = atob(b64);
        let bytes = Uint8Array.from(binary, c => c.charCodeAt(0));

        let stream = new ReadableStream<Uint8Array>({
            start(controller) {
                controller.enqueue(bytes);
                controller.close();
            }
        });

        let decompressed = stream.pipeThrough(new DecompressionStream("gzip"));
        let arrayBuffer = await new Response(decompressed).arrayBuffer();

        let jsonText = new TextDecoder("utf-8").decode(arrayBuffer);
        return jsonText;
    }

    onMount(async () => {
        getDrinks();
    });
</script>

<p>
    {#if drinksFetchError != null}
        {drinksFetchError}
    {:else if drinksData != null}
        {JSON.stringify(drinksData)}
    {/if}
    Fetching drinks...
</p>

