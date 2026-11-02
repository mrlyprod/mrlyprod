import { letters } from "../../apps/shared/font/font.ts";

/* WORD */

export const order = (n: number, count: number) => (n * 37 + 11) % count;

export function Word({ text, className }: { text: string; className?: string }) {
  const { rows, cols, grid } = letters(text.toUpperCase());
  const lit = grid.flatMap((row, y) => row.flatMap((on, x) => (on ? [[x, y]] : [])));
  return (
    <svg className={className ?? "word"} viewBox={`0 0 ${cols} ${rows}`} aria-hidden="true">
      {lit.map(([x, y], n) => (
        <rect
          key={`${x},${y}`}
          x={x}
          y={y}
          width="1"
          height="1"
          style={{ "--k": order(n, lit.length) / lit.length } as object}
        />
      ))}
    </svg>
  );
}
