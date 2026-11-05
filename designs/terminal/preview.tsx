import type { Props } from "@mrlyprod/ui/shell/design.ts";
import type { CSSProperties } from "react";
import { DARK, LIGHT } from "./skin.ts";

const SKINS = { light: LIGHT, dark: DARK };

const tone = (theme: Props["preview"]["theme"]) =>
  Object.fromEntries(Object.entries(SKINS[theme]).filter(([name]) => name !== "--accent")) as CSSProperties;

const SCREEN = [
  " tty1                  21:04",
  "+-[ 1 ]--------------------+",
  "| $ ls                     |",
  "| about  apps  menu  notes |",
  "| $ _                      |",
  "+--------------------------+",
  " F1 Menu  F2 Apps  F3 Find",
].join("\n");

export function Preview({ theme }: Props["preview"]) {
  return (
    <span className="terminal still" data-tone={theme} style={tone(theme)} aria-hidden="true">
      <span className="still-screen">{SCREEN}</span>
    </span>
  );
}
