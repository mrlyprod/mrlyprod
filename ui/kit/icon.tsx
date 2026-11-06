import type { Color } from "@mrlyprod/draw";
import { svg } from "@mrlyprod/draw";
import type { EmojiName, Glyphs, IconName } from "../shell/design.ts";
import { EMOJIS } from "../shell/design.ts";

export type { EmojiName, IconName } from "../shell/design.ts";

type Sets = { icons: Glyphs<IconName>; emojis: Glyphs<EmojiName> };

export type IconProps = { name: IconName; size?: number };

const INK: Color = [1, 2, 3, 255];

const SIZE = 20;

let sets: Sets;

let held = new Map<string, string>();

export function glyphs(next: Sets) {
  sets = next;
  held = new Map();
}

function drawn(kind: keyof Sets, name: string, size: number) {
  const set: Glyphs<string> = sets[kind];
  const key = `${kind} ${name}`;
  if (!held.has(key)) {
    const pen = svg(set.box, set.box);
    set.draw[name]!(pen, INK);
    const body = pen.body().replace(/\d+\.\d{3,}/g, (n) => String(Math.round(+n * 100) / 100));
    held.set(key, body.replaceAll("#010203", "currentColor"));
  }
  return {
    viewBox: `0 0 ${set.box} ${set.box}`,
    side: set.crisp ? Math.max(1, Math.floor(size / set.box)) * set.box : size,
    crisp: set.crisp ? "crispEdges" : undefined,
    body: held.get(key)!,
  };
}

function props(kind: keyof Sets, name: string, size: number) {
  const { viewBox, side, crisp, body } = drawn(kind, name, size);
  return { viewBox, width: side, height: side, shapeRendering: crisp, dangerouslySetInnerHTML: { __html: body } };
}

export function Icon({ name, size = SIZE }: IconProps) {
  return <svg {...props("icons", name, size)} aria-hidden="true" />;
}

export function Emoji({ name, size = SIZE }: { name: EmojiName; size?: number }) {
  return <svg {...props("emojis", name, size)} role="img" aria-label={EMOJIS[name]} />;
}

export function mark(name: IconName, size = SIZE) {
  const { viewBox, side, crisp, body } = drawn("icons", name, size);
  const edges = crisp ? ` shape-rendering="${crisp}"` : "";
  return `<svg viewBox="${viewBox}" width="${side}" height="${side}"${edges} aria-hidden="true">${body}</svg>`;
}

export function glyph(name: IconName, size = SIZE) {
  const box = document.createElement("template");
  box.innerHTML = mark(name, size);
  return box.content.firstChild as SVGSVGElement;
}
