export type Drawn = typeof import("@mrlyprod/draw/figure.ts");

export let drawn: Drawn | null = null;

let art: Promise<Drawn> | null = null;

export const figures = () =>
  (art ??= Promise.all([import("@mrlyprod/draw/figure.ts"), import("site:figures")]).then(([one, table]) => {
    one.start(table);
    drawn = one;
    return one;
  }));
