export enum SortMethod {
    PricePerUnitAscending = "Price Per Unit (asc)",
    PricePerUnitDescending = "Price Per Unit (desc)",
    Strength = "Strength",
    PureEthanol = "Pure Ethanol",
}

export const Comparators: Record<SortMethod, (a, b) => number> = {
    [SortMethod.Strength]: (a, b) => b.strength - a.strength,
    [SortMethod.PricePerUnitAscending]: (a, b) => a.portions[0].ppu - b.portions[0].ppu,
    [SortMethod.PricePerUnitDescending]: (a, b) => b.portions[0].ppu - a.portions[0].ppu,
    [SortMethod.PureEthanol]: (a, b) => b.portions[0].amount * b.portions[0].strength - a.portions[0].amount * a.portions[0].strength,
};
