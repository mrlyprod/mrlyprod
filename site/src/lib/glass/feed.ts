/* FEED */

export type Reach = () => DOMRect;

export type Ear = (canvas: HTMLCanvasElement) => void;

const sources = new Map<HTMLCanvasElement, Reach | null>();
const ears = new Set<Ear>();

const tell = (canvas: HTMLCanvasElement) => {
  for (const ear of ears) ear(canvas);
};

export function source(canvas: HTMLCanvasElement, reach: Reach | null = null): () => void {
  sources.set(canvas, reach);
  tell(canvas);
  return () => {
    if (sources.delete(canvas)) tell(canvas);
  };
}

export function fed(canvas: HTMLCanvasElement): void {
  if (sources.has(canvas)) tell(canvas);
}

export function tap(ear: Ear): () => void {
  ears.add(ear);
  return () => ears.delete(ear);
}

export const sourced = () => [...sources.keys()];

export const reached = (canvas: HTMLCanvasElement) => sources.get(canvas)?.() ?? null;
