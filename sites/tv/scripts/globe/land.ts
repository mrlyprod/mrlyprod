import { mkdir } from "node:fs/promises";
import { dirname, join, relative } from "node:path";
import { grid, latLon } from "../../pages/home/globe/grid.ts";
import { LEVEL, pack } from "../../pages/home/globe/land.ts";

const HERE = dirname(new URL(import.meta.url).pathname);
const TV = join(HERE, "..", "..");
const REPO = join(TV, "..", "..");
const DATA_DIR = join(REPO, "data", relative(REPO, HERE));
const SOURCE = "https://raw.githubusercontent.com/nvkelso/natural-earth-vector/master/geojson/ne_110m_land.geojson";
const FILE = join(DATA_DIR, "ne_110m_land.geojson");
const OUT = join(TV, "pages", "home", "globe", "land.gen.json");

type Ring = number[][];

type Feature = { geometry: { type: string; coordinates: Ring[] | Ring[][] } };

async function geojson(): Promise<Ring[][]> {
  if (!(await Bun.file(FILE).exists())) {
    await mkdir(DATA_DIR, { recursive: true });
    const res = await fetch(SOURCE);
    if (!res.ok) throw new Error(`${SOURCE}: ${res.status}`);
    await Bun.write(FILE, await res.arrayBuffer());
  }
  const { features } = (await Bun.file(FILE).json()) as { features: Feature[] };
  return features.flatMap(({ geometry }) =>
    geometry.type === "Polygon" ? [geometry.coordinates as Ring[]] : (geometry.coordinates as Ring[][]),
  );
}

function inside(ring: Ring, lon: number, lat: number): boolean {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [xi, yi] = ring[i];
    const [xj, yj] = ring[j];
    if (yi > lat !== yj > lat && lon < ((xj - xi) * (lat - yi)) / (yj - yi) + xi) hit = !hit;
  }
  return hit;
}

const onLand = (polygons: Ring[][], lon: number, lat: number) =>
  polygons.some(([outer, ...holes]) => inside(outer, lon, lat) && !holes.some((h) => inside(h, lon, lat)));

async function main() {
  const polygons = await geojson();
  const { centers, corners } = grid(LEVEL);
  const bits = centers.map((c, i): number => {
    const samples = [c, ...corners[i]].map(latLon);
    const hits = samples.filter(([lat, lon]) => onLand(polygons, lon, lat)).length;
    return hits * 2 >= samples.length ? 1 : 0;
  });
  const count = bits.reduce((a, b) => a + b, 0);
  const out = { level: LEVEL, cells: bits.length, land: count, bits: pack(bits) };
  await Bun.write(OUT, `${JSON.stringify(out)}\n`);
  console.log(`${count} land of ${bits.length} cells (${((100 * count) / bits.length).toFixed(1)}%) -> ${OUT}`);
}

await main();
