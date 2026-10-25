import { arts, defs } from "site:icons";
import { markup, SPRITE } from "../lib/appicon.ts";

/* TYPES */

export type AppIconProps = { id: string; className?: string };

const NS = "http://www.w3.org/2000/svg";

const made = new Map<string, string>();

let sprite: SVGSVGElement | null = null;

function mount() {
  if (sprite || typeof document === "undefined") return;
  sprite = document.createElementNS(NS, "svg");
  sprite.setAttribute("class", "appicons");
  sprite.setAttribute("aria-hidden", "true");
  sprite.innerHTML = `<defs>${SPRITE}${defs}</defs>`;
  document.body.prepend(sprite);
}

function html(id: string) {
  if (!made.has(id)) made.set(id, arts[id] ? markup(arts[id]) : "");
  return made.get(id)!;
}

export function AppIcon({ id, className = "face" }: AppIconProps) {
  mount();
  return <span className={className} dangerouslySetInnerHTML={{ __html: html(id) }} />;
}
