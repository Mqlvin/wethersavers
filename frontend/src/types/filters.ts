import { SortMethod } from "./sorters";

export interface FilterObject {
    maxPrice: number;
    sortBy: SortMethod;
    excludeCategories: string[];
}

export function getDefaultFilterObject(): FilterObject {
    return {
        maxPrice: 10.00,
        sortBy: SortMethod.PricePerUnit,
        excludeCategories: []
    };
}

export function applyFilter(drinkObjs: any, filters: FilterObject) {
    let drinkObjBuilder = [];
    for(let drinkObj of drinkObjs) {
        if(filters.excludeCategories.includes(drinkObj.category)) continue;

        let portion = drinkObj.portions[0];
        if(portion.price > filters.maxPrice) continue;

        drinkObjBuilder.push(drinkObj);
    }

    return drinkObjBuilder;
}
