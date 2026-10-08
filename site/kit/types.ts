const TYPES = new Map(
  Object.entries({
    html: "text/html; charset=utf-8",
    js: "text/javascript; charset=utf-8",
    mjs: "text/javascript; charset=utf-8",
    css: "text/css; charset=utf-8",
    json: "application/json",
    map: "application/json",
    webmanifest: "application/manifest+json",
    xml: "application/xml",
    txt: "text/plain; charset=utf-8",
    md: "text/markdown; charset=utf-8",
    tex: "text/plain; charset=utf-8",
    wasm: "application/wasm",
    svg: "image/svg+xml",
    png: "image/png",
    jpg: "image/jpeg",
    jpeg: "image/jpeg",
    gif: "image/gif",
    webp: "image/webp",
    ico: "image/x-icon",
    pdf: "application/pdf",
    woff2: "font/woff2",
    mp4: "video/mp4",
  }),
);

export const kind = (path: string) => TYPES.get(path.match(/\.([^./]+)$/)?.[1]?.toLowerCase() ?? "") ?? "application/octet-stream";
