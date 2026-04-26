<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from '$app/navigation';
    import Title from "$components/Title.svelte";
    import Dropdown from "$components/Dropdown.svelte";
    import MultiSelect from "svelte-multiselect";
    import { API_URL, decodeGzippedBase64 } from "$lib/api";
    import { DirectRequest } from "$types/api";
    import { type FilterObject, getDefaultFilterObject, applyFilter } from "$types/filters";
    import { selectedVenue } from '$lib/venueStore.js';
    import { Comparators, SortMethod } from "$types/sorters";
    import { CollapsibleCard } from 'svelte-collapsible'
    import FilterOptions from "$components/FilterOptions.svelte";

    const venue = $selectedVenue;

    let drinksData = $state<any | null>(null);
    let drinksFetchError = $state<any | null>(null);

    let displayedDrinks = $state<any | null>(null);
    let drinkCategories = $state([]);
    let ignoreCategories = $state([]);
    let isShowingAll = $state(false);

    let filterObj = $state<FilterObject>(getDefaultFilterObject());
    let filterMenuOpen = $state(false);
    
    let absoluteLowestPpu = $state(0.0);


    // this fetches the drinks from the api
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

    // this post-processes the api fetch, returning the list of drinks as an array
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

    function sortDrinksOrder(drinksData: any[], sortMethod: SortMethod): any[] {
        return drinksData.toSorted(Comparators[sortMethod]);
    }

    onMount(async () => {
        if(venue == null) {
            drinksFetchError = "Error, please re-search your venue"; 
            goto("/");
        }

        await getDrinks();
        absoluteLowestPpu = Math.min.apply(Math, drinksData.map((o) => { return o.portions[0].ppu; }));
        drinkCategories = getAllCategories(drinksData);
        console.log(drinksData)
    });


    $effect(() => {
        filterObj.excludeCategories = ignoreCategories;
        if(drinksData != null && filterObj != null && drinksFetchError == null) {
            displayedDrinks = applyFilter(sortDrinksOrder(drinksData, filterObj.sortMethod), filterObj);
            isShowingAll = false;
        }
    });

</script>

<div id="center-wrapper" class="center-container">
    <br>
    <h1 class="thick-shadow" style="font-size: 2.2em;">{venue ? venue.name : ""}</h1>
    <a style="color: white; font-size: 0.95em; font-weight: 600;" href="/" on:click={() => { goto("/"); }}>Or search again...</a>
    <br>

    {#if drinksFetchError != null}
        <p>Error fetching drink data:<br>{drinksFetchError}</p>
    {:else if drinksData == null}
        <p>Fetching drink data...</p>
    {:else} <!-- here we do have drink data -->

        <div id="container" class="center-container">

            <CollapsibleCard bind:open={filterMenuOpen} class="filter-header">
                <div slot="header" id="filter-header" class="center-container box" class:remove-bottom-radius={filterMenuOpen}>
                    <h3>Filters</h3>
                    <svg style="opacity: {filterMenuOpen ? "0.8" : "0.3"}; transition: 0.2s;" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><g fill="none" fill-rule="evenodd"><path d="m12.593 23.258l-.011.002l-.071.035l-.02.004l-.014-.004l-.071-.035q-.016-.005-.024.005l-.004.01l-.017.428l.005.02l.01.013l.104.074l.015.004l.012-.004l.104-.074l.012-.016l.004-.017l-.017-.427q-.004-.016-.017-.018m.265-.113l-.013.002l-.185.093l-.01.01l-.003.011l.018.43l.005.012l.008.007l.201.093q.019.005.029-.008l.004-.014l-.034-.614q-.005-.018-.02-.022m-.715.002a.02.02 0 0 0-.027.006l-.006.014l-.034.614q.001.018.017.024l.015-.002l.201-.093l.01-.008l.004-.011l.017-.43l-.003-.012l-.01-.01z"/><path fill="currentColor" d="M3 4.5A1.5 1.5 0 0 1 4.5 3h15A1.5 1.5 0 0 1 21 4.5v2.086A2 2 0 0 1 20.414 8L15 13.414v7.424a1.1 1.1 0 0 1-1.592.984l-3.717-1.858A1.25 1.25 0 0 1 9 18.846v-5.432L3.586 8A2 2 0 0 1 3 6.586z"/></g></svg>
                </div>
                <div slot="body" id="filter-body">
                    <FilterOptions bind:filterObj={filterObj} drinkCategories={drinkCategories} bind:ignoreCategories={ignoreCategories} />
                </div>
            </CollapsibleCard>

            <br>

            <div id="result-container" class="center-container">
                {#if displayedDrinks != null && displayedDrinks.length > 0}
                    {#each displayedDrinks.slice(0, isShowingAll ? displayedDrinks.length : 10) as drink, idx}
                        {@const relativePpu = (((drink.portions[0].ppu/absoluteLowestPpu) - 1) * 100)}
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
                    <br>
                    {#if !isShowingAll && displayedDrinks.length > 10}
                        <button class="show-all box" type="submit" on:click={() => {isShowingAll = true;}}>Show All</button>
                        <br>
                    {/if}
                    <br>
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

    #filter-header {
        min-width: 200px;
        background-color: white;

        height: 50px;
        flex-direction: row;
        justify-content: space-between;
        box-sizing: border-box;
        padding: 0% 4%;
        
        /* this transition works in a couple to delay remove 'remove-bottom-radius' class */
        transition:
            border-bottom-left-radius 0.22s cubic-bezier(1, 0.10, 1, 0.1),
            border-bottom-right-radius 0.22s cubic-bezier(1, 0.10, 1, 0.1),
            background-color 0.2s linear;
    }

    #filter-header:hover {
        cursor: pointer;
        background-color: #eee;
    }

    .remove-bottom-radius {
        border-bottom-left-radius: 0;
        border-bottom-right-radius: 0;

        /* this transition works in a couple to delay remove 'remove-bottom-radius' class */
        transition:
            border-bottom-left-radius 0s linear,
            border-bottom-right-radius 0s linear,
            background-color 0.2s linear !important;
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

    /* this should be kept same as #result-container */
    :global(.card, .card-header) {
        width: 100%;
        max-width: 400px;
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

    .show-all {
        padding: 12px 30px;
        font-weight: 600;

        background-color: white;
        transition: 0.2s;
    }

    .show-all:hover {
        cursor: pointer;
        background-color: #eee;
    }
</style>
