import { CLEAR, board } from "./frame.js";
import { fold } from "./pixels.js";

const TAU = 2 * Math.PI;

// PAINT

const hex = (c) => "#" + [c[0], c[1], c[2]].map((v) => v.toString(16).padStart(2, "0")).join("");
const alpha = (key, c) => (c[3] < 255 ? ` ${key}-opacity="${c[3] / 255}"` : "");
const fill = (c) => ` fill="${hex(c)}"${alpha("fill", c)}`;
const stroke = (c, thick) => ` fill="none" stroke="${hex(c)}"${alpha("stroke", c)} stroke-width="${thick}"`;
const points = (pts) => pts.map((p) => `${p[0]},${p[1]}`).join(" ");
const ROUND = ` stroke-linecap="round" stroke-linejoin="round"`;

function base64(bytes) {
  let text = "";
  for (let i = 0; i < bytes.length; i += 0x8000) text += String.fromCharCode(...bytes.subarray(i, i + 0x8000));
  return btoa(text);
}

async function href(pixels, png) {
  if (!png) throw new Error("svg: an image needs a png encoder, text({ png })");
  const out = await png(pixels);
  if (typeof out === "string") return out;
  return `data:image/png;base64,${base64(out instanceof Uint8Array ? out : new Uint8Array(out))}`;
}

// SVG

export function svg(width, height, ground = CLEAR) {
  const parts = [];
  if (ground[3] > 0) parts.push(`<rect width="${width}" height="${height}"${fill(ground)}/>`);

  function disc(cx, cy, r, c) {
    if (r > 0) parts.push(`<circle cx="${cx}" cy="${cy}" r="${r}"${fill(c)}/>`);
  }

  function polygon(pts, c) {
    if (pts.length >= 3) parts.push(`<polygon points="${points(pts)}"${fill(c)} fill-rule="evenodd"/>`);
  }

  function ring(cx, cy, r, thick, c) {
    if (r <= 0) return disc(cx, cy, r + thick / 2, c);
    parts.push(`<circle cx="${cx}" cy="${cy}" r="${r}"${stroke(c, thick)}/>`);
  }

  return {
    ...board(width, height),
    async text({ png } = {}) {
      const body = [];
      for (const part of parts) {
        if (typeof part === "string") body.push(part);
        else body.push(`<image x="${part.x}" y="${part.y}" width="${part.w}" height="${part.h}" preserveAspectRatio="none" image-rendering="pixelated" href="${await href(part.pixels, png)}"/>`);
      }
      return `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}">\n${body.join("\n")}\n</svg>\n`;
    },
    rect(x, y, w, h, c) {
      parts.push(`<rect x="${x}" y="${y}" width="${w}" height="${h}"${fill(c)}/>`);
    },
    round_rect(x, y, w, h, r, c) {
      const rr = Math.max(Math.min(r, w / 2, h / 2), 0);
      parts.push(`<rect x="${x}" y="${y}" width="${w}" height="${h}" rx="${rr}" ry="${rr}"${fill(c)}/>`);
    },
    disc,
    ring,
    segment(a, b, thick, c) {
      if (a[0] === b[0] && a[1] === b[1]) return disc(a[0], a[1], thick / 2, c);
      parts.push(`<line x1="${a[0]}" y1="${a[1]}" x2="${b[0]}" y2="${b[1]}"${stroke(c, thick)}${ROUND}/>`);
    },
    polyline(pts, thick, c) {
      if (pts.length >= 2) parts.push(`<polyline points="${points(pts)}"${stroke(c, thick)}${ROUND}/>`);
    },
    triangle(a, b, c, color) {
      polygon([a, b, c], color);
    },
    polygon,
    arc(center, r, angles, thick, c) {
      const [cx, cy] = center;
      const lo = Math.min(angles[0], angles[1]);
      const hi = Math.max(angles[0], angles[1]);
      const span = hi - lo;
      const at = (t) => [cx + r * Math.cos(t), cy + r * Math.sin(t)];
      if (span >= TAU) return ring(cx, cy, r, thick, c);
      if (span === 0) return disc(...at(lo), thick / 2, c);
      const arm = `A${r},${r} 0 0 1`;
      parts.push(`<path d="M${at(lo)} ${arm} ${at(lo + span / 2)} ${arm} ${at(hi)}"${stroke(c, thick)}${ROUND}/>`);
    },
    image(x, y, w, h, pixels) {
      parts.push({ x, y, w, h, pixels: { shape: [...pixels.shape], colors: new Uint8Array(fold(pixels).buffer) } });
    },
  };
}
