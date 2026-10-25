export const fixed = (x: number, places = 4) => (Number.isFinite(x) ? x.toFixed(places) : "none");
