<script lang="ts">
    import { Comparators, SortMethod } from "$types/sorters";
    import { type FilterObject, getDefaultFilterObject, applyFilter } from "$types/filters";
    import Dropdown from "$components/Dropdown.svelte";
    import MultiSelect from "svelte-multiselect";
    import { onMount } from 'svelte';

    // use this to get rid of the annoying keyboard inputs on the phone
    onMount(async () => {
        let inputs = [];
        while(inputs.length == 0) {
            inputs = document.querySelectorAll('.multiselect input');
            await new Promise((r) => setTimeout(r, 20)); // wait until this is in dom, remove items
        }
        for(let i = 0; i < inputs.length; i++) {
            inputs[i].remove();
        }
    });

    let { 
        filterObj = $bindable(), 
        drinkCategories, 
        ignoreCategories = $bindable(), 
    } = $props();
</script>

<div id="filter-container" class="center-container box">
    <div class="filter">
        <label>Max Price: £{(Math.round(filterObj.maxPrice * 100) / 100).toFixed(2)}</label>
        <input type="range" min="0.0" max="10" step="0.1" bind:value={filterObj.maxPrice}>
    </div>
    <div class="filter">
        <label>Sort By</label>
        <Dropdown options={[SortMethod.PricePerUnitAscending, SortMethod.PricePerUnitDescending, SortMethod.Strength, SortMethod.TotalEthanol]} defaultIndex={0} bind:bindValue={filterObj.sortMethod} />
    </div>
    <div class="filter">
        {#if drinkCategories.length != 0}
            <label>Exclude</label>
            <MultiSelect bind:selected={ignoreCategories} options={drinkCategories} />
        {/if}
    </div>
</div>


<style>
    #filter-container {
        width: 100%;
        max-width: 400px;
        justify-content: center;

        padding: 10px;
        box-sizing: border-box;

        color: black;

        background-color: #eee;
        border-top-left-radius: 0px;
        border-top-right-radius: 0px;
    }

    .filter {
        margin: 5px 0px 5px 0px;
        width: 70%;

        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: center;
    }

    label {
        font-size: 0.9em;
        font-weight: 600;

        text-align: left;
    }

    :global(.options) {
        border: 1px solid #d3d3d3;
        max-height: 400px;

        box-shadow:
            0px 4px 16px -11px rgba(0, 0, 0, 0.36),
            0px 8px 8px -5px rgba(0, 0, 0, 0.05);
    }

    :global(.options li, .selected li) {
        font-size: 0.9em;
        opacity: 0.9;
    }

</style>
