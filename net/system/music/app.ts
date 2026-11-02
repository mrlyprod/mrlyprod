import { media } from "../../sdk/media.ts";

/* TITLES */

export const TITLES = [
  "Morning Light",
  "River Song",
  "Slow Train",
  "Blue Hour",
  "Kitchen Radio",
  "Paper Boats",
  "Long Walk Home",
  "Second Street",
  "Quiet Engine",
  "Open Window",
  "Winter Garden",
  "Late Bus",
] as const;

/* APP */

export default media({
  id: "music",
  title: "Music",
  line: "Tracks published as metadata: a library, plays and a queue each.",
  unit: "track",
  titles: TITLES,
});
