export const fixed = (x, places = 4) => (Number.isFinite(x) ? x.toFixed(places) : 'none');

