<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from '$app/navigation';
    import Title from "$components/Title.svelte";
    import Dropdown from "$components/Dropdown.svelte";
    import { API_URL } from "$lib/api";
    import { DirectRequest } from "$types/api";
    import { type FilterObject, getDefaultFilterObject, applyFilter } from "$types/filters";
    import { selectedVenue } from '$lib/venueStore.js';
    const venue = $selectedVenue;

    let drinksData = $state<any | null>(null);
    let drinksFetchError = $state<any | null>(null);

    if(venue == null) {
        drinksFetchError = "Error, please re-search your venue"; 
    }

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

    onMount(async () => {
        await getDrinks();
        // drinksFetchError = null;
        // drinksData = null;
        
        // await new Promise(resolve => setTimeout(resolve, 100));

        // drinksData = [{"name":"Birrificio Italiano - Nigredo","category":"Real ale","strength":6,"portions":[{"amount":568,"price":2.99,"strength":6,"ppu":0.87734747},{"amount":284,"price":1.5,"strength":6,"ppu":0.88028175}]},{"name":"Stowford Press Apple cider","category":"Draught cider","strength":4.5,"portions":[{"amount":568,"price":2.49,"strength":4.5,"ppu":0.9741783},{"amount":284,"price":1.25,"strength":4.5,"ppu":0.9780907}]},{"name":"Almogaver - Vicious IPA","category":"Real ale","strength":5.3,"portions":[{"amount":568,"price":2.99,"strength":5.3,"ppu":0.99322337},{"amount":284,"price":1.5,"strength":5.3,"ppu":0.9965452}]},{"name":"Daleside - Export","category":"Real ale","strength":5,"portions":[{"amount":568,"price":2.99,"strength":5,"ppu":1.052817},{"amount":284,"price":1.5,"strength":5,"ppu":1.0563381}]},{"name":"Kopparberg Sweet Vintage Pear","category":"Bottled cider","strength":7,"portions":[{"amount":500,"price":3.84,"strength":7,"ppu":1.0971428}]},{"name":"Westons Old Rosie Cloudy Cider","category":"Draught cider","strength":6.8,"portions":[{"amount":284,"price":2.12,"strength":6.8,"ppu":1.097763},{"amount":568,"price":4.24,"strength":6.8,"ppu":1.097763}]},{"name":"Greene King Ruddles Best","category":"Real ale","strength":3.4,"portions":[{"amount":284,"price":1.1,"strength":3.4,"ppu":1.139188},{"amount":568,"price":2.2,"strength":3.4,"ppu":1.139188}]},{"name":"Newcastle Brown Ale","category":"World beers","strength":4.7,"portions":[{"amount":550,"price":3.02,"strength":4.7,"ppu":1.1682786}]},{"name":"Shipyard American Pale Ale","category":"Craft","strength":4.5,"portions":[{"amount":568,"price":2.99,"strength":4.5,"ppu":1.1697965},{"amount":284,"price":1.5,"strength":4.5,"ppu":1.1737088}]},{"name":"Shipyard American Pale Ale","category":"Draught craft","strength":4.5,"portions":[{"amount":568,"price":2.99,"strength":4.5,"ppu":1.1697965},{"amount":284,"price":1.5,"strength":4.5,"ppu":1.1737088}]},{"name":"Greene King Abbot Ale","category":"Real ale","strength":5,"portions":[{"amount":284,"price":1.67,"strength":5,"ppu":1.1760564},{"amount":568,"price":3.34,"strength":5,"ppu":1.1760564}]},{"name":"Stan’s Cheddar Valley","category":"Draught cider","strength":6,"portions":[{"amount":284,"price":2.12,"strength":6,"ppu":1.2441314},{"amount":568,"price":4.24,"strength":6,"ppu":1.2441314}]},{"name":"Bud Light","category":"Lager","strength":3.4,"portions":[{"amount":568,"price":2.49,"strength":3.4,"ppu":1.2893537},{"amount":284,"price":1.25,"strength":3.4,"ppu":1.2945318}]},{"name":"Worthington’s Creamflow","category":"Beer","strength":3.4,"portions":[{"amount":568,"price":2.49,"strength":3.4,"ppu":1.2893537},{"amount":284,"price":1.25,"strength":3.4,"ppu":1.2945318}]},{"name":"Thornbridge - Pink Grapefruit Pale","category":"Real ale","strength":4,"portions":[{"amount":568,"price":2.99,"strength":4,"ppu":1.3160212},{"amount":284,"price":1.5,"strength":4,"ppu":1.3204226}]},{"name":"Strongbow","category":"Draught cider","strength":4.5,"portions":[{"amount":284,"price":1.71,"strength":4.5,"ppu":1.3380281},{"amount":568,"price":3.42,"strength":4.5,"ppu":1.3380281}]},{"name":"Sharp’s Doom Bar","category":"Real ale","strength":4,"portions":[{"amount":284,"price":1.53,"strength":4,"ppu":1.3468311},{"amount":568,"price":3.06,"strength":4,"ppu":1.3468311}]},{"name":"Aspall Cyder","category":"Bottled cider","strength":5.5,"portions":[{"amount":500,"price":3.84,"strength":5.5,"ppu":1.3963636}]},{"name":"Leffe Blonde","category":"Lager","strength":6,"portions":[{"amount":568,"price":5.05,"strength":6,"ppu":1.4818077},{"amount":284,"price":2.53,"strength":6,"ppu":1.4847419}]},{"name":"Seven Bro7hers Easy IPA","category":"Craft","strength":4.7,"portions":[{"amount":568,"price":3.99,"strength":4.7,"ppu":1.4946059},{"amount":284,"price":2,"strength":4.7,"ppu":1.4983518}]},{"name":"Mad Squirrel $umo","category":"Craft","strength":4.7,"portions":[{"amount":568,"price":3.99,"strength":4.7,"ppu":1.4946059},{"amount":284,"price":2,"strength":4.7,"ppu":1.4983518}]},{"name":"Seven Bro7hers Easy IPA","category":"Draught craft","strength":4.7,"portions":[{"amount":568,"price":3.99,"strength":4.7,"ppu":1.4946059},{"amount":284,"price":2,"strength":4.7,"ppu":1.4983518}]},{"name":"Mad Squirrel $umo","category":"Draught craft","strength":4.7,"portions":[{"amount":568,"price":3.99,"strength":4.7,"ppu":1.4946059},{"amount":284,"price":2,"strength":4.7,"ppu":1.4983518}]},{"name":"Madri","category":"World beers","strength":4.6,"portions":[{"amount":660,"price":4.59,"strength":4.6,"ppu":1.5118577}]},{"name":"BrewDog Punk IPA","category":"Craft","strength":5.2,"portions":[{"amount":568,"price":4.49,"strength":5.2,"ppu":1.5201787},{"amount":284,"price":2.25,"strength":5.2,"ppu":1.5235645}]},{"name":"BrewDog Punk IPA","category":"Draught craft","strength":5.2,"portions":[{"amount":568,"price":4.49,"strength":5.2,"ppu":1.5201787},{"amount":284,"price":2.25,"strength":5.2,"ppu":1.5235645}]},{"name":"Angry Orchard","category":"Bottled cider","strength":5,"portions":[{"amount":500,"price":3.84,"strength":5,"ppu":1.536}]},{"name":"Bodebrown - Tropical","category":"Real ale","strength":3.4,"portions":[{"amount":568,"price":2.99,"strength":3.4,"ppu":1.5482601},{"amount":284,"price":1.5,"strength":3.4,"ppu":1.5534383}]},{"name":"Asahi","category":"World beers","strength":5,"portions":[{"amount":660,"price":5.4,"strength":5,"ppu":1.6363637}]},{"name":"Staropramen","category":"World beers","strength":5,"portions":[{"amount":660,"price":5.4,"strength":5,"ppu":1.6363637}]},{"name":"Strongbow Dark Fruit","category":"Draught cider","strength":4,"portions":[{"amount":284,"price":1.86,"strength":4,"ppu":1.6373241},{"amount":568,"price":3.72,"strength":4,"ppu":1.6373241}]},{"name":"Poretti","category":"Lager","strength":4.8,"portions":[{"amount":568,"price":4.49,"strength":4.8,"ppu":1.6468604},{"amount":284,"price":2.25,"strength":4.8,"ppu":1.6505282}]},{"name":"Heineken","category":"World beers","strength":5,"portions":[{"amount":650,"price":5.4,"strength":5,"ppu":1.6615385}]},{"name":"Tyskie","category":"World beers","strength":5,"portions":[{"amount":650,"price":5.4,"strength":5,"ppu":1.6615385}]},{"name":"Budweiser","category":"Lager","strength":4.5,"portions":[{"amount":568,"price":4.29,"strength":4.5,"ppu":1.6784036},{"amount":284,"price":2.15,"strength":4.5,"ppu":1.6823161}]},{"name":"Kopparberg Mango","category":"Bottled cider","strength":4,"portions":[{"amount":500,"price":3.39,"strength":4,"ppu":1.695}]},{"name":"Kopparberg Mixed Fruit","category":"Bottled cider","strength":4,"portions":[{"amount":500,"price":3.39,"strength":4,"ppu":1.695}]},{"name":"Kopparberg Strawberry & Lime","category":"Bottled cider","strength":4,"portions":[{"amount":500,"price":3.39,"strength":4,"ppu":1.695}]},{"name":"Bulmers Original","category":"Bottled cider","strength":4.5,"portions":[{"amount":500,"price":3.84,"strength":4.5,"ppu":1.7066666}]},{"name":"Carlsberg Pilsner","category":"Lager","strength":3.8,"portions":[{"amount":568,"price":3.69,"strength":3.8,"ppu":1.7095997},{"amount":284,"price":1.85,"strength":3.8,"ppu":1.7142327}]},{"name":"1664 Bière","category":"Lager","strength":4.6,"portions":[{"amount":568,"price":4.49,"strength":4.6,"ppu":1.7184628},{"amount":284,"price":2.25,"strength":4.6,"ppu":1.7222902}]},{"name":"Carling","category":"Lager","strength":4,"portions":[{"amount":568,"price":3.99,"strength":4,"ppu":1.756162},{"amount":284,"price":2,"strength":4,"ppu":1.7605635}]},{"name":"Coors","category":"Lager","strength":4,"portions":[{"amount":568,"price":3.99,"strength":4,"ppu":1.756162},{"amount":284,"price":2,"strength":4,"ppu":1.7605635}]},{"name":"Birra Moretti ","category":"World beers","strength":4.6,"portions":[{"amount":660,"price":5.4,"strength":4.6,"ppu":1.7786561}]},{"name":"Corona Extra","category":"Lager","strength":4.5,"portions":[{"amount":568,"price":4.59,"strength":4.5,"ppu":1.7957746},{"amount":284,"price":2.3,"strength":4.5,"ppu":1.7996868}]},{"name":"Stella Artois","category":"Lager","strength":4.6,"portions":[{"amount":284,"price":2.36,"strength":4.6,"ppu":1.8064909},{"amount":568,"price":4.72,"strength":4.6,"ppu":1.8064909}]},{"name":"Guinness","category":"Stout","strength":4.1,"portions":[{"amount":568,"price":4.39,"strength":4.1,"ppu":1.8850912},{"amount":284,"price":2.2,"strength":4.1,"ppu":1.8893853}]},{"name":"Bulmers Crushed Red Berries & Lime","category":"Bottled cider","strength":4,"portions":[{"amount":500,"price":3.84,"strength":4,"ppu":1.92}]},{"name":"Thatchers Blood Orange","category":"Bottled cider","strength":4,"portions":[{"amount":500,"price":3.84,"strength":4,"ppu":1.92}]},{"name":"Erdinger","category":"World beers","strength":5.3,"portions":[{"amount":500,"price":5.21,"strength":5.3,"ppu":1.9660376}]},{"name":"Desperados","category":"World beers","strength":5.9,"portions":[{"amount":330,"price":3.84,"strength":5.9,"ppu":1.9722651}]},{"name":"Efes","category":"World beers","strength":5,"portions":[{"amount":500,"price":5.21,"strength":5,"ppu":2.084}]},{"name":"Estrella Galicia","category":"World beers","strength":5.5,"portions":[{"amount":330,"price":3.84,"strength":5.5,"ppu":2.1157024}]},{"name":"Wray & Nephew","category":"Rum","strength":63,"portions":[{"amount":50,"price":7.7,"strength":63,"ppu":2.4444444},{"amount":25,"price":5.95,"strength":63,"ppu":3.7777774}]},{"name":"Au Vodka juicy peach","category":"Vodka","strength":35.2,"portions":[{"amount":50,"price":4.39,"strength":35.2,"ppu":2.494318},{"amount":25,"price":2.64,"strength":35.2,"ppu":3.0000002}]},{"name":"Au Vodka blue raspberry","category":"Vodka","strength":35.2,"portions":[{"amount":50,"price":4.39,"strength":35.2,"ppu":2.494318},{"amount":25,"price":2.64,"strength":35.2,"ppu":3.0000002}]},{"name":"Au Vodka pineapple crush","category":"Vodka","strength":35.2,"portions":[{"amount":50,"price":4.39,"strength":35.2,"ppu":2.494318},{"amount":25,"price":2.64,"strength":35.2,"ppu":3.0000002}]},{"name":"Au Vodka pink lemonade","category":"Vodka","strength":35.2,"portions":[{"amount":50,"price":4.39,"strength":35.2,"ppu":2.494318},{"amount":25,"price":2.64,"strength":35.2,"ppu":3.0000002}]},{"name":"Au Vodka strawberry burst","category":"Vodka","strength":35.2,"portions":[{"amount":50,"price":4.39,"strength":35.2,"ppu":2.494318},{"amount":25,"price":2.64,"strength":35.2,"ppu":3.0000002}]},{"name":"Au Vodka bubblegum","category":"Vodka","strength":35.2,"portions":[{"amount":50,"price":4.39,"strength":35.2,"ppu":2.494318},{"amount":25,"price":2.64,"strength":35.2,"ppu":3.0000002}]},{"name":"Smirnoff Mango & Passionfruit Twist","category":"Vodka","strength":35,"portions":[{"amount":50,"price":4.39,"strength":35,"ppu":2.5085714},{"amount":25,"price":2.64,"strength":35,"ppu":3.017143}]},{"name":"Smirnoff Raspberry Crush","category":"Vodka","strength":35,"portions":[{"amount":50,"price":4.39,"strength":35,"ppu":2.5085714},{"amount":25,"price":2.64,"strength":35,"ppu":3.017143}]},{"name":"Budweiser","category":"World beers","strength":4.5,"portions":[{"amount":330,"price":3.84,"strength":4.5,"ppu":2.5858586}]},{"name":"Corona Extra","category":"World beers","strength":4.5,"portions":[{"amount":330,"price":3.84,"strength":4.5,"ppu":2.5858586}]},{"name":"Southern Comfort","category":"Liqueurs","strength":35,"portions":[{"amount":50,"price":4.64,"strength":35,"ppu":2.6514285},{"amount":25,"price":2.89,"strength":35,"ppu":3.3028572}]},{"name":"Smirnoff Ice","category":"Premixed drinks","strength":4,"portions":[{"amount":275,"price":3.02,"strength":4,"ppu":2.7454545}]},{"name":"Strika","category":"Liqueurs","strength":35,"portions":[{"amount":50,"price":4.82,"strength":35,"ppu":2.7542858},{"amount":25,"price":3.07,"strength":35,"ppu":3.5085714}]},{"name":"Jack Daniel’s Tennessee Apple liqueur","category":"Liqueurs","strength":35,"portions":[{"amount":50,"price":4.82,"strength":35,"ppu":2.7542858},{"amount":25,"price":3.07,"strength":35,"ppu":3.5085714}]},{"name":"Jack Daniel’s Tennessee Apple liqueur","category":"Shots","strength":35,"portions":[{"amount":50,"price":4.82,"strength":35,"ppu":2.7542858},{"amount":25,"price":3.07,"strength":35,"ppu":3.5085714}]},{"name":"Tanqueray Gin","category":"Gin","strength":41.3,"portions":[{"amount":50,"price":5.9,"strength":41.3,"ppu":2.857143},{"amount":25,"price":4.15,"strength":41.3,"ppu":4.0193706}]},{"name":"Fireball Cinnamon Whisky liqueur","category":"Liqueurs","strength":33,"portions":[{"amount":50,"price":4.82,"strength":33,"ppu":2.9212122},{"amount":25,"price":3.07,"strength":33,"ppu":3.7212121}]},{"name":"Tanqueray No. Ten","category":"Gin","strength":47.3,"portions":[{"amount":50,"price":7.01,"strength":47.3,"ppu":2.9640594},{"amount":25,"price":5.26,"strength":47.3,"ppu":4.448203}]},{"name":"Absolut Vanilia","category":"Vodka","strength":38,"portions":[{"amount":50,"price":5.64,"strength":38,"ppu":2.968421},{"amount":25,"price":3.89,"strength":38,"ppu":4.094737}]},{"name":"El Sueño MODA Pineapple Tequila","category":"Tequila","strength":35,"portions":[{"amount":50,"price":5.24,"strength":35,"ppu":2.9942856},{"amount":25,"price":3.49,"strength":35,"ppu":3.9885714}]},{"name":"Jose Cuervo Especial Silver","category":"Tequila","strength":35,"portions":[{"amount":50,"price":5.24,"strength":35,"ppu":2.9942856},{"amount":25,"price":3.49,"strength":35,"ppu":3.9885714}]},{"name":"Smirnoff vodka and Monster","category":"Vodka","strength":37.5,"portions":[{"amount":50,"price":5.64,"strength":37.5,"ppu":3.008},{"amount":25,"price":3.89,"strength":37.5,"ppu":4.1493335}]},{"name":"Beefeater London Blood Orange","category":"Gin","strength":37.5,"portions":[{"amount":50,"price":5.64,"strength":37.5,"ppu":3.008},{"amount":25,"price":3.89,"strength":37.5,"ppu":4.1493335}]},{"name":"Captain Morgan White","category":"Rum","strength":37.5,"portions":[{"amount":50,"price":5.64,"strength":37.5,"ppu":3.008},{"amount":25,"price":3.89,"strength":37.5,"ppu":4.1493335}]},{"name":"Smirnoff vodka and Monster","category":"Long drinks","strength":37.5,"portions":[{"amount":50,"price":5.64,"strength":37.5,"ppu":3.008},{"amount":25,"price":3.89,"strength":37.5,"ppu":4.1493335}]},{"name":"Cazcabel coffee liqueur with tequila blanco","category":"Tequila","strength":34,"portions":[{"amount":50,"price":5.24,"strength":34,"ppu":3.0823526},{"amount":25,"price":3.49,"strength":34,"ppu":4.105882}]},{"name":"Bombay Sapphire","category":"Gin","strength":40,"portions":[{"amount":50,"price":6.43,"strength":40,"ppu":3.215},{"amount":25,"price":4.68,"strength":40,"ppu":4.68}]},{"name":"Captain Morgan Original Spiced Gold","category":"Rum","strength":35,"portions":[{"amount":50,"price":5.64,"strength":35,"ppu":3.222857},{"amount":25,"price":3.89,"strength":35,"ppu":4.4457145}]},{"name":"Antica Sambuca","category":"Shots","strength":38,"portions":[{"amount":25,"price":3.07,"strength":38,"ppu":3.2315788}]},{"name":"Antica Sambuca Raspberry","category":"Shots","strength":38,"portions":[{"amount":25,"price":3.07,"strength":38,"ppu":3.2315788}]},{"name":"Antica Sambuca Apple","category":"Shots","strength":38,"portions":[{"amount":25,"price":3.07,"strength":38,"ppu":3.2315788}]},{"name":"Disaronno amaretto","category":"Liqueurs","strength":28,"portions":[{"amount":50,"price":4.64,"strength":28,"ppu":3.3142858},{"amount":25,"price":2.89,"strength":28,"ppu":4.1285715}]},{"name":"Courvoisier VS Cognac","category":"Cognac and Brandy","strength":40,"portions":[{"amount":50,"price":6.67,"strength":40,"ppu":3.335},{"amount":25,"price":4.92,"strength":40,"ppu":4.92}]},{"name":"Hendrick’s Gin","category":"Gin","strength":41.4,"portions":[{"amount":50,"price":7.01,"strength":41.4,"ppu":3.3864732},{"amount":25,"price":5.26,"strength":41.4,"ppu":5.082125}]},{"name":"Gordon’s","category":"Gin","strength":37.5,"portions":[{"amount":50,"price":6.43,"strength":37.5,"ppu":3.4293332},{"amount":25,"price":4.68,"strength":37.5,"ppu":4.9919996}]},{"name":"Smirnoff","category":"Vodka","strength":37.5,"portions":[{"amount":50,"price":6.62,"strength":37.5,"ppu":3.5306666},{"amount":25,"price":4.87,"strength":37.5,"ppu":5.1946664}]},{"name":"The Kraken Black Spiced Rum","category":"Rum","strength":40,"portions":[{"amount":50,"price":7.13,"strength":40,"ppu":3.565},{"amount":25,"price":5.38,"strength":40,"ppu":5.38}]},{"name":"The Kraken Black Spiced Rum Black Cherry & Madagascan Vanilla","category":"Rum","strength":40,"portions":[{"amount":50,"price":7.13,"strength":40,"ppu":3.565},{"amount":25,"price":5.38,"strength":40,"ppu":5.38}]},{"name":"Gordon’s Pink Gin","category":"Gin","strength":35,"portions":[{"amount":50,"price":6.43,"strength":35,"ppu":3.6742857},{"amount":25,"price":4.68,"strength":35,"ppu":5.3485713}]},{"name":"Fireball cinnamon whisky liqueur","category":"Shots","strength":33,"portions":[{"amount":25,"price":3.07,"strength":33,"ppu":3.7212121}]},{"name":"Bacardi Carta Blanca","category":"Rum","strength":37.5,"portions":[{"amount":50,"price":7.13,"strength":37.5,"ppu":3.8026667},{"amount":25,"price":5.38,"strength":37.5,"ppu":5.738667}]},{"name":"Skittlebomb","category":" Bombs and tequila","strength":40,"portions":[{"amount":25,"price":3.89,"strength":40,"ppu":3.89}]},{"name":"Krakenbomb","category":" Bombs and tequila","strength":40,"portions":[{"amount":25,"price":3.89,"strength":40,"ppu":3.89}]},{"name":"Grey Goose","category":"Vodka","strength":40,"portions":[{"amount":50,"price":7.89,"strength":40,"ppu":3.945},{"amount":25,"price":5.64,"strength":40,"ppu":5.64}]},{"name":"Jose Cuervo Especial Silver","category":" Bombs and tequila","strength":35,"portions":[{"amount":25,"price":3.49,"strength":35,"ppu":3.9885714}]},{"name":"Cazcabel coffee liqueur with tequila blanco","category":" Bombs and tequila","strength":34,"portions":[{"amount":25,"price":3.49,"strength":34,"ppu":4.105882}]},{"name":"Raspberrybomb","category":" Bombs and tequila","strength":37.5,"portions":[{"amount":25,"price":3.89,"strength":37.5,"ppu":4.1493335}]},{"name":"SoCoLocobomb","category":" Bombs and tequila","strength":35,"portions":[{"amount":25,"price":3.89,"strength":35,"ppu":4.4457145}]},{"name":"Flävar blueberry & Lemon","category":"Vodka","strength":21,"portions":[{"amount":50,"price":4.71,"strength":21,"ppu":4.4857144},{"amount":25,"price":2.96,"strength":21,"ppu":5.6380954}]},{"name":"Flävar salted caramel","category":"Vodka","strength":21,"portions":[{"amount":50,"price":4.71,"strength":21,"ppu":4.4857144},{"amount":25,"price":2.96,"strength":21,"ppu":5.6380954}]},{"name":"Flävar strawberry & lime","category":"Vodka","strength":21,"portions":[{"amount":50,"price":4.71,"strength":21,"ppu":4.4857144},{"amount":25,"price":2.96,"strength":21,"ppu":5.6380954}]},{"name":"Captain Morgan Tiki","category":"Rum","strength":25,"portions":[{"amount":50,"price":5.64,"strength":25,"ppu":4.512},{"amount":25,"price":3.89,"strength":25,"ppu":6.224}]},{"name":"Fireballbomb","category":" Bombs and tequila","strength":33,"portions":[{"amount":25,"price":3.89,"strength":33,"ppu":4.715152}]},{"name":"Malibu","category":"Rum","strength":18,"portions":[{"amount":50,"price":4.64,"strength":18,"ppu":5.1555557},{"amount":25,"price":2.89,"strength":18,"ppu":6.4222226}]},{"name":"Malibu","category":"Liqueurs","strength":18,"portions":[{"amount":50,"price":4.64,"strength":18,"ppu":5.1555557},{"amount":25,"price":2.89,"strength":18,"ppu":6.4222226}]},{"name":"Archers peach schnapps","category":"Liqueurs","strength":18,"portions":[{"amount":50,"price":4.82,"strength":18,"ppu":5.355556},{"amount":25,"price":3.07,"strength":18,"ppu":6.822222}]},{"name":"Archers peach schnapps ","category":"Shots","strength":18,"portions":[{"amount":50,"price":4.82,"strength":18,"ppu":5.355556},{"amount":25,"price":3.07,"strength":18,"ppu":6.822222}]},{"name":"Amarettobomb","category":" Bombs and tequila","strength":28,"portions":[{"amount":25,"price":3.89,"strength":28,"ppu":5.557143}]},{"name":"Edinburgh Gin Rhubarb & Ginger Liqueur","category":"Gin","strength":20,"portions":[{"amount":50,"price":5.64,"strength":20,"ppu":5.64},{"amount":25,"price":3.89,"strength":20,"ppu":7.78}]},{"name":"Chambord Black Raspberry liqueur","category":"Liqueurs","strength":16.5,"portions":[{"amount":50,"price":4.82,"strength":16.5,"ppu":5.8424244},{"amount":25,"price":3.07,"strength":16.5,"ppu":7.4424243}]},{"name":"Kahlúa","category":"Liqueurs","strength":16,"portions":[{"amount":50,"price":4.82,"strength":16,"ppu":6.025},{"amount":25,"price":3.07,"strength":16,"ppu":7.6749997}]},{"name":"Flävarbomb","category":" Bombs and tequila","strength":25,"portions":[{"amount":25,"price":3.89,"strength":25,"ppu":6.224}]},{"name":"CORKY’S Blueberry ","category":"Shots","strength":15,"portions":[{"amount":25,"price":2.96,"strength":15,"ppu":7.893333}]},{"name":"CORKY’S Raspberry ","category":"Shots","strength":15,"portions":[{"amount":25,"price":2.96,"strength":15,"ppu":7.893333}]},{"name":"CORKY’S Sour Apple","category":"Shots","strength":15,"portions":[{"amount":25,"price":2.96,"strength":15,"ppu":7.893333}]},{"name":"CORKY’S Sour Cherry","category":"Shots","strength":15,"portions":[{"amount":25,"price":2.96,"strength":15,"ppu":7.893333}]},{"name":"Mozart White Chocolate cream liqueur","category":"Shots","strength":15,"portions":[{"amount":25,"price":3.07,"strength":15,"ppu":8.186666}]},{"name":"Baileys","category":"Liqueurs","strength":17,"portions":[{"amount":25,"price":4.82,"strength":17,"ppu":11.341177}]}];
        // drinksFetchError = null;
        // drinksData = postProcessDrinks(drinksData);
    });


    /* filters */
    let filterObj = $state<FilterObject>(getDefaultFilterObject());
</script>

<div id="center-wrapper" class="center-container">
    <br>
    <Title />
    <br>
    <h1>{venue ? venue.name : ""}</h1>
    <a href="/" on:click={() => { goto("/"); }}>Or search again...</a>
    <br>

    {#if drinksFetchError != null}
        <p>Error fetching drink data:<br>{drinksFetchError}</p>
    {:else if drinksData == null}
        <p>Fetching drink data...</p>
    {:else} <!-- here we do have drink data -->

        <div id="container">

            <div id="filter-container" class="center-container">
                <div class="filter">
                    <label>Max Price £{(Math.round(filterObj.maxPrice * 100) / 100).toFixed(2)}</label>
                    <input type="range" min="0.0" max="10" step="0.1" bind:value={filterObj.maxPrice}>
                </div>
                <div class="filter">
                    <label>Total results</label>
                    <Dropdown options={["5", "10", "15", "20", "All"]} defaultIndex={1} bind:bindValue={filterObj.totalResults} />
                </div>
            </div>

            <div id="result-container" class="center-container">
                {#each applyFilter(drinksData, filterObj) as drink, idx}
                    <div class="result box">
                        <span>#{idx + 1} - </span>
                        <span>{drink.name}</span>
                        <p>£{drink.portions[0].ppu.toFixed(2)}/u</p>
                        <p>{drink.strength}% ABV - {drink.portions[0].amount}ml</p>
                    </div>
                {/each}
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

        height: 30px;
    }

    #filter-container {
        width: 100%;
        justify-content: center;
    }

    .filter {
        min-width: 50%;
        max-width: 400px;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
    }

    #result-container {
        gap: 5px;
    }

    .result {
        width: 100%;
        max-width: 300px;
        height: 60px;

        background-color: #fff;

        color: black;
        font-weight: 600;
    }
</style>
