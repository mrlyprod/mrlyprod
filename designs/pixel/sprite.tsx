export const SPRITES = {
  close: "##....## ###..### .######. ..####.. ..####.. .######. ###..### ##....##",
  zoom: "###..### ##....## #......# ........ ........ #......# ##....## ###..###",
  unzoom: "..#..#.. ..#..#.. ###..### ........ ........ ###..### ..#..#.. ..#..#..",
  tools: ".##..... ######## .##..... ........ .....##. ######## .....##. ........",
  find: "..####.. .#....#. .#....#. .#....#. ..####.. .....#.. ......#. .......#",
  launch: "##.##.## ##.##.## ........ ##.##.## ##.##.## ........ ##.##.## ##.##.##",
  cart: "##...... .######. .#....#. .######. .#...... .####### ..#...#. ........",
  more: "........ ........ ........ ##.##.## ##.##.## ........ ........ ........",
  add: "...##... ...##... ...##... ######## ######## ...##... ...##... ...##...",
  check: "........ .......# ......## #....##. ##..##.. .####... ..##.... ........",
  right: ".#...... .##..... .###.... .####... .###.... .##..... .#...... ........",
  left: "......#. .....##. ....###. ...####. ....###. .....##. ......#. ........",
  down: "........ ........ #######. .#####.. ..###... ...#.... ........ ........",
  pellet: "........ ..####.. .######. .######. .######. .######. ..####.. ........",
  dot: "........ ........ ........ ...##... ...##... ........ ........ ........",
  pac: "..####.. .######. #####... ####.... ####.... #####... .######. ..####..",
  shut: "..####.. .######. ######## ######## ######## ######## .######. ..####..",
  ghost: "..####.. .######. #ww##ww# #wb##wb# ######## ######## ######## ##.##.##",
  lock: "..####.. .#....#. .#....#. ######## ###..### ###..### ######## ........",
  sound: "...#.... ..##..#. ####...# ####.#.# ####.#.# ####...# ..##..#. ...#....",
  minus: "........ ........ ........ ######## ######## ........ ........ ........",
} as const;

export type Name = keyof typeof SPRITES;

const INKS: Record<string, string> = { "#": "currentColor", w: "#ffffff", b: "#2121de" };

export function paths(art: string): [string, string][] {
  const rows = art.split(" ");
  const out = new Map<string, string>();
  rows.forEach((row, y) => {
    let x = 0;
    while (x < row.length) {
      const c = row[x]!;
      if (!INKS[c]) {
        x++;
        continue;
      }
      let w = 1;
      while (row[x + w] === c) w++;
      out.set(c, `${out.get(c) ?? ""}M${x} ${y}h${w}v1h-${w}z`);
      x += w;
    }
  });
  return [...out].map(([c, d]) => [INKS[c]!, d]);
}

export function Sprite({ art, size = 16, label }: { art: Name; size?: number; label?: string }) {
  const rows = SPRITES[art].split(" ");
  return (
    <svg
      className="sprite"
      data-art={art}
      width={size}
      height={size}
      viewBox={`0 0 ${rows[0]!.length} ${rows.length}`}
      shapeRendering="crispEdges"
      role={label ? "img" : undefined}
      aria-label={label}
      aria-hidden={label ? undefined : true}
    >
      {paths(SPRITES[art]).map(([fill, d]) => (
        <path key={fill} fill={fill} d={d} />
      ))}
    </svg>
  );
}

const SVG = "http://www.w3.org/2000/svg";

export function sprite(art: Name, size = 12) {
  const rows = SPRITES[art].split(" ");
  const svg = document.createElementNS(SVG, "svg");
  svg.setAttribute("class", "sprite");
  svg.setAttribute("width", String(size));
  svg.setAttribute("height", String(size));
  svg.setAttribute("viewBox", `0 0 ${rows[0]!.length} ${rows.length}`);
  svg.setAttribute("shape-rendering", "crispEdges");
  svg.setAttribute("aria-hidden", "true");
  for (const [fill, d] of paths(SPRITES[art])) {
    const path = document.createElementNS(SVG, "path");
    path.setAttribute("fill", fill);
    path.setAttribute("d", d);
    svg.append(path);
  }
  return svg;
}

export function Mosaic() {
  return (
    <svg className="pixel defs" width="0" height="0" aria-hidden="true">
      <filter id="px-mosaic" x="0" y="0" width="1" height="1" colorInterpolationFilters="sRGB">
        <feFlood x="2" y="2" width="1" height="1" />
        <feComposite width="6" height="6" />
        <feTile result="grid" />
        <feComposite in="SourceGraphic" in2="grid" operator="in" />
        <feMorphology operator="dilate" radius="3" />
      </filter>
    </svg>
  );
}

export const ARROW =
  "#.......... ##......... #w#........ #ww#....... #www#...... #wwww#..... #wwwww#.... #wwwwww#... #wwwwwww#.. #wwwwwwww#. #wwwww##### #ww#ww#.... #w#.#ww#... ##..#ww#... #....#ww#.. .....###...";

export function cursor(art: string, ink: string, unit: number, hot: [number, number], or: string) {
  const rows = art.split(" ");
  const body = paths(art)
    .map(([fill, d]) => `<path fill="${fill === "currentColor" ? ink : fill}" d="${d}"/>`)
    .join("");
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${rows[0]!.length * unit}" height="${rows.length * unit}" viewBox="0 0 ${rows[0]!.length} ${rows.length}" shape-rendering="crispEdges">${body}</svg>`;
  return `url("data:image/svg+xml,${encodeURIComponent(svg)}") ${hot[0]} ${hot[1]}, ${or}`;
}
