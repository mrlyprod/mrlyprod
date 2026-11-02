import { palette } from "./palette.gen.js";

const NEUTRAL = ["black", "white", "gray"];

export const HUES = Object.keys(palette).filter((name) => !NEUTRAL.includes(name));

export const tints = (blank: string): [string, string][] => [
  ["", blank],
  ...HUES.map((hue): [string, string] => [hue, hue[0].toUpperCase() + hue.slice(1)]),
];
