import { writable } from "svelte/store";

export const selectedVenue = writable<any | null>(null);
