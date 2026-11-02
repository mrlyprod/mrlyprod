/* GLYPH */

export const GLYPHS = {
  launch: ["..#..", "..#..", "#####", "..#..", "..#.."],
  cart: ["#####", "#ooo#", "#ooo#", "#ooo#", "#####"],
  find: ["####.", "#..#.", "#..#.", "####.", "....#"],
  apps: ["##.##", "##.##", ".....", "##.##", "##.##"],
  plus: ["..#..", "..#..", "#####", "..#..", "..#.."],
  minus: [".....", ".....", "#####", ".....", "....."],
};

const cells = (rows: string[], mark: string, dot: boolean) =>
  rows
    .flatMap((row, y) => [...row].flatMap((c, x) => (c === mark ? [[x, y]] : [])))
    .map(([x, y]) => <rect key={`${x},${y}`} className={dot ? "dot" : undefined} x={x} y={y} width="1" height="1" />);

export function Glyph({ art }: { art: keyof typeof GLYPHS }) {
  const rows = GLYPHS[art];
  return (
    <svg
      className="glyphs"
      viewBox={`0 0 ${rows[0]!.length} ${rows.length}`}
      style={{ "--rows": rows.length, "--cols": rows[0]!.length } as object}
      aria-hidden="true"
    >
      {cells(rows, "#", false)}
      {cells(rows, "o", true)}
    </svg>
  );
}
