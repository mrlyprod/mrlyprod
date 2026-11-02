import { media } from "../../sdk/media.ts";

/* TITLES */

export const TITLES = [
  "The Evening Report",
  "Garden Hour",
  "Kitchen Table",
  "Night Shift",
  "The Repair Shop",
  "Harbour Lights",
  "Weather Watch",
  "Small Town Court",
  "The Long Game",
  "Open House",
  "Market Day",
  "Last Train Out",
] as const;

/* APP */

export default media({
  id: "tv",
  title: "TV",
  line: "Shows published as metadata: a library, plays and a queue each.",
  unit: "show",
  titles: TITLES,
});
