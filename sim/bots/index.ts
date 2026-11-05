import body from "./body.ts";
import canon from "./canon.ts";
import { ENGINE } from "./engine.ts";
import maps from "./maps.ts";
import mind from "./mind.ts";
import stocks from "./stocks.ts";
import type { Bot } from "./types.ts";
import weather from "./weather.ts";

const none: Bot = () => null;

export const BOTS = {
  ...ENGINE,
  dawn: none,
  body,
  canon,
  maps,
  mind,
  weather,
  stocks,
} as unknown as Record<string, Bot>;
