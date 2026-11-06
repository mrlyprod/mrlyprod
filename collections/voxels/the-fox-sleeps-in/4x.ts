import { Volume } from "../sculpt.ts";

const v = new Volume(64, {
  rust: "#9c3d1c",
  brush: "#c1561f",
  coat: "#d7662a",
  fur: "#e87a31",
  gold: "#f0a04f",
  cream: "#f6ead6",
  ivory: "#d8c8ae",
  dark: "#45302a",
  sock: "#6b4433",
  moss: "#6c963f",
  shade: "#4a7034",
  deep: "#39582d",
  lichen: "#8fb04d",
  leaf: "#e8c547",
  amber: "#cf8a2c",
});

const top = 11;
const up = (x: number, u: number, z: number, ink: string | null) => v.set(x, top + u, z, ink);
const at = (x: number, u: number, z: number) => v.get(x, top + u, z);
const peak = (x: number, z: number) => {
  for (let u = 30; u >= 1; u--) if (at(x, u, z) !== null) return u;
  return 0;
};

type Step = { x: number; z: number; rh: number; rv: number; s: number; nx: number; nz: number };
const sweep = (knots: number[][]): Step[] => {
  const pick = (i: number) => knots[Math.max(0, Math.min(knots.length - 1, i))]!;
  const blend = (i: number, t: number, k: number) => {
    const [a, b, c, d] = [pick(i - 1)[k]!, pick(i)[k]!, pick(i + 1)[k]!, pick(i + 2)[k]!];
    return 0.5 * (2 * b + (c - a) * t + (2 * a - 5 * b + 4 * c - d) * t * t + (3 * b - a - 3 * c + d) * t ** 3);
  };
  const steps: Step[] = [];
  let s = 0;
  for (let i = 0; i + 1 < knots.length; i++)
    for (let t = 0; t < 1; t += 0.025) {
      const [x, z, rh, rv] = [0, 1, 2, 3].map((k) => blend(i, t, k)) as [number, number, number, number];
      const [x2, z2] = [0, 1].map((k) => blend(i, t + 0.01, k)) as [number, number];
      const len = Math.hypot(x2 - x, z2 - z) || 1;
      const last = steps[steps.length - 1];
      if (last) s += Math.hypot(x - last.x, z - last.z);
      steps.push({ x, z, rh, rv, s, nx: -(z2 - z) / len, nz: (x2 - x) / len });
    }
  return steps;
};

type Paint = (s: number, down: number, lift: number, u: number) => string;
const tube = (steps: Step[], sink: number, relief: (s: number, down: number) => number, paint: Paint) => {
  const reach = Math.max(...steps.map((p) => p.rh)) + 2;
  const span = (axis: "x" | "z") => {
    const along = steps.map((p) => p[axis]);
    return [Math.max(0, Math.floor(Math.min(...along) - reach)), Math.min(63, Math.ceil(Math.max(...along) + reach))];
  };
  const [[x0, x1], [z0, z1]] = [span("x"), span("z")] as [[number, number], [number, number]];
  for (let u = 1; u <= 28; u++)
    for (let z = z0; z <= z1; z++)
      for (let x = x0; x <= x1; x++) {
        let best: Step | undefined;
        let rho = 9;
        let side = 0;
        for (const p of steps) {
          const [dx, dz] = [x - p.x, z - p.z];
          const r = Math.hypot(Math.hypot(dx, dz) / p.rh, (u - p.rv + sink) / p.rv);
          if (r < rho) {
            rho = r;
            best = p;
            side = dx * p.nx + dz * p.nz;
          }
        }
        if (!best || rho > 1.4) continue;
        const th = Math.atan2((u - best.rv + sink) / best.rv, side / best.rh);
        const down = Math.abs(th - Math.PI / 2);
        const lift = relief(best.s, down);
        if (rho <= 1 + lift / best.rh) up(x, u, z, paint(best.s, down, lift, u));
      }
};

const spine = sweep([
  [19.0, 43.0, 9.2, 7.2],
  [16.8, 32.4, 10.4, 7.8],
  [16.8, 22.4, 10.0, 7.6],
  [22.4, 15.6, 9.4, 7.2],
  [31.0, 13.6, 8.8, 7.0],
  [39.6, 15.2, 8.2, 6.8],
  [44.8, 21.2, 7.4, 6.2],
  [46.0, 27.2, 6.2, 5.6],
  [44.8, 30.8, 5.6, 5.2],
]);
const furl = (s: number, down: number) =>
  1.2 * Math.sin(6 * (down + 0.045 * s) + 0.7) * (0.55 + 0.45 * Math.sin(0.3 * s + 1.1 * down + 2)) +
  0.4 * Math.sin(0.15 * s + 1.6 * down + 1) +
  0.6 * Math.sin(15 * (down + 0.05 * s) + 0.5 * Math.sin(0.5 * s + 3 * down)) * (0.5 + 0.5 * Math.sin(0.7 * s + down));
const coatOf: Paint = (s, down, lift) => {
  if (down > 2.15) return "rust";
  if (down < 0.5 + 0.12 * Math.sin(0.25 * s + 1)) return lift < -0.6 ? "rust" : "brush";
  if (lift > 0.75 && down > 0.7) return "fur";
  return lift < -0.7 ? "brush" : "coat";
};
tube(spine, 0.6, furl, coatOf);
const belly = sweep([
  [24.0, 38.0, 8.4, 6.0],
  [23.6, 28.0, 8.8, 6.4],
  [28.0, 21.6, 8.4, 6.0],
  [36.0, 21.2, 7.6, 5.6],
  [40.0, 26.0, 6.8, 5.2],
  [43.6, 32.0, 6.4, 5.0],
  [45.2, 38.0, 5.8, 4.4],
  [47.6, 42.0, 4.4, 3.4],
]);
const underOf: Paint = (_s, down, lift) => (down > 1.9 ? "rust" : lift > 0.7 ? "coat" : "brush");
tube(belly, 0.6, furl, underOf);
const legs = sweep([
  [22.0, 38.0, 7.0, 5.0],
  [29.0, 43.0, 6.5, 4.6],
  [36.0, 45.0, 5.5, 4.2],
  [42.0, 44.0, 4.5, 3.6],
]);
tube(legs, 0.6, furl, underOf);
const chest = sweep([
  [28.0, 26.0, 6.0, 4.4],
  [32.0, 31.0, 6.0, 4.4],
  [36.0, 37.0, 5.6, 4.2],
  [39.0, 42.0, 5.0, 3.8],
]);
tube(chest, 0.6, furl, underOf);
v.ball(49.4, top + 2.2, 41.6, 2.6, 2.2, 2.8, "dark");
v.ball(49.6, top + 2.4, 43.6, 1.6, 1.4, 1.6, "sock");

const body = new Set<string>();
v.each((x, y, z, ink) => {
  if (ink !== null) body.add(`${x},${y},${z}`);
  return undefined;
});

const tail = sweep([
  [17.2, 41.2, 5.0, 5.0],
  [16.4, 48.8, 5.6, 5.6],
  [23.6, 52.4, 5.8, 5.8],
  [32.8, 52.8, 5.8, 5.8],
  [40.8, 51.2, 5.2, 5.2],
  [46.4, 48.8, 4.6, 4.6],
  [51.2, 45.2, 3.8, 3.8],
  [53.6, 40.8, 3.2, 3.2],
]);
const tailEnd = tail[tail.length - 1]!.s;
const tailFur = (s: number, down: number) =>
  0.8 * Math.sin(7 * (down + 0.025 * s) + 1.2) * (0.7 + 0.3 * Math.sin(0.45 * s + 0.5)) +
  0.3 * Math.sin(0.3 * s + 2) +
  0.3 * Math.sin(19 * (down + 0.04 * s) + 0.5 * Math.sin(0.4 * s));
tube(tail, 0.4, tailFur, (s, down, lift, u) => {
  const t = s / tailEnd;
  if (t > 0.74) return u <= 3 || lift < -0.6 ? "ivory" : "cream";
  if (t > 0.61) return lift > 0.7 ? "brush" : "rust";
  if (down > 2.1) return "rust";
  return lift > 0.6 ? "coat" : lift < -0.6 ? "rust" : "brush";
});

const near6 = [
  [1, 0, 0],
  [-1, 0, 0],
  [0, 1, 0],
  [0, -1, 0],
  [0, 0, 1],
  [0, 0, -1],
];
const crease: number[][] = [];
v.each((x, y, z, ink) => {
  if (ink === null || !v.bare(x, y, z)) return undefined;
  const mine = body.has(`${x},${y},${z}`);
  const touch = near6.some(([dx, dy, dz]) => {
    const key = `${x + dx!},${y + dy!},${z + dz!}`;
    return v.get(x + dx!, y + dy!, z + dz!) !== null && body.has(key) !== mine;
  });
  if (touch && !["cream", "ivory", "dark", "sock"].includes(ink)) crease.push([x, y, z]);
  return undefined;
});
crease.forEach(([x, y, z]) => {
  v.set(x!, y!, z!, "rust");
});

const [hx, hu, hz] = [43, 12.5, 35.6];
const earW = (e: number) => 3.9 - 0.36 * Math.max(0, e - 5);
const earC = (e: number) => 5.0 + 0.12 * Math.max(0, e - 5);
for (let u = 1; u <= 26; u++)
  for (let z = 24; z <= 52; z++)
    for (let x = 32; x <= 60; x++) {
      const [r, e, f] = [x - hx, u - hu, z - hz];
      const brow = 0.4 * Math.sin(1.3 * r) * Math.max(0, 1 - Math.abs(f - 1) / 4);
      const skull = (r / 7.6) ** 2 + ((e - brow) / 6.0) ** 2 + (f / 7.0) ** 2 <= 1;
      const t = (f - 2) / 8.4;
      const [rr, re, ec] = [5.0 - 2.2 * t, 3.8 - 1.4 * t, -1.0 - 1.1 * t];
      const muzzle = f >= 2 && f <= 10.4 && (r / rr) ** 2 + ((e - ec) / re) ** 2 <= 1;
      const earIn = Math.abs(Math.abs(r) - earC(e));
      const earF = f >= -3.6 + 0.07 * (e - 2) && f <= -0.6 - 0.1 * (e - 2);
      const ear = e >= 2 && e <= 14 && earF && earIn <= earW(e);
      if (!skull && !muzzle && !ear) continue;
      let ink = "coat";
      if (ear) {
        ink = f < -2.2 || e > 12 ? "dark" : earIn < earW(e) - 1.5 ? "gold" : "fur";
      } else if (muzzle) {
        ink = "fur";
        if (e < ec - 1.0) ink = "cream";
        if (f > 9 && Math.abs(r) < 1.8 && e > ec - 1.8) ink = "dark";
      } else {
        if (Math.abs(r) < 1.8 && f > 0.8) ink = "fur";
        if (e < -3.8 && f > 2) ink = "cream";
        if (e < -4.6) ink = "cream";
      }
      if (ink === "cream" && u <= 3) ink = "ivory";
      up(x, u, z, ink);
    }
[38, 39, 40, 46, 47, 48].forEach((x) => {
  for (let z = 52; z >= 24; z--)
    if (at(x, 14, z) !== null) {
      up(x, 14, z, "dark");
      break;
    }
});

const span = (y: number) => 30.6 - 12.2 * ((top - y) / top) ** 2.4;
for (let y = 0; y <= top; y++)
  for (let z = 0; z < 64; z++)
    for (let x = 0; x < 64; x++) {
      const turn = Math.atan2(z - 31.5, x - 31.5);
      const far = Math.hypot(x - 31.5, z - 31.5) - 0.9 * Math.cos(3 * turn - 0.3) - 0.5 * Math.sin(5 * turn + 1);
      if (far > span(y)) continue;
      const drip = y === top - 1 && far > span(y) - 1.5 && Math.sin(7 * turn + 2) > 0.2;
      const low = y < 5.5 + 1.4 * Math.sin(4 * turn);
      v.set(x, y, z, y === top || drip ? "moss" : low ? "deep" : "shade");
    }

const ground = (x: number, z: number) => v.get(x, top, z) !== null;
const mound = (cx: number, cz: number, rx: number, rz: number, high: number, ink: string) => {
  for (let u = 1; u <= 3; u++)
    for (let z = Math.floor(cz - rz); z <= Math.ceil(cz + rz); z++)
      for (let x = Math.floor(cx - rx); x <= Math.ceil(cx + rx); x++) {
        const far = ((x - cx) / rx) ** 2 + ((z - cz) / rz) ** 2 + ((u - 0.4) / high) ** 2;
        if (far <= 1 && ground(x, z) && at(x, u, z) === null) up(x, u, z, ink);
      }
};
const stain = (cx: number, cz: number, rx: number, rz: number, ink: string) => {
  for (let z = Math.floor(cz - rz); z <= Math.ceil(cz + rz); z++)
    for (let x = Math.floor(cx - rx); x <= Math.ceil(cx + rx); x++)
      if (((x - cx) / rx) ** 2 + ((z - cz) / rz) ** 2 <= 1 && v.get(x, top, z) === "moss") v.set(x, top, z, ink);
};
[
  [28, 4, 4, 2, "shade"],
  [8, 18, 2.5, 3, "shade"],
  [30, 60.5, 5, 1.6, "shade"],
  [5, 31, 2, 3, "shade"],
  [60, 31, 2, 3, "shade"],
  [15, 8, 3, 2, "lichen"],
  [47, 9, 2.5, 2, "lichen"],
  [57, 40, 2.5, 2.5, "lichen"],
  [10, 50, 2.5, 2.5, "lichen"],
  [48, 56.5, 3, 1.6, "lichen"],
].forEach(([x, z, rx, rz, ink]) => {
  stain(x as number, z as number, rx as number, rz as number, ink as string);
});
[
  [5, 22, 2.4, 2.6, 1.6, "moss"],
  [4.5, 38, 2.2, 3, 1.5, "lichen"],
  [7, 45, 2, 2, 1.4, "moss"],
  [59.5, 35, 2.2, 2.8, 1.6, "moss"],
  [36, 3.5, 3, 2, 1.5, "moss"],
  [19, 5.5, 2.4, 2, 1.4, "lichen"],
  [44, 6, 2.2, 1.8, 1.4, "moss"],
  [20, 59.5, 2.6, 1.8, 1.5, "moss"],
  [38, 60, 2.4, 1.8, 1.4, "lichen"],
  [11, 12.5, 2, 2, 1.3, "shade"],
].forEach(([x, z, rx, rz, high, ink]) => {
  mound(x as number, z as number, rx as number, rz as number, high as number, ink as string);
});

const blade = ["..l..", ".lal.", "llall", "llall", "llall", ".lal.", "..a..", "..s.."];
const sprig = ["....lll", "..llall", ".llall.", "llall..", "lal....", "s......"];
const lay = (x0: number, z0: number, rows: string[], key: Record<string, string>, swap = false, flip = false) => {
  rows.forEach((row, j) => {
    [...row].forEach((glyph, i) => {
      if (glyph === ".") return;
      const [dx, dz] = swap ? [j, i] : [i, j];
      const [x, z] = [x0 + (flip ? -dx : dx), z0 + dz];
      if (ground(x, z) && at(x, 1, z) === null) up(x, 1, z, key[glyph]!);
    });
  });
};
const yellow = { l: "leaf", a: "amber", s: "sock" };
const brown = { l: "amber", a: "sock", s: "sock" };
lay(23, 1, sprig, yellow);
lay(56, 17, blade, brown);
lay(2, 26, sprig, yellow, true);
lay(34, 58, blade, yellow, true, true);
lay(9, 9, sprig, brown, false, true);
blade.forEach((row, j) => {
  [...row].forEach((glyph, i) => {
    if (glyph !== ".") up(18 + j, peak(18 + j, 23 + i) + 1, 23 + i, yellow[glyph as keyof typeof yellow]);
  });
});

const air = new Set<number>();
const spot3 = (x: number, y: number, z: number) => (y * 66 + z + 1) * 66 + x + 1;
const queue = [[-1, 0, -1]];
air.add(spot3(-1, 0, -1));
while (queue.length > 0) {
  const [x, y, z] = queue.pop()!;
  near6.forEach(([dx, dy, dz]) => {
    const [px, py, pz] = [x! + dx!, y! + dy!, z! + dz!];
    if ([px, py, pz].some((p) => p < -1 || p > 64) || py < 0 || air.has(spot3(px, py, pz))) return;
    if (v.get(px, py, pz) !== null) return;
    air.add(spot3(px, py, pz));
    queue.push([px, py, pz]);
  });
}
v.each((x, y, z, ink) => (ink === null && !air.has(spot3(x, y, z)) ? (v.get(x, y - 1, z) ?? "coat") : undefined));

v.save(import.meta.dir, "4x");
