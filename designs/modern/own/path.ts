import type { Point } from "@mrlyprod/draw";

const STEP = 0.3;

export function arc(cx: number, cy: number, r: number, from: number, to: number): Point[] {
  const steps = Math.max(2, Math.ceil((Math.abs(to - from) * Math.max(r, 1)) / STEP));
  return Array.from({ length: steps + 1 }, (_, i): Point => {
    const t = from + ((to - from) * i) / steps;
    return [cx + r * Math.cos(t), cy + r * Math.sin(t)];
  });
}

export function oval(cx: number, cy: number, rx: number, ry: number, tilt = 0): Point[] {
  const steps = Math.max(24, Math.ceil((2 * Math.PI * Math.max(rx, ry)) / STEP));
  const c = Math.cos(tilt);
  const s = Math.sin(tilt);
  return Array.from({ length: steps }, (_, i): Point => {
    const t = (2 * Math.PI * i) / steps;
    const x = rx * Math.cos(t);
    const y = ry * Math.sin(t);
    return [cx + x * c - y * s, cy + x * s + y * c];
  });
}

export function round(pts: Point[], r: number | number[], closed = false): Point[] {
  const n = pts.length;
  const out: Point[] = [];
  pts.forEach((p, i) => {
    const want = typeof r === "number" ? r : (r[i] ?? 0);
    const end = !closed && (i === 0 || i === n - 1);
    if (end || want <= 0) {
      out.push(p);
      return;
    }
    const a = pts[(i - 1 + n) % n]!;
    const b = pts[(i + 1) % n]!;
    const la = Math.hypot(a[0] - p[0], a[1] - p[1]);
    const lb = Math.hypot(b[0] - p[0], b[1] - p[1]);
    const ua: Point = [(a[0] - p[0]) / la, (a[1] - p[1]) / la];
    const ub: Point = [(b[0] - p[0]) / lb, (b[1] - p[1]) / lb];
    const half = Math.acos(Math.max(-1, Math.min(1, ua[0] * ub[0] + ua[1] * ub[1]))) / 2;
    if (half < 1e-3 || Math.PI / 2 - half < 1e-3) {
      out.push(p);
      return;
    }
    const t = Math.min(want / Math.tan(half), la / 2, lb / 2);
    const rr = t * Math.tan(half);
    const mx = ua[0] + ub[0];
    const my = ua[1] + ub[1];
    const ml = Math.hypot(mx, my);
    const d = rr / Math.sin(half);
    const c: Point = [p[0] + (mx / ml) * d, p[1] + (my / ml) * d];
    const ta: Point = [p[0] + ua[0] * t, p[1] + ua[1] * t];
    const tb: Point = [p[0] + ub[0] * t, p[1] + ub[1] * t];
    const from = Math.atan2(ta[1] - c[1], ta[0] - c[0]);
    let to = Math.atan2(tb[1] - c[1], tb[0] - c[0]);
    while (to - from > Math.PI) to -= 2 * Math.PI;
    while (from - to > Math.PI) to += 2 * Math.PI;
    out.push(...arc(c[0], c[1], rr, from, to));
  });
  return out;
}

export function box(x: number, y: number, w: number, h: number, r: number): Point[] {
  return round(
    [
      [x, y],
      [x + w, y],
      [x + w, y + h],
      [x, y + h],
    ],
    r,
    true,
  );
}

export function turn(pts: Point[], angle: number, cx = 12, cy = 12): Point[] {
  const c = Math.cos(angle);
  const s = Math.sin(angle);
  return pts.map(([x, y]): Point => [cx + (x - cx) * c - (y - cy) * s, cy + (x - cx) * s + (y - cy) * c]);
}

export function shift(pts: Point[], dx: number, dy: number): Point[] {
  return pts.map(([x, y]): Point => [x + dx, y + dy]);
}

export function flip(pts: Point[], axis = 12): Point[] {
  return pts.map(([x, y]): Point => [2 * axis - x, y]);
}

export function smooth(pts: Point[], closed = true, steps = 12): Point[] {
  const n = pts.length;
  const at = (i: number) => (closed ? pts[(i + n) % n]! : pts[Math.max(0, Math.min(n - 1, i))]!);
  const out: Point[] = [];
  const last = closed ? n : n - 1;
  for (let i = 0; i < last; i++) {
    const [p0, p1, p2, p3] = [at(i - 1), at(i), at(i + 1), at(i + 2)];
    for (let k = 0; k < steps; k++) {
      const t = k / steps;
      const t2 = t * t;
      const t3 = t2 * t;
      const f = (a: number, b: number, c: number, d: number) =>
        0.5 * (2 * b + (-a + c) * t + (2 * a - 5 * b + 4 * c - d) * t2 + (-a + 3 * b - 3 * c + d) * t3);
      out.push([f(p0[0], p1[0], p2[0], p3[0]), f(p0[1], p1[1], p2[1], p3[1])]);
    }
  }
  if (!closed) out.push(pts[n - 1]!);
  return out;
}

export function clip(subject: Point[], hull: Point[]): Point[] {
  const area = hull.reduce((sum, p, i) => {
    const q = hull[(i + 1) % hull.length]!;
    return sum + p[0] * q[1] - q[0] * p[1];
  }, 0);
  const sign = area > 0 ? 1 : -1;
  let out = subject;
  hull.forEach((a, i) => {
    const b = hull[(i + 1) % hull.length]!;
    const side = (p: Point) => sign * ((b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]));
    const cut = (p: Point, q: Point): Point => {
      const sp = side(p);
      const t = sp / (sp - side(q));
      return [p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t];
    };
    const next: Point[] = [];
    out.forEach((p, j) => {
      const q = out[(j + 1) % out.length]!;
      const inP = side(p) >= 0;
      const inQ = side(q) >= 0;
      if (inP) next.push(p);
      if (inP !== inQ) next.push(cut(p, q));
    });
    out = next;
  });
  return out;
}
