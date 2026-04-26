<script>
    let { 
        options, 
        defaultIndex = 0, 
        bindValue = $bindable() 
    } = $props();
    
    let isOpen = $state(false);
    let selected = $state(options[defaultIndex]);

    $effect(() => {
        if (bindValue !== undefined && bindValue !== selected) selected = bindValue;
    });

    $effect(() => {
        if (selected !== undefined && bindValue !== selected) bindValue = selected;
    });

    function selectOption(value) {
        if (value === selected) return;
        selected = value;
        bindValue = value;
        isOpen = false;
    }

    function handleClickOutside(event) {
        if (isOpen && !event.target.closest('#container')) {
            isOpen = false;
        }
    }
</script>

<svelte:window on:click={handleClickOutside} />
<svelte:body on:click={handleClickOutside} />

<div id="container">
    <div class="select-trigger" on:click={() => isOpen = !isOpen}>
        <p style="width: 100%; text-align: center;">{selected}</p>
        <svg style="transform: translateY(-2px) translateX(-6px);" xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" viewBox="0 0 8 8"><path fill="currentColor" d="m2 3l2 2l2-2l1 1l-3 3l-3-3"/></svg>
    </div>

    {#if isOpen}
        <div class="select-options">
            {#each options as option}
                <div class="option" on:click={() => selectOption(option)}><p>{option}</p></div>
            {/each}
        </div>
    {/if}
</div>

<style>
    #container {
        position: relative;

        min-width: 80px;
        width: 100%;
        height: 30px;

        margin: 5px;
    }

    .select-trigger {
        color: black;

        display: flex;
        align-items: center;
        justify-content: space-between;
        cursor: pointer;

        border: 1px solid #d3d3d3;
        border-radius: 4px;
        background: white;

        width: 100%;
        height: 100%;
        box-sizing: border-box;

        font-weight: 600;
        text-align: center;
    }

    .select-options {
        color: black;

        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        background: white;
        box-shadow: 0 8px 24px rgba(0,0,0,0.15);
        z-index: 10;

        border: 1px solid #d3d3d3;
        border-top: none;
        border-radius: 6px;
        box-sizing: border-box;
    }

    .option {
        cursor: pointer;
        height: 36px;

        display: flex;
        justify-content: center;
        align-items: center;
    }

    .option:hover {
        background: #eee;
    }

    p {
        font-weight: 600;
        font-size: 0.9em;
        opacity: 0.8;
    }

    svg {
        position: absolute;
        right: 0;
    }
</style>

