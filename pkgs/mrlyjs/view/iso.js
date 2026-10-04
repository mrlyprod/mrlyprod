// PROJECTION

const RATIO = 0.8660254037844386;

export const EDGES = Object.freeze(
  [[0, 1], [0, 2], [0, 4], [1, 3], [1, 5], [2, 3], [2, 6], [3, 7], [4, 5], [4, 6], [5, 7], [6, 7]].map((edge) => Object.freeze(edge)),
);

export function project(x, y, z) {
  return [(x - y) * RATIO, (x + y) * 0.5 - z];
}

// FACES

export function faces(quads) {
  const out = [];
  for (const { normal, verts } of quads) {
    if (normal.x + normal.y + normal.z <= 0) continue;
    const tone = normal.z > 0 ? 0 : normal.y > 0 ? 1 : 2;
    let depth = 0;
    for (const v of verts) depth += Math.fround(Math.fround(v.x + v.y) + v.z);
    out.push({ quad: verts.map((v) => project(v.x, v.y, v.z)), tone, depth: depth / 4 });
  }
  return out.sort((a, b) => a.depth - b.depth);
}

export function fit(list, frame) {
  let [lx, ly, hx, hy] = [Number.MAX_VALUE, Number.MAX_VALUE, -Number.MAX_VALUE, -Number.MAX_VALUE];
  for (const face of list) {
    for (const p of face.quad) {
      if (p[0] < lx) lx = p[0];
      if (p[1] < ly) ly = p[1];
      if (p[0] > hx) hx = p[0];
      if (p[1] > hy) hy = p[1];
    }
  }
  const sx = Math.max(hx - lx, 1e-9);
  const sy = Math.max(hy - ly, 1e-9);
  const scale = Math.min(frame.w / sx, frame.h / sy);
  const ox = frame.x + (frame.w - sx * scale) / 2;
  const oy = frame.y + (frame.h - sy * scale) / 2;
  return (p) => [ox + (p[0] - lx) * scale, oy + (p[1] - ly) * scale];
}

// DRAWING

export function draw(pen, frame, quads, shade, edge = null) {
  const list = faces(quads);
  if (list.length === 0) return;
  const place = fit(list, frame);
  const thick = Math.max(Math.min(frame.w, frame.h) / 400, 0.6);
  for (const { quad, tone } of list) {
    const pts = quad.map(place);
    pen.polygon(pts, shade[tone]);
    if (edge) pen.polyline([...pts, pts[0]], thick, edge);
  }
}

export function stamp(pen, quads, cx, cy, s, shade, edge, thick) {
  for (const { quad, tone } of faces(quads)) {
    const pts = quad.map((p) => [cx + p[0] * s, cy + p[1] * s]);
    pen.polygon(pts, shade[tone]);
    if (edge) pen.polyline([...pts, pts[0]], thick, edge);
  }
}

export function cage(pen, cx, cy, s, half, thick, color) {
  const corner = (i) => {
    const p = project(((i & 1) * 2 - 1) * half, (((i >> 1) & 1) * 2 - 1) * half, (((i >> 2) & 1) * 2 - 1) * half);
    return [cx + p[0] * s, cy + p[1] * s];
  };
  for (const [a, b] of EDGES) pen.segment(corner(a), corner(b), thick, color);
}
