import { palette } from "./palette.gen.js";

const NEUTRAL = ["black", "white", "gray"];

export const HUES = Object.keys(palette).filter((name) => !NEUTRAL.includes(name));

export const tints = (blank) => [["", blank], ...HUES.map((hue) => [hue, hue[0].toUpperCase() + hue.slice(1)])];
