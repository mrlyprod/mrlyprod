import { Volume } from "../sculpt.ts";

const v = new Volume(32, {
  rust: "#a8421e",
  brush: "#c55a22",
  coat: "#d9692b",
  fur: "#e87a31",
  gold: "#f0a04f",
  cream: "#f6ead6",
  ivory: "#d8c8ae",
  dark: "#45302a",
  moss: "#6c963f",
  shade: "#4a7034",
  lichen: "#8fb04d",
  leaf: "#e8c547",
});

const top = 5;
const up = (x: number, u: number, z: number, ink: string | null) => v.set(x, top + u, z, ink);
const at = (x: number, u: number, z: number) => v.get(x, top + u, z);

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
    for (let t = 0; t < 1; t += 0.04) {
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
  for (let u = 1; u <= 14; u++)
    for (let z = 0; z < 32; z++)
      for (let x = 0; x < 32; x++) {
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
  [9.5, 21.5, 4.6, 3.6],
  [8.4, 16.2, 5.2, 3.9],
  [8.4, 11.2, 5.0, 3.8],
  [11.2, 7.8, 4.7, 3.6],
  [15.5, 6.8, 4.4, 3.5],
  [19.8, 7.6, 4.1, 3.4],
  [22.4, 10.6, 3.7, 3.1],
  [23.0, 13.6, 3.1, 2.8],
  [22.4, 15.4, 2.8, 2.6],
]);
const furl = (s: number, down: number) =>
  0.6 * Math.sin(6 * (down + 0.045 * s) + 0.7) * (0.7 + 0.3 * Math.sin(0.4 * s + 1.1 * down + 2)) +
  0.2 * Math.sin(0.3 * s + 1.6 * down + 1);
const coatOf: Paint = (s, down, lift) => {
  if (down > 2.15) return "rust";
  if (down < 0.5 + 0.12 * Math.sin(0.5 * s + 1)) return "brush";
  return lift > 0.45 && down > 0.7 ? "fur" : "coat";
};
tube(spine, 0.3, furl, coatOf);
const belly = sweep([
  [12.0, 19.0, 4.2, 3.0],
  [11.8, 14.0, 4.4, 3.2],
  [14.0, 10.8, 4.2, 3.0],
  [18.0, 10.6, 3.8, 2.8],
  [20.0, 13.0, 3.4, 2.6],
]);
const underOf: Paint = (_s, down, lift) => (down > 1.9 ? "rust" : lift > 0.4 ? "coat" : "brush");
tube(belly, 0.3, furl, underOf);
const legs = sweep([
  [11.0, 19.0, 3.5, 2.5],
  [14.5, 21.5, 3.2, 2.3],
  [18.0, 22.5, 2.8, 2.1],
  [21.0, 22.0, 2.2, 1.8],
]);
tube(legs, 0.3, furl, underOf);
const chest = sweep([
  [14.0, 13.0, 3.0, 2.2],
  [16.0, 15.5, 3.0, 2.2],
  [18.0, 18.5, 2.8, 2.1],
  [19.5, 21.0, 2.5, 1.9],
]);
tube(chest, 0.3, furl, underOf);
v.ball(24.6, top + 1.3, 21.0, 1.3, 1.1, 1.4, "dark");

const body = new Set<string>();
v.each((x, y, z, ink) => {
  if (ink !== null) body.add(`${x},${y},${z}`);
  return undefined;
});

const tail = sweep([
  [8.6, 20.6, 2.5, 2.5],
  [8.2, 24.4, 2.8, 2.8],
  [11.8, 26.2, 2.9, 2.9],
  [16.4, 26.4, 2.9, 2.9],
  [20.4, 25.6, 2.6, 2.6],
  [23.2, 24.4, 2.3, 2.3],
  [25.6, 22.6, 1.9, 1.9],
  [26.8, 20.4, 1.6, 1.6],
]);
const tailEnd = tail[tail.length - 1]!.s;
const tailFur = (s: number, down: number) =>
  0.45 * Math.sin(7 * (down + 0.05 * s) + 1.2) * (0.7 + 0.3 * Math.sin(0.9 * s + 0.5)) + 0.2 * Math.sin(0.6 * s + 2);
tube(tail, 0.2, tailFur, (s, down, lift, u) => {
  const t = s / tailEnd;
  if (t > 0.74) return u <= 2 ? "ivory" : "cream";
  if (t > 0.62) return "rust";
  if (down > 2.1) return "rust";
  return lift > 0.2 ? "brush" : "coat";
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
  if (touch && !["cream", "ivory", "dark"].includes(ink)) crease.push([x, y, z]);
  return undefined;
});
crease.forEach(([x, y, z]) => {
  v.set(x!, y!, z!, "rust");
});

const [hx, hu, hz] = [21.5, 6.5, 17.8];
const earW = (e: number) => 2.0 - 0.3 * Math.max(0, e - 2.5);
const earC = (e: number) => 2.6 + 0.1 * Math.max(0, e - 2.5);
for (let u = 1; u <= 12; u++)
  for (let z = 12; z <= 26; z++)
    for (let x = 16; x <= 30; x++) {
      const [r, e, f] = [x - hx, u - hu, z - hz];
      const skull = (r / 3.8) ** 2 + (e / 3.0) ** 2 + (f / 3.5) ** 2 <= 1;
      const t = (f - 1) / 4.2;
      const [rr, re, ec] = [2.5 - 1.1 * t, 1.9 - 0.7 * t, -0.5 - 0.55 * t];
      const muzzle = f >= 1 && f <= 5.2 && (r / rr) ** 2 + ((e - ec) / re) ** 2 <= 1;
      const ear = e >= 1 && e <= 7.6 && f >= -2.2 && f <= -0.2 && Math.abs(Math.abs(r) - earC(e)) <= earW(e);
      if (!skull && !muzzle && !ear) continue;
      let ink = "coat";
      if (ear) {
        ink = f < -1.2 || e > 6 ? "dark" : Math.abs(Math.abs(r) - earC(e)) < earW(e) - 0.8 ? "gold" : "fur";
      } else if (muzzle) {
        ink = "fur";
        if (e < ec - 0.5) ink = "cream";
        if (f > 4.5 && Math.abs(r) < 0.9 && e > ec - 0.9) ink = "dark";
      } else {
        if (Math.abs(r) < 0.9 && f > 0.4) ink = "fur";
        if (e < -1.9 && f > 1) ink = "cream";
        if (e < -2.3) ink = "cream";
      }
      if (ink === "cream" && u <= 2) ink = "ivory";
      up(x, u, z, ink);
    }
[19, 20, 23, 24].forEach((x) => {
  for (let z = 26; z >= 12; z--)
    if (at(x, 7, z) !== null) {
      up(x, 7, z, "dark");
      break;
    }
});

const rim = (y: number) => [9.2, 11.6, 13.4, 14.6, 15.2, 15.3][y]!;
for (let y = 0; y <= top; y++)
  for (let z = 0; z < 32; z++)
    for (let x = 0; x < 32; x++) {
      const turn = Math.atan2(z - 15.5, x - 15.5);
      const lobe = 0.45 * Math.cos(3 * turn - 0.3) + 0.25 * Math.sin(5 * turn + 1);
      if (Math.hypot(x - 15.5, z - 15.5) <= rim(y) + lobe) v.set(x, y, z, y === top ? "moss" : "shade");
    }

const patch = (cells: number[], u: number, ink: string) => {
  for (let i = 0; i < cells.length; i += 2) {
    const [x, z] = [cells[i]!, cells[i + 1]!];
    if (at(x, 0, z) !== null && (u === 0 || at(x, u, z) === null)) up(x, u, z, ink);
  }
};
patch([2, 12, 2, 13, 1, 13], 1, "lichen");
patch([29, 9, 29, 10, 28, 9], 1, "moss");
patch([5, 5, 6, 5, 5, 4], 1, "moss");
patch([20, 30, 21, 30, 21, 29], 1, "lichen");
patch([3, 20, 2, 19, 3, 19], 1, "moss");
patch([14, 2, 15, 2, 15, 1], 1, "moss");
patch([8, 2, 9, 2, 10, 2, 9, 3, 9, 1], 0, "shade");
patch([1, 17, 2, 17, 1, 18, 2, 18, 0, 17], 0, "shade");
patch([10, 30, 11, 30, 12, 30, 11, 31], 0, "shade");
patch([29, 15, 30, 15, 30, 16, 30, 17, 29, 16], 0, "shade");
patch([24, 2, 25, 2, 24, 3, 23, 2], 0, "lichen");
patch([3, 8, 3, 9, 2, 9, 2, 10], 0, "lichen");
patch([26, 28, 25, 28, 25, 29, 24, 29], 0, "lichen");
patch([29, 11, 30, 11, 30, 12], 1, "leaf");
patch([1, 15, 1, 16, 2, 16], 1, "leaf");
patch([17, 30, 18, 30, 18, 29], 1, "leaf");
patch([6, 27, 6, 28, 7, 28], 0, "leaf");
const drop = (x: number, z: number, ink: string) => {
  for (let u = 14; u >= 1; u--)
    if (at(x, u, z) !== null) {
      up(x, u + 1, z, ink);
      return;
    }
};
[
  [9, 12],
  [10, 12],
  [10, 13],
  [11, 13],
].forEach(([x, z]) => {
  drop(x!, z!, "leaf");
});

const air = new Set<number>();
const cell = (x: number, y: number, z: number) => (y * 34 + z + 1) * 34 + x + 1;
const queue = [[-1, 0, -1]];
air.add(cell(-1, 0, -1));
while (queue.length > 0) {
  const [x, y, z] = queue.pop()!;
  near6.forEach(([dx, dy, dz]) => {
    const [px, py, pz] = [x! + dx!, y! + dy!, z! + dz!];
    if ([px, py, pz].some((p) => p < -1 || p > 32) || py < 0 || air.has(cell(px, py, pz))) return;
    if (v.get(px, py, pz) !== null) return;
    air.add(cell(px, py, pz));
    queue.push([px, py, pz]);
  });
}
v.each((x, y, z, ink) => (ink === null && !air.has(cell(x, y, z)) ? (v.get(x, y - 1, z) ?? "coat") : undefined));

v.save(import.meta.dir, "2x");
