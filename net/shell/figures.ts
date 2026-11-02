/* TYPES */

export type Drawn = typeof import("../../draw/figure.ts");

export let drawn: Drawn | null = null;

let art: Promise<Drawn> | null = null;

export const figures = () =>
  (art ??= Promise.all([import("../../draw/figure.ts"), import("site:figures")]).then(([one, table]) => {
    one.start(table);
    drawn = one;
    return one;
  }));
