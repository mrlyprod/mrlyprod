import type { Tokens } from "@mrlyprod/ui/shell/design.ts";

export const SKIN: { light: Tokens; dark: Tokens } = {
  light: {
    "--paper": "#ffffff",
    "--ink": "#000000",
    "--line": "#000000",
    "--hairline": "#00000033",
    "--fill": "#0000000f",
    "--fill-strong": "#0000001f",
    "--muted": "#6b6b70",
    "--scrim": "#00000052",
    "--accent": "#008cff",
  },
  dark: {
    "--paper": "#000000",
    "--ink": "#ffffff",
    "--line": "#ffffff",
    "--hairline": "#ffffff33",
    "--fill": "#ffffff0f",
    "--fill-strong": "#ffffff1f",
    "--muted": "#a1a1a6",
    "--scrim": "#0000008c",
    "--accent": "#008cff",
  },
};

export const BAR = { light: "#ffffff", dark: "#000000" };
