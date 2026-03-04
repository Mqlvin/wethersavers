export interface FilterObject {
    maxPrice: number;
    totalResults: number;
    excludeCategories: string[];
}

export function getDefaultFilterObject(): FilterObject {
    return {
        maxPrice: 4.00,
        totalResults: 10,
        excludeCategories: []
    };
}

export function applyFilter(drinkObjs: any, filters: FilterObject) {
    let drinkObjBuilder = [];
    for(let drinkObj of drinkObjs) {
        if(drinkObjBuilder.length >= filters.totalResults) break;

        if(filters.excludeCategories.includes(drinkObj.category)) continue;

        let portion = drinkObj.portions[0];
        if(portion.price > filters.maxPrice) continue;

        drinkObjBuilder.push(drinkObj);
    }

    console.log(drinkObjBuilder)
    return drinkObjBuilder;
}
