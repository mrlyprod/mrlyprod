import { palette } from "@mrlyprod/draw/palette.gen.js";

const NEUTRAL = ["black", "white", "gray"];

export const HUES = Object.keys(palette).filter((name) => !NEUTRAL.includes(name));
