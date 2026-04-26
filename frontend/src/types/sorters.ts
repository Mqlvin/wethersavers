export enum SortMethod {
  Strength = "Strength",
  PricePerUnit = "Price Per Unit",
}

export const Comparators: Record<SortMethod, (a, b) => number> = {
  [SortMethod.Strength]: (a, b) => b.strength - a.strength,
  [SortMethod.PricePerUnit]: (a, b) => a.portions[0].ppu - b.portions[0].ppu
};
