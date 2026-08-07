// PALETTE

export type ColorName =
  | "black"
  | "white"
  | "red"
  | "orange"
  | "yellow"
  | "green"
  | "mint"
  | "teal"
  | "cyan"
  | "blue"
  | "indigo"
  | "purple"
  | "pink"
  | "brown"
  | "gray"

export const POOL: ColorName[] = [
  "red",
  "orange",
  "yellow",
  "green",
  "mint",
  "teal",
  "cyan",
  "blue",
  "indigo",
  "purple",
  "pink",
  "brown",
  "gray",
]

export const HEX: Record<ColorName, string> = {
  black: "#000000",
  white: "#ffffff",
  red: "#ff3d40",
  orange: "#ff8f2c",
  yellow: "#ffd100",
  green: "#32cc58",
  mint: "#00d1bb",
  teal: "#00cad8",
  cyan: "#1ec9f3",
  blue: "#008cff",
  indigo: "#6768fa",
  purple: "#d332e9",
  pink: "#ff325a",
  brown: "#b18462",
  gray: "#8e8e93",
}

// PICK

export function css(name: ColorName): string {
  return `var(--c-${name})`
}

export function randomColor(): ColorName {
  return POOL[Math.floor(Math.random() * POOL.length)] ?? "gray"
}
