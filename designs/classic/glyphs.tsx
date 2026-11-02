/* PIXELS */

const ART = {
  close: ["#.....#", ".#...#.", "..#.#..", "...#...", "..#.#..", ".#...#.", "#.....#"],
  zoom: ["#######", "#######", "#.....#", "#.....#", "#.....#", "#.....#", "#######"],
  restore: ["..#####", "..#####", "..#...#", "#####.#", "#####.#", "#...###", "#####.."],
  tools: ["#######", "#...#.#", "#...#.#", "#...#.#", "#...#.#", "#...#.#", "#######"],
  up: [".......", "...#...", "..###..", ".#####.", "#######", ".......", "......."],
  down: [".......", ".......", "#######", ".#####.", "..###..", "...#...", "......."],
  right: [".#.....", ".##....", ".###...", ".####..", ".###...", ".##....", ".#....."],
  tick: ["......#", ".....##", "#...###", "##.###.", "#####..", ".###...", "..#...."],
  dot: [".......", "..###..", ".#####.", ".#####.", ".#####.", "..###..", "......."],
  plus: [".......", "...#...", "...#...", ".#####.", "...#...", "...#...", "......."],
  minus: [".......", ".......", ".......", ".#####.", ".......", ".......", "......."],
  more: [".......", ".......", ".......", "#..#..#", ".......", ".......", "......."],
  find: [".###...", "#...#..", "#...#..", "#...#..", ".###...", "....##.", ".....##"],
  cart: ["#......", ".######", ".#....#", ".#....#", ".######", "..#..#.", "......."],
  page: ["#####..", "#...##.", "#....##", "#.....#", "#.....#", "#.....#", "#######"],
  cmd: ["##...##", "#.#.#.#", ".#####.", "..#.#..", ".#####.", "#.#.#.#", "##...##"],
  shift: ["...#...", "..#.#..", ".#...#.", "###.###", "..#.#..", "..#.#..", "..###.."],
  grip: ["......#", ".......", "....#.#", ".......", "..#.#.#", ".......", "#.#.#.#"],
  warn: [
    "......##......",
    ".....####.....",
    ".....####.....",
    "....##..##....",
    "....##..##....",
    "...##.##.##...",
    "...##.##.##...",
    "..##..##..##..",
    "..##..##..##..",
    ".##........##.",
    ".##...##...##.",
    "##..........##",
    "##############",
    "##############",
  ],
} as const;

export type Art = keyof typeof ART;

const runs = (rows: readonly string[]) =>
  rows.flatMap((row, y) => [...row.matchAll(/#+/g)].map((hit) => ({ x: hit.index, y, w: hit[0].length })));

/* GLYPH */

export function Glyph({ art, size }: { art: Art; size?: number }) {
  const rows = ART[art];
  const side = rows[0]!.length;
  const px = size ?? side;
  return (
    <svg
      className="pixel"
      viewBox={`0 0 ${side} ${rows.length}`}
      width={px}
      height={px}
      shapeRendering="crispEdges"
      fill="currentColor"
      aria-hidden="true"
    >
      {runs(rows).map(({ x, y, w }) => (
        <rect key={`${x},${y}`} x={x} y={y} width={w} height="1" />
      ))}
    </svg>
  );
}

/* DOM */

const SVG = "http://www.w3.org/2000/svg";

export function pixel(art: Art) {
  const rows = ART[art];
  const side = rows[0]!.length;
  const svg = document.createElementNS(SVG, "svg");
  const set = { class: "pixel", viewBox: `0 0 ${side} ${rows.length}`, width: side, height: side };
  for (const [k, v] of Object.entries(set)) svg.setAttribute(k, String(v));
  svg.setAttribute("shape-rendering", "crispEdges");
  svg.setAttribute("fill", "currentColor");
  svg.setAttribute("aria-hidden", "true");
  for (const { x, y, w } of runs(rows)) {
    const rect = document.createElementNS(SVG, "rect");
    for (const [k, v] of Object.entries({ x, y, width: w, height: 1 })) rect.setAttribute(k, String(v));
    svg.append(rect);
  }
  return svg;
}
