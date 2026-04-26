<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from '$app/navigation';
    import Title from "$components/Title.svelte";
    import Dropdown from "$components/Dropdown.svelte";
    import MultiSelect from "svelte-multiselect";
    import { API_URL } from "$lib/api";
    import { DirectRequest } from "$types/api";
    import { type FilterObject, getDefaultFilterObject, applyFilter } from "$types/filters";
    import { selectedVenue } from '$lib/venueStore.js';
    const venue = $selectedVenue;

    let drinksData = $state<any | null>(null);
    let drinksFetchError = $state<any | null>(null);

    let displayedDrinks = $state<any | null>(null);
    let drinkCategories = $state([]);
    let ignoreCategories = $state([]);

    async function getDrinks() {
        if(drinksData == null && drinksFetchError == null) {
            try {
                let response = await fetch(API_URL + "/drinks/" + venue.identifier);
                let responseObj = new DirectRequest<string>(await response.text());

                if (responseObj.isOk()) {
                    let str = await decodeGzippedBase64(responseObj.data);
                    let json = JSON.parse(str);
                    drinksData = postProcessDrinks(json);
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

    function postProcessDrinks(json: any): any {
        return json.map(drink => {
            let ppuGroups = drink.portions.reduce((acc, portion) => {
                let ppuKey = portion.ppu.toFixed(4);
                if(!acc[ppuKey] || portion.amount > acc[ppuKey].amount) {
                    acc[ppuKey] = portion;
                }
                return acc;
            }, {});

            return {
                ...drink,
                portions: Object.values(ppuGroups)
            };
        });
    }

    function getAllCategories(json: any): any {
        let map = {};
        for(let drinkObj of json) {
            map[drinkObj.category] = 0;
        }
        return Object.keys(map);
    }

    onMount(async () => {
        if(venue == null) {
            drinksFetchError = "Error, please re-search your venue"; 
            goto("/");
        }

        await getDrinks();
        drinkCategories = getAllCategories(drinksData);
    });


    /* filters */
    let filterObj = $state<FilterObject>(getDefaultFilterObject());

    $effect(() => {
        filterObj.excludeCategories = ignoreCategories;
        if(drinksData != null && filterObj != null && drinksFetchError == null) {
            displayedDrinks = applyFilter(drinksData, filterObj);
        }
    });

</script>

<div id="center-wrapper" class="center-container">
    <br>
    <h1 class="thick-shadow">{venue ? venue.name : ""}</h1>
    <a style="color: white;" href="/" on:click={() => { goto("/"); }}>Or search again...</a>
    <br>

    {#if drinksFetchError != null}
        <p>Error fetching drink data:<br>{drinksFetchError}</p>
    {:else if drinksData == null}
        <p>Fetching drink data...</p>
    {:else} <!-- here we do have drink data -->

        <div id="container" class="center-container">

            <div id="filter-container" class="center-container box">
                <div class="filter">
                    <label>Max Price £{(Math.round(filterObj.maxPrice * 100) / 100).toFixed(2)}</label>
                    <input type="range" min="0.0" max="10" step="0.1" bind:value={filterObj.maxPrice}>
                </div>
                <div class="filter">
                    <label>Total results</label>
                    <Dropdown options={["5", "10", "15", "20", "All"]} defaultIndex={1} bind:bindValue={filterObj.totalResults} />
                </div>
                <div class="filter">
                    {#if drinkCategories.length != 0}
                        <label>Exclude</label>
                        <MultiSelect bind:selected={ignoreCategories} options={drinkCategories} />
                    {/if}
                </div>
            </div>

            <br>

            <div id="result-container" class="center-container">
                {#if displayedDrinks != null && displayedDrinks.length > 0}
                    {#each displayedDrinks as drink, idx}
                        {@const relativePpu = idx == 0 ? 0 : (((drink.portions[0].ppu/displayedDrinks[0].portions[0].ppu) - 1) * 100)}
                        {@const relativePpuColour = relativePpu <= 50 ? "" : relativePpu <= 80 ? "low-increase" : relativePpu <= 110 ? "med-increase" : "high-increase"}
                        <div class="result box">
                            <div style="width: 80%; display: flex; flex-direction: column; align-items: left;">
                                {#if drink.name[0] == "["}
                                    {@const m = drink.name.match(/^([^]+)\]\s*(.*)$/)}
                                    <span class="drink-name"><span class="drink-deal">{m[1].substring(1)} </span> {m[2]}</span>
                                {:else}
                                    <span class="drink-name">{drink.name}</span>
                                {/if}
                                <div style="display: flex; align-items: left;">
                                    <span class="drink-volume">{drink.portions[0].amount}ml &nbsp;</span>
                                    <span class="drink-strength">{drink.strength.toFixed(1)}%</span>
                                </div>
                                <span class="drink-price">£{drink.portions[0].price.toFixed(2)}{drink.name[0] == "[" ? " each" : ""}</span>
                            </div>
                            <div style="display: flex; flex-direction: column;">
                                <p class="ppu">{drink.medium}</p>
                                <p style="margin-top: auto;" class="plus-price {relativePpuColour}">+{relativePpu.toFixed(0)}%</p>
                                <p class="ppu {relativePpuColour}">£{drink.portions[0].ppu.toFixed(2)}/u</p>
                            </div>
                        </div>
                    {/each}
                    <br><br>
                {:else}
                    <p>No results found</p>
                {/if}
            </div>

        </div>

    {/if}
</div>

<style>
    #center-wrapper {
        width: 100%;
        min-height: 100px;
        text-align: center;

        display: flex;
        justify-content: center;
    }

    #container {
        width: 80%;
        max-width: 600px;
    }

    #filter-container {
        width: 100%;
        max-width: 400px;
        justify-content: center;

        padding: 10px;
        box-sizing: border-box;

        background-color: #eee;

        color: black;
    }

    .filter {
        margin: 5px 0px 5px 0px;
        width: 100%;

        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
    }

    #result-container {
        gap: 5px;

        width: 100%;
        max-width: 400px;
    }

    .result {
        width: 100%;
        min-height: 90px;

        padding: 10px;
        box-sizing: border-box;

        background-color: #fff;

        color: black;
        font-weight: 600;

        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .drink-name {
        font-size: 0.9em;
        font-weight: 600;

        text-align: left;
    }

    .drink-deal {
        color: #fff;

        padding: 1px 5px 1px 5px;
        margin-right: 2px;

        border-radius: 4px;
        background-color: #aa581d;

        font-size: 0.9em;
    }

    .drink-volume, .drink-strength {
        font-size: 0.90em;
        font-weight: 600;
        opacity: 0.45;

        text-align: left;
    }

    .drink-price {
        font-size: 0.95em;
        font-weight: 700;

        text-align: left;
        margin-top: auto;
    }

    .plus-price, .ppu {
        font-size: 0.8em;
        font-weight: 600;
        opacity: 0.45;

        text-align: right;
    }

    :global(.options, .multiselect) {
        color: black;
        font-size: 1em;
        font-weight: 600;
    }

    :global(.multiselect) {
        padding: 5px;
        min-width: 200px;
    }

    :global(.expand-icon) {
        display: none;
    }

    .low-increase {
        color: #bf1717;
        opacity: 0.60;
    }

    .med-increase {
        color: #bc0000;
        opacity: 0.85;
    }

    .high-increase {
        color: #9a0000;
        opacity: 1;
        font-weight: bold;
    }
</style>
