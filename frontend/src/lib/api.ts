export const IS_DEV = import.meta.env.MODE === "development";

export const API_URL: string = IS_DEV
  ? "http://127.0.0.1:3000/api"
  : "/api";

export async function decodeGzippedBase64(b64: string): Promise<string> {
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
