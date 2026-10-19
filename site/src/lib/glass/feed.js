/* FEED */

const sources = new Map();
const ears = new Set();

const tell = (canvas) => {
  for (const ear of ears) ear(canvas);
};

export function source(canvas, reach = null) {
  sources.set(canvas, reach);
  tell(canvas);
  return () => {
    if (sources.delete(canvas)) tell(canvas);
  };
}

export function fed(canvas) {
  if (sources.has(canvas)) tell(canvas);
}

export function tap(ear) {
  ears.add(ear);
  return () => ears.delete(ear);
}

export const sourced = () => [...sources.keys()];

export const reached = (canvas) => sources.get(canvas)?.() ?? null;
