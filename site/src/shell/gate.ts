export type Size = [number, number];

export const MIN: Size = [1280, 720];

export const fits = ([width, height]: Size, [least, low]: Size = MIN) => width >= least && height >= low;
