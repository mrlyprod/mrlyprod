import { media } from "../../sdk/media.ts";

/* TITLES */

export const TITLES = [
  "How Bread Rises",
  "One Hex at a Time",
  "The Price of Salt",
  "Talking Weather",
  "Letters from Test 3",
  "Founding a Bakery",
  "What a Seed Holds",
  "Ten Minutes on Maps",
  "The Library Hour",
  "Who Writes the Wiki",
  "Rain Days",
  "The Last Chapter",
] as const;

/* APP */

export default media({
  id: "podcasts",
  title: "Podcasts",
  line: "Episodes published as metadata: a library, plays and a queue each.",
  unit: "episode",
  titles: TITLES,
});
