<script lang="ts">
    import { DirectRequest } from "$types/api";
    import { API_URL } from "$lib/api";

    async function fetchData() {
        const resp = await fetch(API_URL + "/drinks");
        let response = new DirectRequest<string>(await resp.text());

        if(response.isOk()) {

            let str = decodeGzippedBase64(response.data);
            let json = JSON.parse(str);
            return json;

        } else {
            return "Error, couldn't load drinks";
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
