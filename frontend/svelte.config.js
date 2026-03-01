import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { resolve } from "path";

const config = {
    preprocess: vitePreprocess(),
    kit: {
        adapter: adapter({
            fallback: "index.html",
        }),
        alias: {
            $types: resolve("./src/types"),
            $components: resolve("./src/components"),
            $lib: resolve("./src/lib"),
        }
    },
};

export default config;
