import { DirectRequest } from "$types/api";
import { API_URL } from "$lib/api";
import { selectedVenue } from "$lib/venueStore.ts";
import { get } from "svelte/store";

export async function load({ params, fetch }) {
    const venue: any = get(selectedVenue);

    if(!venue) { return {} };

    let drinksData = null;
    let drinksFetchError = null;

    try {
        let response = await fetch(API_URL + "/drinks/" + venue.identifier);
        let responseObj = new DirectRequest<string>(await response.text());

        if (responseObj.isOk()) {
            let str = await decodeGzippedBase64(responseObj.data);
            let json = JSON.parse(str);
            drinksData = json;
        } else {
            drinksFetchError = responseObj.error_reason;
        }
    } catch (e) {
        drinksFetchError = "Failed to fetch venues: " + e;
    }

    if(drinksFetchError != null || drinksData == null) {
        return { success: false, error_reason: drinksFetchError };
    }

    return { success: true, data: drinksData };
}


