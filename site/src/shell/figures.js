export let drawn = null;

let art = null;

export const figures = () =>
  (art ??= Promise.all([import('../lib/figure.js'), import('site:figures')]).then(([one, table]) => {
    one.start(table);
    drawn = one;
    return one;
  }));
