const SEEDS = 4294967296;

export const roll = () => Math.floor(Math.random() * SEEDS);

export const fixed = (x, places = 4) => (Number.isFinite(x) ? x.toFixed(places) : 'none');

