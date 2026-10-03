// PROJECTION

const RATIO = 0.8660254037844386;

export function project(x, y, z) {
  return [(x - y) * RATIO, (x + y) * 0.5 - z];
}

function fit(faces, frame) {
  let [lx, ly, hx, hy] = [Number.MAX_VALUE, Number.MAX_VALUE, -Number.MAX_VALUE, -Number.MAX_VALUE];
  for (const face of faces) {
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

export function draw(pen, frame, quads, shade, edge) {
  const faces = [];
  for (const { normal, verts } of quads) {
    if (normal.x + normal.y + normal.z <= 0) continue;
    const tone = normal.z > 0 ? 0 : normal.y > 0 ? 1 : 2;
    let depth = 0;
    for (const v of verts) depth += Math.fround(Math.fround(v.x + v.y) + v.z);
    faces.push({ quad: verts.map((v) => project(v.x, v.y, v.z)), tone, depth: depth / 4 });
  }
  if (faces.length === 0) return;
  faces.sort((a, b) => a.depth - b.depth);
  const place = fit(faces, frame);
  const thick = Math.max(Math.min(frame.w, frame.h) / 400, 0.6);
  for (const { quad, tone } of faces) {
    const pts = quad.map(place);
    pen.polygon(pts, shade[tone]);
    if (edge) pen.polyline([...pts, pts[0]], thick, edge);
  }
}
