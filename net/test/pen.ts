export type Stroke = [string | symbol, ...unknown[]];

export const pen = (log: Stroke[]): CanvasRenderingContext2D =>
  new Proxy({} as CanvasRenderingContext2D, {
    get: (_, key) =>
      key === "measureText"
        ? (text: unknown) => ({ width: String(text).length * 7 })
        : (...args: unknown[]) => log.push([key, ...args]),
    set: (_, key, value: unknown) => {
      log.push([key, value]);
      return true;
    },
  });

export const said = (log: Stroke[]) => log.filter(([key]) => key === "fillText").map(([, text]) => text);
