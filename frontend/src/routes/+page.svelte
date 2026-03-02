<script lang="ts">
    import { TestPayload, DirectRequest } from "$types/api";

    async function fetchData() {
        const resp = await fetch("/api/test");
        let response = new DirectRequest<TestPayload>(await resp.text());

        if(response.isOk()) {
            console.log("happy");
        } else {
            console.log("sad");
        }

        return JSON.stringify(response);
    }
</script>

<div class="prose max-w-none">
    <h1>SPA Frontend</h1>

    <p>
        {#await fetchData()}
            Fetching data from <code>/api/data</code> <br/>
            (should take precisely 3 seconds)
        {:then data}
            {data}
        {/await}
    </p>

    <a href="child-url">Go to another route</a>
</div>
