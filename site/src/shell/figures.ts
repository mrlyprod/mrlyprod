/* TYPES */

export type Drawn = typeof import("../lib/figure.ts");

export let drawn: Drawn | null = null;

let art: Promise<Drawn> | null = null;

export const figures = () =>
  (art ??= Promise.all([import("../lib/figure.ts"), import("site:figures")]).then(([one, table]) => {
    one.start(table);
    drawn = one;
    return one;
  }));
