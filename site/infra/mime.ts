const TEXT = "text/plain; charset=utf-8";

const OCTET = "application/octet-stream";

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
    txt: TEXT,
    md: "text/markdown; charset=utf-8",
    tex: TEXT,
    wasm: "application/wasm",
    svg: "image/svg+xml",
    avif: "image/avif",
    png: "image/png",
    jpg: "image/jpeg",
    jpeg: "image/jpeg",
    gif: "image/gif",
    webp: "image/webp",
    ico: "image/x-icon",
    pdf: "application/pdf",
    otf: "font/otf",
    ttf: "font/ttf",
    woff: "font/woff",
    woff2: "font/woff2",
    mp3: "audio/mpeg",
    wav: "audio/wav",
    mp4: "video/mp4",
    webm: "video/webm",
    gz: "application/gzip",
    zip: "application/zip",
  }),
);

const PASSIVE = /^(image|audio|video|font)\/|^application\/(pdf|wasm|gzip|zip)$/;

export const kind = (path: string) => TYPES.get(path.match(/\.([^./]+)$/)?.[1]?.toLowerCase() ?? "") ?? OCTET;

export const rawKind = (path: string, text: boolean) => {
  const type = kind(path);
  return text || type.startsWith("image/svg") ? TEXT : PASSIVE.test(type) ? type : OCTET;
};
