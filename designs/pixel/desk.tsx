import type { Props } from "@mrlyprod/ui/shell/design.ts";
import { carve, rng } from "./attract.ts";
import { paths, SPRITES } from "./sprite.tsx";

const COLS = 41;

const ROWS = 27;

const SEED = 1980;

const rect = (x: number, y: number, w: number, h: number) =>
  `M${+x.toFixed(3)} ${+y.toFixed(3)}h${+w.toFixed(3)}v${+h.toFixed(3)}h${-w.toFixed(3)}z`;

export function board(cols = COLS, rows = ROWS, seed = SEED) {
  const wall = carve(cols, rows, rng(seed));
  const solid = (x: number, y: number) => x >= 0 && y >= 0 && x < cols && y < rows && wall[y * cols + x] === 1;
  const tube = (x: number, y: number, half: number) => {
    const lo = 0.5 - half;
    let d = rect(x + lo, y + lo, half * 2, half * 2);
    if (solid(x + 1, y)) d += rect(x + 0.5, y + lo, 1, half * 2);
    if (solid(x, y + 1)) d += rect(x + lo, y + 0.5, half * 2, 1);
    return d;
  };
  let outer = "";
  let inner = "";
  let food = "";
  for (let y = 0; y < rows; y++)
    for (let x = 0; x < cols; x++) {
      if (solid(x, y)) {
        outer += tube(x, y, 0.24);
        inner += tube(x, y, 0.11);
      } else if ((x + y) % 2 === 0) food += rect(x + 0.42, y + 0.42, 0.16, 0.16);
    }
  return { outer, inner, food };
}

const BOARD = board();

const CAST: [keyof typeof SPRITES, number, number, string][] = [
  ["pac", 1, 1, "var(--px-a)"],
  ["ghost", COLS - 2, ROWS - 2, "var(--px-b)"],
  ["ghost", COLS - 2, 1, "var(--px-d)"],
  ["ghost", 1, ROWS - 2, "var(--px-c)"],
];

export function Desk({ canvas, home }: Props["desk"]) {
  return (
    <div className="pixel desk" data-home={home || undefined}>
      <canvas className="ground" {...canvas()} />
      <svg
        className="maze"
        viewBox={`0 0 ${COLS} ${ROWS}`}
        preserveAspectRatio="xMidYMid slice"
        shapeRendering="crispEdges"
        aria-hidden="true"
      >
        <path className="walls" d={BOARD.outer} />
        <path className="hollow" d={BOARD.inner} />
        <path className="food" d={BOARD.food} />
        {CAST.map(([art, x, y, ink]) => (
          <g key={`${x}:${y}`} transform={`translate(${x + 0.1} ${y + 0.1}) scale(0.1)`} color={ink}>
            {paths(SPRITES[art]).map(([fill, d]) => (
              <path key={fill} fill={fill} d={d} />
            ))}
          </g>
        ))}
      </svg>
    </div>
  );
}
