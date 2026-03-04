<script lang="ts">
    import { DirectRequest } from "$types/api";
    import { API_URL } from "$lib/api";

    let drinks = $state<string | null>(null);
    let error = $state<string | null>(null);
    let loading = $state(true);

    $effect(() => {
        async function fetchData() {
            try {
                const resp = await fetch(API_URL + "/drinks/5600");
                let response = new DirectRequest<string>(await resp.text());

                if (response.isOk()) {
                    let str = decodeGzippedBase64(response.data);
                    let json = JSON.parse(str);
                    drinks.set(json);
                } else {
                    error = "Error, couldn't load drinks";
                }
            } catch (e) {
                error = "Failed to fetch drinks";
            } finally {
                loading = false;
            }
        }

        fetchData();
    });

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

<div class="center-container container">
    <h1>whatsavers</h1>
    <h4>The best financial advisor in town</h4>

    <input type="text" maxlength="50" class="box venue-search">
</div>

<style>
    .container {
        margin-top: 2em;
    }

    .venue-search {
        width: 600px;
        max-width: 90%;
    }
</style>
