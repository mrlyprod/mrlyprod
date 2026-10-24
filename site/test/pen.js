export const pen = (log) =>
  new Proxy(
    {},
    {
      get: (_, key) =>
        key === "measureText" ? (text) => ({ width: String(text).length * 7 }) : (...args) => log.push([key, ...args]),
      set: (_, key, value) => {
        log.push([key, value]);
        return true;
      },
    },
  );

export const said = (log) => log.filter(([key]) => key === "fillText").map(([, text]) => text);
