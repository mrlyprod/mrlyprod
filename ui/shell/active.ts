import { glyphs } from "../kit/icon.tsx";
import type { Design, Parts } from "./design.ts";

export let design: Design;

export let parts: Parts;

export function apply(next: Design) {
  design = next;
  parts = next.parts;
  glyphs(next);
}
