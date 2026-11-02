import { digest, stream } from "../../sdk/system.ts";

/* TYPES */

export type Image = { size: number; rgb: number[] };

export const SIZE = 16;

/* PIXELS */

export function image(seed: number): Image {
  const { draw } = stream(seed);
  const tone = () => [draw() & 255, draw() & 255, draw() & 255];
  const [top, bottom, spot] = [tone(), tone(), tone()];
  const cx = draw() % SIZE;
  const cy = draw() % SIZE;
  const r = 2 + (draw() % 5);
  const rgb: number[] = [];
  for (let y = 0; y < SIZE; y++)
    for (let x = 0; x < SIZE; x++) {
      const inside = (x - cx) ** 2 + (y - cy) ** 2 <= r * r;
      for (let c = 0; c < 3; c++)
        rgb.push(inside ? spot[c]! : Math.floor((top[c]! * (SIZE - 1 - y) + bottom[c]! * y) / (SIZE - 1)));
    }
  return { size: SIZE, rgb };
}

export const hashOf = (one: Image) => digest(one.rgb);
